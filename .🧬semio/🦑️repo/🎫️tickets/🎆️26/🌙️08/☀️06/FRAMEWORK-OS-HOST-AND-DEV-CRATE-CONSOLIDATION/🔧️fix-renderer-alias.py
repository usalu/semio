
from pathlib import Path
import re

root = Path("/Users/ueli/Documents/semio")
fw = next(p for p in root.iterdir() if p.is_dir() and "framework" in p.name)
rend = next(p for p in (fw / "🛍️products" / "💻️os" / "🔨️modules").iterdir() if "renderer" in p.name)
cands = []
for p in rend.rglob("*"):
    if not p.is_file():
        continue
    s = str(p)
    if "node_modules" in s or "target" in s:
        continue
    if "packages" in s and "react" in s and p.suffix in {".tsx", ".ts"} and ("index" in p.name or "glue" in p.name):
        cands.append(p)
print("cands:")
for c in cands:
    print(" ", c.relative_to(root))
preferred = [c for c in cands if "⚛️react" in str(c) and "packages" in str(c)]
# prefer targets/react package index
preferred2 = [c for c in preferred if "🎯️targets" in str(c)]
entry = (preferred2 or preferred or cands)[0]
print("chosen", entry.relative_to(root), entry.exists())

dev = next(p for p in (fw / "🛍️products" / "💻️os" / "🔨️modules").iterdir() if "dev" in p.name)
vite = None
for p in (dev / "📦️packages").rglob("*"):
    if p.is_file() and "vite.config" in p.name:
        vite = p
        break
print("vite", vite)
text = vite.read_text()
for i, l in enumerate(text.splitlines()):
    if "framework-renderer-react" in l:
        print(f"before {i+1}: {l[:220]}")

rel = "./" + entry.relative_to(root).as_posix()
pat = re.compile(
    r'(\{ find: "@semio-tech/framework-renderer-react", replacement: path\.resolve\(repoRoot, ")[^"]+("\) \})'
)
m = pat.search(text)
if not m:
    raise SystemExit("alias not found")
text2, n = pat.subn(r"\1" + rel.replace("\\", "\\\\") + r"\2", text, count=1)
# safer with lambda
text2, n = pat.subn(lambda mm: mm.group(1) + rel + mm.group(2), text, count=1)
print("replaced", n, "->", rel)
vite.write_text(text2)
for i, l in enumerate(vite.read_text().splitlines()):
    if "framework-renderer-react" in l:
        print(f"after {i+1}: {l[:220]}")
