"""⬆️ Test hoist: moves the `#[cfg(test)] mod x_tests;` mounts of a module file into the facade root and gives the facade the names those tests read through `use super::*`."""
import os, re
from model import *
import imports

MOUNT = re.compile(r'(?:#\[cfg\(test\)\]\n)?#\[path = "([^"]+)"\]\n(?:#\[cfg\(test\)\]\n)?mod (\w+);\n?')

def hoist(file, mod_name, facade_path, facade_depth_fix, crate_ident=None, apply=True):
    raw = open(file, encoding='utf8').read()
    mounts = [m for m in re.finditer(r'(?:[ \t]*#\[cfg\(test\)\]\n)?[ \t]*#\[path = "([^"]+)"\]\n(?:[ \t]*#\[cfg\(test\)\]\n)?[ \t]*mod (\w+);\n?', raw) if 'tests' in m.group(1)]
    fac = open(facade_path, encoding='utf8').read()
    top_defs = {item_name(i) for i in items(raw) if item_name(i) and i['kind'] not in ('use', 'impl')}
    new_fac = []; new_imports = []; names_all = set()
    src_uses = imports.uses(raw)
    for m in mounts:
        tfile = os.path.normpath(os.path.join(os.path.dirname(file), m.group(1)))
        ttext = open(tfile, encoding='utf8').read()
        toks = set(re.findall(r'(?<![\w.])([A-Za-z_]\w*)', strip(ttext)))
        names = sorted(toks & top_defs)
        # child-header imports the test reads through `use super::*`
        for u in src_uses:
            if u['alias'] in toks and u['alias'] not in top_defs:
                new_imports.append(u)
        names_all |= set(names)
        newpath = m.group(1).replace('../../../', '../../', 1)
        new_fac.append('#[cfg(test)]\n#[path = "%s"]\nmod %s;\n' % (newpath, m.group(2)))
    if not mounts: return [], []
    out = raw
    for m in reversed(mounts): out = out[:m.start()] + out[m.end():]
    return (out, new_fac, sorted(names_all), new_imports)
