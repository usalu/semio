"""Shared helpers of the frs (RUST-FILE-SPLITS-8) tools: paths, coverage-checked planning, staging, multiset proofs."""
import collections, hashlib, json, os, re, shutil, sys

ROOT = "/Users/ueli/Documents/semio"
F = ROOT + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
W = F + "/🗑️generated/rf8-work"

R = "🧰️framework/🛍️products/🦑️repo/🔨️modules"
G = "✏️s/🧑‍💻dev/🧩️composition/🧪️tests"
S = "✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session"


def _t(file, tree, crate, cfgs, features=(), lib=None, ext="auto", extra_stub=""):
    return dict(file=file, tree=tree, lib=lib or file, crate=crate, pkg=crate.replace("_", "-"), cfgs=list(cfgs), features=list(features), ext=ext, extra_stub=extra_stub)


TARGETS = {
    "statutes": _t(R + "/📜️statutes/📦️packages/🦀️rust/🦀️.rs", R + "/📜️statutes", "semio_framework_repo_statutes", ["lib", "test"]),
    "verbs": _t(R + "/⌨️cli/🧭️repo-cli/🚦️verbs/🦀️.rs", R + "/⌨️cli/🧭️repo-cli/🚦️verbs", "semio_framework_repo_cli_verbs", ["lib", "test"]),
    "events": _t(R + "/📡️events/📦️packages/🦀️rust/🦀️.rs", R + "/📡️events", "semio_framework_repo_events", ["lib", "test"]),
    "metrics": _t(R + "/📊️metrics/📦️packages/🦀️rust/🦀️.rs", R + "/📊️metrics", "semio_framework_repo_metrics", ["lib", "test"]),
    "move": _t(R + "/🚚️move/📦️packages/🦀️rust/🦀️.rs", R + "/🚚️move", "semio_framework_repo_move", ["lib", "test"]),
    "g3root": dict(_t(G + "/🧊️generation3d/🦀️.rs", G + "/🧊️generation3d", "generation3d", ["test"]), ext_extra=["dsl", "protocol", "store"]),
    "g3geom": _t(G + "/🧩️generation3d-example-geometry/🦀️.rs", G + "/🧩️generation3d-example-geometry", "generation3d_example_geometry", ["test"]),
    "g3unit": dict(_t(G + "/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs", G + "/🧊️generation3d", "generation3d", ["test"], lib=G + "/🧊️generation3d/🦀️.rs"), fam="g3root", ext_extra=["dsl", "protocol", "store"]),
    "session": _t(S + "/🦀️.rs", S, "semio_s_spatial_kernel_semio_session", ["lib", "test", "wasm", "wasmtest"]),
}
G3UNIT = G + "/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs"


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
    """Lines without terminators; a trailing newline yields no extra empty element."""
    ls = text.split("\n")
    if ls and ls[-1] == "":
        ls.pop()
    return ls


def dedent(ls, n=4):
    out = []
    for l in ls:
        k = 0
        while k < n and k < len(l) and l[k] == " ":
            k += 1
        out.append(l[k:])
    return out


class Coverage:
    """Every original line (1-based) must be claimed exactly once."""

    def __init__(self, total):
        self.total = total
        self.owner = [None] * (total + 1)

    def claim(self, who, a, b):
        for i in range(a, b + 1):
            assert self.owner[i] is None, f"line {i} claimed by {self.owner[i]} and {who}"
            self.owner[i] = who

    def check(self):
        missing = [i for i in range(1, self.total + 1) if self.owner[i] is None]
        assert not missing, f"unclaimed lines {missing[:20]} ({len(missing)})"


def multiset(ls):
    return collections.Counter(l.strip() for l in ls if l.strip() != "")


def diff_multiset(orig_ls, new_ls):
    a, b = multiset(orig_ls), multiset(new_ls)
    return a - b, b - a


def stage_dir(name):
    return f"{W}/{name}/stage"


def fam(name):
    return TARGETS[name].get("fam", name)


def snap_dir(name):
    return f"{W}/{fam(name)}/snap"


AUTO_EXT_RX = re.compile(r"(?<![\w:.])(?:::)?((?:semio_|neural_engine)\w*|serde_json|serde|parry3d|js_sys|wasm_bindgen)(?=\s*::|\s+as\b)")


def auto_ext(name, local_mods=()):
    """External crate names the target's snapshot tree refers to as path heads (`<crate>::`, `<crate> as`)."""
    import sys as _s
    _s.path.insert(0, os.path.dirname(__file__))
    from splitter import crate_files, mask_keep_strings
    from stubgen import mask
    t = TARGETS[name]
    found = set()
    for p in crate_files(snap_dir(name) + "/" + t["lib"]):
        for m in AUTO_EXT_RX.finditer(mask(read(p))):
            found.add(m.group(1))
    return sorted((found | set(t.get("ext_extra", []))) - set(local_mods))
