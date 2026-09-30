"""🔎 Lists the leading emoji of every docstring block inside the named `//#region` blocks of one file and reports duplicates.

Usage: python3 doc-emoji-audit.py <file> <region-name> [<region-name> ...]
"""
import re
import sys
from collections import defaultdict


def regions(lines, names):
    spans = []
    for name in names:
        start = next(index for index, line in enumerate(lines) if line.strip() == f"//#region {name}")
        end = next(index for index in range(start, len(lines)) if lines[index].strip() == f"//#endregion {name}")
        spans.append((name, start, end))
    return spans


def leading(text):
    match = re.match(r"^\s*///\s*(\S+)", text)
    return match.group(1) if match else None


def main():
    path, names = sys.argv[1], sys.argv[2:]
    lines = open(path, encoding="utf-8").read().split("\n")
    seen = defaultdict(list)
    for name, start, end in regions(lines, names):
        previous_doc = False
        for index in range(start, end):
            is_doc = lines[index].lstrip().startswith("///")
            if is_doc and not previous_doc:
                seen[leading(lines[index])].append(index + 1)
            previous_doc = is_doc
    duplicates = {emoji: where for emoji, where in seen.items() if len(where) > 1}
    print(f"{sum(len(where) for where in seen.values())} doc blocks, {len(seen)} distinct, {len(duplicates)} duplicated")
    for emoji, where in sorted(duplicates.items(), key=lambda item: item[1][0]):
        print(emoji, where)


main()
