import json, sys
sys.stdout.reconfigure(encoding="utf-8")
T = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
f = json.load(open(f"{T}/🧰️cf3/files.json", encoding="utf-8"))
w = json.load(open(f"{T}/🧰️cf3/why.json", encoding="utf-8"))
ex = w["excluded"]
out = ["# 🧾️ Round 2 Changed Files v3", "",
       "Definitive list by agent cf3 (read-only). Script: `🧰️cf3/classify.py` (status via `git status --porcelain -uall -z` under `🎓️teaching`, content check via `git diff HEAD`, untracked by file content, deleted by `git show HEAD:`). Machine-readable: `🧰️cf3/files.json`, reasons per path: `🧰️cf3/why.json`.", "",
       "A changed file counts when: a report names it (v2 exact list, or the explicit directory attributions below); or it lies in the staffage archive, the visuals module, the python package pyproject/uv.lock; or its diff or content touches `staffage`, `fitted_staffage`, `staffage_motion`, `staffage_height`, `staffage_parts`, `person_glyph`, `object_silhouette`, `radiator(` or silhouettes.", "",
       f"Counts: created {len(f['created'])}, modified {len(f['modified'])}, deleted {len(f['deleted'])}, excluded {len(ex)}, still uncertain 1.", ""]
for title, k in (("Created", "created"), ("Modified", "modified"), ("Deleted", "deleted")):
    out += [f"## {title} ({len(f[k])})", ""] + [f"- `{p}`" for p in f[k]] + [""]
out += [f"## Excluded (foreign) ({len(ex)})", "",
        "Rename-only call-site updates for the public `surface_finishes` / `geometry_producers` / `source_text` API (BIM model module), no staffage relation, no report names these paths:", ""]
for p, st, a in ex:
    if "nakagin" in p:
        continue
    out.append(f"- `{p}` ({st}): BIM API rename follow-through, no staffage calls")
out += ["", "Nakagin persisted occlusion-order cache, new test and case file, no staffage calls and no report names them:", ""]
for p, st, a in ex:
    if "nakagin" in p:
        out.append(f"- `{p}` ({st}): concurrent nakagin order-cache work")
out += ["", "## Still uncertain (1)", "",
        "- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py` (M): kept in Modified because `r2-br-report.md` names it (cutter winding fix), but the same diff also carries the foreign `surface_finishes` rename that the excluded call sites follow.", ""]
open(f"{T}/r2-changed-files-v3.md", "w", encoding="utf-8").write("\n".join(out))
print(len(out))
