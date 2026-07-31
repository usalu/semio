#!/usr/bin/env python3
"""Schema Inconsistency Cleanup — compose/graphql/target.schema.graphql"""
from __future__ import annotations

import re
from pathlib import Path


def repo_root() -> Path:
    p = Path(__file__).resolve().parent
    for _ in range(12):
        cand = p / "compose" / "graphql" / "target.schema.graphql"
        if cand.is_file():
            return p
        p = p.parent
    raise SystemExit("Could not locate repo root")


SCHEMA = repo_root() / "compose" / "graphql" / "target.schema.graphql"

CANON_MODS_OWNS_TAIL = (
    "PlaneModification | PositionModification | AttributeModification | "
    "LocationModification | PlaceModification"
)


def strip_edit_from_owner_line(line: str) -> str:
    if "owner: Entity # reference //" not in line:
        return line
    prefix, _, rest = line.partition("//")
    rest = rest.strip()
    if "|" not in rest or "Edit" not in rest:
        return line
    parts = [p.strip() for p in rest.split("|") if p.strip() and p.strip() != "Edit"]
    return prefix + "// " + " | ".join(parts) + ("\n" if line.endswith("\n") else "")


def remove_ghost_tokens_from_line(line: str) -> str:
    ghosts = (
        "AlternativeModification",
        "EditModification",
        "CheckpointModification",
        "ConflictModification",
        "SessionModification",
    )
    if "owns: EntityConnection # reference //" not in line:
        return line
    prefix, _, rest = line.partition("//")
    parts = [p.strip() for p in rest.split("|") if p.strip() and p.strip() not in ghosts]
    seen: set[str] = set()
    uniq = []
    for p in parts:
        if p not in seen:
            seen.add(p)
            uniq.append(p)
    if not uniq:
        return prefix.rstrip() + "\n" if line.endswith("\n") else prefix.rstrip()
    return prefix + "// " + " | ".join(uniq) + ("\n" if line.endswith("\n") else "")


def bare_owns_modification_blocks(text: str) -> str:
    """interface Modification + type *Modification (not *Modifications): owns without union."""

    def repl_iface(m: re.Match) -> str:
        block = m.group(0)
        return re.sub(
            r"\n  owns: EntityConnection # reference[^\n]*\n",
            "\n  owns: EntityConnection # reference\n",
            block,
            count=1,
        )

    text = re.sub(
        r"interface Modification implements WeakEntity \{[\s\S]*?\n\}",
        repl_iface,
        text,
        count=1,
    )

    def repl_concrete(m: re.Match) -> str:
        block = m.group(0)
        return re.sub(
            r"\n  owns: EntityConnection # reference[^\n]*\n",
            "\n  owns: EntityConnection # reference\n",
            block,
            count=1,
        )

    text = re.sub(
        r"type (\w+Modification) implements Modification \{[\s\S]*?\n\}",
        repl_concrete,
        text,
    )
    return text


