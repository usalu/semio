#!/usr/bin/env python3
"""🧬️ Wraps every `globalThis.fetch = <handler>` assignment in a `stubFetch(…)` call.

Bun's ambient `fetch` carries `preconnect`, so a bare handler is not a `typeof fetch`. Rather than
casting the stub away, this routes each stub through a helper that keeps the real `preconnect`, so
the global stays a complete `fetch`. The right-hand side is found by scanning to the `;` that closes
the statement at nesting depth zero, tracking strings, templates and comments, so multi-line arrow
bodies are wrapped whole.
"""
import io, sys

OPEN, CLOSE = "([{", ")]}"


def end_of_statement(text: str, start: int) -> int:
    """🧭️ Returns the index of the `;` that ends the assignment beginning at `start`."""
    depth, index = 0, start
    while index < len(text):
        char = text[index]
        if char in "\"'`":
            quote, index = char, index + 1
            while index < len(text):
                if text[index] == "\\":
                    index += 2
                    continue
                if quote == "`" and text.startswith("${", index):
                    depth += 1
                    index += 2
                    quote = ""
                    break
                if text[index] == quote:
                    index += 1
                    break
                index += 1
            if quote == "":
                continue
            continue
        if char in OPEN:
            depth += 1
        elif char in CLOSE:
            depth -= 1
        elif char == ";" and depth == 0:
            return index
        index += 1
    return -1


def main(argv: list[str]) -> int:
    import collections, re

    apply = "--apply" in argv
    wanted = collections.defaultdict(set)
    for raw in sys.stdin:
        m = re.match(r"^(.+?)\((\d+),\d+\): error TS\d+: (.*)$", raw.rstrip("\n"))
        if m and "'preconnect'" in m.group(3):
            wanted[m.group(1)].add(int(m.group(2)))
    total = 0
    for path in sorted(wanted):
        text = io.open(path, encoding="utf-8").read()
        marker, cursor, count = "globalThis.fetch = ", 0, 0
        while True:
            at = text.find(marker, cursor)
            if at < 0:
                break
            rhs = at + len(marker)
            if text.count("\n", 0, at) + 1 not in wanted[path]:
                cursor = rhs
                continue
            if text.startswith("stubFetch(", rhs):
                cursor = rhs
                continue
            end = end_of_statement(text, rhs)
            if end < 0:
                print(f"REFUSED {path} at {at}")
                cursor = rhs
                continue
            text = text[:rhs] + "stubFetch(" + text[rhs:end] + ")" + text[end:]
            count += 1
            cursor = end + len("stubFetch()")
        print(f"{path}: {count}")
        total += count
        if apply and count:
            io.open(path, "w", encoding="utf-8").write(text)
    print(f"{'applied' if apply else 'would apply'} {total} wraps")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
