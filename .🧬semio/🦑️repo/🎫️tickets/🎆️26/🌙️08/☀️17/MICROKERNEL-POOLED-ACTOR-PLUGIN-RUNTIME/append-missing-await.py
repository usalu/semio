#!/usr/bin/env python3
"""➕ Compiler-driven `.await` APPENDING for `E0277 ... is not a future` sites with NO existing
`.await` and NO `suggested_replacement` from rustc.

WHY THIS EXISTS
----------------
`insert-await.py` only applies edits rustc itself proposes as a `suggested_replacement`. For a
`let (c, d) = walk(item, depth + 1);` shape — a future-typed call destructured directly into a
tuple pattern — rustc reports `E0277: (_, _) is not a future` with its primary span on the
LEFT-HAND PATTERN, not the call, and offers no textual suggestion (a fix here often also needs
`Box::pin`, so a bare-await suggestion wouldn't compile anyway). `remove-bad-await.py` correctly
REFUSES these (its own safety check: the flagged span isn't literally `await` bytes), so they sit
as permanent residue in both tools.

Measured (see 📓️terra-stdio-await-report.md): every one of these ~109 stdio-scope sites has its
PRIMARY SPAN covering exactly the call expression that needs `.await` (verified by byte content —
e.g. `wrap_at_path(rest, leaf)`, `walk(item, depth + 1)`) even when a DIFFERENT part of the
statement (the let-pattern) is what rustc chose to underline in the rendered text. The JSON
`byte_start`/`byte_end` on the primary span is reliably the call, not the pattern.

WHAT IT DOES
------------
For every in-scope `E0277 ... is not a future` diagnostic whose primary-span byte range does NOT
already end in `.await`, insert `.await` immediately after `byte_end`. Nothing else. Fixpoint
loop like the sibling tools — a newly-awaited call frequently reveals it also needs
`box-recursive-await.py` (self/mutual recursion) or another layer of missing await, so alternate
this with that tool and `insert-await.py` until both reach a real fixpoint.

SAFETY
------
* Span-keyed (R10): only ever touches the exact byte range rustc's own diagnostic flagged.
* Refuses (does not touch, reports instead) any span whose current bytes already end in `.await`
  — that shape belongs to `remove-bad-await.py`, not this tool.
* Per-file descending-offset application, guard set, `--scope` path-segment matching, same as
  every other tool on this ticket.
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
    edits, skipped = [], []
    for d in diags:
        if d.get("level") != "error":
            continue
        if (d.get("code") or {}).get("code") != "E0277":
            continue
        if "is not a future" not in (d.get("message") or ""):
            continue
        for sp in d.get("spans", []):
            if not sp.get("is_primary"):
                continue
            fn = sp.get("file_name", "")
            path = os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))
            if scope and not in_scope(path, scope):
                skipped.append((path, "out-of-scope"))
                continue
            edits.append((path, sp["byte_start"], sp["byte_end"]))
    return edits, skipped


def apply_edits(edits, guard: set):
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
            if data[be:be + 6] == b".await":
                refused.append((path, f"already .await right after {be}, skipping"))
                continue
            if data[max(0, be - 6):be] == b".await":
                refused.append((path, f"span at {bs}:{be} already ends in .await — belongs to remove-bad-await.py"))
                continue
            data = data[:be] + b".await" + data[be:]
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
        edits, skipped = collect(diags, args.scope)
        print(f"[pass {npass}] edit-spans={len(edits)} skipped={len(skipped)}")

        if args.dry_run:
            for path, bs, be in edits[:60]:
                rel = os.path.relpath(path, REPO)
                print(f"    APPEND-AWAIT {rel} [{bs}:{be}]")
            break

        if not edits:
            print("  fixpoint reached")
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
