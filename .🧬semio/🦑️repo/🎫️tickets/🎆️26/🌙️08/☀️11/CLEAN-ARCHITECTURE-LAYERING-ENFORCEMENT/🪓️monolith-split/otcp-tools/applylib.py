"""Generic companion application: P-side mount removal, companion crate files, mirror lib roots.
usage in drivers: ctx = Ctx(cid, pkg, p_manifest_rel); ctx.remove_mounts(); ctx.companion(...)"""
import collections, os, re, sys
sys.path.insert(0, os.path.dirname(__file__)); sys.path.insert(0, os.path.dirname(__file__) + "/../frs-tools")
import edit as E
import genmirror, compgen, union, extcrates


class Node:
    def __init__(self, name):
        self.name = name
        self.keys = []
        self.children = collections.OrderedDict()


class Ctx:
    def __init__(self, cid, pkg, p_manifest_rel, p_dep_name, real_map=None):
        self.cid, self.pkg = cid, pkg
        self.c = union.cfg_of(cid)
        self.snap = os.path.join(union.W, cid, "snap")
        self.live_lib = f"{E.ROOT}/{self.c['p_lib']}"
        self.lib = f"{self.snap}/{self.c['p_lib']}"
        self.p_manifest_rel = p_manifest_rel
        self.p_dep_name = p_dep_name
        self.real_map = real_map or {}
        self.manifest = open(os.path.join(self.snap, p_manifest_rel), encoding="utf-8").read()
        self.specs = compgen.dep_specs(self.manifest)
        self.node_extras = {}
        self.traits = self.trait_names()
        self.mounts = {("::".join(m[0]) or "(root)") + ("" if m[1] == "tests" else "@" + m[1]): (m[0], m[1], self.live(m[2]), m[3], self.live(m[4])) for m in genmirror.test_mounts(self.lib)}

    def trait_names(self):
        names = set()
        for d in self.c['p_dirs']:
            for dp, dn, fn in os.walk(os.path.join(self.snap, d)):
                for f in fn:
                    if f.endswith('.rs'):
                        t = open(os.path.join(dp, f), encoding='utf-8').read()
                        names |= {m.group(1) for m in re.finditer(r'\btrait\s+(\w+)', t)}
        return names

    def add_synthetic(self, host_mp, name, host_file, tfile):
        key = ("::".join(host_mp) or "(root)") + ("" if name == "tests" else "@" + name)
        self.mounts[key] = (tuple(host_mp), name, host_file, None, tfile)
        self.synthetic = getattr(self, "synthetic", {})
        self.synthetic[key] = tfile
        return key

    def live(self, p):
        return p.replace(self.snap, E.ROOT) if isinstance(p, str) else p

    def remove_mounts(self, only=None):
        for key, (host_mp, name, host_file, d, tfile) in self.mounts.items():
            if only and key not in only or d is None:
                continue
            t = E.read(host_file)
            try:
                nt = genmirror.removal(t, d)
            except (AssertionError, IndexError):
                print("already removed or absent:", key)
                continue
            if nt != t:
                E.pre_copy(host_file)
                E.write(host_file, nt)
                E.note("edit", host_file)

    def real(self, mp):
        k = "::".join(mp)
        best = None
        for a, b in self.real_map.items():
            ak = a
            if k == ak or k.startswith(ak + "::") or ak == "":
                if best is None or len(ak) > len(best[0]):
                    best = (ak, b)
        if best:
            rest = k[len(best[0]):].lstrip(":")
            base = tuple(x for x in best[1].split("::") if x)
            return (self.pkg,) + base + tuple(x for x in rest.split("::") if x)
        return (self.pkg,) + tuple(mp)

    def subtree(self, key):
        host_mp, name, host_file, d, tfile = self.mounts[key]
        if key in getattr(self, "synthetic", {}):
            return [tfile]
        return [self.live(f) for f in genmirror.subtree_files(self.lib, host_mp + (name,))]

    def uses(self, key):
        host_mp, name, host_file, d, tfile = self.mounts[key]
        used = genmirror.used_tokens(self.subtree(key))
        snap_host = host_file.replace(E.ROOT, self.snap)
        return genmirror.host_use_lines(snap_host, host_mp, self.pkg, self.real, used, traits=self.traits)

    def uses_node(self, keys):
        """host imports of a mirror node, computed once from the union of the tokens of all its test mounts (no duplicate names)."""
        if not keys:
            return []
        host_mp, name, host_file, d, tfile = self.mounts[keys[0]]
        files = []
        for k in keys:
            files += self.subtree(k)
        used = genmirror.used_tokens(files)
        snap_host = host_file.replace(E.ROOT, self.snap)
        return genmirror.host_use_lines(snap_host, host_mp, self.pkg, self.real, used, traits=self.traits)

    def real_prefix(self, host_mp):
        r = self.real(host_mp)
        return "::".join(r)

    def render_root(self, cdir, keys, title, emoji, allocator=None, extra=None):
        lines = [f"//! {emoji} {title}", "//!", "//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.", "#![cfg(test)]", ""]
        if allocator:
            lines += [allocator, ""]
        root = Node("")
        for k in keys:
            host_mp = self.mounts[k][0]
            cur = root
            for seg in host_mp:
                cur = cur.children.setdefault(seg, Node(seg))
            cur.keys.append(k)
        for mp in self.node_extras:
            cur = root
            for seg in mp:
                cur = cur.children.setdefault(seg, Node(seg))
        lines += [f"pub use {self.pkg}::*;"]
        if extra:
            lines += extra
        lines += self.uses_node(root.keys)
        for k in root.keys:
            host_mp, mname, host_file, d, tfile = self.mounts[k]
            lines += ["", f'#[path = "{os.path.relpath(tfile, cdir)}"]', f"mod {mname};"]
        lines.append("")
        for nm, ch in root.children.items():
            lines += self.emit(ch, (nm,), "", cdir, cdir)
            lines.append("")
        while lines and lines[-1] == "":
            lines.pop()
        return "\n".join(lines) + "\n"

    def emit(self, node, mp, pad, cdir, parent_dir):
        out = []
        single = len(node.keys) == 1 and not node.children and self.mounts[node.keys[0]][4].endswith("/🦀️.rs") and self.mounts[node.keys[0]][1] == "tests"
        if single:
            host_mp, mname, host_file, d, tfile = self.mounts[node.keys[0]]
            mdir = os.path.dirname(tfile)
        else:
            mdir = parent_dir
        rel = os.path.relpath(mdir, parent_dir) if mdir != parent_dir else "."
        out += [f'{pad}#[path = "{rel}"]', f"{pad}pub mod {node.name} {{", f"{pad}    pub use {self.real_prefix(mp)}::*;"]
        uses = list(self.uses_node(node.keys))
        ex = self.node_extras.get(mp)
        if ex:
            for u in ex["uses"]:
                if u not in uses:
                    uses.append(u)
        for u in uses:
            out.append(f"{pad}    {u}")
        body = []
        if ex:
            for inc in ex["includes"]:
                body += ["", f'{pad}    include!("{os.path.relpath(inc, cdir)}");']
        for k in node.keys:
            host_mp, mname, host_file, d, tfile = self.mounts[k]
            if single:
                body += ["", f'{pad}    #[path = "{os.path.basename(tfile)}"]', f"{pad}    mod {mname};"]
            else:
                body += ["", f'{pad}    #[path = "{os.path.relpath(tfile, mdir)}"]', f"{pad}    mod {mname};"]
        for nm, ch in node.children.items():
            body += [""] + self.emit(ch, mp + (nm,), pad + "    ", cdir, mdir)
        out += body
        out.append(f"{pad}}}")
        return out

    def used_deps(self, keys, root_text):
        files = []
        for k in keys:
            files += self.subtree(k)
        used = extcrates.used(files, list(self.specs))
        for ident in self.specs:
            if re.search(r"(?<![\w:])" + re.escape(ident) + r"\s*::", root_text):
                used.setdefault(ident, set()).add("root")
        return used

    def companion(self, cdirrel, keys, crate_name, cid_, project, desc, title, emoji, p_features=(), allocator=None, extra=None, extra_deps=(), p_rel=None, skip_deps=(), framework=False):
        cdir = f"{E.ROOT}/{cdirrel}"
        text = self.render_root(cdir, keys, title, emoji, allocator, extra)
        used = self.used_deps(keys, text + (allocator or ""))
        dep_lines = []
        for ident in sorted(used):
            if ident in skip_deps:
                continue
            sec, line = compgen.merged_line(self.specs[ident])
            dep_lines.append((sec, compgen.reroot(line, self.extra_ups(cdirrel))))
        for l in extra_deps:
            dep_lines.append(("dependencies", l))
        pk = cdirrel + "📦️packages/🦀️rust/"
        depth = len(pk.rstrip("/").split("/")) - (1 if framework else 0)
        pdir = os.path.dirname(self.p_manifest_rel)
        prel = p_rel or os.path.relpath(f"{E.ROOT}/{pdir}", f"{E.ROOT}/{pk.rstrip('/')}")
        cargo = compgen.cargo_text(crate_name, cid_, desc, depth, dep_lines, self.p_dep_name, prel, p_features)
        E.put(pk + "Cargo.toml", cargo)
        E.put(pk + "📋️project.json", compgen.project_json(project))
        E.put(cdirrel + "🦀️.rs", text)
        return sorted(used)

    def extra_ups(self, cdirrel):
        pk = cdirrel + "📦️packages/🦀️rust"
        pdir = os.path.dirname(self.p_manifest_rel)
        return len(pk.rstrip("/").split("/")) - len(pdir.rstrip("/").split("/"))
