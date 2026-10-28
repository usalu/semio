"""📋 [backlog] read-only inventory of every staged batch against the LIVE tree: runs each batch's own `check`/`status` command, writes the raw output to 🗑️generated/backlog/check/<id>.txt and the touched repo paths to 🗑️generated/backlog/files/<id>.txt, prints the per-state counts. Never writes below the repo outside the ticket folder. usage: inventory.py [id...]"""
import ast, collections, json, os, re, subprocess, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = os.path.abspath(os.path.join(C, *[".."] * 7))
G = C + "/🗑️generated/backlog"
os.makedirs(G + "/check", exist_ok=True); os.makedirs(G + "/files", exist_ok=True)
MOD = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"
def py(*a): return ["python3", "-I", *a]
def gen(tool, d): return ("generic", py(C + "/" + tool, C + "/" + d, "status"))
B = {
 "gram3-index-chunks": gen("gram2-g-tools/staged/batch_generic.py", "gram3-g-tools/staged/index-chunks"),
 "gram3-churn": gen("gram2-g-tools/staged/batch_generic.py", "gram3-g-tools/staged/churn"),
 "gram2-row-table": gen("gram2-g-tools/staged/batch_generic.py", "gram2-g-tools/staged/row-table"),
 "walk-walks": gen("walk-g-tools/staged/batch_generic.py", "walk-g-tools/staged/walks"),
 "walk-footprint": gen("walk-g-tools/staged/batch_generic.py", "walk-g-tools/staged/footprint"),
 "gram4-shape-keys": ("sh", ["sh", C + "/gram4-g-tools/staged/shape-keys/apply.sh", "check"]),
 "seal2": ("lines", ["sh", C + "/seal2-g-tools/staged/seal2/check.sh"]),
}
for b in ["rootguard", "unitcache", "sweep", "ledger", "procstore"]:
    B["disk-" + b] = ("lines", ["sh", C + f"/disk-g-tools/staged/{b}/check.sh"])
for b in ["inferfix", "tests", "tests2", "runtimelaw", "leasesentry", "witnesshost", "gaterace", "startupbudget"]:
    B["red-" + b] = ("lines", py(C + "/red-g-tools/stage.py", "check", b))
B.update({
 "fdleak-guard": ("lines", py(C + "/fdleak-g-tools/staged/guard/stage.py", "check")),
 "census2": ("lines", py(C + "/census2-g-tools/staged/stage.py", "check")),
 "testmem2": ("lines", py(C + "/testmem2-g-tools/staged/stage.py", "check")),
 "obsrows": ("lines", py(C + "/obsrows-g-tools/stage.py", "check")),
 "obsrows2": ("lines", py(C + "/obsrows2-g-tools/stage.py", "check", "obsrows2")),
 "bench3-observer": ("lines", ["sh", C + "/bench3-g-tools/staged/observer/apply.sh", "check"]),
 "route2": ("lines", ["bun", C + "/route2-g-tools/apply-staged.mjs", "--dry"]),
 "setup-g": ("lines", py(C + "/setup-g-tools/staged/stage.py", "status")),
 "xplat3": ("lines", py(C + "/xplat3-g-tools/apply.py", "check")),
 "xplat4-platforms": ("lines", py(C + "/xplat4-g-tools/staged/platforms/apply.py", "check")),
 "xplat4-freespace": ("lines", py(C + "/xplat4-g-tools/staged/freespace/apply.py", "check")),
 "xplat4-corpus": ("lines", py(C + "/xplat4-g-tools/staged/corpus/apply.py", "check")),
 "conv-g": ("lines", py(C + "/conv-g-tools/stage.py", "check")),
 "obs-g-fleet": ("lines", py(C + "/obs-g-tools/stage.py", "check")),
 "dashlock": ("lines", py(C + "/dashlock-g-tools/stage.py", "check")),
 "jobsrv": ("lines", py(C + "/jobsrv-g-tools/stage.py", "check")),
 "nxfix-core": ("lines", py(C + "/nxfix-g-tools/stage.py", "check", "--batch", "core")),
 "nxfix-nx-patch": ("lines", py(C + "/nxfix-g-tools/stage.py", "check", "--batch", "nx-patch")),
 "nxlock": ("lines", py(C + "/nxlock-g-tools/stage.py", "check")),
 "rev3fix": ("lines", py(C + "/rev3fix-g-tools/batch.py", C + "/rev3fix-g-tools/staged/rev3fix", "check", R)),
 "store-g-managed": ("lines", py(C + "/store-g-tools/staged/apply-managed-parent-guard.py", "--check")),
 "store-g-foldsafe": ("lines", py(C + "/store-g-tools/staged/apply-foldsafe-preflight.py", "--check")),
})
for d in sorted(os.listdir(C + "/docs3-g-tools/staged")):
    if os.path.isfile(C + f"/docs3-g-tools/staged/{d}/stage.py"):
        B["docs3-" + d] = ("lines", py(C + f"/docs3-g-tools/staged/{d}/stage.py", "check"))
