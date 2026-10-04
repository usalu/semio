#!/usr/bin/env python3
"""🔤️ S4-NORM: norm test sources still call the removed kernel re-exports `dsl::json::from_json_str` / `store::json::from_json_str`
(peer pack-json extraction). Rewrites each call to `semio_framework_pack_json::from_json_str(<arg>, JsonMemberPolicy::Reject)` with a
balanced-parenthesis scan (turbofish kept). `--check` prints the pending count and writes nothing."""
from __future__ import annotations

import re
import sys
from pathlib import Path

NORM = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm")
CALL = re.compile(r"\b(?:dsl|store)::json::from_json_str(?P<turbofish>::<[^>(]*(?:<[^>]*>)?[^>(]*>)?\(")


def rewrite(text: str) -> str:
    out, position = [], 0
    for match in CALL.finditer(text):
        if match.start() < position:
            continue
        depth, index = 1, match.end()
        while depth:
            char = text[index]
            depth += {"(": 1, ")": -1}.get(char, 0)
            index += 1
        argument = text[match.end():index - 1]
        out.append(text[position:match.start()])
        out.append(f"semio_framework_pack_json::from_json_str{match.group('turbofish') or ''}({argument}, semio_framework_pack_json::JsonMemberPolicy::Reject)")
        position = index
    out.append(text[position:])
    return "".join(out)


def main(argv: list[str]) -> int:
    pending = []
    for path in sorted(NORM.rglob("🦀️.rs")):
        text = path.read_text()
        new = rewrite(text)
        if new != text:
            pending.append((path, new))
    if "--check" not in argv:
        for path, new in pending:
            path.write_text(new)
    print(("pending=" if "--check" in argv else "written=") + str(len(pending)))
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv))
