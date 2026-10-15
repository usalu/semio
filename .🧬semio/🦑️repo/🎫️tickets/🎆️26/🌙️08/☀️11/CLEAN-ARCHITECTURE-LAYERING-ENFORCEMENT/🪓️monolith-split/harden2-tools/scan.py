import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from rsitems_lex import lex

OPEN = {'(': ')', '[': ']', '{': '}'}


def toks_of(path):
    src = open(path, encoding='utf-8').read()
    return [t for t in lex(src) if t[0] != 'comment']


def match(toks, i):
    d = 0
    n = len(toks)
    while i < n:
        x = toks[i][1]
        if toks[i][0] == 'p':
            if x in '([{':
                d += 1
            elif x in ')]}':
                d -= 1
                if d == 0:
                    return i
        i += 1
    return n - 1


def skip_attrs(toks, i):
    attrs = []
    n = len(toks)
    while i < n and toks[i][1] == '#' and toks[i][0] == 'p':
        j = i + 1
        if toks[j][1] == '!':
            j += 1
        e = match(toks, j)
        attrs.append(' '.join(t[1] for t in toks[j + 1:e]))
        i = e + 1
    return attrs, i


def parse_vis(toks, i):
    if toks[i][1] == 'pub' and toks[i][0] == 'id':
        if i + 1 < len(toks) and toks[i + 1][1] == '(':
            e = match(toks, i + 1)
            inner = ''.join(t[1] for t in toks[i + 2:e])
            return 'pub(' + inner + ')', e + 1
        return 'pub', i + 1
    return '', i


def skip_generics(toks, i):
    if i < len(toks) and toks[i][1] == '<':
        d = 0
        while i < len(toks):
            if toks[i][1] == '<':
                d += 1
            elif toks[i][1] == '>':
                if toks[i - 1][1] != '-':
                    d -= 1
                    if d == 0:
                        return i + 1
            i += 1
    return i


def block_end(toks, i):
    # i at first token of header; find '{' at depth0 (paren/bracket) or ';'
    n = len(toks)
    d = 0
    while i < n:
        x = toks[i][1]
        if toks[i][0] == 'p':
            if x in '([':
                d += 1
            elif x in ')]':
                d -= 1
            elif x == '{' and d == 0:
                return i, match(toks, i)
            elif x == ';' and d == 0:
                return None, i
        i += 1
    return None, n - 1


class Mod:
    def __init__(self, path):
        self.path = path
        self.parent = None
        self.defs = []  # dicts
        self.children = []


def parse_fields(toks, a, b, tuple_):
    out = []
    i = a
    idx = 0
    while i < b:
        attrs, i = skip_attrs(toks, i)
        if i >= b:
            break
        vis, i = parse_vis(toks, i)
        if tuple_:
            name = str(idx)
        else:
            name = toks[i][1]
            i += 1
            i += 1  # ':'
        idx += 1
        d = 0
        while i < b:
            x = toks[i][1]
            if toks[i][0] == 'p':
                if x in '([{':
                    d += 1
                elif x in ')]}':
                    d -= 1
                elif x == '<':
                    d += 1
                elif x == '>':
                    if toks[i - 1][1] != '-':
                        d -= 1
                elif x == ',' and d == 0:
                    i += 1
                    break
            i += 1
        out.append((name, vis))
    return out


