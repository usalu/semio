from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
plugins = next(p for p in (root / "✏️s").iterdir() if p.name.endswith("plugins"))
procedural = next(p for p in plugins.iterdir() if "procedural" in p.name)
artifacts = next(p for p in procedural.iterdir() if "artifacts" in p.name)
assembly = next(p for p in artifacts.iterdir() if "assembly" in p.name)
print("assembly", assembly)
any_dir = None
for p in assembly.rglob("*"):
    if p.is_dir() and p.name.endswith("editor") and "any" in str(p):
        any_dir = p.parent
        break
print("any_dir", any_dir)
if any_dir is not None:
    print("children", [c.name for c in any_dir.iterdir()])
    schema = next((c for c in any_dir.iterdir() if "schema" in c.name), None)
    print("schema", schema)
    if schema:
        print("schema children", [c.name for c in schema.iterdir()])
        print("schema rs", any((c.name.endswith(".rs") for c in schema.iterdir() if c.is_file())))
