"""🧬 rw2t: rewrites one law file (explicit imports for `use super::*;`, crate:: heads to imports) and moves its mount.
usage: apply_law.py <law> [--new-host <suffix>] [--apply]"""
import sys, re
from gen import *
from ed import backup as _bk, note as _note

HEAD_FIX = [  # (regex of the crate:: head, replacement, imported name or None)
 (r"crate::scenes::", "", None),
 (r"crate::interpreter::", "", None),
 (r"crate::element_contracts::", "", None),
 (r"crate::canvas2d_gumball::paint\(", "semio_framework_os_renderer_wgpu_canvas2d_host::paint(", "FULL"),
 (r"crate::(app_now_ms|collect_fixture_actions)\b", r"\1", None),
]
OVERRIDE = {  # law -> {name: path}
 "wgpu-raster-frame-cost": {"paint2d_navigator_overlay_rect": "semio_framework_os_renderer_wgpu_scene_tables::paint2d_navigator_overlay_rect"},
}
EXTRA = {  # law -> names the head rewrites import
 "wgpu-canvas2d": {"canvas_catalogue_drop": "semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::canvas_catalogue_drop", "fixture_scene_pointer_target": "semio_framework_os_renderer_wgpu_contracts::fixture_scene_pointer_target", "app_now_ms": "semio_framework_os_renderer_wgpu_host_base::app_now_ms", "collect_fixture_actions": "semio_framework_os_renderer_wgpu_frame_actions::collect_fixture_actions", "framework_widget_context": "semio_framework_os_renderer_wgpu_contracts::framework_widget_context", "camera_fixture_lease": "semio_framework_os_renderer_wgpu_scene_state::camera_fixture_lease", "SceneCameraDispatchCursor": "semio_framework_os_renderer_wgpu_scene_state::SceneCameraDispatchCursor"},
}

def head_names(text):
    out = {}
    for m in re.finditer(r"crate::(?:scenes|interpreter|element_contracts)::(\w+)", blank(text)):
        out[m.group(1)] = None
    for m in re.finditer(r"crate::(app_now_ms|collect_fixture_actions)\b", blank(text)):
        out[m.group(1)] = None
    return out

_CACHE = {}
def run(law, new_host=None, apply=False):
    setup()
    if new_host:
        register_crate(new_host)
    if "ix" not in _CACHE:
        _CACHE["ix"] = build_index()
        _CACHE["alias"] = alias_table(list(CRATES))
    ix, alias = _CACHE["ix"], _CACHE["alias"]
    lines, notes, r, rewrites = build(law, ix, alias, new_host)
    path = LAWFILES[law]
    text = read(path)
    host = new_host or HOST[law]
    allowed = closure(host) if host in CRATES else set()
    for nm in head_names(text):
        if nm not in lines:
            pp, c = path_of(nm, ix, host, allowed | {host})
            if pp is None:
                notes.append(f"HEAD-NO-OWNER {nm}")
            else:
                lines[nm] = pp
                if len(c) > 1:
                    notes.append(f"HEAD-AMBIG {nm} -> {[(a, b) for a, b, _ in c]} chose {pp}")
    for k, v in OVERRIDE.get(law, {}).items():
        lines[k] = v
    for k, v in EXTRA.get(law, {}).items():
        lines.setdefault(k, v)
    body = text
    for pat, rep, tag in HEAD_FIX:
        body = re.sub(pat, rep, body)
    if body.count("use super::*;") == 0:
        print("###", law, "has no glob import; skipped")
        return None, text, path, lines
    assert body.count("use super::*;") == 1, law
    blk = block(lines)
    body = body.replace("use super::*;", blk, 1)
    left = re.findall(r"crate::\w+", blank(body))
    print("###", law, "->", host, "notes:", len(notes), "left crate::", sorted(set(left)))
    for n in notes:
        print("   ", n)
    return body, text, path, lines

if __name__ == "__main__":
    law = sys.argv[1]
    nh = sys.argv[sys.argv.index("--new-host") + 1] if "--new-host" in sys.argv else None
    body, text, path, lines = run(law, nh)
    print(block(lines))
    if "--apply" in sys.argv:
        _bk(path)
        assert read(path) == text
        with open(path, "w", encoding="utf8") as h:
            h.write(body)
        _note(path)
        print("applied", path[-70:])
