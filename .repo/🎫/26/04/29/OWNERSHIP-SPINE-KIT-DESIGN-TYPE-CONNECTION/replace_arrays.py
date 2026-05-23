from pathlib import Path
import re

path = Path("semio/graphql/target.schema.graphql")
text = path.read_text(encoding="utf-8")
lines = text.splitlines(True)
out = []
map_suffix = {
    "File": "FileConnection",
    "Folder": "FolderConnection",
    "Family": "FamilyConnection",
    "Type": "TypeConnection",
    "Design": "DesignConnection",
    "Draft": "DraftConnection",
    "Transaction": "TransactionConnection",
    "Checkpoint": "CheckpointConnection",
    "Benchmark": "BenchmarkConnection",
    "Quality": "QualityConnection",
    "Attribute": "AttributeListingConnection",
    "Tag": "TagConnection",
    "Connector": "ConnectorConnection",
    "Representation": "RepresentationConnection",
    "Author": "AuthorConnection",
    "Concept": "ConceptConnection",
    "Prop": "PropConnection",
    "Stat": "StatConnection",
    "Piece": "PieceConnection",
    "Connection": "ConnectionConnection",
    "Layer": "LayerConnection",
    "Group": "GroupConnection",
    "Blueprint": "BlueprintConnection",
    "Operation": "OperationConnection",
    "Change": "ChangeConnection",
    "Alternative": "AlternativeConnection",
    "Conflict": "ConflictConnection",
}
pat = re.compile(r"^(\s+)(\w+):\s*\[(\w+)!\]!\s*$")
for line in lines:
    raw = line.rstrip("\r\n")
    m = pat.match(raw)
    if not m:
        out.append(line)
        continue
    indent, fname, inner = m.groups()
    if fname == "edges":
        out.append(line)
        continue
    if inner == "ID":
        out.append(line)
        continue
    conn = map_suffix.get(inner)
    if not conn:
        raise SystemExit(f"unmapped array type: {inner} in line {line!r}")
    nl = "\r\n" if line.endswith("\r\n") else ("\n" if line.endswith("\n") else "")
    out.append(f"{indent}{fname}: {conn}!{nl}")

path.write_text("".join(out), encoding="utf-8")
print("ok")
