#!/usr/bin/env python3
"""🧊 flat.py: converts a staged tree that mirrors repository paths into a flat tree (`manifest.json` + `f<NNN>` files) so no ticket path exceeds the Windows checkout limit (AGENTS.md: paths <= 256 chars).
usage: python3 -I flat.py convert <dir>... | check <dir>... | expand <dir> <dest> | scratch <tree dir> | list <dir>
`scratch` expands <batch>/tree once to $TMPDIR/flat-trees/<lane tools dir>/<sha1(manifest)[:8]>/staged/<batch>/tree (so law files run in place; the staged preloads compute the same path) and prints that tree path.
manifest.json: {"<repo-relative path>": {"file": "f001", "sha256": "<hex of the file bytes>", "mode": 420}, ...}, sorted by path; numbering is dense and stable for the same set of paths (the obs-g / mirror.py row shape).
Tools that read a flat tree inline: `m=json.load(open(T+"/manifest.json",encoding="utf8")); path=lambda rel: T+"/"+m[rel]["file"]`.
exit 0 ok, 1 on a failed check, 2 on usage errors / symlinks / an already flat directory."""
import hashlib, json, os, shutil, sys, tempfile

NAME = "manifest.json"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def tree_files(top):
    found = []
    for d, dirs, names in os.walk(top):
        for n in names:
            p = os.path.join(d, n)
            if os.path.islink(p):
                raise SystemExit("flat: symlink refused " + p)
            found.append(os.path.relpath(p, top).replace(os.sep, "/"))
    return sorted(found)


def convert(top):
    top = top.rstrip("/")
    if os.path.exists(top + "/" + NAME):
        raise SystemExit("flat: already flat " + top)
    rels = tree_files(top)
    manifest, blobs = {}, []
    for i, rel in enumerate(rels, 1):
        data = open(top + "/" + rel, "rb").read()
        key = "f%03d" % i
        manifest[rel] = {"file": key, "sha256": sha(data), "mode": os.stat(top + "/" + rel).st_mode & 0o777}
        blobs.append((key, data, manifest[rel]["mode"]))
    stash = top + ".flat-tmp"
    os.makedirs(stash)
    for key, data, mode in blobs:
        open(stash + "/" + key, "wb").write(data)
        os.chmod(stash + "/" + key, mode)
    with open(stash + "/" + NAME, "w", encoding="utf8") as out:
        json.dump(manifest, out, ensure_ascii=False, indent=1)
        out.write("\n")
    check(stash)
    shutil.rmtree(top)
    os.rename(stash, top)
    print("converted %s: %d files" % (top, len(rels)))


def load(top):
    return json.load(open(top.rstrip("/") + "/" + NAME, encoding="utf8"))


def check(top):
    top = top.rstrip("/")
    manifest, bad = load(top), 0
    for rel, e in manifest.items():
        p = top + "/" + e["file"]
        if not os.path.isfile(p) or sha(open(p, "rb").read()) != e["sha256"]:
            print("BAD " + e["file"] + " " + rel[-60:])
            bad += 1
    extra = sorted(set(os.listdir(top)) - {e["file"] for e in manifest.values()} - {NAME})
    for n in extra:
        print("EXTRA " + n)
    print("%s: %d files, %d bad, %d extra" % (top, len(manifest), bad, len(extra)))
    return 0 if not bad and not extra else 1


def expand(top, dest, quiet=False):
    for rel, e in load(top).items():
        target = os.path.join(dest, *rel.split("/"))
        os.makedirs(os.path.dirname(target), exist_ok=True)
        shutil.copyfile(top.rstrip("/") + "/" + e["file"], target)
        os.chmod(target, e["mode"])
    if not quiet:
        print("expanded %s -> %s" % (top, dest))


def scratch(tree):
    tree = os.path.abspath(tree.rstrip("/"))
    batch = os.path.dirname(tree)
    raw = open(tree + "/" + NAME, "rb").read()
    root = os.path.join(tempfile.gettempdir(), "flat-trees", os.path.basename(os.path.dirname(os.path.dirname(batch))), hashlib.sha1(raw).hexdigest()[:8], "staged", os.path.basename(batch))
    if not os.path.exists(root + "/.expanded"):
        shutil.rmtree(root, ignore_errors=True)
        expand(tree, root + "/tree", True)
        open(root + "/.expanded", "w").close()
    print(root + "/tree")


if __name__ == "__main__":
    a = sys.argv[1:]
    if not a:
        raise SystemExit(__doc__)
    if a[0] == "convert":
        [convert(t) for t in a[1:]]
    elif a[0] == "check":
        sys.exit(max([check(t) for t in a[1:]] or [2]))
    elif a[0] == "expand" and len(a) == 3:
        expand(a[1], a[2])
    elif a[0] == "scratch" and len(a) == 2:
        scratch(a[1])
    elif a[0] == "list":
        [print(e["file"], e["sha256"][:12], rel) for rel, e in load(a[1]).items()]
    else:
        raise SystemExit(__doc__)
