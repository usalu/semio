#!/usr/bin/env python3
"""✂️ Reduces the artifact root to the aggregator (lane P1, slice S2f): kind spec, definition, declaration and the
language roster only, aliasing exactly the crates it names, with a manifest derived from its closure.
"""
import importlib.util, os, re, json
spec = importlib.util.spec_from_file_location("artifact_lib", os.path.join(os.path.dirname(os.path.abspath(__file__)), "✂️artifact_lib.py"))
L = importlib.util.module_from_spec(spec); spec.loader.exec_module(L)
ROOT = f"{L.ART}/🦀️.rs"; t = L.rd(ROOT)
t = re.sub(r"(?m)^(#\[cfg\(test\)\]\n)?extern crate [a-z_0-9]+ as [a-z_0-9]+;\n", "", t)
t = t.replace("//#region ⚡️SimulationEngine\n//#endregion ⚡️SimulationEngine\n", "")
t = t.replace("crate::io::text::mutations::", "crate::mutations::text::").replace("crate::io::binary::mutations::", "crate::mutations::binary::")
m = re.search(r"(?m)^use crate::snapshot::\{([^}]*)\};\n", t)
names = [x.strip() for x in m.group(1).split(",") if x.strip()]
body = L.strip(t.replace(m.group(0), ""))
used = [n for n in names if re.search(r"(?<![\w:])" + re.escape(n) + r"\b", body)]
t = t.replace(m.group(0), ("use crate::snapshot::{" + ", ".join(used) + "};\n" if used else "") + "use crate::mutations::EnergyModelMutation;\n")
t = re.sub(r"\n\n\n+", "\n\n", t)
t = t.replace("#![allow(clippy::too_many_arguments)]\n", "#![allow(clippy::too_many_arguments)]\n\n@@ALIASES@@\n", 1)
L.wr(ROOT, t)
json.dump([{"owner": L.ART, "package": "semio-s-artifact-energy-model", "alias": None, "description": "energy model artifact: kind spec, definition and declaration over the energy model crates"}],
          open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "🗑️generated", "aggregator.json"), "w"))
print(t[:1200])
