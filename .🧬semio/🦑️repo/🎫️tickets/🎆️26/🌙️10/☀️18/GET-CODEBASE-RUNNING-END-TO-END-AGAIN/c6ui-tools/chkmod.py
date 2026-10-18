"""✅ C5B structural check of the path-module splits on the LIVE tree: every child mounted exactly once by `#[path]`, docstring emoji equals the folder emoji and is unique among siblings (existing folders included) and different from the root's, regions balanced per file, brace depth 0, rustfmt parse, path length, no [DEBUG], largest child. usage: chkmod.py <tag>..."""
import importlib, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
sys.path.insert(0, '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/rf8-tools')
from rslex import lex
REPO = "/Users/ueli/Documents/semio"
RG = re.compile(r"^\s*//\s?#region "); ER = re.compile(r"^\s*//\s?#endregion ")
ok = True
for tag in sys.argv[1:]:
    sp = importlib.import_module("specs." + tag).SPEC
    rootrel = sp["file"]; bdir = os.path.dirname(rootrel)
    root = open(os.path.join(REPO, rootrel), encoding="utf-8").read()
    problems = []
    folders = [c["folder"] for c in sp["children"]]
    texts = {f: open(os.path.join(REPO, bdir, f, "🦀️.rs"), encoding="utf-8").read() for f in folders}
    for c in sp["children"]:
        if root.count('#[path = "%s/🦀️.rs"]' % c["folder"]) != 1:
            problems.append("%s: mounted %d times" % (c["folder"], root.count('#[path = "%s/🦀️.rs"]' % c["folder"])))
    _m = re.match(r"^\s*//[!/]*\s*(\S+)", root)
    root_emoji = _m.group(1).replace("\ufe0f", "") if _m else ""
    seen = {}
    for f, t in texts.items():
        emo = f[: f.index("️") + 1]
        m = re.match(r"^//! (\S+) ", t)
        if not m or m.group(1) != emo:
            problems.append("%s: docstring emoji %s != folder emoji" % (f, m.group(1) if m else None))
        if emo in seen:
            problems.append("duplicate emoji %s" % emo)
        seen[emo] = f
        if emo.replace("️", "") == root_emoji:
            problems.append("%s: equals the root emoji" % f)
    for e in os.listdir(os.path.join(REPO, bdir)):
        if e in texts or not os.path.isdir(os.path.join(REPO, bdir, e)):
            continue
        if "️" in e and e[: e.index("️") + 1] in seen:
            problems.append("sibling emoji clash with %s" % e)
    for r, t in [(rootrel, root)] + [(f"{bdir}/{f}/🦀️.rs", t) for f, t in texts.items()]:
        if len(r.encode()) > 240 or len(os.path.join(REPO, r)) > 256:
            problems.append("path too long %s" % r[-60:])
        no = len([1 for l in t.split("\n") if RG.match(l)]); nc = len([1 for l in t.split("\n") if ER.match(l)])
        if no != nc:
            problems.append("%s: regions %d/%d" % (r.split("/")[-2], no, nc))
        d, _ = lex(t)
        if d[-1] != 0:
            problems.append("%s: brace depth %d" % (r.split("/")[-2], d[-1]))
        if "[DEBUG]" in t and "[DEBUG]" not in open(os.path.join(HERE, "orig", sp["orig"]), encoding="utf-8").read():
            problems.append("%s: [DEBUG] present" % r.split("/")[-2])
        pr = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", os.path.join(REPO, r)], capture_output=True, text=True)
        if pr.returncode != 0 and re.search(r"^error(?!\[internal\])", pr.stderr, flags=re.M):
            problems.append("rustfmt parse failed %s: %s" % (r.split("/")[-2], pr.stderr[:100]))
    sizes = {f: len(t.split("\n")) - 1 for f, t in texts.items()}
    print(f"[{tag}] root={len(root.split(chr(10))) - 1} children={len(texts)} max-child={max(sizes.values())} {'OK' if not problems else 'PROBLEMS'}")
    for p in problems:
        print("    PROBLEM", p); ok = False
print("CHKMOD", "OK" if ok else "FAILED")
sys.exit(0 if ok else 1)
