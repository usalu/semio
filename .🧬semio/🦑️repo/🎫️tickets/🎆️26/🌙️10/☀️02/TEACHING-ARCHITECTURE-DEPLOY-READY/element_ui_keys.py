import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
modules = os.path.join(base, framework, next(name for name in os.listdir(os.path.join(base, framework)) if name.endswith("modules")))
ui = os.path.join(modules, next(name for name in os.listdir(modules) if name.endswith("ui")))
elements = os.path.join(ui, next(name for name in os.listdir(ui) if name.endswith("elements")))
wanted = ("UiDriver", "Navbar", "WindowChrome", "Icons", "LayeredOverview", "OverviewCard", "I18n")
for name in os.listdir(elements):
    if not any(name.endswith(suffix) for suffix in wanted):
        continue
    folder = os.path.join(elements, name)
    if not os.path.isdir(folder):
        continue
    for file in os.listdir(folder):
        if not file.endswith((".ts", ".tsx")):
            continue
        path = os.path.join(folder, file)
        lines = open(path, encoding="utf-8", errors="ignore").read().splitlines()
        hits = [(index, line) for index, line in enumerate(lines, 1) if "ui." in line or "useLabel" in line]
        if not hits:
            continue
        print(name.encode("unicode_escape").decode(), file.encode("unicode_escape").decode(), len(hits))
        for index, line in hits:
            if "ui." in line:
                print(f"  {index}:{line.strip()[:200]}")
