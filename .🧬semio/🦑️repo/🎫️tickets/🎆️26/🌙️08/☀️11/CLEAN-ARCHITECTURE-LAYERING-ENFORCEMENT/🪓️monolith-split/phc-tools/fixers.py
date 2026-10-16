#!/usr/bin/env python3
"""🩹️ Post-rewrite fixers of the plugin-host carve: bindgen-opaque names, crate-root names of the monolith, region tidy."""
import os, sys, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import hcmod, carve
from carve import inline_descendants, free_idents, attached_start

WIT_IDENT = 'semio_framework_plugin_host_wit'
OPAQUE = ['actor_bindings', 'wit_capabilities', 'wit_effects', 'wit_events', 'wit_host_async', 'wit_lifetime', 'wit_types', 'wit_ui', 'wit_jobs', 'wit_reactor']
ROOT_USE = {
    'operation_authority': 'use semio_framework_plugin_guest_contract::operation_authority;',
    'original_actor_context': 'use semio_framework_plugin_guest_contract::original_actor_context;',
}
EXTERN_ALIAS = ['dsl', 'protocol', 'store']


def binds(S, X, seen=None):
    seen = seen or set()
    if id(S) in seen:
        return False
    seen = seen | {id(S)}
    if X in S.items:
        return True
    for u in S.uses:
        if not u.glob and u.binding == X:
            return True
    for u in S.uses:
        if u.glob:
            r = hcmod.resolve(S, u.path, None, True)
            if r is not None and r.kind == 'mod' and r.mod is not None and binds(r.mod, X, seen):
                return True
    return False


def scope_at(fm, i):
    best = fm
    for c in inline_descendants(fm):
        a, b = c.item.tok_range
        if a <= i < b:
            best = c
    return best


def uses_of(fm, names, need_colons=True):
    """{scope: {name}} for identifiers that start a path (followed by `::`, not preceded by `::` or `.`)."""
    toks = fm.toks
    out = collections.defaultdict(set)
    for i, t in enumerate(toks):
        if t.k != 'id' or t.x not in names:
            continue
        prev = toks[i - 1].x if i else ''
        prev2 = toks[i - 2].x if i > 1 else ''
        if prev == '.' or (prev == ':' and prev2 == ':'):
            continue
        if prev in ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'mod', 'let', 'mut', 'ref', 'as'):
            continue
        nx = toks[i + 1].x if i + 1 < len(toks) else ''
        nx2 = toks[i + 2].x if i + 2 < len(toks) else ''
        if need_colons and not (nx == ':' and nx2 == ':'):
            continue
        out[id(scope_at(fm, i))].add(t.x)
    return out


def insertion_point(fm, S, src):
    if S.uses:
        last = max((u.item for u in S.uses), key=lambda it: it.b)
        pos = last.b
        while pos < len(src) and src[pos] != '\n':
            pos += 1
        return pos + 1, ''
    if S.inline:
        body = S.item.toks_all
        k = next(idx for idx, t in enumerate(body) if t.x == '{')
        return body[k].b, '\n'
    c = carve.Carver.header_end
    return c(None, src, fm.toks), '\n'


def fix_opaque(ws, C, roots):
    """Adds the `use` of every bindgen-opaque name a scope uses but does not bind."""
    C.build_new(roots)
    n = 0
    log = []
    for ident, root in C.new.roots.items():
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            scopes = {id(S): S for S in [fm] + list(inline_descendants(fm))}
            patches = []
            for sid, used in uses_of(fm, set(OPAQUE)).items():
                S = scopes[sid]
                C.activate(C.new)
                missing = sorted(x for x in used if not binds(S, x))
                if not missing:
                    continue
                if ident == WIT_IDENT:
                    names = [f'crate::{x}' for x in missing]
                else:
                    names = [f'{WIT_IDENT}::{x}' for x in missing]
                text = ''.join(f'use {p};\n' for p in names)
                pos, lead = insertion_point(fm, S, fm.src)
                patches.append((pos, pos, lead + text))
                log.append((fm.file, ident, missing))
            if patches:
                out = fm.src
                for a, b, r in sorted(patches, key=lambda p: -p[0]):
                    out = out[:a] + r + out[b:]
                ws.write(fm.file, out)
                n += 1
    return n, log


