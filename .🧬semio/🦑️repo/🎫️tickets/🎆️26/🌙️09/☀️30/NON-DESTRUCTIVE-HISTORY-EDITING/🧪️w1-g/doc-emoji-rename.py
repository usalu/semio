"""🎨 Replaces the leading emoji of docstring lines by line number, checking each line still starts with the expected one.

Usage: python3 doc-emoji-rename.py <file> <line>:<old>:<new> [...]
"""
import sys


def main():
    path = sys.argv[1]
    lines = open(path, encoding="utf-8").read().split("\n")
    for spec in sys.argv[2:]:
        number, old, new = spec.split(":")
        index = int(number) - 1
        prefix = lines[index][: len(lines[index]) - len(lines[index].lstrip())]
        expected = f"{prefix}/// {old}"
        if not lines[index].startswith(expected):
            raise SystemExit(f"line {number} does not start with {old!r}: {lines[index][:80]!r}")
        lines[index] = f"{prefix}/// {new}" + lines[index][len(expected):]
    open(path, "w", encoding="utf-8").write("\n".join(lines))
    print(f"renamed {len(sys.argv) - 2} docstring emojis")


main()
