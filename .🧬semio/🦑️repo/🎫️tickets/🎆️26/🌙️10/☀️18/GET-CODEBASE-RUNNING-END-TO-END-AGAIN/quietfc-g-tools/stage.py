"""📦️ quietfc-g: the FIX-OF-RED batch `quietfc2` as flat whole-file replacements (`staged/files/fNNN` + `staged/manifest.json`, repository path -> {file, base, new}).
Usage: python3 -I stage.py build [--overlay DIR] | check|apply|revert [--root DIR] [--backup DIR]
build: after `sh rebuild.sh`, compares the overlay with the live tree and stages every changed or new file with the sha256 of the live file as `base`. check: pending / applied / base-changed per file. apply: all-or-nothing (the backup of every pending file is taken from the file as it is now, so a stale backup of an older base can never be restored), refuses on base-changed (a peer edited the file since staging: run `sh apply.sh apply` instead, which edits by anchor, or `build` again). revert: puts the backups back and removes the new files. The batch contains the whole quietfc change (it was reverted at 21:48) plus the g5 upgrade-size fix, the quiet-limit and the soft-stop grants."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
MANIFEST = os.path.join(STAGED, "manifest.json")
REPO = "/Users/ueli/Documents/semio"
OVERLAY = "/private/tmp/claude-501/-Users-ueli-Documents-semio/de5d5cb4-dca7-469c-b840-78655a0500ed/scratchpad/quietfc-g-ov"


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.isfile(path) else None


def arg(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default


def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".quietfc-tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.replace(temporary, path)


def build(overlay):
    sys.path.insert(0, HERE)
    import overlay as farm
    rows, index = {}, 0
    roots = list(farm.REAL_DIRS) + list(farm.REAL_FILES)
    for rel_root in roots:
        top = os.path.join(overlay, rel_root)
        walk = [(os.path.dirname(top), [], [os.path.basename(top)])] if os.path.isfile(top) else os.walk(top)
        for base, dirs, names in walk:
            dirs[:] = [d for d in dirs if d not in ("node_modules", ".DS_Store")]
            for name in sorted(names):
                if name in (".DS_Store",) or name.endswith(".quietfc.tmp"):
                    continue
                source = os.path.join(base, name)
                rel = os.path.relpath(source, overlay)
                if os.path.islink(source) or len(rel) > 250:
                    continue
                data = open(source, "rb").read()
                live = digest(os.path.join(REPO, rel))
                if live == hashlib.sha256(data).hexdigest():
                    continue
                file = "f%03d" % index
                index += 1
                replace(os.path.join(STAGED, "files", file), data)
                rows[rel] = {"file": file, "base": live, "new": hashlib.sha256(data).hexdigest()}
    json.dump(rows, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print("built", len(rows), "files")


def states(root, rows):
    out = {}
    for rel, row in rows.items():
        now = digest(os.path.join(root, rel))
        out[rel] = "applied" if now == row["new"] else "pending" if now == row["base"] else "base-changed"
    return out


def main():
    mode, root = sys.argv[1], arg("--root", REPO)
    backups = arg("--backup", os.path.join(STAGED, "backup"))
    if mode == "build":
        return build(arg("--overlay", OVERLAY))
    rows = json.load(open(MANIFEST, encoding="utf-8"))
    state = states(root, rows)
    if mode == "check":
        for rel, value in state.items():
            print(value, rel)
        return
    if mode == "apply":
        bad = [rel for rel, value in state.items() if value == "base-changed"]
        if bad:
            print("REFUSED, a peer edited these since staging (run `sh apply.sh apply` for the anchor-based route):\n" + "\n".join(bad))
            sys.exit(2)
        for rel, value in state.items():
            if value == "applied":
                continue
            target = os.path.join(root, rel)
            backup = os.path.join(backups, rows[rel]["file"])
            if os.path.exists(target):
                os.makedirs(os.path.dirname(backup), exist_ok=True)
                shutil.copy2(target, backup)
            replace(target, open(os.path.join(STAGED, "files", rows[rel]["file"]), "rb").read())
        print("applied", sum(1 for v in state.values() if v != "applied"), "files")
        return
    if mode == "revert":
        for rel, value in state.items():
            if value != "applied":
                continue
            target, backup = os.path.join(root, rel), os.path.join(backups, rows[rel]["file"])
            if os.path.exists(backup):
                replace(target, open(backup, "rb").read())
            elif rows[rel]["base"] is None:
                os.remove(target)
        print("reverted")
        return
    sys.exit("usage: stage.py build|check|apply|revert")


main()