MOD_OWNER: dict[str, str] = {}
ATTR_OPS = (
    "AddedAttributeToConcept | AddedAttributesToConcept | RemovedAttributeFromConcept | RemovedAttributesFromConcept | "
    "AddedAttributeToDesign | AddedAttributesToDesign | RemovedAttributeFromDesign | RemovedAttributesFromDesign | "
    "AddedAttributeToPiece | AddedAttributesToPiece | RemovedAttributeFromPiece | RemovedAttributesFromPiece | "
    "AddedAttributeToPort | AddedAttributesToPort | RemovedAttributeFromPort | RemovedAttributesFromPort | "
    "AddedAttributeToQuality | AddedAttributesToQuality | RemovedAttributeFromQuality | RemovedAttributesFromQuality | "
    "AddedAttributeToTag | AddedAttributesToTag | RemovedAttributeFromTag | RemovedAttributesFromTag | "
    "AddedAttributeToType | AddedAttributesToType | RemovedAttributeFromType | RemovedAttributesFromType"
)
CONTAINERS = "Modifications | KitModifications | TypeModifications | DesignModifications | PieceModifications"
MOD_OWNER["AttributeModification"] = f"{ATTR_OPS} | AttributeModifications | {CONTAINERS}"
MOD_OWNER["QualityModification"] = (
    "CreatedQuality | CreatedQualities | RenamedQuality | UpdatedQualityDescription | UpdatedQualityIcon | "
    "DeletedQuality | DeletedQualities | KitModifications | Modifications"
)
MOD_OWNER["TagModification"] = (
    "CreatedTag | CreatedTags | RenamedTag | UpdatedTagDescription | UpdatedTagIcon | DeletedTag | DeletedTags | "
    "KitModifications | Modifications"
)
MOD_OWNER["ConceptModification"] = (
    "CreatedConcept | CreatedConcepts | RenamedConcept | UpdatedConceptDescription | UpdatedConceptIcon | "
    "DeletedConcept | DeletedConcepts | KitModifications | Modifications"
)
MOD_OWNER["PortModification"] = (
    "CreatedPort | CreatedPorts | RenamedPort | UpdatedPortDescription | UpdatedPortIcon | DeletedPort | DeletedPorts | "
    "TypeModifications | Modifications"
)
MOD_OWNER["ConnectorModification"] = (
    "AddedConnector | AddedConnectors | RenamedConnector | UpdatedConnectorDescription | UpdatedConnectorIcon | "
    "RemovedConnector | RemovedConnectors | TypeModifications | Modifications"
)
MOD_OWNER["TypeModification"] = (
    "CreatedType | CreatedTypes | RenamedType | UpdatedTypeDescription | UpdatedTypeIcon | DeletedType | DeletedTypes | "
    "KitModifications | Modifications"
)
MOD_OWNER["PieceModification"] = (
    "CreatedFixedPiece | DraggedPiece | DraggedPieces | MovedPiece | MovedPieces | RenamedPiece | UpdatedPieceDescription | "
    "FixedPiece | FixedPieces | ChangedPieceToType | ChangedPiecesToType | "
    "AddedChildPieceWithParentConnection | AddedChildPiecesWithParentConnections | "
    "AddedHangingChildPieceWithParentConnection | AddedHangingChildPiecesWithParentConnections | "
    "DeletedPiece | DeletedPieces | DeletedPiecesAndConnections | DesignModifications | Modifications"
)
MOD_OWNER["ConnectionModification"] = (
    "AddedChildPieceWithParentConnection | AddedChildPiecesWithParentConnections | "
    "AddedHangingChildPieceWithParentConnection | AddedHangingChildPiecesWithParentConnections | "
    "DeletedPiecesAndConnections | DesignModifications | Modifications"
)
MOD_OWNER["DesignModification"] = (
    "CreatedDesign | CreatedDesigns | DeletedDesign | DeletedDesigns | FlattenedDesign | RenamedKit | ChangedDescription | "
    "KitModifications | Modifications"
)
MOD_OWNER["KitModification"] = "RenamedKit | ChangedDescription | Modifications"
MOD_OWNER["LayerModification"] = "DesignModifications | Modifications"
MOD_OWNER["GroupModification"] = "DesignModifications | Modifications"
MOD_OWNER["RepresentationModification"] = "TypeModifications | Modifications"
MOD_OWNER["StatModification"] = "Modifications"
for n in (
    "BenchmarkModification",
    "PropModification",
    "AuthorModification",
    "FamilyModification",
    "FolderModification",
    "FileModification",
    "PlaceModification",
):
    MOD_OWNER[n] = f"{n.replace('Modification', 'Modifications')} | Modifications"
MOD_OWNER["SideModification"] = "ConnectionModifications | Modifications"
GEO = (
    "MovedPiece | MovedPieces | DraggedPiece | DraggedPieces | CreatedFixedPiece | "
    "AddedHangingChildPieceWithParentConnection | AddedHangingChildPiecesWithParentConnections | FlattenedDesign | "
    "PieceModifications | DesignModifications | KitModifications | Modifications"
)
for g in (
    "VectorModification",
    "PointModification",
    "CoordinateModification",
    "OffsetModification",
    "PlaneModification",
    "LocationModification",
):
    MOD_OWNER[g] = GEO
MOD_OWNER["PositionModification"] = (
    "MovedPiece | MovedPieces | DraggedPiece | DraggedPieces | CreatedFixedPiece | "
    "AddedHangingChildPieceWithParentConnection | AddedHangingChildPiecesWithParentConnections | FlattenedDesign | "
    "PieceModifications | Modifications"
)

