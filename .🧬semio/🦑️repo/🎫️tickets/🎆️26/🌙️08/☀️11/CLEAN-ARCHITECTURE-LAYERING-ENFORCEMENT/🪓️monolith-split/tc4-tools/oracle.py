"""🔬️ Scoped full-resolution differential (RESOLUTION-ORACLE.md) of the generation3d-app-laws split: the baseline sandbox restores the snapshot (tc4-tools/orig) of the unsplit target, the current sandbox copies the six new roots; both are patched identically (extern crate lines removed, external glob imports neutralised) and compiled with `rustc --test --emit=metadata` into the sandbox only. Rows are (error text with backticks masked, primary span file without line numbers). usage: oracle.py"""
import collections, os, re, shutil, subprocess, sys
R = "/Users/ueli/Documents/semio/"
HERE = os.path.dirname(os.path.abspath(__file__))
GEN = os.path.realpath(HERE + "/../🗑️generated") + "/"
SB = "/Users/ueli/.cache/tc4/sb"
REL = "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/"
MAN = "✏️s/🧑‍💻dev/🧩️composition/📦️packages/🦀️rust"
OLDROOT = "🦀️.rs"
NEWROOTS = ["🔭️viewer-laws", "🪟️editor-mode-laws", "🧭️editor-structure-laws", "🎮️editor-command-laws", "🧮️editor-unit-laws", "🕹️editor-surface-laws"]
EXTERNAL_GLOB = re.compile(r"^(\s*)use (?!crate::|super::|self::|std::|core::|alloc::)([A-Za-z_][A-Za-z0-9_]*)::(?:[A-Za-z0-9_]+::)*\*;", re.M)

def under(path, root):
    assert os.path.realpath(path).startswith(os.path.realpath(root) + os.sep) or os.path.realpath(path) == os.path.realpath(root), path

def patch(text):
    text = re.sub(r"^extern crate [^;]*;\n", "", text, flags=re.M)
    return EXTERNAL_GLOB.sub(lambda m: m.group(1) + "const _: () = ();", text)

def build(name, source):
    dst = f"{SB}/{name}"
    if os.path.exists(dst): shutil.rmtree(dst)
    os.makedirs(dst + "/" + REL)
    os.makedirs(dst + "/" + MAN)
    for link in ("✏️s/🔌️plugins", "🧰️framework"):
        os.makedirs(os.path.dirname(f"{dst}/{link}"), exist_ok=True)
        os.symlink(R + link, f"{dst}/{link}")
    for rel, text in source:
        path = f"{dst}/{REL}{rel}"
        under(path, dst)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf8").write(patch(text) if rel.endswith(".rs") else text)
    return dst

def base_source():
    out = []
    for line in open(HERE + "/orig/index.tsv", encoding="utf8"):
        key, _, path = line.rstrip("\n").split("\t")
        if (R + path).startswith(R + REL) and path.endswith(".rs"):
            out.append((path[len(REL):], open(HERE + "/orig/" + key, encoding="utf8").read()))
    return out

def cur_source():
    out = []
    root = R + REL
    for dp, dn, fn in os.walk(root):
        for f in fn:
            if f.endswith(".rs"):
                p = os.path.join(dp, f)
                out.append((p[len(root):], open(p, encoding="utf8").read()))
    return out

def run(dst, root_rel):
    out = f"{SB}/out"
    os.makedirs(out, exist_ok=True)
    cmd = ["nice", "-n", "10", "rustc", "+nightly-2026-07-20", "--edition", "2021", "--test", "--crate-name", "laws", "--emit=metadata", "-o", f"{out}/x.rmeta", f"{dst}/{REL}{root_rel}"]
    p = subprocess.run(cmd, capture_output=True, text=True, env=dict(os.environ, CARGO_MANIFEST_DIR=f"{dst}/{MAN}"))
    shutil.rmtree(out)
    return p.stderr

def norm(err, dst):
    rows = collections.Counter()
    lines = err.splitlines()
    for i, l in enumerate(lines):
        if l.startswith("error") and "aborting due to" not in l:
            span = ""
            for j in range(i + 1, min(i + 4, len(lines))):
                m = re.match(r"\s*--> (.*?):\d+:\d+", lines[j])
                if m: span = os.path.normpath(m.group(1)).replace(dst + "/", "").replace(REL, ""); break
            rows[(re.sub(r"`[^`]*`", "`_`", l), span)] += 1
    return rows

base = build("base", base_source()); cur = build("cur", cur_source())
eb = run(base, OLDROOT)
open(GEN + "tc4-oracle-base.err", "w").write(eb)
nb = norm(eb, base)
allnew = collections.Counter(); total_cur = 0
for r in NEWROOTS:
    ec = run(cur, r + "/🦀️.rs")
    open(GEN + f"tc4-oracle-cur-{r}.err", "w").write(ec)
    nc = norm(ec, cur)
    total_cur += sum(nc.values())
    for k, v in nc.items(): allnew[k] = max(allnew[k], v)
    print(r, "errors", sum(nc.values()), "distinct", len(nc))
new = {k for k in allnew if k not in nb}
gone = {k for k in nb if k not in allnew}
print("base errors", sum(nb.values()), "distinct", len(nb), "| current (6 roots) errors", total_cur, "distinct", len(allnew))
for (k, s) in sorted(new): print("  NEW", k[:150], "|", s[-100:])
for (k, s) in sorted(gone): print("  GONE", k[:150], "|", s[-100:])
