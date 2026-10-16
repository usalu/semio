#!/usr/bin/env python3
"""🧱️ Group driver: turns a spec (crates to flip, class B roots, promoted names, skipped files, hub replacements) into an overlay of new file texts, manifests and the repository edits.
usage: python3 <spec>.py [--overlay | --apply | --preview-notes]"""
import collections, json, os, re, sys
from lib import *
import flipcore as F
import world as Wd
import rsparse as P
from rs import blank
import widen as WD
import manifest as MF
import owners as O
import uo

OVERLAY_DIR = SCRATCH + "/pi1"


class Spec:
    group = []
    roots_b = {}
    promote = {}
    skip_files = set()
    skip_mounts = set()
    hub_file = None
    hub_units = {}
    delete = []
    name = "g"
    drop_manifest_stub = True
    extra_edits = {}          # abs path -> callable(text) -> text  (consumer re-points, recorded)
    extra_mounts = []         # (module name, abs file): files no module mounts yet that a group crate wires at the flip
    rehome = []               # facade-resident files whose parent module moves into a crate
    host_reexport_into = {}   # crate key -> composed hub module that re-exports the items its relocated test units define
    root_extra = {}           # crate key -> text appended to its root (re-exports of promoted modules)
    dev_texts = {}            # crate key -> shared fixture files its tests mount (their dependencies count as dev-dependencies)


def replace_units(hub_text, units):
    clean = blank(hub_text)
    stmts = P.statements(hub_text, clean, 0, len(hub_text))
    edits = []
    found = set()
    for st in stmts:
        if st.kind == "mod" and not st.inline and st.name in units:
            edits.append((st.start, st.end, units[st.name]))
            found.add(st.name)
    assert found == set(units), set(units) - found
    edits.sort(reverse=True)
    t = hub_text
    for a, b, rep in edits:
        t = t[:a] + rep + t[b:]
    return t


MODOPEN = re.compile(r"\bmod\s+[A-Za-z_]\w*\s*\{")


def root_super_fix(text):
    """`pub(super)` at the module level of a crate root climbs above the crate: it becomes `pub(crate)`"""
    clean = blank(text)
    opens = {m.end() - 1 for m in MODOPEN.finditer(clean)}
    out, last, stack = [], 0, []
    pos = 0
    rx = re.compile(r"pub\s*\(\s*super\s*\)")
    marks = [m for m in rx.finditer(clean)]
    if not marks:
        return text
    depth_at = {}
    j = 0
    for i, c in enumerate(clean):
        if j < len(marks) and i == marks[j].start():
            depth_at[i] = sum(1 for s in stack if s)
            j += 1
        if c == "{":
            stack.append(i in opens)
        elif c == "}" and stack:
            stack.pop()
    for m in marks:
        if depth_at.get(m.start(), 1) == 0:
            out.append(text[last:m.start()])
            out.append("pub(crate)")
            last = m.end()
    out.append(text[last:])
    return "".join(out)


