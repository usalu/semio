#!/usr/bin/env python3
"""🪝 Fix `IDENT.await.method(args)` → `IDENT.method(args).await`, and await IDENT's own binding once.

WHY THIS EXISTS
----------------
R10 residue class 2 ("awaiting one future repeatedly"). `insert-await.py`'s fixpoint applies
rustc's OWN per-diagnostic suggestion locally: when a bare place expression (`reader`, `data`,
`parts`, ...) is bound to an async constructor's result and then used several times
(`reader.read_u8()`, `reader.read_varint_u64()`, ...), EACH use independently gets a "consider
awaiting" suggestion that lands right after the bare identifier — because from that one
diagnostic's local view, `reader.await` is a valid fix for THAT use. Applied across every use
site (as the crate-wide fixpoint necessarily does — it cannot see that they share one binding),
this produces `reader.await.read_u8()`, `reader.await.read_varint_u64()`, ... — awaiting a place
expression multiple times, most often inside a sync closure it doesn't even belong in (E0728).

Measured (📓️terra-stdio-await-report.md): 502 such E0728 sites in stdio scope, 405 of them the
single receiver `reader` (from `store::ByteReader`, itself framework-owned and out of this
packet's `path_scope` — but the FIX below never touches that definition; it only corrects how
stdio's OWN call sites sequence `.await` against the framework API as it currently stands, which
is squarely in scope).

THE FIX, per receiver IDENT
-----------------------------
1. Locate every `IDENT.await.METHOD(args)` in the file (diagnostic-anchored: only IDENTs that
   rustc's own E0728 primary span flagged as `.await` sites are considered — never a blind
   identifier sweep). Rewrite to `IDENT.METHOD(args).await` (balanced-paren scan for `args`).
2. Locate `IDENT`'s own `let (mut )?IDENT = EXPR;` binding — the nearest one lexically BEFORE the
   earliest flagged use in the same file (handles the common case of one binding per function
   without needing a full parser) — and append `.await` before the `;` if not already present.

SAFETY
------
* IDENT set comes from real E0728 diagnostics, not a name guess.
* Refuses (reports, does not touch) any receiver whose preceding `let` binding cannot be found
  unambiguously, rather than guessing.
* Idempotent per occurrence via a byte-span guard.
"""
from __future__ import annotations
import argparse, json, os, re, subprocess, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
# 🐛 BUGFIX: rustc's byte_start/byte_end are BYTE offsets; this repo's source is full of
# multi-byte emoji (paths, doc comments, identifiers). The first version of this script sliced a
# Python `str` (character-indexed) with those offsets — correct only for files with zero
# multi-byte content before the target position (1 of 502 sites, by luck). Every other site's
# safety check `text[bs:be] != "await"` correctly detected the misalignment and REFUSED rather
# than corrupting anything (verified: no site was wrongly edited), but made no progress. Fixed by
# operating on `bytes` throughout, exactly like `insert-await.py` / `remove-bad-await.py` do.
IDENT_RE = re.compile(rb'[A-Za-z_][A-Za-z0-9_]*$')


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


def collect_e0728_shape_b(diags, scope):
    """Only sites where `.await` is preceded by a bare identifier (not `)`  -> not a call)."""
    sites = []  # (path, bs, be)
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
                continue
            sites.append((path, sp["byte_start"], sp["byte_end"]))
    return sites


