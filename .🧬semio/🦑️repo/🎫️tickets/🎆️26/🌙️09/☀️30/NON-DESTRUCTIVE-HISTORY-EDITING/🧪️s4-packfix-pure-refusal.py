"""🪶️ S4-PACKFIX — moves a pure binary codec module from `<prefix>PackError` to `<prefix>PackRefusal`.

Usage: `python3 🧪️s4-packfix-pure-refusal.py [--write] <prefix> <file>...` (e.g. prefix `dsl::`). Collapses
`<prefix>PackError::Refusal(<refusal>)` to `<refusal>` and renames every remaining `<prefix>PackError` to `<prefix>PackRefusal`,
so in-memory readers whose primitives already return `PackRefusal` stop claiming transport ownership. Idempotent.
"""
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]


def closing(text, start):
    depth, index = 0, start
    while True:
        char = text[index]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
            if depth == 0:
                return index
        elif char == '"':
            index += 1
            while text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        index += 1


def convert(text, prefix):
    head = f"{prefix}PackError::Refusal("
    while head in text:
        start = text.index(head)
        end = closing(text, start + len(head) - 1)
        text = text[:start] + text[start + len(head):end] + text[end + 1:]
    return text.replace(f"{prefix}PackError", f"{prefix}PackRefusal")


def main():
    arguments = [arg for arg in sys.argv[1:] if arg != "--write"]
    write, prefix = "--write" in sys.argv[1:], arguments[0]
    for argument in arguments[1:]:
        path = ROOT / argument
        text = path.read_text(encoding="utf-8")
        after = convert(text, prefix)
        print(f"{'converted' if after != text else 'same'} {argument}")
        if write and after != text:
            path.write_text(after, encoding="utf-8")


main()
