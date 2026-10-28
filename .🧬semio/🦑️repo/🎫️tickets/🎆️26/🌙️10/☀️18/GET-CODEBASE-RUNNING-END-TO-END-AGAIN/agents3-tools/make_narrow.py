"""🧱️ agents3: builds the flat staged batch `staged/narrow/` (manifest {rel:{file,base,new}}, files/, base/, stage.py from the xplat5 mode-keeping template, verify.sh) from the LIVE bytes through `edit_narrow.edits`, so `base` is live as it is now.
Rebuild right before landing when the host-capacity files moved: `python3 -I make_narrow.py`. Writes atomically; refuses when an anchor no longer matches (drift) with the anchor named."""
import hashlib, json, os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(HERE)
LIVE = os.path.abspath(os.path.join(C, *[".."] * 7))
sys.path.insert(0, HERE)
import edit_narrow as E  # noqa: E402

OUT = os.path.join(HERE, "staged", "narrow")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    tmp = OUT + ".new"
    shutil.rmtree(tmp, ignore_errors=True)
    os.makedirs(os.path.join(tmp, "files"))
    os.makedirs(os.path.join(tmp, "base"))
    manifest = {}
    for index, rel in enumerate(E.FILES):
        base = open(os.path.join(LIVE, rel), "rb").read()
        new = E.edits(rel, base.decode("utf8")).encode("utf8")
        name = "%02d%s" % (index, os.path.splitext(rel)[1])
        open(os.path.join(tmp, "base", name), "wb").write(base)
        open(os.path.join(tmp, "files", name), "wb").write(new)
        manifest[rel] = {"file": name, "base": sha(base), "new": sha(new)}
    json.dump(manifest, open(os.path.join(tmp, "manifest.json"), "w", encoding="utf8"), indent=1, ensure_ascii=False)
    shutil.copyfile(os.path.join(C, "xplat5-tools", "stage.template.py"), os.path.join(tmp, "stage.py"))
    shutil.copyfile(os.path.join(HERE, "verify_narrow.sh"), os.path.join(tmp, "verify.sh"))
    shutil.rmtree(OUT, ignore_errors=True)
    os.replace(tmp, OUT)
    for rel, row in manifest.items():
        print("%s %s" % (row["file"], rel[-80:]))


if __name__ == "__main__":
    main()
