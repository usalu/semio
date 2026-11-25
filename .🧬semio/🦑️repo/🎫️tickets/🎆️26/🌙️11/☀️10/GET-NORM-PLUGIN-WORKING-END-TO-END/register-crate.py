"""🧾️ Registers a new crate in its family workspace: member, workspace dependency and the package router crate list.
usage: register-crate.py <family-root> <crate-name> <manifest-dir-relative-to-family>"""
import re, sys
root, name, rel = sys.argv[1:4]
cargo = root + "/Cargo.toml"
text = open(cargo, encoding="utf8").read()
member = f'    "{rel}",\n'
if member not in text:
    i = text.index("members = [\n") + len("members = [\n")
    text = text[:i] + member + text[i:]
dep = f'[workspace.dependencies.{name}]\n'
if dep not in text:
    text = text.rstrip("\n") + f'\n\n{dep}path = "{rel}"\n'
open(cargo, "w", encoding="utf8").write(text)
script = root + "/📦️packages/🦀️rust/📜️script.ts"
body = open(script, encoding="utf8").read()
if f'"{name}"' not in body:
    body = body.replace('{packages:["', '{packages:["' + name + '","', 1)
    open(script, "w", encoding="utf8").write(body)
print("registered", name)
