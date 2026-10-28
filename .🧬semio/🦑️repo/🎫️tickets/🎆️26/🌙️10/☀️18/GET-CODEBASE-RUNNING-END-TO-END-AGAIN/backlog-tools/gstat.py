"""[backlog] summarise `batch_generic.py <dir> status` output: counts of edit states and new files present. usage: gstat.py <tool> <batch-dir> [root]"""
import ast, collections, os, subprocess, sys
tool, batch = sys.argv[1], sys.argv[2]
env = dict(os.environ)
if len(sys.argv) > 3:
    env["BATCH_ROOT"] = sys.argv[3]
out = subprocess.run(["python3", "-I", tool, batch, "status"], capture_output=True, text=True, env=env)
try:
    s = ast.literal_eval(out.stdout.strip())
except Exception:
    print("ERR", (out.stdout + out.stderr)[-300:]); sys.exit(1)
c = collections.Counter(s["edits"])
n = collections.Counter(bool(v) for v in s["new"].values())
print("edits", dict(c), "new-files present/absent", n.get(True, 0), n.get(False, 0))
