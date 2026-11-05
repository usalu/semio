import pathlib, tomllib
base = pathlib.Path("✏️s/🔌️plugins/🀄️wfc")
files = list(base.rglob("Cargo.toml"))
for file in files:
    tomllib.loads(file.read_text(encoding="utf-8"))
print(f"[DEBUG] TOML parsed {len(files)} WFC Cargo manifests")
