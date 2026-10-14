#!/usr/bin/env python3
"""✂️ Splits the fem3d mounted editor session (4 018 active lines, over the unit ceiling) along its own regions: the
visual job (regions 🔖️Contract and 📦️PageVisualJob) moves to `🧵️session/🎞️visual-job`, mounted as
`editor::fem3d::session::visual_job`; the mounted session keeps 💼️MountedSession and its laws. Each part imports only
the header names it uses; private visual-job items, methods and fields the session names become `pub`; consumers of
visual-job items are relocated to the new module.

usage: ✂️fem3d-session-split.py
"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = "/Users/ueli/Documents/semio"
A = REPO + "/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
SR = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session"
S = A + "/" + SR
src = open(S + "/🦀️.rs", encoding="utf-8").read()
a = src.index("//#region 🔖️Contract"); b = src.index("//#endregion 📦️PageVisualJob\n") + len("//#endregion 📦️PageVisualJob\n")
c = src.index("//#region 💼️MountedSession")
doc_end = src.index("\n\n") + 2
doc = src[:doc_end]; header = src[doc_end:a]; job = src[a:b]; rest = src[b:]
tests = open(S + "/🧪️tests/🔬️unit/🦀️.rs", encoding="utf-8").read()
code = lambda t: re.sub(r"//[^\n]*", "", t)
uses = []
for m in re.finditer(r"(?m)^(#\[cfg\(test\)\]\n)?use ([^;]+);", header):
    attr, body = m.group(1) or "", m.group(2)
    mm = re.match(r"([\w:]+)::\{(.*)\}$", body.strip(), re.S)
    names = [x.strip() for x in mm.group(2).split(",") if x.strip()] if mm else [body.strip().split("::")[-1]]
    base = mm.group(1) if mm else "::".join(body.strip().split("::")[:-1])
    uses.append((attr, base, names))
def imports(text):
    out = []
    t = code(text)
    for attr, base, names in uses:
        keep = [n for n in names if re.search(r"(?<![\w:])" + n + r"\b", t)]
        if keep: out.append(f"{attr}use {base}::{keep[0] if len(keep) == 1 else '{' + ', '.join(keep) + '}'};")
    return "\n".join(out)
items = re.findall(r"(?m)^(pub(?:\([^)]*\))?\s+)?(?:const|static|fn|struct|enum|type|trait)\s+(\w+)", job)
job_names = [n for _, n in items]
sess_code = code(rest + tests)
need_pub = [n for v, n in items if (not v or v.startswith("pub(")) and re.search(r"(?<![\w:.])" + n + r"\b", sess_code)]
for n in need_pub:
    job = re.sub(r"(?m)^(?:pub\([^)]*\)\s+)?((?:const|static|fn|struct|enum|type|trait)\s+" + n + r"\b)", r"pub \1", job)
meths = set(re.findall(r"(?m)^    (?:pub\([^)]*\)\s+)?fn (\w+)", job)) - set(re.findall(r"(?m)^    pub fn (\w+)", job))
for mname in [m for m in meths if re.search(r"(?:\.|::)\s*" + m + r"\s*(?:::<[^>]*>)?\(", sess_code)]:
    job = re.sub(r"(?m)^    (?:pub\([^)]*\)\s+)?fn " + mname + r"\b", "    pub fn " + mname, job)
fields = set(re.findall(r"(?m)^    (?:pub\([^)]*\)\s+)?(\w+):\s", job))
for f in [f for f in fields if re.search(r"\.\s*" + f + r"\b(?!\s*\()", sess_code) or re.search(r"\b" + f + r"\s*:", code(rest))]:
    job = re.sub(r"(?m)^    (?:pub\([^)]*\)\s+)?" + f + r":(\s)", "    pub " + f + r":\1", job)
pub_job = [n for n in job_names if re.search(r"(?m)^pub\s+(?:const|static|fn|struct|enum|type|trait)\s+" + n + r"\b", job)]
used_in_session = [n for n in pub_job if re.search(r"(?<![\w:.])" + n + r"\b", sess_code)]
job_file = "//! 🎞️ The FEM3D page visual job: its admission contract, staged mesh/solve pipeline and paged world-snapshot\n//! publication — what one mounted session runs per document revision.\n\n" + imports(job) + "\n\n" + job
os.makedirs(S + "/🎞️visual-job", exist_ok=True)
open(S + "/🎞️visual-job/🦀️.rs", "w", encoding="utf-8").write(job_file)
vj = "crate::editor::fem3d::session::visual_job"
sess = doc + imports(rest + tests) + "\n" + (f"use {vj}::{{{', '.join(used_in_session)}}};\n" if used_in_session else "") + "\n" + rest
open(S + "/🦀️.rs", "w", encoding="utf-8").write(sess)
root = open(A + "/🦀️.rs", encoding="utf-8").read()
old = f'        #[path = "{SR}/🦀️.rs"]\n        pub mod session;\n'
assert old in root
root = root.replace(old, f'        #[path = "."]\n        pub mod session {{\n            #[path = "{SR}/🦀️.rs"]\n            mod component;\n            pub use component::*;\n            #[path = "{SR}/🎞️visual-job/🦀️.rs"]\n            pub mod visual_job;\n        }}\n', 1)
open(A + "/🦀️.rs", "w", encoding="utf-8").write(root)
json.dump({n: vj for n in pub_job}, open(HERE + "/🚚️fem3d-visual-job-map.json", "w"), ensure_ascii=False, indent=1)
files = [os.path.join(r, f) for base in (A, REPO + "/🌎️hub/🧩️compositions/🏗️fem") for r, _, fs in os.walk(base) for f in fs if f.endswith(".rs") and "🎞️visual-job" not in r]
subprocess.run([sys.executable, HERE + "/🚚️relocate-root.py", HERE + "/🚚️fem3d-visual-job-map.json", "crate::editor::fem3d::session"] + files, check=True, stdout=subprocess.DEVNULL)
subprocess.run([sys.executable, HERE + "/🚚️relocate-root.py", HERE + "/🚚️fem3d-visual-job-map.json", "semio_s_artifact_fem_3d::editor::fem3d::session"] + files, check=True, stdout=subprocess.DEVNULL)
print("pub widened", len(need_pub), "session imports", len(used_in_session), "job lines", job_file.count("\n"), "session lines", sess.count("\n"))
