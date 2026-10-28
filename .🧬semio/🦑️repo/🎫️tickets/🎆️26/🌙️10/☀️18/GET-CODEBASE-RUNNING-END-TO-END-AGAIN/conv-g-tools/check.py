import json, os, hashlib, subprocess, sys, re
import build, ops, lib
H = build.HERE; BUN = sys.argv[1]
paths = list(build.PATHS) + list(getattr(ops, "NEW", {}).keys())
bad = 0
for p in dict.fromkeys(paths):
    key = hashlib.sha256(p.encode()).hexdigest()[:10]; f = os.path.join(H, "new", key)
    if not os.path.exists(f): continue
    t = open(f, encoding="utf8").read()
    if p.endswith(".json"):
        try: json.loads(t)
        except Exception as e: print("JSON-BAD", p[-60:], e); bad += 1
    elif p.endswith((".ts", ".mjs", ".mts")):
        ext = "ts" if p.endswith((".ts", ".mts")) else "js"
        r = subprocess.run([BUN, "-e", "const t=new Bun.Transpiler({loader:'%s'});t.transformSync(await Bun.file(process.argv[1]).text())" % ext, f], capture_output=True, text=True)
        if r.returncode: print("SYNTAX-BAD", p[-70:], r.stderr[:300]); bad += 1
    elif p.endswith(".py"):
        r = subprocess.run(["python3", "-B", "-c", "import ast,sys;ast.parse(open(sys.argv[1],encoding='utf8').read())", f], capture_output=True, text=True)
        if r.returncode: print("PY-BAD", p[-60:], r.stderr[-200:]); bad += 1
print("bad", bad)
