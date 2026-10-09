import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
products = os.path.join(base, framework, next(name for name in os.listdir(os.path.join(base, framework)) if name.endswith("products")))
quiz = os.path.join(products, next(name for name in os.listdir(products) if "quiz" in name))
target = None
for dirpath, dirs, files in os.walk(quiz):
    dirs[:] = [name for name in dirs if name not in {"node_modules", "dist"}]
    if os.path.basename(dirpath).endswith("i18n") and "modules" in dirpath and "react" in dirpath:
        for name in files:
            if name.endswith(".ts"):
                target = os.path.join(dirpath, name)
print(target.encode("unicode_escape").decode() if target else "missing")
if target:
    lines = open(target, encoding="utf-8").read().splitlines()
    for index, line in enumerate(lines[:80], 1):
        print(f"{index}:{line[:200]}")
