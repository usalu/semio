"""stage.py <build|apply|revert|check> [--mirror M] [--root R]: stages the purge-g batch `purge` (files in `files-purge.txt`, bytes in `staged/purge/files/fNN`, base bytes in `staged/purge/base/fNN`: flat names because the real paths exceed the path limit inside the ticket).
build: copies the files from the mirror and records base and new sha256 (a new file has base null). apply: all-or-nothing, refuses when a file changed since staging (base-changed). revert: restores the base bytes (or removes a new file) for every file that still equals its new bytes. check: one state per file (applied, pending, base-changed)."""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged", "purge")
MANIFEST = os.path.join(STAGED, "manifest.json")


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.exists(path) else None


def arg(name, default):
    return sys.argv[sys.argv.index(name) + 1] if name in sys.argv else default


def replace(path, data):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    temporary = path + ".purge-tmp"
    with open(temporary, "wb") as handle:
        handle.write(data)
    os.replace(temporary, path)


def build(mirror, root):
    rows = {}
    for index, rel in enumerate(line.strip() for line in open(os.path.join(HERE, "files-purge.txt"), encoding="utf-8").read().split("\n") if line.strip()):
        data, name = open(os.path.join(mirror, rel), "rb").read(), "f%02d" % index
        live = os.path.join(root, rel)
        if os.path.exists(live) and not os.path.exists(os.path.join(STAGED, "base", name)):
            replace(os.path.join(STAGED, "base", name), open(live, "rb").read())
        replace(os.path.join(STAGED, "files", name), data)
        base = hashlib.sha256(open(os.path.join(STAGED, "base", name), "rb").read()).hexdigest() if os.path.exists(os.path.join(STAGED, "base", name)) else None
        rows[rel] = {"file": name, "base": base, "new": hashlib.sha256(data).hexdigest()}
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
            print("REFUSED (a peer edited the file since staging; rebase the batch):\n" + "\n".join("base-changed " + rel for rel in bad))
            sys.exit(2)
        for rel, value in state.items():
            if value != "applied":
                replace(os.path.join(root, rel), open(os.path.join(STAGED, "files", rows[rel]["file"]), "rb").read())
        print("applied", sum(1 for v in state.values() if v != "applied"), "files")
        return
    if mode == "revert":
        for rel, value in state.items():
            if value != "applied":
                continue
            base = os.path.join(STAGED, "base", rows[rel]["file"])
            if rows[rel]["base"] is None:
                os.remove(os.path.join(root, rel))
            else:
                replace(os.path.join(root, rel), open(base, "rb").read())
        print("reverted")
        return
    sys.exit("usage: stage.py build|apply|revert|check")


main()
