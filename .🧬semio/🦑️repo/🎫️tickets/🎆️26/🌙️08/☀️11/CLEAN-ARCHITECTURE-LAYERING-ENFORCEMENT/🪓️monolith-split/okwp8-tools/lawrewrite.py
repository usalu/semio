"""✍️ Splits kernel exact-law groups so each moved law addresses the companion crate that now hosts it."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import wp8lib as L, lawgroups as G
PKG = 'package: "semio-framework-os-kernel"'
def split_group(g, indent):
    lm = re.search(r'laws:\s*\[', g)
    if not lm: return None
    a = lm.end(); b = g.index(']', a)
    span = g[a:b]
    items = re.findall(r'"([A-Za-z_][A-Za-z0-9_:]*)"', span)
    multi = '\n' in span
    per = collections.OrderedDict()
    stay = []
    for x in items:
        last = x.split('::')[-1]
        cs = G.name2crate.get(last)
        if cs:
            assert len(cs) == 1, (x, cs)
            per.setdefault(cs[0], []).append(last)
        else:
            stay.append(x)
    if not per: return None
    def render(laws):
        if multi:
            lines = span.split('\n')
            item_indent = re.match(r'\s*', lines[1]).group(0) if len(lines) > 2 else indent + '  '
            close_indent = lines[-1]
            inner = '\n' + ''.join('%s"%s",\n' % (item_indent, x) for x in laws) + close_indent
        else:
            inner = ', '.join('"%s"' % x for x in laws)
        return g[:a] + inner + g[b:]
    out = []
    if stay: out.append(render(stay))
    for crate, laws in per.items():
        t = render(laws)
        t = t.replace(PKG, 'package: "semio-framework-%s"' % crate)
        t = re.sub(r'target:\s*\{\s*kind:\s*"lib"\s*,\s*name:\s*"semio_framework_os_kernel"\s*\}', 'target: { kind: "lib" }', t)
        t = re.sub(r',?\s*cargoArgs:\s*\[[^\]]*\]', '', t)
        out.append(t)
    return out
def rewrite(path, apply):
    t = open(path, encoding='utf8').read()
    gs = G.groups(t)
    changed = 0
    for s, e in reversed(gs):
        ls = t.rfind('\n', 0, s) + 1
        indent = re.match(r'\s*', t[ls:s]).group(0) if t[ls:s].strip() == '' else '        '
        res = split_group(t[s:e], indent)
        if res is None: continue
        sep = ',\n' + indent if t[ls:s].strip() == '' else ', '
        t = t[:s] + sep.join(res) + t[e:]
        changed += 1
    if apply and changed:
        open(path, 'w', encoding='utf8').write(t)
        with open(os.path.dirname(os.path.abspath(__file__)) + '/touched.txt', 'a', encoding='utf8') as f:
            f.write('M\t' + path.replace(L.R + '/', '') + '\n')
    return changed, t
if __name__ == '__main__':
    apply = sys.argv[1] == '--apply'
    for f in sys.argv[2:]:
        n, t = rewrite(f, apply)
        print(f[-70:], 'groups rewritten', n)
