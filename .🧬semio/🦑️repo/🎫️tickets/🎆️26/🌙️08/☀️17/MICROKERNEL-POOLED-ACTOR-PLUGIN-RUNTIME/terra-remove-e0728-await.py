#!/usr/bin/env python3
"""✂️ Compiler-driven removal of `.await` inside a now-sync function — E0728 sibling of remove-bad-await.py.

WHY THIS EXISTS
----------------
`terra-green-codec-r9.py` R9-reverted 9,947 top-level codec/computation helpers in
`semio-s-plugin-stdio` (see 📓️terra-stdio-green-report.md). It deliberately never touches a call
site (R10 discipline — this is a signature-only edit). The fallout is `.await` sitting on an
expression whose value is no longer a future, INSIDE a caller that itself may now also be sync
(because the caller was itself one of the 9,947 reverted functions, or because it was always sync
and had been legally awaiting a genuinely-async callee that this pass just made sync).

`remove-bad-await.py` (this ticket's existing tool) only matches `E0277 "X is not a future"`. Once
the CALLER is non-async, rustc reports the identical shape as `E0728 "await is only allowed inside
async functions and blocks"` instead — same defect, different diagnostic code, because the syntactic
rule `.await` requires an enclosing `async` context fires before the type-level "is this a future"
check ever runs. This tool is `remove-bad-await.py`'s E0728 counterpart: same span-keyed mechanics,
same safety properties, different diagnostic code.

WHAT IT DOES
------------
Parses `cargo check --message-format=json`, finds every `E0728` whose message is exactly `` `await`
is only allowed inside `async` functions and blocks `` and whose PRIMARY span text is exactly
`await`, and deletes that span plus the `.` immediately before it. Nothing else.

SAFETY PROPERTIES (identical discipline to remove-bad-await.py / R10)
-----------------------------------------------------------------------
* Span-keyed, never name-keyed. Byte offsets, not line/column.
* Edits applied per file in DESCENDING offset order so earlier offsets stay valid.
* A guard set means a span is never edited twice across passes.
* `--scope` matches PATH SEGMENTS, never a substring.
* `--max-files` blast-radius guard.
* Fixpoint loop: removing one `.await` can reveal the next (e.g. a chained
  `.foo().await.bar().await`).
* Refuses (never guesses) if the byte range is not literally `await`, or the preceding byte is not
  `.` — reported, not silently skipped.

USAGE
-----
    python3 terra-remove-e0728-await.py --crate semio-s-plugin-stdio --target-dir <dir> --dry-run
    python3 terra-remove-e0728-await.py --crate semio-s-plugin-stdio --target-dir <dir> --apply \
        --scope '✏️s/🔌️plugins/🗄️stdio' --max-files 2500
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"


def run_check(crate: str, target_dir: str, all_targets: bool) -> list[dict]:
    cmd = ["cargo", "check", "-p", crate, "--message-format=json",
           "--all-targets" if all_targets else "--lib"]
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


def collect(diags, scope):
    edits, skipped = [], []
    for d in diags:
        if d.get("level") != "error":
            continue
        code = (d.get("code") or {}).get("code") or ""
        msg = d.get("message") or ""
        if code != "E0728":
            continue
        if "only allowed inside" not in msg:
            continue
        for sp in d.get("spans", []):
            if not sp.get("is_primary"):
                continue
            fn = sp.get("file_name", "")
            path = os.path.normpath(fn if os.path.isabs(fn) else os.path.join(REPO, fn))
            if scope and not in_scope(path, scope):
                skipped.append((path, "out-of-scope"))
                continue
            edits.append((path, sp["byte_start"], sp["byte_end"], msg[:60]))
    return edits, skipped


def apply_edits(edits, guard):
    by_file = defaultdict(list)
    for path, bs, be, why in edits:
        if (path, bs, be) in guard:
            continue
        guard.add((path, bs, be))
        by_file[path].append((bs, be, why))

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
        for bs, be, _why in items:
            if last_start is not None and be > last_start:
                continue
            if data[bs:be] != b"await":
                refused.append((path, f"span at {bs} is {data[bs:be]!r}, not b'await'"))
                continue
            if bs == 0 or data[bs - 1:bs] != b".":
                refused.append((path, f"byte before {bs} is not '.', left alone"))
                continue
            data = data[:bs - 1] + data[be:]
            last_start = bs
            applied += 1
        with open(path, "wb") as fh:
            fh.write(data)
    return applied, refused


def main() -> int:
    ap = argparse.ArgumentParser(description="Remove `.await` in non-async fns (E0728), driven by rustc")
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--max-passes", type=int, default=12)
    ap.add_argument("--max-files", type=int, default=2500)
    ap.add_argument("--all-targets", action="store_true")
    ap.add_argument("--target-dir", required=True)
    args = ap.parse_args()
    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    guard: set = set()
    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir, args.all_targets)
        errors = [d for d in diags if d.get("level") == "error"]
        edits, skipped = collect(diags, args.scope)
        print(f"[pass {npass}] errors={len(errors)} e0728-await={len(edits)} out-of-scope={len(skipped)}")
        if args.dry_run:
            for path, bs, be, why in edits[:40]:
                print(f"    REMOVE {os.path.relpath(path, REPO)} [{bs}:{be}]  ({why})")
            break
        if not edits:
            print("  fixpoint reached")
            break
        touched = {e[0] for e in edits}
        if args.max_files and len(touched) > args.max_files:
            print(f"  ABORT: would edit {len(touched)} files, over --max-files={args.max_files}")
            break
        applied, refused = apply_edits(edits, guard)
        print(f"  removed {applied}")
        for path, why in refused[:10]:
            print(f"    REFUSED {os.path.relpath(path, REPO)}: {why}")
        if applied == 0:
            print("  no progress — stopping")
            break
    return 0


if __name__ == "__main__":
    sys.exit(main())
