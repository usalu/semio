#!/usr/bin/env python3
"""🌉 Compiler-driven `resolve_ready(...)` bridge for `.await` trapped inside a SYNC closure.

WHY THIS EXISTS
----------------
R10 residue class 1 ("`.await` inside a sync closure") is common in stdio's binary diff/codec
trees: a generic helper like `write_bin_vec<T>(w, items, write_item: impl Fn(&mut ByteWriter, &T))`
takes a plain SYNC `Fn` callback (most call sites pass genuinely-sync closures — `|w, x|
w.write_f64_le(*x)` — so the helper's bound must stay sync, not be changed crate-wide). A handful
of call sites instead pass a bare `async fn` item (`write_bin_json`) or a closure that calls one
recursively, and `insert-await.py`'s fixpoint dutifully adds `.await` at the (correct, per rustc's
own suggestion) call site — which is illegal inside that sync closure -> E0728.

`semio_framework_plugin::resolve_ready<F: Future>(fut: F) -> F::Output` is this codebase's OWN
existing, already-used bridge for exactly this shape (see e.g. the bcf composer's
`resolve_ready((entry.compose)(sources))`). Since every one of these decode/encode helpers
operates on already-in-memory buffers/trees with no real suspension point, resolving the future
immediately is correct, not a hack — it's the established E5 idiom in this crate.

WHAT MAKES THIS SAFE (span-keyed per R10)
------------------------------------------
Only touches an E0728 diagnostic's PRIMARY span, verified byte-for-byte to be the literal `await`
token (5 bytes) preceded by a `.` which is itself preceded by a `)` — i.e. exactly the shape
`SOMECALL(args).await`. That is "shape A" of this residue (measured split: 224 shape-A vs 502
"shape B" bare-place-expression `X.await.method()` sites — those are a DIFFERENT residue class,
this tool explicitly refuses/skips them, see 📓️terra-stdio-await-report.md).

Transform: locate the matching OPENING paren of the call by a balanced backward scan from the
`)` immediately before `.await`, then locate the call's callee-start by scanning backward over
identifier/path characters (`A-Za-z0-9_:`) from that opening paren. Rewrite
`CALLEE(args).await` -> `semio_framework_plugin::resolve_ready(CALLEE(args))`.

Idempotent: skips any span already inside a `resolve_ready(` wrapper.
"""
from __future__ import annotations
import argparse, json, os, subprocess, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
BRIDGE = b"semio_framework_plugin::resolve_ready"
# 🐛 BUGFIX: `set(b"ABC...")` yields a set of INTEGERS in Python 3 (iterating `bytes` yields ints),
# but `data[i:i+1]` is a length-1 `bytes` SLICE — `b's' in {115, ...}` is always False (a bytes
# object never equals an int). That silently made the old backward callee-scan a no-op on every
# call, producing corrupted output like `v_raw::entriessemio_framework_plugin::resolve_ready(())`
# instead of `semio_framework_plugin::resolve_ready(v_raw::entries())` — the callee name was left
# in place and only the empty `()` args got wrapped. Confirmed and repaired for real via
# `repair-wrap-corruption.py` in this same ticket folder; this fix prevents recurrence.
CALLEE_CHARS = set(bytes([b]) for b in b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_:")


def run_check(crate: str, target_dir: str) -> list[dict]:
    cmd = ["cargo", "check", "-p", crate, "--message-format=json", "--lib"]
    env = dict(os.environ, CARGO_TARGET_DIR=target_dir)
    proc = subprocess.run(cmd, cwd=REPO, env=env, capture_output=True, text=True)
    out = []
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") == "compiler-message" and msg.get("message"):
            out.append(msg["message"])
    return out


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    if not want:
        return True
    return any(parts[i:i + len(want)] == want for i in range(len(parts) - len(want) + 1))


def find_matching_open_paren(data: bytes, close_idx: int) -> int:
    depth = 0
    i = close_idx
    while i >= 0:
        c = data[i:i + 1]
        if c == b")":
            depth += 1
        elif c == b"(":
            depth -= 1
            if depth == 0:
                return i
        i -= 1
    return -1


def collect(diags: list[dict], scope: str | None):
    edits, skipped_b, other = [], [], []
    for d in diags:
        if d.get("level") != "error":
            continue
        if (d.get("code") or {}).get("code") != "E0728":
            continue
        for sp in d.get("spans", []):
            if not sp.get("is_primary"):
                continue
            fn = sp.get("file_name", "")
            path = os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))
            if scope and not in_scope(path, scope):
                other.append((path, "out-of-scope"))
                continue
            edits.append((path, sp["byte_start"], sp["byte_end"]))
    return edits, skipped_b, other


