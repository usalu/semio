#!/usr/bin/env python3
"""🪓 C6PLUG explicit-import module splitter (descendant of modsplit.py): same item moving, region and widening machinery, but no glob anywhere.
A child never says `use super::*;`; it names every foreign item it uses (`use super::Name;` for root items, `use super::sibling::Name;` for sibling items, the original path for names the root imported).
The root names every re-exported item (`pub use child::{A, B};`), drops the `use` bindings that only its children needed and keeps the ones the root itself or a glob consumer (mounted test module, inline module) still reads.
usage: xsplit.py stage|lossless|paths|install|reinstall|list <tag>"""
import os, re, sys, json, collections, shutil, importlib

T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
import modsplit as M
from modsplit import Doc, select, collapse, prune_empty_regions, widen_children, rewrite_includes, rebase_super, inline_body, vis_rank, PINNED, rel_folders, paths_check, norm_line
from rsitems import lex
from usetree import parse_use_item

REPO = M.REPO
SCR = M.SCR
DEFKINDS = ('fn', 'struct', 'enum', 'trait', 'type', 'const', 'static', 'union', 'mod', 'macro')
CONTEXT_KW = {'fn', 'struct', 'enum', 'trait', 'type', 'mod', 'union', 'static', 'macro_rules'}
MAXW = 250


def inline_body2(lines, it, indent, who):
    h = None
    for L in range(it['s'], it['e'] + 1):
        if M.HDR.match(lines[L]):
            h = L
            break
    assert h is not None, (who, 'no inline mod header', it['s'] + 1)
    assert lines[it['e']].strip() == '}', (who, 'mod does not end with a closing brace', it['e'] + 1)
    ded = []
    for l in lines[h + 1:it['e']]:
        if l.strip() == '':
            ded.append('')
        elif l.startswith(' ' * indent):
            ded.append(l[indent:])
        else:
            ded.append(l)
    while ded and ded[0] == '':
        ded.pop(0)
    while ded and ded[-1] == '':
        ded.pop()
    pre = lines[it['s']:h]
    head = M.HDR.match(lines[h]).group(2).strip() + ';'
    return pre, head, ded, lines[h]


def item_text(doc, it):
    return '\n'.join(doc.lines[it['s']:it['e'] + 1])


def leading_attrs(text):
    toks = lex(text)
    out = []
    i = 0
    while i < len(toks):
        k, x, sl, el, si = toks[i]
        if k == 'comment':
            i += 1
            continue
        if k == 'p' and x == '#':
            j = i + 1
            if toks[j][1] == '!':
                break
            depth = 0
            while True:
                if toks[j][1] == '[':
                    depth += 1
                elif toks[j][1] == ']':
                    depth -= 1
                    if depth == 0:
                        break
                j += 1
            out.append(text[si:toks[j][4] + 1])
            i = j + 1
            continue
        break
    return out


def binders(toks):
    """lowercase identifiers a token stream binds locally: let/for patterns, closure parameters, typed parameters and fields."""
    n = len(toks)
    out = set()

    def pat(i, stops):
        depth = 0
        while i < n:
            k, x = toks[i][0], toks[i][1]
            if k == 'p':
                if x in '([{':
                    depth += 1
                elif x in ')]}':
                    if depth == 0:
                        return i
                    depth -= 1
                elif depth == 0 and x in stops:
                    return i
            elif k == 'id':
                if depth == 0 and x in stops:
                    return i
                nxt = toks[i + 1][1] if i + 1 < n else ''
                if x[0].islower() and x not in ('mut', 'ref', 'in', 'box', 'let', 'if', 'else') and nxt not in ('(', ':', '{') and not (nxt == ':' and toks[i + 2][1] == ':'):
                    out.add(x)
            i += 1
        return i

    for i, (k, x, sl, el, si) in enumerate(toks):
        if k == 'id' and x == 'let':
            pat(i + 1, {'=', ';', ':'})
        elif k == 'id' and x == 'for':
            pat(i + 1, {'in'})
        elif k == 'p' and x == '|':
            p1 = toks[i - 1][1] if i > 0 else ''
            if p1 in ('(', ',', '=', '{', ';', 'move', 'return', '['):
                if i + 1 < n and toks[i + 1][1] == '|':
                    continue
                j = i + 1
                while j < n and toks[j][1] != '|':
                    if toks[j][0] == 'id' and toks[j][1][0].islower() and toks[j][1] not in ('mut', 'ref'):
                        nj = toks[j + 1][1] if j + 1 < n else ''
                        pj = toks[j - 1][1]
                        if pj != ':' and nj != '::':
                            out.add(toks[j][1])
                    j += 1
        elif k == 'id' and x[0].islower() and i + 1 < n and toks[i + 1][1] == ':' and not (i + 2 < n and toks[i + 2][1] == ':'):
            out.add(x)
    return out


