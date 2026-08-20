#!/usr/bin/env python3
"""🪲️ Span-keyed (R10) fixer for the `sdk-dropped-futures` packet's 97 "unused implementer of
`std::future::Future`" warnings on `semio-framework-plugin`. Adapted from the already-vetted
`terra-oskat-unused-future-fixer.py` (operates purely in BYTES throughout — reads bytes, indexes
bytes, writes bytes — so it does NOT repeat the `terra-hard-unawaited-future-fixer.py` byte/codepoint
confusion bug documented in `📓️terra-alltargets-hard-report.md`).

Every one of the 97 sites was hand-reviewed (see `📓️terra-sdk-dropped-futures-report.md`) and sorted
into two buckets:
  - 90 are a plain dropped `.await` on an already-`async fn` caller — this tool appends `.await`
    right before the statement's trailing `;` (or at the tail-expression's end if there is none).
  - 7 need special handling this tool DELIBERATELY EXCLUDES (a plain `.await` there is either E0728
    "await inside a sync closure" or E0733 "recursion in an async fn requires boxing"): jobs.rs
    lines 390/391 (inside a `LocalKey::with` sync closure — fixed by hand via the file's own
    established `resolve_ready` bridge) and component.rs lines 245/11957/11962/11967/11971 (self-
    recursive async fns — fixed by hand via `Box::pin(...).await`).
"""
import json
import sys
from pathlib import Path
from collections import defaultdict

REPO_ROOT = Path("/Users/ueli/Documents/semio")

# (file substring, line number) pairs to skip — hand-fixed separately, see docstring.
EXCLUDE = {
    ("💼️jobs/🦀️component.rs", 390),
    ("💼️jobs/🦀️component.rs", 391),
    ("🔌️plugin/🦀️component.rs", 245),
    ("🔌️plugin/🦀️component.rs", 11957),
    ("🔌️plugin/🦀️component.rs", 11962),
    ("🔌️plugin/🦀️component.rs", 11967),
    ("🔌️plugin/🦀️component.rs", 11971),
}


def is_excluded(file_name: str, line_start: int) -> bool:
    for suffix, line in EXCLUDE:
        if file_name.endswith(suffix) and line_start == line:
            return True
    return False


def collect(json_path: str):
    by_file = defaultdict(list)
    skipped = []
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
                if is_excluded(sp["file_name"], sp["line_start"]):
                    skipped.append((sp["file_name"], sp["line_start"]))
                    continue
                by_file[sp["file_name"]].append((sp["byte_start"], sp["byte_end"]))
    return by_file, skipped


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
    by_file, skipped = collect(json_path)
    total = 0
    for rel_path, spans in sorted(by_file.items()):
        n = fix_file(rel_path, spans)
        total += n
        print(f"{n:4d}  {rel_path}")
    print(f"TOTAL fixed: {total}")
    print(f"Excluded (hand-fixed separately): {len(skipped)}")
    for f, l in skipped:
        print(f"  SKIP {f}:{l}")


if __name__ == "__main__":
    main()