MODS_OWNER = {k.replace("Modification", "Modifications"): v for k, v in MOD_OWNER.items()}


def set_interface_modification_owner_operation(text: str) -> str:
    return re.sub(
        r"(interface Modification implements WeakEntity \{[\s\S]*?)(  owner: Entity # reference //)[^\n]+",
        r"\1\2 Operation",
        text,
        count=1,
    )


def set_type_modifications_owner_operation(text: str) -> str:
    return re.sub(
        r"(type Modifications implements WeakEntity \{[\s\S]*?)(  owner: Entity # reference //)[^\n]+",
        r"\1\2 Operation",
        text,
        count=1,
    )


def apply_mod_owner(text: str) -> str:
    for mod, union in MOD_OWNER.items():
        text = re.sub(
            rf"(type {re.escape(mod)} implements Modification \{{[\s\S]*?)(  owner: Entity # reference //)[^\n]+",
            rf"\1\2 {union}",
            text,
            count=1,
        )
    return text


def apply_mods_owner(text: str) -> str:
    for mods, union in MODS_OWNER.items():
        text = re.sub(
            rf"(type {re.escape(mods)} implements WeakEntity \{{[\s\S]*?)(  owner: Entity # reference //)[^\n]+",
            rf"\1\2 {union}",
            text,
            count=1,
        )
    return text


def fix_tag_mods_owns(text: str) -> str:
    tag_owns = (
        "TagModifications | TagModification | PlaneModification | PositionModification | "
        "AttributeModification | LocationModification | PlaceModification"
    )
    text = re.sub(
        r"(type TagModifications implements WeakEntity \{[\s\S]*?\n)(  owns: EntityConnection # reference //)[^\n]+",
        rf"\1\2 {tag_owns}",
        text,
        count=1,
    )
    for name in ("PositionModifications", "LocationModifications", "PlaceModifications"):
        item = name.replace("Modifications", "Modification")
        tail = CANON_MODS_OWNS_TAIL
        text = re.sub(
            rf"(type {re.escape(name)} implements WeakEntity \{{[\s\S]*?\n)(  owns: EntityConnection # reference //)[^\n]+",
            rf"\1\2 {name} | {item} | {tail}",
            text,
            count=1,
        )
    return text


def narrow_modification_triple(text: str) -> str:
    for mod, ent, diff in [
        ("VectorModification", "Vector", "VectorDiff"),
        ("PointModification", "Point", "PointDiff"),
        ("CoordinateModification", "Coordinate", "CoordinateDiff"),
        ("OffsetModification", "Offset", "OffsetDiff"),
        ("PlaneModification", "Plane", "PlaneDiff"),
        ("PositionModification", "Position", "PositionDiff"),
        ("LocationModification", "Location", "LocationDiff"),
        ("AttributeModification", "Attribute", "AttributeDiff"),
        ("PlaceModification", "Place", "PlaceDiff"),
        ("FamilyModification", "Family", "FamilyDiff"),
        ("FolderModification", "Folder", "FolderDiff"),
        ("FileModification", "File", "FileDiff"),
        ("AuthorModification", "Author", "AuthorDiff"),
        ("PropModification", "Prop", "PropDiff"),
        ("BenchmarkModification", "Benchmark", "BenchmarkDiff"),
        ("QualityModification", "Quality", "QualityDiff"),
        ("TagModification", "Tag", "TagDiff"),
        ("ConceptModification", "Concept", "ConceptDiff"),
        ("StatModification", "Stat", "StatDiff"),
        ("PortModification", "Port", "PortDiff"),
        ("ConnectorModification", "Connector", "ConnectorDiff"),
        ("RepresentationModification", "Representation", "RepresentationDiff"),
        ("TypeModification", "Type", "TypeDiff"),
        ("LayerModification", "Layer", "LayerDiff"),
        ("GroupModification", "Group", "GroupDiff"),
        ("PieceModification", "Piece", "PieceDiff"),
        ("ConnectionModification", "Connection", "ConnectionDiff"),
        ("SideModification", "Side", "SideDiff"),
        ("DesignModification", "Design", "DesignDiff"),
        ("KitModification", "Kit", "KitDiff"),
    ]:
        text = re.sub(
            rf"(type {re.escape(mod)} implements Modification \{{[\s\S]*?)(\n  before: Entity! # reference //)\s*modification(\n  diff: Diff! # reference //)\s*modification(\n  after: Entity! # reference //)\s*modification",
            rf"\1\2 {ent}\3 {diff}\4 {ent}",
            text,
            count=1,
        )
    return text


