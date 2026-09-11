from pathlib import Path

base = Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/�Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly")
print("assembly root", base.exists())
any_dir = None
for p in base.rglob("✏️editor"):
    if p.is_dir() and "✳️any" in str(p):
        any_dir = p.parent
        break
print("any_dir", any_dir)
if any_dir is not None:
    print("editor rs", (any_dir / "✏️editor" / "🦀️.rs").exists())
    print("io", (any_dir / "🚪️io").exists())
    schema = any_dir / "🧬️schema"
    print("schema", schema.exists())
    print("schema children", [p.name for p in schema.iterdir()])
    print("schema rs", (schema / "🦀️.rs").exists())
