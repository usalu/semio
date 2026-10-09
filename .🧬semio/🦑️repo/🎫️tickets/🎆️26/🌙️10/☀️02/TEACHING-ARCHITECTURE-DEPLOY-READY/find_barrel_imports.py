import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
ui = None
for dirpath, dirs, files in os.walk(os.path.join(base, framework)):
    dirs[:] = [name for name in dirs if name not in {"node_modules", "dist", "target"}]
    if os.path.basename(dirpath).endswith("ui") and "modules" in dirpath:
        ui = dirpath
        break
print("ui", ui.encode("unicode_escape").decode())
needles = ("react/\\U0001f7e6\\ufe0f.tsx", "react-dom/server", "from \"three\"")
# find imports of the react barrel: a path ending in react/<blue>.tsx
hits = []
for dirpath, dirs, files in os.walk(ui):
    dirs[:] = [name for name in dirs if name not in {"node_modules", "dist", "target"}]
    for name in files:
        if not name.endswith((".ts", ".tsx")):
            continue
        path = os.path.join(dirpath, name)
        if "tests" in path.split(os.sep) or "stories" in path.split(os.sep):
            continue
        text = open(path, encoding="utf-8", errors="ignore").read().splitlines()
        for index, line in enumerate(text, 1):
            if "import" not in line and "from" not in line:
                continue
            if "react-dom/server" in line or 'from "three"' in line or (line.strip().endswith('.tsx";') and "/react/" in line.replace("\\", "/")):
                if "react-dom/server" in line or 'from "three"' in line or "\u269b\ufe0freact" in line:
                    hits.append(f"{os.path.relpath(path, base).encode('unicode_escape').decode()}:{index}:{line.strip()[:220]}")
print("hits", len(hits))
for hit in hits:
    print(hit)