def parse_module_items(toks, a, b, mod):
    i = a
    while i < b:
        attrs, i = skip_attrs(toks, i)
        if i >= b:
            break
        start = i
        vis, i = parse_vis(toks, i)
        while i < b and (toks[i][1] in ('async', 'unsafe', 'default') or toks[i][0] == 'str' or toks[i][1] == 'extern' and toks[i + 1][0] == 'str' or (toks[i][1] == 'const' and toks[i + 1][1] in ('fn', 'unsafe', 'async', 'extern'))):
            i += 1
        kw = toks[i][1]
        line = toks[i][2] + 1
        if kw in ('fn', 'enum', 'union', 'type', 'static', 'const', 'mod', 'trait'):
            name = toks[i + 1][1]
            if name == 'mut':
                name = toks[i + 2][1]
            ob, cb = block_end(toks, i + 2)
            if kw == 'trait' and ob is not None:
                methods = []
                j = ob + 1
                while j < cb:
                    at, j = skip_attrs(toks, j)
                    v2, j = parse_vis(toks, j)
                    while toks[j][1] in ('async', 'unsafe', 'const') and toks[j + 1][1] in ('fn', 'async', 'unsafe', 'const'):
                        j += 1
                    if toks[j][1] in ('fn', 'const', 'type'):
                        methods.append(toks[j + 1][1])
                    ob2, cb2 = block_end(toks, j)
                    j = cb2 + 1
                mod.defs.append(dict(kind='trait', name=name, vis=vis, line=line, methods=methods))
            else:
                mod.defs.append(dict(kind=kw, name=name, vis=vis, line=line))
            if kw == 'mod' and ob is not None:
                mod.inline = getattr(mod, 'inline', [])
            i = (cb if ob is not None else cb) + 1
        elif kw == 'struct':
            name = toks[i + 1][1]
            j = skip_generics(toks, i + 2)
            fields = []
            if toks[j][1] == '(':
                e = match(toks, j)
                fields = parse_fields(toks, j + 1, e, True)
                k = e + 1
                while k < b and toks[k][1] != ';' and toks[k][1] != '{':
                    k += 1
                i = k + 1 if toks[k][1] == ';' else match(toks, k) + 1
                mod.defs.append(dict(kind='struct', name=name, vis=vis, line=line, tuple=True))
            else:
                ob, cb = block_end(toks, j)
                if ob is not None:
                    fields = parse_fields(toks, ob + 1, cb, False)
                mod.defs.append(dict(kind='struct', name=name, vis=vis, line=line, tuple=False))
                i = cb + 1
            for fn_, fv in fields:
                mod.defs.append(dict(kind='field', owner=name, name=fn_, vis=fv, line=line, tuple=mod.defs[-1].get('tuple', False)))
        elif kw == 'impl':
            j = skip_generics(toks, i + 1)
            ob, cb = block_end(toks, j)
            hdr = toks[j:ob] if ob is not None else []
            texts = [t[1] for t in hdr]
            trait = 'for' in texts
            if 'where' in texts:
                hdr = hdr[:texts.index('where')]
                texts = [t[1] for t in hdr]
            if trait:
                selfhdr = hdr[texts.index('for') + 1:]
            else:
                selfhdr = hdr
            sid = [t[1] for t in selfhdr if t[0] == 'id' and t[1] not in ('dyn', 'mut', 'const', 'unsafe', 'impl')]
            # strip generic args: take first path ident chain before '<'
            owner = ''
            dd = 0
            for t in selfhdr:
                if t[1] == '<':
                    dd += 1
                elif t[1] == '>':
                    dd -= 1
                elif dd == 0 and t[0] == 'id' and t[1] not in ('dyn', 'mut', 'const', 'unsafe', 'impl', 'crate', 'self', 'super'):
                    owner = t[1]
            if not trait and ob is not None:
                k = ob + 1
                while k < cb:
                    at, k = skip_attrs(toks, k)
                    if k >= cb:
                        break
                    v2, k = parse_vis(toks, k)
                    while toks[k][1] in ('async', 'unsafe', 'default') or toks[k][0] == 'str' or toks[k][1] == 'extern' or (toks[k][1] == 'const' and toks[k + 1][1] in ('fn', 'unsafe', 'async', 'extern')):
                        k += 1
                    if toks[k][1] in ('fn', 'const', 'type'):
                        nm = toks[k + 1][1]
                        mod.defs.append(dict(kind='assoc-' + toks[k][1], owner=owner, name=nm, vis=v2, line=toks[k][2] + 1))
                    ob2, cb2 = block_end(toks, k)
                    k = cb2 + 1
            i = cb + 1 if ob is not None else cb + 1
        elif kw == 'macro_rules':
            name = toks[i + 2][1]
            ob, cb = block_end(toks, i + 3)
            mod.defs.append(dict(kind='macro', name=name, vis='pub' if any('macro_export' in a for a in attrs) else '', line=line))
            i = cb + 1
            if i < b and toks[i][1] == ';':
                i += 1
        elif kw == 'use':
            ob, cb = block_end(toks, i)
            i = cb + 1
        elif kw == 'extern':
            ob, cb = block_end(toks, i)
            i = cb + 1
        elif kw == 'include' or toks[i][0] == 'id':
            ob, cb = block_end(toks, i)
            i = cb + 1
        else:
            i += 1


