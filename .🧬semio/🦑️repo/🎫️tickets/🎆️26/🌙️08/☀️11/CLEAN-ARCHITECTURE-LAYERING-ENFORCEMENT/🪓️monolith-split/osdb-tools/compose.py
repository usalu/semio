"""🧩 Compose every crate's source files from the pristine tree and the plan.

`Composer.run()` returns {relpath (below the db root): text} for all written or rewritten files, plus a report.
"""
import collections
import os
import re

from lib import *
from model import parse_items, region_name
from rs import strip, idents
from scope import Scope, ALL_MODULE_IDENTS, FACADE_FAMILY, STANDALONE_MODULES
from sources import load_all
from plan import *
from assign import assign_all, test_assign, ORDERS, TEST_FILES
from rewrite import Resolver, FileCtx, map_path, find_uses, rewrite_uses, format_uses, leaf_text, map_family

SUBMODS = {'writer', 'release'}
EMOJI = {
    'db_wal_log': '🧾', 'db_wal': '🪵', 'db_artifact_contract': '🧬', 'db_artifact_history': '📜', 'db_artifact_engine': '🛠',
    'db_artifact': '🎬', 'db_engine_base': '🧱', 'db_engine_capability_open': '🔭', 'db_engine_catalog_read': '📖',
    'db_engine_catalog_bootstrap': '🥾', 'db_engine_catalog_create': '🏗', 'db_engine_handle': '🎫', 'db_engine': '🎛',
    'db_io_core': '🧲', 'db_io_registry': '📇', 'db_io_driver': '🚦', 'db_storage_memory': '🗃', 'db_storage_fs': '📁',
}
GATED_LIBS = {c.lib for c in CRATES if c.gated}
TESTKIT_GATED = {'FaultScript', 'FaultStorage'}
TESTKIT_SPEC = {'db_wal': dict(file='📝️wal/🧪️tests/🔬️unit/🦀️.rs', exports=['committed_fixture_storage', 'aborted_event_fixture_storage'], lib='db_wal')}


def extern_alias_lines(text_stripped):
    out = []
    for a in ('pack', 'dsl', 'store'):
        if re.search(r'(?<![A-Za-z0-9_:])' + a + r'::', text_stripped):
            out.append(f'extern crate semio_framework_os_kernel as {a};')
    return out


CFG_TEST_ATTR = re.compile(r'(?m)^[ \t]*#\[cfg\(\s*test\s*\)\][ \t]*$')


def gated_split(text):
    """(normal, gated): the text without and with the constructs directly under a plain `#[cfg(test)]` attribute."""
    st = strip(text)
    spans = []
    for m in CFG_TEST_ATTR.finditer(st):
        i = m.end()
        n = len(st)
        while True:
            while i < n and st[i] in ' \t\n':
                i += 1
            if st.startswith('#[', i):
                depth = 0
                while i < n:
                    if st[i] == '[':
                        depth += 1
                    elif st[i] == ']':
                        depth -= 1
                        if depth == 0:
                            i += 1
                            break
                    i += 1
                continue
            break
        depth = 0
        j = i
        seen_brace = False
        while j < n:
            ch = st[j]
            if ch in '([{':
                depth += 1
                if ch == '{':
                    seen_brace = True
            elif ch in ')]}':
                depth -= 1
                if depth < 0:
                    break
                if depth == 0 and ch == '}' and seen_brace:
                    j += 1
                    break
            elif ch in ';,' and depth == 0:
                j += 1
                break
            j += 1
        spans.append((m.start(), j))
    normal = list(text)
    gated = []
    for a, b in spans:
        gated.append(text[a:b])
        for k in range(a, b):
            if normal[k] != '\n':
                normal[k] = ' '
    return ''.join(normal), '\n'.join(gated)


