"""🧰️ Shared helpers of the RENDERER-TEST-BINS (rtb) tools: repository paths, text IO with pre-edit copies, lexer and item table re-exports, test-file inventory."""
import collections, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from rs import blank, uses, expand_use, item_names
import items as itm

REPO = "/Users/ueli/Documents/semio"
SPLIT = REPO + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
GEN = SPLIT + "/🗑️generated"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad/rtb"
SNAP = SCRATCH + "/snap0"
PRE = SCRATCH + "/pre"
E = REPO + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
T = E + "/🎯️targets/🧊️wgpu"
R = T + "/🧊️renderer"
EL = E + "/🧱️elements"
SH = EL + "/🐚️Shell/🎯️targets/🧊️wgpu"
PFX = "semio_framework_os_renderer_wgpu_"
PKG_PREFIX = "semio-framework-os-renderer-wgpu-"
INPUTS = GEN + "/rtb-inputs-os-renderer-wgpu-lib-test-semio-framework-os-renderer-wgpu.txt"


def read(path):
    with open(path, encoding="utf8") as handle:
        return handle.read()


def write(path, text):
    os.makedirs(os.path.dirname(path) or ".", exist_ok=True)
    with open(path, "w", encoding="utf8") as handle:
        handle.write(text)


def backup(path):
    """Keeps the first pre-edit copy of a repo file in the scratch tree (never overwritten)."""
    rel = os.path.relpath(path, REPO)
    dst = PRE + "/" + rel
    if os.path.exists(path) and not os.path.exists(dst):
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        shutil.copy2(path, dst)


def edit(path, text):
    backup(path)
    write(path, text)


def rel(path):
    return os.path.relpath(path, REPO)


def crate_table():
    out = {}
    for d in sorted(os.listdir(T)):
        mf = os.path.join(T, d, "📦️packages/🦀️rust/Cargo.toml")
        if os.path.isfile(mf):
            m = re.search(r'^name = "' + re.escape(PKG_PREFIX) + r'([a-z0-9-]+)"', read(mf), re.M)
            if m:
                out[m.group(1)] = d
    return out


def lib_name(suffix):
    return PFX + suffix.replace("-", "_")


def tokens(text):
    clean = blank(text)
    out = set()
    for m in re.finditer(r"(?<![A-Za-z0-9_$])([A-Za-z_][A-Za-z0-9_]*)", clean):
        s, e = m.start(), m.end()
        k = s - 1
        while k >= 0 and clean[k] in " \t\n":
            k -= 1
        if k >= 0 and clean[k] == "." and not (k >= 1 and clean[k - 1] == "."):
            continue
        j = e
        while j < len(clean) and clean[j] in " \t":
            j += 1
        if j < len(clean) and clean[j] == ":" and clean[j:j + 2] != "::":
            continue
        if m.group(1) != "_":
            out.add(m.group(1))
    for s in re.finditer(r'"((?:[^"\\\n]|\\.)*)"', text):
        for f in re.finditer(r"\{([A-Za-z_][A-Za-z0-9_]*)(?::[^{}]*)?\}", s.group(1)):
            out.add(f.group(1))
    return out


def test_sources():
    """Repo-relative paths of the test-related source files in the monolith lib-test closure (baseline census inputs)."""
    rows = [l.split("\t") for l in read(INPUTS).strip().split("\n")]
    return [r[3] for r in rows if r[0] == "source"]
