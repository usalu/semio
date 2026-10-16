"""📦️ Generates the facade root: explicit re-exports of the whole pre-split public surface from the carved crates."""
import collections, re, sys, os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rlib import *
import rcrates as C
import surface

OVERRIDES_ROOT = {"read_varint_u64": "wire-core", "write_varint_u64": "wire-core"}

DOC = ("//! 📦️ Package glue — wiring only. Domain lives at the carved replication crates.\n"
       "//!\n"
       "//! The crate's `[lib] name` is `protocol`: every replica, authority and plugin crate speaks the\n"
       "//! replication contract through that one canonical name. Each path below re-exports exactly what the\n"
       "//! pre-split crate exported, from the crate that owns it.\n\n")

def def_cfg(r):
    if r[0] != "def": return ""
    for a in r[1].defs[r[2]]["attrs"]:
        if re.search(r"cfg\s*\(", a):
            return re.sub(r"\s+", " ", a.strip())
    return ""

def cfg_of(node, name):
    for pub, leaves, it in node.uses:
        for lf in leaves:
            if usetree.local_name(lf) == name and not lf[2]:
                for a in it["attrs"]:
                    if "cfg" in a: return a.strip()
    return ""

class Facade:
    def __init__(self, model):
        self.model = model
        self.W = model.old
        self.tab = surface.surface(self.W)
        self.mods = surface.public_modules(self.W)
        self.canon = {}
        for path in sorted(self.mods, key=lambda p: (len(p), p)):
            self.canon.setdefault(id(self.mods[path]), path)

    def render_module(self, path, indent, out):
        node = self.mods[path]
        ent = self.tab[path]
        groups = collections.OrderedDict()
        lines = []
        childmods = []
        for name in sorted(ent):
            r = ent[name]
            if r[0] == "mod":
                target = self.canon[id(r[1])]
                if target == path + (name,):
                    childmods.append(name)
                else:
                    tp = "crate::" + "::".join(target)
                    lines.append(indent + "pub use " + tp + ("" if target[-1] == name else " as " + name) + ";")
                continue
            if r[0] == "ext":
                cfg = cfg_of(node, name)
                last = r[1][-1]
                lines.append((indent + cfg + "\n" if cfg else "") + indent + "pub use " + "::".join(r[1]) + ("" if last == name else " as " + name) + ";")
                continue
            if path == () and name in OVERRIDES_ROOT:
                p = self.model.def_ref_in(OVERRIDES_ROOT[name], name, None)
            else:
                p = self.model.def_ref(r[1], r[2], None)
            prefix, _, leaf = p.rpartition("::")
            groups.setdefault((prefix, def_cfg(r)), []).append(leaf + ("" if r[2] == name else " as " + name))
        for (prefix, cfg), items_ in groups.items():
            items_ = sorted(items_)
            if cfg: out.append(indent + cfg)
            out.append(indent + "pub use " + prefix + "::" + (items_[0] if len(items_) == 1 else "{" + ", ".join(items_) + "}") + ";")
        out.extend(lines)
        for name in childmods:
            out.append("")
            out.append(indent + "pub mod " + name + " {")
            self.render_module(path + (name,), indent + "    ", out)
            out.append(indent + "}")

    def render(self):
        out = []
        self.render_module((), "", out)
        return DOC + "pub use semio_framework_value::*;\n\n" + "\n".join(out) + "\n"

    def conformance(self):
        """One private module per public path, each importing every name that path exposed before the split."""
        lines = []
        for path in sorted(self.tab, key=lambda p: (len(p), p)):
            if not self.tab[path]: continue
            prefix = "crate" + ("::" + "::".join(path) if path else "")
            mod = "root" if not path else "_".join(path)
            by_cfg = collections.OrderedDict()
            for n in sorted(self.tab[path]):
                r = self.tab[path][n]
                cfg = def_cfg(r) if r[0] == "def" else cfg_of(self.mods[path], n)
                by_cfg.setdefault(cfg, []).append(n)
            body = "".join("    " + (c + "\n    " if c else "") + "use " + prefix + "::{" + ", ".join(ns) + "};\n" for c, ns in by_cfg.items())
            lines.append("mod " + mod + " {\n    #![allow(unused_imports)]\n" + body + "}")
        return lines
