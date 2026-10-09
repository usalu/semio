import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
modules = os.path.join(base, framework, next(name for name in os.listdir(os.path.join(base, framework)) if name.endswith("modules")))
ui = os.path.join(modules, next(name for name in os.listdir(modules) if name.endswith("ui")))
targets = os.path.join(ui, next(name for name in os.listdir(ui) if name.endswith("targets")))
react = os.path.join(targets, next(name for name in os.listdir(targets) if name.endswith("react")))
path = os.path.join(react, next(name for name in os.listdir(react) if name.endswith("i18n")), "\U0001f7e6\ufe0f.ts")
lines = open(path, encoding="utf-8").read().splitlines()
# measure sibling keys of `nav:` inside `ui: {` for the German bundle, which starts at the first `ui: {` after line 92
start = next(index for index, line in enumerate(lines) if line.strip() == "ui: {")
depth = 0
begun = False
current = None
sizes = []
for index in range(start, len(lines)):
    line = lines[index]
    if not begun:
        begun = True
        depth = 1
        continue
    if depth == 1 and line.strip().endswith(": {") and not line.strip().startswith("//"):
        current = (line.strip().split(":")[0], index, 0)
    if current and depth >= 1:
        current = (current[0], current[1], current[2] + len(line) + 1)
    delta = line.count("{") - line.count("}")
    depth += delta
    if current and depth == 1:
        sizes.append(current)
        current = None
    if depth == 0:
        break
sizes.sort(key=lambda item: item[2], reverse=True)
print("groups", len(sizes), "chars", sum(item[2] for item in sizes))
for name, line, size in sizes[:25]:
    print(f"{size:7} L{line + 1} {name}")
