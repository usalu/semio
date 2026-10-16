#!/usr/bin/env python3
# 🗺️ Mapping of the old renderer-root paths (`crate::<module>::…`) the Shell tree uses to the owning crate paths.
import os, re, sys, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from model import *
EL = E + "/🧱️elements"
import facades

PFX = "semio_framework_os_renderer_wgpu_"
RENDERER_ROOT = R + "/🦀️.rs"


def host_base_names():
    names = set()
    for cfg, vis, p, local, glob in uses(read(RENDERER_ROOT), top_only=True):
        if glob or not p:
            continue
        if p[0] == PFX + "host_base" and len(p) == 2:
            names.add(local)
    return names


HB = host_base_names()
SCENES = facades.scenes()
ENGINE = facades.engine_canvas()
INTERP = facades.facade_map(EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs")

MODULE_TO_PATH = {
    "dock": [PFX + "dock"],
    "program_bridge": [PFX + "program_bridge"],
    "media_slots": [PFX + "media_slots"],
    "element_contracts": [PFX + "contracts"],
    "agent_bridge": [PFX + "agent_elements", "agent_bridge"],
    "agent_presence": [PFX + "agent_elements", "agent_presence"],
    "agent_approvals": [PFX + "agent_elements", "agent_approvals"],
    "agent_chat_panel": [PFX + "agent_elements", "agent_chat_panel"],
    "agent_bridge_door": [PFX + "agent_elements", "agent_bridge_door"],
    "hub_connection": [PFX + "hub_workspace", "hub_connection"],
    "canvas_presence": [PFX + "hub_workspace", "canvas_presence"],
    "space_administration": [PFX + "hub_workspace", "space_administration"],
    "hub_sign_in": [PFX + "hub_contracts", "hub_sign_in"],
    "space_browser": [PFX + "hub_contracts", "space_browser"],
    "socket_door": [PFX + "doors", "socket_door"],
    "directory_door": [PFX + "doors", "directory_door"],
    "native_accessibility": [PFX + "native_platform", "native_accessibility"],
    "frame_latency": [PFX + "host_base", "frame_latency"],
    "mesh_assets": [PFX + "host_base", "mesh_assets"],
    "runtime_assets": [PFX + "host_base", "runtime_assets"],
    "plugin_catalog": [PFX + "host_base", "plugin_catalog"],
    "shell_vocab": [PFX + "shell_vocab"],
    "icon_atlas": [PFX + "icon_host"],
    "fixture_original_turn": [PFX + "ui_engine_tests", "fixture_original_turn"],
    "canvas2d_gumball": [PFX + "canvas2d_host"],
    "kernel_runtime": [PFX + "kernel_runtime"],
    "scale_bench": [PFX + "kernel_runtime", "scale_bench"],
    "parallel_runtime": [PFX + "kernel_protocol", "parallel_runtime"],
    "native_runtime_modules": [PFX + "program_bridge", "native_runtime_modules"],
    "native_temporal": [PFX + "native_platform", "native_temporal"],
    "runtime_mailbox_core": [PFX + "mailbox_core"],
    "canvas2d_paint": [PFX + "canvas2d_paint"],
    "asset_authority": [PFX + "asset_authority"],
    "glb_decode": [PFX + "asset_decode"],
}

ROOT_GLOB_CRATES = ["asset-authority", "asset-decode"]


def root_glob_index():
    import modtree
    import items as itm
    table = crate_table()
    idx = {}
    for c in ROOT_GLOB_CRATES:
        root = T + "/" + table[c] + "/🦀️.rs"
        text = read(root)
        globbed = set(re.findall(r"pub(?:\([a-z]+\))? use ([A-Za-z_0-9]+)::\*;", blank(text)))
        files = [root]
        for m in modtree.MOUNT.finditer(blank(text)):
            if m.group(2) in globbed:
                pm = modtree.PATH_ATTR.search(text[m.start(1):m.end(1)])
                if pm:
                    files.append(os.path.normpath(os.path.join(os.path.dirname(root), pm.group(1))))
        for f in files:
            for it in itm.table(read(f)):
                if it["kind"] in ("impl", "use") or not it["vis"].startswith("pub"):
                    continue
                idx.setdefault(it["name"], PFX + c.replace("-", "_"))
    return idx


def map_crate_path(tokens):
    """tokens: ['crate', head, ...]; returns the new token list or None when the path is not a renderer-root path."""
    if len(tokens) < 2 or tokens[0] != "crate":
        return None
    head, rest = tokens[1], tokens[2:]
    if head in MODULE_TO_PATH:
        return MODULE_TO_PATH[head] + rest
    if head in ("interpreter", "scenes", "engine_canvas"):
        if not rest:
            raise KeyError(f"bare facade path crate::{head}")
        table = {"interpreter": INTERP, "scenes": SCENES, "engine_canvas": ENGINE}[head]
        ent = table.get(rest[0])
        if not ent:
            raise KeyError(f"no facade entry for crate::{head}::{rest[0]}")
        return list(ent[0][1]) + rest[1:]
    if head in HB:
        return [PFX + "host_base", head] + rest
    raise KeyError(f"unknown root path crate::{head}")


if __name__ == "__main__":
    print(len(HB), len(SCENES), len(ENGINE), len(INTERP))
