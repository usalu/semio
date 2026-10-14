#!/usr/bin/env python3
"""🧭️ Dissolves a stdio format root's re-export barrels before a crate cut.

Detects the root's barrels (`pub use` re-exports, `pub mod schema|engine { pub use super::..::*; }` alias modules and
every standard-level `pub mod engine { pub use super::subsets::..::*; }` facade), writes the `🧭️canonical-paths.py`
config, runs it over the crate and every indexed consumer, then deletes the barrels; root code that used a removed
root re-export gets a private canonical `use`. External-crate re-exports become aliases of their real crate.

usage: 🧭️barrels.py <artifact dir (repo-rel)> plan|apply
"""
import json, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
REAL = {"pack": "semio_framework_pack", "protocol": "semio_framework_os_kernel", "store": "semio_framework_os_kernel", "dsl": "semio_framework_os_kernel"}
artifact, mode = sys.argv[1], sys.argv[2]
root = os.path.join(REPO, artifact, "🦀️.rs")
src = open(root, encoding="utf-8").read()
crate = re.search(r'(?m)^name\s*=\s*"([^"]+)"', open(os.path.join(REPO, artifact, "📦️packages", "🦀️rust", "Cargo.toml"), encoding="utf-8").read()).group(1).replace("-", "_")


def block(text, start):
    depth, j = 0, text.index("{", start)
    while True:
        if text[j] == "{": depth += 1
        elif text[j] == "}":
            depth -= 1
            if depth == 0: return j + 1
        j += 1


barrels, aliases, spans, root_names = {"": []}, {}, [], {}
alias_mods = {}
for m in re.finditer(r"(?m)^pub mod (\w+) \{\n((?:[ \t]*pub use super::[^\n]*\n|[ \t]*//[^\n]*\n|[ \t]*\n)+)\}\n", src):
    owners = []
    for u in re.finditer(r"pub use super::([\w:]+?)(?:::\*|::(\w+))?;", m.group(2)):
        owners.append(u.group(1))
    barrels[m.group(1)] = owners
    alias_mods[m.group(1)] = owners[0] if owners else None
    spans.append((m.start(), m.end()))
for m in re.finditer(r"(?m)^([ \t]+)pub mod engine \{\n((?:\1    pub use super::[^\n]*\n)+)\1\}\n", src):
    prefix = []
    for n in re.finditer(r"(?m)^([ \t]*)pub mod (\w+) \{", src[:m.start()]):
        indent = len(n.group(1))
        if indent < len(m.group(1)):
            prefix = [p for p in prefix if p[0] < indent] + [(indent, n.group(2))]
    path = [p[1] for p in prefix if p[0] < len(m.group(1))]
    base = "::".join(path)
    owners = [base + "::" + u.group(1) for u in re.finditer(r"pub use super::([\w:]+?)(?:::\*|::\w+)?;", m.group(2))]
    barrels[base + "::engine"] = owners
    lead = src.rfind("\n", 0, m.start()) + 1
    while True:
        prev = src.rfind("\n", 0, lead - 1) + 1
        if re.match(r"[ \t]*//", src[prev:lead]): lead = prev
        else: break
    spans.append((lead, m.end()))
for m in re.finditer(r"(?m)^pub use ([^;]+);\n", src):
    body = m.group(1).strip()
    head = body.split("::")[0]
    names = [n.strip().split(" as ")[-1] for n in re.sub(r".*\{|\}", "", body).split(",")] if "{" in body else [body.split("::")[-1]]
    if head in ("crate", "standards", "schema", "self"):
        path = re.sub(r"::\{.*\}$", "", body) if "{" in body else "::".join(body.split("::")[:-1])
        path = path.removeprefix("crate::").removeprefix("self::")
        if path.split("::")[0] in alias_mods and alias_mods[path.split("::")[0]]:
            path = alias_mods[path.split("::")[0]] + path[len(path.split("::")[0]):]
        barrels[""].append(path)
        for n in names: root_names[n] = path
    else:
        real = REAL.get(head, head)
        base_path = (re.sub(r"::\{.*\}$", "", body) if "{" in body else "::".join(body.split("::")[:-1])).replace(head, real, 1)
        for n in names: aliases.setdefault(base_path, []).append(n)
    spans.append((m.start(), m.end()))
barrels[""] = list(dict.fromkeys(barrels[""] + [alias_mods[k] for k in alias_mods if alias_mods[k]]))
cfg = {"root": root, "crate": crate, "barrels": barrels, "aliases": aliases}
cfg_path = os.path.join(HERE, "🗑️generated", crate + "-paths.json")
json.dump(cfg, open(cfg_path, "w"), ensure_ascii=False, indent=1)
print(json.dumps({k: v for k, v in barrels.items()}, ensure_ascii=False, indent=1)); print("aliases", aliases)
ns = {}
exec(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], ns)
files = set(ns["Crate"](root).files)
index = json.load(open(os.path.join(HERE, "🗑️generated", "stdio-consumers.json"), encoding="utf-8"))
ext = [os.path.join(REPO, f) for f in index.get(crate, []) if os.path.normpath(os.path.join(REPO, f)) not in files]
ext_path = os.path.join(HERE, "🗑️generated", crate + "-external.txt")
open(ext_path, "w").write("\n".join(ext))
args = [sys.executable, os.path.join(HERE, "🧭️canonical-paths.py"), cfg_path, "--external", ext_path] + (["--write"] if mode == "apply" else [])
print(subprocess.run(args, capture_output=True, text=True).stdout)
if mode == "apply":
    src = open(root, encoding="utf-8").read()
    keep = []
    for a, b in sorted(spans, reverse=True):
        src = src[:a] + src[b:]
    rest = re.sub(r"(?s)#\[path = \"\.\"\]\npub mod standards \{.*", "", src)
    for name, path in root_names.items():
        if re.search(r"(?<![\w:])" + re.escape(name) + r"(?![\w])", rest):
            keep.append(f"use crate::{path}::{name};")
    for prefix, owners in alias_mods.items():
        src = re.sub(r"(?<![\w:])" + prefix + r"::", lambda m: (owners or prefix) + "::", src) if owners else src
    if keep:
        at = src.index("\n", src.index("extern crate")) + 1 if "extern crate" in src else 0
        last = list(re.finditer(r"(?m)^extern crate [^\n]*\n", src))
        at = last[-1].end() if last else at
        src = src[:at] + "\n" + "\n".join(keep) + "\n" + src[at:]
    open(root, "w", encoding="utf-8").write(re.sub(r"\n{3,}", "\n\n", src))
    print("removed", len(spans), "barrels; root uses", keep)
