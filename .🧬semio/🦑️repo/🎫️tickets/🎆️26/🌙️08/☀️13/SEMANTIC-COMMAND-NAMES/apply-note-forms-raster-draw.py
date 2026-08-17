#!/usr/bin/env python3
"""Apply semantic command splits for note, forms, raster, draw only."""
from __future__ import annotations

import importlib.util
import os
import re
import sys
from pathlib import Path

REPO = Path("/Users/ueli/Documents/semio")
TICKET = Path(__file__).resolve().parent
COMPONENT = "🦀️component.rs"
GLUE = "📦️glue.rs"

PLUGINS = [
    REPO / "✏️s/🔌️plugins/🗒️note",
    REPO / "✏️s/🔌️plugins/📋️forms",
    REPO / "✏️s/🔌️plugins/🖨️raster",
    REPO / "✏️s/🔌️plugins/🖍️draw",
]
PLUGIN_RELS = [
    "✏️s/🔌️plugins/🗒️note",
    "✏️s/🔌️plugins/📋️forms",
    "✏️s/🔌️plugins/🖨️raster",
    "✏️s/🔌️plugins/🖍️draw",
]

spec = importlib.util.spec_from_file_location("split_commands", TICKET / "🔧️split-commands.py")
sc = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sc)


def ours(rel: str) -> bool:
    return any(rel.startswith(p) for p in PLUGIN_RELS)


def find_ours() -> list[Path]:
    files = []
    for root, dirs, fnames in os.walk(REPO / "✏️s"):
        if "target" in Path(root).parts:
            dirs.clear()
            continue
        if COMPONENT not in fnames:
            continue
        path = Path(root) / COMPONENT
        rel = path.relative_to(REPO).as_posix()
        if f"/{sc.COMMANDS_MARK}/" not in f"/{rel}":
            continue
        after = rel.split(f"/{sc.COMMANDS_MARK}/", 1)[1]
        if after != f"{path.parent.name}/{COMPONENT}":
            continue
        if not ours(rel):
            continue
        files.append(path)
    return sorted(files)


def expand_nudge_macros() -> None:
    path = REPO / "✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🎮️commands/🕹️nudge/🦀️component.rs"
    text = path.read_text(encoding="utf-8")
    if "macro_rules! directional_nudge" not in text:
        return
    specs = [
        ("nudge_selection_up", "NudgeSelectionUp", "nudge-selection-up", "0.0", "-NUDGE_STEP"),
        ("nudge_selection_down", "NudgeSelectionDown", "nudge-selection-down", "0.0", "NUDGE_STEP"),
        ("nudge_selection_left", "NudgeSelectionLeft", "nudge-selection-left", "-NUDGE_STEP", "0.0"),
        ("nudge_selection_right", "NudgeSelectionRight", "nudge-selection-right", "NUDGE_STEP", "0.0"),
        ("nudge_selection_up_fast", "NudgeSelectionUpFast", "nudge-selection-up-fast", "0.0", "-NUDGE_STEP_FAST"),
        ("nudge_selection_down_fast", "NudgeSelectionDownFast", "nudge-selection-down-fast", "0.0", "NUDGE_STEP_FAST"),
        ("nudge_selection_left_fast", "NudgeSelectionLeftFast", "nudge-selection-left-fast", "-NUDGE_STEP_FAST", "0.0"),
        ("nudge_selection_right_fast", "NudgeSelectionRightFast", "nudge-selection-right-fast", "NUDGE_STEP_FAST", "0.0"),
    ]
    blocks = ["//#region 🔖️DirectionalNudges\n"]
    for module, payload, key, dx, dy in specs:
        blocks.append(
            f"pub mod {module} {{\n"
            f"    use super::*;\n\n"
            f"    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]\n"
            f'    #[dsl(keyword = "{key}")]\n'
            f"    pub struct {payload} {{}}\n\n"
            f"    pub fn handle(_payload: &{payload}, doc: &ArtifactView<'_, NoteSnapshot>, cfg: &ConfigView<'_, NoteConfig>) -> Result<Emit<NoteMutation, NoteConfigMutation>, Fault> {{\n"
            f"        Ok(nudge(doc.snapshot, cfg.snapshot, {dx}, {dy}))\n"
            f"    }}\n"
            f"}}\n\n"
        )
    blocks.append("//#endregion 🔖️DirectionalNudges\n")
    expanded = "".join(blocks)
    text = re.sub(
        r"//#region 🔖️DirectionalNudges\n.*?//#endregion 🔖️DirectionalNudges\n",
        expanded,
        text,
        count=1,
        flags=re.S,
    )
    path.write_text(text, encoding="utf-8")
    print("expanded nudge directional macros")


