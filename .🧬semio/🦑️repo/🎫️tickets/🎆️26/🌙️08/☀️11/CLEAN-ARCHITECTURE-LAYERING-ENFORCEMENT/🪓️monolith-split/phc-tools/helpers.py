#!/usr/bin/env python3
"""🧰️ Shared helpers for the hub carve plan scripts: item removal, mount rewriting, new-file creation."""
import os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import carve, hcmod
from carve import parse_items, attached_start, REPO

HUB = REPO + '/🌎️hub/'


def P(rel):
    return HUB + rel


def remove_items(ws, file, pred, keep_blank=True):
    src = ws.read(file)
    its, toks = parse_items(src)
    spans = []
    for it in its:
        if pred(it, toks):
            a = attached_start(src, it)
            b = it.b + (1 if it.b < len(src) and src[it.b] == '\n' else 0)
            spans.append((a, b, it))
    for a, b, it in sorted(spans, key=lambda x: -x[0]):
        src = src[:a] + src[b:]
    src = re.sub(r'\n{3,}', '\n\n', src)
    ws.write(file, src)
    return [it for a, b, it in spans]


def use_glob_path(it, toks=None):
    """For a `use a::b::*;` item return ['a','b'] else None."""
    leaves, _ = hcmod.parse_use(list(it.toks_all), next(i for i, t in enumerate(it.toks_all) if t.x == 'use'), len(it.toks_all))
    out = []
    for segs, binding, glob in leaves:
        if glob:
            out.append(segs)
    return out


def drop_mounts(ws, file, mods, glob_reexports=True):
    """Remove `mod NAME;` mount items and the `use NAME::*` / `pub use NAME::*` re-exports of the dropped modules."""
    mods = set(mods)

    def pred(it, toks):
        if it.kind == 'mod' and it.name in mods:
            return True
        if glob_reexports and it.kind == 'use':
            try:
                globs = use_glob_path(it)
            except StopIteration:
                return False
            return any(len(g) == 1 and g[0] in mods for g in globs)
        return False

    return remove_items(ws, file, pred)


def privatize_glob_reexports(ws, file, mods):
    src = ws.read(file)
    for m in mods:
        src = re.sub(r'(?m)^pub use ' + re.escape(m) + r'::\*;', 'use ' + m + '::*;', src)
    ws.write(file, src)


def create(ws, file, text):
    if ws.exists(file):
        raise FileExistsError(file)
    ws.write(file, text)


def append(ws, file, text):
    src = ws.read(file)
    ws.write(file, src.rstrip('\n') + '\n\n' + text.strip('\n') + '\n')


def replace_once(ws, file, old, new):
    s = ws.read(file)
    if s.count(old) != 1:
        raise ValueError(f'{os.path.relpath(file, REPO)}: expected one occurrence of {old[:70]!r}, found {s.count(old)}')
    ws.write(file, s.replace(old, new))
