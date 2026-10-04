#!/usr/bin/env python3
"""🪵️ S3-NORM: EN 1995 member role / support enum values on the wire, schema-first.

The snapshot schema (`$defs/MemberRole`, `$defs/SupportType`) and the Rust `#[value(rename_all = "camelCase")]` spell the
two enums in camelCase; the committed EN 1995 vectors still carried the PascalCase variant names. This rewrites every
`"role"`, `"support"` and the `change-member-role` / `change-member-support` `"newValue"` member to the schema's value
(text-preserving: only the quoted value changes), and drops the leaf-level `x-semio-ui.options` restatement of the two
leaves so the option labels resolve from the one `$defs` truth.

Usage: python3 🧪️s3-norm-en1995-enum-wire.py [--check]
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
SUBSET = ROOT / "✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995/🏅️standards/🔖️1/🪆️subsets/✳️any"
SNAPSHOT = SUBSET / "🧬️schema/📸️snapshot/🔣️.json"
LEAVES = {"🎯️change-member-role": "MemberRole", "📍️change-member-support": "SupportType"}
FIELDS = {"role": "MemberRole", "support": "SupportType"}


def camel(name: str) -> str:
    return name[0].lower() + name[1:]


def main() -> int:
    check = "--check" in sys.argv
    defs = json.loads(SNAPSHOT.read_text())["$defs"]
    enums = {key: defs[key]["enum"] for key in set(FIELDS.values())}
    mapping = {key: {value[0].upper() + value[1:]: value for value in values} for key, values in enums.items()}
    for key, values in mapping.items():
        assert all(camel(pascal) == wire for pascal, wire in values.items()), key
    pending = 0
    for path in sorted((SUBSET / "🧫️fixtures/🧬️mutations").rglob("🔣️.json")):
        text = path.read_text()
        out = text
        for field, enum in FIELDS.items():
            out = re.sub(rf'("{field}": )"([A-Za-z]+)"', lambda m: f'{m.group(1)}"{mapping[enum].get(m.group(2), m.group(2))}"', out)
        leaf = path.relative_to(SUBSET / "🧫️fixtures/🧬️mutations").parts[0]
        if leaf in LEAVES:
            enum = LEAVES[leaf]
            out = re.sub(r'("newValue": )"([A-Za-z]+)"', lambda m: f'{m.group(1)}"{mapping[enum].get(m.group(2), m.group(2))}"', out)
        if out != text:
            pending += 1
            if not check:
                path.write_text(out)
    for leaf in LEAVES:
        path = SUBSET / "🧬️schema/🧬️mutations" / leaf / "🧬️schema/🔣️.json"
        text = path.read_text()
        document = json.loads(text)
        ui = document["properties"]["newValue"]["x-semio-ui"]
        if "options" in ui:
            pending += 1
            if not check:
                del ui["options"]
                path.write_text(json.dumps(document, indent=2, ensure_ascii=False) + "\n")
    stale = [str(p.relative_to(SUBSET)) for p in (SUBSET / "🧫️fixtures/🧬️mutations").rglob("🔣️.json") if re.search(r'"(role|support)": "[A-Z]', p.read_text())]
    print(f"{'pending' if check else 'rewritten'}={pending} stale-after={len(stale) if not check else 'n/a'}")
    return 1 if check and pending else 0


if __name__ == "__main__":
    sys.exit(main())
