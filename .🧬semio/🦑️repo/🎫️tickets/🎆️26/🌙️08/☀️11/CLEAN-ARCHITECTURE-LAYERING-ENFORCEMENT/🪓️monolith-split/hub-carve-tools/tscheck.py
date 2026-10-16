#!/usr/bin/env python3
"""🔎️ Static check of the TypeScript source readers: every literal a hub check asserts on a Rust source must be where the reader looks, now as before the carve."""
import os, re, sys, collections
REPO = '/Users/ueli/Documents/semio/'
OLD = '/tmp/hubw/baseline/'
SCRIPT = REPO + '🌎️hub/📦️packages/🦀️rust/📜️script.ts'
PKG = '🌎️hub/📦️packages/🦀️rust'
BIN_ROOTS = ['⚙️runtime', '📒️ledgers', '🧱️models', '🧠️state', '🛰️session-routes', '📨️document-routes', '🛣️routes']
BIN_TESTS = ['🧰️testkit/🦀️.rs', '🚀️boot/🔬️unit/🦀️.rs', '🧭️open-plan/🔬️unit/🦀️.rs', '📇️directory/🔬️unit/🦀️.rs', '👥️presence/🔬️unit/🦀️.rs', '🔐️session/🔬️unit/🦀️.rs', '🎲️hostile-input/🔬️unit/🦀️.rs']
PATH_ATTR = re.compile(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
INCLUDE = re.compile(r'include!\(\s*"([^"]+)"\s*\)')


def rd(base, rel):
    p = os.path.normpath(os.path.join(base, rel))
    return open(p).read() if os.path.exists(p) else None


def tree(base, rel, seen=None):
    seen = seen if seen is not None else set()
    p = os.path.normpath(os.path.join(base, rel))
    if p in seen or not os.path.exists(p):
        return ''
    seen.add(p)
    text = open(p).read()
    out = [text]
    d = os.path.dirname(p)
    for m in PATH_ATTR.finditer(text):
        child = os.path.normpath(os.path.join(d, m.group(1)))
        if '/🧪️tests/' in child:
            continue
        out.append(tree('/', child, seen))
    return '\n'.join(out)


def old_new(kind, arg):
    """-> (old text, new text) for one reader call."""
    if kind == 'bin':
        o = tree(OLD, '🌎️hub/🏗️bootstrap/🦀️.rs')
        n = '\n'.join(tree(REPO, f'🌎️hub/🏗️bootstrap/{r}/🦀️.rs') for r in BIN_ROOTS) + '\n' + tree(REPO, '🌎️hub/🏗️bootstrap/🦀️.rs')
        return o, n
    if kind == 'bintest':
        o = rd(OLD, '🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs') or ''
        n = '\n'.join(rd(REPO, '🌎️hub/🧪️tests/🔬️bin-unit/' + f) or '' for f in BIN_TESTS)
        return o, n
    rel = arg
    if rel.startswith('🌎️hub') and not rel.startswith(PKG):
        pass
    if kind == 'file':
        return rd(OLD, rel) or '', rd(REPO, rel) or ''
    if kind == 'tree':
        return tree(OLD, rel), tree(REPO, rel)


def lits(expr):
    return re.findall(r'"([^"]*)"', expr)


def resolve_path(expr):
    ls = lits(expr)
    if not ls:
        return None
    if any(x.startswith('../') for x in ls[:1]) or ls[0].startswith('../../'):
        base = PKG + '/'
        return os.path.normpath(base + '/'.join(ls))
    return os.path.normpath('/'.join(ls))


def readers(src):
    pat = re.compile(r'(readFileSync|readModuleTreeSource|readHubBinSource|readHubBinTestSource)\(((?:[^()]|\([^()]*\))*)\)')
    for m in pat.finditer(src):
        fn, args = m.group(1), m.group(2)
        if fn == 'readHubBinSource':
            yield m, 'bin', None
        elif fn == 'readHubBinTestSource':
            yield m, 'bintest', None
        else:
            path = resolve_path(args.split('"utf8"')[0])
            if path and path.startswith('🌎️hub') and path.endswith('.rs'):
                yield m, 'file' if fn == 'readFileSync' else 'tree', path


def main():
    src = open(SCRIPT).read()
    bad = collections.OrderedDict()
    cache = {}
    for m, kind, path in readers(src):
        key = (kind, path)
        if key not in cache:
            cache[key] = old_new(kind, path)
        o, n = cache[key]
        end = m.end()
        # assigned variable?
        head = src[max(0, m.start() - 60):m.start()]
        am = re.search(r'(?:const|let)\s+(\w+)(?:\s*:[^=]+)?\s*=\s*(?:await\s+)?$', head)
        uses = []
        if am:
            var = am.group(1)
            window = src[end:end + 9000]
            # stop at the next assignment of the same var
            for um in re.finditer(r'\b' + var + r'\b\s*\.\s*(includes|indexOf|startsWith|lastIndexOf)\(\s*(["\'`])((?:[^\\]|\\.)*?)\2', window):
                uses.append(um.group(3))
        tail = src[end:end + 400]
        im = re.match(r'(?:\s*\.\s*split\([^)]*\)\[0\]!?)?\s*\.\s*(includes|indexOf)\(\s*(["\'`])((?:[^\\]|\\.)*?)\2', tail)
        if im:
            uses.append(im.group(3))
        line = src.count('\n', 0, m.start()) + 1
        if am:
            for sm in re.finditer(r'\b' + am.group(1) + r'\.slice\(\s*' + am.group(1) + r'\.indexOf\((["\'`])((?:[^\\]|\\.)*?)\1[^)]*\)\s*,\s*' + am.group(1) + r'\.indexOf\((["\'`])((?:[^\\]|\\.)*?)\3', src[end:end + 9000]):
                a, b = sm.group(2), sm.group(4)
                if '${' in a or '${' in b:
                    continue
                oa, ob = o.find(a), o.find(b)
                na, nb = n.find(a), n.find(b)
                if oa >= 0 and ob > oa and not (na >= 0 and nb > na):
                    bad.setdefault((line, kind, path), []).append(f'ORDER {a[:40]!r} .. {b[:40]!r} (new {na},{nb})')
        for u in uses:
            if '${' in u or len(u) < 6:
                continue
            u2 = u.replace('\\"', '"').replace("\\'", "'").replace('\\\\', '\\').replace('\\n', '\n')
            if u2 in o and u2 not in n:
                bad.setdefault((line, kind, path), []).append(u2[:90])
    for k, v in bad.items():
        print(k)
        for u in sorted(set(v)):
            print('     MISSING', repr(u))
    print('sites with missing literals:', len(bad))


if __name__ == '__main__':
    main()
