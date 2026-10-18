"""✅ pai verification: lossless reconstruction of the snapshot from the live (or staged) root + children, forwarder shape and structure checks.
usage: verify.py stage|live"""
import os, re, sys, json, subprocess, collections
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE); sys.path.insert(0, HERE + "/../rf8-tools")
from methods import parse, split_sig, params
from spec import CHILDREN, ROOT, IMPL_HEADER
from rslex import lex
REPO = "/Users/ueli/Documents/semio"
mode = sys.argv[1]
base = ("/Users/ueli/.cache/pai/stage/" if mode == "stage" else REPO + "/") + ROOT
bdir = os.path.dirname(base)
orig = open(HERE + "/pai.orig.rs", encoding="utf-8").read()
OL, oa, ob, oms = parse(orig)
oby = {m["name"]: m for m in oms}
root = open(base, encoding="utf-8").read()
problems = []
def bad(msg):
    problems.append(msg); print("PROBLEM", msg)
RL = root.split("\n")
mounts = []
for c in CHILDREN:
    mounts += ['#[path = "%s/🦀️.rs"]' % c["folder"], "mod %s;" % c["mod"], ""]
i = next(k for k, l in enumerate(RL) if l == mounts[0])
if RL[i:i + len(mounts) - 1] != mounts[:-1] or RL[i - 1] != "":
    bad("mount block not contiguous after a blank line")
else:
    del RL[i - 1:i + len(mounts) - 1]
RL = "\n".join(RL).split("\n")
blocks = {}
for c in CHILDREN:
    t = open(os.path.join(bdir, c["folder"], "🦀️.rs"), encoding="utf-8").read()
    L = t.split("\n")
    if L[0] != c["doc"] or L[1] != "" or L[2] != "use super::*;" or L[3] != "" or L[4] != IMPL_HEADER or L[-2] != "}" or L[-1] != "":
        bad("child frame " + c["folder"])
    cl, ca, cb, cms = parse(t, "VcsArtifactApp<A, M> {")
    got = [m["name"] for m in cms]
    if got != c["methods"]:
        bad("child methods %s: %s" % (c["folder"], got))
    for m in cms:
        blk = cl[m["s"]: m["e"] + 1]
        h = m["h"] - m["s"]
        if not blk[h].startswith("    pub(super) "):
            bad("missing pub(super) " + m["name"])
        blk[h] = "    " + blk[h][len("    pub(super) "):]
        blocks[m["name"]] = blk
    inner = L[5:-2]
    exp = []
    for n in c["methods"]:
        if exp: exp.append("")
        exp += blocks[n] if n in blocks else []
    expected = ["    pub(super) " + x[4:] if (k == 0 and False) else x for k, x in enumerate(exp)]
    if len(inner) != len(exp):
        bad("child inner length " + c["folder"])
rl, ra, rb, rms = parse("\n".join(RL))
fwd = {m["name"]: m for m in rms if m["name"] in blocks}
if set(fwd) != set(blocks):
    bad("forwarders vs moved: %s" % (set(blocks) ^ set(fwd)))
out = []
k = 0
heads = sorted((m["s"], m["e"], m["name"]) for m in rms if m["name"] in blocks)
pos = 0
for s, e, n in heads:
    out += rl[pos:s]
    f = rl[s:e + 1]
    if len(f) != 1:
        bad("forwarder not one line " + n)
    oh = split_sig(OL, oby[n])
    fsig = split_sig(rl, [m for m in rms if m["name"] == n][0])
    norm = lambda x: re.sub(r"\s+", "", re.sub(r"(?<=[(,])(\s*)mut (\w+)\s*:", lambda y: y.group(1) + y.group(2) + ":", x))
    if norm(oh[0]) != norm(fsig[0]):
        bad("forwarder signature differs " + n)
    callargs = re.search(r"\{\s*self\.%s\((.*)\)(\.await)?\s*\}\s*$" % n, f[0])
    if not callargs:
        bad("forwarder call shape " + n)
    else:
        a = [x.strip() for x in callargs.group(1).split(",") if x.strip()]
        if a != params(oh[2]):
            bad("forwarder args " + n)
        if bool(callargs.group(2)) != oby[n]["is_async"]:
            bad("forwarder await " + n)
    out += blocks[n]
    pos = e + 1
out += rl[pos:]
if "\n".join(out) != orig:
    import difflib
    d = list(difflib.unified_diff(orig.split("\n"), out, lineterm="", n=0))
    bad("reconstruction differs from snapshot: %d diff lines, first: %s" % (len(d), d[:6]))
else:
    print("OK reconstruction equals snapshot byte for byte (%d moved methods, %d lines)" % (len(blocks), orig.count("\n") + 1))
for c in CHILDREN:
    p = os.path.join(bdir, c["folder"], "🦀️.rs")
    pr = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", p], capture_output=True, text=True)
    if pr.returncode: bad("rustfmt parse " + c["folder"] + pr.stderr[:100])
pr = subprocess.run(["rustfmt", "+nightly-2026-07-20", "--edition", "2021", "--emit", "stdout", "--config", "skip_children=true", base], capture_output=True, text=True)
if pr.returncode: bad("rustfmt parse root" + pr.stderr[:100])
for p in [base] + [os.path.join(bdir, c["folder"], "🦀️.rs") for c in CHILDREN]:
    t = open(p, encoding="utf-8").read()
    d, lit = lex(t)
    if "[DEBUG]" in t: bad("[DEBUG] in " + p)
    rel = os.path.relpath(p, "/Users/ueli/.cache/pai/stage" if mode == "stage" else REPO)
    if len(rel.encode()) > 240 or len(os.path.join(REPO, rel)) > 256: bad("path too long " + rel)
    print("lines", t.count("\n") + (0 if t.endswith("\n") else 1), "bytes", len(t.encode()), rel.split("/")[-2])
print("PROBLEMS", len(problems))
sys.exit(1 if problems else 0)
