"""🔗️ First-party edge check: renderer crates a crate's source names against the dependencies its manifest declares and the plan's rank order."""
import re, sys, json, subprocess, os
from model import *
import oracle2_closure as oc
import deps_check as dc
PFX = 'semio-framework-os-renderer-wgpu-'
def declared(crate):
    man = dc.toml(T + '/' + FOLDER[crate] + '/📦️packages/🦀️rust/Cargo.toml')
    out = set()
    tabs = [man.get('dependencies', {})] + [v.get('dependencies', {}) for v in man.get('target', {}).values()]
    for tb in tabs:
        for k in tb:
            if k.startswith(PFX): out.add(k[len(PFX):])
    return out
def used(crate):
    names = set()
    for f in oc.closure(T + '/' + FOLDER[crate] + '/🦀️.rs'):
        for m in re.finditer(r'semio_framework_os_renderer_wgpu_([a-z0-9_]+)', strip(open(f, encoding='utf8').read())): names.add(m.group(1).replace('_', '-'))
    return names
if __name__ == '__main__':
    for c in sys.argv[1:]:
        d, u = declared(c), used(c)
        print(c, 'undeclared:', sorted(u - d), 'declared-unused:', sorted(d - u))
