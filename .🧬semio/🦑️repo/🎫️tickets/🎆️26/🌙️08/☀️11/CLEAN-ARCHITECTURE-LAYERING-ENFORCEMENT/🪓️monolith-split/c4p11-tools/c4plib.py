"""📚 c4p11 shared helpers: paths, baseline access (byte copies of the eight live files taken before the first edit, `scratchpad/c4p11-work/orig/<key>.rs`), the prepared text (baseline plus the declared function extractions), line multisets, and the repo write guard.
Only files named in a plan (the targets and their new children) are ever written to the repo; everything else goes to the scratchpad stage or to `🗑️generated/c4p11-*`."""
import collections, hashlib, json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
WORK = SCRATCH + "/c4p11-work"
ORIG = WORK + "/orig"
STAGE = WORK + "/stage"
PREP = WORK + "/prep"
BASE_REV = "b84bf526c7ba26f99c4054b47da1c1a4d5064e3d"
NORM = "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any"
SCHEMA = f"{ANY}/🧬️schema"
INF = f"{SCHEMA}/💡️inferences"
KEYS = {
    f"{NORM}/⚡️din18599/{INF}/⚖️checks/🦀️.rs": "dsn",
    f"{NORM}/🌍️en1997/{INF}/⚖️checks/🦀️.rs": "e97",
    f"{NORM}/🌍️en1997/{SCHEMA}/🧪️tests/⚖️compliance/🦀️.rs": "e97t",
    f"{NORM}/🌬️din16798/{INF}/⚖️checks/🦀️.rs": "d16",
    f"{NORM}/🏛️en1992/{INF}/⚖️checks/🦀️.rs": "e92",
    f"{NORM}/🏭️vdi3805/{INF}/📚️document-checks/🦀️.rs": "vdc",
    f"{NORM}/🏭️vdi3805/{INF}/🧱️sheet-checks/🦀️.rs": "vds",
    f"{NORM}/📇️iso16757/{INF}/⚖️checks/📈️part1/🦀️.rs": "iso",
}
sys.path.insert(0, F + "/frs-tools")
sys.path.insert(0, F + "/c4p11-tools")


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


def apply_extractions(ls, extracts):
    """Each extraction cuts the 1-based inclusive line range `cut` out of its host function, leaves `call` lines in its place and appends a new function (`head` lines, the cut lines verbatim, `}`) after the host's closing line `host_end`."""
    ls = list(ls)
    for e in sorted(extracts, key=lambda e: -e["cut"][0]):
        a, b = e["cut"]
        moved = ls[a - 1:b]
        new_fn = [""] + e["head"] + moved + ["}"]
        he = e["host_end"]
        assert he >= b, ("host_end before cut end", e["name"])
        ls = ls[:a - 1] + e["call"] + ls[b:he] + new_fn + ls[he:]
    return ls


def prep_lines(rel, spec):
    ls = lines_of(base_text(rel))
    return apply_extractions(ls, spec.get("extract", []))


def repo_write(path, text, allowed):
    real = os.path.realpath(os.path.dirname(os.path.abspath(path)))
    assert real.startswith(os.path.realpath(ROOT) + os.sep), path
    assert os.path.abspath(path) in allowed, f"not a planned file: {path}"
    write(path, text)


def log(name, text):
    p = os.path.join(GEN, "c4p11-" + name)
    write(p, text)
    return p
