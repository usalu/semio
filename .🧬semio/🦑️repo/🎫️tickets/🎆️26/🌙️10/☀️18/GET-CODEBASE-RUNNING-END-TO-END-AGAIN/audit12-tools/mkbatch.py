#!/usr/bin/env python3
"""🧱 [audit12] (re)builds staged batch <name> from the CURRENT live files: copies each target to a scratch root, replays the anchored edits of edits.py, writes staged/<name>/{manifest.json,base/,files/,stage.py} with base = live; usage: python3 -I mkbatch.py <name> [--root DIR]  (exit 1 = anchor lost; verify.sh is kept)"""
import hashlib, json, os, shutil, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
sys.path.insert(0, HERE)
from edits import BATCHES
DOCS = {
 "sigexit": "🛑 Staged batch `sigexit` (lane audit12): a signal-ended Script envelope writes its notice and exits 130/143 only when the cancelled run's rejection is not handled, so a caller that catches it keeps its DATA and exit status (progress `sigint` regression of the digest and window laws).",
 "farmexports": "📇 Staged batch `farmexports` (lane audit12): the observer declaration roster fixture names the two exports farmid `authority` added (adoptInferenceAuthorityV1, inferenceAuthorityRowsV1).",
 "saytest": "🗣 Staged batch `saytest` (lane audit12): the native diagnostics budget law reads both finish() renderings in English, as the say batch made the hidden-lines row language-bound.",
 "typedecl": "🔤 Staged batch `typedecl` (lane audit12): the three type errors G46 added (session canonicalRootV1 declaration lacks the platform parameter, door liveness languages typed as string, suites law passes unknown to toContain).",
 "privparent": "📍 Staged batch `privparent` (lane audit12): the private graph workspace parent budget counts the real observation staging suffix (7-digit pid, 13-digit clock) and falls back to a short system scratch, so the bare optional-cache suite on a 27-char checkout no longer overruns 256 units.",
}
def sha(path):
    if not os.path.exists(path): return None
    with open(path, "rb") as f: return hashlib.sha256(f.read()).hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def main():
    name = sys.argv[1]; root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(C)
    scratch = os.path.join(C, "🗑️generated", "audit12", "build", name); shutil.rmtree(scratch, ignore_errors=True)
    for rel in BATCHES[name]: os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True); shutil.copyfile(os.path.join(root, rel), os.path.join(scratch, rel))
    if subprocess.run([sys.executable, "-I", os.path.join(HERE, "edits.py"), name, scratch]).returncode: sys.exit(1)
    out = os.path.join(HERE, "staged", name)
    for sub in ("base", "files"): shutil.rmtree(os.path.join(out, sub), ignore_errors=True); os.makedirs(os.path.join(out, sub))
    manifest = {}
    for index, rel in enumerate(BATCHES[name]):
        file = f"{index:02d}{os.path.splitext(rel)[1]}"
        shutil.copyfile(os.path.join(root, rel), os.path.join(out, "base", file)); shutil.copyfile(os.path.join(scratch, rel), os.path.join(out, "files", file))
        manifest[rel] = {"file": file, "base": sha(os.path.join(root, rel)), "new": sha(os.path.join(scratch, rel))}
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    lines = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read().split("\n")
    lines[1] = '"""' + DOCS[name]
    open(os.path.join(out, "stage.py"), "w", encoding="utf8").write("\n".join(lines))
    shutil.rmtree(scratch, ignore_errors=True)
    print(name, *(f"\n  {r[-90:]} {'changed' if v['base'] != v['new'] else 'UNCHANGED'}" for r, v in manifest.items()))
main()
