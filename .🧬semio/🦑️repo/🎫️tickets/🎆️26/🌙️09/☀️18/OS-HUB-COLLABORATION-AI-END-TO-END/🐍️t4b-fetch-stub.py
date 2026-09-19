#!/usr/bin/env python3
"""🌐️ Routes every `globalThis.fetch = …` stub through the shared `stubFetch` helper.

Two forms occur: a parenthesised handler cast with `as typeof fetch` / `as typeof globalThis.fetch`,
and a bare `vi.fn(…)` mock. Both are rewritten to `stubFetch(<handler>)`, which keeps Bun's real
`preconnect` instead of casting the difference away. The right-hand side is found by scanning to the
`;` that closes the statement at nesting depth zero, tracking strings and templates, so multi-line
arrow bodies are wrapped whole. Lines are taken from tsc's own capture, so nothing is guessed.
"""
import io, re, sys

OPEN, CLOSE = "([{", ")]}"
MARKER = "globalThis.fetch = "
CAST = re.compile(r"\s+as typeof (?:globalThis\.)?fetch$")


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
                if text[index] == quote:
                    index += 1
                    break
                index += 1
            continue
        if char in OPEN:
            depth += 1
        elif char in CLOSE:
            depth -= 1
        elif char == ";" and depth == 0:
            return index
        index += 1
    return -1


def matching_close(text: str, start: int) -> int:
    """🔗️ Returns the index of the bracket closing the one at `start`, or -1."""
    depth, index = 0, start
    while index < len(text):
        char = text[index]
        if char in "\"'`":
            quote, index = char, index + 1
            while index < len(text):
                if text[index] == "\\":
                    index += 2
                    continue
                if text[index] == quote:
                    index += 1
                    break
                index += 1
            continue
        if char in OPEN:
            depth += 1
        elif char in CLOSE:
            depth -= 1
            if depth == 0:
                return index
        index += 1
    return -1


def rewrite(path: str, apply: bool) -> int:
    """🩹️ Wraps every stub assignment in the file and reports how many changed."""
    text = io.open(path, encoding="utf-8").read()
    cursor, count = 0, 0
    while True:
        at = text.find(MARKER, cursor)
        if at < 0:
            break
        rhs = at + len(MARKER)
        if text.startswith("stubFetch(", rhs) or text.startswith("original", rhs) or text.startswith("resolvedFetch", rhs):
            cursor = rhs
            continue
        end = end_of_statement(text, rhs)
        if end < 0:
            print(f"REFUSED {path} at {at}")
            cursor = rhs
            continue
        handler = text[rhs:end]
        stripped, cast = CAST.subn("", handler)
        if not cast and not stripped.startswith(("vi.fn(", "vitest.vi.fn(")):
            cursor = end
            continue
        if stripped.startswith("(") and stripped.endswith(")") and matching_close(stripped, 0) == len(stripped) - 1:
            stripped = stripped[1:-1]
        text = text[:rhs] + "stubFetch(" + stripped + ")" + text[end:]
        count += 1
        cursor = rhs + len("stubFetch(") + len(stripped) + 1
    print(f"{path}: {count}")
    if apply and count:
        io.open(path, "w", encoding="utf-8").write(text)
    return count


def main(argv: list[str]) -> int:
    apply = "--apply" in argv
    total = sum(rewrite(path, apply) for path in argv[1:] if not path.startswith("--"))
    print(f"{'applied' if apply else 'would apply'} {total} wraps")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
