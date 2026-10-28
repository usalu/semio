#!/usr/bin/env python3
"""🏗️ [sealprep] Rebuilds the two staged batches of the sealed flip from the CURRENT files of a root (default: live), so `stage.py check` says `base` there.
usage: make.py [--root DIR]
batch `tools`: self-g-tools/post-flip.sh -> audit11's copy (a blocked promotion fails the flip). The mirror default (`mirror.py run` sets SEMIO_REPO_SEAL=live) is mirror_seal.py, an anchored edit, because other lanes keep editing mirror-tools/mirror.py.
batch `flip` : the sealed policy line `defaultMode` live -> sealed (the same one-line edit as self-g-tools/flip.py, as a hashed row).
batch `chainlaw`: the process-chain law pinned per mode (live bounds unchanged, sealed bounds measured), edited by chainlaw.py on a scratch copy of the CURRENT files, because the sealed default adds the trampoline to the chain (3 heavy processes instead of 2).
The tools row applies only to a root that holds the ticket folder (the live root); the flip row applies everywhere."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(C))))))
TICKET = os.path.relpath(C, ROOT)
POLICY = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔣️.json"
sha = lambda data: hashlib.sha256(data).hexdigest()
def stage(name, rows):
    base = os.path.join(HERE, "staged", name)
    for part in ("files", "base"): shutil.rmtree(os.path.join(base, part), ignore_errors=True); os.makedirs(os.path.join(base, part))
    manifest = {}
    for index, (rel, old, new) in enumerate(rows):
        key = "f%03d" % index
        open(os.path.join(base, "base", key), "wb").write(old); open(os.path.join(base, "files", key), "wb").write(new)
        manifest[rel] = {"file": key, "base": sha(old), "new": sha(new)}
    json.dump(manifest, open(os.path.join(base, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "stage.template.py"), encoding="utf8").read().replace("@NAME@", name)
    open(os.path.join(base, "stage.py"), "w", encoding="utf8").write(template)
    print(name, len(manifest), "rows ->", os.path.relpath(base, C))
read = lambda rel: open(os.path.join(ROOT, rel), "rb").read()
policy = read(POLICY); anchor = b'  "defaultMode": "live",'
assert policy.count(anchor) == 1, "policy anchor: the policy is not in live mode (or the line moved)"
post = TICKET + "/self-g-tools/post-flip.sh"
audit_post = open(os.path.join(C, "audit11-tools/staged/flip/post-flip.sh"), "rb").read()
stage("flip", [(POLICY, policy, policy.replace(anchor, b'  "defaultMode": "sealed",'))])
stage("tools", [(post, read(post), audit_post)])
import subprocess, tempfile
CHAIN = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔗️process-chain/"
names = ["🔣️.json", "🧬️schema/🔣️.json", "🟦️.ts"]
with tempfile.TemporaryDirectory(prefix="sp") as scratch:
    rows = []
    for name in names:
        target = os.path.join(scratch, CHAIN + name); os.makedirs(os.path.dirname(target), exist_ok=True); shutil.copy2(os.path.join(ROOT, CHAIN + name), target)
    subprocess.run([sys.executable, "-I", os.path.join(HERE, "chainlaw.py"), scratch], check=True)
    for name in names: rows.append((CHAIN + name, read(CHAIN + name), open(os.path.join(scratch, CHAIN + name), "rb").read()))
stage("chainlaw", rows)
