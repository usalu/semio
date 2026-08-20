#!/usr/bin/env python3
"""📦 Compiler-driven `Box::pin(...)` insertion for self/mutually-recursive `async fn` cycles.

WHY THIS EXISTS
----------------
`insert-await.py` cannot fix E0733 ("recursion in an async fn requires boxing") — rustc offers
no `suggested_replacement` for it, because merely adding `.await` doesn't compile; the future's
compile-time size is genuinely unbounded until one edge of the recursion cycle goes through the
heap. This is R10's documented residue class 3.

WHAT MAKES THIS SAFE (span-keyed, per R10 — never name/regex-guessed)
-----------------------------------------------------------------------
For an E0733 diagnostic, rustc's OWN spans identify the exact call expression(s) that close the
cycle — a non-primary span on the main diagnostic labelled "recursive call here", and (for
mutual recursion) a child diagnostic "which leads to this async fn" carrying a span labelled
"...leading to this recursive call". Both were measured, byte-for-byte, to cover EXACTLY
`CALLEE(args).await` — e.g. `apply_array_diff(diff, items).await` — nothing more, nothing less.
So the edit is mechanical and precise: strip the trailing `.await` (6 bytes), wrap the remainder
in `Box::pin(...)`, and put `.await` back outside it: `Box::pin(apply_array_diff(diff, items)).await`.

This is NOT a name-keyed sweep — it only ever touches a byte span rustc itself flagged as the
recursive edge, exactly the same discipline as `insert-await.py`/`remove-bad-await.py`.

FIXPOINT
--------
A single async fn can sit in more than one recursive path (e.g. a diff enum with several
recursive variants, each its own cycle edge) — boxing one flagged edge does not necessarily
clear every E0733 for that function. Runs to a fixpoint like the other tools: recompile, collect
newly-flagged spans, apply, repeat.

USAGE
-----
    python3 box-recursive-await.py --crate semio-s-plugin-stdio --dry-run --scope '🔌️plugins/🗄️stdio' --target-dir <dir>
    python3 box-recursive-await.py --crate semio-s-plugin-stdio --apply   --scope '🔌️plugins/🗄️stdio' --target-dir <dir>
"""
from __future__ import annotations
import argparse, json, os, subprocess, sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"


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


def collect(diags: list[dict], scope: str | None):
    """🔎 Every span labelled 'recursive call here' / '...leading to this recursive call' under an
    E0733 diagnostic. Returns (edits, skipped) where edit = (path, byte_start, byte_end)."""
    edits, skipped = [], []

    def scan_spans(spans):
        for sp in spans:
            label = (sp.get("label") or "")
            if "recursive call" not in label:
                continue
            fn = sp.get("file_name", "")
            path = os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))
            if scope and not in_scope(path, scope):
                skipped.append((path, "out-of-scope"))
                continue
            edits.append((path, sp["byte_start"], sp["byte_end"]))

    for d in diags:
        if d.get("level") != "error":
            continue
        if (d.get("code") or {}).get("code") != "E0733":
            continue
        scan_spans(d.get("spans", []))
        for ch in d.get("children", []):
            scan_spans(ch.get("spans", []))

    return edits, skipped


def apply_edits(edits, guard: set) -> tuple[int, list]:
    by_file = defaultdict(list)
    for path, bs, be in edits:
        key = (path, bs, be)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append((bs, be))

    applied, refused = 0, []
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
            chunk = data[bs:be]
            if not chunk.endswith(b".await"):
                refused.append((path, f"span at {bs}:{be} is {chunk!r}, does not end in .await"))
                continue
            inner = chunk[: -len(b".await")]
            if inner.startswith(b"Box::pin(") and inner.endswith(b")"):
                continue  # already boxed
            replacement = b"Box::pin(" + inner + b").await"
            data = data[:bs] + replacement + data[be:]
            last_start = bs
            applied += 1
        with open(path, "wb") as fh:
            fh.write(data)
    return applied, refused


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
        e0733 = [d for d in diags if d.get("level") == "error"
                 and (d.get("code") or {}).get("code") == "E0733"]
        edits, skipped = collect(diags, args.scope)
        print(f"[pass {npass}] E0733-diagnostics={len(e0733)} edit-spans={len(edits)} "
              f"skipped={len(skipped)}")

        if args.dry_run:
            for path, bs, be in edits[:60]:
                rel = os.path.relpath(path, REPO)
                print(f"    BOX {rel} [{bs}:{be}]")
            break

        if not edits:
            print("  fixpoint reached (no E0733 recursive-call spans left)")
            break
        applied, refused = apply_edits(edits, guard)
        print(f"  applied {applied}")
        for path, why in refused:
            print(f"  !! refused {os.path.relpath(path, REPO)}: {why}")
        if applied == 0:
            print("  no progress — stopping")
            break

    return 0


if __name__ == "__main__":
    sys.exit(main())
