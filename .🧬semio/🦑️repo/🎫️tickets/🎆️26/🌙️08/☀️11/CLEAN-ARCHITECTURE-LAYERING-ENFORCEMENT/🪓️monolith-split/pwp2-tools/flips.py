#!/usr/bin/env python3
# 🔁️ Flips the interim mounts that source owners placed on the lift files this package now wires: `#[path = ".../🛬️lifted/..."] mod x; pub use x::*;` -> `pub use <crate>::<module>::*;`
import re, os
from common import *
import model as M

LIFT_TARGET = {rel: (key, mod) for key, lst in M.LIFTS_IN.items() for mod, rel, st in lst if st == 'incoming'}
MOUNT = re.compile(r'(?P<attr>#\[path\s*=\s*"(?P<path>[^"]*)"\][ \t]*\n)(?P<vis>pub(?:\([^)]*\))?[ \t]+)?mod[ \t]+(?P<name>\w+)[ \t]*;(?P<tail>[ \t]*\n(?P<use>(?P<uvis>pub(?:\([^)]*\))?[ \t]+)?use[ \t]+(?P=name)::(?P<what>\*|\{[^}]*\}|\w+)[ \t]*;[ \t]*\n?))?')
# real-tree exceptions: a flip that would make a lower crate depend on a higher one is reported, not applied
UPWARD = set()

MINE = {M.ROOT[k] for k in M.WP2} | {f for k in M.WP2 for f in M.CHILDREN[k].values()} | {r for l in M.LIFTS_IN.values() for _, r, _ in l}

def flip(text, file_rel, allow_upward=False, skip=()):
    if file_rel in MINE or file_rel in skip:
        return text, []
    base = os.path.dirname(file_rel)
    hits = []
    def sub(m):
        rel = os.path.normpath(os.path.join(base, m.group('path')))
        tgt = LIFT_TARGET.get(rel)
        if not tgt or (rel in UPWARD and not allow_upward):
            return m.group(0)
        key, mod = tgt
        lib = M.libname(key)
        hits.append((file_rel, rel, key, mod))
        if m.group('use'):
            return f"{m.group('uvis') or 'pub '}use {lib}::{mod}::{m.group('what')};\n"
        return f"{(m.group('vis') or '')}use {lib}::{mod};\n"
    return MOUNT.sub(sub, text), hits
