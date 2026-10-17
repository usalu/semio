#!/usr/bin/env python3
"""✂️ c4p17 splitter for the polynomial-internals root: each top-level inline `pub mod x { ... }` becomes a file module `<folder>/🦀️.rs` (body dedented by 4, multi-line string literals untouched, `include!` re-based by one `../`).
usage: polysplit.py stage|lossless"""
import os, re, sys, collections
T = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, T)
from rsitems import lex
import c4p17 as P

V = '️'
MODS = [('univariate', '📏', 'univariate'), ('multivariate', '🕸', 'multivariate'), ('finite', '🧮', 'finite'), ('factor', '🔍', 'factor'), ('roots', '🌱', 'roots'), ('algebraic', '🔢', 'algebraic')]
DOCFIX = {'univariate': [('//! 📈️ Dense univariate polynomials', '//! 📏️ Dense univariate polynomials')], 'algebraic': [('//! 🌱️ Real algebraic numbers', '//! 🔢️ Real algebraic numbers')]}
FILE = '✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📈️polynomial-internals/🦀️.rs'


def orig_lines():
    return open(os.path.join(T, 'orig', 'poly.rs.txt'), encoding='utf-8').read().split('\n')


def find_mods(lines):
    out = []
    for name, emoji, kebab in MODS:
        s = [i for i, l in enumerate(lines) if l == 'pub mod %s {' % name]
        assert len(s) == 1, (name, s)
        e = [i for i in range(s[0] + 1, len(lines)) if lines[i] == '}']
        assert e, name
        e = e[0]
        out.append(dict(name=name, emoji=emoji, folder=emoji + V + kebab, s=s[0], e=e))
    return out


def string_inner_lines(body):
    skip = set()
    for k, x, sl, el, si in lex(body):
        if k == 'str' and el > sl:
            skip.update(range(sl + 1, el + 1))
    return skip


def build():
    lines = orig_lines()
    mods = find_mods(lines)
    children = {}
    for m in mods:
        body = lines[m['s'] + 1:m['e']]
        keep = string_inner_lines('\n'.join(body))
        out = []
        for i, l in enumerate(body):
            if i not in keep and l.startswith('    '):
                l = l[4:]
            elif i not in keep and l.strip() and not l.startswith('    '):
                raise AssertionError(('body line not indented', m['name'], i, l[:60]))
            out.append(l)
        txt = '\n'.join(out)
        txt = re.sub(r'(include!\()"(?!\.\./)([^"/][^"]*)"', r'\1"../\2"', txt)
        for a, b in DOCFIX.get(m['name'], []):
            assert txt.count(a) == 1, (m['name'], a)
            txt = txt.replace(a, b)
        children[m['folder']] = txt.rstrip('\n') + '\n'
    parent = []
    byS = {m['s']: m for m in mods}
    skip = set()
    for m in mods:
        skip.update(range(m['s'], m['e'] + 1))
    for i, l in enumerate(lines):
        if i in byS:
            m = byS[i]
            parent.append('#[path = "%s/🦀️.rs"]' % m['folder'])
            parent.append('pub mod %s;' % m['name'])
        if i in skip:
            continue
        parent.append(l)
    return '\n'.join(parent), children, mods


def stage():
    ptxt, children, mods = build()
    d = os.path.join(P.SCR, 'stage', 'poly')
    import shutil
    shutil.rmtree(d, ignore_errors=True)
    os.makedirs(d)
    open(os.path.join(d, '🦀️.rs'), 'w', encoding='utf-8').write(ptxt)
    for f, t in children.items():
        os.makedirs(os.path.join(d, f))
        open(os.path.join(d, f, '🦀️.rs'), 'w', encoding='utf-8').write(t)
    print('staged poly parent', ptxt.count('\n') + 1, {f: t.count('\n') for f, t in children.items()})


def norm(l):
    return re.sub(r'(include!\()"(?:\.\./)+', r'\1"', l.strip())


def lossless():
    lines = orig_lines()
    d = os.path.join(P.SCR, 'stage', 'poly')
    mods = find_mods(lines)
    new = open(os.path.join(d, '🦀️.rs'), encoding='utf-8').read().split('\n')
    for m in mods:
        new += open(os.path.join(d, m['folder'], '🦀️.rs'), encoding='utf-8').read().split('\n')
    O = collections.Counter(norm(l) for l in lines if l.strip())
    N = collections.Counter(norm(l) for l in new if l.strip())
    miss, extra = O - N, N - O
    decl_miss = collections.Counter()
    for m in mods:
        decl_miss['pub mod %s {' % m['name']] += 1
        decl_miss['}'] += 1
    unexplained_miss = miss - decl_miss
    print('orig non-blank', sum(O.values()), 'new non-blank', sum(N.values()))
    print('missing', dict(miss))
    print('extra', dict(extra))
    allowed_extra = collections.Counter()
    for m in mods:
        allowed_extra['#[path = "%s/🦀️.rs"]' % m['folder']] += 1
        allowed_extra['pub mod %s;' % m['name']] += 1
    for m in mods:
        for a, b in DOCFIX.get(m['name'], []):
            full = [l for l in lines if a in l]
            assert len(full) == 1, a
            allowed_extra[norm(full[0].replace(a, b))] += 1
            decl_miss[norm(full[0])] += 1
    unexplained_miss = miss - decl_miss
    bad_extra = extra - allowed_extra
    rr = re.compile(r'^\s*//\s?#region\b', re.M); re_ = re.compile(r'^\s*//\s?#endregion\b', re.M)
    ot = '\n'.join(lines); nt = '\n'.join(new)
    print('regions orig/new', len(rr.findall(ot)), len(rr.findall(nt)), 'endregions orig/new', len(re_.findall(ot)), len(re_.findall(nt)))
    ok = not unexplained_miss and not bad_extra and len(rr.findall(ot)) == len(rr.findall(nt)) and len(re_.findall(ot)) == len(re_.findall(nt))
    print('LOSSLESS', 'OK' if ok else 'FAIL', 'unexplained missing', dict(unexplained_miss), 'unexplained extra', dict(bad_extra))
    ml = lambda t: sum(1 for l in t.split('\n') if l.strip())
    return ok


if __name__ == '__main__':
    {'stage': stage, 'lossless': lambda: sys.exit(0 if lossless() else 1)}[sys.argv[1]]()