def is_canvas(path: Path) -> bool:
    rel = path.relative_to(REPO).as_posix()
    return "🖍️draw" in rel and "/🖱️canvas/" in rel


def flatten_mod_body(body: str) -> str:
    return sc.drop_super_use(sc.dedent(body))


def canvas_custom(mapping: sc.Mapping, path: Path) -> None:
    text = path.read_text(encoding="utf-8")
    mods = sc.extract_mods(text)
    tests = sc.extract_tests(text)
    uses = sc.file_level_uses(text, mods, tests)
    helpers = sc.strip_tests_and_mods(text, mods, tests)
    # drop module doc + uses from leftover so helpers is the shared machinery
    helper_src = sc.file_level_helpers(text, mods, tests)
    emoji, slug = sc.split_emoji_slug(path.parent.name)
    by_name = {m["name"]: m for m in mods}

    order = [
        "canvas_pointer_down",
        "canvas_pointer_move",
        "canvas_pointer_up",
        "canvas_double_click",
        "canvas_commit_draft",
        "canvas_escape",
    ]
    extra_uses = {
        "canvas_pointer_down": [],
        "canvas_pointer_move": [
            "use crate::apps::draw::commands::canvas_pointer_down::{best_pick_layer_id, canvas_point_to_world, draw_gesture, finish_gesture_emit, resolve_pick_targets_at, DrawSession, DRAW_MARQUEE_THRESHOLD_PX, DRAW_PICK_TOLERANCE_PX};",
            "use serde::{Deserialize, Serialize};",
        ],
        "canvas_pointer_up": [
            "use crate::apps::draw::commands::canvas_pointer_down::{canvas_point_to_world, draw_gesture, finish_gesture_emit, DrawSession};",
            "use serde::{Deserialize, Serialize};",
        ],
        "canvas_double_click": [
            "use crate::apps::draw::commands::canvas_pointer_down::{draw_gesture, finish_gesture_emit, DrawSession};",
            "use serde::{Deserialize, Serialize};",
        ],
        "canvas_commit_draft": [
            "use crate::apps::draw::commands::canvas_pointer_down::{draw_gesture, finish_gesture_emit, DrawSession};",
            "use serde::{Deserialize, Serialize};",
        ],
        "canvas_escape": [
            "use crate::apps::draw::commands::canvas_pointer_down::{draw_gesture, finish_gesture_emit, DrawSession};",
            "use serde::{Deserialize, Serialize};",
        ],
    }

    for name in order:
        mod = by_name[name]
        kebab = sc.snake_to_kebab(name)
        new_folder = f"{emoji}{kebab}"
        new_dir = path.parent.parent / new_folder
        new_file = new_dir / COMPONENT
        body = flatten_mod_body(mod["body"])
        if name == "canvas_pointer_down":
            # keep ALL shared machinery in this command file (DrawSession type identity)
            doc = sc.module_doc(emoji, kebab, text)
            chunks = [doc.rstrip() + "\n"]
            if uses.strip():
                chunks.append("\n" + uses.rstrip() + "\n")
            chunks.append("\n")
            if helper_src.strip():
                chunks.append(helper_src.rstrip() + "\n\n")
            chunks.append(body.rstrip() + "\n")
            if tests:
                chunks.append("\n" + tests.rstrip() + "\n")
            content = re.sub(r"\n{3,}", "\n\n", "".join(chunks))
            if not content.endswith("\n"):
                content += "\n"
        else:
            content = sc.flatten_command_file(
                emoji=emoji,
                kebab=kebab,
                original_text=text,
                uses=uses,
                helpers="",
                body=body,
                tests="",
                extra_uses=extra_uses[name],
            )
        mapping.add(old_file=path, old_mod="canvas", new_file=new_file, new_mod=name, kebab=kebab)
        rec = mapping.by_old[path.relative_to(REPO).as_posix()][-1]
        rec["content"] = content
        rec["new_dir"] = str(new_dir)
        rec["delete_old"] = True


