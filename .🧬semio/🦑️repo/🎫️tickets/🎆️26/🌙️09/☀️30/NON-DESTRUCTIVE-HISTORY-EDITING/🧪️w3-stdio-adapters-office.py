#!/usr/bin/env python3
"""🧾️ W3-STDIO-CASES item 6, second half: the office (`from_json_str` + `<Agg as Mutation<Snap>>::from_payload_value` +
`params_are_wire`) and ifc/step (`mutation_from_payload_json::<Snap, Agg>`) case adapters move onto the one decode path —
`law::wire_operation(kind, &params, mutation_from_payload_json, mutation_payload_json)` — and undo through
`mutation_inverse`. Idempotent. Usage: `python3 🧪️w3-stdio-adapters-office.py <adapter.rs>...`."""

import re
import sys

LAW = "semio_s_plugin_stdio_test_oracle::law"


def convert(path: str) -> None:
    original = open(path, encoding="utf-8").read()
    text = re.sub(
        r"([ \t]*)let \w+: DslValue = from_json_str\(&(\w+)\.to_string\(\)\)\.map_err\(\|error\| error\.to_string\(\)\)\?;\n"
        r"[ \t]*let (\w+) = <\w+ as Mutation<\w+>>::from_payload_value\(&?(\w+), \w+\)\.map_err\(\|error\| error\.to_string\(\)\)\?;\n"
        r"[ \t]*params_are_wire\([^\n]*\n",
        lambda m: f"{m.group(1)}let {m.group(3)} = wire_operation(&{m.group(4)}, &{m.group(2)}, mutation_from_payload_json, mutation_payload_json)?;\n",
        original,
    )
    text = re.sub(r"<\w+ as Mutation<\w+>>::inverse\(", "mutation_inverse(", text)
    text = re.sub(
        r"mutation_from_payload_json::<\w+, \w+>\((&\w+\.str\(\"kind\"\)), &(\w+)\.get\(\"params\"\)\.cloned\(\)\.unwrap_or\(Json::Null\)\.to_string\(\)\)",
        lambda m: f'wire_operation({m.group(1)}, &{m.group(2)}.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)',
        text,
    )

    def crate_use(match: re.Match) -> str:
        items = [item.strip() for item in match.group(3).split(",") if item.strip() and item.strip() not in ("from_json_str", "to_json_string", "DslValue", "Mutation")]
        for name in ("mutation_from_payload_json", "mutation_inverse", "mutation_payload_json"):
            if re.search(rf"\b{name}\(|\b{name}\)|, {name}\b", text) and name not in items:
                items.append(name)
        items.sort(key=lambda item: (not item[0].islower(), item))
        body = f"{{{', '.join(items)}}}" if len(items) > 1 else items[0]
        return f"{match.group(1)}use {match.group(2)}::{body};\n"

    text = re.sub(r"([ \t]*)use (semio_s_artifact_stdio_[a-z0-9_]+)::\{([^{}]*)\};\n", crate_use, text, count=1)
    if "wire_operation(" in text:
        if re.search(rf"use {re.escape(LAW)}::params_are_wire;", text):
            text = re.sub(rf"use {re.escape(LAW)}::params_are_wire;", f"use {LAW}::wire_operation;", text)
        elif f"{LAW}::wire_operation" not in text and "wire_operation;" not in text:
            match = re.search(r"^([ \t]*)use semio_s_artifact_stdio_[a-z0-9_]+::\{[^{}]*\};\n", text, re.M)
            text = text[:match.end()] + f"{match.group(1)}use {LAW}::wire_operation;\n" + text[match.end():]
    if text != original:
        print(f"[w3-stdio] convert {path}")
        open(path, "w", encoding="utf-8").write(text)


if __name__ == "__main__":
    for argument in sys.argv[1:]:
        convert(argument)
