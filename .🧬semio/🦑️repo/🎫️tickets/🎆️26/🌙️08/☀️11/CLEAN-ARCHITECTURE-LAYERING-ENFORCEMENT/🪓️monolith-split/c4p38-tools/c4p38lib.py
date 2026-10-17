"""📚 c4p38 shared helpers: paths, the pre-edit snapshots (`orig/<n>.rs` with SHA256), line multisets and the repo write guard.
Only the seven targets of CENSUS4-PKG-38 and their planned children are ever written to the repo; everything else goes to the scratchpad stage or to `🗑️generated/c4p38-*`."""
import collections, hashlib, json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
TOOLS = F + "/c4p38-tools"
GEN = F + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
WORK = SCRATCH + "/c4p38-work"
STAGE = WORK + "/stage"
BASE_REV = "b84bf526c7ba26f99c4054b47da1c1a4d5064e3d"
sys.path.insert(0, F + "/frs-tools")
sys.path.insert(0, TOOLS)


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


def orig_text(n):
    text = read(f"{TOOLS}/orig/{n}.rs")
    want = {l.split()[1]: l.split()[0] for l in read(f"{TOOLS}/orig/SHA256").splitlines() if l.strip()}
    got = hashlib.sha256(text.encode("utf-8")).hexdigest()
    assert want[f"{n}.rs"] == got, f"orig/{n}.rs does not match its SHA256 record"
    return text


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
    p = os.path.join(GEN, "c4p38-" + name)
    write(p, text)
    return p