def prune_diffs_and_modaggregates(text: str) -> str:
    group_diff = """type GroupDiff implements Diff {
  # Node
  id: ID! # computed // hash
  # Entity
  hash: String! # cached
  owner: Entity # reference // GroupModification
  owns: EntityConnection # reference
  # GroupDiff
  removeDescription: Boolean # computed
  color: String! # computed
  removeColor: Boolean # computed
  removeIcon: Boolean # computed
  pieces: PieceConnection # computed
}
"""
    piece_diff = """type PieceDiff implements Diff {
  # Node
  id: ID! # computed // hash
  # Entity
  hash: String! # cached
  owner: Entity # reference // PieceModification
  owns: EntityConnection # reference
  # PieceDiff
  removeName: Boolean # computed
  removeDescription: Boolean # computed
  position: Position # computed
  removePosition: Boolean # computed
  scale: Float # computed
  removeScale: Boolean # computed
  blueprint: Entity # computed // Type | Design
  props: PropConnection # computed
  attributes: AttributeConnection # computed
}
"""
    design_diff = """type DesignDiff implements Diff {
  # Node
  id: ID! # computed // hash
  # Entity
  hash: String! # cached
  owner: Entity # reference // DesignModification
  owns: EntityConnection # reference
  # DesignDiff
  removeDescription: Boolean # computed
  removeIcon: Boolean # computed
  image: String! # computed
  removeImage: Boolean # computed
  place: Place # computed
  removePlace: Boolean # computed
  unit: String! # computed
  removeUnit: Boolean # computed
  removeCreatedBy: Boolean # computed
  removeAuthoredBy: Boolean # computed
  removeChangedIn: Boolean # computed
  removeLastChangedBy: Boolean # computed
  removeLastChangedIn: Boolean # computed
}
"""
    for name, body in (
        ("GroupDiff", group_diff),
        ("PieceDiff", piece_diff),
        ("DesignDiff", design_diff),
    ):
        text = re.sub(rf"type {name} implements Artifact \{{[\s\S]*?\n}}\n", body, text, count=1)

    for name in ("GroupModifications", "PieceModifications", "DesignModifications"):
        item = name.replace("Modifications", "Modification")
        body = f"""type {name} implements WeakEntity {{
  # Node
  id: ID! # computed // hash
  # Entity
  hash: String! # cached
  owner: Entity # reference // Operation
  owns: EntityConnection # reference // {name} | {item} | {CANON_MODS_OWNS_TAIL}
  # Modifications
  removed: EntityConnection # computed
  modifications: {item}Connection # computed
  added: EntityConnection # computed
}}
"""
        text = re.sub(rf"type {name} implements Artifact \{{[\s\S]*?\n}}\n", body, text, count=1)
    return text


def all_modifications_weak(text: str) -> str:
    return re.sub(
        r"type (\w+Modifications) implements Entity \{",
        r"type \1 implements WeakEntity {",
        text,
    )


def operation_interface_strong(text: str) -> str:
    return text.replace(
        "interface Operation implements Entity { # implements StrongEntity",
        "interface Operation implements StrongEntity {",
        1,
    )


def interface_modification_generic_triple(text: str) -> str:
    return text.replace(
        "  before: Entity! # reference // modification\n  diff: Diff! # reference // modification\n  after: Entity! # reference // modification",
        "  before: Entity! # reference // Entity\n  diff: Diff! # reference // Diff\n  after: Entity! # reference // Entity",
        1,
    )