def refs_of(text, with_fmt=True):
    """identifiers read from root-level scope: not field/method access, not a path tail, not a binding or declaration name, not a local that shadows (unless called)."""
    toks = [t for t in lex(text) if t[0] != 'comment']
    local = binders(toks)
    out = set()
    n = len(toks)
    for i, (k, x, sl, el, si) in enumerate(toks):
        if k == 'str' and with_fmt:
            for m in re.finditer(r'\{([A-Za-z_][A-Za-z0-9_]*)\s*[}:]', x):
                if m.group(1) not in local:
                    out.add(m.group(1))
            continue
        if k != 'id':
            continue
        p1 = toks[i - 1][1] if i > 0 else ''
        p2 = toks[i - 2][1] if i > 1 else ''
        n1 = toks[i + 1][1] if i + 1 < n else ''
        n2 = toks[i + 2][1] if i + 2 < n else ''
        if p1 == '.' and p2 != '.':
            continue
        if p1 == ':' and p2 == ':':
            continue
        if n1 == ':' and n2 != ':':
            continue
        if p1 in CONTEXT_KW and toks[i - 1][0] == 'id':
            continue
        if p1 in ('let', 'ref') or (p1 == 'for' and n1 == 'in') or (p1 == 'mut' and p2 in ('let', 'ref', '(', ',')):
            continue
        if x in local and not (n1 in ('(', '!') or (n1 == ':' and n2 == ':')):
            continue
        out.add(x)
    return out


def refs_by_item(texts):
    out = set()
    for t in texts:
        out |= refs_of(t)
    return out


def qual_refs(text):
    """first identifier after `crate::`, plus the markers `super::` and `self::` found in the text."""
    toks = [t for t in lex(text) if t[0] != 'comment']
    out = set()
    marks = set()
    n = len(toks)
    for i in range(n - 3):
        if toks[i][0] == 'id' and toks[i + 1][1] == ':' and toks[i + 2][1] == ':':
            if toks[i][1] == 'crate' and toks[i + 3][0] == 'id':
                out.add(toks[i + 3][1])
            elif toks[i][1] in ('super', 'self'):
                marks.add(toks[i][1])
                if toks[i + 3][0] == 'id':
                    out.add(toks[i + 3][1])
    return out, marks


def defs_of(doc, it):
    k, nm = it['kind'], it['name']
    if k in DEFKINDS and nm:
        return [nm]
    return []


def extern_alias(text):
    m = re.match(r'\s*extern\s+crate\s+(\w+)(?:\s+as\s+(\w+))?\s*;', text)
    if m:
        return m.group(2) or m.group(1)
    return None


def cfg_key(attrs):
    return tuple(a for a in attrs if a.startswith('#[cfg('))


import subprocess, tempfile

RUSTFMT = ['rustfmt', '+nightly-2026-07-20', '--edition', '2021', '--config-path', REPO + '/rustfmt.toml']


def fmt_uses(raw):
    """normalizes a list of raw `use` lines (attributes allowed) through rustfmt: sorting, wrapping, trailing commas."""
    if not raw:
        return []
    os.makedirs(SCR + '/fmt', exist_ok=True)
    fp = SCR + '/fmt/imports.rs'
    open(fp, 'w', encoding='utf-8').write('\n'.join(raw) + '\n')
    r = subprocess.run(RUSTFMT + [fp], capture_output=True, text=True)
    assert r.returncode == 0, ('rustfmt failed on imports', r.stderr[:300], raw[:3])
    out = open(fp, encoding='utf-8').read().split('\n')
    if out and out[-1] == '':
        out.pop()
    return [l for l in out if l != '' or False]


def fill_list(prefix, names, suffix=';'):
    if len(names) == 1:
        return ['%s%s%s' % (prefix, names[0], suffix)]
    return ['%s{%s}%s' % (prefix, ', '.join(names), suffix)]


def sort_names(names):
    def key(n):
        if n == 'self':
            return (0, n)
        if n.isupper() or (n.replace('_', '').isupper() and n[0].isupper() and '_' in n):
            return (3, n)
        if n[0].isupper():
            return (2, n)
        return (1, n)
    return sorted(set(names), key=key)


