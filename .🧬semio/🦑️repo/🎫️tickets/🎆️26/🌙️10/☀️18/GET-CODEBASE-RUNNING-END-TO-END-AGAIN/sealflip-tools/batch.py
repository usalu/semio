#!/usr/bin/env python3
"""🧰️ [sealflip] Builds the flat batch staged/sealflip from a mirror: `base <root>` keeps the base bytes of every listed file (the mirror state = live + B1 + D2, flip NOT applied), `make <root>` takes the edited bytes from the same mirror and writes files/, manifest.json ({rel:{file,base,new}}) and the stage.py (xplat5 template, keeps file modes). `rebase <root>` = base + make against a newer tree when the anchors still apply (re-run the edit, then make)."""
import hashlib, json, os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__)); OUT = os.path.join(HERE, "staged", "sealflip"); C = os.path.dirname(HERE)
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
FILES = [f"{B}/🏛️sealed/🔣️.json", f"{B}/🏛️sealed/🧬️schema/🔣️.json", f"{B}/🏛️sealed/🟦️.ts", f"{B}/🏛️sealed/🚦️trampoline/🟦️.ts",
         f"{B}/🏛️sealed/🧪️tests/🚦️trampoline/🟦️.ts", f"{B}/🏛️sealed/🧪️tests/🧫️fixtures/📜️trampoline.ts", f"{B}/🏛️sealed/🧪️tests/🗂️store/🟦️.ts",
         f"{B}/🚪️door/🧪️tests/🟦️.ts", f"{B}/🚪️door/🧪️tests/🚦️startup/🟦️.ts", f"{B}/🏛️sealed/README.md", "🧰️framework/🛍️products/🦑️repo/📓️build-guide.md",
         f"{B}/🧭️invocation/🕸️graph/🧠️memo/🧬️schema/🔣️.json", f"{B}/🧭️invocation/🕸️graph/🧠️memo/🧪️tests/🟦️.ts", f"{B}/🔭️witness/🧬️schema/🔣️.json", f"{B}/🔭️witness/🧪️tests/🟦️.ts"]
sha = lambda p: hashlib.sha256(open(p, "rb").read()).hexdigest() if os.path.exists(p) else None
def main(verb, root):
    names = {rel: "f%03d" % i for i, rel in enumerate(FILES, 1)}
    if verb == "base":
        os.makedirs(os.path.join(OUT, "base"), exist_ok=True)
        for rel, n in names.items(): shutil.copyfile(os.path.join(root, rel), os.path.join(OUT, "base", n))
        print("base kept:", len(names)); return
    os.makedirs(os.path.join(OUT, "files"), exist_ok=True); manifest = {}
    for rel, n in names.items():
        new = os.path.join(root, rel); base = os.path.join(OUT, "base", n)
        if sha(new) == sha(base): print("UNCHANGED", rel); continue
        shutil.copyfile(new, os.path.join(OUT, "files", n)); manifest[rel] = {"file": n, "base": sha(base), "new": sha(new)}
    json.dump(manifest, open(os.path.join(OUT, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
    text = open(os.path.join(C, "xplat5-tools", "stage.template.py"), encoding="utf8").read().replace(
        '"""🌌 Staged batch `universe` (lane rustedit): print\'s Bazel plan becomes reusable although a dependency reads a root-anchored pattern.',
        '"""🏛️ Staged batch `sealflip` (lane sealflip): the sealed layer gets its own switch `SEMIO_REPO_SEAL` (the dashboard owns `SEMIO_REPO_IMPLEMENTATION`=rust|go) and explicit promotion (a command runs the promoted snapshot; only `SEMIO_REPO_SEAL=promote` or a snapshot older than a day promotes); neither switch keys the graph memo or the plan witness. Lands AFTER conv-g + seal2, BEFORE the flip.')
    open(os.path.join(OUT, "stage.py"), "w", encoding="utf8").write(text); print("made:", len(manifest), "files")
main(sys.argv[1], os.path.abspath(sys.argv[2]))
