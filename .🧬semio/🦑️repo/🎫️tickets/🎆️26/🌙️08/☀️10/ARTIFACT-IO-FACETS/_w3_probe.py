
from pathlib import Path
d = next(x for x in Path("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript").iterdir() if x.name.endswith("registry"))
p = d / "📜️script.ts"
lines = p.read_text().splitlines()
for i, l in enumerate(lines):
    if "TAXONOMY_ARTIFACT" in l or "FACET_DIR" in l or "SNAPSHOT" in l and "const" in l:
        print(f"{i+1}:{l[:160]}")