def file_type_design_fixes(text: str) -> str:
    # File
    text = text.replace(
        "type File implements Artifact {\n  # Node\n  id: ID! # data // uuidv7\n  # Entity\n  hash: String! # cached\n  owner: Entity # reference // Kit | Folder\n  owns: EntityConnection # reference\n  # Artifact\n  name: String! # data\n  description: String! # data\n  icon: String! # data\n  createdAt: Timestamp # computed\n  createdBy: Author # data\n  authoredBy: AuthorConnection # data\n  changedIn: CheckpointConnection # data\n  lastChangedAt: Timestamp # computed\n  lastChangedBy: Author # data\n  lastChangedIn: Checkpoint # data\n  edits: EditConnection # computed",
        "type File implements Artifact {\n  # Node\n  id: ID! # data // uuidv7\n  # Entity\n  hash: String! # cached\n  owner: Entity # reference // Kit | Folder\n  owns: EntityConnection # reference\n  # Artifact\n  name: String! # data\n  description: String! # data\n  icon: String! # data\n  createdAt: Timestamp # computed\n  createdBy: Author # computed\n  authoredBy: AuthorConnection # computed\n  changedIn: CheckpointConnection # computed\n  lastChangedAt: Timestamp # computed\n  lastChangedBy: Author # computed\n  lastChangedIn: Checkpoint # computed\n  changes: ChangeConnection # computed\n  edits: EditConnection # computed",
        1,
    )
    # Type — locate Type implements Artifact block start
    text = text.replace(
        "  createdBy: Author # data\n  authoredBy: AuthorConnection # data\n  changedIn: CheckpointConnection # data\n  lastChangedAt: Timestamp # computed\n  lastChangedBy: Author # data\n  lastChangedIn: Checkpoint # data\n  edits: EditConnection # computed\n  # Type",
        "  createdBy: Author # computed\n  authoredBy: AuthorConnection # computed\n  changedIn: CheckpointConnection # computed\n  lastChangedAt: Timestamp # computed\n  lastChangedBy: Author # computed\n  lastChangedIn: Checkpoint # computed\n  changes: ChangeConnection # computed\n  edits: EditConnection # computed\n  # Type",
        1,
    )
    text = text.replace(
        "type Type implements Artifact {\n  # Node\n  id: ID! # data // uuidv7\n  # Entity\n  hash: String! # cached\n  owner: Entity # reference // Kit | Representation\n  owns: EntityConnection # reference\n  # RichStrongEntity\n  name: String! # data\n  description: String! # data\n  icon: String! # data\n  createdAt: Timestamp # computed\n  createdBy: Author # data",
        "type Type implements Artifact {\n  # Node\n  id: ID! # data // uuidv7\n  # Entity\n  hash: String! # cached\n  owner: Entity # reference // Kit | Representation\n  owns: EntityConnection # reference\n  # RichStrongEntity\n  name: String! # data\n  description: String! # data\n  icon: String! # data\n  createdAt: Timestamp # computed\n  createdBy: Author # computed",
        1,
    )
    # Design
    text = text.replace(
        "  createdBy: Author # data\n  authoredBy: AuthorConnection # data\n  changedIn: CheckpointConnection # data\n  lastChangedAt: Timestamp # computed\n  lastChangedBy: Author # data\n  lastChangedIn: Checkpoint # data\n  edits: EditConnection # computed\n  # Design",
        "  createdBy: Author # computed\n  authoredBy: AuthorConnection # computed\n  changedIn: CheckpointConnection # computed\n  lastChangedAt: Timestamp # computed\n  lastChangedBy: Author # computed\n  lastChangedIn: Checkpoint # computed\n  changes: ChangeConnection # computed\n  edits: EditConnection # computed\n  # Design",
        1,
    )
    return text


def delete_mod_attr_conn(text: str) -> str:
    return re.sub(
        r"\ntype ModificationAttributesConnection implements EntityConnection \{[\s\S]*?\n\}\n",
        "\n",
        text,
        count=1,
    )


def insert_clump_edges(text: str) -> str:
    block = """
type ClumpEdge implements EntityEdge {
  # EntityEdge
  cursor: String! # computed
  # ClumpEdge
  node: Clump! # reference
}

type ClumpConnection implements EntityConnection {
  # EntityConnection
  edges: [ClumpEdge!]! # computed
  pageInfo: PageInfo! # computed
  hash: String! # cached
}

"""
    return text.replace("#endregion Clump\n", block + "#endregion Clump\n", 1)


