from pathlib import Path

entities = [
    "Kit", "Attribute", "Author", "Location", "Folder", "File", "Concept", "Quality",
    "Benchmark", "Stat", "Tag", "Model", "Port", "Connector", "Prop", "Layer", "Group",
    "Piece", "Connection", "Type", "Design",
]
out = ["// Per-entity DTO newtypes (generated)\n"]
for e in entities:
    out.append(
        f"""#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct {e}IdDto(pub IdDto);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {e}InputDto(pub InputDto);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {e}MetadataDto(pub MetadataRecord);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {e}ShallowDto(pub ShallowRecord);

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct {e}FullDto(pub FullRecord);
"""
    )
Path("oop_dto_entities.inc.rs").write_text("\n".join(out), encoding="utf-8")
