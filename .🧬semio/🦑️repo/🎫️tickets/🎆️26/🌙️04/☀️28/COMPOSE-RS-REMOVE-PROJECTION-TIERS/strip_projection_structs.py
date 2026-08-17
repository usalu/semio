#!/usr/bin/env python3
"""Strip projection-tier structs from compose/rs/lib.rs (Metadata, Shallow, id-only Ref).
   Renames *Full -> *Bundle, PoseFull -> Placement. Preserves EntityRef and *StoreRef."""
from __future__ import annotations

import re
from pathlib import Path

RS = Path("compose/rs/lib.rs")
text = RS.read_text(encoding="utf-8")

# --- Protect aliases that end with Ref but are not id-only projection structs ---
PROTECT = [
    "EntityRef",
    "AttributeStoreRef",
    "AuthorStoreRef",
    "BenchmarkStoreRef",
    "ConceptStoreRef",
    "ConnectionStoreRef",
    "ConnectorStoreRef",
    "DesignStoreRef",
    "FileStoreRef",
    "FolderStoreRef",
    "LocationStoreRef",
    "GroupStoreRef",
    "KitStoreRef",
    "LayerStoreRef",
    "PieceStoreRef",
    "PortStoreRef",
    "FamilyStoreRef",
    "PropStoreRef",
    "QualityStoreRef",
    "RepresentationStoreRef",
    "SideStoreRef",
    "StatStoreRef",
    "TagStoreRef",
    "TypeStoreRef",
    "KitGraphRef",
    "KitGraphWeak",
    "DesignStoreWeak",
    "TypeStoreWeak",
    "PieceStoreWeak",
    "ConnectionStoreWeak",
    "ConnectorStoreWeak",
    "RepresentationStoreWeak",
    "FamilyStoreWeak",
    "PortStoreWeak",
    "AttributeStoreWeak",
    "AuthorStoreWeak",
    "BenchmarkStoreWeak",
    "ConceptStoreWeak",
    "FileStoreWeak",
    "FolderStoreWeak",
    "LocationStoreWeak",
    "GroupStoreWeak",
    "LayerStoreWeak",
    "PropStoreWeak",
    "QualityStoreWeak",
    "StatStoreWeak",
    "TagStoreWeak",
    "KitGraphWeak",
]
PLACE = []
for i, name in enumerate(PROTECT):
    tok = f"§§PROT{i}§§"
    PLACE.append((tok, name))
    text = text.replace(name, tok)


def strip_struct_named(src: str, struct_name: str) -> str:
    """Remove `pub struct Name { ... }` with preceding #[derive(...)] lines."""
    i = 0
    out = []
    n = len(src)
    needle = f"pub struct {struct_name} {{"
    while i < n:
        j = src.find(needle, i)
        if j == -1:
            out.append(src[i:])
            break
        # include leading derives / attrs
        k = j
        while k > 0 and src[k - 1] != "\n":
            k -= 1
        line_start = src.rfind("\n", 0, j)
        line_start = 0 if line_start == -1 else line_start + 1
        # walk back over consecutive attribute lines
        attr_start = line_start
        p = line_start
        while p > 0:
            prev_nl = src.rfind("\n", 0, p - 1)
            prev_line = src[prev_nl + 1 : p] if prev_nl != -1 else src[:p]
            pl = prev_line.strip()
            if pl.startswith("#[") or pl.startswith("#![") or pl == "":
                attr_start = prev_nl + 1 if prev_nl != -1 else 0
                p = attr_start - 1 if attr_start > 0 else 0
                continue
            break
        brace_open = src.find("{", j)
        if brace_open == -1:
            out.append(src[i:])
            break
        depth = 0
        t = brace_open
        while t < n:
            c = src[t]
            if c == "{":
                depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0:
                    t += 1
                    # consume trailing whitespace / one newline
                    while t < n and src[t] in " \t":
                        t += 1
                    if t < n and src[t] == "\n":
                        t += 1
                    out.append(src[i:attr_start])
                    i = t
                    break
            t += 1
        else:
            out.append(src[i:])
            break
    return "".join(out)


# Id-only *Ref structs to remove (not EntityRef — never added to list)
REF_STRUCTS = [
    "AttributeRef",
    "AuthorRef",
    "BenchmarkRef",
    "ConceptRef",
    "ConnectionRef",
    "ConnectorRef",
    "DesignRef",
    "FileRef",
    "FolderRef",
    "LocationRef",
    "GroupRef",
    "KitRef",
    "LayerRef",
    "PieceRef",
    "PortRef",
    "FamilyRef",
    "PropRef",
    "QualityRef",
    "RepresentationRef",
    "SideRef",
    "StatRef",
    "TagRef",
    "TypeRef",
]

