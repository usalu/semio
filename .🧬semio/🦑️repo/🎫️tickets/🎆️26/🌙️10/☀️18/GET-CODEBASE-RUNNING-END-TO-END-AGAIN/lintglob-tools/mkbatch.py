"""🧺 Builds a staged batch in `runner-tools/staged/<batch>/` in the house style.

`mkbatch.py init <batch> <rel>...`: copies each live file to `base/fNN<ext>` and `files/fNN<ext>` (edit the `files/` copies), writes `rels.json`, `stage.py` (from `xplat5-tools/stage.template.py`).
`mkbatch.py add <batch> <rel>...`: appends files the same way.
`mkbatch.py seal <batch>`: writes `manifest.json` `{rel:{file,base,new}}` from `base/` and `files/` and prints the edited files.
`mkbatch.py rebase <batch>`: refreshes `base/` from live for files whose live bytes changed and prints them (re-apply your edit after)."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
ROOT = os.path.abspath(os.path.join(C, *([".."] * 7)))


def sha(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None


def add(folder, rels):
    rows = json.load(open(os.path.join(folder, "rels.json"))) if os.path.exists(os.path.join(folder, "rels.json")) else {}
    for rel in rels:
        if rel in rows:
            continue
        name = "f%02d%s" % (len(rows) + 1, os.path.splitext(rel)[1])
        rows[rel] = name
        live = os.path.join(ROOT, rel)
        for sub in ("base", "files"):
            os.makedirs(os.path.join(folder, sub), exist_ok=True)
            if os.path.exists(live):
                shutil.copyfile(live, os.path.join(folder, sub, name))
        print(name, rel)
    json.dump(rows, open(os.path.join(folder, "rels.json"), "w"), ensure_ascii=False, indent=1)


def seal(folder):
    rows = json.load(open(os.path.join(folder, "rels.json")))
    manifest = {rel: {"file": name, "base": sha(os.path.join(folder, "base", name)), "new": sha(os.path.join(folder, "files", name))} for rel, name in rows.items()}
    json.dump(manifest, open(os.path.join(folder, "manifest.json"), "w"), ensure_ascii=False, indent=1)
    for rel, row in manifest.items():
        print("edited" if row["base"] != row["new"] else "SAME  ", row["file"], rel[-100:])


def rebase(folder):
    for rel, name in json.load(open(os.path.join(folder, "rels.json"))).items():
        if sha(os.path.join(ROOT, rel)) != sha(os.path.join(folder, "base", name)):
            shutil.copyfile(os.path.join(ROOT, rel), os.path.join(folder, "base", name))
            print("live moved:", name, rel)


if __name__ == "__main__":
    verb, batch = sys.argv[1], sys.argv[2]
    folder = os.path.join(HERE, "staged", batch)
    if verb == "init":
        os.makedirs(folder, exist_ok=True)
        shutil.copyfile(os.path.join(C, "xplat5-tools", "stage.template.py"), os.path.join(folder, "stage.py"))
        add(folder, sys.argv[3:])
    elif verb == "add":
        add(folder, sys.argv[3:])
    elif verb == "seal":
        seal(folder)
    else:
        rebase(folder)
