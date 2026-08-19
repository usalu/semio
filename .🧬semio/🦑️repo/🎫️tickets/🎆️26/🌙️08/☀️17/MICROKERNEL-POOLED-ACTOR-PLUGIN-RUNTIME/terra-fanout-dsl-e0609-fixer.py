#!/usr/bin/env python3
"""🩹 Diagnostic-driven `.await` insertion for the specific E0609 shape rustc gives NO suggestion
for: `no field 'X' on type 'impl Future<Output = ...>'` with an empty `children` list (so
`insert-await.py`'s suggestion-based matching finds nothing to apply, even though the fix is
completely unambiguous — the field access is missing `.await` on its receiver).

WHY THIS EXISTS (kernel-fanout-dsl packet, 2026-08-19)
-------------------------------------------------------
`insert-await.py` applies only rustc's own `suggested_replacement`. For this specific error shape
in 🗣️dsl/📖️grammar/component.rs, rustc emits ZERO children/suggestions (confirmed by inspecting the
raw `--message-format=json-diagnostic-rendered-ansi` output) — likely because the surrounding
function already has enough OTHER errors that rustc suppresses the usual "consider awaiting"
help. `insert-await.py` therefore correctly finds nothing to apply here; this is not a bug in it.

This script is diagnostic-driven, per R10: it reads the exact byte span rustc's E0609 diagnostic
flags (the field-name span, e.g. `kind` in `self.peek().kind`) from a captured
`--message-format=json-diagnostic-rendered-ansi` stream, and inserts `.await` immediately before
the `.` that precedes that exact span — nothing is matched by name/regex. It is NOT a generic
`.await` inserter: it only fires on E0609 "no field on Future" diagnostics, and only when the byte
immediately preceding the span is literally `.` (sanity-checked before every edit, and re-verified
against the CURRENT file content — never a stale cached position).

Usage:
    cargo check -p <crate> --message-format=json-diagnostic-rendered-ansi --lib \
        > diag.jsonl   # or --all-targets
    python3 terra-fanout-dsl-e0609-fixer.py --diag diag.jsonl --scope <path> [--apply]

Without --apply, only reports what it would do. Edits are applied per file in DESCENDING byte
order so earlier offsets stay valid, exactly like insert-await.py.
"""
import argparse
import json
import os
import re
import sys
from collections import defaultdict

REPO = "/Users/ueli/Documents/semio"
FIELD_RE = re.compile(r"^no field `([a-zA-Z0-9_]+)` on type `impl Future<Output ?= ?.*>`$")


def in_scope(abs_path: str, scope: str) -> bool:
    rel = os.path.relpath(abs_path, REPO)
    parts = rel.split(os.sep)
    want = [p for p in scope.strip("/").split("/") if p]
    for i in range(len(parts) - len(want) + 1):
        if parts[i:i + len(want)] == want:
            return True
    return False


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--diag", required=True, help="path to captured json-diagnostic-rendered-ansi stream")
    ap.add_argument("--scope", default=None)
    ap.add_argument("--apply", action="store_true")
    args = ap.parse_args()

    candidates = []  # (abs_path, byte_start, byte_end, field_name, line)
    with open(args.diag, encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line.startswith("{"):
                continue
            try:
                msg = json.loads(line)
            except json.JSONDecodeError:
                continue
            if msg.get("reason") != "compiler-message":
                continue
            m = msg.get("message") or {}
            if m.get("level") != "error":
                continue
            if (m.get("code") or {}).get("code") != "E0609":
                continue
            fm = FIELD_RE.match(m.get("message", ""))
            if not fm:
                continue
            for span in m.get("spans", []):
                if not span.get("is_primary"):
                    continue
                path = span.get("file_name", "")
                abs_path = path if os.path.isabs(path) else os.path.join(REPO, path)
                abs_path = os.path.normpath(abs_path)
                if args.scope and not in_scope(abs_path, args.scope):
                    continue
                candidates.append((abs_path, span["byte_start"], span["byte_end"], fm.group(1), span.get("line_start")))

    by_file = defaultdict(list)
    for c in candidates:
        by_file[c[0]].append(c)

    total_applied = 0
    total_skipped = 0
    for path, items in by_file.items():
        with open(path, "rb") as fh:
            data = fh.read()
        items.sort(key=lambda c: c[1], reverse=True)
        seen_spans = set()
        for abs_path, bs, be, field, line in items:
            key = (bs, be)
            if key in seen_spans:
                continue
            seen_spans.add(key)
            dot_pos = bs - 1
            if dot_pos < 0 or data[dot_pos:dot_pos + 1] != b".":
                print(f"  SKIP {os.path.relpath(path, REPO)}:{line} field `{field}` — byte {dot_pos} is not `.` "
                      f"(got {data[max(0,dot_pos-5):dot_pos+5]!r}); leaving untouched", file=sys.stderr)
                total_skipped += 1
                continue
            # sanity: the field name itself must match at [bs:be]
            if data[bs:be].decode("utf-8", "replace") != field:
                print(f"  SKIP {os.path.relpath(path, REPO)}:{line} — span text mismatch, expected `{field}`", file=sys.stderr)
                total_skipped += 1
                continue
            print(f"  {'EDIT' if args.apply else 'WOULD EDIT'} {os.path.relpath(path, REPO)}:{line} "
                  f"insert `.await` before `.{field}`")
            if args.apply:
                data = data[:dot_pos] + b".await" + data[dot_pos:]
            total_applied += 1
        if args.apply:
            with open(path, "wb") as fh:
                fh.write(data)

    print(f"\n{'applied' if args.apply else 'would apply'}: {total_applied}  skipped: {total_skipped}  files: {len(by_file)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
