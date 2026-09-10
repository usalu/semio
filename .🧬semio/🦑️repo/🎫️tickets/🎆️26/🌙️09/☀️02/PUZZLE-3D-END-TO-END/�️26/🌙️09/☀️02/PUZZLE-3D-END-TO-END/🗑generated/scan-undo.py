from pathlib import Path

root = Path("/Users/ueli/Documents/semio")
p = next(root.joinpath("🧰️framework").rglob("plugin-runtime-plugin-builder-contract/🦀️.rs"))
lines = p.read_text().splitlines()
print("FILE", p)
print("TOTAL", len(lines))
for i, line in enumerate(lines, 1):
    if any(key in line for key in ("reserved_undo", "fn contract_app", "window-without-undo", "missing-instance", "admit_addressed", "history route")):
        print(f"{i}:{line}")
