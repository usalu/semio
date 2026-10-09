import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
root = os.path.join(base, framework)
needles = ("react-dom/server", 'from "three"', "from 'three'", "metabolism", "pdfjs-dist", "@xyflow/react")
skip_parts = {"node_modules", "dist", "target", ".nx", "print", "presentation"}
hits = []
for dirpath, dirs, files in os.walk(root):
    dirs[:] = [name for name in dirs if name not in skip_parts]
    rel = os.path.relpath(dirpath, root)
    parts = rel.split(os.sep)
    if "products" in parts and "quiz" not in rel:
        if parts[-1] != "products":
            dirs.clear()
        continue
    for name in files:
        if not name.endswith((".ts", ".tsx")):
            continue
        path = os.path.join(dirpath, name)
        text = open(path, encoding="utf-8", errors="ignore").read()
        found = [needle for needle in needles if needle in text]
        if found:
            hits.append((os.path.relpath(path, base), found))
print("hits", len(hits))
for path, found in hits:
    print(path.encode("unicode_escape").decode(), found)