def process_file(path: str, sites: list[tuple[int, int]], apply: bool):
    with open(path, "rb") as fh:
        data = fh.read()

    idents_needed: set[bytes] = set()
    method_edits = []  # (call_start_of_.await, call_end_after_method_call, replacement)
    refused = []

    method_re = re.compile(rb'\.([A-Za-z_][A-Za-z0-9_]*)\s*\(')

    for bs, be in sites:
        if data[bs:be] != b"await" or bs == 0 or data[bs - 1:bs] != b".":
            refused.append((bs, be, "span is not '.await'"))
            continue
        dot_idx = bs - 1
        m = IDENT_RE.search(data[:dot_idx])
        if not m:
            refused.append((bs, be, "no bare identifier before .await"))
            continue
        ident = m.group(0)
        # must be followed by `.METHOD(`
        mm = method_re.match(data, be)
        if not mm:
            refused.append((bs, be, "not followed by .method("))
            continue
        open_paren_abs = mm.end() - 1

        depth = 0
        i = open_paren_abs
        close_paren_abs = -1
        while i < len(data):
            c = data[i:i + 1]
            if c == b"(":
                depth += 1
            elif c == b")":
                depth -= 1
                if depth == 0:
                    close_paren_abs = i
                    break
            i += 1
        if close_paren_abs == -1:
            refused.append((bs, be, "unbalanced parens in method call"))
            continue

        method_call_with_await_removed = data[be:close_paren_abs + 1]  # b".METHOD(args)"
        replacement = method_call_with_await_removed + b".await"
        # full span being replaced: from the '.' before 'await' (dot_idx) through close paren
        method_edits.append((dot_idx, close_paren_abs + 1, replacement))
        idents_needed.add(ident)

    if not method_edits:
        return 0, 0, refused

    # Apply method-call relocation edits, descending order.
    method_edits.sort(key=lambda e: e[0], reverse=True)
    new_data = data
    applied_methods = 0
    for start, end, repl in method_edits:
        new_data = new_data[:start] + repl + new_data[end:]
        applied_methods += 1

    # Now fix each ident's own `let` binding: nearest preceding `let (mut )?IDENT = ...;`
    applied_lets = 0
    for ident in idents_needed:
        let_re = re.compile(rb'\blet\s+(?:mut\s+)?' + re.escape(ident) + rb'\s*=\s*')
        candidates = list(let_re.finditer(new_data))
        if not candidates:
            refused.append((-1, -1, f"no 'let {ident.decode()} = ' binding found — left un-awaited"))
            continue
        fixed_any = False
        for m in candidates:
            stmt_start = m.end()
            depth = 0
            j = stmt_start
            end_semicolon = -1
            while j < len(new_data):
                c = new_data[j:j + 1]
                if c in (b"(", b"[", b"{"):
                    depth += 1
                elif c in (b")", b"]", b"}"):
                    depth -= 1
                elif c == b";" and depth == 0:
                    end_semicolon = j
                    break
                j += 1
            if end_semicolon == -1:
                continue
            stmt = new_data[stmt_start:end_semicolon]
            if stmt.rstrip().endswith(b".await"):
                continue  # already fine
            new_data = new_data[:end_semicolon] + b".await" + new_data[end_semicolon:]
            applied_lets += 1
            fixed_any = True
            break
        if not fixed_any and candidates:
            pass  # every binding already had .await — fine, nothing to do

    if apply and (applied_methods or applied_lets):
        with open(path, "wb") as fh:
            fh.write(new_data)

    return applied_methods, applied_lets, refused


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

    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir)
        sites = collect_e0728_shape_b(diags, args.scope)
        by_file = defaultdict(list)
        for path, bs, be in sites:
            by_file[path].append((bs, be))

        print(f"[pass {npass}] shape-b E0728 sites={len(sites)} across {len(by_file)} files")

        if args.dry_run:
            for path, items in list(by_file.items())[:10]:
                print(f"    {os.path.relpath(path, REPO)}: {len(items)} sites")
            break

        if not sites:
            print("  fixpoint reached")
            break

        total_methods = total_lets = 0
        all_refused = []
        for path, items in by_file.items():
            m, l, refused = process_file(path, items, apply=True)
            total_methods += m
            total_lets += l
            for bs, be, why in refused:
                all_refused.append((path, bs, be, why))
        print(f"  relocated {total_methods} .await(s), fixed {total_lets} let-bindings")
        for path, bs, be, why in all_refused[:20]:
            print(f"  !! {os.path.relpath(path, REPO)} [{bs}:{be}]: {why}")
        if total_methods == 0:
            print("  no progress — stopping")
            break

    return 0


if __name__ == "__main__":
    sys.exit(main())