META_STRUCTS = [
    "AttributeMetadata",
    "AuthorMetadata",
    "BenchmarkMetadata",
    "ConceptMetadata",
    "ConnectionMetadata",
    "ConnectorMetadata",
    "DesignMetadata",
    "FileMetadata",
    "FolderMetadata",
    "LocationMetadata",
    "GroupMetadata",
    "KitMetadata",
    "LayerMetadata",
    "PieceMetadata",
    "PortMetadata",
    "FamilyMetadata",
    "PropMetadata",
    "QualityMetadata",
    "RepresentationMetadata",
    "SideMetadata",
    "StatMetadata",
    "TagMetadata",
    "TypeMetadata",
]

SHALLOW_STRUCTS = [
    "AttributeShallow",
    "AuthorShallow",
    "BenchmarkShallow",
    "ConceptShallow",
    "ConnectionShallow",
    "ConnectorShallow",
    "DesignShallow",
    "FileShallow",
    "FolderShallow",
    "LocationShallow",
    "GroupShallow",
    "KitShallow",
    "LayerShallow",
    "PieceShallow",
    "PortShallow",
    "FamilyShallow",
    "PropShallow",
    "QualityShallow",
    "RepresentationShallow",
    "SideShallow",
    "StatShallow",
    "TagShallow",
    "TypeShallow",
]

for name in REF_STRUCTS + META_STRUCTS + SHALLOW_STRUCTS:
    text = strip_struct_named(text, name)

# Rename *Full -> *Bundle (longest entity names first to avoid partial issues)
ENTITIES = sorted(
    {
        "Representation",
        "Connection",
        "Attribute",
        "Benchmark",
        "Connector",
        "Attribute",
        "Author",
        "Concept",
        "Design",
        "Family",
        "Folder",
        "Location",
        "Quality",
        "Layer",
        "Piece",
        "Stat",
        "Group",
        "Folder",
        "File",
        "Port",
        "Prop",
        "Side",
        "Tag",
        "Type",
        "Kit",
    },
    key=len,
    reverse=True,
)

text = text.replace("PoseFull", "Placement")

for e in ENTITIES:
    text = text.replace(f"{e}Full", f"{e}Bundle")

# Merge type names: projection tiers -> Bundle
for e in ENTITIES:
    text = text.replace(f"{e}Metadata", f"{e}Bundle")
    text = text.replace(f"{e}Shallow", f"{e}Bundle")

for name in REF_STRUCTS:
    simple = name.replace("Ref", "")
    # TypeRef -> Type + Ref -> Type only wrong. REF_STRUCTS are XRef where X is prefix.
    text = text.replace(name, "Id")

# Methods
pairs = [
    ("apply_full_fields", "apply_bundle_fields"),
    ("from_full", "from_bundle"),
    ("from_shallow", "from_bundle"),
    ("to_full", "to_bundle"),
    ("to_shallow", "to_bundle"),
    ("to_metadata", "to_bundle"),
    ("to_ref", "to_key"),
    ("replace_from_full", "replace_from_bundle"),
    ("ReplaceKitFromFull", "ReplaceKitFromBundle"),
    ("ReadKitFullCommand", "ReadKitBundleCommand"),
    ("read_kit_full_command", "read_kit_bundle_command"),
    ("ReadTypeFullCommand", "ReadTypeBundleCommand"),
    ("ReadDesignFullCommand", "ReadDesignBundleCommand"),
    ("ReadPieceFullCommand", "ReadPieceBundleCommand"),
    ("ReadConnectionFullCommand", "ReadConnectionBundleCommand"),
    ("ReadKitTypesFullCommand", "ReadKitTypesBundleCommand"),
    ("ReadKitDesignsFullCommand", "ReadKitDesignsBundleCommand"),
    ("ReadKitFilesFullCommand", "ReadKitFilesBundleCommand"),
    ("ReadKitFoldersFullCommand", "ReadKitFoldersBundleCommand"),
    ("ReadKitLocationsFullCommand", "ReadKitLocationsBundleCommand"),
    ("ReadKitFamiliesFullCommand", "ReadKitFamiliesBundleCommand"),
    ("ReadKitPortsFullCommand", "ReadKitPortsBundleCommand"),
    ("ReadKitAuthorsFullCommand", "ReadKitAuthorsBundleCommand"),
    ("ReadKitConceptsFullCommand", "ReadKitConceptsBundleCommand"),
    ("ReadKitTagsFullCommand", "ReadKitTagsBundleCommand"),
    ("ReadKitQualitiesFullCommand", "ReadKitQualitiesBundleCommand"),
    ("ReadKitPropsFullCommand", "ReadKitPropsBundleCommand"),
    ("ReadKitAttributesFullCommand", "ReadKitAttributesBundleCommand"),
    ("the_kit_dto", "the_kit_bundle"),
    ("KitFullSnapshot", "KitBundleSnapshot"),
    ("GqlKitFullSnapshot", "GqlKitBundleSnapshot"),
]
for a, b in pairs:
    text = text.replace(a, b)

# Restore protected
for tok, name in PLACE:
    text = text.replace(tok, name)

RS.write_text(text, encoding="utf-8")
print("done")