def rewrite_glue_in_plugins(mapping: sc.Mapping) -> int:
    count = 0
    old_to_recs = mapping.by_old
    for plugin in PLUGINS:
        for glue in plugin.rglob(GLUE):
            if "target" in glue.parts:
                continue
            text = glue.read_text(encoding="utf-8")
            orig = text

            def repl(m: re.Match) -> str:
                indent = m.group(1)
                path = m.group(2)
                glue_dir = glue.parent
                abs_target = (glue_dir / path).resolve()
                try:
                    rel = abs_target.relative_to(REPO).as_posix()
                except ValueError:
                    return m.group(0)
                recs = old_to_recs.get(rel)
                if not recs:
                    return m.group(0)
                lines = []
                for rec in recs:
                    new_abs = REPO / rec["new_rel"]
                    new_rel_from_glue = os.path.relpath(new_abs, glue_dir).replace("\\", "/")
                    lines.append(f'{indent}#[path = "{new_rel_from_glue}"]')
                    lines.append(f'{indent}pub mod {rec["new_mod"]};')
                return "\n".join(lines)

            text = sc.PATH_MOD_RE.sub(repl, text)
            # draw glue comment about canvas gesture machine
            text = text.replace(
                "apps::draw::commands::canvas",
                "apps::draw::commands::canvas_pointer_down",
            )
            if text != orig:
                glue.write_text(text, encoding="utf-8")
                count += 1
                print("updated glue", glue.relative_to(REPO).as_posix())
    return count


