"""📏 C5C formatting-debt proof for module-split tags: rustfmt --check hunk counts (skip_children) of the snapshot versus the live root and children, plus the changed-line multiset (blank-normalised, `pub(super) ` removed). usage: fmt.py <tag>..."""
import collections, importlib, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
REPO = "/Users/ueli/Documents/semio"
CMD = ["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config", "skip_children=true"]


def hunks(path):
    r = subprocess.run(CMD + [path], capture_output=True, text=True)
    bad = bool(re.search(r"^error(?!\[internal\])", r.stderr, flags=re.M)) and r.returncode not in (0, 1)
    lines = collections.Counter()
    for l in r.stdout.split("\n"):
        if l.startswith(("-", "+")) and not l.startswith(("---", "+++")):
            k = re.sub(r"\s+", "", l[1:].replace("pub(super) ", "").replace("pub(in super::super) ", "").replace("../", ""))
            if k:
                lines[l[0] + k] += 1
    return len(re.findall(r"^Diff in ", r.stdout, flags=re.M)), bad, lines


for tag in sys.argv[1:]:
    sp = importlib.import_module("specs." + tag).SPEC
    base = os.path.join(REPO, os.path.dirname(sp["file"]))
    o, ob, ol = hunks(os.path.join(HERE, "orig", sp["orig"]))
    files = [os.path.join(REPO, sp["file"])] + [os.path.join(base, x, "🦀️.rs") for x in __import__("specs.lib", fromlist=["folders_of"]).folders_of(sp)]
    n, nb, nl = 0, False, collections.Counter()
    for f in files:
        h, b, l = hunks(f)
        n += h
        nb = nb or b
        nl += l
    print(f"{tag}: hunks original={o} new={n} parse-failures original={ob} new={nb} files={len(files)} changed-line multiset original={sum(ol.values())} new={sum(nl.values())} identical={ol == nl}")
    for k, v in list((ol - nl).items())[:4]:
        print("   ORIG-ONLY", v, k[:140])
    for k, v in list((nl - ol).items())[:4]:
        print("   NEW-ONLY", v, k[:140])
