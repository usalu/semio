import pathlib
root = pathlib.Path(__file__).resolve().parents[7]
teaching = next(path for path in root.iterdir() if path.name.endswith("teaching"))
manifest = next(teaching.glob("Cargo.toml"))
print(manifest.read_text(encoding="utf-8")[:1500])
