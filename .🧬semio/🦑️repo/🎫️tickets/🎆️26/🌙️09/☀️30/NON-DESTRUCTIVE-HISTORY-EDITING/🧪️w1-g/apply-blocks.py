"""🩹 Applies anchored replacement blocks to one source file atomically (one read, one write).

Usage: python3 apply-blocks.py <target-file> <blocks-file> [<blocks-file> ...]
A blocks file holds any number of blocks:
    <<<<<<< ANCHOR
    exact old text (must occur exactly once in the target)
    =======
    replacement text
    >>>>>>> END
Every anchor is checked before anything is written; a missing or ambiguous anchor aborts with no change.
"""
import sys


def parse_blocks(text):
    blocks = []
    rest = text
    while True:
        start = rest.find("<<<<<<< ANCHOR\n")
        if start < 0:
            return blocks
        middle = rest.find("\n=======\n", start)
        end = rest.find("\n>>>>>>> END", middle)
        if middle < 0 or end < 0:
            raise SystemExit("malformed block")
        blocks.append((rest[start + len("<<<<<<< ANCHOR\n"):middle], rest[middle + len("\n=======\n"):end]))
        rest = rest[end + len("\n>>>>>>> END"):]


def main():
    target = sys.argv[1]
    with open(target, encoding="utf-8") as handle:
        source = handle.read()
    blocks = []
    for path in sys.argv[2:]:
        with open(path, encoding="utf-8") as handle:
            blocks.extend(parse_blocks(handle.read()))
    for index, (old, _) in enumerate(blocks):
        count = source.count(old)
        if count != 1:
            raise SystemExit(f"block {index} anchor occurs {count} times: {old[:160]!r}")
    for old, new in blocks:
        source = source.replace(old, new, 1)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(source)
    print(f"applied {len(blocks)} blocks to {target}")


main()