def find_mod_decls(toks, base_dir, modname_stack=None):
    """yield (rel_path_attr or None, name, inline_dirs list) for `mod x;` decls"""
    res = []
    stack = []
    i = 0
    n = len(toks)
    pending_attrs = []
    while i < n:
        t = toks[i]
        if t[0] == 'p' and t[1] == '#':
            j = i + 1
            if toks[j][1] == '!':
                j += 1
            e = match(toks, j)
            pending_attrs.append(' '.join(x[1] for x in toks[j + 1:e]))
            i = e + 1
            continue
        if t[0] == 'id' and t[1] == 'mod' and i + 2 < n and toks[i + 1][0] == 'id':
            name = toks[i + 1][1]
            if toks[i + 2][1] == ';':
                pa = None
                for a in pending_attrs:
                    m = re.match(r'path\s*=\s*"(.*)"', a.replace(' ', '')) if False else None
                    if a.replace(' ', '').startswith('path='):
                        pa = a.replace(' ', '')[5:].strip('"')
                res.append((pa, name, list(stack)))
                pending_attrs = []
                i += 3
                continue
            elif toks[i + 2][1] == '{':
                stack.append((name, match(toks, i + 2)))
                pending_attrs = []
                i += 3
                continue
        if t[0] == 'p' and t[1] == '}':
            if stack and stack[-1][1] == i:
                stack.pop()
        if t[0] == 'id' and t[1] == 'include' and i + 1 < n and toks[i + 1][1] == '!':
            pass
        if t[0] == 'p' and t[1] not in '#':
            pending_attrs = [] if t[1] in ';{}' else pending_attrs
        elif t[0] == 'id' and t[1] not in ('pub', 'unsafe', 'extern', 'crate', 'in', 'super', 'self'):
            pending_attrs = []
        i += 1
    return res


def find_includes(toks):
    out = []
    for i, t in enumerate(toks):
        if t[0] == 'id' and t[1] in ('include', 'include_str') and i + 3 < len(toks) and toks[i + 1][1] == '!' and toks[i + 2][1] == '(' and toks[i + 3][0] == 'str':
            out.append((t[1], toks[i + 3][1].strip('"')))
    return out


def load_tree(root, extra_roots=()):
    mods = {}
    order = []

    def visit(path, parent):
        path = os.path.normpath(path) if False else path
        key = os.path.realpath(path)
        if key in mods:
            return mods[key]
        m = Mod(path)
        m.key = key
        m.parent = parent
        m.toks = toks_of(path)
        mods[key] = m
        order.append(m)
        parse_module_items(m.toks, 0, len(m.toks), m)
        here = os.path.dirname(path)
        for pa, name, stack in find_mod_decls(m.toks, here):
            if pa is not None:
                if stack:
                    cands = [os.path.join(here, *[s[0] for s in stack], pa), os.path.join(here, pa), os.path.join(here, os.path.splitext(os.path.basename(path))[0], *[s[0] for s in stack], pa)]
                else:
                    cands = [os.path.join(here, pa)]
            else:
                cands = [os.path.join(here, name + '.rs'), os.path.join(here, name, 'mod.rs')]
            tgt = next((c for c in cands if os.path.exists(c)), None)
            if tgt is None:
                m.missing = getattr(m, 'missing', []) + [(name, pa)]
                continue
            child = visit(tgt, m)
            m.children.append(child)
        for kind, rel in find_includes(m.toks):
            if kind == 'include':
                p = os.path.join(here, rel)
                if os.path.exists(p):
                    child = visit(p, m)
                    child.included = True
                    m.children.append(child)
        return m

    r = visit(root, None)
    return mods, order, r


def ancestors(m):
    out = []
    while m is not None:
        out.append(m)
        m = m.parent
    return out


def scope_module(m, vis):
    if vis == '' or vis == 'pub(self)':
        return m
    if vis == 'pub' or vis == 'pub(crate)':
        return None
    if vis == 'pub(super)':
        return m.parent if m.parent is not None else m
    return m


def in_scope(scope, u):
    if scope is None:
        return True
    return scope in ancestors(u)