def fix_root_names(ws, C, roots, extern_deps):
    """Crate roots of the carve declare the monolith-root names their files use: `crate::operation_authority`, `crate::original_actor_context` and the `dsl`/`protocol`/`store` aliases of the kernel crate."""
    C.build_new(roots)
    changed = []
    for ident, root in C.new.roots.items():
        used_names = set()
        used_alias = set()
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            toks = fm.toks
            for i, t in enumerate(toks):
                if t.k != 'id':
                    continue
                prev = toks[i - 1].x if i else ''
                prev2 = toks[i - 2].x if i > 1 else ''
                nx = toks[i + 1].x if i + 1 < len(toks) else ''
                nx2 = toks[i + 2].x if i + 2 < len(toks) else ''
                if t.x in ROOT_USE and nx == ':' and nx2 == ':' and prev == ':' and prev2 == ':' and i > 2 and toks[i - 3].x == 'crate':
                    used_names.add(t.x)
                if t.x in EXTERN_ALIAS and nx == ':' and nx2 == ':' and not (prev == ':' and prev2 == ':') and prev != '.':
                    used_alias.add(t.x)
        if not used_names and not used_alias:
            continue
        src = root.src
        add = []
        for x in sorted(used_alias):
            if not any(it.kind == 'extern_crate' and it.name == x for it in root.all_items):
                add.append(f'extern crate semio_framework_os_kernel as {x};')
        for x in sorted(used_names):
            if x not in {u.binding for u in root.uses}:
                add.append(ROOT_USE[x])
        if not add:
            continue
        pos = carve.Carver.header_end(None, src, root.toks)
        ws.write(root.file, src[:pos] + '\n' + '\n'.join(add) + '\n' + src[pos:])
        changed.append((ident, add))
    return changed


REGION = re.compile(r'^(\s*)//#(end)?region(?:\s+(.*?))?\s*$')


def tidy_regions(text):
    """Merges `//#endregion X` + blank lines + `//#region X` into one span and drops empty region pairs."""
    lines = text.split('\n')
    changed = True
    while changed:
        changed = False
        # empty pair
        i = 0
        out = []
        while i < len(lines):
            m = REGION.match(lines[i])
            if m and not m.group(2):
                j = i + 1
                while j < len(lines) and lines[j].strip() == '':
                    j += 1
                m2 = REGION.match(lines[j]) if j < len(lines) else None
                if m2 and m2.group(2) and (m2.group(3) or '').strip() == (m.group(3) or '').strip():
                    i = j + 1
                    changed = True
                    while out and out[-1].strip() == '' and i < len(lines) and lines[i].strip() == '':
                        i += 1
                    continue
            out.append(lines[i])
            i += 1
        lines = out
        # adjacent end/start of same name
        out = []
        i = 0
        while i < len(lines):
            m = REGION.match(lines[i])
            if m and m.group(2):
                j = i + 1
                while j < len(lines) and lines[j].strip() == '':
                    j += 1
                m2 = REGION.match(lines[j]) if j < len(lines) else None
                if m2 and not m2.group(2) and (m2.group(3) or '').strip() == (m.group(3) or '').strip():
                    i = j + 1
                    changed = True
                    continue
            out.append(lines[i])
            i += 1
        lines = out
    return re.sub(r'\n{3,}', '\n\n', '\n'.join(lines))


def fix_async_trait_allow(ws, C, roots):
    """A crate root carries the monolith's `#![allow(async_fn_in_trait)]` when any of its files declares a trait with an `async fn`."""
    C.build_new(roots)
    changed = []
    for ident, root in C.new.roots.items():
        has = False
        for fm in hcmod.walk(root):
            if fm.inline or getattr(fm, 'missing', False):
                continue
            for S in [fm] + list(inline_descendants(fm)):
                for it in S.all_items:
                    if it.kind == 'trait':
                        ts = it.toks_all
                        if any(ts[i].x == 'async' and ts[i + 1].x == 'fn' for i in range(len(ts) - 1)):
                            has = True
        if has and '#![allow(async_fn_in_trait)]' not in root.src:
            src = root.src
            pos = carve.Carver.header_end(None, src, root.toks)
            ws.write(root.file, src[:pos] + '\n#![allow(async_fn_in_trait)]\n' + src[pos:])
            changed.append(ident)
    return changed


