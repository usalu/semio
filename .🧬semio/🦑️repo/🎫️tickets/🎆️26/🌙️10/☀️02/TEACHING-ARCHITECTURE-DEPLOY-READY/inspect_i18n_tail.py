import os

base = r"C:\git\semio"
framework = next(name for name in os.listdir(base) if name.endswith("framework"))
modules = os.path.join(base, framework, next(name for name in os.listdir(os.path.join(base, framework)) if name.endswith("modules")))
ui = os.path.join(modules, next(name for name in os.listdir(modules) if name.endswith("ui")))
targets = os.path.join(ui, next(name for name in os.listdir(ui) if name.endswith("targets")))
react = os.path.join(targets, next(name for name in os.listdir(targets) if name.endswith("react")))
path = os.path.join(react, next(name for name in os.listdir(react) if name.endswith("i18n")), "\U0001f7e6\ufe0f.ts")
lines = open(path, encoding="utf-8").read().splitlines()
for index in range(2148, 2210):
    print(f"{index + 1}:{lines[index][:200]}")
