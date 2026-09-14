"""🧹️ W1-B test surgery helper: removes named top-level (or module-level) Rust items with their doc comments
and attributes, using a string-, char- and comment-aware brace matcher.

Usage: python3 🐍️w1b-rust-items.py <file.rs> <item-name> [<item-name> ...]
"""
import re
import sys


def item_end(text: str, start: int) -> int:
    depth = 0
    index = start
    seen_brace = False
    nesting = 0
    length = len(text)
    while index < length:
        char = text[index]
        if text.startswith("//", index):
            index = text.index("\n", index)
            continue
        if text.startswith("/*", index):
            index = text.index("*/", index) + 2
            continue
        raw = re.match(r'r(#*)"', text[index:index + 8])
        if raw and (index == 0 or not (text[index - 1].isalnum() or text[index - 1] == "_")):
            closing = '"' + raw.group(1)
            index = text.index(closing, index + len(raw.group(0))) + len(closing)
            continue
        if char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
            index += 1
            continue
        if char == "'":
            literal = re.match(r"'(\\.|[^\\'])'", text[index:index + 12]) or re.match(r"'\\u\{[0-9a-fA-F]+\}'", text[index:index + 12])
            if literal:
                index += len(literal.group(0))
                continue
        if char in "([":
            nesting += 1
        elif char in ")]":
            nesting -= 1
        elif char == "{":
            depth += 1
            seen_brace = True
        elif char == "}":
            depth -= 1
            if seen_brace and depth == 0:
                end = index + 1
                if text.startswith("\n", end):
                    end += 1
                return end
        elif char == ";" and not seen_brace and depth == 0 and nesting == 0:
            end = index + 1
            if text.startswith("\n", end):
                end += 1
            return end
        index += 1
    raise ValueError("unterminated item")


def remove_item(text: str, name: str) -> str:
    pattern = re.compile(r"^(?P<indent>[ \t]*)(?:pub(?:\([a-z]+\))? )?(?:async )?(?:fn|struct|enum|const|static|type|mod|trait) " + re.escape(name) + r"\b", re.M)
    match = pattern.search(text)
    if not match:
        raise SystemExit(f"item not found: {name}")
    start = match.start()
    lines_before = text[:start].split("\n")
    lines_before.pop()
    while lines_before and re.match(r"^[ \t]*(///|#\[|//)", lines_before[-1]):
        lines_before.pop()
    block_start = len("\n".join(lines_before)) + (1 if lines_before else 0)
    end = item_end(text, match.end())
    removed = text[:block_start] + text[end:]
    return re.sub(r"\n{3,}", "\n\n", removed)


def main() -> None:
    path = sys.argv[1]
    text = open(path, encoding="utf-8").read()
    for name in sys.argv[2:]:
        text = remove_item(text, name)
    open(path, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    main()
