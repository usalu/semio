#!/usr/bin/env python3
"""🧪️ S4-BUMP: a small Rust-aware field/argument remover for the store description wave (design §20.6, wave B).

It tokenizes just enough Rust (line/block comments, normal/raw/byte strings, char literals vs lifetimes) to match brackets and split
top-level commas, then removes:
- `description: <expr>` / shorthand `description` members of struct literals and patterns whose head path ends with one of the given
  type names (the replication `Edit` only when the same braces also name `forwards` or `mutation_meta`, so domain `Edit` commands stay);
- the positional argument at a given index from calls of the given method/function names.
Every removal is reported (file:line); `--apply` writes, otherwise it is a dry run.
"""
from __future__ import annotations
import pathlib
import re
import sys

OPEN = {"(": ")", "[": "]", "{": "}"}
CLOSE = {")", "]", "}"}


def code_mask(text: str) -> list[bool]:
    """🎭️ True for every character that is Rust code (not inside a comment, string or char literal)."""
    mask = [True] * len(text)
    i, n = 0, len(text)
    while i < n:
        c = text[i]
        if text.startswith("//", i):
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                mask[k] = False
            i = j
            continue
        if text.startswith("/*", i):
            depth, j = 1, i + 2
            while j < n and depth:
                if text.startswith("/*", j):
                    depth, j = depth + 1, j + 2
                elif text.startswith("*/", j):
                    depth, j = depth - 1, j + 2
                else:
                    j += 1
            for k in range(i, j):
                mask[k] = False
            i = j
            continue
        raw = re.match(r'b?r(#*)"', text[i:i + 12])
        if raw and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = raw.group(1)
            end = text.find('"' + hashes, i + raw.end())
            j = n if end < 0 else end + 1 + len(hashes)
            for k in range(i, j):
                mask[k] = False
            i = j
            continue
        if c == '"' or (c == "b" and text.startswith('b"', i) and (i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_"))):
            j = i + (2 if c == "b" else 1)
            while j < n and text[j] != '"':
                j += 2 if text[j] == "\\" else 1
            j += 1
            for k in range(i, min(j, n)):
                mask[k] = False
            i = j
            continue
        if c == "'":
            literal = re.match(r"'(?:\\(?:x[0-9a-fA-F]{2}|u\{[0-9a-fA-F]+\}|.)|[^\\'\n])'", text[i:i + 14])
            if literal:
                for k in range(i, i + literal.end()):
                    mask[k] = False
                i += literal.end()
                continue
        i += 1
    return mask


def matching(text: str, mask: list[bool], start: int) -> int:
    """🔚️ Index of the bracket closing the one at `start`."""
    stack = [OPEN[text[start]]]
    i = start + 1
    while i < len(text):
        if mask[i]:
            c = text[i]
            if c in OPEN:
                stack.append(OPEN[c])
            elif c in CLOSE:
                if c != stack.pop():
                    raise ValueError(f"unbalanced bracket at {i}")
                if not stack:
                    return i
        i += 1
    raise ValueError(f"unclosed bracket from {start}")


def top_level_items(text: str, mask: list[bool], open_at: int, close_at: int) -> list[tuple[int, int]]:
    """✂️ (start, end) spans of the top-level comma-separated items between two brackets (end excludes the comma)."""
    items, depth, item_start, i = [], 0, open_at + 1, open_at + 1
    while i < close_at:
        if mask[i]:
            c = text[i]
            if c in OPEN:
                depth += 1
            elif c in CLOSE:
                depth -= 1
            elif c == "," and depth == 0:
                items.append((item_start, i))
                item_start = i + 1
        i += 1
    if text[item_start:close_at].strip():
        items.append((item_start, close_at))
    return items


def remove_item(text: str, items: list[tuple[int, int]], index: int, close_at: int) -> tuple[int, int]:
    """🧹️ The span to delete for item `index`: up to the next item's start, or (for the last item) from the previous comma on."""
    start, end = items[index]
    if index + 1 < len(items):
        return start, items[index + 1][0]
    trimmed = start + len(text[start:end].rstrip())
    if index > 0:
        return items[index - 1][1], trimmed
    return start + len(text[start:end]) - len(text[start:end].lstrip()), trimmed


def head_path(text: str, mask: list[bool], brace: int) -> str:
    """🧭️ The `a::b::C` path immediately before a `{` (generic args skipped), or ''."""
    j = brace - 1
    while j >= 0 and text[j] in " \t\n":
        j -= 1
    if j >= 0 and text[j] == ">" and mask[j]:
        depth = 0
        while j >= 0:
            if text[j] == ">":
                depth += 1
            elif text[j] == "<":
                depth -= 1
                if depth == 0:
                    j -= 1
                    break
            j -= 1
        while j >= 0 and text[j] in " \t":
            j -= 1
        if j >= 1 and text[j - 1:j + 1] == "::":
            j -= 2
    end = j + 1
    while j >= 0 and (text[j].isalnum() or text[j] in "_:"):
        j -= 1
    return text[j + 1:end]


def strip_struct_fields(text: str, types: set[str], edit_type: str | None, field: str) -> tuple[str, list[tuple[int, str]]]:
    removed: list[tuple[int, str]] = []
    while True:
        mask = code_mask(text)
        found = False
        for match in re.finditer(r"\{", text):
            brace = match.start()
            if not mask[brace]:
                continue
            head = head_path(text, mask, brace)
            last = head.split("::")[-1] if head else ""
            qualified_tail = "::".join(head.split("::")[-2:]) if head else ""
            if not (last in types or qualified_tail in types or (edit_type is not None and last == edit_type)):
                continue
            try:
                close = matching(text, mask, brace)
            except ValueError:
                continue
            items = top_level_items(text, mask, brace, close)
            names = [re.match(r"\s*(?:ref\s+|mut\s+)*([A-Za-z_][A-Za-z0-9_]*)", text[s:e]) for s, e in items]
            keys = [m.group(1) if m else "" for m in names]
            if last == edit_type and last not in types and qualified_tail not in types and not ({"forwards", "mutation_meta"} & set(keys)):
                continue
            for index, (s, e) in enumerate(items):
                item = text[s:e].strip()
                if re.fullmatch(rf"(?:ref\s+|mut\s+)*{field}(\s*:.*)?", item, flags=re.S):
                    a, b = remove_item(text, items, index, close)
                    removed.append((text.count("\n", 0, brace) + 1, f"{head} {{ {item[:60]} }}"))
                    text = text[:a] + text[b:]
                    found = True
                    break
            if found:
                break
        if not found:
            return text, removed


def strip_call_arguments(text: str, calls: dict[str, int]) -> tuple[str, list[tuple[int, str]]]:
    removed: list[tuple[int, str]] = []
    pattern = re.compile(r"\b(" + "|".join(map(re.escape, calls)) + r")\s*\(")
    offset = 0
    while True:
        mask = code_mask(text)
        match = None
        for candidate in pattern.finditer(text, offset):
            if mask[candidate.start()]:
                match = candidate
                break
        if match is None:
            return text, removed
        name = match.group(1)
        open_at = match.end() - 1
        close = matching(text, mask, open_at)
        items = top_level_items(text, mask, open_at, close)
        index = calls[name]
        before = text[:match.start()].rstrip()
        is_definition = before.endswith("fn")
        if is_definition or index >= len(items):
            offset = match.end()
            continue
        a, b = remove_item(text, items, index, close)
        removed.append((text.count("\n", 0, match.start()) + 1, f"{name}(… arg {index}: {text[items[index][0]:items[index][1]].strip()[:60]})"))
        text = text[:a] + text[b:]
        offset = match.start() + 1


def main() -> None:
    apply = "--apply" in sys.argv
    print("library module — imported by the wave-B driver", apply)


if __name__ == "__main__":
    main()
