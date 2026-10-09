import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
root = os.path.join(base, framework)
needles = ("ui-react/i18n", "/i18n/", "framework-server", "@semio-tech/framework-server")
hits = []
for dirpath, dirs, files in os.walk(root):
    dirs[:] = [name for name in dirs if name not in {"node_modules", "dist", "target"}]
    rel = os.path.relpath(dirpath, root)
    if "print" in rel or "os" in rel.split(os.sep):
        dirs[:] = [name for name in dirs if name not in {"print"}]
    for name in files:
        if not name.endswith((".ts", ".tsx")):
            continue
        path = os.path.join(dirpath, name)
        if "tests" in path.split(os.sep) or "stories" in path.split(os.sep):
            continue
        if "quiz" not in path and "ui" not in path and "teaching" not in path:
            continue
        for index, line in enumerate(open(path, encoding="utf-8", errors="ignore"), 1):
            if any(needle in line for needle in needles) and ("import" in line or "from" in line):
                hits.append(f"{os.path.relpath(path, base).encode('unicode_escape').decode()}:{index}:{line.strip()[:200]}")
print("hits", len(hits))
for hit in hits:
    print(hit)
