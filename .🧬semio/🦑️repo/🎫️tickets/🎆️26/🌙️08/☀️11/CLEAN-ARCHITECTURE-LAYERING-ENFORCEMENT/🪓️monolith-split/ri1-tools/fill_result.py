#!/usr/bin/env python3
# 📝️ Fills the computed tables of RESULT-RENDERER-INTEGRATION-1.md (census, files); the oracle tables are filled by hand from the logs.
import sys, os, collections, re
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from common import *
P = SPLIT + "/RESULT-RENDERER-INTEGRATION-1.md"
t = read(P)
rows = collections.defaultdict(dict)
for l in open(RS + "/census-final.tsv", encoding="utf8"):
    p = l.rstrip("\n").split("\t")
    name = p[0].replace("semio-framework-os-renderer-wgpu-", "").replace("semio-framework-os-renderer-wgpu", "<top>")
    rows[name][(p[1], p[2])] = p[3:]
def f(d, k):
    v = d.get(k)
    if not v: return "-"
    if v[0] == "ERROR": return "ERROR (missing source input)"
    return f"{v[0]} / {v[1]:>} / {v[2]}".replace(":>", "")
lines = ["| crate | lib native files / bytes / lines | test native | lib wasm32 | verdict |", "|---|---|---|---|---|"]
for n, d in rows.items():
    verdict = (d.get(("native", "lib")) or ["", "", "", "", ""])[4]
    lines.append(f"| `{n}` | {f(d, ('native','lib'))} | {f(d, ('native','test'))} | {f(d, ('wasm','lib'))} | {verdict} |")
lines.append("")
lines.append("Before (end of WP1): the top crate lib closure was 344 files / 3,925,929 B / 64,299 lines (native); it is now 336 / 3,299,414 / 53,368 and still over the gate (`OVER-GATE`: the eight oversized dependencies and the monolith remainder). The `native-platform` test closure could not be counted at WP1 (the stale temporal fixture path) and can now (10 / 92,430 / 1,504). Raw rows: `scratchpad/ri1/census-final.tsv`, command `python3 ri1-tools/census.py <crates> --top`.")
t = t.replace("CENSUS_TABLE", "\n".join(lines))
files = [l.strip() for l in open(SPLIT + "/ri1-tools/touched.txt", encoding="utf8") if l.strip() and not l.startswith("#")]
def short(x): return x.replace("🎯️targets/🧊️wgpu/", "T/").replace("🧱️elements/", "EL/")
groups = collections.OrderedDict()
for x in files:
    key = "crate folders `T/<crate>/…` (roots, manifests)" if x.startswith("🎯️targets/🧊️wgpu/") and x.count("/") <= 6 and ("Cargo.toml" in x or x.endswith("🦀️.rs")) and "🧊️renderer" not in x else None
    groups.setdefault(key or "other", []).append(short(x))
txt = "; ".join(f"`{short(x)}`" for x in files)
t = t.replace("FILES_LIST", f"{len(files)} files below `E` (new or changed since the pre-edit snapshot, three foreign changes by peers excluded): " + txt + ". Outside `E`: `📜️script.ts` (7 lines), `Cargo.lock` (two `update --workspace --offline` runs), `🧰️framework/🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/{🟦️.ts, 🧪️tests/🟦️.ts}`, `🧰️framework/🔨️modules/🖱️ui/🧪️tests/♻️physical-job-close/🟦️.ts`, `PLAN-RENDERER-WGPU-CRATES.md`, `🗑️generated/rwp0-crates.json`")
write(P, t)
print("filled", len(files))