def insert_thekit_edges(text: str) -> str:
    block = """
type TheKitEdge implements EntityEdge {
  # EntityEdge
  cursor: String! # computed
  # TheKitEdge
  node: TheKit! # reference
}

type TheKitConnection implements EntityConnection {
  # EntityConnection
  edges: [TheKitEdge!]! # computed
  pageInfo: PageInfo! # computed
  hash: String! # cached
}

"""
    return text.replace(
        "type Alternative implements Version {",
        block + "type Alternative implements Version {",
        1,
    )


def blueprint_comment(text: str) -> str:
    return text.replace(
        "type BlueprintEdge implements EntityEdge {\n  # EntityEdge",
        "type BlueprintEdge implements EntityEdge {\n  # Polymorphic edge over Type | Design (no Blueprint type)\n  # EntityEdge",
        1,
    )


def tag_misc(text: str) -> str:
    text = text.replace(
        "  involves: EntityConnection\n",
        "  involves: EntityConnection # reference\n",
        1,
    )
    text = text.replace(
        "  removed: EntityConnection # data\n  modifications: ModificationConnection # data\n  added: EntityConnection # data",
        "  removed: EntityConnection # computed\n  modifications: ModificationConnection # computed\n  added: EntityConnection # computed",
        1,
    )
    text = re.sub(
        r"(type RenamedKitInput implements Input \{[\s\S]*?\n  name: String!) (\n)",
        r"\1 # data\2",
        text,
        count=1,
    )
    text = re.sub(
        r"(type ChangedDescriptionInput implements Input \{[\s\S]*?\n  description: String!) (\n)",
        r"\1 # data\2",
        text,
        count=1,
    )
    return text


def ensure_diff_banners(text: str) -> str:
    """🧩 Insert `# Diff` before `# <Name>Diff` on every concrete Diff type (idempotent)."""

    def repl_block(m: re.Match[str]) -> str:
        block = m.group(0)
        if "\n  # Diff\n" in block:
            return block
        return re.sub(
            r"(\n  owns: EntityConnection # reference[^\n]*\n)(  # \w+Diff\n)",
            r"\1  # Diff\n\2",
            block,
            count=1,
        )

    return re.sub(r"type \w+Diff implements Diff \{[\s\S]*?\n\}", repl_block, text)


def narrow_placeholder_operation_scopes(text: str) -> str:
    """🧩 Replace generic `scope: … // Entity` with the real scope kind for kit/type/design entry ops."""

    specs: list[tuple[str, str]] = [
        ("CreatedQualities", "KitModifications"),
        ("CreatedQuality", "KitModifications"),
        ("CreatedTags", "KitModifications"),
        ("CreatedTag", "KitModifications"),
        ("CreatedConcepts", "KitModifications"),
        ("CreatedConcept", "KitModifications"),
        ("CreatedPorts", "TypeModifications"),
        ("CreatedPort", "TypeModifications"),
        ("CreatedTypes", "KitModifications"),
        ("CreatedType", "KitModifications"),
        ("CreatedDesigns", "KitModifications"),
        ("CreatedDesign", "KitModifications"),
        ("ChangedDescription", "Kit"),
    ]
    for op, scp in specs:
        text = re.sub(
            rf"(type {re.escape(op)} implements Operation \{{[\s\S]*?)(  scope: Entity! # reference // Entity\n)",
            rf"\1  scope: Entity! # reference // {scp}\n",
            text,
            count=1,
        )
    return text


def find_brace_end(s: str, open_pos: int) -> int:
    depth = 0
    i = open_pos
    while i < len(s):
        if s[i] == "{":
            depth += 1
        elif s[i] == "}":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    return -1


