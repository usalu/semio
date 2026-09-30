#!/usr/bin/env python3
"""🧾️ W2-A: adds `transaction`/`mutations` to every `CommandView { .. }` literal and moves every
`ui_history_panel(history, controller, is_de, read_only, view)` call to the
`(history, time_travel, controller, locale, read_only, view)` signature. Usage: python3 <this> <file>..."""
import re
import sys


def command_view_literals(text: str) -> str:
    out, cursor = [], 0
    for match in re.finditer(r"CommandView \{", text):
        start = match.end()
        depth, index = 1, start
        while depth:
            char = text[index]
            depth += {"{": 1, "}": -1}.get(char, 0)
            index += 1
        literal = text[start:index - 1]
        if "transaction" in literal or ".." in literal.split("inverse")[-1]:
            continue
        inverse = re.search(r"(\n(\s*)inverse: [^\n]*,)", literal)
        if inverse is None:
            continue
        indent = inverse.group(2)
        cut = start + inverse.end()
        out.append(text[cursor:cut])
        out.append(f"\n{indent}transaction: None,\n{indent}mutations: Vec::new(),")
        cursor = cut
    out.append(text[cursor:])
    return "".join(out)


def panel_calls(text: str) -> str:
    return re.sub(r"ui_history_panel\(([^,()]+(?:\([^()]*\))?), (\"[^\"]*\"|[^,]+), (false|true|is_de), ", lambda m: f"ui_history_panel({m.group(1)}, None, {m.group(2)}, {'Locale::De' if m.group(3) == 'true' else 'Locale::En'}, ", text)


for path in sys.argv[1:]:
    with open(path, encoding="utf-8") as handle:
        source = handle.read()
    updated = panel_calls(command_view_literals(source))
    if updated != source:
        with open(path, "w", encoding="utf-8") as handle:
            handle.write(updated)
        print(f"updated {path}")
