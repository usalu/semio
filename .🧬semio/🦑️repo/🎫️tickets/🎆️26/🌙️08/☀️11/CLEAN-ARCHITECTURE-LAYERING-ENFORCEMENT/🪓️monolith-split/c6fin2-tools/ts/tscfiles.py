"""🧪 Type-checks a file set with the repo tsconfig and the cached TypeScript (node_modules is empty in this checkout): writes a tsconfig under the cache, runs tsc, prints diagnostics grouped by code; TS2307 (missing external packages) is summarised only. usage: tscfiles.py <label> <root-list-file> [filter-substring]"""
import json, os, subprocess, sys, collections, re
R = "/Users/ueli/Documents/semio"
label, listfile = sys.argv[1], sys.argv[2]
files = [l.strip() for l in open(listfile, encoding='utf-8') if l.strip()]
files = [f if f.startswith('/') else os.path.join(R, f) for f in files]
files = [f for f in files if os.path.exists(f)]
cfg = {"extends": R + "/tsconfig.json", "compilerOptions": {"noEmit": True, "incremental": False, "composite": False}, "include": [], "files": files}
out = f"/Users/ueli/.cache/c6fin2/tsc-{label}.json"
json.dump(cfg, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
pr = subprocess.run(["node", "/Users/ueli/.cache/c6fin2/nm/node_modules/typescript/bin/tsc", "--noEmit", "-p", out, "--pretty", "false"], capture_output=True, text=True, cwd=R)
txt = pr.stdout + pr.stderr
open(f"/Users/ueli/.cache/c6fin2/tsc-{label}.txt", "w", encoding="utf-8").write(txt)
codes = collections.Counter(re.findall(r"error (TS\d+)", txt))
print("exit", pr.returncode, "diagnostics", sum(codes.values()), dict(codes.most_common(12)))