def format_group(entries, attrs, vis=''):
    """entries: list of (bound, fullpath); returns lines (attrs first)."""
    groups = collections.OrderedDict()
    for bound, full in sorted(entries, key=lambda e: e[1]):
        parts = full.split('::')
        leaf = parts[-1]
        parent = '::'.join(parts[:-1])
        item = leaf if bound == leaf else '%s as %s' % (leaf, bound)
        groups.setdefault(parent, []).append(item)
    lines = []
    pre = (vis + ' ') if vis else ''
    for parent, items in groups.items():
        for a in attrs:
            lines.append(a)
        base = '%suse %s::' % (pre, parent) if parent else '%suse ' % pre
        if len(items) == 1 and ' as ' in items[0] or len(items) == 1:
            lines.append('%s%s;' % (base, items[0]))
        else:
            lines.extend(fill_list(base, sort_names(items)))
    return lines


class Splitter:
    def __init__(self, tag, spec):
        self.tag = tag
        self.sp = spec
        orig = open(os.path.join(T, 'orig', spec['orig']), encoding='utf-8').read()
        if spec.get('pre'):
            orig = spec['pre'](orig)
        self.orig_text = orig
        self.doc = Doc(orig)
        self.gen_lines = []
        self.removed_lines = []
        self.notes = []

    def run(self):
        doc, sp = self.doc, self.sp
        lines = doc.lines
        children = sp['children']
        owner = {}
        plans = []
        for ci, c in enumerate(children):
            idxs = select(doc, c['ranges'], c['folder'])
            keep = []
            for ii in idxs:
                it = doc.items[ii]
                if it['kind'] in PINNED or (it['kind'] == 'mod' and lines[it['e']].rstrip().endswith(';')):
                    continue
                assert ii not in owner, ('item in two children', ii, it['name'])
                assert it['balanced'], ('moved item with unbalanced markers', it['s'] + 1, it['e'] + 1, it['name'])
                owner[ii] = ci
                keep.append(ii)
            assert keep, ('empty child', c['folder'])
            plans.append(keep)
        self.owner, self.plans = owner, plans
        self.inline = {}
        for ci, c in enumerate(children):
            if c.get('inline'):
                assert len(plans[ci]) == 1, ('inline child must select exactly one mod item', c['folder'])
                it = doc.items[plans[ci][0]]
                assert it['kind'] == 'mod' and it['name'] == c['inline'] and not lines[it['e']].rstrip().endswith(';'), ('inline child needs the inline mod', c['inline'])
                self.inline[ci] = it
        self.cinfo = {}
        for ci in self.inline:
            it = self.inline[ci]
            pre, head, body, hdr = inline_body2(doc.lines, it, 4, children[ci]['folder'])
            assert not [l for l in pre if l.strip()], ('inline mod with attributes or docs before the header', pre)
            while body and body[0].strip() == '' :
                body.pop(0)
            glob_line = body[0].strip() if body else ''
            self.cinfo[ci] = dict(body=body, head=head, glob=glob_line, hdr=hdr)
            children[ci]['modvis'] = head.split(' mod ')[0].strip() if ' mod ' in head else ''
            self.removed_lines.extend([hdr.strip(), '}'])
            bd = Doc('\n'.join(body))
            self.cinfo[ci]['doc'] = bd
        self.tables()
        child_items = [self.child_body(ci) for ci in range(len(children))]
        self.child_bodies = child_items
        self.resolve_imports()
        self.root_text = self.build_root()
        return self.finish()

    def tables(self):
        doc, sp = self.doc, self.sp
        lines = doc.lines
        children = sp['children']
        self.root_defs = {}
        self.root_kept = []
        self.extern_names = set()
        self.bindings = collections.OrderedDict()
        self.use_items = []
        self.mount_items = []
        self.child_defs = [dict() for _ in children]
        self.def_vis = {}
        self.def_cfg = {}
        for ii, it in enumerate(doc.items):
            txt = item_text(doc, it)
            if ii in self.owner:
                ci = self.owner[ii]
                if ci in self.inline:
                    self.root_defs[it['name']] = it
                    for bit in self.cinfo[ci]['doc'].items:
                        for nme in defs_of(self.cinfo[ci]['doc'], bit):
                            self.child_defs[ci][nme] = bit
                    continue
                for nme in defs_of(doc, it):
                    self.child_defs[ci][nme] = it
                    self.def_cfg[nme] = cfg_key(leading_attrs(txt))
                for nme in children[ci].get('extra_defs', {}):
                    if nme in txt:
                        self.child_defs[ci][nme] = dict(it, kind='macro-defined', name=nme)
                if it['kind'] not in DEFKINDS and it['kind'] not in ('impl', 'use'):
                    self.notes.append(('opaque moved item (may define names)', children[ci]['folder'], it['kind'], txt.split('\n')[0][:80]))
                continue
            self.root_kept.append(ii)
            if it['kind'] == 'use':
                attrs = leading_attrs(txt)
                for bound, full, glob in parse_use_item(lex(txt)):
                    assert not glob, ('glob in root use', full, txt[:100])
                    self.bindings.setdefault(bound, []).append(dict(full=full, attrs=attrs, item=ii))
                self.use_items.append(ii)
            elif it['kind'] == 'crate':
                al = extern_alias(txt.lstrip())
                if al:
                    self.extern_names.add(al)
                elif not txt.lstrip().startswith('#!['):
                    self.notes.append(('unknown crate-kind root item', txt[:60]))
            elif it['kind'] == 'mod' and lines[it['e']].rstrip().endswith(';'):
                self.root_defs[it['name']] = it
                self.mount_items.append(ii)
            else:
                for nme in defs_of(doc, it):
                    self.root_defs[nme] = it
                    self.def_cfg[nme] = cfg_key(leading_attrs(txt))
        allrefs = refs_by_item(item_text(doc, doc.items[ii]) for ii in range(len(doc.items)) if doc.items[ii]['kind'] != 'use')
        self.dark = {b for b in self.bindings if b not in allrefs}
        routes = sp.get('dark_routes', {})
        self.dark -= set(routes)
        if self.dark:
            self.notes.append(('dark bindings (no identifier use; trait or unused import) kept in the root', sorted(self.dark)))
        self.sibling_owner = {}
        for ci, d in enumerate(self.child_defs):
            if ci in self.inline:
                continue
            for nme in d:
                if nme in self.sibling_owner and self.sibling_owner[nme] != ci:
                    raise AssertionError(('name defined in two children', nme))
                self.sibling_owner[nme] = ci

    def item_texts(self, ci):
        if ci in self.inline:
            bd = self.cinfo[ci]['doc']
            return [item_text(bd, it) for it in bd.items if it['kind'] != 'use' or True]
        return [item_text(self.doc, self.doc.items[ii]) for ii in self.plans[ci]]

    def child_body(self, ci):
        if ci in self.inline:
            body = list(self.cinfo[ci]['body'])
            g = self.cinfo[ci]['glob']
            if g == 'use crate::*;':
                body.pop(0)
                self.removed_lines.append('    use crate::*;')
                while body and body[0].strip() == '':
                    body.pop(0)
            return body
        doc = self.doc
        lines = doc.lines
        out = []
        cur = []
        first = True
        for ii in self.plans[ci]:
            it = doc.items[ii]
            target = list(it['stack'])
            p = 0
            while p < len(cur) and p < len(target) and cur[p] == target[p]:
                p += 1
            closers = [doc.regions[r]['close'] for r in reversed(cur[p:])]
            openers = [doc.regions[r]['open'] for r in target[p:]]
            if closers:
                while out and out[-1] == '':
                    out.pop()
                out.extend(closers)
            if not first and out and out[-1] != '':
                out.append('')
            out.extend(openers)
            out.extend(lines[it['s']:it['e'] + 1])
            cur = target
            first = False
        out.extend(doc.regions[r]['close'] for r in reversed(cur))
        return out

    def external_texts(self):
        texts = []
        for p in self.sp.get('external', []):
            fp = os.path.join(REPO, p)
            if os.path.isfile(fp):
                texts.append((p, open(fp, encoding='utf-8').read()))
        return texts

    def split_items(self, text):
        d = Doc(text)
        return [dict(text=item_text(d, it)) for it in d.items]

    def resolve_imports(self):
        sp = self.sp
        children = sp['children']
        self.imports = []
        self.import_gen = []
        self.used_root_defs = set()
        self.used_bindings = set()
        for ci, c in enumerate(children):
            need = refs_by_item(self.item_texts(ci))
            need = set(need) | {n for n, mods in sp.get('dark_routes', {}).items() if c['mod'] in mods}
            skip = set(c.get('skip_imports', []))
            local = set(self.child_defs[ci])
            if ci not in self.inline:
                for ii in self.plans[ci]:
                    it = self.doc.items[ii]
                    if it['kind'] == 'mod':
                        local.add(it['name'])
            lines_by_attr = collections.OrderedDict()
            ext_entries = collections.OrderedDict()
            sup_root = []
            sup_sib = collections.defaultdict(list)
            for nme in sorted(need):
                if nme in local or nme in skip:
                    continue
                if nme in self.root_defs:
                    sup_root.append(nme)
                    self.used_root_defs.add(nme)
                    continue
                if nme in self.sibling_owner and self.sibling_owner[nme] != ci:
                    sup_sib[self.sibling_owner[nme]].append(nme)
                    continue
                if nme in self.bindings:
                    bs = self.bindings[nme]
                    fulls = {b['full'] for b in bs}
                    assert len(fulls) == 1, ('bound twice to different paths', nme, fulls)
                    b = bs[0]
                    full = b['full']
                    if full.startswith('self::'):
                        full = 'super::' + full[len('self::'):]
                    elif full.startswith('super::'):
                        full = 'super::' + full
                    key = tuple(a for a in b['attrs'])
                    ext_entries.setdefault(key, []).append((nme, full))
                    self.used_bindings.add(nme)
                    continue
            allb = set()
            for t_ in self.item_texts(ci):
                allb |= binders([x for x in lex(t_) if x[0] != 'comment'])
            imported = set(sup_root) | {n for ents in ext_entries.values() for n, _ in ents} | {n for v in sup_sib.values() for n in v}
            rv = sorted(n for n in imported if n in allb)
            if rv:
                self.notes.append(('REVIEW shadowed-name imports', c['folder'], rv))
            raw = []
            for key, ents in ext_entries.items():
                raw.extend(format_group(ents, list(key)))
            by_cfg = collections.OrderedDict()
            for nme in sup_root:
                by_cfg.setdefault(('', self.def_cfg.get(nme, ())), []).append(nme)
            for sj in sorted(sup_sib):
                for nme in sup_sib[sj]:
                    by_cfg.setdefault((children[sj]['mod'], self.def_cfg.get(nme, ())), []).append(nme)
            for (modn, cfg), names in by_cfg.items():
                raw.extend(cfg)
                raw.extend(fill_list('use super::%s' % (modn + '::' if modn else ''), sort_names(names)))
            flat = fmt_uses(raw)
            self.imports.append(flat)
            self.import_gen.extend(flat)

    def reexports(self):
        """returns {child index: {(vis, cfgkey): [names]}} for names the root must expose."""
        sp = self.sp
        children = sp['children']
        res = [collections.OrderedDict() for _ in children]
        self.needed_private = collections.defaultdict(set)
        self.ext_refs = set()
        for p, t in self.external_texts():
            self.ext_refs |= refs_by_item(it_['text'] for it_ in self.split_items(t))
        self.root_refs = refs_by_item(item_text(self.doc, self.doc.items[ii]) for ii in self.root_kept if self.doc.items[ii]['kind'] != 'use')
        self.qual = set()
        for ii, it in enumerate(self.doc.items):
            q, marks = qual_refs(item_text(self.doc, it))
            self.qual |= q
            if marks and ii in self.owner:
                self.notes.append(('REVIEW moved item with super::/self:: path', children[self.owner[ii]]['folder'], it['name'], sorted(marks)))
        for p_, t_ in self.external_texts():
            self.qual |= qual_refs(t_)[0]

        for ci, c in enumerate(children):
            if ci in self.inline:
                continue
            for nme, vis in c.get('extra_defs', {}).items():
                if vis in ('pub', 'pub(crate)'):
                    res[ci].setdefault((vis, ()), []).append(nme)
                elif nme in self.root_refs or nme in self.ext_refs or nme in self.qual:
                    res[ci].setdefault(('', ()), []).append(nme)
            for ii in self.plans[ci]:
                it = self.doc.items[ii]
                nme = it['name']
                if it['kind'] not in DEFKINDS or not nme or it['kind'] == 'macro':
                    if it['kind'] == 'macro':
                        raise AssertionError(('macro_rules item moved: needs a textual-order decision', nme))
                    continue
                vis = it['vis']
                txt = item_text(self.doc, it)
                cfg = cfg_key(leading_attrs(txt))
                if vis == 'pub':
                    res[ci].setdefault(('pub', cfg), []).append(nme)
                elif vis == 'pub(crate)':
                    if nme in self.root_refs or nme in self.ext_refs or nme in self.qual or nme in c.get('root_uses', []):
                        res[ci].setdefault(('pub(crate)', cfg), []).append(nme)
                elif vis == '':
                    if nme in self.root_refs or nme in self.ext_refs or nme in self.qual or nme in c.get('root_uses', []):
                        needcfg = cfg
                        if nme not in self.root_refs and nme not in c.get('root_uses', []) and sp.get('external_cfg_test'):
                            needcfg = cfg + ('#[cfg(test)]',)
                        res[ci].setdefault(('', needcfg), []).append(nme)
                else:
                    raise AssertionError(('unsupported visibility on a moved item', vis, nme))
        return res

    def build_root(self):
        doc = self.doc
        lines = doc.lines
        sp = self.sp
        children = sp['children']
        rex = self.reexports()
        self.rex = rex
        skip = set()
        for ii, ci in self.owner.items():
            it = doc.items[ii]
            for L in range(it['s'], it['e'] + 1):
                skip.add(L)
        first_line = min(doc.items[self.plans[ci][0]]['s'] for ci in range(len(children)))
        root_refs = self.root_refs | self.ext_refs | self.dark
        for ci, c in enumerate(children):
            for k, names in rex[ci].items():
                root_refs |= set(names) if k[0] == '' else set()
        use_edit = {}
        seen_kept = {}
        self.dup_removed = []
        core_refs = self.root_refs | self.dark
        for ii in self.use_items:
            it = doc.items[ii]
            txt = item_text(doc, it)
            attrs = leading_attrs(txt)
            binds = parse_use_item(lex(txt))
            keep = []
            ext_only = []
            for b, f, g in binds:
                if not (b in root_refs or b in sp.get('keep_uses', [])):
                    continue
                if f == 'crate::' + b and b in self.root_defs:
                    self.dup_removed.append((b, f + ' (self import of a root module)'))
                    continue
                key = (b, tuple(attrs))
                if key in seen_kept and seen_kept[key] == f:
                    self.dup_removed.append((b, f))
                    continue
                seen_kept[key] = f
                keep.append((b, f))
                if b not in core_refs and sp.get('external_cfg_test') and '#[cfg(test)]' not in attrs:
                    ext_only.append((b, f))
            if len(keep) == len(binds) and not ext_only:
                use_edit[ii] = ('same', None)
            elif not keep:
                use_edit[ii] = ('drop', None)
            else:
                raw = []
                plain = [e for e in keep if e not in ext_only]
                if plain:
                    raw.extend(format_group(plain, attrs))
                if ext_only:
                    raw.extend(format_group(ext_only, attrs + ['#[cfg(test)]']))
                use_edit[ii] = ('regen', fmt_uses(raw))
        wl = ['']
        for ci, c in enumerate(children):
            path_line = '#[path = "%s/🦀️.rs"]' % c['folder']
            modvis = c.get('modvis', '')
            mod_line = '%smod %s;' % (modvis + ' ' if modvis else '', c['mod'])
            if c.get('cfg_mod'):
                wl.append(c['cfg_mod'])
                self.gen_lines.append(c['cfg_mod'])
            wl.extend([path_line, mod_line])
            self.gen_lines.extend([path_line, mod_line])
            names_by = rex[ci]
            raw = []
            for (vis, cfg), names in names_by.items():
                if not names:
                    continue
                raw.extend(cfg)
                pre = {'pub': 'pub use ', 'pub(crate)': 'pub(crate) use ', '': 'use '}[vis]
                raw.extend(fill_list('%s%s::' % (pre, c['mod']), sort_names(names)))
            ul = fmt_uses(raw)
            wl.extend(ul)
            self.gen_lines.extend(ul)
            wl.append('')
        out = []
        use_lines = {}
        for ii, (kind, new) in use_edit.items():
            it = doc.items[ii]
            if kind == 'drop':
                for L in range(it['s'], it['e'] + 1):
                    skip.add(L)
                self.removed_lines.extend(lines[it['s']:it['e'] + 1])
            elif kind == 'regen':
                for L in range(it['s'], it['e'] + 1):
                    skip.add(L)
                self.removed_lines.extend(lines[it['s']:it['e'] + 1])
                use_lines[it['s']] = new
                self.gen_lines.extend(new)
        for L, line in enumerate(lines):
            if L == first_line:
                out.extend(wl)
            if L in use_lines:
                out.extend(use_lines[L])
            if L in skip:
                continue
            out.append(line)
        ptxt = collapse(out)
        ptxt, nrem = prune_empty_regions(ptxt, sp.get('keep_empty', ()))
        return collapse(ptxt.split('\n'))

    def finish(self):
        sp = self.sp
        children = sp['children']
        child_files = {}
        for ci, c in enumerate(children):
            out = [c['doc'], '']
            self.gen_lines.append(c['doc'])
            imps = self.imports[ci] + list(c.get('uses', []))
            self.gen_lines.extend(imps)
            if imps:
                out.extend(imps)
                out.append('')
            out.extend(self.child_bodies[ci])
            while out and out[-1] == '':
                out.pop()
            child_files[c['folder']] = '\n'.join(out) + '\n'
        ext = [t for _, t in self.external_texts()]
        rep = []
        child_files = widen_children2(self.root_text, child_files, ext, rep, sp.get('nest', False), set(sp.get('no_widen', [])))
        for f, t in list(child_files.items()):
            child_files[f] = rewrite_includes2(t, f.count('/') + 1)
        for f, lst in sp.get('patch', {}).items():
            for old_, new_ in lst:
                t = child_files[f]
                assert t.count(old_) == 1, (f, old_[:80], t.count(old_))
                child_files[f] = t.replace(old_, new_)
        ptxt = self.root_text
        if sp.get('leaf_concat'):
            needle = 'rust: include_str!("🦀️.rs")'
            lines_ = ptxt.split('\n')
            hit = [i for i, l in enumerate(lines_) if needle in l]
            assert len(hit) == 1, ('leaf_concat needs exactly one rust leaf', hit)
            names_ = ['🦀️.rs'] + [c['folder'] + '/🦀️.rs' for c in children]
            neu = 'rust: concat!(' + ', '.join('include_str!("%s")' % n for n in names_) + ')'
            self.removed_lines.append(lines_[hit[0]])
            lines_[hit[0]] = lines_[hit[0]].replace(needle, neu)
            self.gen_lines.append(lines_[hit[0]])
            ptxt = '\n'.join(lines_)
        for old_, new_ in sp.get('patch_parent', []):
            assert ptxt.count(old_) == 1, ('parent', old_[:80], ptxt.count(old_))
            ptxt = ptxt.replace(old_, new_)
        self.root_text = ptxt
        self.child_files = child_files
        self.widen_report = rep
        return ptxt, child_files