def rewrite_app_imports() -> None:
    # note
    note = REPO / "✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🦀️component.rs"
    text = note.read_text(encoding="utf-8")
    old = """use crate::apps::note::commands::block::{add_block, delete_block, delete_selection, duplicate_block, duplicate_selection, move_block, patch_blocks};
use crate::apps::note::commands::camera::{set_camera, set_camera_zoom};
use crate::apps::note::commands::drawing::{set_eraser_radius, set_pencil_width};
use crate::apps::note::commands::engagement::{engagement_input, engagement_submit, navigator_engagement_input};
use crate::apps::note::commands::export::{load_request, save_download};
use crate::apps::note::commands::fixture::{set_active_example, set_fixture_json};
use crate::apps::note::commands::grid::{set_grid_opacity, set_grid_spacing, set_grid_subdivisions, set_grid_visible};
use crate::apps::note::commands::ink::ink_apply_events;
use crate::apps::note::commands::locale::set_locale;
use crate::apps::note::commands::nudge::{nudge_selection, nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
use crate::apps::note::commands::selection::{clear_selection, select_all, set_hover, set_selection};
use crate::apps::note::commands::snap::{set_snap_enabled, set_snap_grid_spacing};
use crate::apps::note::commands::utility::set_active_utility;"""
    new = """use crate::apps::note::commands::{add_block, delete_block, delete_selection, duplicate_block, duplicate_selection, move_block, patch_blocks};
use crate::apps::note::commands::{set_camera, set_camera_zoom};
use crate::apps::note::commands::{set_eraser_radius, set_pencil_width};
use crate::apps::note::commands::{engagement_input, engagement_submit, navigator_engagement_input};
use crate::apps::note::commands::{load_request, save_download};
use crate::apps::note::commands::{set_active_example, set_fixture_json};
use crate::apps::note::commands::{set_grid_opacity, set_grid_spacing, set_grid_subdivisions, set_grid_visible};
use crate::apps::note::commands::ink_apply_events;
use crate::apps::note::commands::set_locale;
use crate::apps::note::commands::{nudge_selection, nudge_selection_down, nudge_selection_down_fast, nudge_selection_left, nudge_selection_left_fast, nudge_selection_right, nudge_selection_right_fast, nudge_selection_up, nudge_selection_up_fast};
use crate::apps::note::commands::{clear_selection, select_all, set_hover, set_selection};
use crate::apps::note::commands::{set_snap_enabled, set_snap_grid_spacing};
use crate::apps::note::commands::set_active_utility;"""
    if old not in text:
        raise SystemExit("note app imports block not found")
    note.write_text(text.replace(old, new, 1), encoding="utf-8")

    # forms
    forms = REPO / "✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs"
    text = forms.read_text(encoding="utf-8")
    old = "use crate::apps::forms::commands::{contribution, export, import, locale, option, question, selection, step, try_wizard, vector};"
    new = """use crate::apps::forms::commands::{
    add_question, add_question_option, add_step, add_vector_field, drop_question_kind, export_fixture, move_question, move_step, next_step, patch_question_options,
    patch_questions, patch_step, patch_vector_field, previous_step, remove_question, remove_question_option, remove_step, remove_vector_field, reset_try, set_active_example,
    set_contributions, set_locale, set_selection, set_spec_json, set_try_value, set_try_values, submit, update_form,
};"""
    if old not in text:
        raise SystemExit("forms glob import not found")
    text = text.replace(old, new, 1)
    nested = """use contribution::set_contributions;
use export::export_fixture;
use import::{set_active_example, set_spec_json};
use locale::set_locale;
use option::{add_question_option, patch_question_options, remove_question_option};
use question::{add_question, drop_question_kind, move_question, patch_questions, remove_question};
use selection::set_selection;
use step::{add_step, move_step, patch_step, remove_step, update_form};
use try_wizard::{next_step, previous_step, reset_try, set_try_value, set_try_values, submit};
use vector::{add_vector_field, patch_vector_field, remove_vector_field};"""
    if nested not in text:
        raise SystemExit("forms nested command uses not found")
    text = text.replace(nested, "", 1)
    forms.write_text(text, encoding="utf-8")

    # raster
    raster = REPO / "✏️s/🔌️plugins/🖨️raster/🎛️apps/🖨️raster/🦀️component.rs"
    text = raster.read_text(encoding="utf-8")
    old = """use crate::apps::raster::commands::brush::{set_brush_opacity, set_brush_size};
use crate::apps::raster::commands::camera::{set_camera, set_camera_zoom, set_composite_viewport};
use crate::apps::raster::commands::layer::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::apps::raster::commands::locale::set_locale;
use crate::apps::raster::commands::selection::{select_all, set_hover, set_selection};
use crate::apps::raster::commands::utility::set_active_utility;"""
    new = """use crate::apps::raster::commands::{set_brush_opacity, set_brush_size};
use crate::apps::raster::commands::{set_camera, set_camera_zoom, set_composite_viewport};
use crate::apps::raster::commands::{add_layer, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_layer_visible, toggle_layer_visible};
use crate::apps::raster::commands::set_locale;
use crate::apps::raster::commands::{select_all, set_hover, set_selection};
use crate::apps::raster::commands::set_active_utility;"""
    if old not in text:
        raise SystemExit("raster app imports block not found")
    raster.write_text(text.replace(old, new, 1), encoding="utf-8")

    # draw
    draw = REPO / "✏️s/🔌️plugins/🖍️draw/🎛️apps/🖍️draw/🦀️component.rs"
    text = draw.read_text(encoding="utf-8")
    text = text.replace(
        "use crate::apps::draw::commands::canvas::DrawSession;",
        "use crate::apps::draw::commands::canvas_pointer_down::DrawSession;",
        1,
    )
    old = "use crate::apps::draw::commands::{canvas, document, layer, view};"
    new = """use crate::apps::draw::commands::{
    add_layer, canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up, clear_selection, combine_boolean,
    commit_document, delete_layer, drop_layer_kind, duplicate_layer, engagement_input, engagement_submit, move_layer, patch_layer, patch_layers, select_all, set_active_example,
    set_active_utility, set_camera, set_camera_zoom, set_fixture_json, set_hover, set_locale, set_selected_opacity, set_selection, set_snapshot, toggle_layer_visible,
};"""
    if old not in text:
        raise SystemExit("draw glob import not found")
    text = text.replace(old, new, 1)
    nested = """use document::{commit_document, set_active_example, set_snapshot, set_fixture_json};
use layer::{add_layer, combine_boolean, delete_layer, drop_layer_kind, duplicate_layer, move_layer, patch_layer, patch_layers, set_selected_opacity, toggle_layer_visible};
use view::{clear_selection, engagement_input, engagement_submit, select_all, set_active_utility, set_camera, set_camera_zoom, set_hover, set_locale, set_selection};
use canvas::{canvas_commit_draft, canvas_double_click, canvas_escape, canvas_pointer_down, canvas_pointer_move, canvas_pointer_up};"""
    if nested not in text:
        raise SystemExit("draw nested command uses not found")
    text = text.replace(nested, "", 1)
    draw.write_text(text, encoding="utf-8")
    print("rewrote four app component import blocks")


