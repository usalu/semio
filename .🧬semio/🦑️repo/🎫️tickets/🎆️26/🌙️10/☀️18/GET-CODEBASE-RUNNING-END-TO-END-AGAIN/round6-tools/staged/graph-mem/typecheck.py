#!/usr/bin/env python3
"""Type checks the patched graph module and the new TypeScript laws in the overlay (tsc --noEmit over an explicit file list) and prints only diagnostics of touched files.
usage: typecheck.py [OVERLAY_DIR]"""
import json, os, subprocess, sys
overlay = os.path.abspath(sys.argv[1]) if len(sys.argv) > 1 else "/private/tmp/graph-mem/overlay"
E = lambda code: chr(code) + "️"
L = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
B = os.path.join(L, E(0x1F3D7) + "build"); G = os.path.join(B, E(0x1F578) + "graph")
TS, T = E(0x1F7E6) + ".ts", E(0x1F9EA) + "tests"
files = [os.path.join(G, TS), os.path.join(B, E(0x1F9E0) + "memory", T, TS), os.path.join(B, T, E(0x1F578) + "graph", E(0x1F9F2) + "owner-claims", TS), os.path.join(B, E(0x1F441) + "observation", T, E(0x1F9E0) + "retention", TS), os.path.join(G, E(0x1F570) + "generations", T, E(0x1F9F6) + "shared-strings", TS), os.path.join(G, E(0x1F4E5) + "inference", E(0x1F4E6) + "project-nodes", T, E(0x1F6A6) + "governed-dispatch", TS)]
json.dump({"extends": "./tsconfig.json", "compilerOptions": {"noEmit": True, "incremental": False, "skipLibCheck": True}, "include": [], "files": files}, open(os.path.join(overlay, "tsconfig.graphmem.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
done = subprocess.run(["nice", "-n", "10", os.path.join(overlay, "node_modules/.bin/tsc"), "-p", "tsconfig.graphmem.json", "--pretty", "false"], cwd=overlay, capture_output=True, text=True, env=dict(os.environ, NODE_OPTIONS="--max-old-space-size=3000"))
lines = [line for line in (done.stdout + done.stderr).splitlines() if line.strip()]
touched = [E(0x1F9E0) + "memory", E(0x1F9F2) + "owner-claims", E(0x1F9E0) + "retention", E(0x1F9F6) + "shared-strings", E(0x1F6A6) + "governed-dispatch", G + os.sep + TS]
mine = [line for line in lines if any(name in line for name in touched)]
print("diagnostics total", len(lines), "in touched files", len(mine))
for line in mine[:30]:
    print(line[-260:])
sys.exit(1 if mine else 0)
