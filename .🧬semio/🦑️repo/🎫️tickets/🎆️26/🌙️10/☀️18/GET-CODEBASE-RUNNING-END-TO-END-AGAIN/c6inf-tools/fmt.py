"""📏 C5B formatting-debt proof: rustfmt --check hunks (skip_children) of the snapshot versus the live root and children of the same module split (changed-line multiset, blank-normalised, `pub(super) ` and `../` removed). usage: fmt.py <tag>..."""
import collections, importlib, os, re, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
REPO = "/Users/ueli/Documents/semio"
CMD = ["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--check", "--config", "skip_children=true"]
def hunks(path):
    r = subprocess.run(CMD + [path], capture_output=True, text=True)
    bad = ("error" in r.stderr.lower() and "Diff in" not in r.stdout and r.returncode not in (0, 1))
    lines = collections.Counter()
    for l in r.stdout.split("\n"):
        if l.startswith(("-", "+")) and not l.startswith(("---", "+++")):
            k = re.sub(r"\s+", "", l[1:].replace("pub(super) ", "").replace("../../", "../").replace("../", ""))
            lines[l[0] + k] += 1
    return len(re.findall(r"^Diff in ", r.stdout, flags=re.M)), bad, lines
for tag in sys.argv[1:]:
    sp = importlib.import_module("specs." + tag).SPEC
    base = os.path.join(REPO, os.path.dirname(sp["file"]))
    otext = open(os.path.join(HERE, "orig", sp["orig"]), encoding="utf-8").read()
    if sp.get("pre"):
        otext = sp["pre"](otext)
    os.makedirs(os.path.join(HERE, "fmt-tmp"), exist_ok=True)
    tmp = os.path.join(HERE, "fmt-tmp", f"{tag}.rs")
    open(tmp, "w", encoding="utf-8").write(otext)
    o, ob, ol = hunks(tmp)
    files = [os.path.join(REPO, sp["file"])] + [os.path.join(base, c["folder"], "🦀️.rs") for c in sp["children"]]
    n, nb, nl = 0, False, collections.Counter()
    for f in files:
        h, b, l = hunks(f)
        n += h; nb = nb or b; nl += l
    print(f"{tag}: hunks original={o} new={n} parse-failures original={ob} new={nb} files={len(files)} changed-line multiset original={sum(ol.values())} new={sum(nl.values())} identical={ol == nl}")
    for k, v in list((ol - nl).items())[:4]:
        print("   ORIG-ONLY", v, k[:140])
    for k, v in list((nl - ol).items())[:4]:
        print("   NEW-ONLY", v, k[:140])
