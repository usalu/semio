"""Regenerate per-entity DTO newtypes inside semio/rs/lib.rs.

Run from repo root or from this directory:
  python semio/rs/gen_dto_wrappers.py
"""

from __future__ import annotations

from pathlib import Path


ENTITY_NAMES = [
    "Kit",
    "Attribute",
    "Author",
    "Location",
    "Folder",
    "File",
    "Concept",
    "Quality",
    "Benchmark",
    "Stat",
    "Tag",
    "Model",
    "Port",
    "Connector",
    "Prop",
    "Layer",
    "Group",
    "Piece",
    "Connection",
    "Type",
    "Design",
]

REGION_BEGIN = "// region generated oop_dto_entities (semio/rs/gen_dto_wrappers.py)\n"
REGION_END = "// endregion generated oop_dto_entities\n"


def emit_entities_source() -> str:
    lines = ["// Per-entity DTO newtypes (generated)\n"]
    for name in ENTITY_NAMES:
        lines.append(
            f"""#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct {name}IdDto(pub IdDto);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {name}InputDto(pub InputDto);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {name}MetadataDto(pub MetadataRecord);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {name}ShallowDto(pub ShallowRecord);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {name}FullDto(pub FullRecord);
"""
        )
    return "".join(lines)


def main() -> None:
    root = Path(__file__).resolve().parent
    lib = root / "lib.rs"
    text = lib.read_text(encoding="utf-8")
    if REGION_BEGIN not in text or REGION_END not in text:
        raise SystemExit(
            "Could not find region markers in lib.rs; expected:\n"
            f"  {REGION_BEGIN.strip()}\n  ...\n  {REGION_END.strip()}"
        )
    before, _mid, tail = text.partition(REGION_BEGIN)
    _inner, _end, after = tail.partition(REGION_END)
    new_inner = emit_entities_source().rstrip() + "\n"
    new_text = before + REGION_BEGIN + new_inner + REGION_END + after
    lib.write_text(new_text, encoding="utf-8")
    print(f"Updated {lib}")


if __name__ == "__main__":
    main()
