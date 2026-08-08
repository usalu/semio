#!/usr/bin/env python3
"""Fix map field parity and diff GraphQL optionality."""
import json
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
MAP_FIELDS = {
    "📓️iso16757": ["partNumberInputs"],
    "📔️vdi3805": ["editionProfile", "geometry", "curves"],
}

for folder, fields in MAP_FIELDS.items():
    base = ROOT / folder
    prefix = folder.split("️")[1] if "️" in folder else folder
    for facet in ["🧬️schema", "📸️snapshot/🧬️schema", "🔺️diff/🧬️schema"]:
        ts = base / facet / "🟦️component.ts"
        if ts.exists():
            t = ts.read_text(encoding="utf-8")
            for f in fields:
                t = re.sub(f"{f}: string;", f"{f}: Record<string, string>;", t)
            ts.write_text(t, encoding="utf-8")
        gql = base / facet / "🔗️component.graphql"
        if gql.exists():
            t = gql.read_text(encoding="utf-8")
            for f in fields:
                camel = f
                entry = f"{prefix.capitalize()}StringMapEntry"
                if facet.startswith("🔺"):
                    t = re.sub(
                        f"{camel}: [^\\n]+@state",
                        f"{camel}: [{entry}!]! @state",
                        t,
                    )
                else:
                    t = re.sub(
                        f"{camel}: [^\\n]+@state",
                        f"{camel}: [{entry}!]! @state",
                        t,
                    )
            if "type " + entry not in t and facet == "🧬️schema":
                t += f"\ntype {entry} {{ key: String! value: String! }}\n"
            gql.write_text(t, encoding="utf-8")
        proto = base / facet / "🛰️component.proto"
        if proto.exists():
            t = proto.read_text(encoding="utf-8")
            for f in fields:
                snake = re.sub(r"(?<!^)(?=[A-Z])", "_", f).lower()
                t = re.sub(
                    f"(optional )?(string|double) {snake} =",
                    f"map<string, string> {snake} =",
                    t,
                )
            proto.write_text(t, encoding="utf-8")

for folder in MAP_FIELDS:
    for gql in (ROOT / folder).rglob("🔺️diff/🧬️schema/🔗️component.graphql"):
        t = gql.read_text(encoding="utf-8")
        t = re.sub(r": ([A-Za-z0-9_\[\]!]+)!(\s+@state)", r": \1\2", t)
        gql.write_text(t, encoding="utf-8")
