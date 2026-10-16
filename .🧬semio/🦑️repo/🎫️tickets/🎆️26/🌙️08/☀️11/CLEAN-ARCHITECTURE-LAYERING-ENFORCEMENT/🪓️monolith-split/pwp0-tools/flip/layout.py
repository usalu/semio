# 🧱️ Final crate layouts derived from the staged tree: which files a crate owns, which of them are its entry files (mounted from the crate root, flattened), and the module path of every definition.
import os, re, collections
from common import *
from resolve import World
import model

pt = model.pt


EXTERNAL_ROOTS = ["🏛️ownership/🧱️owner/🦀️.rs", "🏇️mounted-owner/🦀️.rs", "🧬️guest-contract/🦀️.rs", "🧠️interpreter/🦀️.rs", "🧠️interpreter/🏃️runtime/🦀️.rs", "🖥️host/🦀️.rs", "🖨️describe/🏷️component-receipt/🧬️schema/🦀️.rs", "🖨️describe/🛂️descriptor-emission/🦀️.rs"]
_EXT = {}


def external_files(w):
    key = id(w)
    if key not in _EXT:
        s = set()
        for r in EXTERNAL_ROOTS:
            if w.T.exists(r):
                s |= set(w.T.tree(r))
        _EXT[key] = s
    return _EXT[key]


def slug(rel):
    seg = rel.split("/")
    base = seg[-2] if seg[-1] == "🦀️.rs" else seg[-1]
    s = re.sub(r"[^A-Za-z0-9]+", "_", base.encode("ascii", "ignore").decode()).strip("_").lower()
    return s or "piece"


class Layout:
    def __init__(self, w, crates):
        self.w = w
        self.crates = crates
        self.owner = {}
        self.entries = {}
        self.parent_in = {}
        self.mname = {}
        self.flat = {}
        for k in crates:
            self.entries[k] = []
        files = [f for f in w.files if f.endswith(".rs") and f not in external_files(w)]
        by = collections.defaultdict(list)
        for f in files:
            c = model.file_owner(f)
            self.owner[f] = c
            if c in crates:
                by[c].append(f)
        self.by = by
        for k in crates:
            root = CRATES[k]["root"] + "/🦀️.rs"
            owned = set(by[k])
            mounted_from_owned = {}
            for f in sorted(owned):
                for name, child, m in w.T.children(f):
                    if child in owned and name != "<include>" and child != f:
                        mounted_from_owned.setdefault(child, (f, name, m))
                    elif child in owned and name == "<include>":
                        mounted_from_owned.setdefault(child, (f, "<include>", None))
            self.parent_in[k] = mounted_from_owned
            ent = [f for f in sorted(owned) if f not in mounted_from_owned and f != root and not self._is_test(f)]
            self.entries[k] = ent
            self.root = getattr(self, "root", {})
            self.root[k] = root

    @staticmethod
    def _is_test(f):
        return "/🧪️tests/" in f or f.startswith("🧪️tests/")

    def modpath(self, k, f):
        """module path of file f inside crate k (None when f is not owned)"""
        root = self.root[k]
        if f == root:
            return []
        if f in self.entries[k]:
            return []
        pm = self.parent_in[k].get(f)
        if not pm:
            return None
        parent, name, _ = pm
        pp = self.modpath(k, parent)
        if pp is None:
            return None
        if name == "<include>":
            return pp
        for u in self.w.T.info(parent).uses:
            if u["glob"] and u["path"] == [name] and u["vis"].startswith("pub"):
                return pp
        return pp + [name]


if __name__ == "__main__":
    import sys
    w = World()
    keys = sys.argv[1:] or [k for k in CRATES if CRATES[k]["kind"] == "lower"]
    L = Layout(w, set(keys))
    for k in keys:
        print(k, "owned", len(L.by[k]), "entries", [e.split("/")[-2] for e in L.entries[k]][:12])
