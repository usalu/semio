#!/usr/bin/env python3
"""🐫️ W2-W media: the repo-wide rule makes a leaf root the camelCase `payload_value()` Rust emits. The media leaves
whose Rust struct had no `#[value(rename_all)]` were the outlier against their TS/GraphQL twins, feature rows and
snapshot records, so this adds `rename_all = "camelCase"` to each struct and renames the same members in its leaf
schema (properties and `required`, every `x-semio-ui` annotation moving with its member). Idempotent.
"""
import json
import re
from pathlib import Path

ARTIFACTS = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts")
AGGREGATES = [
    "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations",
    "🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🧬️schema/🧬️mutations",
    "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline/🧬️schema/🧬️mutations",
    "📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🧬️schema/🧬️mutations",
    "🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/🧬️mutations",
]


def camel(name):
    head, *rest = name.split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def main():
    for aggregate in AGGREGATES:
        for schema_path in sorted((ARTIFACTS / aggregate).glob("*/🧬️schema/🔣️.json")):
            schema = json.loads(schema_path.read_text())
            snake = [key for key in schema.get("properties", {}) if "_" in key]
            if not snake:
                continue
            schema["properties"] = {camel(key): value for key, value in schema["properties"].items()}
            schema["required"] = [camel(key) for key in schema.get("required", [])]
            schema_path.write_text(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
            rust_path = schema_path.parent.parent / "🦀️.rs"
            source = rust_path.read_text()
            pattern = re.compile(r"(#\[mutation_leaf\(contract = ::protocol\)\]\n(?:#\[dsl\([^\n]*\)\]\n)?)(pub struct \w+ \{)")
            if 'rename_all = "camelCase"' not in source:
                source, count = pattern.subn(r'\1#[value(rename_all = "camelCase")]\n\2', source)
                assert count == 1, rust_path
                rust_path.write_text(source)
            print(schema_path.parent.parent.name, snake, "->", [camel(key) for key in snake])


if __name__ == "__main__":
    main()
