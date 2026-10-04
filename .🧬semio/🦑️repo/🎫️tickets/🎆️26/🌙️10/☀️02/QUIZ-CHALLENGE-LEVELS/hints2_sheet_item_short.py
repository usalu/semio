import pathlib, sys
root = pathlib.Path(r"C:/git/semio/🧰️framework/🛍️products/❓️quiz/🔨️modules")
old = "SheetItem { id: (*id).to_string(), label: text(id), icon: None }"
new = "SheetItem { id: (*id).to_string(), label: text(id), short: None, icon: None }"
for rel in ["📏️scoring/🧪️tests/🔬️unit/🦀️.rs", "✅️validation/🧪️tests/🔬️unit/🦀️.rs", "⛰️challenge/🧪️tests/🔬️unit/🦀️.rs"]:
    path = root / rel
    data = path.read_bytes().decode("utf-8")
    count = data.count(old)
    path.write_bytes(data.replace(old, new).encode("utf-8"))
    print(rel, count)
