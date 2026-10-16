# 🏗️ Flip build: turns a set of staged crates into real crates inside a sandbox copy of the plugin tree (IN) and writes the result into OUT (mirror of the plugin folder). Never touches the repository.
# usage: fb.py <in-plugin-dir> <out-dir> <real-before,comma> <keys,comma>
import os, re, sys, json, collections, shutil
from common import *
from rs import blank, expand_use
from resolve import World
from layout import Layout, slug, external_files
from engine2 import Engine, expand_group_uses, toks_used, local_names, SKIP_TOK, EXT_ALIASES
import edits as ED
import model

pt = model.pt
RANK = {k: v["rank"] for k, v in CRATES.items()}
LIBKEY = {PFX + k.replace("-", "_"): k for k in CRATES}


def libname(k):
    return PFX + k.replace("-", "_")


def is_test_rel(f):
    return "/🧪️tests/" in f or f.startswith("🧪️tests/")


class FB:
    def __init__(self, plug, real_before, keys):
        self.plug = plug
        self.keys = list(keys)
        self.real_before = set(real_before)
        self.real = self.real_before | set(keys)
        self.w = World(plug)
        self.parents = self.w.parents
        self.L = Layout(self.w, self.real)
        self.E = Engine(self.w, self.L, self.real, self.keys)
        self.out = {}
        self.log = []
        self.shim_extra = collections.defaultdict(list)
        self.hosts = []
        for kk in self.real:
            for e in self.L.entries.get(kk, []):
                if e in self.L.by[kk] and self.parents.get(e):
                    self.L.mname[e] = self.entry_name(kk, e)
        self.problems = []

    # ------------------------------------------------------------------ text access
    def text(self, rel):
        if rel in self.out:
            return self.out[rel]
        return read(self.plug + "/" + rel)

    def set(self, rel, text):
        self.out[rel] = text

    def owner(self, rel):
        return self.L.owner.get(rel) or model.file_owner(rel)

    # ------------------------------------------------------------------ naming
    def entry_name(self, k, e):
        for p, name, m in self.parents.get(e, []):
            if name != "<include>":
                return name
        for p, name, m in self.parents.get(e, []):
            if name == "<include>":
                ms = ED.find_mounts(self.text(p), p, e)
                for x in ms:
                    if x["kind"] == "include" and x.get("inline"):
                        return x["inline"][2]
        return slug(e)

    def entries(self, k):
        out = []
        for e in self.L.entries[k]:
            if e in self.L.by[k] and self.parents.get(e):
                out.append(e)
        return out

    # ------------------------------------------------------------------ per file transform
    def glob_children_text(self, f, seen=None):
        seen = seen or set()
        out = []
        for name, child, m in self.w.T.children(f):
            if child in seen or name == "<include>" and False:
                continue
            seen.add(child)
            ct = self.text(child)
            if re.search(r"(?m)^[ \t]*(?:pub(?:\([^)]*\))?[ \t]+)?use[ \t]+super::\*;", blank(ct)) or name == "<include>":
                out.append(ct)
                out.append(self.glob_children_text(child, seen))
        return "\n".join(out)

    def transform_file(self, k, f):
        t = self.text(f)
        nres = len(self.E.unresolved)
        t = self.E.rewrite(t, f, k)
        t, dropped = ED.prune_dead_uses(t, keep_if=lambda p: p[0] not in ("crate", "super"), extra=self.glob_children_text(f))
        if dropped:
            self.log.append(("pruned-dead-imports", f, dropped))
            gone = set(dropped)
            self.E.unresolved[nres:] = [u for u in self.E.unresolved[nres:] if u[1] not in gone]
        parent = self.w.parent_module(f)
        pf = self.w.parent_file(f)
        if parent is not None and pf is not None and self.owner(pf) != k:
            t = self.E.explode(t, f, parent, k)
        return t

    # ------------------------------------------------------------------ foreign mounts
    def foreign_children(self, k):
        """mounts inside files owned by k whose child is owned by another crate: (parent file, child file, owner of child)"""
        res = []
        for f in self.L.by[k]:
            for name, child, m in self.w.T.children(f):
                j = self.owner(child)
                if j != k and not j.startswith(("UNMAPPED",)) and child not in external_files(self.w):
                    res.append((f, child, j, name))
        return res

    def fix_foreign_mounts(self, k):
        for parent, child, j, name in self.foreign_children(k):
            t = self.text(parent)
            names = []
            top = j.startswith("TOP")
            down = j in self.real and RANK.get(j, 99) < RANK[k] and not top
            repl = ""
            if down and name != "<include>":
                repl = f"pub(crate) use {libname(j)}::{self.entry_name(j, child)};\n"
            nt = ED.remove_mount(t, parent, child, repl, names)
            if not down and name != "<include>":
                nt, removed = ED.drop_uses_of(nt, names)
                for stmt in removed:
                    self.shim_extra[parent].append((j, child, stmt))
            if top and name != "<include>":
                self.hosts.append(dict(parent=parent, child=child, name=name, owner=j, cfg=self.entry_cfg(child)))
            if nt != t:
                self.set(parent, nt)
                self.log.append(("foreign-mount", k, parent, child, j))

    # ------------------------------------------------------------------ roots
    def aliases_used(self, k):
        blob = "\n".join(blank(self.text(f)) for f in self.L.by[k])
        used = []
        for a in EXT_ALIASES:
            if re.search(r"(?<![A-Za-z0-9_$.:])" + a + r"::", blob) or re.search(r"(?<![A-Za-z0-9_$.:])use\s+" + a + r"\b", blob):
                used.append(a)
        return used

    def assemble_root(self, k):
        root = self.L.root[k]
        t = self.text(root)
        ents = self.entries(k)
        head, rest = [], t.split("\n")
        i = 0
        while i < len(rest) and (rest[i].startswith("//!") or rest[i].strip() == ""):
            head.append(rest[i]); i += 1
        body = "\n".join(rest[i:])
        have = set(re.findall(r"extern crate semio_framework_os_kernel as (\w+);", body))
        alias = [f"extern crate semio_framework_os_kernel as {a};" for a in self.aliases_used(k) if a not in have]
        mounts = []
        rootdir = os.path.dirname(root)
        for e in ents:
            if os.path.dirname(e) == rootdir and os.path.basename(e) == os.path.basename(root):
                continue
            name = self.entry_name(k, e)
            cfg = self.entry_cfg(e)
            mounts += [x for x in cfg] + ["#[doc(hidden)]", f'#[path = "{os.path.relpath(e, rootdir)}"]', f"pub mod {name};", f"pub use {name}::*;"]
        pre = "\n".join(head).rstrip("\n") + "\n" if head else ""
        text = pre + ("\n".join(alias) + "\n" if alias else "") + body.lstrip("\n") + ("\n" + "\n".join(mounts) + "\n" if mounts else "")
        self.set(root, text)

    def entry_cfg(self, e):
        res = []
        for p, name, m in self.parents.get(e, []):
            if m is None:
                continue
            for a in re.findall(r"#\[\s*cfg\((?:[^\[\]]|\[[^\]]*\])*\)\s*\]", m["attrs"]):
                if a not in res:
                    res.append(a)
        return res

    # ------------------------------------------------------------------ run
    def run(self):
        for k in self.keys:
            for f in sorted(self.L.by[k]):
                self.set(f, self.transform_file(k, f))
            self.fix_foreign_mounts(k)
        for k in self.keys:
            self.assemble_root(k)
        import hubs
        changes, host_files = hubs.build(self)
        for rel, t in changes.items():
            self.set(rel, t)
        for rel, t in host_files.items():
            self.set(rel, t)
        return self.out


if __name__ == "__main__":
    plug, outdir, real_before, keys = sys.argv[1:5]
    fb = FB(plug, set(filter(None, real_before.split(","))), keys.split(","))
    fb.run()
    for rel, text in fb.out.items():
        write(outdir + "/" + rel, text)
    print("files:", len(fb.out), "unresolved:", len(fb.E.unresolved))
    for u in fb.E.unresolved[:120]:
        print("  ", u[4] if len(u) > 4 else "?", u[0][-60:], u[1][:90], u[2])
    for l in fb.log[:40]:
        print("  log", l)
