"""One-off transform for semio/graphql/target.schema.graphql Operation Input plan."""
from __future__ import annotations

import re
from pathlib import Path

EMPTY_INPUTS = frozenset(
    {
        "RemovedAttributeFromQualityInput",
        "RemovedAttributesFromQualityInput",
        "DeletedQualityInput",
        "DeletedQualitiesInput",
        "RemovedAttributeFromTagInput",
        "RemovedAttributesFromTagInput",
        "DeletedTagInput",
        "DeletedTagsInput",
        "RemovedAttributeFromConceptInput",
        "RemovedAttributesFromConceptInput",
        "DeletedConceptInput",
        "DeletedConceptsInput",
        "RemovedAttributeFromPortInput",
        "RemovedAttributesFromPortInput",
        "DeletedPortInput",
        "DeletedPortsInput",
        "RemovedAttributeFromTypeInput",
        "RemovedAttributesFromTypeInput",
        "DeletedTypeInput",
        "DeletedTypesInput",
        "RemovedConnectorInput",
        "RemovedConnectorsInput",
        "FixedPieceInput",
        "FixedPiecesInput",
        "RemovedAttributeFromPieceInput",
        "RemovedAttributesFromPieceInput",
        "DeletedPieceInput",
        "DeletedPiecesInput",
        "DeletedPiecesAndConnectionsInput",
        "DeletedDesignInput",
        "DeletedDesignsInput",
        "FlattenedDesignInput",
        "RemovedAttributeFromDesignInput",
        "RemovedAttributesFromDesignInput",
    }
)

# Operation types that gain <Op>Input in owns comment (non-empty inputs only)
INPUT_TO_OP: dict[str, str] = {}


def op_name_from_input_type(input_type: str) -> str:
    assert input_type.endswith("Input"), input_type
    return input_type[: -len("Input")]


def looks_like_command_input_block(inner: list[str]) -> bool:
    for line in inner:
        s = line.strip()
        if not s or s.startswith("#"):
            continue
        # e.g. rename(newName: String!): ID!
        if re.match(r"^\w+\([^)]*\):", s):
            return True
    return False


def parse_type_block(lines: list[str], start: int) -> tuple[str, list[str], int]:
    """Return (type_name, inner_lines_without_braces, index_after_closing_brace)."""
    first = lines[start]
    m = re.match(r"^type (\w+) \{$", first)
    if not m:
        raise ValueError(f"Expected type line at {start}: {first!r}")
    name = m.group(1)
    inner: list[str] = []
    i = start + 1
    while i < len(lines):
        if lines[i] == "}":
            return name, inner, i + 1
        inner.append(lines[i])
        i += 1
    raise ValueError(f"Unclosed type {name}")


def build_input_type(type_name: str, inner: list[str]) -> list[str]:
    op = op_name_from_input_type(type_name)
    INPUT_TO_OP[type_name] = op
    # Drop first line if it is only `# TypeName` duplicate header
    rest = list(inner)
    if rest and re.match(rf"^  # {re.escape(type_name)}$", rest[0]):
        rest = rest[1:]
    out = [
        f"type {type_name} implements Input {{",
        "  # Node",
        "  id: ID! # computed // hash",
        "  # Entity",
        "  hash: String! # cached",
        f"  owner: Entity # reference // {op}",
        "  owns: EntityConnection # reference",
        "  # Input",
        "  # Arguments",
    ]
    out.extend(rest)
    out.append("}")
    return out


def main() -> None:
    path = Path("semio/graphql/target.schema.graphql")
    lines = path.read_text(encoding="utf-8").splitlines()
    out: list[str] = []
    i = 0
    in_commands = False
    while i < len(lines):
        line = lines[i]
        if line.strip() == "#region Commands":
            in_commands = True
        if in_commands or not re.match(r"^type \w+ \{$", line):
            out.append(line)
            i += 1
            continue
        name, inner, j = parse_type_block(lines, i)
        if not name.endswith("Input"):
            out.append(line)
            i += 1
            continue
        if looks_like_command_input_block(inner):
            out.extend(lines[i:j])
            i = j
            continue
        if name in EMPTY_INPUTS:
            i = j  # skip entire type block
            continue
        # non-empty operation input -> implements Input
        out.extend(build_input_type(name, inner))
        i = j

    text = "\n".join(out) + "\n"

    # Remove input: ConcreteInput! # data from operations that had empty inputs
    for empty in EMPTY_INPUTS:
        op = op_name_from_input_type(empty)
        #   input: RemovedAttributeFromQualityInput! # data
        pat = rf"\n  input: {re.escape(empty)}! # data\n"
        text = re.sub(pat, "\n", text)

    # Fix CreatedFixedPieceInput owner/owns/id per plan (WeakEntity, op owns input)
    text = text.replace(
        "type CreatedFixedPieceInput implements Input {\n"
        "  # Node\n"
        "  id: ID! # data // uuidv7\n"
        "  # Entity\n"
        "  hash: String! # cached\n"
        "  owner: Entity # reference // Edit\n"
        "  owns: EntityConnection # reference // Piece\n",
        "type CreatedFixedPieceInput implements Input {\n"
        "  # Node\n"
        "  id: ID! # computed // hash\n"
        "  # Entity\n"
        "  hash: String! # cached\n"
        "  owner: Entity # reference // CreatedFixedPiece\n"
        "  owns: EntityConnection # reference\n",
    )

    # owns: EntityConnection # reference // ... append <Op>Input for operations that have inputs
    for input_type, op in sorted(INPUT_TO_OP.items(), key=lambda x: -len(x[0])):
        needle = f"type {op} implements Operation {{"
        idx = text.find(needle)
        if idx == -1:
            continue
        # find owns line within this type (next ~25 lines)
        slice_end = text.find("\n}\n", idx)
        if slice_end == -1:
            continue
        chunk = text[idx:slice_end]
        owns_re = re.compile(
            r"(owns: EntityConnection # reference // )([^\n]*)",
            re.MULTILINE,
        )
        m = owns_re.search(chunk)
        if not m:
            continue
        prefix, union = m.group(1), m.group(2)
        if input_type in union:
            continue
        new_union = f"{input_type} | {union}" if union.strip() else input_type
        new_chunk = owns_re.sub(prefix + new_union, chunk, count=1)
        text = text[:idx] + new_chunk + text[slice_end:]

    path.write_text(text, encoding="utf-8")
    print("Updated", path, "inputs:", len(INPUT_TO_OP))


if __name__ == "__main__":
    main()
