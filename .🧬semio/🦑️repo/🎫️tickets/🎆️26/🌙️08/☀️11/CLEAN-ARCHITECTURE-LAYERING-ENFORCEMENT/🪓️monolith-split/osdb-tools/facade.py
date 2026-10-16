"""🪞 The thin facade crate `semio-framework-os-kernel-db` (lib `db`): explicit re-exports of the surface the hub uses.

Scans the hub sources for `db::...` paths, resolves every name to the crate that now defines it and writes the facade root.
"""
import collections
import json
import os
import re

from lib import *
from rs import strip
from scope import FACADE_FAMILY

HUB = f'{REPO}/🌎️hub'
FAMILY_SKIP = {'semio_framework_async'}
RENAME = {'db_storage': 'storage', 'db_engine': 'engine'}


def hub_files():
    out = []
    for root, ds, fs in os.walk(HUB):
        ds[:] = [d for d in ds if d not in ('node_modules', 'target', 'dist', '.git')]
        for f in fs:
            if f.endswith('.rs'):
                out.append(os.path.join(root, f))
    return out


def scan_hub():
    """Returns (root_names, family_names: {family: set(names)}, files_needing_rename)."""
    root = set()
    fam = collections.defaultdict(set)
    rename_files = []
    for p in hub_files():
        t = strip(read(p))
        if not re.search(r'(?<![A-Za-z0-9_:])db::', t):
            continue
        rename_files.append(p)
        for m in re.finditer(r'(?<![A-Za-z0-9_:])db::([A-Za-z_][A-Za-z0-9_]*)(?:::([A-Za-z_][A-Za-z0-9_]*))?', t):
            a, b = m.group(1), m.group(2)
            a = RENAME.get(a, a)
            if a in FACADE_FAMILY or a in ('storage', 'engine'):
                if b:
                    fam[a].add(b)
            elif a in FAMILY_SKIP:
                continue
            else:
                root.add(a)
        for m in re.finditer(r'(?<![A-Za-z0-9_:])use\s+db::([A-Za-z_][A-Za-z0-9_]*)::\{([^}]*)\}', t):
            a = RENAME.get(m.group(1), m.group(1))
            for part in m.group(2).split(','):
                nm = part.strip().split(' as ')[0].strip()
                if nm and re.match(r'[A-Za-z_]', nm):
                    fam[a].add(nm)
    return root, fam, rename_files


def render(composer, root, fam):
    r = composer.resolver
    out = ['//! 🗄️ Db facade: the stable `db` surface the hub consumes, re-exported explicitly from the crates that define it.', '#![cfg(not(target_arch = "wasm32"))]', '']
    out.append('pub use semio_framework_async;')
    by_lib = collections.defaultdict(list)
    for n in sorted(root):
        if n in r.scope.root:
            p = r.root(n)
            if len(p) == 2:
                by_lib[p[0]].append(n)
            else:
                out.append(f'pub use {"::".join(p)};')
    for lib in sorted(by_lib):
        out.append(f'pub use {lib}::{{{", ".join(by_lib[lib])}}};')
    out.append('')
    for family in sorted(fam):
        module = FACADE_FAMILY.get(family, 'db_' + family)
        names = sorted(fam[family])
        groups = collections.defaultdict(list)
        for n in names:
            try:
                lib = r.lib(module, n) if r.split(module) else module
            except KeyError:
                lib = module
            groups[lib].append(n)
        out.append(f'pub mod {family} {{')
        for lib in sorted(groups):
            ns = sorted(groups[lib])
            out.append(f'    pub use {lib}::{{{", ".join(ns)}}};' if len(ns) > 1 else f'    pub use {lib}::{ns[0]};')
        out.append('}')
        out.append('')
    return '\n'.join(out)


if __name__ == '__main__':
    root, fam, files = scan_hub()
    print(sorted(root))
    print({k: sorted(v) for k, v in fam.items()})
    print(len(files))
