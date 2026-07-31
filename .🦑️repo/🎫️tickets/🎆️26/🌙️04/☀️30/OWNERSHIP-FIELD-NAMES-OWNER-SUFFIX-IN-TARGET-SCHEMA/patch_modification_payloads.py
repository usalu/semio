"""Sync *Modification payload fields from matching entity # data fields.

Modification payload fields are always nullable (strip ``!``) so null means unchanged.
``removeField`` booleans are emitted only when the corresponding entity field was nullable.
"""
from __future__ import annotations

import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE
for _ in range(14):
    if (ROOT / "compose" / "graphql" / "target.schema.graphql").is_file():
        break
    ROOT = ROOT.parent
PATH = ROOT / "compose" / "graphql" / "target.schema.graphql"

SKIP_ENTITY_PREFIXES = (
    "Modification",
    "Diff",
    "Diffs",
    "Edge",
    "Connection",
    "Input",
)


def extract_entity_data_fields(text_lines: list[str], entity_name: str) -> list[tuple[str, str]]:
    for idx, line in enumerate(text_lines):
        if not re.match(rf"^type {re.escape(entity_name)}\b", line):
            continue
        block = []
        depth = 0
        j = idx
        while j < len(text_lines):
            block.append(text_lines[j])
            depth += text_lines[j].count("{") - text_lines[j].count("}")
            j += 1
            if depth == 0:
                break
        inner = block[1:-1]
        cap = len(inner)
        for k, ln in enumerate(inner):
            if ln.strip() == "# Computed Fields":
                cap = k
                break
        out: list[tuple[str, str]] = []
        for ln in inner[:cap]:
            m = re.match(r"^(\s*)(\w+)(\([^)]*\))?\s*:\s*(.+?)\s*#\s*data\s*$", ln)
            if not m:
                continue
            fname = m.group(2)
            ftyp = m.group(4).strip()
            out.append((fname, ftyp))
        return out
    return []


def strip_outer_required(typ: str) -> str:
    """Patch-field types are nullable on modifications (strip trailing ``!`` markers)."""
    t = typ.strip()
    while t.endswith("!"):
        t = t[:-1].strip()
    return t


def remove_boolean_name(field: str) -> str:
    return "remove" + field[0].upper() + field[1:]


def build_payload_lines(entity_name: str, fields: list[tuple[str, str]], section_comment: str) -> list[str]:
    lines = [f"  # {section_comment}"]
    for fname, ftyp in fields:
        entity_typ = ftyp.strip()
        entity_required = entity_typ.endswith("!")
        mod_typ = strip_outer_required(entity_typ)
        lines.append(f"  {fname}: {mod_typ} # computed")
        if not entity_required:
            lines.append(f"  {remove_boolean_name(fname)}: Boolean # computed")
    return lines


def patch_modification_blocks(text: str) -> str:
    lines = text.splitlines()
    out: list[str] = []
    i = 0
    while i < len(lines):
        m = re.match(r"^type (\w+Modification) implements Entity \{", lines[i])
        if not m:
            out.append(lines[i])
            i += 1
            continue

        mod_name = m.group(1)
        entity_name = mod_name[: -len("Modification")]
        if any(entity_name.startswith(p) or entity_name.endswith(p) for p in ("Diff",)) or entity_name in SKIP_ENTITY_PREFIXES:
            out.append(lines[i])
            i += 1
            continue

        start = i
        i += 1
        depth = 1
        block_lines: list[str] = [lines[start]]
        while i < len(lines) and depth > 0:
            depth += lines[i].count("{") - lines[i].count("}")
            block_lines.append(lines[i])
            i += 1

        inner = block_lines[1:-1]
        data_fields = extract_entity_data_fields(lines, entity_name)
        if not data_fields:
            out.extend(block_lines)
            continue

        # Keep weak/mod header through ownedEntities line
        kept: list[str] = []
        j = 0
        while j < len(inner):
            ln = inner[j]
            kept.append(ln)
            if re.match(r"^\s+ownedEntities:\s*", ln):
                j += 1
                break
            j += 1

        payload = build_payload_lines(entity_name, data_fields, mod_name)
        new_inner = kept + payload
        new_block = [block_lines[0]] + new_inner + [block_lines[-1]]
        out.extend(new_block)

    return "\n".join(out) + ("\n" if text.endswith("\n") else "")


def main() -> None:
    raw = PATH.read_text(encoding="utf-8")
    new = patch_modification_blocks(raw)
    PATH.write_text(new, encoding="utf-8")
    print(f"patched {PATH}")


if __name__ == "__main__":
    main()