def widen_children2(ptxt, child_files, external_texts, report, nest, exclude):
    all_ids = {'__parent__': M.W.ident_set(ptxt)}
    for f, t in child_files.items():
        all_ids[f] = M.W.ident_set(t)
    ext = set()
    for t in external_texts:
        ext |= M.W.ident_set(t)
    res = {}
    for f, t in child_files.items():
        others = set(ext)
        for g, s_ in all_ids.items():
            if g != f:
                others |= s_
        others -= exclude
        ed = M.W.plan(t, others, nest)
        res[f] = M.W.apply(t, ed)
        for e in ed:
            report.append((f,) + tuple(e[3]))
    return res


INC2 = re.compile(r'(include_str!|include_bytes!|include!)\(\s*"([^"\\]+)"')


def rewrite_includes2(text, depth):
    def sub(m):
        path = m.group(2)
        if path.startswith('/') or ':' in path:
            return m.group(0)
        return '%s("%s%s"' % (m.group(1), '../' * depth, path)
    return INC2.sub(sub, text)


def write_stage(tag, sp, sp_obj):
    ptxt, files = sp_obj.root_text, sp_obj.child_files
    d = os.path.join(SCR, 'stage', tag)
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    open(os.path.join(d, '🦀️.rs'), 'w', encoding='utf-8').write(ptxt + ('\n' if not ptxt.endswith('\n') else ''))
    for f, t in files.items():
        os.makedirs(os.path.join(d, f))
        open(os.path.join(d, f, '🦀️.rs'), 'w', encoding='utf-8').write(t)
    with open(os.path.join(SCR, 'stage', tag + '.widen.tsv'), 'w', encoding='utf-8') as fh:
        for r in sp_obj.widen_report:
            fh.write('\t'.join(r) + '\n')
    json.dump(dict(gen=sp_obj.gen_lines, removed=sp_obj.removed_lines), open(os.path.join(SCR, 'stage', tag + '.decl.json'), 'w', encoding='utf-8'), ensure_ascii=False, indent=0)
    return d


