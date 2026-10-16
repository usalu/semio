"""Narrow-function analysis and the world facade."""
import sys, os, re, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from wgen import *
from wext import external_world_names
import w10, wengine as W
from wrewrite import assign_modes, comp_list, TYNAME
from wengine import resolve_calls, fixpoint, apply_hints, STATE_TY

def narrow_api(E):
    allfns, files = w10.load_world_from(w10.SNAP)
    defs, unres = resolve_calls(allfns)
    apply_hints(allfns, files)
    fixpoint(allfns, defs)
    assign_modes(allfns)
    out = {}
    for f in allfns:
        if f.parent is None and f.has_param and f.mode == 'narrow' and f.name in E:
            out[f.name] = (f, files[f.file])
    return out, allfns

def build_facade(crates, wdef, defcfg, E, outroot):
    lines = ["//! 🌍️ Application-neutral 3D world canvas: the facade over the `semio-framework-os-infinite-world-*` producer crates.", "//!", "//! Re-exports exactly the names the engine, renderer and plugins import; the implementation lives in the crates below.", ""]
    items = []
    for nm in sorted(E):
        if nm in wdef:
            variants = {v for v in defcfg.get(nm, [()])}
            attrs = next(iter(variants)) if len(variants) == 1 else ()
            items.append((wdef[nm], nm, tuple(attrs)))
    groups = collections.OrderedDict()
    for ci, nm, attrs in sorted(items, key=lambda x: (x[0], x[2], x[1].lower())):
        groups.setdefault((ci, attrs), []).append(nm)
    for (ci, attrs), ns in groups.items():
        ident = crate_ident(CRATES[ci][0])
        lines += list(attrs)
        lines.append(f"pub use {ident}::{{{', '.join(ns)}}};" if len(ns) > 1 else f"pub use {ident}::{ns[0]};")
    return '\n'.join(lines) + '\n'
