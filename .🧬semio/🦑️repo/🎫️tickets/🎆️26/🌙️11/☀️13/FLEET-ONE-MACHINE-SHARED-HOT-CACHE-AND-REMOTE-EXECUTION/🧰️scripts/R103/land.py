import hashlib, os, shutil, sys
R = "/Users/ueli/Documents/semio/"
N = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/"
SRC = os.path.dirname(os.path.abspath(__file__)) + "/"
BK = "/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R103/backup/"
dry = "--dry" in sys.argv
def read(p): return open(p, encoding="utf8").read()
def once(s, old, new):
    assert s.count(old) == 1, ("anchor", s.count(old), old[:90])
    return s.replace(old, new)
def sha(p): return hashlib.sha256(open(p, "rb").read()).hexdigest()[:12]
nodes = read(N + "🟨️.mjs")
add = read(SRC + "set-aside.add.txt")
nodes = once(nodes, "/** 🕸️ How often the projects being inferred are named again", add + "/** 🕸️ How often the projects being inferred are named again")
nodes = once(nodes, "const loop=async end=>{await resolved;", read(SRC + "set-aside.aside.txt").rstrip("\n") + "\n  const loop=async end=>{await resolved;")
old = 'await runInOwnerScopeV1(scope,()=>collector?collector.guard({project:job.name,root:job.json.root,phase:"project"},()=>inferProject(job,away)):inferProject(job,away));'
nodes = once(nodes, old, old[:-1] + ".catch(error=>{if(!setAside(error))throw error;});")
import subprocess, tempfile
PT = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🪪️graph-state/📦️project-records/🧳️portable/"
tmp = tempfile.mkdtemp(prefix="r103-")
open(tmp + "/p.ts", "w", encoding="utf8").write(read(PT + "🟦️.ts"))
subprocess.run(["python3", SRC + "edit-portable.py", tmp + "/p.ts", SRC + "portable.add.txt"], check=True)
plan = {N + "🟨️.mjs": nodes, PT + "🟦️.ts": read(tmp + "/p.ts")}
tests = {N + "🧪️tests/🪂️set-aside/🟦️.ts": SRC + "set-aside.test.ts", PT + "🧪️tests/🪶️rest-walk/🟦️.ts": SRC + "rest-walk.test.ts"}
if dry:
    for k in plan: print("ok", k[-40:], len(plan[k]))
    sys.exit(0)
before = {k: sha(k) for k in plan}
for k in plan:
    os.makedirs(os.path.dirname(BK + k[len(R):]), exist_ok=True)
    shutil.copy(k, BK + k[len(R):])
for k, v in plan.items():
    tmp = k + ".r103tmp"
    open(tmp, "w", encoding="utf8").write(v)
    os.replace(tmp, k)
for dest, src in tests.items():
    os.makedirs(os.path.dirname(dest), exist_ok=True)
    shutil.copy(src, dest)
for k in plan: print(k[len(R):][-60:], before[k], "->", sha(k))
