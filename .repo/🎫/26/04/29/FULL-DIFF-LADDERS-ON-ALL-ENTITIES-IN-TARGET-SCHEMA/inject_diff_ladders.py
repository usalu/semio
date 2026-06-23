#!/usr/bin/env python3
"""
🔖 Injects per-entity Modification→DiffsConnection ladders into compose/graphql/target.schema.graphql
and merges global unions (Modification, DiffOwned, DiffsOwned, aggregates).

Temporary generator kept under the ticket folder per repo rules.
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[6]
SCHEMA = ROOT / "compose" / "graphql" / "target.schema.graphql"

# 🔖 Mirrors existing PlaneDiffsOwner concrete roots (Operation implementations omitted — unions stay object-only).
DIFFS_OWNER_BODY = """  | AttributeDiff
  | Change
  | ChangedDescription
  | CreatedFixedPiece
  | Diffs
  | DraggedPiece
  | FixedPiece
  | LocationDiff
  | PlaneDiff
  | PlaceDiff
  | PositionDiff
  | RenamedKit"""

SKIP_IF_LADDER_PRESENT = frozenset({"Plane", "Position", "Location", "Place"})

# 🔖 Attribute region already defines modification/diff paths; only AttributesDiff→AttributeDiffs rename is applied.
SKIP_ENTITIES = frozenset({"Attribute"})

WEAK_ENTITIES = frozenset({"Vector", "Point", "Coordinate", "Offset"})

# 🔖 Connector / Representation / Type share one `#region Type` block.
# 🔖 Connection lives under `#region Piece`; VCS entities under `#region VCS`.
ENTITY_HOME_REGION: dict[str, str] = {
    "Connector": "Type",
    "Representation": "Type",
    "Type": "Type",
    "Connection": "Piece",
    "Change": "VCS",
    "Transaction": "VCS",
    "Draft": "VCS",
    "Checkpoint": "VCS",
    "Alternative": "VCS",
    "Graph": "VCS",
    "Session": "VCS",
    "ReadVersion": "VCS",
    "WriteVersion": "VCS",
    "Conflict": "VCS",
}

STRONG_ENTITIES = [
    "Family",
    "Folder",
    "File",
    "Author",
    "Prop",
    "Benchmark",
    "Quality",
    "Tag",
    "Concept",
    "Stat",
    "Connector",
    "Representation",
    "Type",
    "Layer",
    "Group",
    "Piece",
    "Connection",
    "Side",
    "Design",
    "Kit",
    "Change",
    "Transaction",
    "Draft",
    "Checkpoint",
    "Alternative",
    "Graph",
    "Session",
    "ReadVersion",
    "WriteVersion",
    "Conflict",
]


def region_bounds(text: str, region: str) -> tuple[int, int]:
    """🔖 [start, end) covering `#region {region}` … `#endregion {region}` with nested `#region` blocks."""
    header = f"#region {region}\n"
    start = text.find(header)
    if start < 0:
        raise KeyError(region)
    stack = [region]
    pos = start + len(header)
    while stack:
        j = text.find("#", pos)
        if j < 0:
            raise RuntimeError(f"unclosed region {region}")
        line_end = text.find("\n", j)
        line = text[j:] if line_end < 0 else text[j:line_end]
        chunk_end = len(text) if line_end < 0 else line_end + 1
        if line.startswith("#region "):
            inner = line[len("#region ") :].strip()
            stack.append(inner)
        elif line.startswith("#endregion "):
            end_name = line[len("#endregion ") :].strip()
            if not stack or stack[-1] != end_name:
                raise RuntimeError(f"endregion mismatch in {region}: got {end_name} stack={stack}")
            stack.pop()
            if not stack:
                return start, chunk_end
        pos = chunk_end
    raise RuntimeError(f"unclosed region {region}")


def rename_attributes_diff_typenames(text: str) -> str:
    """🔖 AttributesDiff → AttributeDiffs for naming alignment with *Diffs."""
    text = text.replace("AttributesDiffConnection", "AttributeDiffsConnection")
    text = text.replace("AttributesDiffEdge", "AttributeDiffsEdge")
    text = text.replace("AttributesDiffOwner", "AttributeDiffsOwner")
    text = text.replace("AttributesDiffOwned", "AttributeDiffsOwned")
    text = text.replace("type AttributesDiff implements Entity", "type AttributeDiffs implements Entity")
    text = text.replace("owner: AttributesDiffOwner!", "owner: AttributeDiffsOwner!")
    text = text.replace("# AttributesDiffOwner", "# AttributeDiffsOwner")
    text = text.replace("# AttributesDiffOwned", "# AttributeDiffsOwned")
    text = text.replace("| AttributesDiff\n", "| AttributeDiffs\n")
    text = text.replace("| AttributesDiff ", "| AttributeDiffs ")
    text = text.replace("node: AttributesDiff!", "node: AttributeDiffs!")
    return text


def entity_ladder(name: str, *, weak_entity: bool) -> str:
    """🔖 Nine prefixed SDL fragments in fixed internal order."""
    mo = f"{name}ModificationOwner"
    md = f"{name}ModificationOwned"
    id_weak = "# hash"
    id_strong = "# uuidv7"
    id_line = id_weak if weak_entity else id_strong
    entity_tag = "# WeakEntity" if weak_entity else "# StrongEntity"
    return f"""