def uses_in(m):
    """token facts: dot-calls, dot-fields, path assoc (Type::name), struct literal fields"""
    toks = m.toks
    n = len(toks)
    dotcall = collections.defaultdict(list)
    dotfield = collections.defaultdict(list)
    assoc = collections.defaultdict(list)
    lit = collections.defaultdict(list)
    ident = collections.defaultdict(list)
    for i, t in enumerate(toks):
        if t[0] == 'id':
            ident[t[1]].append(t[2] + 1)
        if t[0] == 'p' and t[1] == '.' and i + 1 < n and toks[i + 1][0] == 'id' and not (i > 0 and toks[i - 1][1] == '.'):
            nm = toks[i + 1][1]
            nxt = toks[i + 2][1] if i + 2 < n else ''
            if nxt == '(' or (nxt == ':' and i + 3 < n and toks[i + 3][1] == ':'):
                dotcall[nm].append(t[2] + 1)
            else:
                dotfield[nm].append(t[2] + 1)
        if t[0] == 'num' and i > 0 and toks[i - 1][1] == '.':
            dotfield[t[1]].append(t[2] + 1)
        if t[0] == 'id' and i + 3 < n and toks[i + 1][1] == ':' and toks[i + 2][1] == ':' and toks[i + 3][0] == 'id':
            assoc[(t[1], toks[i + 3][1])].append(t[2] + 1)
        if t[0] == 'id' and i + 1 < n and toks[i + 1][1] == '{' and t[1][:1].isupper():
            e = match(toks, i + 1)
            d = 0
            j = i + 2
            while j < e:
                x = toks[j]
                if x[0] == 'p' and x[1] in '([{':
                    d += 1
                elif x[0] == 'p' and x[1] in ')]}':
                    d -= 1
                elif d == 0 and x[0] == 'id' and j + 1 <= e and (toks[j + 1][1] in (':', ',', '}') and not (toks[j + 1][1] == ':' and toks[j + 2][1] == ':')) and toks[j - 1][1] in ('{', ','):
                    lit[(t[1], x[1])].append(x[2] + 1)
                j += 1
    return dict(dotcall=dotcall, dotfield=dotfield, assoc=assoc, lit=lit, ident=ident)


def main():
    root = sys.argv[1]
    only = [os.path.realpath(p) for p in sys.argv[2:]]
    mods, order, r = load_tree(root)
    print('modules', len(order), 'missing', sum(len(getattr(m, 'missing', [])) for m in order), file=sys.stderr)
    for m in order:
        for x in getattr(m, 'missing', []):
            print('MISSING', m.path, x, file=sys.stderr)
    facts = {m.key: uses_in(m) for m in order}
    rel = lambda p: p
    all_method_names = collections.Counter()
    all_field_names = collections.Counter()
    for m in order:
        for d in m.defs:
            if d['kind'] in ('assoc-fn', 'assoc-const', 'assoc-type'):
                all_method_names[d['name']] += 1
            if d['kind'] == 'field':
                all_field_names[d['name']] += 1
    report = []
    for m in order:
        if only and m.key not in only:
            continue
        for d in m.defs:
            k = d['kind']
            if k not in ('field', 'assoc-fn', 'assoc-const', 'assoc-type', 'trait', 'struct'):
                continue
            sm = scope_module(m, d['vis'])
            if sm is None:
                continue
            for u in order:
                if u is m or in_scope(sm, u):
                    continue
                f = facts[u.key]
                hits = []
                nm = d['name']
                ow = d.get('owner')
                if k == 'field':
                    for ln in f['dotfield'].get(nm, []):
                        hits.append(('.field', ln))
                    for ln in f['lit'].get((ow, nm), []):
                        hits.append(('lit', ln))
                elif k.startswith('assoc-'):
                    for ln in f['dotcall'].get(nm, []):
                        hits.append(('.call', ln))
                    for ln in f['assoc'].get((ow, nm), []):
                        hits.append(('Type::', ln))
                    if ow == 'Self' or True:
                        for ln in f['assoc'].get(('Self', nm), []):
                            hits.append(('Self::', ln))
                elif k == 'trait':
                    for mn in d['methods']:
                        for ln in f['dotcall'].get(mn, []):
                            hits.append(('trait-method:' + mn, ln))
                elif k == 'struct' and d.get('tuple') and d['vis'] == '':
                    pass
                if hits:
                    report.append(dict(def_file=m.path, def_line=d['line'], kind=k, owner=ow, name=nm, vis=d['vis'], use_file=u.path, hits=hits,
                                       owner_in_use=(ow in f['ident']) if ow else None,
                                       ambiguous=(all_method_names[nm] if k.startswith('assoc') else all_field_names[nm] if k == 'field' else 1)))
    json.dump(report, sys.stdout, ensure_ascii=False, indent=1)


if __name__ == '__main__':
    main()
