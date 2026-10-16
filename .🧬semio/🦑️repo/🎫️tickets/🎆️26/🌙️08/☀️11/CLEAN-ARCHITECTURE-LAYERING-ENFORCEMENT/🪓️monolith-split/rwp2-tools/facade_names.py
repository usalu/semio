"""🏛️ Facade import generator: the names a carved crate exports (column-0 items, thread_local statics, glob-mounted children) that code outside the crate names, grouped by cfg attributes."""
import re, os, sys, collections
from model import *
import oracle2_closure as oc, widen as wd
CFG = re.compile(r'^\s*#\[cfg\(.*\)\]\s*$')
def flat_files(root):
    """Files whose column-0 items live in the crate root namespace: the root and every child mounted with a glob re-export, transitively."""
    out = [root]; stack = [root]
    while stack:
        f = stack.pop(); raw = open(f, encoding='utf8').read()
        for m in re.finditer(r'#\[\s*path\s*=\s*"([^"]+)"\s*\]\s*(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', raw):
            if re.search(r'(?:pub(?:\([^)]*\))?\s+)use\s+' + re.escape(m.group(2)) + r'::\*\s*;', raw):
                child = os.path.normpath(os.path.join(os.path.dirname(f), m.group(1)))
                if child not in out and os.path.isfile(child): out.append(child); stack.append(child)
    return out
def exported(crate):
    root = T + '/' + FOLDER[crate] + '/🦀️.rs'
    out = collections.defaultdict(set)
    for f in flat_files(root):
        if '🧪️tests' in f: continue
        raw = open(f, encoding='utf8').read(); lines = raw.split('\n')
        for it in items(raw):
            names = []
            nm = item_name(it)
            if it['kind'] == 'thread_local!':
                names = re.findall(r'static\s+(\w+)\s*:', '\n'.join(lines[it['start']:it['end'] + 1]))
            elif it['kind'] == 'mod':
                head = lines[it['head']]
                if re.match(r'(?:pub(?:\([^)]*\))?)\s+mod\s', head) and nm and not nm.endswith('_tests') and nm != 'tests': names = [nm]
            elif nm and it['kind'] not in ('use', 'impl'):
                names = [nm]
            attrs = tuple(l.strip() for l in lines[it['start']:it['head']] if CFG.match(l))
            for n in names:
                if n != '_': out[n].add(attrs)
    return out
def facade_uses(crate, exclude=(), pub='pub'):
    ex = exported(crate)
    root = T + '/' + FOLDER[crate] + '/🦀️.rs'
    owned = set(oc.closure(root))
    toks = set()
    for f in wd.all_files():
        if f not in owned: toks |= set(re.findall(r'[A-Za-z_]\w*', strip(open(f, encoding='utf8').read())))
    groups = collections.defaultdict(list)
    for n, cfgs in ex.items():
        if n in toks and n not in exclude:
            for c in cfgs: groups[c].append(n)
    ident = 'semio_framework_os_renderer_wgpu_' + crate.replace('-', '_')
    out = ''
    for c, ns in sorted(groups.items()):
        out += ''.join(a + '\n' for a in c) + '%s use %s::{%s};\n' % (pub, ident, ', '.join(sorted(ns)))
    return out
if __name__ == '__main__':
    print(facade_uses(sys.argv[1]))