union {mo} =
  | {name}Diff
  | {name}Modification
  | {name}

union {md} =
  | {name}

type {name}Modification implements Entity {{
  # WeakEntity
  id: ID! {id_weak}
  hash: String!
  owner: {mo}!
  owner{name}Diff: {name}Diff
  owner{name}Modification: {name}Modification
  owner{name}: {name}
  ownerEntity: OwnerEntity
  ownedEntities: OwnedEntityConnection
}}

type {name}ModificationEdge implements EntityEdge {{
  cursor: String!
  node: {name}Modification!
}}

type {name}ModificationConnection implements EntityConnectionInterface {{
  edges: [{name}ModificationEdge!]!
  pageInfo: PageInfo!
  hash: String!
}}

type {name}Diff implements Diff {{
  # WeakEntity
  id: ID! # hash
  hash: String!
  owner: DiffOwner!
  ownerDiffs: Diffs
  ownerChange: Change
  ownerOperation: Operation
  ownerEntity: OwnerEntity # DiffOwner
  ownedEntities: OwnedEntityConnection # DiffOwned
  # Diff
  before: {name}!
  modification: {name}Modification!
  after: {name}!
}}

type {name}DiffEdge implements EntityEdge {{
  cursor: String!
  node: {name}Diff!
}}

type {name}DiffConnection implements EntityConnectionInterface {{
  edges: [{name}DiffEdge!]!
  pageInfo: PageInfo!
  hash: String!
}}

union {name}DiffsOwner =
{DIFFS_OWNER_BODY}

union {name}DiffsOwned =
  | {name}Diffs
  | {name}Diff
  | PlaneDiff
  | PositionDiff
  | AttributeDiff
  | LocationDiff
  | PlaceDiff

type {name}Diffs implements Entity {{
  # WeakEntity
  id: ID! # hash
  hash: String!
  owner: {name}DiffsOwner!
  ownerDiff: Diff
  ownerOperation: Operation
  ownerChange: Change
  ownerEntity: OwnerEntity # {name}DiffsOwner
  ownedEntities: OwnedEntityConnection # {name}DiffsOwned
  removed: EntityConnection
  diffs: {name}DiffConnection
  added: EntityConnection
}}

type {name}DiffsEdge implements EntityEdge {{
  cursor: String!
  node: {name}Diffs!
}}

