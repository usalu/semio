#!/usr/bin/env python3
"""Independent key-completeness check of the cached site `build` (R2): the TypeScript module resolver's value-import closure of the Vite entry and configuration (x-site's site-closure.ts)
minus the declared inputs minus the closure the inference derives (x-inputs' real-inferred.ts) must be empty (generated modules excluded). usage: check-build-closure.py <root>"""
import json, os, subprocess, sys, tempfile
root = os.path.abspath(sys.argv[1])
tools = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
with tempfile.TemporaryDirectory() as scratch:
    closure, inferred = os.path.join(scratch, "closure.json"), os.path.join(scratch, "inferred.json")
    env = {**os.environ, "REPO": root, "TARGET": "build", "SEMIO_INPUTS_ROOT": root}
    for command in (["bun", os.path.join(tools, "x-site-tools", "site-closure.ts"), closure], ["bun", os.path.join(tools, "x-inputs-tools", "real-inferred.ts"), "build", inferred]):
        run = subprocess.run(command, env=env, capture_output=True, text=True, cwd=root)
        if run.returncode:
            sys.exit(run.stderr[-800:])
    reached, derived = json.load(open(closure, encoding="utf8")), json.load(open(inferred, encoding="utf8"))
    covered = {path.replace("{workspaceRoot}/", "") for path in derived["closure"]}
    gap = [path for path in reached["missing"] if "🤖️generated" not in path and path not in covered]
    print(json.dumps({"reached": reached["reached"], "declared_gap": len(reached["missing"]), "inferred": len(covered), "gap": gap}, ensure_ascii=False))
    sys.exit(1 if gap else 0)
