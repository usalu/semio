"""✍️ Source rewriting for the crate carve: use statements, crate-root glob, qualified paths.

Pure text transformation. `Resolver` knows which new crate hosts each original (module, name).
"""
import os
import re
import subprocess
import tempfile

from lib import *
from rs import strip, idents
from scope import Scope, FACADE_FAMILY, ALL_MODULE_IDENTS

USE_RE = re.compile(r'(?m)^([ \t]*)((?:pub(?:\([^)]*\))?[ \t]+)?)use[ \t\n]+([^;]*);')
STD_ROOTS = ('std', 'core', 'alloc')
RUSTFMT = ['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config-path', f'{REPO}/rustfmt.toml', '--config', 'unstable_features=true,skip_children=true', '--emit', 'stdout']


UNKNOWN = set()


class Resolver:
    """Maps (module ident, name) of the ORIGINAL tree to the lib name of the crate that now hosts it."""

    def __init__(self, scope, homes=None, reexports=None):
        self.scope = scope
        self.homes = homes or {}
        self.reexports = reexports or {}

    def split(self, module_ident):
        return module_ident in self.homes

    def lib(self, module_ident, name):
        h = self.homes.get(module_ident)
        if h is None:
            return module_ident
        if name in h:
            return h[name]
        if '*' in h:
            return h['*']
        if name in self.reexports.get(module_ident, {}):
            return self.reexports[module_ident][name]
        UNKNOWN.add((module_ident, name))
        return module_ident

    def root(self, name):
        kind, target = self.scope.root[name]
        if kind == 'ext':
            return tuple(target.split('::'))
        return (self.lib(target, name), name)


class FileCtx:
    def __init__(self, resolver, module_ident, local_defs, crate_alias=None):
        self.resolver = resolver
        self.module_ident = module_ident
        self.local_defs = set(local_defs)
        self.crate_alias = crate_alias
        self.warnings = []


def leaf_text(path, alias, glob):
    s = '::'.join(path) + ('::*' if glob else '')
    if alias:
        s += f' as {alias}'
    return s


def format_uses(statements):
    """rustfmt over `use` statements, each in its own blank-line separated group."""
    if not statements:
        return []
    src = '\n\n'.join(statements) + '\n'
    with tempfile.NamedTemporaryFile('w', suffix='.rs', delete=False, encoding='utf-8') as f:
        f.write(src)
        p = f.name
    try:
        r = subprocess.run(RUSTFMT + [p], capture_output=True, text=True)
        out = r.stdout if r.returncode == 0 else src
    finally:
        os.unlink(p)
    blocks = [b.strip('\n') for b in re.split(r'\n\s*\n', out.strip('\n')) if b.strip()]
    return blocks if len(blocks) == len(statements) else statements


def map_family(rest, ctx):
    r = ctx.resolver
    if not rest:
        return None
    head = rest[0]
    if head in FACADE_FAMILY:
        mod = FACADE_FAMILY[head]
        tail = rest[1:]
        if not tail:
            return None
        if r.split(mod):
            return (r.lib(mod, tail[0]),) + tail
        return (mod,) + tail
    if head in ALL_MODULE_IDENTS:
        tail = rest[1:]
        if r.split(head) and tail:
            return (r.lib(head, tail[0]),) + tail
        return rest
    if head in r.scope.root:
        return r.root(head) + rest[1:]
    return rest


def map_path(path, ctx):
    """Rewrite the path of one use leaf. None means: drop the leaf (bare module import)."""
    r = ctx.resolver
    first = path[0]
    if first == 'crate':
        rest = path[1:]
        if not rest:
            return None
        head = rest[0]
        if head in ALL_MODULE_IDENTS and (head != ctx.module_ident or ctx.resolver.split(head)):
            tail = rest[1:]
            if not tail or tail == ('self',):
                return None
            if r.split(head):
                while tail and tail[0] in ('writer', 'release') and len(tail) > 1:
                    tail = tail[1:]
                if tail in (('writer',), ('release',)):
                    return None
                if head == 'db_wal' and tail[0] == 'tests':
                    tail = ('testkit',) + tail[1:]
            return ((r.lib(head, tail[0]),) if r.split(head) else (head,)) + tail
        if head in ctx.local_defs:
            return path
        if head in r.scope.root:
            return r.root(head) + rest[1:]
        return path
    if first in ALL_MODULE_IDENTS and (first != ctx.module_ident or ctx.resolver.split(first)):
        tail = path[1:]
        if not tail or tail == ('self',):
            return None
        if r.split(first):
            while tail and tail[0] in ('writer', 'release') and len(tail) > 1:
                tail = tail[1:]
            if tail in (('writer',), ('release',)):
                return None
        return ((r.lib(first, tail[0]),) if r.split(first) else (first,)) + tail
    if first == 'super' and ctx.module_ident == 'db_storage':
        rest = tuple(x for x in path[1:] if x not in ('writer', 'release') and x != 'super')
        if len(rest) != len(path) - 1 and rest and rest[0] in r.homes.get('db_storage', {}):
            return (r.lib('db_storage', rest[0]),) + rest
    if ctx.crate_alias and first == ctx.crate_alias:
        return map_family(path[1:], ctx)
    return path


