import pathlib, tomllib
path = pathlib.Path("D:/semio/✏️s/🔌️plugins/🧩️puzzle/Cargo.toml")
data = tomllib.loads(path.read_text(encoding="utf-8"))
assert data["workspace"]["dependencies"]["semio-framework-trace"]["path"] == "../../../🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust"
print("trace workspace dependency is unique")
