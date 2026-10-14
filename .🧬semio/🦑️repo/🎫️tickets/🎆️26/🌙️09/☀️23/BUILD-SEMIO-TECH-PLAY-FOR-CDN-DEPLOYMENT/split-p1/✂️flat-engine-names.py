#!/usr/bin/env python3
"""✂️ Retargets the artifact's uses of the former flat engine re-export surface (`crate::Model`, `use crate::{EnergyJob, …}`)
to the engine crate that owns each item (lane P1, slice S2). Artifact-owned names and aliases are left alone.
"""
import importlib.util, os, re
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
index = L.engine_index()
def fix(text):
    def group(m):
        items = [x.strip() for x in m.group(1).split(",") if x.strip()]
        keep, moved = [], {}
        for it in items:
            head = re.match(r"[A-Za-z_]\w*", it).group(0)
            if "::" not in it and head in index and head not in L.ALIASES: moved.setdefault(index[head], []).append(it)
            else: keep.append(it)
        if not moved: return m.group(0)
        out = (["use crate::{" + ", ".join(keep) + "};"] if keep else []) + [f"use crate::{a}::{{{', '.join(sorted(v))}}};" for a, v in sorted(moved.items())]
        return "\n".join(out)
    text = re.sub(r"use crate::\{([^{}]*)\};", group, text)
    s = L.strip(text); edits = []
    for m in re.finditer(r"\bcrate::([A-Za-z_]\w*)\b(?!\s*::\s*\{)", s):
        name = m.group(1)
        if name in index and name not in L.ALIASES and not s[m.end():m.end() + 2] == "::":
            edits.append((m.start(), m.end(), f"crate::{index[name]}::{name}"))
    for a, b, r in reversed(edits): text = text[:a] + r + text[b:]
    return text
n = 0
for r, dirs, fs in os.walk(L.ART):
    dirs[:] = [d for d in dirs if d not in ("target", "node_modules", "📦️packages")]
    for f in fs:
        if f.endswith(".rs"):
            p = os.path.join(r, f); t = L.rd(p); t2 = fix(t)
            if t2 != t: L.wr(p, t2); n += 1; print(os.path.relpath(p, L.ART))
root_p = f"{L.ART}/🦀️.rs"; root = L.rd(root_p)
if "as sim_wire;" not in root:
    root = L.replace_once(root, "extern crate semio_s_plugin_energy_engine_sim as sim;\n", "extern crate semio_s_plugin_energy_engine_sim as sim;\nextern crate semio_s_plugin_energy_engine_sim_restore as sim_restore;\nextern crate semio_s_plugin_energy_engine_sim_retirement as sim_retirement;\nextern crate semio_s_plugin_energy_engine_sim_wire as sim_wire;\n")
    L.wr(root_p, root)
print("files", n)
