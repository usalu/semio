"""📚 c4p23 shared helpers: paths, baseline access (byte copies of the eight live files taken before the first edit, `🗑️generated/c4p23-orig/<key>.rs`), line multisets, and the repo write guard.
Only files named in a plan (the targets and their new children) are ever written to the repo; everything else goes to the stage dir or to `🗑️generated/c4p23-*`."""
import collections, hashlib, json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
WORK = GEN + "/c4p23-work"
ORIG = GEN + "/c4p23-orig"
STAGE = GEN + "/c4p23-stage"
R = "✏️s/🔌️plugins"
ART = f"{R}/🖨️raster/🗿️artifacts/🖨️raster"
RA = f"{ART}/🏅️standards/🔖️1/🪆️subsets/✳️any"
ED = f"{R}/🗄️stdio/📇️registry/🧬️contract/✏️editing"
KEYS = {
    f"{RA}/✏️editor/🧪️tests/🔬️unit/🦀️.rs": "rtu",
    f"{RA}/🚪️io/🦀️.rs": "rio",
    f"{RA}/🧬️schema/🔺️diff/🦀️.rs": "rdf",
    f"{RA}/🧬️schema/🦀️.rs": "rsc",
    f"{ED}/🦀️.rs": "edr",
    f"{ED}/🩹️patch/🦀️.rs": "edp",
    f"{ED}/🪟️details/🦀️.rs": "edd",
    f"{ED}/🪟️details/🧪️tests/🔬️unit/🦀️.rs": "edu",
}
sys.path.insert(0, F + "/frs-tools")
sys.path.insert(0, F + "/c4p23-tools")


def read(path):
    with open(path, encoding="utf-8") as f:
        return f.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as f:
        f.write(text)


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def lines_of(text):
    ls = text.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def base_text(rel):
    return read(f"{ORIG}/{KEYS[rel]}.rs")


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip() != "")


def diff_multiset(orig_ls, new_ls):
    a, b = multiset(orig_ls), multiset(new_ls)
    return a - b, b - a


def repo_write(path, text, allowed):
    real = os.path.realpath(os.path.dirname(os.path.abspath(path)))
    assert real.startswith(os.path.realpath(ROOT) + os.sep), path
    assert os.path.abspath(path) in allowed, f"not a planned file: {path}"
    write(path, text)


def log(name, text):
    p = os.path.join(GEN, "c4p23-" + name)
    write(p, text)
    return p