def stage(tag, mod):
    sp = mod.SPEC
    s = Splitter(tag, sp)
    s.run()
    d = write_stage(tag, sp, s)
    sizes = {f: t.count('\n') for f, t in s.child_files.items()}
    print('staged', tag, 'root', s.root_text.count('\n') + 1, 'children', sizes, 'widened', len(s.widen_report))
    for n in s.notes:
        print('  NOTE', n)
    if s.dup_removed:
        print('  NOTE duplicate root imports dropped', s.dup_removed)
    return d


def strip_vis(l):
    return norm_line(l).replace('pub(in super::super) ', '')


def lossless(tag, mod, d=None):
    sp = mod.SPEC
    orig = open(os.path.join(T, 'orig', sp['orig']), encoding='utf-8').read()
    if sp.get('pre'):
        orig = sp['pre'](orig)
    d = d or os.path.join(SCR, 'stage', tag)
    decl = json.load(open(os.path.join(SCR, 'stage', tag + '.decl.json'), encoding='utf-8'))
    files = [os.path.join(d, '🦀️.rs')] + [os.path.join(d, c['folder'], '🦀️.rs') for c in sp['children']]
    MK = re.compile(r'^\s*//\s?#(end)?region\b')
    O = collections.Counter(strip_vis(l) for l in orig.split('\n') if l.strip() and not MK.match(l))
    N = collections.Counter()
    for f in files:
        N += collections.Counter(strip_vis(l) for l in open(f, encoding='utf-8').read().split('\n') if l.strip() and not MK.match(l))
    gen = collections.Counter(strip_vis(l) for l in decl['gen'] if l.strip())
    rem = collections.Counter(strip_vis(l) for l in decl['removed'] if l.strip())
    for l in sp.get('expect_missing', []):
        rem[strip_vis(l)] += 1
    for l in sp.get('expect_extra', []):
        gen[strip_vis(l)] += 1
    miss = O - N - rem
    extra = N - O - gen
    print('files', len(files), 'orig non-blank', sum(O.values()), 'new non-blank', sum(N.values()), 'declared generated', sum(gen.values()), 'declared removed', sum(rem.values()))
    print('missing', sum(miss.values()))
    for l, c in sorted(miss.items()):
        print('  -', c, l[:200])
    print('extra', sum(extra.values()))
    for l, c in sorted(extra.items()):
        print('  +', c, l[:200])
    unmet_gen = gen - N
    if unmet_gen:
        print('declared generated lines not present', sum(unmet_gen.values()), list(unmet_gen)[:3])
    rr = re.compile(r'^\s*//\s?#region\b', re.M)
    re_ = re.compile(r'^\s*//\s?#endregion\b', re.M)
    mo, eo = len(rr.findall(orig)), len(re_.findall(orig))
    mn = sum(len(rr.findall(open(f, encoding='utf-8').read())) for f in files)
    en = sum(len(re_.findall(open(f, encoding='utf-8').read())) for f in files)
    print('regions orig/new', mo, mn, 'endregions orig/new', eo, en)
    glob_lines = [l for f in files for l in open(f, encoding='utf-8').read().split('\n') if re.match(r'^\s*(pub(\([^)]*\))?\s+)?use\s+[^;]*::\*;', l)]
    orig_globs = [l for l in orig.split('\n') if re.match(r'^\s*(pub(\([^)]*\))?\s+)?use\s+[^;]*::\*;', l)]
    print('glob use lines orig/new', len(orig_globs), len(glob_lines))
    for l in glob_lines:
        print('  glob', l.strip()[:120])
    ok = not miss and not extra and mn - en == mo - eo and len(glob_lines) <= len(orig_globs)
    print('LOSSLESS', 'OK' if ok else 'FAIL')
    return ok


