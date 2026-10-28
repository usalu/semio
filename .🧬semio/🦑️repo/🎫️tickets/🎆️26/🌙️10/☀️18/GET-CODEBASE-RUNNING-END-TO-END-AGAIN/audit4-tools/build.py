#!/usr/bin/env python3
"""🧱 Rebuilds a staged audit4 batch from the CURRENT live files (rebase route): copies the batch's live files into a scratch root, replays its anchored edit script (exits on a lost anchor), writes `staged/<batch>/{manifest.json,files,base}` and the batch's stage.py from the mode-keeping template.
usage: python3 -I build.py gatelock|agedlight [--root LIVE]"""
import hashlib, json, os, shutil, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__)); C = os.path.dirname(HERE)
H = "🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/"; S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/"
BATCHES = {
    "gatelock": dict(edit="edit_gatelock.py", files=[H + r for r in ["🎫️gate/📋️contract/🟦️.ts", "🎫️gate/🟦️.ts", "🔼️upgrade/🟦️.ts", "🎫️gate/🤝️handover/🟦️.ts"]], new={H + "🧪️tests/🔏️fence/🟦️.ts": os.path.join(HERE, "src", "fence", "🟦️.ts")},
                     title="🔏 Staged batch `gatelock` (lane audit4): the machine-wide host-capacity gate lock is fenced (release only while still owned, no grant written under a lost lock, progress published after the release)."),
    "agedlight": dict(edit="edit_agedlight.py", files=[S + "🚦️trampoline/🟦️.ts", S + "🧪️tests/🚦️trampoline/🟦️.ts"], new={},
                      title="🕰 Staged batch `agedlight` (lane audit4): under explicit promotion a light command whose aged promotion did not move the snapshot runs the last good snapshot instead of exiting 75."),
}
def sha(data): return hashlib.sha256(data).hexdigest()
def root_of(start):
    here = start
    while not os.path.isdir(os.path.join(here, "🧰️framework")) and os.path.dirname(here) != here: here = os.path.dirname(here)
    return here
def main():
    name = sys.argv[1]; batch = BATCHES[name]
    live = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else root_of(HERE)
    out = os.path.join(HERE, "staged", name); scratch = tempfile.mkdtemp(prefix="a4b-")
    try:
        for rel in batch["files"]:
            os.makedirs(os.path.dirname(os.path.join(scratch, rel)), exist_ok=True); shutil.copyfile(os.path.join(live, rel), os.path.join(scratch, rel))
        if subprocess.run([sys.executable, "-I", os.path.join(HERE, batch["edit"]), scratch]).returncode: sys.exit("edit script refused: anchor lost, nothing written")
        os.makedirs(out, exist_ok=True)
        for sub in ("files", "base"): shutil.rmtree(os.path.join(out, sub), ignore_errors=True); os.makedirs(os.path.join(out, sub))
        manifest = {}
        for index, rel in enumerate(batch["files"]):
            base = open(os.path.join(live, rel), "rb").read(); new = open(os.path.join(scratch, rel), "rb").read(); file = f"e{index:02d}"
            open(os.path.join(out, "base", file), "wb").write(base); open(os.path.join(out, "files", file), "wb").write(new)
            manifest[rel] = {"file": file, "base": sha(base), "new": sha(new)}
        for index, (rel, source) in enumerate(batch["new"].items()):
            new = open(source, "rb").read(); file = f"n{index:02d}"; open(os.path.join(out, "files", file), "wb").write(new)
            manifest[rel] = {"file": file, "base": None, "new": sha(new)}
        json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
        template = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read().splitlines(True)
        template[1] = '"""' + batch["title"] + "\n"
        open(os.path.join(out, "stage.py"), "w", encoding="utf8").write("".join(template))
        print("built", name, len(manifest), "files")
    finally: shutil.rmtree(scratch, ignore_errors=True)
main()
