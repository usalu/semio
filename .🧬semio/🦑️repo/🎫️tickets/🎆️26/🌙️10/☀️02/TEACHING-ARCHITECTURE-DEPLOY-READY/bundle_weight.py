import gzip
import pathlib

root = pathlib.Path(__file__).resolve().parents[7]
dist = next(path for path in (root / "🎓️teaching").rglob("dist") if path.parent.name.endswith("typescript") and "architecture" in str(path))
print("dist", dist)
files = [path for path in dist.rglob("*") if path.is_file() and "pages" not in path.parts]
total = 0
rows = []
for path in files:
    size = path.stat().st_size
    total += size
    rel = path.relative_to(dist).as_posix()
    gz = len(gzip.compress(path.read_bytes(), compresslevel=9)) if path.suffix in {".js", ".css"} else 0
    rows.append((size, gz, rel))
print("total", total, "files", len(files))
for size, gz, rel in sorted(rows, reverse=True)[:15]:
    print(f"{size:10} {gz:8} {rel}")
needles = ["pdf.worker", "pdfjs", "THREE", "xyflow", "react-dom", "WebGLRenderer"]
for path in files:
    if path.suffix not in {".js", ".mjs"}:
        continue
    text = path.read_text(encoding="utf-8", errors="ignore")
    hits = [needle for needle in needles if needle in text]
    if hits:
        print("hits", path.name, hits)