def install(tag, mod):
    sp = mod.SPEC
    d = os.path.join(SCR, 'stage', tag)
    target = os.path.join(REPO, sp['file'])
    cur = open(target, 'rb').read()
    orig = open(os.path.join(T, 'orig', sp['orig']), 'rb').read()
    assert cur == orig, 'repo file differs from the recorded original; refusing to overwrite'
    base = os.path.dirname(target)
    folders = rel_folders(sp)
    for f in folders:
        assert not os.path.exists(os.path.join(base, f)), ('child folder exists', f)
    for f in folders:
        cd = os.path.join(base, f)
        os.makedirs(cd)
        shutil.copyfile(os.path.join(d, f, '🦀️.rs'), os.path.join(cd, '🦀️.rs'))
    tmp = target + '.pfa-tmp'
    shutil.copyfile(os.path.join(d, '🦀️.rs'), tmp)
    os.replace(tmp, target)
    shutil.rmtree(os.path.join(SCR, 'installed', tag), ignore_errors=True)
    shutil.copytree(d, os.path.join(SCR, 'installed', tag))
    print('installed', tag)


if __name__ == '__main__':
    cmd, tag = sys.argv[1], sys.argv[2]
    if cmd == 'list':
        txt = open(os.path.join(T, 'orig', tag + '.rs'), encoding='utf-8').read()
        try:
            sp_ = importlib.import_module('specs.' + tag).SPEC
            if sp_.get('pre'):
                txt = sp_['pre'](txt)
        except ModuleNotFoundError:
            pass
        M.listing(Doc(txt))
        sys.exit(0)
    mod = importlib.import_module('specs.' + tag)
    if cmd == 'stage':
        stage(tag, mod)
    elif cmd == 'lossless':
        sys.exit(0 if lossless(tag, mod, sys.argv[3] if len(sys.argv) > 3 else None) else 1)
    elif cmd == 'paths':
        sys.exit(0 if paths_check(tag, mod) else 1)
    elif cmd == 'install':
        install(tag, mod)
