"""Add missing field classification comments as `# kind // detail` (detail optional). Does not remove existing tags."""
from __future__ import annotations

import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE
PATH: Path | None = None
for _ in range(14):
    cand = ROOT / "semio" / "graphql" / "target.schema.graphql"
    if cand.is_file():
        PATH = cand
        break
    ROOT = ROOT.parent
else:
    raise SystemExit("annotate_field_comments: could not find semio/graphql/target.schema.graphql from script location")

# Field definition: indent, name, optional args, colon, type part (may include directives), optional comment
FIELD_RE = re.compile(
    r"^(\s+)([a-zA-Z_][a-zA-Z0-9_]*)(\([^)]*\))?\s*:\s*([^#\n]+?)(\s*(#.*)?)$"
)

SECTION_MARKERS = {
    "# WeakEntity",
    "# StrongEntity",
    "# hashed",
    "# strong",
    "# Data Fields",
    "# Computed Fields",
    "# Reference Fields",
    "# Diff",
    "# Modification",
    "# Artifact",
    "# Document",
    "# data",
    "# computed",
    "# reference",
}


def normalize_entity_owner_comment(rest: str) -> str:
    """entityOwner line: ensure `# computed // Hint` form."""
    rest = rest.strip()
    if not rest.startswith("#"):
        return " # computed"
    body = rest[1:].strip()
    if body.startswith("computed"):
        return " " + rest  # already has computed
    # `# DiffOwner` → `# computed // DiffOwner`
    return f" # computed // {body}"


def normalize_owned_comment(rest: str) -> str:
    rest = rest.strip()
    if not rest.startswith("#"):
        return " # computed"
    body = rest[1:].strip()
    if body.startswith("computed"):
        return " " + rest
    return f" # computed // {body}"


def main() -> None:
    lines = PATH.read_text(encoding="utf-8").splitlines()
    out: list[str] = []
    section = "top"

    opens_decl = re.compile(r"^\s*(extend\s+)?(type|interface|input)\s+\w+")

    for line in lines:
        stripped = line.strip()

        if opens_decl.match(line):
            m_open = opens_decl.match(line)
            kind = m_open.group(2)
            section = "input_section" if kind == "input" else "waiting_marker"

        if stripped in SECTION_MARKERS or stripped.replace("  ", "").startswith("# WeakEntity"):
            if "# WeakEntity" in stripped or stripped == "# hashed":
                section = "weak_shell"
            elif "# StrongEntity" in stripped or stripped == "# strong":
                section = "strong_shell"
            elif "Data Fields" in stripped or stripped == "# data":
                section = "data_section"
            elif "Computed Fields" in stripped or stripped == "# computed":
                section = "computed_section"
            elif "Reference Fields" in stripped or stripped == "# reference":
                section = "reference_section"
            elif stripped == "# Diff":
                section = "diff_section"
            elif stripped == "# Modification":
                section = "mod_section"
            elif stripped == "# Artifact":
                section = "artifact_section"
            elif stripped == "# Document":
                section = "document_section"
            out.append(line)
            continue

        m = FIELD_RE.match(line)
        if not m:
            out.append(line)
            continue

        indent, name, _args, typ_raw, tail = m.group(1), m.group(2), m.group(3), m.group(4).rstrip(), m.group(5) or ""
        typ = typ_raw.strip()

        # Already has a classification tag (# data / # computed / # cached / # reference)
        if re.search(r"#\s*(data|computed|cached|reference)\b", tail):
            out.append(line)
            continue

        # Preserve lines that already have any # comment (union hints only)
        if tail.strip().startswith("#") and name not in ("entityOwner", "ownedEntities"):
            out.append(line)
            continue

        add = ""

        if name == "hash" and typ == "String!":
            add = " # cached"
        elif name == "id" and typ == "ID!":
            if section == "strong_shell":
                add = " # data // uuidv7"
            elif section == "weak_shell":
                add = " # computed // hash"
            else:
                add = " # computed // hash"
        elif name == "owner":
            add = " # reference"
        elif name in ("ownerDiffs", "diffOwner", "changeOwner", "operationOwner"):
            add = " # reference // spine"
        elif name == "entityOwner":
            add = normalize_entity_owner_comment(tail)
            line = f"{indent}{name}: {typ_raw.strip()}{add}"
            out.append(line.rstrip())
            continue
        elif name == "ownedEntities":
            add = normalize_owned_comment(tail)
            line = f"{indent}{name}: {typ_raw.strip()}{add}"
            out.append(line.rstrip())
            continue
        elif name in ("before", "after") and section == "diff_section":
            add = " # reference // diff"
        elif name == "modification" and section == "diff_section":
            add = " # reference // diff"
        elif name in ("edges", "pageInfo"):
            add = " # computed"
        elif name == "cursor":
            add = " # computed"
        elif name == "node":
            add = " # reference"
        elif section == "data_section":
            add = " # data"
        elif section == "computed_section":
            add = " # computed"
        elif section == "reference_section":
            add = " # reference"
        elif section == "artifact_section":
            if typ.endswith("Connection") or typ.endswith("Connection!"):
                add = " # computed"
            elif typ == "Timestamp":
                add = " # data"
            elif typ in ("Author", "Checkpoint"):
                add = " # reference"
        elif section == "document_section":
            if typ.endswith("Connection") or typ.endswith("Connection!"):
                add = " # computed"
            elif typ == "File":
                add = " # reference"
            elif typ == "Timestamp":
                add = " # data"
        elif typ.endswith("Connection") or typ.endswith("Connection!"):
            add = " # computed"
        elif section == "weak_shell" and name.endswith("Owner") and name != "owner":
            add = " # reference"
        elif section == "input_section":
            add = " # data"

        if add:
            line = f"{indent}{name}: {typ_raw.strip()}{add}"

        out.append(line.rstrip())

    PATH.write_text("\n".join(out) + "\n", encoding="utf-8")
    print(f"updated {PATH}")


if __name__ == "__main__":
    main()
