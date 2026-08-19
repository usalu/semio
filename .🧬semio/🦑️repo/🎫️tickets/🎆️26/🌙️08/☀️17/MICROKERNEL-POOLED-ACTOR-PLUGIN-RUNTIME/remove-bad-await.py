#!/usr/bin/env python3
"""✂️ Compiler-driven removal of `.await` on things that are NOT futures — the inverse of insert-await.py.

WHY THIS EXISTS
---------------
The universal-async conversion runs in both directions. When a function is correctly reverted to sync
under **R9** (a pure computation whose consumers are language-barred from being async), every call site
that had already gained `.await` now reads `not_a_future.await` — rustc's `E0277: X is not a future`.

There can be hundreds of these after one R9 reversion, and they are *not* something `insert-await.py`
can help with: that tool only ever adds. Doing it with a text pattern is exactly the R10 trap — the
call shapes vary (nested parens, method chains, turbofish), a regex that matches most of them silently
misses the rest, and a regex keyed on the *function name* collides with std methods.

WHAT IT DOES
------------
Parses `cargo check --message-format=json`, finds every `E0277 ... is not a future`, and deletes the
byte span rustc itself points at — which is precisely the `await` token — plus the `.` immediately
before it. Nothing else. If the byte before the span is not `.`, the site is left alone and reported,
because that means the shape is not what we assumed.

SAFETY PROPERTIES (same discipline as insert-await.py)
------------------------------------------------------
* **Span-keyed, never name-keyed** (R10). It cannot touch a site rustc did not flag.
* Byte offsets, not line/column — this repo is full of multi-byte emoji in paths and source.
* Edits applied per file in DESCENDING offset order so earlier offsets stay valid.
* A guard set means a span is never edited twice.
* `--scope` matches PATH SEGMENTS, never a substring, so a run cannot wander into another packet's
  files (a substring scope once reached into 314 files on this ticket).
* `--max-files` blast-radius guard.
* Fixpoint loop: removing one `.await` can reveal the next.

USAGE
-----
    python3 remove-bad-await.py --crate semio-framework-os-kernel --dry-run
    python3 remove-bad-await.py --crate semio-framework-os-kernel --apply --scope '🔨️modules/📡️spr'
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
    """🩺 Run cargo check and return parsed JSON diagnostics."""
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
    """🎯 Path-SEGMENT containment, never a substring match."""
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    if not want:
        return True
    return any(parts[i:i + len(want)] == want for i in range(len(parts) - len(want) + 1))


def collect(diags, scope):
    """🔎 Every `X is not a future` primary span — that span IS the `await` token."""
    edits, skipped = [], []
    for d in diags:
        if d.get("level") != "error":
            continue
        code = (d.get("code") or {}).get("code") or ""
        msg = d.get("message") or ""
        if code != "E0277" or "is not a future" not in msg:
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
    """✍️ Delete `.await`: the flagged span plus the `.` immediately preceding it."""
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
                continue  # overlapping; next pass
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
    ap = argparse.ArgumentParser(description="Remove `.await` on non-futures, driven by rustc")
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None, help="repo-relative path, matched on PATH SEGMENTS")
    ap.add_argument("--max-passes", type=int, default=8)
    ap.add_argument("--max-files", type=int, default=60)
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
        print(f"[pass {npass}] errors={len(errors)} bad-await={len(edits)} out-of-scope={len(skipped)}")
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
