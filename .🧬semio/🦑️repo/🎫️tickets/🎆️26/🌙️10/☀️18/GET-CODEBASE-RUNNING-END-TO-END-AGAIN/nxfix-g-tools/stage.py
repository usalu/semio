"""stage.py <build|apply|revert|check> [--batch NAME] [--mirror M] [--root R]: stages one nxfix-g batch (default `core`; its files are listed in `files-NAME.txt`, its bytes live in `staged/NAME/`).
build: copies the files listed in `files.txt` from the scratch mirror into `staged/files/f<NN>` (flat names: the real paths would exceed the path limit inside the ticket folder; `manifest.json` maps the name to the repository path) and records base (the live bytes) and new sha256 in `staged/manifest.json`.
apply: all-or-nothing; every file must still equal its base (or already equal its new bytes); originals are saved under `staged/backup/`; files are replaced atomically (never edited in place).
revert: restores the saved originals of every file that still equals its new bytes. check: prints one state per file (pending | applied | base-changed)."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
BATCH = sys.argv[sys.argv.index("--batch") + 1] if "--batch" in sys.argv else "core"
STAGED = os.path.join(HERE, "staged", BATCH)
MANIFEST = os.path.join(STAGED, "manifest.json")


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None


def arg(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default


def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".nxfix-tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.replace(temporary, path)


def build(mirror, root):
    rows = {}
    for index, rel in enumerate(line.strip() for line in open(os.path.join(HERE, "files-%s.txt" % BATCH), encoding="utf-8").read().split("\n") if line.strip()):
        data = open(os.path.join(mirror, rel), "rb").read()
        name = "f%02d" % index
        replace(os.path.join(STAGED, "files", name), data)
        rows[rel] = {"file": name, "base": digest(os.path.join(root, rel)), "new": hashlib.sha256(data).hexdigest()}
    os.makedirs(STAGED, exist_ok=True)
    json.dump(rows, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("built", len(rows), "files")


def states(root, rows):
    out = {}
    for rel, row in rows.items():
        now = digest(os.path.join(root, rel))
        out[rel] = "applied" if now == row["new"] else "pending" if now == row["base"] else "base-changed"
    return out


def main():
    mode, root, mirror = sys.argv[1], arg("--root", "/Users/ueli/Documents/semio"), arg("--mirror", "")
    if mode == "build":
        return build(mirror, root)
    rows = json.load(open(MANIFEST, encoding="utf-8"))
    state = states(root, rows)
    if mode == "check":
        for rel, value in state.items():
            print(value, rel)
        return
    if mode == "apply":
        bad = [rel for rel, value in state.items() if value == "base-changed"]
        if bad:
            print("REFUSED: base changed since staging:\n" + "\n".join(bad))
            sys.exit(2)
        for rel, value in state.items():
            if value == "applied":
                continue
            target = os.path.join(root, rel)
            backup = os.path.join(STAGED, "backup", rows[rel]["file"])
            if os.path.exists(target) and not os.path.exists(backup):
                os.makedirs(os.path.dirname(backup), exist_ok=True)
                shutil.copy2(target, backup)
            replace(target, open(os.path.join(STAGED, "files", rows[rel]["file"]), "rb").read())
        print("applied", sum(1 for v in state.values() if v != "applied"), "files")
        return
    if mode == "revert":
        for rel, value in state.items():
            if value != "applied":
                continue
            target, backup = os.path.join(root, rel), os.path.join(STAGED, "backup", rows[rel]["file"])
            if os.path.exists(backup):
                replace(target, open(backup, "rb").read())
            elif rows[rel]["base"] is None:
                os.remove(target)
        print("reverted")
        return
    sys.exit("usage: stage.py build|apply|revert|check")


main()
