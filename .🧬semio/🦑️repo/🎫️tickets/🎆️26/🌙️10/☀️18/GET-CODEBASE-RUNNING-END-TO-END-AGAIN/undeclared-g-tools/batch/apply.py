#!/usr/bin/env python3
"""undeclared-g batch: apply | revert | check. Idempotent, anchored, byte-backed (backup/ keeps the pre-apply bytes of every file once). Refuses to apply over a file whose bytes are neither the base nor already patched.
usage: python3 apply.py apply|revert|check"""
import hashlib, json, os, shutil, sys, subprocess
W = os.environ.get("UNDECLARED_G_ROOT", "/Users/ueli/Documents/semio")
HERE = os.path.dirname(os.path.abspath(__file__))
L = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FILES = {
    "f1": f"{L}/🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs",
    "f4": f"{L}/📥️inputs/🟨️.mjs",
    "course": "🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json",
}
BASE = json.load(open(os.path.join(HERE, "base-hashes.json"), encoding="utf8"))
sha = lambda p: hashlib.sha256(open(os.path.join(W, p), "rb").read()).hexdigest()
backup = lambda key: os.path.join(HERE, "backup", key)

def patched_f1(text): return "resolutionConfigsV1" in text
def patched_f4(text): return 'const ownerExtensions = native ? "{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,jsx,mjs,cjs,css,scss,html,svg,sql}" : extensions;' in text
def patched_course(text): return '"closure": { "typescript": ["{projectRoot}/tsconfig.site.json"], "modules": ["{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts"] }' in text
STATE = {"f1": patched_f1, "f4": patched_f4, "course": patched_course}

def edit(key, text):
    if key == "f4":
        old = 'const ownerExtensions = native ? "{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,mjs,cjs}" : extensions;'
        assert text.count(old) == 1, "f4 anchor"
        return text.replace(old, 'const ownerExtensions = native ? "{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,jsx,mjs,cjs,css,scss,html,svg,sql}" : extensions;')
    if key == "course":
        old = '"closure": { "typescript": ["{projectRoot}/tsconfig.site.json"] }'
        assert text.count(old) == 1, "course anchor"
        return text.replace(old, '"closure": { "typescript": ["{projectRoot}/tsconfig.site.json"], "modules": ["{workspaceRoot}/🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts"] }')
    raise KeyError(key)

def state(key):
    text = open(os.path.join(W, FILES[key]), encoding="utf8").read()
    if STATE[key](text): return "applied"
    return "base" if sha(FILES[key]) == BASE[key] else "changed-since-staging"

cmd = sys.argv[1] if len(sys.argv) > 1 else "check"
if cmd == "check":
    for key in FILES: print(key, state(key), FILES[key])
elif cmd == "apply":
    rows = {key: state(key) for key in FILES}
    if "changed-since-staging" in rows.values(): sys.exit(f"refused, files changed since staging: {rows}")
    os.makedirs(os.path.join(HERE, "backup"), exist_ok=True)
    for key, row in rows.items():
        if row == "applied": print(key, "already applied"); continue
        path = os.path.join(W, FILES[key])
        if not os.path.exists(backup(key)): shutil.copyfile(path, backup(key))
        if key == "f1":
            run = subprocess.run(["python3", "-I", os.path.join(HERE, "../staged/f1/patch.py"), path], capture_output=True, text=True)
            print(key, run.stdout.strip(), run.stderr.strip()); assert run.returncode == 0
        else:
            text = edit(key, open(path, encoding="utf8").read())
            open(path, "w", encoding="utf8").write(text); print(key, "applied")
elif cmd == "revert":
    for key in FILES:
        if os.path.exists(backup(key)) and state(key) == "applied":
            shutil.copyfile(backup(key), os.path.join(W, FILES[key])); print(key, "reverted")
        else: print(key, "nothing to revert (" + state(key) + ")")
else: sys.exit("usage: apply|revert|check")
