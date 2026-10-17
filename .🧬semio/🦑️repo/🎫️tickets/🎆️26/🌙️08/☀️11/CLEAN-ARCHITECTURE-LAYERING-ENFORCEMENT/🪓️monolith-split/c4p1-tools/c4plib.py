"""📚 c4p1 shared helpers: paths, baseline access (git blob of the pre-edit revision), line multisets, and the repo write guard.
Only files named in a plan (the nine targets and their new children) are ever written to the repo; everything else goes to the scratchpad stage or to `🗑️generated/c4p1-*`."""
import collections, hashlib, json, os, re, subprocess, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = F + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
WORK = SCRATCH + "/c4p1-work"
STAGE = WORK + "/stage"
BASE_REV = "b84bf526c7ba26f99c4054b47da1c1a4d5064e3d"
INF = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world"
sys.path.insert(0, F + "/frs-tools")
sys.path.insert(0, F + "/c4p1-tools")


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
    p = subprocess.run(["git", "-c", "core.fsmonitor=false", "-c", "core.quotepath=false", "cat-file", "blob", f"{BASE_REV}:{rel}"], cwd=ROOT, capture_output=True)
    assert p.returncode == 0, (rel, p.stderr.decode()[:200])
    return p.stdout.decode("utf-8")


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
    p = os.path.join(GEN, "c4p1-" + name)
    write(p, text)
    return p
