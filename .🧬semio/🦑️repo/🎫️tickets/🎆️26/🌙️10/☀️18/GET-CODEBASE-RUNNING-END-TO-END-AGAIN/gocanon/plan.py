#!/usr/bin/env python3
"""Faithful port of canonicalGoPlan (go-plan/🟦️.ts). usage: plan.py <moduleRoot> <overlay.json> -> prints packages (one per line) on stdout."""
import hashlib, json, os, re, sys

TAX = '/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json'
t = json.load(open(TAX))
SRC = set(t['semanticDirectoryMemberKinds']['members-of-members-of-members-of-modules']['memberNames'])
TESTS = t['testsDirName']
PKGS = t['packagesDirName']
FIX = t['testFixturesDirName']
ORA = t['testOraclesDirName']
IMPL = '🐹️.go'
LANG = '🐹️go'


def clause(source):
    rest = source.lstrip('\ufeff')
    while True:
        rest = rest.lstrip()
        if rest.startswith('//'):
            i = rest.find('\n')
            rest = '' if i < 0 else rest[i + 1:]
        elif rest.startswith('/*'):
            i = rest.find('*/')
            rest = '' if i < 0 else rest[i + 2:]
        else:
            break
    m = re.match(r'package\s+([^\W\d]\w*)', rest)
    return m.group(1) if m else None


def module_plan(root):
    root = os.path.realpath(root)
    src, tst, pk = {}, {}, set()

    def owner_rel(owner):
        r = os.path.relpath(owner, root).replace(os.sep, '/')
        return '.' if r == '.' else './' + r

    def mp(owner, source, test):
        rel = os.path.relpath(source, root).replace(os.sep, '/')
        i = hashlib.sha256(rel.encode()).hexdigest()[:16]
        v = os.path.join(owner, f"zz_semio_{i}{'_test' if test else ''}.go")
        assert not os.path.exists(v), v
        (tst if test else src)[v] = source
        pk.add(owner_rel(owner))

    def walk(d):
        if d != root and os.path.exists(os.path.join(d, 'go.mod')):
            return
        names = sorted(os.listdir(d))
        if any(os.path.isfile(os.path.join(d, n)) and not os.path.islink(os.path.join(d, n)) and n.endswith('.go') for n in names):
            pk.add(owner_rel(d))
        for name in names:
            p = os.path.join(d, name)
            if not os.path.isdir(p) or os.path.islink(p):
                continue
            if os.path.exists(os.path.join(p, 'go.mod')):
                continue
            if name in SRC:
                s = os.path.join(p, IMPL)
                if os.path.isfile(s) and not os.path.islink(s):
                    mp(d, s, False)
                continue
            if name == TESTS:
                for c in sorted(os.listdir(p)):
                    cp = os.path.join(p, c)
                    if not os.path.isdir(cp) or os.path.islink(cp):
                        continue
                    s = os.path.join(cp, IMPL)
                    if os.path.isfile(s) and not os.path.islink(s):
                        mp(d, s, True)
                continue
            if name in ('.git', 'node_modules', 'target', 'vendor', FIX, ORA, PKGS):
                continue
            walk(p)

    walk(root)
    packages_dir = os.path.dirname(root)
    if os.path.basename(root) == LANG and os.path.basename(packages_dir) == PKGS:
        owner = os.path.dirname(packages_dir)
        entry = os.path.join(root, IMPL)
        cases = os.path.join(owner, TESTS)
        if os.path.isfile(entry) and not os.path.islink(entry) and os.path.isdir(cases) and not os.path.islink(cases):
            subject = clause(open(entry, encoding='utf8').read())
            if subject:
                for c in sorted(os.listdir(cases)):
                    cp = os.path.join(cases, c)
                    if not os.path.isdir(cp) or os.path.islink(cp):
                        continue
                    s = os.path.join(cp, IMPL)
                    if not os.path.isfile(s) or os.path.islink(s):
                        continue
                    k = clause(open(s, encoding='utf8').read())
                    if k in (subject, subject + '_test'):
                        mp(root, s, True)
    return src, tst, pk


def replace_roots(root):
    gm = os.path.join(root, 'go.mod')
    if not os.path.exists(gm):
        return []
    out, block = [], False
    for raw in open(gm).read().splitlines():
        line = re.sub(r'//.*$', '', raw).strip()
        if not line:
            continue
        if re.match(r'^replace\s*\($', line):
            block = True
            continue
        if block and line == ')':
            block = False
            continue
        d = line
        if re.match(r'^replace\s+', line):
            d = re.sub(r'^replace\s+', '', line)
        elif not block:
            continue
        parts = d.split()
        if '=>' not in parts:
            continue
        rep = parts[parts.index('=>') + 1]
        if not (os.path.isabs(rep) or rep.startswith('.')):
            continue
        c = os.path.realpath(os.path.join(root, rep))
        if os.path.exists(os.path.join(c, 'go.mod')):
            out.append(c)
    return sorted(set(out))


def plan(root):
    root = os.path.realpath(root)
    src, tst, pk = module_plan(root)
    seen = {root}

    def add(o):
        for dep in replace_roots(o):
            if dep in seen:
                continue
            seen.add(dep)
            s, _, _ = module_plan(dep)
            for k, v in s.items():
                assert k not in src or src[k] == v, k
                src[k] = v
            add(dep)

    add(root)
    return src, tst, sorted(pk)


if __name__ == '__main__':
    root, out = sys.argv[1], sys.argv[2]
    s, tt, pk = plan(root)
    rep = dict(s)
    rep.update(tt)
    json.dump({'Replace': rep}, open(out, 'w'), ensure_ascii=False, indent=1)
    print('\n'.join(pk))
    print(f'sources={len(s)} tests={len(tt)}', file=sys.stderr)
