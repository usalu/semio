#!/usr/bin/env python3
"""🗺️ Re-points exact-law groups of the hub scripts from the retired `semio-hub` lib/bin to the crates that now own each law."""
import os, re, sys, json, collections

MAPPING = '/tmp/hubw/law-mapping.json'


def load():
    mp = json.load(open(MAPPING))
    byold, byname = {}, collections.defaultdict(list)
    for oc, op, nc, np_ in mp:
        byold[(oc, op)] = (nc, np_)
        byname[op.split('::')[-1]].append((oc, op, nc, np_))
    return byold, byname


def pkg_of(ident):
    return 'semio-hub' if ident == 'os_hub' else ident.replace('_', '-')


def match_brace(s, i, open_c, close_c):
    d = 0
    j = i
    in_str = None
    while j < len(s):
        c = s[j]
        if in_str:
            if c == '\\':
                j += 2
                continue
            if c == in_str:
                in_str = None
        elif c in '"\'`':
            in_str = c
        elif c == open_c:
            d += 1
        elif c == close_c:
            d -= 1
            if d == 0:
                return j
        j += 1
    raise ValueError('unbalanced')


def top_level_keys(obj):
    """Splits `{ k: v, k: v }` into [(key, value text)] at depth 1."""
    body = obj[1:-1]
    out, depth, start, in_str = [], 0, 0, None
    parts = []
    j = 0
    while j < len(body):
        c = body[j]
        if in_str:
            if c == '\\':
                j += 2
                continue
            if c == in_str:
                in_str = None
        elif c in '"\'`':
            in_str = c
        elif c in '([{':
            depth += 1
        elif c in ')]}':
            depth -= 1
        elif c == ',' and depth == 0:
            parts.append(body[start:j])
            start = j + 1
        j += 1
    parts.append(body[start:])
    for p in parts:
        p = p.strip()
        if not p:
            continue
        m = re.match(r'(\w+)\s*:\s*(.*)$', p, re.S)
        out.append((m.group(1), m.group(2).strip()) if m else (None, p))
    return out


STR = re.compile(r'"((?:[^"\\]|\\.)*)"')


def literal_strings(text):
    text = text.strip()
    if not (text.startswith('[') and text.endswith(']')):
        return None
    inner = text[1:-1]
    rest = STR.sub('', inner)
    if rest.replace(',', '').strip():
        return None
    return [m.group(1) for m in STR.finditer(inner)]


def split_list(text):
    """Elements of a literal `[...]` of strings (also the bare comma-free argument arrays)."""
    return literal_strings(text)


def locate(lit, kind, byold, byname):
    old = 'os_hub' if kind == 'bin' else 'semio_hub'
    if (old, lit) in byold:
        nc, np_ = byold[(old, lit)]
        return nc, np_
    if '::' in lit:
        base = lit.split('::')[-1]
    else:
        base = lit
    cands = [c for c in byname.get(base, []) if c[0] == old and (c[1] == lit or c[1].endswith('::' + lit) or '::' not in lit)]
    cands = list({(c[2], c[3]) for c in cands})
    if len(cands) == 1:
        return cands[0]
    cands = list({(c[2], c[3]) for c in byname.get(base, []) if c[0] == old})
    if len(cands) == 1:
        return cands[0]
    return None


def rewrite_object(obj, indent, byold, byname, features_of, report, line):
    keys = dict((k, v) for k, v in top_level_keys(obj) if k)
    tgt = keys.get('target', '')
    if '"test"' in tgt:
        return None
    kind = 'bin' if 'bin' in tgt else 'lib'
    laws = literal_strings(keys.get('laws', ''))
    if laws is None:
        report.append((line, 'NON-LITERAL laws', keys.get('laws', '')[:80]))
        return None
    cargo = keys.get('cargoArgs')
    cargo_items = literal_strings(cargo) if cargo is not None else None
    order, groups = [], collections.OrderedDict()
    for lit in laws:
        loc = locate(lit, kind, byold, byname)
        if loc is None:
            report.append((line, 'UNMAPPED', lit))
            return None
        nc, np_ = loc
        newlit = lit if '::' not in lit else np_
        groups.setdefault(nc, []).append(newlit)
    multiline = '\n' in obj
    out = []
    for nc, ls in groups.items():
        pkg = pkg_of(nc)
        tgt = '{ kind: "bin", name: "os-hub" }' if nc == 'os_hub' else '{ kind: "lib" }'
        parts = [f'package: "{pkg}"', f'target: {tgt}']
        if cargo is not None:
            if cargo_items is None:
                parts.append(f'cargoArgs: {cargo}')
            else:
                items = list(cargo_items)
                while '--features' in items:
                    i = items.index('--features')
                    f = items[i + 1]
                    if f.split(',')[0] != 'integration-fixtures' or pkg in features_of.get('integration-fixtures', ()):
                        break
                    del items[i:i + 2]
                    if pkg not in features_of.get('integration-fixtures', ()):
                        continue
                if cargo_items is not None:
                    parts.append('cargoArgs: [' + ', '.join('"%s"' % x for x in items) + ']')
        law_text = '[' + ', '.join('"%s"' % x for x in ls) + ']'
        if multiline:
            inner = (',\n' + indent + '  ').join(parts + [f'laws: {law_text}'])
            out.append('{\n' + indent + '  ' + inner + ',\n' + indent + '}')
        else:
            out.append('{ ' + ', '.join(parts + [f'laws: {law_text}']) + ' }')
    return (',\n' + indent if multiline else ', ').join(out)


def rewrite(s, byold, byname, features_of):
    report = []
    out, pos = [], 0
    for m in re.finditer(r'\{\s*package: "semio-hub"', s):
        a = m.start()
        if a < pos:
            continue
        b = match_brace(s, a, '{', '}')
        obj = s[a:b + 1]
        line = s.count('\n', 0, a) + 1
        ls = s.rfind('\n', 0, a) + 1
        indent = re.match(r'\s*', s[ls:a]).group(0) if s[ls:a].strip() == '' else re.match(r'\s*', s[ls:]).group(0)
        new = rewrite_object(obj, indent, byold, byname, features_of, report, line)
        if new is None:
            continue
        out.append(s[pos:a])
        out.append(new)
        pos = b + 1
    out.append(s[pos:])
    return ''.join(out), report


if __name__ == '__main__':
    byold, byname = load()
    path = sys.argv[1]
    s = open(path).read()
    features_of = {'integration-fixtures': {'semio-hub', 'semio-hub-boot-laws', 'semio-hub-hostile-input-laws', 'semio-hub-trusted-catalog', 'semio-hub-trusted-catalog-contract', 'semio-hub-inference', 'semio-hub-inference-runtime', 'semio-hub-trusted-catalog-fixtures'}}
    new, report = rewrite(s, byold, byname, features_of)
    open(sys.argv[2], 'w').write(new)
    for r in report:
        print(r)