def prune_unused_uses(ws, C, roots, file):
    """Drops the leaves of the plain `use` items of one file whose binding no token of the file names and no mounted child imports through `super`."""
    C.build_new(roots)
    fm = C.new.by_file[file]
    use_ranges = [u.item.tok_range for S in [fm] + list(inline_descendants(fm)) for u in S.uses]
    ids = {t.x for i, t in enumerate(fm.toks) if t.k == 'id' and not any(a <= i < b for a, b in use_ranges)}
    ids |= {name for _, name in carve.fmt_arg_idents(fm.toks)}
    keep_all = False
    for child in fm.children.values():
        if child.inline or getattr(child, 'missing', False):
            continue
        for u in child.uses:
            if u.path and u.path[0] == 'super':
                if u.glob and len(u.path) == 1:
                    keep_all = True
                elif len(u.path) >= 2:
                    ids.add(u.path[1])
    patches = []
    by_item = collections.OrderedDict()
    for u in fm.uses:
        by_item.setdefault(id(u.item), (u.item, []))[1].append(u)
    for _, (it, leaves) in by_item.items():
        if it.vis.startswith('pub') or keep_all:
            continue
        keep, changed = [], False
        for u in leaves:
            if u.glob or u.binding in ids or u.binding == '_':
                keep.append((u.path, None if u.binding == u.path[-1] else u.binding, u.glob))
            else:
                changed = True
        if changed:
            a = attached_start(fm.src, it)
            text = carve.render_use(it.attrs, it.vis, keep) if keep else ''
            patches.append((a, it.b + (1 if it.b < len(fm.src) and fm.src[it.b] == '\n' and not keep else 0), text))
    out = fm.src
    for a, b, r in sorted(patches, key=lambda p: -p[0]):
        out = out[:a] + r + out[b:]
    ws.write(file, out)
    return len(patches)


EXT_TRAITS = {
    'ShardTransport': ('semio_framework_actor::ShardTransport', {'send', 'recv', 'heartbeat', 'kill'}),
    'Read': ('std::io::Read', {'read', 'read_to_end', 'read_exact', 'read_to_string'}),
    'Write': ('std::io::Write', {'write', 'write_all', 'flush', 'write_fmt'}),
}


IGNORE_METHODS = {'GuestRelayWakeReceiptReceiver': {'collect'}, 'GuestRuntime': {'checkpoint'}}


def _glob_names(m, seen=None):
    seen = seen or set()
    out = {}
    for u in m.uses:
        if u.glob:
            r = hcmod.resolve(m, u.path, None, True)
            if r is not None and r.kind == 'mod' and r.mod is not None and id(r.mod) not in seen:
                tgt = r.mod
                seen2 = seen | {id(tgt)}
                for n, its in tgt.items.items():
                    out.setdefault(n, ('item', tgt, its[0]))
                for uu in tgt.uses:
                    if not uu.glob:
                        out.setdefault(uu.binding, ('use', tgt, uu))
                for n, v in _glob_names(tgt, seen2).items():
                    out.setdefault(n, v)
    return out


def fix_trait_imports(ws, C, roots):
    """Re-imports the traits a file reached through a dropped glob and whose methods it still calls."""
    C.build_new(roots)
    added = []
    for file, fm in list(C.new.by_file.items()):
        if getattr(fm, 'missing', False):
            continue
        toks = fm.toks
        ids = {t.x for t in toks if t.k == 'id'}
        methods = {toks[i].x for i in range(1, len(toks)) if toks[i - 1].x == '.' and toks[i - 1].k == 'p' and toks[i].k == 'id'}
        if not methods:
            continue
        C.activate(C.base)
        old_names = {}
        for om in C.old_mods_for(fm):
            for n, v in _glob_names(om).items():
                old_names.setdefault(n, v)
        lines = []
        for n, (kind, tgt, obj) in old_names.items():
            if not n[:1].isupper() or n in ids:
                continue
            C.activate(C.new)
            if binds(fm, n):
                continue
            path = None
            if kind == 'item' and obj.kind == 'trait':
                ms = {obj.toks[i + 1].x for i, t in enumerate(obj.toks) if t.x == 'fn' and i + 1 < len(obj.toks)}
                if (ms - IGNORE_METHODS.get(n, set())) & methods:
                    C.activate(C.base)
                    r = hcmod.lookup(tgt, n)
                    nd = C.map_def(carve.def_of(r))
                    segs, _ = C.newpath(fm, nd)
                    path = '::'.join(segs)
            elif n in EXT_TRAITS and EXT_TRAITS[n][1] & methods:
                path = EXT_TRAITS[n][0]
            if path:
                lines.append(f'use {path} as _;')
                added.append((os.path.relpath(file, carve.REPO), n))
        if lines:
            src = fm.src
            pos, lead = insertion_point(fm, fm, src)
            ws.write(file, src[:pos] + lead + '\n'.join(lines) + '\n' + src[pos:])
    C.activate(C.new)
    return added
