from pathlib import Path

root = Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript")
print("root exists", root.exists())
for p in root.iterdir():
    print(repr(p.name))
    if p.name.endswith("registry"):
        for f in p.iterdir():
            print(" ", repr(f.name), f.stat().st_size)
            if "launch" in f.name:
                t = f.read_text(encoding="utf-8")
                print("LAUNCH FILE", f)
                for i, line in enumerate(t.splitlines(), 1):
                    if any(k in line for k in ("policy", "Policy", "generateLaunch", "name:", "group")):
                        if i < 250 or "policy" in line.lower() or "Taxonomy" in line or "Gate" in line:
                            print(f"{i}:{line[:180]}")
