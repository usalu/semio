"""stage.py <build|apply|revert|check> [--mirror M] [--root R]: stages the nxlock-g batch `nxlock` (files listed in `files-nxlock.txt`, bytes in `staged/nxlock/files/f<NN>`, flat names because the real paths exceed the path limit inside the ticket).
The batch is layered on nxfix-g's batch `core`: a file `core` also stages has the `core` bytes as its base. build: copies the files from the mirror and records base and new sha256. apply: all-or-nothing, refuses while `core` is not applied (state needs-core) or a file changed since staging (base-changed); originals go to `staged/nxlock/backup/`. revert: restores them for every file that still equals its new bytes. check: one state per file."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged", "nxlock")
MANIFEST = os.path.join(STAGED, "manifest.json")
CORE = os.path.join(HERE, "..", "nxfix-g-tools", "staged", "core", "manifest.json")


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None


def arg(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default


def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".nxlock-tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.replace(temporary, path)


def build(mirror, root):
    core = json.load(open(CORE, encoding="utf-8"))
    rows = {}
    for index, rel in enumerate(line.strip() for line in open(os.path.join(HERE, "files-nxlock.txt"), encoding="utf-8").read().split("\n") if line.strip()):
        data = open(os.path.join(mirror, rel), "rb").read()
        name = "f%02d" % index
        replace(os.path.join(STAGED, "files", name), data)
        layered = core.get(rel)
        rows[rel] = {"file": name, "base": layered["new"] if layered else digest(os.path.join(root, rel)), "coreBase": layered["base"] if layered else None, "new": hashlib.sha256(data).hexdigest()}
    json.dump(rows, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("built", len(rows), "files")


def states(root, rows):
    out = {}
    for rel, row in rows.items():
        now = digest(os.path.join(root, rel))
        out[rel] = "applied" if now == row["new"] else "pending" if now == row["base"] else "needs-core" if row["coreBase"] is not None and now == row["coreBase"] else "base-changed"
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
        bad = [rel for rel, value in state.items() if value in ("base-changed", "needs-core")]
        if bad:
            print("REFUSED (apply nxfix-g batch core first when needs-core; base-changed means a peer edited the file since staging):\n" + "\n".join(state[rel] + " " + rel for rel in bad))
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