type {name}DiffsConnection implements EntityConnectionInterface {{
  edges: [{name}DiffsEdge!]!
  pageInfo: PageInfo!
  hash: String!
}}
"""


def find_brace_block_end(s: str, open_brace: int) -> int:
    """🔖 First index after outer `{` … `}` starting at open_brace."""
    depth = 0
    i = open_brace
    while i < len(s):
        c = s[i]
        if c == "{":
            depth += 1
        elif c == "}":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise RuntimeError("unbalanced braces")


def inject_after_primary_connection(text: str, entity: str, region: str, ladder: str) -> str:
    """🔖 Insert ladder immediately after `type {entity}Connection …` inside region."""
    if f"type {entity}DiffsConnection implements EntityConnectionInterface" in text:
        return text
    rs, re_ = region_bounds(text, region)
    chunk = text[rs:re_]
    conn_name = f"{entity}Connection"
    needle = f"type {conn_name} implements EntityConnectionInterface {{"
    ci = chunk.find(needle)
    if ci < 0:
        raise RuntimeError(f"no {conn_name} inside #{region} for {entity}")
    abs_open = rs + ci + len(needle) - 1
    abs_end = find_brace_block_end(text, abs_open)
    while abs_end < len(text) and text[abs_end] in " \t":
        abs_end += 1
    if abs_end < len(text) and text[abs_end] == "\r":
        abs_end += 1
    if abs_end < len(text) and text[abs_end] == "\n":
        abs_end += 1
    return text[:abs_end] + "\n" + ladder.strip() + "\n\n" + text[abs_end:]


def merge_union(text: str, union_name: str, new_members: list[str]) -> str:
    """🔖 Replace `union Name = …` body with sorted unique members (multiline)."""
    m = re.search(
        rf"(union\s+{re.escape(union_name)}\s*=\s*)([^\n]+(?:\n\s*\|[^\n]+)*)",
        text,
        re.MULTILINE,
    )
    if not m:
        raise RuntimeError(f"union {union_name} not found")
    existing = re.findall(r"\|\s*(\w+)", m.group(2))
    merged = sorted(set(existing + new_members), key=lambda s: s.lower())
    body = "\n" + "\n".join(f"  | {x}" for x in merged) + "\n"
    return text[: m.start()] + m.group(1) + body + text[m.end() :]


def merge_aggregate_union(text: str, union_pat: str, extras: list[str]) -> str:
    """🔖 Merge AggregateEntityEdge / EntityConnection / OwnerEntity / OwnedEntityConnection."""
    m = re.search(rf"(union\s+{union_pat}\s*=\s*)([^\n]+(?:\n\s*\|[^\n]+)*)", text)
    if not m:
        raise RuntimeError(f"aggregate union {union_pat} not found")
    existing = re.findall(r"\|\s*(\w+)", m.group(2))
    merged = sorted(set(existing + extras), key=lambda s: s.lower())
    body = "\n" + "\n".join(f"  | {x}" for x in merged) + "\n"
    return text[: m.start()] + m.group(1) + body + text[m.end() :]


def remove_piece_mod_stub(text: str) -> str:
    """🔖 Drop minimal PieceModification stub — replaced by generated ladder."""
    return re.sub(
        r"\ntype PieceModification implements Entity \{\s*# WeakEntity\s*id: ID! # hash\s*hash: String!\s*ownerEntity: OwnerEntity\s*ownedEntities: OwnedEntityConnection\s*\}\s*\n",
        "\n",
        text,
        count=1,
    )


def main() -> None:
    path = SCHEMA if len(sys.argv) < 2 else Path(sys.argv[1])
    text = path.read_text(encoding="utf-8")
    text = rename_attributes_diff_typenames(text)
    text = remove_piece_mod_stub(text)

    prefixes = sorted(
        set(WEAK_ENTITIES) | set(STRONG_ENTITIES),
        key=str.lower,
    )

    mods: list[str] = []
    diffs: list[str] = []
    diffses: list[str] = []
    edges: list[str] = []
    conns: list[str] = []
    owners: list[str] = []

    for name in prefixes:
        if name in SKIP_ENTITIES:
            continue
        if name in SKIP_IF_LADDER_PRESENT:
            continue
        weak_entity = name in WEAK_ENTITIES
        region = ENTITY_HOME_REGION.get(name, name)
        lad = entity_ladder(name, weak_entity=weak_entity)
        text = inject_after_primary_connection(text, name, region, lad)
        mods.append(f"{name}Modification")
        diffs.append(f"{name}Diff")
        diffses.append(f"{name}Diffs")
        edges.extend([f"{name}ModificationEdge", f"{name}DiffEdge", f"{name}DiffsEdge"])
        conns.extend(
            [
                f"{name}ModificationConnection",
                f"{name}DiffConnection",
                f"{name}DiffsConnection",
            ]
        )
        owners.extend([f"{name}Modification", f"{name}Diff", f"{name}Diffs"])

    text = merge_union(text, "Modification", mods)
    text = merge_union(text, "DiffOwned", diffs)
    text = merge_union(text, "DiffsOwned", diffses)

    text = merge_aggregate_union(text, r"AggregateEntityEdge", edges)
    text = merge_aggregate_union(text, r"EntityConnection", conns)
    text = merge_aggregate_union(text, r"OwnerEntity", owners)
    text = merge_aggregate_union(text, r"OwnedEntityConnection", conns)

    path.write_text(text, encoding="utf-8")
    print(f"[inject_diff_ladders] updated {path}")


if __name__ == "__main__":
    main()
