"""🔄️ quietfc-g: re-stages files of the flat batch. Usage: python3 -I restage.py rebase|pull <mirror-root> [rel...] — `rebase` sets `base` to the current live sha256 of every row whose live file changed and whose staged file was already merged by hand (named rels), `pull` copies the named files from a mirror root into `staged/files` and refreshes their `new` sha256; with no rel, `pull` takes every row whose mirror file differs from the staged one and adds rows for new files."""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
STAGED = os.path.join(HERE, "staged")
MANIFEST = os.path.join(STAGED, "manifest.json")
REPO = "/Users/ueli/Documents/semio"


def digest(path):
    return hashlib.sha256(open(path, "rb").read()).hexdigest() if os.path.isfile(path) else None


def main():
    mode, mirror, names = sys.argv[1], sys.argv[2], sys.argv[3:]
    rows = json.load(open(MANIFEST, encoding="utf-8"))
    if mode == "rebase":
        for rel in names:
            rows[rel]["base"] = digest(os.path.join(REPO, rel))
    elif mode == "pull":
        for rel in names or list(rows):
            source = os.path.join(mirror, rel)
            data = open(source, "rb").read()
            if rel not in rows:
                rows[rel] = {"file": "f%03d" % (1 + max(int(row["file"][1:]) for row in rows.values())), "base": digest(os.path.join(REPO, rel)), "new": None}
            target = os.path.join(STAGED, "files", rows[rel]["file"])
            if digest(source) != digest(target):
                open(target, "wb").write(data)
                print("pulled", rows[rel]["file"], rel[-60:])
            rows[rel]["new"] = digest(source)
    else:
        sys.exit("usage")
    json.dump(rows, open(MANIFEST, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


main()
