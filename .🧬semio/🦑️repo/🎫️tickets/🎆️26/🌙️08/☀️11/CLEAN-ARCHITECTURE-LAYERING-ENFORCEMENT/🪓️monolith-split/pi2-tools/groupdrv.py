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
Wd_O = O
import uo

OVERLAY_DIR = SCRATCH + "/pi2"


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
    providers = {}            # companion key -> host module names that other bins call: mounted without `cfg(test)`, dependencies are normal ones


ROOT_EXT = {
    "DslValue": "semio_framework_value::DslValue",
    "Fault": "semio_framework::Fault",
    "FaultCode": "semio_framework::FaultCode",
    "FaultOrigin": "semio_framework::FaultOrigin",
    "ViewModel": "semio_framework::ViewModel",
    "OriginalFixtureCaller": "semio_framework_plugin_testing_caller::OriginalFixtureCaller",
    "OriginalRenderFailure": "semio_framework_plugin_ownership::sdk_fault::OriginalRenderFailure",
    "OriginalRenderResult": "semio_framework_plugin_ownership::sdk_fault::OriginalRenderResult",
}
ROOT_EXT_RX = re.compile(r"(?<![A-Za-z0-9_:$])crate::(" + "|".join(ROOT_EXT) + r")\b")


PATH_SUBST = [
    ("crate::app_testing::artifact_app_laws::OriginalFixtureCaller", "semio_framework_plugin_testing_caller::OriginalFixtureCaller"),
    ("artifact_app_laws::OriginalFixtureCaller", "semio_framework_plugin_testing_caller::OriginalFixtureCaller"),
    ("use crate::declarations;", "use semio_framework_plugin_decl_registry::declarations;"),
    ("pub use super::OriginalFixtureCaller;", "pub use semio_framework_plugin_testing_caller::OriginalFixtureCaller;"),
    ("crate::plugin_app::NoPluginApp", "semio_framework_plugin_reactor_app::NoPluginApp"),
    ("super::PluginBuilder::new", "semio_framework_plugin_builder::PluginBuilder::new"),
    ("crate::reserved_jobs::FrameworkCutJob", "semio_framework_plugin_job::reserved_jobs::FrameworkCutJob"),
    ("semio_framework_plugin_job::reserved_jobs::original_configuration_failure::OriginalConfigurationFailure", "semio_framework_plugin_job::reserved_jobs::OriginalConfigurationFailure"),
    ("use crate::__semio_dispatch_PluginApp;", "use semio_framework_plugin_reactor_app::__semio_dispatch_PluginApp;"),
    ("crate::declarations::fixture::", "crate::declarations_fixture_tests::fixture::"),
]
PATH_RX = [
    (r"(?<![A-Za-z0-9_:$])crate::plugin_runtime\b", "semio_framework_plugin::plugin_runtime"),
    (r"(?<![A-Za-z0-9_:$])crate::InstanceExtensions\b", "semio_framework_plugin_instance_app::InstanceExtensions"),
    (r"(?<![A-Za-z0-9_:$])crate::app::PluginLifecycleStep\b", "semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep"),
    (r"(?<![A-Za-z0-9_:$])crate::app::ArtifactInitializationAdmission\b", "semio_framework_plugin_ownership::store_lifecycle::ArtifactInitializationAdmission"),
    (r"(?<![A-Za-z0-9_:$])crate::app::ArtifactStoreInitializationJob\b", "semio_framework_plugin_ownership::store_lifecycle::ArtifactStoreInitializationJob"),
    (r"(?<![A-Za-z0-9_:$])crate::bounded_first_step_tool_proofs!", "semio_framework_plugin_decl_app::bounded_first_step_tool_proofs!"),
    (r"(?<![A-Za-z0-9_:$])crate::transient_root!", "semio_framework_plugin::transient_root!"),
    (r"(?<![A-Za-z0-9_:$])super::super::(RUNTIME_CLOSE_CONSTRUCTION_(?:LIVE|FAIL))\b", r"semio_framework_plugin_reactor_lifecycle::life_cleanup_pumps::\1"),
    (r"(?<![A-Za-z0-9_:$])crate::app_testing::mutation_fixture::", "crate::testing_laws_unit_1::mutation_fixture::"),
    (r"(?<![A-Za-z0-9_:$])crate::media_operation_refusal::tests::", "crate::document_media_test_hosts::document_media_unit_1::tests::"),
    (r"(?<![A-Za-z0-9_:$])crate::reserved_jobs::", "semio_framework_plugin_job::reserved_jobs::"),
    (r"(?<![A-Za-z0-9_:$])crate::plugin_app::", "semio_framework_plugin_reactor_app::"),
    (r"(?<![A-Za-z0-9_:$])crate::media_contract::(original_(?:completion_result_receiving|handle_receiving|download_receiving_chunk)_and_physical_close)\b", r"crate::media_publication_tests_host::\1"),
]
PATH_RX.append((r"(?<![A-Za-z0-9_:$])crate::media_contract::\{", "semio_framework_plugin_decl_views::media_contract::{"))
PATH_RX.append((r"(?<![A-Za-z0-9_:$])crate::app_testing::", "semio_framework_plugin_testing_laws::"))
PROVIDER_PATHS = {"crate::reactor::reactor_driver::": "reactor_core_test_hosts::reactor_core_unit_1::reactor_driver::"}
ROOT_GROUP_RX = re.compile(r"(?<![A-Za-z0-9_:$])use crate::\{([^{}]*)\};")


