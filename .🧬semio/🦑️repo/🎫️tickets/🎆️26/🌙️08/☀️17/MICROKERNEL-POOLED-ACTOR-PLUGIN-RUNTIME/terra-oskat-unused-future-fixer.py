#!/usr/bin/env python3
"""🪲️ Span-keyed (R10) fixer for rustc's `unused_must_use` "unused implementer of `Future`
that must be used" warning — the exact "silent no-op" defect class this ticket's brief calls
out as highest-value: a call to a now-async helper (assertion helpers, byte-writers, etc.)
left un-awaited compiles clean but the callee's whole body silently never runs.

Reads a `cargo check --message-format=json` capture, collects every such warning's PRIMARY
span (always a bare expression-statement — one call, no binding), and inserts `.await` right
before the statement's trailing `;` if the span includes it, else appends `.await` directly at
the span's end (a semicolon-less tail expression). Processes each file back-to-front by byte
offset so earlier insertions never invalidate later spans' offsets, and skips (does not touch)
any span whose text already ends in `.await` or `.await;` (already fixed by an earlier pass).
"""
import json
import sys
from pathlib import Path
from collections import defaultdict

REPO_ROOT = Path("/Users/ueli/Documents/semio")


def collect(json_path: str):
    by_file = defaultdict(list)
    for line in open(json_path, encoding="utf-8"):
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            m = json.loads(line)
        except Exception:
            continue
        if m.get("reason") != "compiler-message":
            continue
        msg = m.get("message")
        if not msg or msg.get("level") != "warning":
            continue
        if "unused implementer of" not in msg.get("message", ""):
            continue
        for sp in msg.get("spans", []):
            if sp.get("is_primary"):
                by_file[sp["file_name"]].append((sp["byte_start"], sp["byte_end"]))
    return by_file


def fix_file(rel_path: str, spans: list) -> int:
    path = REPO_ROOT / rel_path
    data = path.read_bytes()
    spans = sorted(set(spans), key=lambda s: -s[0])  # back-to-front, deduped
    n = 0
    for byte_start, byte_end in spans:
        chunk = data[byte_start:byte_end]
        if chunk.rstrip().endswith(b".await") or chunk.rstrip(b";").rstrip().endswith(b".await"):
            continue  # already fixed
        if data[byte_end - 1 : byte_end] == b";":
            insert_at = byte_end - 1
        else:
            insert_at = byte_end
        data = data[:insert_at] + b".await" + data[insert_at:]
        n += 1
    if n:
        path.write_bytes(data)
    return n


def main():
    json_path = sys.argv[1]
    by_file = collect(json_path)
    total = 0
    for rel_path, spans in sorted(by_file.items()):
        n = fix_file(rel_path, spans)
        total += n
        print(f"{n:4d}  {rel_path}")
    print(f"TOTAL fixed: {total}")


if __name__ == "__main__":
    main()