class Composer:
    def __init__(self):
        self.srcs = load_all()
        self.scope = Scope()
        self.A = assign_all(self.srcs)
        self.T = test_assign(self.srcs, self.A)
        self.homes = {}
        self.reexports = {}
        for fid, m in self.A.items():
            s = self.srcs[fid]
            h = self.homes.setdefault(s.module, {})
            for i, lib in m.items():
                it = s.items[i]
                if it.kind == 'use':
                    decl = '\n'.join(s.lines[it.header:it.end + 1])
                    mm = re.match(r'(?:pub(?:\([^)]*\))?\s+)?use[ \t\n]+(.*?);\s*$', decl, re.S)
                    if mm:
                        for p, a, g in parse_use(re.sub(r'\s+', ' ', mm.group(1)).strip()):
                            if not g:
                                if p[0] in ('crate',) or p[0] in ALL_MODULE_IDENTS or p[0] == 'super':
                                    self.reexports.setdefault(s.module, {})[a or p[-1]] = lib
                                else:
                                    h[a or p[-1]] = lib
                    continue
                if it.kind == 'impl' or not it.name:
                    continue
                h[it.name] = lib
        self.resolver = Resolver(self.scope, self.homes, self.reexports)
        self.testkit = {}
        for module, spec in TESTKIT_SPEC.items():
            s = self.srcs[spec['file']]
            sl = s.stripped.split('\n')
            names = {}
            for i, it in enumerate(s.items):
                if it.kind not in ('use', 'impl') and it.name:
                    names[it.name] = i
            seen = set()

            def closure(n):
                if n in seen:
                    return
                seen.add(n)
                it = s.items[names[n]]
                txt = re.sub(r'(?<!\.)\.\s*[A-Za-z_]\w*', '.', '\n'.join(sl[it.start:it.end + 1]))
                for x in idents(txt):
                    if x in names and x != n:
                        closure(x)
            for e in spec['exports']:
                closure(e)
            self.testkit[module] = dict(spec, idxs=sorted(names[n] for n in seen), names=seen)
            self.homes.setdefault(module, {})['testkit'] = spec['lib']
        self.report = collections.defaultdict(list)
        self.files = {}
        self.test_dirs = collections.defaultdict(dict)

    # ---------------------------------------------------------------- helpers
    def family(self, module):
        return [fid for fid, s in self.srcs.items() if s.module == module and s.role in ('main', 'sub')]

    def emit_items(self, fid, idxs, dest_dir):
        """Item text of the given indices of one source file with region transitions. Returns list of lines."""
        s = self.srcs[fid]
        out = []
        cur = []
        for i in idxs:
            it = s.items[i]
            want = list(it.regions)
            k = 0
            while k < len(cur) and k < len(want) and cur[k] == want[k]:
                k += 1
            for raw in reversed(cur[k:]):
                out.append(f'//#endregion {region_name(raw).split(" — ")[0]}')
            for raw in want[k:]:
                out.append(raw)
            cur = want
            comments = [g for g in it.gap_before if g.strip() and not g.startswith('//#') and not g.startswith('//!') and not g.startswith('#![')]
            if out and out[-1].strip() and not out[-1].startswith('//#region'):
                out.append('')
            elif out and out[-1].startswith('//#region') and False:
                pass
            out.extend(comments)
            nm_gate = it.self_type if it.kind == 'impl' else it.name
            if s.module == 'db_fault_testing' and nm_gate in TESTKIT_GATED and s.role == 'main':
                out.append('#[cfg(any(test, feature = "testkit"))]')
            body = '\n'.join(s.lines[it.start:it.end + 1])
            body = re.sub(r'\Apub\(super\)|\A((?:#\[[^\n]*\]\n|///[^\n]*\n)*)pub\(super\)', lambda m: (m.group(1) or '') + 'pub(crate)', body, count=1)
            body = self.fix_includes(body, s.path, dest_dir)
            body = self.fix_mod_paths(body, s.path, dest_dir)
            out.extend(body.split('\n'))
        for raw in reversed(cur):
            out.append(f'//#endregion {region_name(raw).split(" — ")[0]}')
        return out

    def fix_labels(self, text, orig_mod, new_mod):
        """process_isolated_law labels name the exact libtest path: db_<module>::<orig mod>::fn becomes <new mod>::fn in the crate's own test binary."""
        return re.sub(r'(process_isolated_law\(\s*")db_[a-z0-9_]+::' + re.escape(orig_mod) + r'::', lambda m: m.group(1) + new_mod + '::', text)

    def fix_mod_paths(self, body, src_path, dest_dir):
        def repl(m):
            tgt = os.path.normpath(os.path.join(os.path.dirname(src_path), m.group(1)))
            return f'#[path = "{os.path.relpath(tgt, dest_dir)}"]'
        return re.sub(r'#\[path = "([^"]+)"\]', repl, body)

    def fix_includes(self, body, src_path, dest_dir):
        def repl(m):
            rel = m.group(2)
            tgt = os.path.normpath(os.path.join(os.path.dirname(src_path), rel))
            new = os.path.relpath(tgt, dest_dir)
            return f'{m.group(1)}"{new}"'
        return re.sub(r'(include(?:_str|_bytes)?!\(\s*)"([^"]+)"', repl, body)

    # ---------------------------------------------------------------- code path rewriting
    def rewrite_code(self, text, ctx, lib, module):
        """Qualified path rewriting for a composed file of crate `lib` (module family `module`)."""
        r = self.resolver
        st = strip(text)
        edits = []
        use_spans = [(u['start'], u['end']) for u in find_uses(text)]

        def in_use(pos):
            return any(a <= pos < b for a, b in use_spans)

        def home_path(name):
            h = r.lib(module, name)
            return 'crate' if h == lib else h

        _st0 = strip(text)
        for _m in reversed(list(re.finditer(r'(?<![A-Za-z0-9_:])((?:crate::)?db_wal)::tests::', _st0))):
            text = text[:_m.start()] + _m.group(1) + '::testkit::' + text[_m.end():]
        st = strip(text)
        use_spans = [(u['start'], u['end']) for u in find_uses(text)]
        fam_names = r.homes.get(module, {})
        # db_x::Name / crate::db_x::Name
        for m in re.finditer(r'(?<![A-Za-z0-9_:])(crate::)?(db_[a-z0-9_]+)::((?:writer::|release::)*)([A-Za-z_][A-Za-z0-9_]*)', st):
            if in_use(m.start()):
                continue
            ident = m.group(2)
            if ident not in ALL_MODULE_IDENTS:
                continue
            name = m.group(4)
            if ident == module:
                tgt = home_path(name) if name in fam_names else None
                if tgt is None:
                    continue
                edits.append((m.start(), m.end(), f'{tgt}::{name}'))
            elif r.split(ident):
                edits.append((m.start(), m.end(), f'{r.lib(ident, name)}::{name}'))
            else:
                if m.group(1) or m.group(3):
                    edits.append((m.start(), m.end(), f'{ident}::{name}'))
        # flattened sub-module paths: writer::X, writer::release::X, release::X
        for m in re.finditer(r'(?<![A-Za-z0-9_:])(?:crate::)?claimed_backend::', st):
            if in_use(m.start()):
                continue
            edits.append((m.start(), m.end(), 'crate::claimed_backend::' if lib == 'db_io_core' else 'db_io_core::claimed_backend::'))
        sub_fam = r.homes.get('db_storage', {})
        for m in re.finditer(r'(?<![A-Za-z0-9_:])((?:writer::)?(?:release::)|writer::)([A-Za-z_][A-Za-z0-9_]*)', st):
            if in_use(m.start()):
                continue
            name = m.group(2)
            if name in sub_fam:
                h = sub_fam[name]
                edits.append((m.start(), m.end(), f'crate::{name}' if h == lib else f'{h}::{name}'))
        # super::X chains in flattened sub-file items are handled by the caller (they depend on the origin file)
        for m in re.finditer(r'(?<![A-Za-z0-9_])(?:super::)+(?:writer::|release::)+([A-Za-z_][A-Za-z0-9_]*)', st):
            if in_use(m.start()) or module != 'db_storage':
                continue
            name = m.group(1)
            if name in fam_names:
                tgt = home_path(name)
                edits.append((m.start(), m.end(), f'{tgt}::{name}'))
        # crate::Name
        for m in re.finditer(r'(?<![A-Za-z0-9_])crate::(?!db_[a-z0-9_]+::)([A-Za-z_][A-Za-z0-9_]*)', st):
            if in_use(m.start()):
                continue
            name = m.group(1)
            if name in fam_names:
                tgt = home_path(name)
                edits.append((m.start(), m.end(), f'{tgt}::{name}'))
            elif name in r.scope.root and name not in ctx.local_defs:
                edits.append((m.start(), m.end(), '::'.join(r.root(name))))
        # db::fam::Name (crate alias)
        if ctx.crate_alias:
            pat = re.compile(r'(?<![A-Za-z0-9_:])' + re.escape(ctx.crate_alias) + r'::([A-Za-z_][A-Za-z0-9_]*)(::[A-Za-z_][A-Za-z0-9_]*)?')
            for m in pat.finditer(st):
                if in_use(m.start()):
                    continue
                rest = (m.group(1),) + ((m.group(2)[2:],) if m.group(2) else ())
                np = map_family(rest, ctx)
                if np is not None and len(np) >= len(rest):
                    edits.append((m.start(), m.end(), '::'.join(np)))
        final = []
        for e in sorted(edits):
            if final and e[0] < final[-1][1]:
                continue
            final.append(e)
        for s, e, t in sorted(final, reverse=True):
            text = text[:s] + t + text[e:]
        st2 = strip(text)
        selfs = [m for m in re.finditer(r'(?<![A-Za-z0-9_:])' + re.escape(lib) + r'::', st2)]
        for m in reversed(selfs):
            text = text[:m.start()] + 'crate::' + text[m.end():]
        return text

    def flatten_super(self, body, fid, lib, module):
        """Inside sub-file items, `super::X` / `super::super::X` meant the parent module: point at the flattened homes."""
        r = self.resolver
        fam = r.homes.get(module, {})

        def repl(m):
            name = m.group(2)
            if name in fam:
                h = fam[name]
                return (f'crate::{name}' if h == lib else f'{h}::{name}')
            return m.group(0)
        return re.sub(r'(?<![A-Za-z0-9_])((?:super::)+)(?:(?:writer|release)::)*([A-Za-z_][A-Za-z0-9_]*)', repl, body)

    # ---------------------------------------------------------------- traits
    def trait_methods(self):
        """trait name -> (module, set of method names) over the whole db tree."""
        if hasattr(self, '_traits'):
            return self._traits
        out = {}
        for fid, s in self.srcs.items():
            for it in s.items:
                if it.kind == 'trait':
                    body = '\n'.join(strip(s.text).split('\n')[it.start:it.end + 1])
                    ms = set(re.findall(r'^\s{4}(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)', body, re.M))
                    out[it.name] = (s.module, ms)
        self._traits = out
        return out

    # ---------------------------------------------------------------- imports
    def source_leaves(self, module):
        """Private top-level use leaves of the family files: (path, alias, glob, attrs, origin fid)."""
        out = []
        for fid in self.family(module):
            s = self.srcs[fid]
            for i, it in enumerate(s.items):
                if it.kind != 'use' or it.vis.startswith('pub'):
                    continue
                decl = '\n'.join(s.lines[it.header:it.end + 1])
                m = re.match(r'use[ \t\n]+(.*?);\s*$', decl, re.S)
                if not m:
                    continue
                tree = re.sub(r'\s+', ' ', m.group(1)).strip()
                attrs = tuple(a.strip() for a in it.attrs)
                for path, alias, glob in parse_use(tree):
                    if len(path) == 1 and not glob and path[0] not in ('std', 'core', 'alloc') and (path[0] in self.scope.root or path[0] in self.resolver.homes.get(module, {})):
                        continue
                    out.append((path, alias, glob, attrs, fid))
        return out

    def file_defs(self, fid):
        s = self.srcs[fid]
        names = set(s.defs)
        for it in s.items:
            if it.kind == 'use' and it.vis.startswith('pub'):
                decl = '\n'.join(s.lines[it.header:it.end + 1])
                mm = re.match(r'(?:pub(?:\([^)]*\))?\s+)?use[ \t\n]+(.*?);\s*$', decl, re.S)
                if mm:
                    for p, a, g in parse_use(re.sub(r'\s+', ' ', mm.group(1)).strip()):
                        if not g:
                            names.add(a or p[-1])
        return names

    def build_imports(self, lib, modules, body, tests, local_defs, extra_leaves=(), pre_bound=(), only_extra=False, scoped=None, kit=()):
        """Import map {bound key: (path, alias, attrs)} for a composed file. `modules`: source module idents contributing to it.
        `scoped`: [(scope fid, text)] -> family names are resolved per origin file scope."""
        r = self.resolver
        if isinstance(modules, str):
            modules = [modules]
        nd = lambda t: idents(re.sub(r'(?<!\.)\.\s*[A-Za-z_]\w*', '.', strip(t)))
        body_s = strip(body)
        body_normal, body_gated = gated_split(body)
        body_ids = nd(body_normal)
        body_gated_ids = nd(body_gated) if body_gated else set()
        test_ids = set()
        test_calls = set()
        for t in tests:
            ts = strip(t)
            test_ids |= nd(t)
            test_calls |= set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::\s*<[^>]*>\s*)?\(', ts))
        kit_ids = set()
        for t in kit:
            kit_ids |= nd(t)
            test_calls |= set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::\s*<[^>]*>\s*)?\(', strip(t)))
        used_all = body_ids | test_ids | body_gated_ids | kit_ids
        fam_used = {}
        if scoped is None:
            for m in modules:
                for n in used_all:
                    if n in r.homes.get(m, {}):
                        fam_used[n] = r.homes[m][n]
        else:
            for sfid, text in scoped:
                m = self.srcs[sfid].module
                fam_m = r.homes.get(m, {})
                defs = self.file_defs(sfid)
                for n in nd(text):
                    if n in defs and n in fam_m:
                        fam_used[n] = fam_m[n]
        calls_body = set(re.findall(r'\.\s*([A-Za-z_]\w*)\s*(?:::\s*<[^>]*>\s*)?\(', body_s))
        kept = {}
        conflicts = []

        def add(path, alias, attrs, bn, only_tests):
            if only_tests and not any('cfg(' in a for a in attrs):
                attrs = attrs + (('#[cfg(any(test, feature = "testkit"))]',) if bn in kit_ids else ('#[cfg(test)]',))
            if bn in kept and kept[bn][0] != path:
                conflicts.append((bn, kept[bn][0], path))
                return
            if bn in kept:
                if kept[bn][2] and not attrs:
                    kept[bn] = (path, alias, attrs)
                return
            kept[bn] = (path, alias, attrs)

        leaves = list(extra_leaves) if only_extra else [lf for m in modules for lf in self.source_leaves(m)] + list(extra_leaves)
        for path, alias, glob, attrs, fid in leaves:
            lm = self.srcs[fid].module if fid in self.srcs else modules[0]
            fam = r.homes.get(lm, {})
            ctx = FileCtx(r, lm, local_defs)
            if glob and path == ('crate',):
                continue
            if path[0] == 'super':
                rest = path[1:]
                while rest and rest[0] == 'super':
                    rest = rest[1:]
                rest = tuple(x for x in rest if x not in SUBMODS)
                if not rest or glob:
                    continue
                if rest[0] in fam:
                    path = ('crate',) + rest if fam[rest[0]] == lib else (fam[rest[0]],) + rest
                    if path[0] == 'crate':
                        continue
                else:
                    path = ('crate',) + rest
            if path == ('crate',) and alias:
                continue
            np = map_path(path + ('*',), ctx)[:-1] if glob else map_path(path, ctx)
            if np is None:
                continue
            if len(np) >= 2 and np[-1] == 'self' and not glob:
                np = np[:-1]
            if np[0] == 'crate':
                continue
            if glob:
                add(np + ('*',), alias, attrs, '::'.join(np) + '::*', False)
                continue
            bn = alias or np[-1]
            if np[0] == lib:
                continue
            if bn == '_':
                add(np, alias, attrs, '_' + leaf_text(np, alias, False), False)
                continue
            if bn not in used_all:
                continue
            add(np, alias, attrs, bn, bn not in body_ids)
        bound = set(kept) | set(pre_bound)
        for n in sorted(fam_used):
            if n not in local_defs and n not in bound and fam_used[n] != lib:
                add((fam_used[n], n), None, (), n, n not in body_ids)
        bound = set(kept) | set(pre_bound)
        allfam = set()
        for m in modules:
            allfam |= set(r.homes.get(m, {}))
        for n in sorted(used_all if not only_extra else set()):
            if n in r.scope.root_for(lib) and n not in local_defs and n not in bound and n not in allfam:
                p = r.root(n)
                if p[0] != lib:
                    add(p, None, (), n, n not in body_ids)
        bound = {k for k in kept if not k.startswith('_')} | set(pre_bound)
        traits = self.trait_methods()
        for tname, (tmod, methods) in traits.items():
            if not (methods & (calls_body | test_calls)):
                continue
            if tmod in modules and tname in r.homes.get(tmod, {}):
                home = r.homes[tmod][tname]
                if home == lib or tname in bound:
                    continue
                order = ORDERS.get(tmod) or []
                if lib in order and home in order and order.index(home) > order.index(lib):
                    self.report['upward-trait-skipped'].append((lib, tname, home))
                    continue
                add((home, tname), '_', (), '_' + home + tname, not (methods & calls_body))
            elif tname in r.scope.root and tname not in bound and tname not in local_defs:
                p = r.root(tname)
                if p[0] != lib:
                    add(p, '_', (), '_' + tname, not (methods & calls_body))
        return kept, conflicts

    def format_kept(self, kept):
        groups = collections.defaultdict(list)
        for bn, (path, alias, attrs) in kept.items():
            groups[attrs].append((path, alias))
        stmts = []
        for attrs, leaves in groups.items():
            by_head = collections.defaultdict(list)
            for path, alias in leaves:
                by_head[path[0]].append((path, alias))
            for head in sorted(by_head):
                items = by_head[head]
                singles = [(p, a) for p, a in items if len(p) == 1]
                multi = [(p, a) for p, a in items if len(p) > 1]
                for p, a in singles:
                    stmts.append((attrs, f'use {leaf_text(p, a, False)};'))
                if multi:
                    inner = sorted(leaf_text(p[1:], a, False) for p, a in multi)
                    inner = [x.replace('::*::*', '::*') for x in inner]
                    stmts.append((attrs, f'use {head}::' + (inner[0] if len(inner) == 1 else '{' + ', '.join(inner) + '}') + ';'))
        stmts.sort(key=lambda x: (x[0], x[1]))
        fmt = format_uses([t for a, t in stmts])
        out = []
        for (attrs, _), f in zip(stmts, fmt):
            out.extend(attrs)
            out.extend(f.split('\n'))
        return out

    # ---------------------------------------------------------------- whole-module crates
    def compose_whole(self, lib):
        r = self.resolver
        module = lib
        files = {}
        fids = [fid for fid, s in self.srcs.items() if s.module == module]
        main = [f for f in fids if self.srcs[f].role == 'main'][0]
        local = set(self.scope.defs[module])
        all_ids = set()
        for f in fids:
            all_ids |= idents(self.srcs[f].stripped)
        explicit = set()
        for u in find_uses(self.srcs[main].text):
            for p, a, g in parse_use(u['tree']):
                if not g:
                    explicit.add(a or p[-1])
        root_names = {n for n in all_ids if n in r.scope.root_for(lib) and n not in local and n not in explicit}
        alias = None
        for u in find_uses(self.srcs[main].text):
            for p, a, g in parse_use(u['tree']):
                if p == ('crate',) and a:
                    alias = a
        for f in fids:
            s = self.srcs[f]
            ctx = FileCtx(r, module, local, crate_alias=alias)
            t = s.text
            file_ids = all_ids if f == main else idents(s.stripped)
            t = rewrite_uses(t, ctx, file_ids, root_names if f == main else set())
            t = re.sub(r'(process_isolated_law\(\s*")db_[a-z0-9_]+::', lambda m: m.group(1), t)
            t = self.rewrite_code(t, ctx, lib, module)
            if f == main:
                head = []
                if 'async fn' in t:
                    head.append('#![allow(async_fn_in_trait)]')
                if lib in GATED_LIBS:
                    head.append('#![cfg(not(target_arch = "wasm32"))]')
                head.extend(extern_alias_lines(strip(t)))
                if head:
                    ls = t.split('\n')
                    k = 0
                    while k < len(ls) and ls[k].startswith('//!'):
                        k += 1
                    t = '\n'.join(ls[:k] + head + ([''] if k < len(ls) and ls[k].strip() else []) + ls[k:])
            files[f] = t
        return files

    # ---------------------------------------------------------------- chunk crates
    def test_dir_name(self, tf, taken):
        d = os.path.basename(os.path.dirname(tf))
        if d in taken and taken[d] != tf:
            sub = os.path.basename(os.path.dirname(os.path.dirname(os.path.dirname(tf))))
            d = f'{d}-{re.sub(r"^[^A-Za-z0-9]+", "", sub)}'
        taken[d] = tf
        return d

    def compose_chunk(self, lib):
        crate = BY_LIB[lib]
        r = self.resolver
        files = {}
        node = crate.node
        root_rel = f'{node}/🦀️.rs'
        dest_dir = os.path.dirname(os.path.join(PRISTINE, root_rel))
        contributing = []
        for m in [crate.module] + sorted({s.module for s in self.srcs.values()} - {crate.module}):
            for fid in self.family(m):
                if any(l == lib for l in self.A.get(fid, {}).values()):
                    contributing.append(fid)
        modules = list(dict.fromkeys(self.srcs[f].module for f in contributing))
        body_lines = []
        parts = []
        local_defs = set()
        for fid in contributing:
            s = self.srcs[fid]
            idxs = sorted(i for i, l in self.A.get(fid, {}).items() if l == lib)
            for i in idxs:
                it = s.items[i]
                if it.kind not in ('impl', 'use') and it.name and it.name != '_':
                    local_defs.add(it.name)
                if it.kind == 'use':
                    decl = '\n'.join(s.lines[it.header:it.end + 1])
                    mm = re.match(r'(?:pub(?:\([^)]*\))?\s+)?use[ \t\n]+(.*?);\s*$', decl, re.S)
                    if mm:
                        for p, a, g in parse_use(re.sub(r'\s+', ' ', mm.group(1)).strip()):
                            if not g:
                                local_defs.add(a or p[-1])
        for fid in contributing:
            s = self.srcs[fid]
            idxs = sorted(i for i, l in self.A.get(fid, {}).items() if l == lib)
            seg = self.emit_items(fid, idxs, dest_dir)
            text = '\n'.join(seg)
            if s.role == 'sub':
                text = self.flatten_super(text, fid, lib, s.module)
            text = self.rewrite_code(text, FileCtx(r, s.module, local_defs), lib, s.module)
            parts.append((fid, text))
            if body_lines and body_lines[-1].strip():
                body_lines.append('')
            body_lines.extend(text.split('\n'))
        body = '\n'.join(body_lines)
        tests = []
        taken = {}
        used_mods = set()
        for module in modules:
            for tf in TEST_FILES.get(module, []):
                idxs = sorted(i for i, libs in self.T.get(tf, {}).items() if lib in libs)
                if not idxs:
                    continue
                tdir = self.test_dir_name(tf, taken)
                trel = f'{node}/🧪️tests/{tdir}/🦀️.rs'
                tdest = os.path.dirname(os.path.join(PRISTINE, trel))
                s = self.srcs[tf]
                tbody = '\n'.join(self.emit_items(tf, idxs, tdest))
                tdef = {s.items[i].name for i in idxs if s.items[i].kind not in ('impl', 'use') and s.items[i].name}
                tbody = self.rewrite_code(tbody, FileCtx(r, module, tdef), lib, module)
                mod_name = None
                for nm, cfid, is_test in self.srcs[s.parent].mounts:
                    if cfid == tf:
                        mod_name = nm
                base = mod_name or 'tests'
                if base in used_mods:
                    base = re.sub(r'[^A-Za-z0-9_]+', '_', os.path.basename(os.path.dirname(trel))).strip('_').lower() + '_tests'
                    base = re.sub(r'^_+', '', base) or f'tests_{len(used_mods)}'
                    if not re.match(r'[a-z]', base):
                        base = 't_' + base
                used_mods.add(base)
                tbody = self.fix_labels(tbody, mod_name or 'tests', base)
                tests.append(dict(tf=tf, rel=trel, body=tbody, defs=tdef, mod=base, idxs=idxs, module=module))
        tk_body = None
        tk = None
        for module in modules:
            spec = self.testkit.get(module)
            if spec and spec['lib'] == lib:
                tk = spec
                ts = self.srcs[tk['file']]
                trel = f'{node}/🧰️testkit/🦀️.rs'
                tdest = os.path.dirname(os.path.join(PRISTINE, trel))
                tk_body = '\n'.join(self.emit_items(tk['file'], tk['idxs'], tdest))
                for e in tk['exports']:
                    tk_body = re.sub(r'(?m)^(pub\(crate\) )?(async fn|fn) ' + re.escape(e) + r'\b', lambda m: 'pub ' + m.group(2) + ' ' + e, tk_body)
                tk_defs = {ts.items[i].name for i in tk['idxs']}
                tk_body = self.rewrite_code(tk_body, FileCtx(r, module, tk_defs), lib, module)
        test_texts = [t['body'] for t in tests]
        kit_texts = [tk_body] if tk_body is not None else []
        scoped = parts + [(self.srcs[t['tf']].parent, t['body']) for t in tests] + ([(self.srcs[tk['file']].parent, tk_body)] if tk_body is not None else [])
        kept, conflicts = self.build_imports(lib, modules, body, test_texts, local_defs, scoped=scoped, kit=kit_texts)
        if conflicts:
            self.report['import-conflicts'].append((lib, conflicts))
        all_ids = idents(strip(body))
        for t in test_texts + kit_texts:
            all_ids |= idents(strip(t))
        body = rewrite_uses(body, FileCtx(r, modules[0], local_defs), all_ids, set())
        head = []
        owner_main = [f for f in contributing if self.srcs[f].role == 'main' and self.srcs[f].module == crate.module]
        attrs_in = []
        if crate.doc_owner and owner_main:
            gap = []
            for it in self.srcs[owner_main[0]].items[:1]:
                gap = [g for g in it.gap_before if g.startswith('//!') or g.startswith('#![')]
            head.extend(g for g in gap if g.startswith('//!'))
            attrs_in = [g for g in gap if g.startswith('#![')]
        else:
            head.append(f'//! {EMOJI[lib]}️ {crate.desc}.')
        inner = list(dict.fromkeys(attrs_in))
        if 'async fn' in body and not any('async_fn_in_trait' in a for a in inner):
            inner.append('#![allow(async_fn_in_trait)]')
        if lib in GATED_LIBS:
            inner.append('#![cfg(not(target_arch = "wasm32"))]')
        head.extend(inner)
        head.extend(extern_alias_lines(strip(body) + '\n'.join(strip(t) for t in test_texts + kit_texts)))
        imp = self.format_kept(kept)
        out = head + [''] + imp + ([''] if imp else []) + body.split('\n')
        if tk_body is not None:
            out += ['', '#[cfg(any(test, feature = "testkit"))]', '#[path = "🧰️testkit/🦀️.rs"]', 'pub mod testkit;']
        if tests:
            out += ['', '//#region 🧪️Tests']
            for t in tests:
                rel_path = os.path.relpath(os.path.join(PRISTINE, t['rel']), dest_dir)
                out += ['#[cfg(test)]', f'#[path = "{rel_path}"]', f'mod {t["mod"]};']
            out.append('//#endregion 🧪️Tests')
        files[root_rel] = '\n'.join(out).rstrip('\n') + '\n'
        if tk_body is not None:
            tkept, tconf = self.build_imports(lib, tk and self.srcs[tk['file']].module, tk_body, [], tk_defs, extra_leaves=[(p, a, g, at, tk['file']) for (p, a, g, at) in self.leaves_of(tk['file'])], pre_bound=set(), only_extra=True, scoped=[(self.srcs[tk['file']].parent, tk_body)])
            tb = rewrite_uses(tk_body, FileCtx(r, self.srcs[tk['file']].module, tk_defs), idents(strip(tk_body)), set())
            lines = ['//! 🧰️ Shared WAL test fixtures for the other db crates (behind the `testkit` feature).', '', 'use super::*;'] + ([''] if tkept else []) + self.format_kept(tkept) + ['']
            files[f'{node}/🧰️testkit/🦀️.rs'] = '\n'.join(lines + tb.split('\n')).rstrip('\n') + '\n'
        for m in re.finditer(r'include!\("([^"]+)"\)', body):
            inc = os.path.normpath(os.path.join(dest_dir, m.group(1)))
            if os.path.exists(inc):
                itext = read(inc)
                itext = self.flatten_super(itext, None, lib, 'db_storage')
                itext = self.rewrite_code(itext, FileCtx(r, 'db_storage', set()), lib, 'db_storage')
                files[os.path.relpath(inc, PRISTINE)] = itext
        for t in tests:
            extra = [(p, a, g, at, t['tf']) for (p, a, g, at) in self.leaves_of(t['tf'])]
            if re.search(r'(?<![A-Za-z0-9_:])pack_corruption::', strip(t['body'])):
                extra.append((('semio_framework_pack', 'corruption_testing'), 'pack_corruption', False, ('#[cfg(test)]',), t['tf']))
            tkept, tconf = self.build_imports(lib, t['module'], t['body'], [], t['defs'], extra_leaves=extra, pre_bound=set(), only_extra=True, scoped=[(self.srcs[t['tf']].parent, t['body'])])
            if tconf:
                self.report['import-conflicts'].append((lib + ':test', tconf))
            lines = ['use super::*;'] + ([''] if tkept else []) + self.format_kept(tkept) + ['']
            tb = rewrite_uses(t['body'], FileCtx(r, t['module'], t['defs']), idents(strip(t['body'])), set())
            files[t['rel']] = '\n'.join(lines + tb.split('\n')).rstrip('\n') + '\n'
        return files

    def leaves_of(self, fid):
        s = self.srcs[fid]
        out = []
        for i, it in enumerate(s.items):
            if it.kind != 'use' or it.vis.startswith('pub'):
                continue
            decl = '\n'.join(s.lines[it.header:it.end + 1])
            m = re.match(r'use[ \t\n]+(.*?);\s*$', decl, re.S)
            if not m:
                continue
            tree = re.sub(r'\s+', ' ', m.group(1)).strip()
            attrs = tuple(a.strip() for a in it.attrs)
            for path, alias, glob in parse_use(tree):
                if path == ('super',) and glob:
                    continue
                if len(path) == 1 and not glob and path[0] in self.scope.root:
                    continue
                out.append((path, alias, glob, attrs))
        return out

    # ---------------------------------------------------------------- all
    def run(self):
        out = {}
        for c in CRATES:
            if c.kind == 'whole':
                out.update(self.compose_whole(c.lib))
            else:
                out.update(self.compose_chunk(c.lib))
        return out
