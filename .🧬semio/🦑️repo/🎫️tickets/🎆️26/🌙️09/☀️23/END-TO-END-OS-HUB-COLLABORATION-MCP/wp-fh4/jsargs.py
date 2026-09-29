"""✂️ Top-level argument spans of a TS/JS call (strings, template literals with `${}` nesting, brackets) and a call rewriter
that drops one argument — used by FH4's TS-twin edits."""
from __future__ import annotations

import re


def arguments(text: str, open_at: int) -> tuple[list[tuple[int, int]], int]:
    """✂️ Argument spans of the call whose `(` is at `open_at`, and the index after its `)`."""
    depth, at, start, spans = 0, open_at, open_at + 1, []
    stack: list[str] = []
    while at < len(text):
        char = text[at]
        if stack and stack[-1] == "`":
            if char == "\\":
                at += 2
                continue
            if char == "`":
                stack.pop()
            elif text.startswith("${", at):
                stack.append("${")
                at += 2
                continue
            at += 1
            continue
        if char in "\"'":
            at += 1
            while text[at] != char:
                at += 2 if text[at] == "\\" else 1
        elif char == "`":
            stack.append("`")
        elif char in "([{":
            if char == "{" and stack and stack[-1] == "${":
                stack.append("{")
            depth += 1
        elif char in ")]}":
            if char == "}" and stack and stack[-1] == "${":
                stack.pop()
                at += 1
                continue
            if char == "}" and stack and stack[-1] == "{":
                stack.pop()
            depth -= 1
            if depth == 0:
                spans.append((start, at))
                return [span for span in spans if text[span[0]:span[1]].strip()], at + 1
        elif char == "," and depth == 1 and not stack:
            spans.append((start, at))
            start = at + 1
        at += 1
    raise ValueError(f"unbalanced call at {open_at}")


def drop_argument(text: str, name: str, index: int) -> tuple[str, int]:
    """🗑️ Removes argument `index` from every `name(` call (not its definition `const name =`)."""
    edits = []
    for match in re.finditer(r"(?<![\w.$])" + re.escape(name) + r"\(", text):
        if re.search(r"(const|function)\s+$", text[:match.start()][-20:]):
            continue
        spans, end = arguments(text, match.end() - 1)
        if len(spans) <= index:
            continue
        keep = [text[a:b].strip() for i, (a, b) in enumerate(spans) if i != index]
        edits.append((match.end(), end - 1, ", ".join(keep)))
    for start, stop, new in sorted(edits, reverse=True):
        text = text[:start] + new + text[stop:]
    return text, len(edits)
