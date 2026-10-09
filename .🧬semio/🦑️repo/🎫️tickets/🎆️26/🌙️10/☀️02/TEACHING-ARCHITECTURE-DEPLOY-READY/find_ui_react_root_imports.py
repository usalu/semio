import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
teaching = next(name for name in os.listdir(base) if name.endswith("teaching"))
roots = [os.path.join(base, framework), os.path.join(base, teaching)]
needle = "@semio-tech/ui-react"
hits = []
for root in roots:
    for dirpath, dirs, files in os.walk(root):
        dirs[:] = [name for name in dirs if name not in {"node_modules", "dist", "target", ".nx"}]
        parts = set(dirpath.split(os.sep))
        if "tests" in parts and "quiz" not in dirpath:
            continue
        for name in files:
            if not name.endswith((".ts", ".tsx")):
                continue
            path = os.path.join(dirpath, name)
            if "tests" in path.split(os.sep) or "stories" in path.split(os.sep):
                continue
            for index, line in enumerate(open(path, encoding="utf-8", errors="ignore"), 1):
                if needle in line and "ui-react/" not in line:
                    hits.append(f"{os.path.relpath(path, base).encode('unicode_escape').decode()}:{index}:{line.strip()[:240]}")
print("hits", len(hits))
for hit in hits:
    print(hit)
