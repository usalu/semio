#!/usr/bin/env python3
"""🧷️ Builds and rebases the staged batch `bootown` (lane installown): the native install owns the PATH repo, the bootstrap launcher installer chains into it.
usage: python3 -I build.py init   --root LIVE --mirror MIRROR     first cut: base = live bytes now, new = the mirror bytes (README: the live bytes with the two sentences added)
       python3 -I build.py rebase --root LIVE                     after live drift: per row three-way merge (git merge-file) of live-now, old base and new; base becomes live-now; a conflict stops with exit 3 and leaves the batch untouched
Streams file by file; stdlib plus `git merge-file`."""
import hashlib, json, os, subprocess, sys, tempfile
HERE = os.path.dirname(os.path.abspath(__file__))
BATCH = os.path.join(HERE, "staged", "bootown")
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/"
ROWS = [
    M + "🚀️launcher/📥️install/🐚️.sh",
    M + "🚀️launcher/📥️install/🔵️.ps1",
    M + "🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts",
    M + "🚀️launcher/🧪️tests/📥️install-contract/🥒️.feature",
    M + "🚀️launcher/README.md",
    M + "💻️client/🔌️mcp/📜️script.ts",
    M + "💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json",
    M + "💻️client/🔌️mcp/📥️installation/🧫️fixtures/🔣️.json",
    M + "💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts",
    M + "💻️client/🔌️mcp/📥️installation/🧪️tests/🥒️.feature",
    M + "🔩️native/🥾️bootstrap/🐚️.sh",
    M + "🔩️native/🥾️bootstrap/🔵️.ps1",
    ".github/actions/repo-launcher/action.yml",
    "🧰️framework/🛍️products/🦑️repo/README.md",
]
README = "🧰️framework/🛍️products/🦑️repo/README.md"
README_PATCHES = [
    ("builds the launcher from the checkout. On a bare machine the existing bootstrap `setup` calls it and installs Node, rustup and uv; nothing installs Bun or Go by hand.",
     "builds the launcher from the checkout and then runs `repo run repo-mcp:install` with it, which installs the native `repo` command as the only `repo` on the `PATH` (`--launcher-only`, on Windows `-LauncherOnly`, stops after the launcher; the launcher directory is never on the `PATH`). On a bare machine the existing bootstrap `setup` calls it with `--launcher-only` and installs Node, rustup and uv; nothing installs Bun or Go by hand."),
    ("und baut den Launcher aus dem Checkout. Auf einem leeren Rechner ruft das vorhandene Bootstrap `setup` ihn auf und installiert Node, rustup und uv; nichts installiert Bun oder Go von Hand.",
     "und baut den Launcher aus dem Checkout und führt danach damit `repo run repo-mcp:install` aus, das den nativen Befehl `repo` als einziges `repo` im `PATH` installiert (`--launcher-only`, unter Windows `-LauncherOnly`, hört nach dem Launcher auf; der Launcher-Ordner liegt nie im `PATH`). Auf einem leeren Rechner ruft das vorhandene Bootstrap `setup` ihn mit `--launcher-only` auf und installiert Node, rustup und uv; nichts installiert Bun oder Go von Hand."),
]
def sha(data): return hashlib.sha256(data).hexdigest() if data is not None else None
def read(path):
    return open(path, "rb").read() if os.path.exists(path) else None
def key(index, rel): return "%03d%s" % (index, os.path.splitext(rel)[1])
def argument(name):
    return os.path.abspath(sys.argv[sys.argv.index(name) + 1]) if name in sys.argv else None
def write(manifest, bases, news):
    for sub in ("files", "base"):
        os.makedirs(os.path.join(BATCH, sub), exist_ok=True)
        for old in os.listdir(os.path.join(BATCH, sub)): os.remove(os.path.join(BATCH, sub, old))
    for index, rel in enumerate(ROWS):
        name = manifest[rel]["file"]
        open(os.path.join(BATCH, "files", name), "wb").write(news[rel])
        if bases[rel] is not None: open(os.path.join(BATCH, "base", name), "wb").write(bases[rel])
    json.dump(manifest, open(os.path.join(BATCH, "manifest.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
def init(root, mirror):
    manifest, bases, news = {}, {}, {}
    for index, rel in enumerate(ROWS):
        bases[rel] = read(os.path.join(root, rel))
        if rel == README:
            text = bases[rel].decode("utf8")
            for old, new in README_PATCHES:
                assert text.count(old) == 1, old[:60]
                text = text.replace(old, new)
            news[rel] = text.encode("utf8")
        else: news[rel] = read(os.path.join(mirror, rel))
        assert news[rel] is not None and bases[rel] is not None, rel
        manifest[rel] = {"file": key(index, rel), "base": sha(bases[rel]), "new": sha(news[rel])}
    write(manifest, bases, news)
    print("built", len(ROWS), "rows")
def rebase(root):
    manifest = json.load(open(os.path.join(BATCH, "manifest.json"), encoding="utf8"))
    bases, news, changed = {}, {}, 0
    for rel in ROWS:
        row = manifest[rel]
        old_base = read(os.path.join(BATCH, "base", row["file"])); mine = read(os.path.join(BATCH, "files", row["file"]))
        live = read(os.path.join(root, rel))
        if sha(live) == row["base"] or sha(live) == row["new"]: bases[rel], news[rel] = old_base, mine; continue
        with tempfile.TemporaryDirectory() as scratch:
            paths = []
            for name, data in (("live", live), ("base", old_base), ("mine", mine)):
                paths.append(os.path.join(scratch, name)); open(paths[-1], "wb").write(data)
            merged = subprocess.run(["git", "merge-file", "-p"] + paths, capture_output=True)
        if merged.returncode != 0: print("CONFLICT", rel[-80:], merged.returncode); sys.exit(3)
        bases[rel], news[rel] = live, merged.stdout; changed += 1; print("rebased", rel[-80:])
    for index, rel in enumerate(ROWS):
        manifest[rel] = {"file": manifest[rel]["file"], "base": sha(bases[rel]), "new": sha(news[rel])}
    write(manifest, bases, news)
    print("rebased rows:", changed)
if sys.argv[1] == "init": init(argument("--root"), argument("--mirror"))
elif sys.argv[1] == "rebase": rebase(argument("--root"))
else: sys.exit("verb: init|rebase")
