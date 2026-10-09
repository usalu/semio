import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
modules = os.path.join(base, framework, next(name for name in os.listdir(os.path.join(base, framework)) if name.endswith("modules")))
assets = os.path.join(modules, next(name for name in os.listdir(modules) if name.endswith("assets")))
icons = os.path.join(assets, next(name for name in os.listdir(assets) if name.endswith("icons")))
print("icons", icons.encode("unicode_escape").decode())
for dirpath, dirs, files in os.walk(icons):
    dirs[:] = [name for name in dirs if name != "node_modules"]
    for name in files:
        if name.endswith((".ts", ".tsx")):
            path = os.path.join(dirpath, name)
            print(os.path.getsize(path), os.path.relpath(path, icons).encode("unicode_escape").decode())
