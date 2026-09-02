#!/usr/bin/env python3
import os, shutil

ROOT = "/Users/ueli/Documents/semio"
os.chdir(ROOT)
STDIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"

OLD_URI = "asset://🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🧪️dancing/🖼️.gif"
src = f"{STDIO}/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/📚️examples/💃️dancing/🖼️assets/🧪️dancing/🖼️.gif"

cases = {
    "mutate-gif-89a": "base",
    "mutate-gif-89a-application": "application",
    "mutate-gif-89a-comment": "comment",
    "mutate-gif-89a-graphic-control": "graphic-control",
}

def subset_dir(sub): return f"{STDIO}/🎞️gif/🏅️standards/🔖️89a/🪆️subsets/✳️{sub}"

def replace_in_file(path, old, new, required=True):
    if not os.path.exists(path):
        return 0
    text = open(path, encoding="utf8").read()
    n = text.count(old)
    if required and n == 0:
        raise AssertionError(f"missing in {path}")
    if n:
        text = text.replace(old, new)
        open(path, "w", encoding="utf8").write(text)
    return n

report = []
for case, sub in cases.items():
    sub_dir = subset_dir(sub)
    dst_dir = f"{sub_dir}/📚️examples/💃️dancing/🖼️assets/🧪️dancing"
    os.makedirs(dst_dir, exist_ok=True)
    shutil.copyfile(src, f"{dst_dir}/🖼️.gif")
    new_uri = "asset://📚️examples/💃️dancing/🖼️assets/🧪️dancing/🖼️.gif"
    case_src = f"{STDIO}/🎞️gif/🧪️tests/{case}"
    feature = f"{case_src}/🥒️.feature"
    rustf = f"{case_src}/🦀️.rs"
    n1 = replace_in_file(feature, OLD_URI, new_uri)
    n2 = replace_in_file(rustf, OLD_URI, new_uri, required=False)
    os.makedirs(f"{sub_dir}/🧪️tests", exist_ok=True)
    dst = f"{sub_dir}/🧪️tests/{case}"
    shutil.move(case_src, dst)
    report.append((case, sub, dst, n1, n2))

for r in report:
    print(r)
