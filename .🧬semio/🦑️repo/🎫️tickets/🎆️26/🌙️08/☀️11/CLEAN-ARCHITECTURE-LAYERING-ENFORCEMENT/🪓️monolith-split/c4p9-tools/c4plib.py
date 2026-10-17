"""📚 c4p9 shared helpers: paths, baseline access (byte copies of the nine live files taken before the first edit, `scratchpad/c4p9-work/orig/<key>.rs`), line multisets, and the repo write guard.
Only files named in a plan (the targets and their new children) are ever written to the repo; everything else goes to the scratchpad stage or to `🗑️generated/c4p9-*`."""
import collections, hashlib, json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
WORK = SCRATCH + "/c4p9-work"
ORIG = WORK + "/orig"
STAGE = WORK + "/stage"
BASE_REV = "b84bf526c7ba26f99c4054b47da1c1a4d5064e3d"
ANY = "✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = f"{ANY}/🧬️schema"
REGS = f"{SCHEMA}/🗄️registers"
P3D = "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
KEYS = {
    f"{SCHEMA}/💡️inferences/🦀️.rs": "inf",
    f"{REGS}/🌐️context-requirements/🦀️.rs": "ctx",
    f"{REGS}/📋️decisions/🦀️.rs": "dec",
    f"{REGS}/🔁️operations-requirements/🦀️.rs": "ops",
    f"{REGS}/🗂️catalog/🦀️.rs": "cat",
    f"{REGS}/🗃️records/🦀️.rs": "rec",
    f"{REGS}/🧭️program-structure/🦀️.rs": "str",
    f"{P3D}/🦀️.rs": "p3d",
}
sys.path.insert(0, F + "/frs-tools")
sys.path.insert(0, F + "/c4p9-tools")


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
    p = os.path.join(GEN, "c4p9-" + name)
    write(p, text)
    return p
