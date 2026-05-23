import re
from pathlib import Path

p = Path("semio/graphql/target.schema.graphql")
text = p.read_text(encoding="utf-8")
pairs = [
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
]
for mod, ent, diff in pairs:
    pat = (
        rf"(type {re.escape(mod)} implements Modification \{{[\s\S]*?)"
        rf"(\n  before: Entity! # reference //)\s*modification"
        rf"(\n  diff: Diff! # reference //)\s*modification"
        rf"(\n  after: Entity! # reference //)\s*modification"
    )
    rep = rf"\1\2 {ent}\3 {diff}\4 {ent}"
    text, n = re.subn(pat, rep, text, count=1)
    if n == 0:
        print("miss", mod)
p.write_text(text, encoding="utf-8")
print("ok")