class Driver:
    def __init__(self, spec):
        self.spec = spec
        self.pl = None
        self.ov = None

    def plan(self):
        sp = self.spec
        pl = F.Plan(sp.group, {k: v for k, v in sp.roots_b.items()}, promote=sp.promote, extra_mounts=sp.extra_mounts)
        pl.build(skip_files=sp.skip_files)
        out = pl.finalize(skip_mounts=sp.skip_mounts, skip_files=sp.skip_files, root_extra=sp.root_extra)
        self.pl, self.out = pl, out
        return pl, out

    def overlay(self):
        sp = self.spec
        pl, out = self.plan()
        ov = {}
        for f, t in out.items():
            ov[os.path.relpath(f, REPO)] = t
        for f in sp.delete:
            ov[os.path.relpath(f, REPO)] = None
        for K, rf in pl.roots_all.items():
            relp = os.path.relpath(pl.roots.get(K, rf), REPO)
            if relp in ov and ov[relp]:
                ov[relp] = root_super_fix(ov[relp])
        flipped = set(sp.group) | set(uo.BASE_REAL) | set(uo.flipped_by_me())
        hub = read(sp.hub_file)
        hub = replace_units(hub, sp.hub_units) if sp.hub_units else hub
        for K in sp.group:
            ht = pl.host_text(K, flipped)
            if ht:
                hp = pl.host_path(K)
                ov[os.path.relpath(hp, REPO)] = ht
                for uf, ut in pl.host_unit_files.get(K, {}).items():
                    ov[os.path.relpath(uf, REPO)] = ut
                rel_host = os.path.relpath(hp, os.path.dirname(sp.hub_file))
                hub = hub.rstrip("\n") + f'\n#[cfg(test)]\n#[path = "{rel_host}"]\npub(crate) mod {pl.host_module_name(K)};\n'
        for (K, unit), names in pl.host_exports.items():
            target = sp.host_reexport_into.get(K)
            if not target or not names:
                continue
            i = hub.index("pub(crate) mod " + target + " {")
            depth, j = 0, hub.index("{", i)
            k = j
            while True:
                if hub[k] == "{":
                    depth += 1
                elif hub[k] == "}":
                    depth -= 1
                    if depth == 0:
                        break
                k += 1
            line = f"    #[cfg(test)]\n    pub use super::{pl.host_module_name(K)}::{unit}::{{{', '.join(names)}}};\n"
            hub = hub[:k] + line + hub[k:]
        ov[os.path.relpath(sp.hub_file, REPO)] = hub
        self.repoints = []
        fe = pl.foreign_mount_edits(skip=set(sp.hub_units))
        for f, rows in fe.items():
            relp = os.path.relpath(f, REPO)
            cur = ov.get(relp, read(f))
            for old, new, note in rows:
                assert cur.count(old) == 1, (rel(f), old[:80], cur.count(old))
                cur = cur.replace(old, new)
                self.repoints.append((rel(f), note))
            ov[relp] = cur
        self.rehome_notes = {}
        for f in sp.rehome:
            r = pl.rehome(f)
            if r is not None:
                ov[os.path.relpath(f, REPO)] = r[0]
                self.rehome_notes[rel(f)] = r[1]
        for f, fn in sp.extra_edits.items():
            cur = ov.get(os.path.relpath(f, REPO), read(f))
            ov[os.path.relpath(f, REPO)] = fn(cur)
        rep = self.widen(ov)
        self.ov, self.widen_report = ov, rep
        import hashlib
        self.pre = {}
        for relp in ov:
            f = REPO + "/" + relp
            if os.path.exists(f):
                self.pre[relp] = hashlib.sha256(open(f, "rb").read()).hexdigest()
        return ov

    def kfiles(self, ov):
        pl = self.pl
        kf = collections.defaultdict(set)
        for relp, t in ov.items():
            f = REPO + "/" + relp
            if t is None or not f.startswith(PLUG + "/") or not f.endswith(".rs") or "/🪝️mounts/" in f:
                continue
            k = pl.LY.part_owner(pl.file_part[f]) if f in pl.file_part else O.owner_of(rel(f))
            if k in pl.gset:
                kf[k].add(relp)
        return kf

    def widen(self, ov):
        kf = self.kfiles(ov)
        live = {os.path.relpath(f, REPO): f for f in WD.live_rs_files()}
        report = {}
        for K in self.spec.group:
            outside = set()
            for relp, f in live.items():
                if relp in kf[K] or (relp in ov and ov[relp] is None):
                    continue
                if relp in ov:
                    outside |= set(WD.IDENT.findall(blank(ov[relp])))
                else:
                    outside |= WD.file_idents(f)
            for relp, t in ov.items():
                if relp not in live and t is not None and relp not in kf[K] and relp.endswith(".rs"):
                    outside |= set(WD.IDENT.findall(blank(t)))
            changed = 0
            alldefs = {}
            for relp in sorted(kf[K]):
                t = ov[relp]
                alldefs[relp] = WD.collect(t, blank(t), 0, len(t))
            names = {d[1] for ds in alldefs.values() for d in ds} & outside
            grow = True
            while grow:
                grow = False
                for ds in alldefs.values():
                    by_stmt = collections.defaultdict(set)
                    for d in ds:
                        if d[0] == "use":
                            by_stmt[d[6].start].add(d[1])
                    for locs in by_stmt.values():
                        if locs & names and not locs <= names:
                            names |= locs
                            grow = True
            for relp in sorted(kf[K]):
                t = ov[relp]
                nt = WD.apply_widen(t, alldefs[relp], names)
                if nt != t:
                    ov[relp] = nt
                    changed += 1
            report[K] = changed
        return report

    def manifests(self):
        pl, ov = self.pl, self.ov
        res = {}
        kf = self.kfiles(ov)
        for K in self.spec.group:
            prod, test = [], []
            for relp in kf[K]:
                f = REPO + "/" + relp
                a, b = pl.split_prod_test(f, ov[relp]) if f in pl.file_part else (ov[relp], "")
                prod.append(a)
                test.append(b)
            for f in self.spec.dev_texts.get(K, []):
                test.append(read(f))
            res[K] = MF.plan_manifest(K, prod, test, flip_lib=(K in self.spec.roots_b))
        return res

    def write_overlay(self):
        path = OVERLAY_DIR + "/overlay-" + self.spec.name + ".json"
        os.makedirs(OVERLAY_DIR, exist_ok=True)
        json.dump(self.ov, open(path, "w"), ensure_ascii=False)
        return path

    def apply(self):
        import ed, hashlib
        mans = self.manifests()
        stale = []
        for relp, h in self.pre.items():
            f = REPO + "/" + relp
            if not os.path.exists(f) or hashlib.sha256(open(f, "rb").read()).hexdigest() != h:
                stale.append(relp)
        if stale:
            raise SystemExit("a peer edited files since the overlay was built; regenerate: " + ", ".join(stale[:5]))
        for relp, t in self.ov.items():
            path = REPO + "/" + relp
            if t is None:
                ed.delete(path)
            else:
                ed.put(path, t)
        fac = read(MF.PKG_FACADE + "/Cargo.toml")
        for K, (text, rep) in mans.items():
            ed.put(MF.pkg_dir(K) + "/Cargo.toml", text)
            for f in rep.get("features", []):
                fac = MF.facade_forward(fac, f, K)
        if fac != read(MF.PKG_FACADE + "/Cargo.toml"):
            ed.put(MF.PKG_FACADE + "/Cargo.toml", fac)
        for K in self.spec.group:
            if K in self.spec.roots_b and self.spec.drop_manifest_stub:
                stub = MF.pkg_dir(K) + "/🦀️.rs"
                if os.path.exists(stub):
                    ed.delete(stub)
        cur = uo.flipped_by_me()
        for K in self.spec.group:
            if K not in cur:
                cur.append(K)
        write(uo.FLIPPED, json.dumps(cur))
        return mans


def main(spec):
    d = Driver(spec)
    args = sys.argv[1:]
    d.overlay()
    pl = d.pl
    if "--notes" in args or not args:
        for K, n in pl.fnotes.items():
            print(K, len(n))
            for x in n:
                print("   ", x)
    if "--moved" in args:
        for K, rows in pl.moved_mounts.items():
            for r in rows:
                print("moved-mount", K, r)
        for K, rows in pl.dropped_uses.items():
            for r in rows:
                print("dropped-use", K, r[1], "|", r[0][:200].replace("\n", " "))
    if "--repoints" in args or "--overlay" in args:
        for r in d.repoints:
            print("repoint", r)
    if "--overlay" in args:
        print("overlay", len(d.ov), "files, widened", d.widen_report, "->", d.write_overlay())
        for K, (t, rep) in d.manifests().items():
            print("manifest", K, rep)
    if "--apply" in args:
        mans = d.apply()
        print("applied", len(d.ov), "files;", {K: r for K, (t, r) in mans.items()})
    return d