def paths_generic(out):
    s = ast.literal_eval(out.strip()); return sorted(set(s["new"]) ), s
def main():
    want = sys.argv[1:] or list(B)
    for name in want:
        kind, cmd = B[name]
        r = subprocess.run(cmd, capture_output=True, text=True, cwd=R)
        out = r.stdout + r.stderr
        open(f"{G}/check/{name}.txt", "w").write(out)
        files = set(); summary = ""
        if name == "seal2":
            m = json.load(open(C + "/seal2-g-tools/staged/seal2/manifest.json", encoding="utf8"))
            files = {e["file"] for e in m["edits"]} | set(m["files"] if isinstance(m["files"], list) else m["files"])
            summary = " ".join(f"{k}:{v}" for k, v in collections.Counter(l.split()[0] for l in out.splitlines() if l.strip()).items())
        elif kind == "generic":
            try:
                s = ast.literal_eval(r.stdout.strip())
                edits = json.load(open(cmd[3] + "/edits.json", encoding="utf8"))
                for e in edits["edits"]: files.add(e.get("file") or edits.get("file"))
                files |= set(s["new"])
                c = collections.Counter(s["edits"]); n = sum(1 for v in s["new"].values() if v)
                summary = f"edits {dict(c)} new {n}/{len(s['new'])}"
            except Exception as ex: summary = "ERR " + str(ex)[:80]
        else:
            c = collections.Counter()
            for line in out.splitlines():
                m = re.match(r"^\s*([A-Za-z][A-Za-z0-9_()\-:]*)\s+(.*?)\s*(?:sha=\S+)?$", line)
                if not m: continue
                st, p = m.group(1), m.group(2).strip()
                p = re.sub(r"\s+\(.*$", "", p) if " (" in p and not os.path.exists(R + "/" + p) else p
                p = p.replace("R/", MOD, 1) if p.startswith("R/") else p.replace("FW/", "🧰️framework/", 1) if p.startswith("FW/") else p
                if re.match(r"^[^\s].*[/.]", p) and len(p) > 6 and not p.startswith(("state", "summary", "anchors", "all ")):
                    files.add(p); c[st] += 1
                elif st.lower() in ("drift", "state", "clean"): c[st] += 1
            summary = " ".join(f"{k}:{v}" for k, v in sorted(c.items())) or out.strip()[:100].replace("\n", " | ")
        lst = "\n".join(sorted(f for f in files if f)) + "\n"
        open(f"{G}/files/{name}.txt", "w").write(lst)
        if files:
            os.makedirs(C + "/backlog-tools/files", exist_ok=True); open(f"{C}/backlog-tools/files/{name}.txt", "w").write(lst)
        print(f"{name:22} {len(files):4} files  {summary}")
main()