def normalize_operation_type_blocks(text: str) -> str:
    out: list[str] = []
    i = 0
    n = len(text)
    while i < n:
        m = re.search(r"type (\w+) implements Operation \{", text[i:])
        if not m:
            out.append(text[i:])
            break
        out.append(text[i : i + m.start()])
        name = m.group(1)
        start = i + m.start()
        brace_open = text.index("{", start)
        end = find_brace_end(text, brace_open)
        if end < 0:
            out.append(text[i:])
            break
        inner = text[brace_open + 1 : end - 1]
        fields: dict[str, str] = {}
        order: list[str] = []
        for raw in inner.splitlines():
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            mm = re.match(r"(\w+):", line)
            if mm:
                k = mm.group(1)
                fields[k] = raw
                if k not in order:
                    order.append(k)
        parts = [f"type {name} implements Operation {{"]
        parts.append("  # Node")
        for k in ("id",):
            if k in fields:
                parts.append(fields[k])
        parts.append("  # Entity")
        for k in ("hash", "owner", "owns"):
            if k in fields:
                parts.append(fields[k])
        parts.append("  # Operation")
        for k in ("scope", "input", "modification"):
            if k in fields:
                parts.append(fields[k])
        outs = [k for k in order if k not in ("id", "hash", "owner", "owns", "scope", "input", "modification")]
        if outs:
            parts.append("  # Operation Output")
            for k in outs:
                parts.append(fields[k])
        parts.append("}")
        out.append("\n".join(parts))
        if end < n and text[end] == "\n":
            out.append("\n")
            end += 1
        i = end
    return "".join(out)


def add_operation_edges(text: str) -> str:
    for m in reversed(list(re.finditer(r"type (\w+) implements Operation \{", text))):
        name = m.group(1)
        if f"type {name}Edge implements EntityEdge" in text:
            continue
        brace_open = text.index("{", m.start())
        end = find_brace_end(text, brace_open)
        if end < 0:
            continue
        insert = f"""

type {name}Edge implements EntityEdge {{
  # EntityEdge
  cursor: String! # computed
  # {name}Edge
  node: {name}! # reference
}}

type {name}Connection implements EntityConnection {{
  # EntityConnection
  edges: [{name}Edge!]! # computed
  pageInfo: PageInfo! # computed
  hash: String! # cached
}}
"""
        text = text[:end] + insert + text[end:]
    return text


def regions(text: str) -> str:
    text = text.replace("#region Kit\n", "#region Kit Entities\n", 1)
    text = text.replace("\n#endregion Entities\n", "\n", 1)
    text = text.replace(
        "#endregion VCS\n\n#region Schema",
        "#endregion VCS\n#endregion Entities\n\n#region Schema",
        1,
    )
    return text


def move_design_before_clump(text: str) -> str:
    m_clump = re.search(r"#region Clump\n", text)
    if not m_clump:
        return text
    m_design = re.search(
        r"type Design implements Artifact \{[\s\S]*?type DesignModificationsConnection implements EntityConnection \{[\s\S]*?\n\}\n",
        text,
    )
    if not m_design or m_design.start() < m_clump.start():
        return text
    block = m_design.group(0)
    text = text[: m_design.start()] + text[m_design.end() :]
    m2 = re.search(r"#region Clump\n", text)
    if not m2:
        return text
    return text[: m2.start()] + block + text[m2.start() :]


def main() -> None:
    text = SCHEMA.read_text(encoding="utf-8")

    lines = [remove_ghost_tokens_from_line(strip_edit_from_owner_line(ln)) for ln in text.splitlines(keepends=True)]
    text = "".join(lines)

    text = bare_owns_modification_blocks(text)
    text = set_interface_modification_owner_operation(text)
    text = set_type_modifications_owner_operation(text)
    text = prune_diffs_and_modaggregates(text)
    text = operation_interface_strong(text)
    text = all_modifications_weak(text)
    text = interface_modification_generic_triple(text)
    text = apply_mod_owner(text)
    text = apply_mods_owner(text)
    text = fix_tag_mods_owns(text)
    text = narrow_modification_triple(text)
    text = file_type_design_fixes(text)
    text = delete_mod_attr_conn(text)
    text = insert_clump_edges(text)
    text = insert_thekit_edges(text)
    text = blueprint_comment(text)
    text = tag_misc(text)
    text = normalize_operation_type_blocks(text)
    text = add_operation_edges(text)
    text = regions(text)
    text = move_design_before_clump(text)
    text = ensure_diff_banners(text)
    text = narrow_placeholder_operation_scopes(text)

    SCHEMA.write_text(text, encoding="utf-8")
    print("OK", SCHEMA)


if __name__ == "__main__":
    main()
