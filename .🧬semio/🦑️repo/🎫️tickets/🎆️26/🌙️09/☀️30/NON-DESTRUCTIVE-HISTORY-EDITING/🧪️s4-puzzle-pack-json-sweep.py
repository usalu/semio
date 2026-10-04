#!/usr/bin/env python3
"""🎒️ S4-PUZZLE: moves puzzle 2d/5d off the deleted kernel `dsl::json` / `dsl::os_pack::json` paths onto the extracted
`semio_framework_pack_json` crate, exactly as the peer sweep converted puzzle 3d: `from_json_str` gains the explicit
`JsonMemberPolicy::Reject` argument (inserted before the call's matching parenthesis, string literals respected), every
other function keeps its name, doc references are renamed. Idempotent; `--check` only reports. The three sqlite owners
(S4-INFRA) are never touched."""
import pathlib
import re
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts")
TREES = ("◻️2d", "🖐️5d")
POLICY = ", semio_framework_pack_json::JsonMemberPolicy::Reject"
CALL = re.compile(r"dsl::(?:os_pack::)?json::from_json_str(::<[^()]*?>)?\(")


def closing(text: str, start: int) -> int:
    depth, index = 1, start
    while index < len(text):
        char = text[index]
        if char == "r" and re.match(r'r#*"', text[index:]) and not (text[index - 1].isalnum() or text[index - 1] == "_"):
            hashes = re.match(r"r(#*)\"", text[index:]).group(1)
            end = text.index('"' + hashes, index + len(hashes) + 2)
            index = end + 1 + len(hashes)
            continue
        if char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
            index += 1
            continue
        if char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
            if depth == 0:
                return index
        index += 1
    raise ValueError("unbalanced call")


def convert(text: str) -> str:
    out, cursor = [], 0
    for match in CALL.finditer(text):
        end = closing(text, match.end())
        out.append(text[cursor:match.start()])
        out.append("semio_framework_pack_json::from_json_str" + (match.group(1) or "") + "(" + text[match.end():end] + POLICY + ")")
        cursor = end + 1
    out.append(text[cursor:])
    text = "".join(out)
    text = text.replace("dsl::os_pack::json::", "semio_framework_pack_json::").replace("dsl::json::", "semio_framework_pack_json::")
    return text.replace("`dsl::json`", "`semio_framework_pack_json`")


def main() -> None:
    check = "--check" in sys.argv
    changed = 0
    for tree in TREES:
        for path in sorted((ROOT / tree).rglob("🦀️.rs")):
            if "🪶️sqlite" in path.parts or "node_modules" in path.parts:
                continue
            text = path.read_text(encoding="utf-8")
            if "dsl::json" not in text and "dsl::os_pack::json" not in text:
                continue
            written = convert(text)
            if written != text:
                changed += 1
                if not check:
                    path.write_text(written, encoding="utf-8")
    print(f"{changed} file(s) {'pending' if check else 'converted'}")
    sys.exit(1 if check and changed else 0)


if __name__ == "__main__":
    main()
