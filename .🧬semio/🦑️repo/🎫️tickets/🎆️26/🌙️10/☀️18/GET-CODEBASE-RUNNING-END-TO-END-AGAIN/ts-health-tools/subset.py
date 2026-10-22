import json, sys, os, subprocess
base, tag = sys.argv[1], sys.argv[2]
files = sys.argv[3:]
root = "/Users/ueli/Documents/semio/"
G = root + ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/ts-health-run/"
cfg = {"extends": root + base, "compilerOptions": {"incremental": False, "noEmit": True, "tsBuildInfoFile": None}, "include": [], "files": [root + f for f in files]}
os.makedirs(G + "subset", exist_ok=True)
p = G + "subset/" + tag + ".json"
json.dump(cfg, open(p, "w"), ensure_ascii=False, indent=1)
env = dict(os.environ, NODE_OPTIONS="--max-old-space-size=8192")
r = subprocess.run(["nice", "-n", "10", "node", root + "node_modules/typescript/bin/tsc", "-p", p, "--noEmit", "--pretty", "false"], capture_output=True, text=True, cwd=root, env=env)
out = (r.stdout + r.stderr)
open(G + "subset/" + tag + ".txt", "w").write(out)
lines = [l for l in out.split("\n") if "error TS" in l and any(f.split("/")[-3] in l or True for f in files)]
sel = [l for l in lines if any(l.startswith(f) for f in files)]
print("total errors", len(lines), "in-subset", len(sel))
for l in sel[:40]: print(l[:260])
