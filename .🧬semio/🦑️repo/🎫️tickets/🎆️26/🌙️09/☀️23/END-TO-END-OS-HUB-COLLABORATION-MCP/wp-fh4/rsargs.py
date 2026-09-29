"""✂️ Rust call/definition helpers for FH4's edits: drop one argument from every call of a helper (arity-guarded, so a
re-run is a no-op) and find helpers whose text parameter no longer reaches their body (the codemod's leftovers)."""
from __future__ import annotations

import importlib.util
import re

spec = importlib.util.spec_from_file_location("p2codemod", "/Users/ueli/Documents/semio/.tmp-ticket/wp-s20/s20-p2/p2-codemod.py")
codemod = importlib.util.module_from_spec(spec)
spec.loader.exec_module(codemod)


def drop_argument(text: str, name: str, index: int, arity: int) -> str:
    """🗑️ Removes argument `index` from every call `name(` that has exactly `arity` arguments (never the `fn name(` itself)."""
    edits = []
    for match in re.finditer(r"(?<![\w.])" + re.escape(name) + r"\(", text):
        if re.search(r"fn\s+$", text[max(0, match.start() - 8):match.start()]):
            continue
        parsed = codemod.arguments(text, match.end() - 1)
        if parsed is None:
            continue
        spans, end = parsed
        if len(spans) != arity:
            continue
        keep = [text[a:b].strip() for i, (a, b) in enumerate(spans) if i != index]
        edits.append((match.end(), end - 1, ", ".join(keep)))
    for start, stop, new in sorted(edits, reverse=True):
        text = text[:start] + new + text[stop:]
    return text


def dead_text_parameters(text: str) -> list[tuple[str, str, int]]:
    """🔎️ `(fn name, parameter, line)` for every fn whose `message`/`reason`/`text`/`detail`/`label`/`noun` parameter its body
    never names."""
    found = []
    for match in re.finditer(r"\bfn\s+(\w+)\s*(?:<[^>]*>)?\s*\(", text):
        parsed = codemod.arguments(text, match.end() - 1)
        if parsed is None:
            continue
        spans, end = parsed
        brace = text.find("{", end)
        semicolon = text.find(";", end)
        if brace < 0 or (0 <= semicolon < brace):
            continue
        depth, at = 0, brace
        while at < len(text):
            depth += {"{": 1, "}": -1}.get(text[at], 0)
            if depth == 0:
                break
            at += 1
        body = text[brace:at]
        for a, b in spans:
            param = re.match(r"\s*(?:mut\s+)?(\w+)\s*:", text[a:b])
            if param and param.group(1) in ("message", "reason", "text", "detail", "label", "noun", "blocker", "why") and not re.search(r"\b" + param.group(1) + r"\b", body):
                found.append((match.group(1), param.group(1), text.count("\n", 0, match.start()) + 1))
    return found
