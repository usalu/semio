#!/usr/bin/env python3
"""🔗 Diagnostic-driven `.await` insertion for E0271 chain-mismatch errors.

WHY THIS EXISTS
---------------
`insert-await.py` deliberately excludes E0271 from AWAIT_CODES because rustc's own "help: consider
`await`ing on the `Future`" suggestion for this shape is placed on the WRONG token — it suggests
awaiting the enclosing array/collection literal (`[ActionArgDef::text(..).await.required()].await`),
which does not compile (`[T; N]` never implements `IntoFuture`). Verified by hand on
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs:985` before writing this.

The CORRECT fix is different: E0271 here always means one more method in a builder chain
(`.required()`, `.default_value(..)`, etc.) is itself `async fn` and its own return value was never
awaited before being placed in the collection. rustc already computes the exact span of that
un-awaited call in every E0271 diagnostic's "note: calling an async function returns a future" child
— that span's `byte_end` is exactly where `.await` belongs. This script inserts there, and nowhere
else.

SAFETY PROPERTIES (same discipline as insert-await.py)
-------------------------------------------------------
* Diagnostic-driven, byte-offset-keyed, never name/regex-keyed (R10).
* One edit per (file, byte_end), de-duplicated; applied per file in descending-offset order so
  earlier offsets stay valid.
* `--scope` restricts to path segments, honouring packet ownership.

USAGE
-----
    python3 terra-lastgate-chain-await-fixer.py --crate semio-framework --scope <path> --dry-run
    python3 terra-lastgate-chain-await-fixer.py --crate semio-framework --scope <path> --apply
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    if not want:
        return True
    for i in range(len(parts) - len(want) + 1):
        if parts[i:i + len(want)] == want:
            return True
    return False


def run_check(crate: str, target_dir: str, all_targets: bool = False) -> list[dict]:
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


def collect_edits(diags: list[dict], scope: str | None):
    """🔎 For every E0271 error, find its 'calling an async function returns a future' note child
    and record an insertion of `.await` at that span's byte_end."""
    edits, other = [], []
    for diag in diags:
        if diag.get("level") != "error":
            continue
        code = (diag.get("code") or {}).get("code") or ""
        if code != "E0271":
            continue
        primary = ""
        for sp in diag.get("spans", []):
            if sp.get("is_primary"):
                primary = f'{sp.get("file_name")}:{sp.get("line_start")}'
                break
        note = None
        for child in diag.get("children", []):
            if child.get("level") == "note" and "returns a future" in (child.get("message") or ""):
                note = child
                break
        if note is None:
            other.append((primary, code, "no 'returns a future' note child"))
            continue
        nsp = next((s for s in note.get("spans", []) if s.get("is_primary")), None)
        if nsp is None:
            other.append((primary, code, "note child has no primary span"))
            continue
        path = nsp.get("file_name", "")
        abs_path = path if os.path.isabs(path) else os.path.join(REPO, path)
        abs_path = os.path.normpath(abs_path)
        if scope and not in_scope(abs_path, scope):
            other.append((primary, code, "out-of-scope"))
            continue
        edits.append((abs_path, nsp["byte_end"], nsp["byte_end"], ".await", f"{code} @ {primary}"))
    return edits, other


def apply_edits(edits, guard: set) -> int:
    by_file = defaultdict(list)
    for path, bs, be, repl, why in edits:
        key = (path, bs, be)
        if key in guard:
            continue
        guard.add(key)
        by_file[path].append((bs, be, repl, why))

    applied = 0
    for path, items in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        items.sort(key=lambda t: t[0], reverse=True)
        taken, last_start = [], None
        for bs, be, repl, why in items:
            if last_start is not None and be > last_start:
                continue
            taken.append((bs, be, repl, why))
            last_start = bs
        for bs, be, repl, _why in taken:
            data = data[:bs] + repl.encode("utf-8") + data[be:]
        with open(path, "wb") as fh:
            fh.write(data)
        applied += len(taken)
    return applied


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--crate", required=True)
    ap.add_argument("--apply", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--target-dir", required=True)
    ap.add_argument("--max-passes", type=int, default=8)
    ap.add_argument("--all-targets", action="store_true")
    args = ap.parse_args()
    if not args.apply and not args.dry_run:
        ap.error("choose --apply or --dry-run")

    guard: set = set()
    for npass in range(1, args.max_passes + 1):
        diags = run_check(args.crate, args.target_dir, args.all_targets)
        errors = [d for d in diags if d.get("level") == "error"]
        edits, other = collect_edits(diags, args.scope)
        print(f"[pass {npass}] errors={len(errors)} E0271-edits={len(edits)} other={len(other)}")
        if args.dry_run:
            for path, bs, be, repl, why in edits[:40]:
                print(f"    EDIT {os.path.relpath(path, REPO)} @{be} -> insert {repl!r}   ({why})")
            for p, c, m in other[:20]:
                print(f"    SKIP {c} {p}: {m}")
            break
        if not edits:
            print("  fixpoint reached")
            break
        applied = apply_edits(edits, guard)
        print(f"  applied {applied}")
        if applied == 0:
            print("  no progress — stopping")
            break
    return 0


if __name__ == "__main__":
    sys.exit(main())
