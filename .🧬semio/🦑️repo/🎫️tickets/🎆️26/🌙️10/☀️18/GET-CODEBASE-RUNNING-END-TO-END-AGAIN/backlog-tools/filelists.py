"""📋 [backlog] writes backlog-tools/files/<id>.txt (the repo-relative paths every guarded batch touches, from the batch's STRUCTURED data, never from abbreviated check output); land.py backs these up before `apply`. usage: filelists.py"""
import json, os, tarfile
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MOD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"; LIB = MOD + "📚️library/"
J = lambda p: json.load(open(f"{C}/{p}", encoding="utf8"))
out = {}
for b in ["tests", "tests2", "runtimelaw", "leasesentry", "witnesshost", "gaterace", "startupbudget"]:
    out["red-" + b] = [f["file"] for f in J(f"red-g-tools/staged/{b}/edits.json")["files"]]
for name, tool in [("census2", "census2-g-tools/staged"), ("fdleak-guard", "fdleak-g-tools/staged/guard")]:
    out[name] = [f["file"] for f in J(f"{tool}/batch.json")["files"]] + J(f"{tool}/new-files.json")
out["setup-g"] = list(J("setup-g-tools/staged/index-edited.json"))
for base in ("overlay",):
    for dp, _, fn in os.walk(f"{C}/setup-g-tools/staged/{base}"):
        for f in fn: out["setup-g"].append(os.path.relpath(os.path.join(dp, f), f"{C}/setup-g-tools/staged/{base}"))
out["obsrows"] = [LIB + f["file"] for f in J("obsrows-g-tools/batch/batch.json")["files"]]
try:
    out["obsrows"] += [LIB + n for n in tarfile.open(f"{C}/obsrows-g-tools/batch/new-files.tgz").getnames() if not n.endswith("/")]
except Exception: pass
out["obsrows2"] = [f["file"] for f in J("obsrows2-g-tools/staged/obsrows2/edits.json")["files"]]
r = J("rev3fix-g-tools/staged/rev3fix/edits.json"); out["rev3fix"] = [e["file"] for e in r["edits"]] + [n["path"] for n in r["newFiles"]]
m = J("seal2-g-tools/staged/seal2/manifest.json"); out["seal2"] = [e["file"] for e in m["edits"]] + [(f["path"] if isinstance(f, dict) else f) for f in m["files"]]
for name, d in [("gram3-index-chunks", "gram3-g-tools/staged/index-chunks"), ("gram3-churn", "gram3-g-tools/staged/churn"), ("gram2-row-table", "gram2-g-tools/staged/row-table"), ("walk-walks", "walk-g-tools/staged/walks"), ("walk-footprint", "walk-g-tools/staged/footprint")]:
    e = J(f"{d}/edits.json"); out[name] = [x.get("file") or e.get("file") for x in e["edits"]] + list(J(f"{d}/tree/manifest.json"))
out["gram4-shape-keys"] = [x["file"] for x in J("gram4-g-tools/staged/shape-keys/edits.json")["edits"]]
for dp, _, fn in os.walk(f"{C}/gram4-g-tools/staged/shape-keys/tree"):
    for f in fn: out["gram4-shape-keys"].append(os.path.relpath(os.path.join(dp, f), f"{C}/gram4-g-tools/staged/shape-keys/tree"))
out["bench3-observer"] = [LIB + "🏗️build/📏️incremental/🏎️bench/" + f for f in ("🧬️schema/🔣️.json", "🧪️tests/🧫️fixtures/🔣️.json")]
os.makedirs(f"{C}/backlog-tools/files", exist_ok=True)
for k, v in out.items():
    open(f"{C}/backlog-tools/files/{k}.txt", "w", encoding="utf8").write("\n".join(sorted(set(x for x in v if x))) + "\n"); print(f"{k:20} {len(set(v))}")
