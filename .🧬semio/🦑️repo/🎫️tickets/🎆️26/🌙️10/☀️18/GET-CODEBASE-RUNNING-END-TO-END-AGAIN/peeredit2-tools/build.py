#!/usr/bin/env python3
"""🏗 Builds (and rebases) the flat staged batch of a lane change set from the CURRENT files of a root: every edited file's base is the root's bytes and its new bytes are the anchored edits of `<set>/edits_*.py` applied in order (anchors must be unique, see anchored.py); every new file (`NEW <rel>` in files.txt) and every whole-file replacement of an existing file (`REPLACE <rel>`, base = the root's bytes) comes from the flat store `<set>/new/` (`new/map.json` maps rel -> nNN). Writes `staged/<set>/{manifest.json,files/,base/,stage.py}` from the house template (mode-keeping atomic replace).
usage: python3 -I build.py <set> <root> [--compare <dev-root>] | python3 -I build.py <set> --capture <dev-root>   (--capture copies the new files of files.txt from a dev root into the flat store)"""
import hashlib, json, os, runpy, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sha = lambda data: hashlib.sha256(data).hexdigest()
def rows(name):
    out = []
    for line in open(os.path.join(HERE, name, "files.txt"), encoding="utf-8"):
        line = line.rstrip("\n")
        if line: out.append(("new", line[4:]) if line.startswith("NEW ") else ("replace", line[8:]) if line.startswith("REPLACE ") else (False, line))
    return out
def main():
    name, store = sys.argv[1], os.path.join(HERE, sys.argv[1])
    if "--capture" in sys.argv:
        dev = sys.argv[sys.argv.index("--capture") + 1]; os.makedirs(os.path.join(store, "new"), exist_ok=True)
        mapping = {rel: "n%02d" % i for i, (_n, rel) in enumerate([r for r in rows(name) if r[0]])}
        for rel, flat in mapping.items(): shutil.copyfile(os.path.join(dev, rel), os.path.join(store, "new", flat))
        json.dump(mapping, open(os.path.join(store, "new", "map.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1); print("captured", len(mapping)); return
    root = os.path.abspath(sys.argv[2]); out = os.path.join(HERE, "staged", name)
    edits = []
    for module in sorted(f for f in os.listdir(store) if f.startswith("edits_") and f.endswith(".py")): edits += runpy.run_path(os.path.join(store, module))["EDITS"]
    texts, bad = {}, 0
    for rel, old, new, *mode in edits:
        if rel not in texts: texts[rel] = open(os.path.join(root, rel), encoding="utf-8").read()
        count = texts[rel].count(old)
        if mode == ["all"] and count: texts[rel] = texts[rel].replace(old, new); continue
        if count != 1: print("ANCHOR %d× %s :: %s" % (count, rel[-60:], old[:80].replace("\n", "⏎"))); bad += 1; continue
        texts[rel] = texts[rel].replace(old, new, 1)
    if bad: sys.exit(2)
    mapping = json.load(open(os.path.join(store, "new", "map.json"), encoding="utf-8"))
    shutil.rmtree(out, ignore_errors=True); os.makedirs(os.path.join(out, "files")); os.makedirs(os.path.join(out, "base"))
    manifest = {}
    for index, (new, rel) in enumerate(rows(name)):
        flat = "f%02d" % (index + 1)
        if new == "new":
            data = open(os.path.join(store, "new", mapping[rel]), "rb").read()
            if os.path.exists(os.path.join(root, rel)): print("NEW FILE EXISTS ON ROOT", rel); sys.exit(2)
            manifest[rel] = {"file": flat, "base": None, "new": sha(data)}
        elif new == "replace":
            base, data = open(os.path.join(root, rel), "rb").read(), open(os.path.join(store, "new", mapping[rel]), "rb").read()
            open(os.path.join(out, "base", flat), "wb").write(base); manifest[rel] = {"file": flat, "base": sha(base), "new": sha(data)}
        else:
            if rel not in texts: print("NO EDIT FOR", rel); sys.exit(2)
            base = open(os.path.join(root, rel), "rb").read(); data = texts[rel].encode("utf-8")
            open(os.path.join(out, "base", flat), "wb").write(base); manifest[rel] = {"file": flat, "base": sha(base), "new": sha(data)}
        open(os.path.join(out, "files", flat), "wb").write(data)
    missing = set(texts) - {rel for kind, rel in rows(name) if not kind}
    if missing: print("EDITED BUT NOT LISTED", missing); sys.exit(2)
    json.dump(manifest, open(os.path.join(out, "manifest.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    template = open(os.path.join(HERE, "..", "xplat5-tools", "stage.template.py"), encoding="utf-8").read()
    head, rest = template.split('"""', 2)[0], template.split('"""', 2)[2]
    doc = open(os.path.join(store, "STAGE_DOC.txt"), encoding="utf-8").read().strip()
    open(os.path.join(out, "stage.py"), "w", encoding="utf-8").write(head + '"""' + doc + '"""' + rest)
    shutil.copyfile(os.path.join(store, "verify.sh"), os.path.join(out, "verify.sh"))
    print("staged", len(manifest), "files into", out)
    if "--compare" in sys.argv:
        dev = sys.argv[sys.argv.index("--compare") + 1]
        for rel, row in manifest.items():
            other = os.path.join(dev, rel)
            if not os.path.exists(other) or sha(open(other, "rb").read()) != row["new"]: print("DIFFERS FROM DEV", rel[-80:])
main()
