#!/usr/bin/env python3
"""🧪️ Moves the fem<d> editor tests that drive the whole app (the `unit_tests::context` harness) out of their owners'
unit-test trees into the editor crate's integration target `editor_app` (`✏️editor/🧪️tests/🔬️app/🦀️.rs`).

`pre`  (before the crate cut): records each test file's owner module and unmounts it from the owner.
`post` (after the crate cut):  rewrites each test file against the owning crates (`use super::*` becomes the owner's
       public glob plus the owner's own imports) and writes the target's module list.

usage: 🧪️editor-app-tests.py 2d|3d pre|post
"""
import json, os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
D = sys.argv[1]
ART = os.path.join(REPO, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/" + {"2d": "◻️2d", "3d": "🧊️3d"}[D])
ED = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor")
STATE = os.path.join(HERE, "🧪️editor-app-tests.json" if D == "2d" else "🧪️fem3d-editor-app-tests.json")
g = {}
exec(compile(open(os.path.join(HERE, "🕸️module-graph.py"), encoding="utf-8").read().rsplit("\nmain()", 1)[0], "mg", "exec"), g)

LAYERS = [(r.replace("fem2d", "fem" + D), c.replace("fem_2d", "fem_" + D)) for r, c in [
    ("editor::fem2d::commands::(set_camera|set_result_display|canvas_pointer_down|canvas_pointer_move|canvas_pointer_up|focus_entity|result_animation_tick|set_result_animation|gumball)\\b", "semio_s_artifact_fem_2d_editor"),
    ("editor::fem2d::commands\\b", "semio_s_artifact_fem_2d_editor_commands"),
    ("editor::fem2d::interaction\\b", "semio_s_artifact_fem_2d_editor_interaction"),
    ("editor::fem2d::modes::edit::windows::model\\b", "semio_s_artifact_fem_2d_editor_model_window"),
    ("editor::fem2d::modes::edit::windows::results\\b", "semio_s_artifact_fem_2d_editor_results_window"),
    ("editor::fem2d::panels\\b", "semio_s_artifact_fem_2d_editor_panels"),
    ("editor::fem2d::session::visual_job\\b", "semio_s_artifact_fem_2d_editor_visual_job"),
    ("editor::fem2d::session\\b", "semio_s_artifact_fem_2d_editor_session"),
    ("editor::fem2d::transient\\b", "semio_s_artifact_fem_2d_editor_transient"),
    ("editor::fem2d::terminology\\b", "semio_s_artifact_fem_2d_editor_terminology"),
    ("editor::fem2d\\b", "semio_s_artifact_fem_2d_editor"),
]]

def owner_crate(modpath):
    return next(c for r, c in LAYERS if re.match(r, modpath))

def harness_files():
    out = []
    for d, _, fs in os.walk(ED):
        for f in fs:
            p = os.path.join(d, f)
            if f == "🦀️.rs" and "🧪️tests" in p:
                t = open(p, encoding="utf-8").read()
                lower = not re.match(r"(🧪️tests|🌉️wasm|🛠️|📚️|🎭️modes/✏️edit/🦀️|🎮️commands/(🖱️canvas|🎯️focus|🎥️set-camera|⏯️|⏱️|👁️set-result|🧭️gumball))", os.path.relpath(p, ED))
                if "unit_tests::" in t or (lower and re.search(r"create_fem%sd_app|Fem%sdPlayApp|Fem%sdCommand|fem%sd_route" % (D[0], D[0], D[0], D[0]), t)):
                    out.append(p)
    return sorted(out)

def pre():
    crate = g["Crate"](os.path.join(ART, "🦀️.rs"))
    rows = json.load(open(STATE, encoding="utf-8")) if os.path.exists(STATE) else []
    done = {os.path.join(ED, r["test"]) for r in rows}
    for t in harness_files():
        if t in done: continue
        owner = next(f for f in crate.files if any(m.group(1) and os.path.normpath(os.path.join(os.path.dirname(f), m.group(1))) == t for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]', open(f, encoding="utf-8").read())))
        src = open(owner, encoding="utf-8").read()
        relp = os.path.relpath(t, os.path.dirname(owner))
        m = re.search(r'(?:[ \t]*///[^\n]*\n)*[ \t]*#\[cfg\(test\)\]\n[ \t]*#\[path = "' + re.escape(relp) + r'"\]\n[ \t]*(?:pub(?:\([^)]*\))? )?mod ([a-z_0-9]+);\n', src)
        assert m, (owner, relp)
        open(owner, "w", encoding="utf-8").write(src[:m.start()] + src[m.end():])
        modpath = "::".join(x for x in crate.files[owner] if x != "component")
        rows.append({"test": os.path.relpath(t, ED), "owner": os.path.relpath(owner, ED), "module": m.group(1), "modpath": modpath, "crate": owner_crate(modpath)})
    json.dump(rows, open(STATE, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(f"{len(rows)} app tests unmounted")

def owner_uses(owner_src, crate_ident, modpath):
    """🔗️ The owner's top-level `use` items, resolved to absolute paths outside the owner's module."""
    s = g["strip"](owner_src)
    out = []
    for m in re.finditer(r'^use\s+([^;]+);', s, re.M):
        text = owner_src[m.start(1):m.end(1)]
        text = re.sub(r'^crate::', crate_ident + "::", text.strip())
        if text.startswith("super::") or text.startswith("self::"):
            base = modpath.split("::")
            while text.startswith("super::"):
                base.pop(); text = text[len("super::"):]
            if text.startswith("self::"): text = text[len("self::"):]
            text = crate_ident + "::" + "::".join(base + [text])
        out.append(f"use {text};")
    return out

def rewrite_paths(src):
    src = src.replace("crate::editor::fem%sd::unit_tests::context::" % D[0], "crate::context::")
    def sub(m):
        rest = m.group(1)
        return owner_crate(rest) + "::" + rest
    return re.sub(r'\bcrate::(editor::fem%sd(?:::[A-Za-z_][A-Za-z0-9_]*)*)' % D[0], sub, src)

def post():
    rows = json.load(open(STATE, encoding="utf-8"))
    for r in rows:
        t = os.path.join(ED, r["test"]); src = open(t, encoding="utf-8").read()
        if r["test"] == "🧪️tests/🔬️unit/🦀️.rs":
            end = src.index("\nuse super::*;")
            src = src[end + 1:]
        owner_src = open(os.path.join(ED, r["owner"]), encoding="utf-8").read()
        uses = [u for u in owner_uses(owner_src, r["crate"], r["modpath"]) if not u.startswith("use crate::")]
        glob = f"use {r['crate']}::{r['modpath']}::*;"
        src = rewrite_paths(src)
        src = src.replace("use super::*;\n", "\n".join([glob] + uses) + "\n", 1)
        open(t, "w", encoding="utf-8").write(src)
    print(f"rewrote {len(rows)} app tests")

{"pre": pre, "post": post}[sys.argv[2]]()