_SF = {}


def sf_exports():
    """names the external `semio_framework` crate exports at its root (the monolith root glob re-exported them as `crate::Name`)"""
    if not _SF:
        t = read(REPO + "/🧰️framework/📦️packages/🦀️rust/🦀️.rs")
        c = blank(t)
        for m in re.finditer(r"pub\s+use\s+([^;]*);", c):
            body = m.group(1)
            if "{" in body:
                for part in body[body.index("{") + 1:body.rindex("}")].split(","):
                    mm = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*$", part.strip())
                    if mm and "{" not in part:
                        _SF[mm.group(1)] = "semio_framework::" + mm.group(1)
            else:
                mm = re.search(r"([A-Za-z_][A-Za-z0-9_]*)\s*$", body.strip())
                if mm and mm.group(1) != "*":
                    _SF[mm.group(1)] = "semio_framework::" + mm.group(1)
        for k, v in ROOT_EXT.items():
            _SF[k] = v
    return _SF


def fix_root_ext(text, companion=False, bin_key=None, providers=None):
    """names the monolith root re-exported from the external `semio_framework` glob: spelled with their real path"""
    ext_map = sf_exports() if companion else ROOT_EXT
    rx = re.compile(r"(?<![A-Za-z0-9_:$])crate::(" + "|".join(sorted(map(re.escape, ext_map), key=len, reverse=True)) + r")\b") if companion else ROOT_EXT_RX

    def grouped(m):
        names = [n.strip() for n in m.group(1).split(",") if n.strip()]
        ext = [n for n in names if n in ext_map]
        if not ext:
            return m.group(0)
        rest = [n for n in names if n not in ext_map]
        lines = [f"use {ext_map[n]};" for n in ext]
        if rest:
            lines.append("use crate::{" + ", ".join(rest) + "};")
        return "\n".join(lines)
    text = ROOT_GROUP_RX.sub(grouped, text)
    for a, b in PATH_SUBST:
        text = text.replace(a, b)
    if companion:
        for a, b in PATH_RX:
            text = re.sub(a, b, text)
        for a, b in PROVIDER_PATHS.items():
            home = next((k for k, mods in (providers or {}).items() if b.split("::")[0] in mods), None)
            if home:
                prefix = "crate::" if bin_key == home else "semio_framework_plugin_" + home.replace("-", "_") + "::"
                text = text.replace(a, prefix + b)
    return rx.sub(lambda m: ext_map[m.group(1)], text)


USE_LINE = re.compile(r"^(?:pub(?:\([a-z]+\))? )?use [^\n]*;$")


def local_macro_reexports(text):
    """`pub use <abs path>::NAME;` of a `macro_rules! NAME` this very file defines names the macro itself: the local re-export stays a bare `use NAME;`"""
    names = set(re.findall(r"(?m)^[ \t]*macro_rules!\s+([A-Za-z_]\w*)", text))
    for n in names:
        text = re.sub(r"(?m)^((?:pub(?:\([a-z]+\))?\s+)?use)\s+(?:[A-Za-z_]\w*::)+" + re.escape(n) + r";", lambda m: m.group(1) + " " + n + ";", text)
    return text


def narrow_local_macro_uses(text):
    """a `macro_rules!` is only nameable crate-wide: its local `use NAME;` re-export keeps `pub(crate)`"""
    for n in set(re.findall(r"(?m)^[ \t]*macro_rules!\s+([A-Za-z_]\w*)", text)):
        text = re.sub(r"(?m)^pub use " + re.escape(n) + r";", "pub(crate) use " + n + ";", text)
    return text


def drop_spurious_component(text):
    """the wasm-only `abi::component` module is imported by name collision with a `.component` field; without a `component::` path use it goes"""
    if re.search(r"\bcomponent::", blank(text)):
        return text
    return re.sub(r"(?m)^use semio_framework_plugin_abi::component;\n", "", text)


