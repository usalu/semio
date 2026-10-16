#!/usr/bin/env python3
# 🔀️ Interpreter flip: rewrites the Interpreter child files into crate-shaped files for ui-engine-focus, ui-engine-dump and ui-engine,
# writes the three crate roots and turns the Interpreter parent into the facade. usage: ifx.py [--dry]
import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)) + "/w1")
from ed import *
from rs import blank, expand_use, uses
import items, facades

DRY = "--dry" in sys.argv
PFX = "semio_framework_os_renderer_wgpu_"
INT = EL + "/🗣️Interpreter/🎯️targets/🧊️wgpu"
F = facades.scenes()
G = facades.engine_canvas()

def child(name):
    for d in os.listdir(INT):
        if d.endswith(name) and os.path.isdir(INT + "/" + d):
            return INT + "/" + d + "/🦀️.rs"
    raise KeyError(name)

MODS = {
    "ui_engine_focus": {"text_and_table_focus": child("text-and-table-focus"), "ink_and_vfs_focus": child("ink-and-vfs-focus"),
                        "scene_intents_and_clipboard": child("retained-ui-engine") [:-len("/🦀️.rs")] + "/" + [d for d in os.listdir(child("retained-ui-engine")[:-len("/🦀️.rs")]) if d.endswith("scene-intents-and-clipboard")][0] + "/🦀️.rs"},
    "ui_engine_dump": {"introspection": child("introspection"), "chrome_ledger": child("chrome-ledger"), "mesh_and_board2d_stats": child("mesh-and-board2d-stats")},
    "ui_engine": {"retained_ui_engine": child("retained-ui-engine"), "scene_interaction": child("scene-interaction"), "retained_document_consumer": child("retained-document-consumer"), "render_plan_validator": child("render-plan-validator")},
}
FOLDER = {"ui_engine_focus": "👆️ui-engine-focus", "ui_engine_dump": "🔎️ui-engine-dump", "ui_engine": "🖱️ui-engine"}
RANK = {"ui_engine_focus": 0, "ui_engine_dump": 1, "ui_engine": 2}
MOD_CRATE = {m: c for c, ms in MODS.items() for m in ms}
SIDE = {"ui_image_loading": PFX + "scene_canvas2d::ui_image_loading"}
ROOTMODS = {
    "frame_latency": [PFX + "host_base", "frame_latency"],
    "mesh_assets": [PFX + "host_base", "mesh_assets"],
    "runtime_assets": [PFX + "host_base", "runtime_assets"],
    "native_accessibility": [PFX + "native_platform", "native_accessibility"],
    "media_slots": [PFX + "media_slots"],
    "renderer_worker_pool": [PFX + "host_base", "renderer_worker_pool"],
    "app_now_ms": [PFX + "host_base", "app_now_ms"],
    "log_debug": [PFX + "host_base", "log_debug"],
}

def pub_names(paths):
    names = set()
    for p in paths:
        for it in items.table(read(p)):
            if it["kind"] in ("impl", "use"):
                continue
            names.add(it["name"])
    return names

CONTRACT_FILES = [T + "/🤝️contracts/🦀️.rs", T + "/🧬️element-contracts/🦀️.rs"] + [T + "/🤝️contracts/📥️lifted/" + d + "/🦀️.rs" for d in os.listdir(T + "/🤝️contracts/📥️lifted")]
CONTRACT_NAMES = pub_names(CONTRACT_FILES)
INTENT_NAMES = pub_names([MODS["ui_engine_focus"]["scene_intents_and_clipboard"]])

def lib_of(crate):
    return PFX + crate

def facade_path(table, name):
    ent = table.get(name)
    if not ent:
        return None
    return ent[0][1]

def resolve_head(first, rest_names, here):
    """first: module the old path went through; returns replacement prefix tokens (list) for the segments up to and excluding the leaf, or None."""
    raise NotImplementedError

