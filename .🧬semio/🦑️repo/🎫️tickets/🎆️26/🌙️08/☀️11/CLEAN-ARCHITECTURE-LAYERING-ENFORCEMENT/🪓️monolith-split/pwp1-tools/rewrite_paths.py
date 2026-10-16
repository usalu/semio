#!/usr/bin/env python3
# 🧭️ Rewrites `crate::<monolith module>::…` (and, in a hub unit's root file, `super::<monolith module>::…`) of carved files to the new crate paths.
# usage: rewrite_paths.py <crate> [--apply]   (reports unresolved first segments)
import os, re, sys
from lib import *
from rs import blank
import wp1files, refs, modmap
from crates_wp1 import OWNER, NESTED, ROOT

# monolith module name -> (crate key, kind) where kind is 'root' when the module file is the crate root, else 'child' (a `pub mod` of the crate root)
TABLE = {
    "artifact_io": ("abi", "root"), "operation_authority": ("abi", "child"), "sqlite_wire": ("abi", "child"), "wasip2": ("abi", "child"),
    "host": ("actor-host", "root"), "document_backbone_binding": ("actor-host", "child"),
    "artifact_identity": ("identity", "root"),
    "local_interaction": ("interaction", "root"), "interaction_topology": ("interaction", "child"),
    "tool_cancellation": ("operation", "root"), "completion_effects": ("operation", "child"), "completion_vacant": ("operation", "child"),
    "window_config": ("window-config", "root"),
    "window_surface": ("window-transient", "root"), "window_mutation": ("window-transient", "child"), "engagement": ("window-transient", "child"), "window_transient": ("window-transient", "child"),
    "window_kits": ("window-kits", "root"), "media_kit": ("window-kits", "child"), "text_kit": ("window-kits", "child"), "image_kit": ("window-kits", "child"), "mesh_kit": ("window-kits", "child"),
    "world3d_sun": ("window-kits", "child"),
    "tree_kit": ("window-kits-rich", "child"), "table_kit": ("window-kits-rich", "child"), "world3d_host": ("window-kits-rich", "child"), "document_kit": ("window-kits-rich", "child"),
}


# item-level moves inside WP1 (plan 5.2 / Appendix J, intra-package groups): (old module, item) -> (new crate, new child module or None for the crate root)
MOVED = {
    ("window_surface", "UiAssemblyResult"): ("interaction", None), ("window_surface", "ui_assembly_error"): ("interaction", None),
    ("window_mutation", "ErasedWindowMutationValue"): ("window-config", "window_mutation"), ("window_mutation", "WindowAuthorityRefreshStep"): ("window-config", "window_mutation"),
    ("window_mutation", "refresh"): ("window-config", "window_mutation"),
    ("world3d_host", "WorldSunConfig"): ("window-kits", "world3d_sun"), ("world3d_host", "world3d_scene"): ("window-kits", "world3d_sun"),
    ("world3d_host", "world3d_environment_json"): ("window-kits", "world3d_sun"),
}


def moved_target(name, item, this_crate):
    crate, child = MOVED[(name, item)]
    base = "crate" if crate == this_crate else lib_name(crate)
    return base if child is None else f"{base}::{child}"


def target(name, this_crate):
    crate, kind = TABLE[name]
    if crate == this_crate:
        return "crate" if kind == "root" else f"crate::{name}"
    lib = lib_name(crate)
    return lib if kind == "root" else f"{lib}::{name}"


def rewrite_text(text, this_crate, unit_root):
    clean = blank(text)
    out, last = [], 0
    unresolved = []
    rx = re.compile(r"(?<![A-Za-z0-9_$:])(crate|super)::([A-Za-z_][A-Za-z0-9_]*)")
    for gm in re.finditer(r"(?<![A-Za-z0-9_$:])(crate|super)::\{([^}]*)\}", clean):
        if gm.group(1) == "crate" or unit_root:
            names = [x.strip().split("::")[0] for x in gm.group(2).split(",") if x.strip()]
            if any(n in TABLE or n in ("app", "io") for n in names):
                unresolved.append((gm.group(1) + "::{group}", gm.group(2).strip()[:90], clean.count("\n", 0, gm.start()) + 1))
    for m in rx.finditer(clean):
        head, name = m.group(1), m.group(2)
        if head == "super" and not unit_root:
            continue
        if name not in TABLE:
            if name not in ("self",):
                unresolved.append((head, name, clean.count("\n", 0, m.start()) + 1))
            continue
        item = re.match(r"::([A-Za-z_][A-Za-z0-9_]*)", clean[m.end():m.end() + 80])
        if item and (name, item.group(1)) in MOVED:
            out.append(text[last:m.start()])
            out.append(moved_target(name, item.group(1), this_crate))
            last = m.end() + len(item.group(0)) - len(item.group(0))
            out.append("")
            # drop the old `::item`'s module part by skipping the matched `::item` and re-adding it
            out.append("::" + item.group(1))
            last = m.end() + len(item.group(0))
            continue
        new = target(name, this_crate)
        rest = clean[m.end():m.end() + 2]
        if rest != "::" and new != "crate" and not new.startswith("crate::") and TABLE[name][1] == "root":
            unresolved.append((head, name + " (bare)", clean.count("\n", 0, m.start()) + 1))
            continue
        if new == "crate" and rest == "::":
            out.append(text[last:m.start()])
            out.append("crate")
            last = m.end()
            continue
        out.append(text[last:m.start()])
        if new == "crate":
            out.append(f"crate")
        else:
            out.append(new)
        last = m.end()
    out.append(text[last:])
    return "".join(out), unresolved


def crate_units(crate):
    mounts = modmap.snapshot_mounts()
    units = {}
    for u, (c, _) in TABLE.items():
        if c == crate:
            units[u] = mounts[u][0] if u in mounts else None
    return units


if __name__ == "__main__":
    crate = sys.argv[1]
    apply = "--apply" in sys.argv
    skip_tests = "--skip-tests" in sys.argv
    for unit, f in crate_units(crate).items():
        if not f:
            continue
        paths = refs.module_paths(unit, f)
        for p, mpath in paths.items():
            text = read(p)
            new, un = rewrite_text(text, crate, len(mpath) == 1)
            if un:
                print("UNRESOLVED", p.replace(PLUG + "/", ""), un[:8])
            if new != text:
                print("rewrite", p.replace(PLUG + "/", ""))
                if apply:
                    write(p, new)
