#!/usr/bin/env python3
import os, shutil

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
STDIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"
SEMIO = f"{STDIO}/🧿️semio"
ANY_EXAMPLES = f"{SEMIO}/🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples"

def subset_dir(sub): return f"{SEMIO}/🏅️standards/🔖️v1/🪆️subsets/✳️{sub}"

OLD_PREFIX = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/"
NEW_PREFIX = "asset://📚️examples/"

cases = {
    "mutate-semio-animation": ("animation", "🚶️walk"),
    "mutate-semio-audio": ("audio", "🎵️tone"),
    "mutate-semio-brep": ("brep", "🧊️solid"),
    "mutate-semio-cad": ("cad", "📐️drawing"),
    "mutate-semio-document": ("document", "📄️memo"),
    "mutate-semio-flow": ("flow", "🌊️pipeline"),
    "mutate-semio-model": ("model", "🏢️building"),
    "mutate-semio-presentation": ("presentation", "📽️deck"),
    "mutate-semio-video": ("video", "🎥️clip"),
}

def replace_in_file(path, old, new):
    if not os.path.exists(path):
        return 0
    text = open(path, encoding="utf8").read()
    n = text.count(old)
    if n:
        text = text.replace(old, new)
        open(path, "w", encoding="utf8").write(text)
    return n

report = []
for case, (sub, exname) in cases.items():
    src_ex = f"{ANY_EXAMPLES}/{exname}"
    sd = subset_dir(sub)
    dst_ex = f"{sd}/📚️examples/{exname}"
    if not os.path.exists(dst_ex):
        shutil.copytree(src_ex, dst_ex)
    case_src = f"{SEMIO}/🧪️tests/{case}"
    total = 0
    for fname in ["🥒️.feature", "🦀️.rs", "🐍️.py", "🟦️.ts"]:
        p = f"{case_src}/{fname}"
        total += replace_in_file(p, OLD_PREFIX, NEW_PREFIX)
    dst_parent = f"{sd}/🧪️tests"
    os.makedirs(dst_parent, exist_ok=True)
    dst = f"{dst_parent}/{case}"
    shutil.move(case_src, dst)
    report.append((case, sub, exname, total, dst))

for r in report:
    print(r)
