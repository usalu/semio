"""🔓️ Narrowest-visibility widening: items, fields and re-exports of a carved crate that another crate (or the facade) names become `pub`; everything else keeps its visibility."""
import os, re, sys
from model import *
import oracle2_closure as oc
ELEMENT_DIRS = [EL + '/⚙️EngineCanvas', EL + '/🎞️Scenes', EL + '/🖼️IconRenderHost', EL + '/📐️Canvas2dHost', E + '/🧪️tests']
RESTRICTED = r'pub\((?:super|crate)\)'
ITEM = re.compile(r'^(?P<ind>[ \t]*)(?P<vis>' + RESTRICTED + r')\s+(?P<kw>(?:(?:unsafe|async|const|extern\s+"[^"]*")\s+)*(?:fn|struct|enum|union|trait|type|static|const|mod))\s+(?P<name>\w+)', re.M)
FIELD = re.compile(r'^(?P<ind>[ \t]+)(?P<vis>' + RESTRICTED + r')\s+(?P<name>\w+)\s*:', re.M)
REEXP = re.compile(r'^(?P<ind>[ \t]*)(?P<vis>' + RESTRICTED + r')\s+use\s+(?P<body>[^;]*?(?:as\s+(?P<alias>\w+)|::(?P<leaf>\w+)|::\{(?P<list>[^}]*)\}))\s*;', re.M)
TLSTATIC = re.compile(r'(?m)^([ \t]+)static\s+(\w+)\s*:')
PRIV = re.compile(r'^(?P<kw>(?:(?:unsafe|async|const)\s+)*(?:fn|struct|enum|union|trait|type|static|const))\s+(?P<name>\w+)', re.M)

def all_files():
    out = []
    for d in [EL, E + '/🧪️tests', T]:
        for dp, dn, fn in os.walk(d):
            dn[:] = [x for x in dn if x not in ('node_modules', 'target', 'dist')]
            out += [os.path.join(dp, f) for f in fn if f.endswith('.rs')]
    return sorted(set(out))

def tokens(path):
    return set(re.findall(r'[A-Za-z_]\w*', strip(open(path, encoding='utf8').read())))

def owner_type(its, line):
    for it in its:
        if it['start'] <= line <= it['end']:
            if it['kind'] == 'impl':
                m = re.match(r'impl(?:<[^>]*>)?\s+(?:[\w:<>, ]+\s+for\s+)?([A-Za-z_]\w*)', it['name'])
                return m.group(1) if m else None
            return item_name(it)
    return None

def widen(crate, apply=False, extra_owned=(), owned_override=None):
    root = T + '/' + FOLDER[crate] + '/🦀️.rs' if crate else ''
    owned = set(owned_override) if owned_override else set(oc.closure(root)) | set(extra_owned)
    codes = {}
    for f in all_files():
        if f not in owned: codes[f] = strip(open(f, encoding='utf8').read())
    alltok = set()
    for c in codes.values(): alltok |= set(re.findall(r'[A-Za-z_]\w*', c))
    def member_used(tname, n):
        pat = re.compile(r'(?:\.\s*|::)' + re.escape(n) + r'\b|(?<![\w.])' + re.escape(n) + r'\s*:(?!:)')
        for c in codes.values():
            if (tname is None or tname in c) and pat.search(c): return True
        return False
    changed = []
    for f in sorted(owned):
        t = open(f, encoding='utf8').read(); orig = t
        its = items(t)
        def lineno(m): return t.count('\n', 0, m.start())
        def item(m):
            n = m.group('name'); top = not m.group('ind')
            if top: hit = n in alltok
            else: hit = member_used(owner_type(its, lineno(m)), n)
            if hit:
                changed.append((os.path.basename(os.path.dirname(f)), 'item' if top else 'method', n, m.group('vis'))); return m.group(0).replace(m.group('vis'), 'pub', 1)
            return m.group(0)
        def field(m):
            n = m.group('name')
            if member_used(owner_type(its, lineno(m)), n):
                changed.append((os.path.basename(os.path.dirname(f)), 'field', n, m.group('vis'))); return m.group(0).replace(m.group('vis'), 'pub', 1)
            return m.group(0)
        def reexp(m):
            ns = [m.group('alias') or m.group('leaf')] if (m.group('alias') or m.group('leaf')) else [x.split(' as ')[-1].strip() for x in m.group('list').split(',') if x.strip()]
            n = next((x for x in ns if x in alltok), None)
            if n:
                changed.append((os.path.basename(os.path.dirname(f)), 'use', n, m.group('vis'))); return m.group(0).replace(m.group('vis'), 'pub', 1)
            return m.group(0)
        def priv(m):
            if m.group('name') in alltok:
                changed.append((os.path.basename(os.path.dirname(f)), 'private', m.group('name'), '')); return 'pub ' + m.group(0)
            return m.group(0)
        def tl(m):
            if m.group(2) in alltok and any(i['kind'] == 'thread_local!' and i['start'] <= lineno(m) <= i['end'] for i in its):
                changed.append((os.path.basename(os.path.dirname(f)), 'tlstatic', m.group(2), '')); return m.group(1) + 'pub static ' + m.group(2) + ':'
            return m.group(0)
        t = ITEM.sub(item, t); t = FIELD.sub(field, t); t = REEXP.sub(reexp, t); t = PRIV.sub(priv, t); t = TLSTATIC.sub(tl, t)
        if apply and t != orig: open(f, 'w', encoding='utf8').write(t)
    return changed

if __name__ == '__main__':
    crate = sys.argv[1]; apply = '--apply' in sys.argv
    ch = widen(crate, apply)
    from collections import Counter
    print(crate, 'widened' if apply else 'would widen', len(ch), dict(Counter(k for _, k, _, _ in ch)))
    if '-v' in sys.argv:
        for c in ch: print('  ', c)