def apply_edits(edits, guard: set):
    by_file = defaultdict(list)
    for path, bs, be in edits:
        key = (path, bs, be)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append((bs, be))

    applied, refused, shape_b = 0, [], 0
    for path, items in by_file.items():
        try:
            with open(path, "rb") as fh:
                data = fh.read()
        except OSError as exc:
            refused.append((path, f"unreadable: {exc}"))
            continue
        items.sort(key=lambda t: t[0], reverse=True)
        last_start = None
        for bs, be in items:
            if last_start is not None and be > last_start:
                continue
            if data[bs:be] != b"await" or bs == 0 or data[bs - 1:bs] != b".":
                refused.append((path, f"span at {bs}:{be} is {data[bs:be]!r}, not the expected '.await'"))
                continue
            dot_idx = bs - 1
            if dot_idx == 0 or data[dot_idx - 1:dot_idx] != b")":
                shape_b += 1  # bare place-expression shape — not this tool's job
                continue
            close_paren = dot_idx - 1
            open_paren = find_matching_open_paren(data, close_paren)
            if open_paren == -1:
                refused.append((path, f"unbalanced parens before {close_paren}"))
                continue
            callee_start = open_paren
            while callee_start > 0 and data[callee_start - 1:callee_start] in CALLEE_CHARS:
                callee_start -= 1
            # 🐛 BUGFIX: the scan above stops at the METHOD name for `receiver.method(args)` —
            # correct for free/path calls (`v_raw::entries()`) but wrong for method calls, where
            # it left `receiver.` stranded outside the wrap, producing
            # `receiver.semio_framework_plugin::resolve_ready(method(args))` (a hard parse error:
            # "expected a pattern, found an expression"). Found and confirmed via a coordinator
            # audit request, 96 sites across 38 files — repaired separately with
            # `fix-method-wrap-corruption.py`; this loop prevents recurrence by extending the scan
            # back across every `.field` / `.method_result` / `[index]` segment of the receiver
            # chain, not just the final method name.
            while callee_start > 0 and data[callee_start - 1:callee_start] == b".":
                seg_end = callee_start - 1
                seg_start = seg_end
                while seg_start > 0 and data[seg_start - 1:seg_start] in CALLEE_CHARS:
                    seg_start -= 1
                if seg_start == seg_end:
                    break  # a bare '.' with no identifier before it — stop, don't guess
                callee_start = seg_start
            if data[max(0, callee_start - len(BRIDGE) - 1):callee_start].endswith(b"resolve_ready(") or \
               data[callee_start:callee_start + len(BRIDGE)] == BRIDGE:
                continue  # already bridged
            call_text = data[callee_start:close_paren + 1]
            new_expr = BRIDGE + b"(" + call_text + b")"
            data = data[:callee_start] + new_expr + data[be:]
            last_start = callee_start
            applied += 1
        with open(path, "wb") as fh:
            fh.write(data)
    return applied, refused, shape_b


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--max-passes", type=int, default=10)
    ap.add_argument("--target-dir", required=True)
    args = ap.parse_args()

    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    guard: set = set()
    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir)
        edits, _, other = collect(diags, args.scope)
        print(f"[pass {npass}] E0728-spans={len(edits)} skipped-out-of-scope={len(other)}")

        if args.dry_run:
            for path, bs, be in edits[:60]:
                rel = os.path.relpath(path, REPO)
                print(f"    BRIDGE {rel} [{bs}:{be}]")
            break

        if not edits:
            print("  fixpoint reached")
            break
        applied, refused, shape_b = apply_edits(edits, guard)
        print(f"  applied {applied}, shape-b (skipped, not ours) {shape_b}")
        for path, why in refused:
            print(f"  !! refused {os.path.relpath(path, REPO)}: {why}")
        if applied == 0:
            print("  no progress — stopping")
            break

    return 0


if __name__ == "__main__":
    sys.exit(main())
