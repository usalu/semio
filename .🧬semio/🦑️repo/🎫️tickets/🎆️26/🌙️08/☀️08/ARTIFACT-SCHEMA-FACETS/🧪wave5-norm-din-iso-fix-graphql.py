#!/usr/bin/env python3
"""Add GraphQL required markers to match JSON schema cardinality."""
import re
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
FOLDERS = ["📓️iso16757", "📔️vdi3805", "📕️din4108", "📗️din16798", "📙️din18599"]

for folder in FOLDERS:
    for gql in (ROOT / folder).rglob("🔗️component.graphql"):
        text = gql.read_text(encoding="utf-8")
        text = re.sub(
            r": (Float|Int|Boolean|String)(\s+@state)",
            r": \1!\2",
            text,
        )
        text = re.sub(
            r": \[([A-Za-z0-9_]+!)\](\s+@state)",
            r": [\1]!\2",
            text,
        )
        text = re.sub(
            r": (En\d+|Din|Iso|Vdi)[A-Za-z0-9]+(\s+@state)",
            r": \1!\2",
            text,
        )
        gql.write_text(text, encoding="utf-8")
