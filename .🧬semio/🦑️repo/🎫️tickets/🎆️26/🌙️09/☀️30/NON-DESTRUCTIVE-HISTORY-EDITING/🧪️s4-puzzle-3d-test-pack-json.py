#!/usr/bin/env python3
"""🧪️ S4-PUZZLE: finishes the peer value/DSL sweep in the puzzle 3d editor TESTS (the libs were converted, the tests not):
the bare `json::` module path of the deleted `use dsl::json;` becomes `semio_framework_pack_json::`, every one-argument
`parse(text)` / `from_json_str(text)` of `semio_framework_pack_json` gains the explicit `JsonMemberPolicy::Reject`, and `protocol::{Terminology,
Locale}` (no longer re-exported) become `semio_framework_ui_locale::{Terminology, Locale}`. Idempotent; `--check` reports."""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor")
FILES = ("🧪️tests/🔬️unit/🦀️.rs", "🧪️tests/🔬️mutation-latency/🦀️.rs", "📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs")
POLICY = ", semio_framework_pack_json::JsonMemberPolicy::Reject"
BARE_PARSE = re.compile(r"(?<![\w.:])(?:parse|from_json_str)\(")
BARE_JSON = re.compile(r"(?<![\w:])json::")


def call_end(text: str, start: int) -> tuple[int, bool]:
    depth, index, comma = 1, start, False
    while index < len(text):
        char = text[index]
        if char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        elif char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
            if depth == 0:
                return index, comma
        elif char == "," and depth == 1:
            comma = True
        elif char == "|" and depth == 1:
            pass
        index += 1
    raise ValueError("unbalanced call")


def convert(text: str) -> str:
    out, cursor = [], 0
    for match in BARE_PARSE.finditer(text):
        if match.start() < cursor:
            continue
        end, comma = call_end(text, match.end())
        if comma:
            continue
        out.append(text[cursor:end] + POLICY)
        cursor = end
    out.append(text[cursor:])
    text = BARE_JSON.sub("semio_framework_pack_json::", "".join(out))
    return text.replace("protocol::Terminology::", "semio_framework_ui_locale::Terminology::").replace("protocol::Locale::", "semio_framework_ui_locale::Locale::")


def main() -> None:
    check = "--check" in sys.argv
    changed = 0
    for relative in FILES:
        path = ROOT / relative
        text = path.read_text(encoding="utf-8")
        written = convert(text)
        if written != text:
            changed += 1
            print(f"{'pending' if check else 'converted'} {relative}")
            if not check:
                path.write_text(written, encoding="utf-8")
    print(f"{changed} file(s) {'pending' if check else 'converted'}")
    sys.exit(1 if check and changed else 0)


if __name__ == "__main__":
    main()