def rewrite_plugin_paths(mapping: sc.Mapping) -> int:
    # longest first inner-mod transitions + leftover helper relocations
    transitions: list[tuple[str, str]] = []
    helper_moves = [
        ("commands::canvas::", "commands::canvas_pointer_down::"),
        ("commands::question::question_shell", "commands::add_question::question_shell"),
        ("commands::question::default_question_for_kind", "commands::add_question::default_question_for_kind"),
    ]
    for old_rel, recs in mapping.by_old.items():
        old_path = Path(old_rel)
        old_mod = sc.kebab_to_snake(sc.split_emoji_slug(old_path.parent.name)[1])
        for rec in recs:
            inner = rec["new_mod"]
            transitions.append((f"commands::{old_mod}::{inner}", f"commands::{inner}"))
        # glue aliases that differ from folder slug
        if old_mod == "try":
            for rec in recs:
                transitions.append((f"commands::try_wizard::{rec['new_mod']}", f"commands::{rec['new_mod']}"))
        if old_mod == "artifact":
            for rec in recs:
                transitions.append((f"commands::document::{rec['new_mod']}", f"commands::{rec['new_mod']}"))
    transitions.sort(key=lambda t: len(t[0]), reverse=True)

    known_old = set()
    for old_rel, recs in mapping.by_old.items():
        old_mod = sc.kebab_to_snake(sc.split_emoji_slug(Path(old_rel).parent.name)[1])
        known_old.add(old_mod)
        if old_mod == "try":
            known_old.add("try_wizard")
        if old_mod == "artifact":
            known_old.add("document")

    changed = 0
    for plugin in PLUGINS:
        for path in plugin.rglob("*"):
            if "target" in path.parts:
                continue
            if path.suffix not in {".rs", ".md", ".ts", ".json"}:
                continue
            if not path.is_file():
                continue
            text = path.read_text(encoding="utf-8")
            orig = text
            for old, new in transitions:
                text = text.replace(old, new)
            for old, new in helper_moves:
                text = text.replace(old, new)

            def glob_sub(m: re.Match) -> str:
                oldm = m.group(1)
                inner = m.group(2)
                if oldm == "canvas_pointer_down" or oldm not in known_old:
                    return m.group(0)
                if oldm in {"canvas"}:
                    return f"commands::canvas_pointer_down::{{{inner}}}"
                return f"commands::{{{inner}}}"

            text = re.sub(r"commands::([A-Za-z0-9_]+)::\{([^}]+)\}", glob_sub, text)
            if text != orig:
                path.write_text(text, encoding="utf-8")
                changed += 1
    print(f"updated {changed} plugin files for qualified command paths")
    return changed


def fix_nudge_tests(mapping: sc.Mapping) -> None:
    recs = mapping.by_old.get("✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🎮️commands/🕹️nudge/🦀️component.rs")
    if not recs:
        return
    names = [
        "nudge_selection_up",
        "nudge_selection_down",
        "nudge_selection_left",
        "nudge_selection_right",
        "nudge_selection_right_fast",
        "nudge_selection_left_fast",
        "nudge_selection_up_fast",
        "nudge_selection_down_fast",
        "nudge_selection",
    ]
    for rec in recs:
        path = REPO / rec["new_rel"]
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        if "mod tests" not in text:
            continue
        for name in names:
            if name == rec["new_mod"]:
                continue
            text = text.replace(f"{name}::", f"crate::apps::note::commands::{name}::")
        text = text.replace(
            "crate::apps::note::commands::block::add_block::AddBlock",
            "crate::apps::note::commands::add_block::AddBlock",
        )
        path.write_text(text, encoding="utf-8")


def main() -> int:
    expand_nudge_macros()
    files = find_ours()
    print(f"command files: {len(files)}")
    mapping = sc.Mapping()
    for path in files:
        if is_canvas(path):
            canvas_custom(mapping, path)
            continue
        sc.plan_file(path, mapping)
    print(f"planned rewrites from {len(mapping.by_old)} old files")
    sc.DRY = False
    sc.apply_mapping(mapping)
    rewrite_glue_in_plugins(mapping)
    rewrite_app_imports()
    rewrite_plugin_paths(mapping)
    fix_nudge_tests(mapping)
    sc.dump_mapping(mapping, TICKET / "scratch-note-forms-raster-draw-mapping.json")
    return 0


if __name__ == "__main__":
    sys.exit(main())
