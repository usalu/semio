#!/usr/bin/env python3
"""🧱 [mem2] Builds one flat staged batch from anchored edits against a root (not a library file): usage `python3 -I mkbatch.py <batch dir> [--root DIR]` where <batch dir>/edits.py defines EDITS = [(rel, [(find, replace), ...])] (each find exactly once in the CURRENT file of the root, applied in order) and NEW = {rel: source file relative to <batch dir>/new}; writes manifest.json {rel:{file,base,new}}, files/, base/ and copies the mode-keeping stage.py template. Re-run it to rebase onto a tree that moved (anchors must still be unique)."""
import hashlib, importlib.util, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TEMPLATE = os.path.join(HERE, "..", "xplat5-tools", "stage.template.py")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    batch = os.path.abspath(sys.argv[1])
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else "/Users/ueli/Documents/semio"
    spec = importlib.util.spec_from_file_location("edits", os.path.join(batch, "edits.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    for name in ("files", "base"):
        shutil.rmtree(os.path.join(batch, name), ignore_errors=True)
        os.makedirs(os.path.join(batch, name))
    manifest = {}
    for index, (rel, edits) in enumerate(module.EDITS):
        data = open(os.path.join(root, rel), "rb").read()
        text = data.decode("utf-8")
        for find, replace in edits:
            if text.count(find) != 1:
                sys.exit(f"anchor not exactly once ({text.count(find)}) in {rel}: {find[:120]}")
            text = text.replace(find, replace)
        name = f"f{index:02d}"
        open(os.path.join(batch, "base", name), "wb").write(data)
        open(os.path.join(batch, "files", name), "wb").write(text.encode("utf-8"))
        manifest[rel] = {"file": name, "base": sha(data), "new": sha(text.encode("utf-8"))}
    for index, (rel, source) in enumerate(getattr(module, "NEW", {}).items()):
        data = open(os.path.join(batch, "new", source), "rb").read()
        if os.path.exists(os.path.join(root, rel)):
            sys.exit(f"new file already exists in the root: {rel}")
        name = f"n{index:02d}"
        open(os.path.join(batch, "files", name), "wb").write(data)
        manifest[rel] = {"file": name, "base": None, "new": sha(data)}
    json.dump(manifest, open(os.path.join(batch, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    template = open(TEMPLATE, encoding="utf-8").read()
    doc = getattr(module, "DOC", "Staged batch (lane mem2).")
    start, end = template.index('"""'), template.index('\nusage:')
    open(os.path.join(batch, "stage.py"), "w", encoding="utf-8").write(template[:start] + '"""' + doc + template[end:])
    print(json.dumps({"batch": os.path.basename(batch), "root": root, "files": len(manifest)}))


if __name__ == "__main__":
    main()
