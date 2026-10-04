"""🔎️ Lists docstrings of the given source files that do not start with an emoji, and emojis that start more than one.

    python domain_docstring_emojis.py <file>…

A docstring is a `///` or `//!` block (Rust), a `/** … */` block (TypeScript) or a triple-quoted string that opens a
module, class or function (Python). The leading emoji is everything before the first space of its first line. Test
functions without a docstring are not listed; AGENTS.md asks for an emoji only where a docstring exists.
"""

import io
import re
import sys
from collections import defaultdict


def leading(text: str) -> str:
    return text.strip().split(" ", 1)[0]


def emoji(token: str) -> bool:
    return bool(token) and not token[0].isascii() and not token[0].isalnum()


def docstrings(path: str, source: str) -> list[tuple[int, str]]:
    found = []
    lines = source.splitlines()
    if path.endswith(".rs"):
        previous = False
        for number, line in enumerate(lines, 1):
            stripped = line.strip()
            opening = stripped.startswith(("///", "//!"))
            if opening and not previous:
                found.append((number, stripped[3:]))
            previous = opening
    elif path.endswith(".ts") or path.endswith(".tsx"):
        for match in re.finditer(r"/\*\*\s*(.*)", source):
            found.append((source.count("\n", 0, match.start()) + 1, match.group(1)))
    elif path.endswith(".py"):
        for match in re.finditer(r'(?:^|\n)(\s*)(?:def |class )[^\n]*:\n\s*"""(.*)|\A"""(.*)', source):
            found.append((source.count("\n", 0, match.start()) + 2, match.group(2) or match.group(3) or ""))
    return found


def main() -> int:
    failures = 0
    for path in sys.argv[1:]:
        seen = defaultdict(list)
        for number, text in docstrings(path, io.open(path, encoding="utf-8").read()):
            token = leading(text)
            if not emoji(token):
                print(f"{path}:{number}: no leading emoji: {text[:60]!r}")
                failures += 1
            else:
                seen[token].append(number)
        for token, numbers in seen.items():
            if len(numbers) > 1:
                print(f"{path}: {token} starts {len(numbers)} docstrings (lines {numbers})")
                failures += 1
    print(f"{failures} finding(s) in {len(sys.argv) - 1} file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