def find_uses(text):
    """All `use` statements: dict(start, end, indent, vis, tree, attrs) with start covering preceding attribute lines."""
    st = strip(text)
    lines = text.split('\n')
    offs = [0]
    for l in lines:
        offs.append(offs[-1] + len(l) + 1)
    out = []
    for m in USE_RE.finditer(st):
        line_no = st.count('\n', 0, m.start())
        k = line_no - 1
        attrs = []
        while k >= 0 and lines[k].lstrip().startswith('#[') and lines[k].rstrip().endswith(']'):
            attrs.insert(0, lines[k].strip())
            k -= 1
        start = offs[k + 1]
        tree = re.sub(r'\s+', ' ', text[m.start(3):m.end(3)]).strip()
        out.append(dict(start=start, end=m.end(), indent=m.group(1), vis=m.group(2).strip(), tree=tree, attrs=attrs))
    return out


def rewrite_uses(text, ctx, used_names, root_names):
    """Returns the new text with every `use` statement rewritten. `used_names` filters unused leaves of private uses."""
    edits = []
    for u in find_uses(text):
        leaves = parse_use(u['tree'])
        new = []
        for path, alias, glob in leaves:
            if glob and path == ('crate',):
                for n in sorted(root_names):
                    new.append((ctx.resolver.root(n), None, False))
                continue
            if path == ('crate',) and alias:
                ctx.crate_alias = alias
                continue
            if len(path) == 1 and not glob and path[0] in ctx.resolver.scope.root_low and path[0] not in ctx.local_defs:
                new.append((ctx.resolver.root(path[0]), alias, False))
                continue
            np = map_path(path + ('*',), ctx) if glob else map_path(path, ctx)
            if np is None:
                continue
            if glob:
                np = np[:-1]
            if len(np) >= 2 and np[-1] == 'self' and not glob:
                np = np[:-1]
            if not glob and alias != '_' and not u['vis']:
                if (alias or np[-1]) not in used_names:
                    continue
            new.append((np, alias, glob))
        by_head = {}
        for leaf in new:
            by_head.setdefault(leaf[0][0], []).append(leaf)
        stmts = []
        for head in sorted(by_head):
            items = by_head[head]
            singles = [(p, a, g) for p, a, g in items if len(p) == 1 and not g]
            multi = [(p, a, g) for p, a, g in items if not (len(p) == 1 and not g)]
            for p, a, g in singles:
                stmts.append(leaf_text(p, a, g))
            if multi:
                inner = sorted(leaf_text(p[1:], a, g) if len(p) > 1 else '*' for p, a, g in multi)
                stmts.append(head + '::' + (inner[0] if len(inner) == 1 else '{' + ', '.join(inner) + '}'))
        if [(tuple(p), a, g) for p, a, g in leaves] == [(tuple(p), a, g) for p, a, g in new]:
            continue
        formatted = format_uses([f'{u["vis"] + " " if u["vis"] else ""}use {t};' for t in stmts])
        block = []
        for stmt in formatted:
            for a in u['attrs']:
                block.append(u['indent'] + a)
            for i, l in enumerate(stmt.split('\n')):
                block.append((u['indent'] if i == 0 else u['indent']) + l if l.strip() else l)
        edits.append((u['start'], u['end'], '\n'.join(block)))
    for s, e, t in sorted(edits, reverse=True):
        end = e
        if not t:
            if text[end:end + 1] == '\n':
                end += 1
        text = text[:s] + t + text[end:]
    return text


QUAL = re.compile(r'(?<![A-Za-z0-9_:])(crate::)?((?:db_[a-z0-9_]+))::([A-Za-z_][A-Za-z0-9_]*)')
CRATE_NAME = re.compile(r'(?<![A-Za-z0-9_:])crate::([A-Za-z_][A-Za-z0-9_]*)')
ALIAS_PATH = None


def rewrite_paths(text, ctx):
    """Rewrite qualified paths in code: crate::db_x::N, db_x::N (split modules), crate::RootName, db::fam::N."""
    st = strip(text)
    r = ctx.resolver
    edits = []
    use_spans = [(u['start'], u['end']) for u in find_uses(text)]

    def in_use(pos):
        return any(a <= pos < b for a, b in use_spans)

    for m in QUAL.finditer(st):
        if in_use(m.start()):
            continue
        ident, name = m.group(2), m.group(3)
        if ident == ctx.module_ident or ident not in ALL_MODULE_IDENTS:
            continue
        lib = r.lib(ident, name) if r.split(ident) else ident
        edits.append((m.start(), m.end(), f'{lib}::{name}'))
    for m in CRATE_NAME.finditer(st):
        if in_use(m.start()):
            continue
        name = m.group(1)
        if name in ALL_MODULE_IDENTS:
            continue
        if name in ctx.local_defs or name not in r.scope.root:
            continue
        p = r.root(name)
        edits.append((m.start(), m.end(), '::'.join(p)))
    if ctx.crate_alias:
        pat = re.compile(r'(?<![A-Za-z0-9_:])' + re.escape(ctx.crate_alias) + r'::([A-Za-z_][A-Za-z0-9_]*)(::[A-Za-z_][A-Za-z0-9_]*)?')
        for m in pat.finditer(st):
            if in_use(m.start()):
                continue
            rest = (m.group(1),) + ((m.group(2)[2:],) if m.group(2) else ())
            np = map_family(rest, ctx)
            if np is None:
                continue
            if len(np) >= len(rest):
                edits.append((m.start(), m.end(), '::'.join(np)))
    seen = set()
    final = []
    for s, e, t in sorted(edits):
        if any(s < b and a < e for a, b, _ in final):
            continue
        final.append((s, e, t))
    for s, e, t in sorted(final, reverse=True):
        text = text[:s] + t + text[e:]
    return text