def dedupe_uses(text):
    """a module-level one-line `use` statement repeated with the same attributes is kept once"""
    lines = text.split("\n")
    out, seen, attrs = [], set(), []
    for l in lines:
        if l.startswith("#["):
            attrs.append(l)
            out.append(l)
            continue
        if USE_LINE.match(l):
            key = ("\n".join(attrs), l)
            if key in seen:
                for _ in attrs:
                    out.pop()
                attrs = []
                continue
            seen.add(key)
        attrs = []
        out.append(l)
    return "\n".join(out)


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


PROD_ONLY = {"testing-fixtures"}


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
        for K, mods in sp.providers.items():
            rf = pl.roots.get(K) or pl.roots_all[K]
            relp = os.path.relpath(rf, REPO)
            tx = ov.get(relp)
            if tx:
                for md in mods:
                    rx = re.compile(r"#\[cfg\((?:test|any\(test, feature = \"[a-z-]+\"\))\)\]\n(#\[path = \"[^\"]+\"\]\n(?:pub(?:\([a-z]+\))? )?mod " + re.escape(md) + r";)")
                    tx, n = rx.subn(r"\1", tx)
                    assert n == 1, (K, md, n)
                ov[relp] = tx
        for K in sp.group:
            if Wd_O.CR[K].get("kind") != "companion":
                continue
            rf = pl.roots.get(K) or pl.roots_all[K]
            relp = os.path.relpath(rf, REPO)
            tx = ov.get(relp)
            if not tx:
                continue
            mods = re.findall(r"pub mod ([A-Za-z0-9_]+);", tx)
            body = re.sub(r"\A(?://![^\n]*\n)+\n?", "", tx)
            em = Wd_O.CR[K]["emoji"]
            doc = f"//! {em} Companion test crate {K} of the plugin SDK partition: hosts the test roots {', '.join('`' + m + '`' for m in mods)}.\n//!\n//! 🧭️ The test sources stay where they are and are mounted here with `#[path]`; every name they use from the former facade root is spelled with the crate that owns it.\n\n"
            ov[relp] = doc + body
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
        kfs = self.kfiles(ov)
        for K, rs in kfs.items():
            for relp in rs:
                nt = drop_spurious_component(local_macro_reexports(dedupe_uses(fix_root_ext(ov[relp], Wd_O.CR[K].get("kind") == "companion", K, sp.providers))))
                if Wd_O.CR[K].get("kind") == "companion":
                    nt = nt.replace('env!("CARGO_MANIFEST_DIR"), "/../../', 'env!("CARGO_MANIFEST_DIR"), "/../../../../')
                if nt != ov[relp]:
                    ov[relp] = nt
        rep = self.widen(ov)
        for relp, t in list(ov.items()):
            if t and "macro_rules!" in t:
                nt = narrow_local_macro_uses(t)
                if nt != t:
                    ov[relp] = nt
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
            if t is None or not f.startswith(PLUG + "/") or not f.endswith(".rs") or re.search(r"/🪝️mounts/[^/]+/🦀️\.rs$", f):
                continue
            if "/🪝️mounts/" in f and O.owner_of(rel(f)) not in pl.gset:
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
                if K in PROD_ONLY:
                    a, b = a + "\n" + b, ""
                prod.append(a)
                test.append(b)
            for f in self.spec.dev_texts.get(K, []):
                test.append(read(f))
            if Wd_O.CR[K].get("kind") == "companion":
                import binmanifest as BM
                res[K] = BM.plan(K, self.closure_texts(K), provider=K in self.spec.providers)
            else:
                res[K] = MF.plan_manifest(K, prod, test, flip_lib=(K in self.spec.roots_b))
        return res

    def closure_texts(self, K):
        """texts (overlay first, live otherwise) of every file reachable from the lib root of K through `#[path]` mounts and literal includes"""
        import tree as T
        root = REPO + "/" + Wd_O.CR[K]["dir"] + "/🦀️.rs"
        seen, out, stack = set(), [], [root]
        while stack:
            f = stack.pop()
            if f in seen:
                continue
            seen.add(f)
            relp = os.path.relpath(f, REPO)
            if relp in self.ov:
                tx = self.ov[relp]
            elif os.path.exists(f):
                tx = read(f)
            else:
                continue
            if tx is None:
                continue
            out.append(tx)
            clean = blank(tx)
            here = os.path.dirname(f)
            for m in T.MOUNT.finditer(clean):
                if m.group(4) == "{":
                    continue
                pm = T.PATH_ATTR.search(tx[m.start(1):m.end(1)])
                if pm:
                    stack.append(os.path.normpath(os.path.join(here, pm.group(1))))
            for m in T.INCLUDE.finditer(tx):
                if clean[m.start():m.start() + 8] == "include!":
                    stack.append(os.path.normpath(os.path.join(here, m.group(1))))
        return out

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
            for f in ([] if Wd_O.CR[K].get("kind") == "companion" else rep.get("features", [])):
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