def map_path(tokens, here):
    """tokens: path tokens of a use leaf / code path (e.g. ['super','retained_ui_engine','UI_ENGINE']); returns new tokens or None when untouched."""
    if len(tokens) < 2:
        return None
    a, b = tokens[0], tokens[1]
    rest = tokens[2:]
    if a == "super" and b in MOD_CRATE:
        c = MOD_CRATE[b]
        if b == "retained_ui_engine" and RANK[here] < RANK["ui_engine"]:
            if not rest:
                return None
            n = rest[0]
            if n in CONTRACT_NAMES:
                return [PFX + "contracts"] + rest
            if n in INTENT_NAMES:
                if here == "ui_engine_focus":
                    return ["crate", "scene_intents_and_clipboard"] + rest
                return [lib_of("ui_engine_focus"), "scene_intents_and_clipboard"] + rest
            raise SystemExit(f"upward reference {tokens} from {here}")
        if c == here:
            return ["crate", b] + rest
        if RANK[c] > RANK[here]:
            raise SystemExit(f"upward reference {tokens} from {here}")
        return [lib_of(c), b] + rest
    if a == "super" and b in SIDE:
        return SIDE[b].split("::") + rest
    if a == "crate" and b in ("scenes", "engine_canvas"):
        if not rest:
            return None
        tbl = F if b == "scenes" else G
        full = facade_path(tbl, rest[0])
        if full is None:
            raise SystemExit(f"no facade entry for crate::{b}::{rest[0]}")
        return list(full) + rest[1:]
    if a == "crate" and b in ROOTMODS:
        return list(ROOTMODS[b]) + rest
    return None

def sub_code(text, regex, fn):
    clean = blank(text)
    out, last = [], 0
    for m in regex.finditer(clean):
        r = fn(m, text)
        if r is None:
            continue
        out.append(text[last:m.start()])
        out.append(r)
        last = m.end()
    out.append(text[last:])
    return "".join(out)

USE = re.compile(r"(?m)^((?:#\[[^\n]*\]\n)*)(pub(?:\([^)]*\))?\s+)?use\s+([^;]+);[ \t]*\n")

def rewrite_use(m, text, here):
    attrs, vis, body = text[m.start(1):m.end(1)], text[m.start(2):m.end(2)] if m.group(2) else "", text[m.start(3):m.end(3)]
    leaves = expand_use(body)
    changed = False
    groups = {}
    order = []
    for path, alias, glob in leaves:
        new = map_path(path, here) if len(path) >= 2 else None
        if new is None and path and path[0] == "super" and len(path) >= 2 and path[1] in ("retained_ui_engine",) and glob is False:
            pass
        if new is not None:
            changed = True
        p = new if new is not None else path
        if glob:
            key = ("::".join(p), None)
            order.append((key, "*"))
            continue
        parent, leaf = "::".join(p[:-1]), p[-1]
        piece = leaf if (alias is None or alias == leaf) else f"{leaf} as {alias}"
        order.append(((parent, "lst"), piece))
    if not changed:
        return None
    lines = []
    seen = {}
    for (parent, kind), piece in order:
        if kind is None:
            lines.append(f"{attrs}{vis}use {parent}::*;\n")
        else:
            seen.setdefault(parent, []).append(piece)
    for parent, pieces in seen.items():
        body = pieces[0] if len(pieces) == 1 else "{" + ", ".join(pieces) + "}"
        lines.append(f"{attrs}{vis}use {parent}::{body};\n" if parent else f"{attrs}{vis}use {body};\n")
    return "".join(lines)

CODE_PATH = re.compile(r"(?<![A-Za-z0-9_:])(?:crate|super)(?:::[A-Za-z_][A-Za-z0-9_]*)+")

def rewrite_code_paths(m, text, here):
    toks = text[m.start():m.end()].split("::")
    new = map_path(toks, here)
    if new is None:
        return None
    return "::".join(new)

VIS = re.compile(r"\bpub\((?:crate|super)\)\s")

def widen(text):
    return sub_code(text, VIS, lambda m, t: "pub ")

def transform(path, here):
    text = read(path)
    text = sub_code(text, USE, lambda m, t: rewrite_use(m, t, here))
    text = sub_code(text, CODE_PATH, lambda m, t: rewrite_code_paths(m, t, here))
    text = widen(text)
    return text

def main():
    res = {}
    for crate, mods in MODS.items():
        for mod, path in mods.items():
            res[path] = (crate, transform(path, crate))
    for path, (crate, text) in res.items():
        if DRY:
            old = read(path)
            print(path.split("/")[-2], len(old), len(text))
        else:
            write(path, text)

if __name__ == "__main__":
    main()
