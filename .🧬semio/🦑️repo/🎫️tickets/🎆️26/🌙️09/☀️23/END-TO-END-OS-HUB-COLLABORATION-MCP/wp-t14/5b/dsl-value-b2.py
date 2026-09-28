#!/usr/bin/env python3
"""🧾️ T13 item 5b, phase B2 (T14): removes the action-bus bridge `optional_json_to_dsl(Option<serde_json::Value>)` — an
exported API whose signature needs a third-party type — after phase A + B1 (`dsl-value.py`, `dsl_value!` present).

Guest + SDK code builds action args as `DslValue` natively: every app wrapper and SDK measure closure takes
`Option<DslValue>`, their literal callers become `dsl_value!`, the lowpoly callers drop their DslValue→JSON→DslValue round
trip, puzzle3d/5d closures drop theirs, layout's interaction args and the infinite world's action args are built as DslValue
(JSON text payloads through the first-party `pack::json::to_json_string`). The renderer's literal-arg builders
(`scene_action`, `block_list_action`, `canvas_addressed_action`) take `DslValue` with `dsl_value!` callers; the renderer's
JSON-sourced args (parsed operations, fixture vectors, serde maps of world3d cancel/effective args, tutorial history args)
convert once where they enter the descriptor through the value module's own `DslValue::from(serde_json::Value)`.

usage: dsl-value-b2.py [--write] [--diff] [--root <tree>]   (default: dry run on the live tree; requires phase A + B1 in the tree)"""
import re
import sys
from pathlib import Path

args = sys.argv[1:]
ROOT = Path(args[args.index("--root") + 1]) if "--root" in args else Path("/Users/ueli/Documents/semio")
WRITE = "--write" in args
problems, edits, notes = [], {}, []
MAC = "semio_framework::dsl_value!"
DV = "semio_framework::DslValue"
PL = "✏️s/🔌️plugins/"
ANY = "🏅️standards/🔖️1/🪆️subsets/✳️any/"
OS = "🧰️framework/🛍️products/💻️os/🔨️modules/"
RE = OS + "📺️renderer/🧑‍🎨engine/🧱️elements/"


def path_of(rel):
    return ROOT / rel


def text_of(rel):
    path = path_of(rel)
    return edits.get(path) or path.read_text(encoding="utf-8")


def put(rel, text):
    path = path_of(rel)
    if text != path.read_text(encoding="utf-8") or path in edits:
        edits[path] = text


def exact(rel, old, new, count=1):
    text = text_of(rel)
    found = text.count(old)
    if new and new in text and (old in new or found == 0):
        return
    if found != count:
        problems.append(f"{rel}: anchor found {found} times (expected {count}): {old.strip()[:90]!r}")
        return
    put(rel, text.replace(old, new))


def split_args(text, start):
    depth, index, spans, begin = 1, start, [], start
    while index < len(text) and depth:
        char = text[index]
        if char in "([{":
            depth += 1
        elif char in ")]}":
            depth -= 1
            if depth == 0:
                spans.append((begin, index))
                return spans, index
        elif char == "|" and depth == 1:
            closing = text.index("|", index + 1)
            index = closing
        elif char == "," and depth == 1:
            spans.append((begin, index))
            begin = index + 1
        elif char == '"':
            index += 1
            while index < len(text) and text[index] != '"':
                index += 2 if text[index] == "\\" else 1
        elif char == "'" and re.match(r"'(\\.|[^\\'])'", text[index:index + 4]):
            index = text.index("'", index + 1)
        index += 1
    return spans, index


LITERAL = re.compile(r"(\s*)(?:serde_json::|store::|pack::)?json!\((.*)\)(\s*)", re.S)
OPTION_LITERAL = re.compile(r"(\s*)Some\(\s*(?:serde_json::|store::|pack::)?json!\((.*)\)\s*\)(\s*)", re.S)
ROUND_TRIP = re.compile(r"(\s*)Some\(\(&(dsl::DslValue::object\(.*\))\)\.into\(\)\)(\s*)", re.S)


def rewrite_calls(rel, name, index, mode, expect_other=0, wrap_other=False):
    """Rewrites argument `index` of every `name(…)` call in `rel`: mode `option` (`Some(json!(L))` → `Some(dsl_value!(L))`,
    `Some((&dsl::DslValue::object(…)).into())` → `Some(dsl::DslValue::object(…))`, `None` kept) or `plain` (`json!(L)` →
    `dsl_value!(L)`). Any other argument form is listed; `expect_other` is how many of those the table handles elsewhere."""
    text = text_of(rel)
    out, cursor, other = [], 0, []
    for match in re.finditer(rf"(?<![A-Za-z0-9_]){re.escape(name)}\(", text):
        if match.start() < cursor or text[max(0, match.start() - 3):match.start()] == "fn ":
            continue
        spans, _ = split_args(text, match.end())
        if len(spans) <= index:
            continue
        begin, end = spans[index]
        argument = text[begin:end]
        if argument.strip() == "None" or MAC in argument or (mode == "option" and "dsl::DslValue::object(" in argument and ".into()" not in argument):
            continue
        replacement = None
        if mode == "option":
            literal = OPTION_LITERAL.fullmatch(argument)
            trip = ROUND_TRIP.fullmatch(argument)
            if literal:
                replacement = f"{literal.group(1)}Some({MAC}({literal.group(2)})){literal.group(3)}"
            elif trip:
                replacement = f"{trip.group(1)}Some({trip.group(2)}){trip.group(3)}"
        else:
            literal = LITERAL.fullmatch(argument)
            if literal:
                replacement = f"{literal.group(1)}{MAC}({literal.group(2)}){literal.group(3)}"
        if replacement is None and wrap_other and mode == "plain":
            replacement = f"{DV}::from({argument.strip()})"
            notes.append(f"  wrapped {rel}:{text.count(chr(10), 0, begin) + 1}: {' '.join(argument.split())[:90]}")
        if replacement is None:
            other.append(f"{rel}:{text.count(chr(10), 0, begin) + 1}: {' '.join(argument.split())[:110]}")
            continue
        out.append(text[cursor:begin])
        out.append(replacement)
        cursor = end
    if cursor:
        out.append(text[cursor:])
        put(rel, "".join(out))
    if len(other) != expect_other:
        problems.append(f"{rel}: {name} has {len(other)} unhandled argument forms (expected {expect_other})")
    notes.extend(f"  other {row}" for row in other)


value = text_of("🧰️framework/🔨️modules/🌱️value/🦀️.rs")
if "macro_rules! dsl_value" not in value:
    raise SystemExit("phase A (dsl-value.py) is not in this tree — apply it first")

# 🌉️ The bridge itself.
exact("🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs", """/// 🌉️ Bridges staged JSON action args into the owned DSL boundary.
pub fn optional_json_to_dsl(args: Option<serde_json::Value>) -> Option<DslValue> {
    args.map(DslValue::from)
}

""", "")

# 🔌️ Plugin SDK: region note, declared-verb probe, world-3d measure closures.
SDK = OS + "🔌️plugin/🦀️.rs"
exact(SDK, """    // 🎯️ Shared ~30x hand-rolled `fn x_action(action: &str, args: Option<Value>) -> ActionDescriptor {
    // ActionDescriptor { controller_id: X_CONTROLLER_ID.into(), action: action.into(), args:
    // optional_json_to_dsl(args) } }` body — every app keeps its own locally-named wrapper (so call
""", """    // 🎯️ Shared ~30x hand-rolled `fn x_action(action: &str, args: Option<DslValue>) -> ActionDescriptor {
    // ActionDescriptor { controller_id: X_CONTROLLER_ID.into(), action: action.into(), args } }` body —
    // every app keeps its own locally-named wrapper (so call
""")
exact(SDK, "let example = semio_framework::optional_json_to_dsl(examples[\"verbs\"].get(&action.id).cloned()).unwrap_or(DslValue::Object(Vec::new()));",
      "let example = examples[\"verbs\"].get(&action.id).map(DslValue::from).unwrap_or(DslValue::Object(Vec::new()));")
exact(SDK, "is_de: bool, action: impl Fn(&str, Option<Value>) -> ActionDescriptor) -> WindowMeasure {", "is_de: bool, action: impl Fn(&str, Option<DslValue>) -> ActionDescriptor) -> WindowMeasure {")
exact(SDK, "p: &WorldProjectionConfig, action: impl Fn(&str, Option<Value>) -> ActionDescriptor) -> WindowMeasure {", "p: &WorldProjectionConfig, action: impl Fn(&str, Option<DslValue>) -> ActionDescriptor) -> WindowMeasure {")
exact(SDK, 'action("setProjection", Some(json!({ "field": field })))', f'action("setProjection", Some({MAC}({{ "field": field }})))')
exact(SDK, 'action("setProjectionParam", Some(json!({ "param": param })))', f'action("setProjectionParam", Some({MAC}({{ "param": param }})))')
HOST_UNIT = OS + "🔌️plugin/🧪️tests/🔬️world3d-host-unit/🦀️.rs"
exact(HOST_UNIT, '|action, args| ActionDescriptor { controller_id: "t".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }', '|action, args| ActionDescriptor { controller_id: "t".into(), action: action.into(), args }')
exact(HOST_UNIT, '|action: &str, args: Option<serde_json::Value>| ActionDescriptor { controller_id: "t".into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }', f'|action: &str, args: Option<{DV}>| ActionDescriptor {{ controller_id: "t".into(), action: action.into(), args }}')

# ♾️ Infinite world-3d host.
WORLD = OS + "♾️infinite/🌍️world/🦀️.rs"
exact(WORLD, "use semio_framework::{optional_json_to_dsl, GranularityDefinition,", "use semio_framework::{GranularityDefinition,")
exact(WORLD, """fn action_args(value: serde_json::Value) -> Option<semio_framework::DslValue> {
    optional_json_to_dsl(Some(value))
}

""", "")
world = text_of(WORLD)
if "action_args(" in world:
    out, cursor, count = [], 0, 0
    for match in re.finditer(r"(?<![A-Za-z0-9_])action_args\(", world):
        spans, close = split_args(world, match.end())
        literal = LITERAL.fullmatch(world[spans[0][0]:spans[0][1]]) if len(spans) == 1 else None
        if literal is None:
            continue
        out.append(world[cursor:match.start()])
        out.append(f"Some({MAC}({literal.group(2)}))")
        cursor = close + 1
        count += 1
    out.append(world[cursor:])
    put(WORLD, "".join(out))
    notes.append(f"  world action_args literals: {count}")
exact(WORLD, """            let mut args = json!({
                "surfaceId": state.surface_id,
                "windowId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "sx": 1.0,
                "sy": 1.0,
                "sz": 1.0,
            });
            if handle == GumballHandle::ScaleX {
                args["sx"] = json!(scale);
            } else if handle == GumballHandle::ScaleY {
                args["sy"] = json!(scale);
            } else {
                args["sz"] = json!(scale);
            }
            return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "scaleSelection".into(), args: action_args(args) });""",
      f"""            let (sx, sy, sz) = match handle {{
                GumballHandle::ScaleX => (scale, 1.0, 1.0),
                GumballHandle::ScaleY => (1.0, scale, 1.0),
                _ => (1.0, 1.0, scale),
            }};
            let args = {MAC}({{
                "surfaceId": state.surface_id,
                "windowId": state.surface_id,
                "mode": selection_mode_label(state),
                "ids": ids,
                "sx": sx,
                "sy": sy,
                "sz": sz,
            }});
            return Some(ActionDescriptor {{ controller_id: state.controller_id.clone(), action: "scaleSelection".into(), args: Some(args) }});""")
exact(WORLD, """fn world_brush_mesh_args(surface_id: &str, extra: serde_json::Value) -> Option<semio_framework::DslValue> {
    let mut args = json!({ "surfaceId": surface_id, "windowId": surface_id });
    let (Some(object), Some(extra)) = (args.as_object_mut(), extra.as_object()) else { return None };
    for (key, value) in extra {
        object.insert(key.clone(), value.clone());
    }
    action_args(args)
}""", f"""fn world_brush_mesh_args(surface_id: &str, extra: {DV}) -> Option<{DV}> {{
    let {DV}::Object(extra) = extra else {{ return None }};
    let mut entries = vec![("surfaceId".to_string(), {DV}::String(surface_id.to_string())), ("windowId".to_string(), {DV}::String(surface_id.to_string()))];
    entries.extend(extra.into_iter().filter(|(key, _)| key != "surfaceId" && key != "windowId"));
    Some({DV}::Object(entries))
}}""")
exact(WORLD, 'world_brush_mesh_args(&state.surface_id, json!({ "url": url, "digest": digest }))', f'world_brush_mesh_args(&state.surface_id, {MAC}({{ "url": url, "digest": digest }}))', 2)
exact(WORLD, """    let mut page = json!({ "url": run.url, "digest": run.digest, "page": run.page, "pageCount": run.page_count });
    if let Some(object) = page.as_object_mut() {
        if !positions.is_empty() {
            object.insert("positionsB64".into(), json!(base64_codec::base64_standard_encode(positions)));
        }
        if !indices.is_empty() {
            object.insert("indicesB64".into(), json!(base64_codec::base64_standard_encode(indices)));
        }
    }""", f"""    let mut page = {MAC}({{ "url": run.url, "digest": run.digest, "page": run.page, "pageCount": run.page_count }});
    if let {DV}::Object(entries) = &mut page {{
        if !positions.is_empty() {{
            entries.push(("positionsB64".into(), {DV}::String(base64_codec::base64_standard_encode(positions))));
        }}
        if !indices.is_empty() {{
            entries.push(("indicesB64".into(), {DV}::String(base64_codec::base64_standard_encode(indices))));
        }}
    }}""")

exact(WORLD, """        Some((id, granularity)) => json!([{ "granularity": granularity, "id": resolved_item_id(state, id) }]),
        None => json!([]),""", f"""        Some((id, granularity)) => {MAC}([{{ "granularity": granularity, "id": resolved_item_id(state, id) }}]),
        None => {MAC}([]),""")

# 🀄️ wfc fill tools: the args literal becomes a DslValue literal.
for rel in (PL + "🀄️wfc/🗿️artifacts/◻️2d/" + ANY + "✏️editor/🎭️modes/✏️edit/🛠️tools/🔣️fill/🦀️.rs",
            PL + "🀄️wfc/🗿️artifacts/🔲️grid2d/" + ANY + "✏️editor/🎭️modes/✏️edit/🛠️tools/🪣fill/🦀️.rs",
            PL + "🀄️wfc/🗿️artifacts/🧊️3d/" + ANY + "✏️editor/🎭️modes/✏️edit/🛠️tools/📑️fill/🦀️.rs",
            PL + "🀄️wfc/🗿️artifacts/🧱️grid3d/" + ANY + "✏️editor/🎭️modes/✏️edit/🛠️tools/🪣️fill/🦀️.rs"):
    count = text_of(rel).count("semio_framework::optional_json_to_dsl(Some(args))")
    exact(rel, "let args = serde_json::json!(", f"let args = {MAC}(", count or 1)
    exact(rel, "semio_framework::optional_json_to_dsl(Some(args))", "Some(args)", count or 1)

# 🎯️ App wrappers: `Option<DslValue>` in, literal callers rewritten.
WRAPPERS = [
    (PL + "🌀️procedural/🗿️artifacts/🧊️generation3d/" + ANY + "✏️editor/🦀️.rs", "generation3d_action", "pub fn generation3d_action(action: &str, args: Option<Value>) -> ActionDescriptor {\n    ActionDescriptor { controller_id: GENERATION_3D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }"),
    (PL + "🌀️procedural/🗿️artifacts/🧊️generation3d/" + ANY + "👁️viewer/🦀️.rs", "generation3d_view_action", "pub fn generation3d_view_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {\n    ActionDescriptor { controller_id: GENERATION3D_VIEW_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }"),
    (PL + "🌍️gis/🗿️artifacts/🗺️gismap/" + ANY + "✏️editor/🦀️.rs", "gis2d_window_action", "pub fn gis2d_window_action(action: &str, args: Option<Value>) -> ActionDescriptor {\n    ActionDescriptor { controller_id: GIS2D_PLAY_APP_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }"),
    (PL + "💠️lowpoly/🗿️artifacts/💠️lowpoly/" + ANY + "✏️editor/🦀️.rs", "lowpoly_window_action", "pub fn lowpoly_window_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {\n    ActionDescriptor { controller_id: LOWPOLY_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework::optional_json_to_dsl(args) }"),
    (PL + "🗒️note/🗿️artifacts/🗒️note/" + ANY + "✏️editor/🦀️.rs", "note_action", "pub fn note_action(action: &str, args: Option<serde_json::Value>) -> ActionDescriptor {\n    ActionDescriptor { controller_id: NOTE_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }"),
    (PL + "🧩️puzzle/🗿️artifacts/◻️2d/" + ANY + "✏️editor/🦀️.rs", "puzzle2d_action", "pub fn puzzle2d_action(action: &str, args: Option<Value>) -> ActionDescriptor {\n    ActionDescriptor { controller_id: PUZZLE2D_PLAY_CONTROLLER_ID.into(), action: action.into(), args: semio_framework_plugin::optional_json_to_dsl(args) }"),
]
for rel, name, old in WRAPPERS:
    new = re.sub(r"args: Option<(?:serde_json::)?Value>\)", f"args: Option<{DV}>)", old)
    new = re.sub(r"args: semio_framework(?:_plugin)?::optional_json_to_dsl\(args\) \}", "args }", new)
    exact(rel, old, new)
LOWPOLY_SUN = PL + "💠️lowpoly/🗿️artifacts/💠️lowpoly/" + ANY + "✏️editor/🛠️options/🌞️sun/🦀️.rs"
PROCEDURAL_MEASURE_FILES = [PL + "🌀️procedural/🗿️artifacts/🧊️generation3d/" + ANY + rel for rel in (
    "✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs", "✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🦀️.rs",
    "✏️editor/🎭️modes/🧬️generate/🪟️windows/👁️preview/🦀️.rs", "👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🦀️.rs")]
for rel in PROCEDURAL_MEASURE_FILES:
    text = text_of(rel)
    updated = re.sub(r"Fn\(&str, Option<(?:serde_json::)?Value>\) -> ((?:semio_framework_plugin::)?ActionDescriptor)", lambda m: f"Fn(&str, Option<{DV}>) -> {m.group(1)}", text)
    if updated == text and f"Fn(&str, Option<{DV}>)" not in text:
        problems.append(f"{rel}: no measure closure type to retype")
    put(rel, updated)
CALLER_FILES = {}
for rel in [p for p, _, _ in WRAPPERS] + PROCEDURAL_MEASURE_FILES:
    CALLER_FILES.setdefault(rel, set())
import subprocess
for _, name, _ in WRAPPERS:
    listing = subprocess.run(["git", "grep", "-l", "-w", name, "--", "✏️s/*.rs"], cwd="/Users/ueli/Documents/semio", capture_output=True, text=True).stdout.split()
    for rel in listing:
        rewrite_calls(rel, name, 1, "option")

# 🌞️ Measure closures of the world-3d SDK helpers pass `Option<DslValue>` straight through.
exact(PL + "🏭️process/🗿️artifacts/🧊️process3d/" + ANY + "✏️editor/🎭️modes/✏️edit/🪟️windows/🪚️workpiece/☑️options/☀️sun/🦀️.rs",
      "action: action.into(), args: semio_framework::optional_json_to_dsl(args) }", "action: action.into(), args }")
for rel in (PL + "📐️cad/🗿️artifacts/📐️cad/" + ANY + "✏️editor/🎭️modes/✏️edit/☑️options/🌞️sun/🦀️.rs",
            PL + "📐️cad/🗿️artifacts/📐️cad/" + ANY + "✏️editor/🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs"):
    exact(rel, "cad_window_action(action, semio_framework::optional_json_to_dsl(args))", "cad_window_action(action, args)")
for rel in (PL + "🧩️puzzle/🗿️artifacts/🖐️5d/" + ANY + "✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/☀️sun/🦀️.rs",
            PL + "🧩️puzzle/🗿️artifacts/🖐️5d/" + ANY + "✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️3d/☑️options/🎥️projection/🦀️.rs",
            PL + "🧩️puzzle/🗿️artifacts/🧊️3d/" + ANY + "✏️editor/🎭️modes/✏️edit/☑️options/☀️sun/🦀️.rs",
            PL + "🧩️puzzle/🗿️artifacts/🧊️3d/" + ANY + "✏️editor/🎭️modes/✏️edit/☑️options/🎥️projection/🦀️.rs"):
    exact(rel, "args.map(|value| dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&value)))", "args.map(|value| dsl::os_pack::json::from_dsl_value(&value))")

# 📏️ Layout: interaction args as DslValue, menu rows and export redispatch.
LAYOUT = PL + "📏️layout/🗿️artifacts/📏️layout/" + ANY + "✏️editor/🦀️.rs"
exact(LAYOUT, """pub fn layout_select_action_args(ids: &[String], merge: &str) -> Value {
    let targets: Vec<Value> = ids.iter().map(|id| json!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })).collect();
    json!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "targets": serde_json::to_string(&targets).unwrap_or_default(), "merge": merge, "method": "pick" })
}""", f"""pub fn layout_select_action_args(ids: &[String], merge: &str) -> {DV} {{
    let targets: Vec<{DV}> = ids.iter().map(|id| {MAC}({{ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id }})).collect();
    {MAC}({{ "domainId": LAYOUT_INTERACTION_ELEMENTS, "targets": dsl::os_pack::json::to_json_string(&targets), "merge": merge, "method": "pick" }})
}}""")
exact(LAYOUT, """pub fn layout_hover_action_args(id: Option<&str>) -> Value {
    let targets: Vec<Value> = id.map(|id| vec![json!({ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id })]).unwrap_or_default();
    json!({ "domainId": LAYOUT_INTERACTION_ELEMENTS, "channel": "pointer", "targets": serde_json::to_string(&targets).unwrap_or_default() })
}""", f"""pub fn layout_hover_action_args(id: Option<&str>) -> {DV} {{
    let targets: Vec<{DV}> = id.map(|id| vec![{MAC}({{ "granularity": LAYOUT_GRANULARITY_ELEMENT, "id": id }})]).unwrap_or_default();
    {MAC}({{ "domainId": LAYOUT_INTERACTION_ELEMENTS, "channel": "pointer", "targets": dsl::os_pack::json::to_json_string(&targets) }})
}}""")
exact(LAYOUT, "args: semio_framework::optional_json_to_dsl(Some(layout_select_action_args(ids, merge))), delay_ms: 0", "args: Some(layout_select_action_args(ids, merge)), delay_ms: 0")
exact(LAYOUT, "args: semio_framework::optional_json_to_dsl(Some(layout_hover_action_args(id))), delay_ms: 0", "args: Some(layout_hover_action_args(id)), delay_ms: 0")
exact(LAYOUT, "fn layout_context_menu_item(id: &str, label: &str, icon: &str, action: &str, args: Option<Value>, destructive: bool, disabled: bool) -> ContextMenuItemSpec {",
      f"fn layout_context_menu_item(id: &str, label: &str, icon: &str, action: &str, args: Option<{DV}>, destructive: bool, disabled: bool) -> ContextMenuItemSpec {{")
exact(LAYOUT, "        args: semio_framework_plugin::optional_json_to_dsl(args),\n        destructive: destructive.then_some(true),", "        args,\n        destructive: destructive.then_some(true),")
rewrite_calls(LAYOUT, "layout_context_menu_item", 4, "option", 3)
exact(LAYOUT, "            let issue_value = json!({\n", f"            let issue_value = {MAC}({{\n")
SUBMIT = PL + "📏️layout/🗿️artifacts/📏️layout/" + ANY + "✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"
exact(SUBMIT, "Some(serde_json::json!(", f"Some({MAC}(", 4)
exact(SUBMIT, "action: action.into(), args: semio_framework::optional_json_to_dsl(args), delay_ms: 0", "action: action.into(), args, delay_ms: 0")

# 🧩️ Puzzle 2d context-menu rows, puzzle 3d engagement redispatch.
PUZZLE2D = PL + "🧩️puzzle/🗿️artifacts/◻️2d/" + ANY + "✏️editor/🦀️.rs"
exact(PUZZLE2D, "let item = |id: &str, label: &str, icon: &str, action: &str, args: Option<Value>, destructive: bool, disabled: bool| ContextMenuItemSpec {",
      f"let item = |id: &str, label: &str, icon: &str, action: &str, args: Option<{DV}>, destructive: bool, disabled: bool| ContextMenuItemSpec {{")
exact(PUZZLE2D, "        args: semio_framework_plugin::optional_json_to_dsl(args),\n        destructive: destructive.then_some(true),", "        args,\n        destructive: destructive.then_some(true),")
rewrite_calls(PUZZLE2D, "item", 4, "option")
P3 = PL + "🧩️puzzle/🗿️artifacts/🧊️3d/" + ANY + "✏️editor/🎮️commands/"
exact(P3 + "📨️engagement-submit/🦀️.rs", "let start = serde_json::json!(", f"let start = {MAC}(")
exact(P3 + "📨️engagement-submit/🦀️.rs", "args: semio_framework::optional_json_to_dsl(Some(start))", "args: Some(start)")
exact(P3 + "🛑️engagement-abort/🦀️.rs", "let args = serde_json::json!(", f"let args = {MAC}(")
exact(P3 + "🛑️engagement-abort/🦀️.rs", "args: semio_framework::optional_json_to_dsl(Some(args))", "args: Some(args)")

# 📺️ Renderer: literal-arg builders take DslValue; JSON-sourced args convert where they enter the descriptor.
SCENES = RE + "🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"
exact(SCENES, "pub(crate) fn scene_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {\n    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.into(), args: semio_framework::optional_json_to_dsl(Some(args)) }",
      f"pub(crate) fn scene_action(scene: &UiComponentSceneNode, action: &str, args: {DV}) -> ActionDescriptor {{\n    ActionDescriptor {{ controller_id: scene.controller_id.clone(), action: action.into(), args: Some(args) }}")
exact(SCENES, "fn block_list_action(scene: &UiComponentSceneNode, action: &str, args: Value) -> ActionDescriptor {\n    ActionDescriptor { controller_id: scene.controller_id.clone(), action: action.to_string(), args: semio_framework::optional_json_to_dsl(Some(args)) }",
      f"fn block_list_action(scene: &UiComponentSceneNode, action: &str, args: {DV}) -> ActionDescriptor {{\n    ActionDescriptor {{ controller_id: scene.controller_id.clone(), action: action.to_string(), args: Some(args) }}")
exact(SCENES, """fn canvas_addressed_action(controller_id: &str, surface_id: &str, action: &str, fields: Value) -> ActionDescriptor {
    let mut args = serde_json::Map::new();
    args.insert("surfaceId".into(), Value::String(surface_id.to_string()));
    if let Value::Object(fields) = fields {
        args.extend(fields);
    }
    ActionDescriptor { controller_id: controller_id.to_string(), action: action.to_string(), args: semio_framework::optional_json_to_dsl(Some(Value::Object(args))) }""",
      f"""fn canvas_addressed_action(controller_id: &str, surface_id: &str, action: &str, fields: {DV}) -> ActionDescriptor {{
    let mut args = vec![("surfaceId".to_string(), {DV}::String(surface_id.to_string()))];
    if let {DV}::Object(fields) = fields {{
        args.extend(fields.into_iter().filter(|(key, _)| key != "surfaceId"));
    }}
    ActionDescriptor {{ controller_id: controller_id.to_string(), action: action.to_string(), args: Some({DV}::Object(args)) }}""")
exact(SCENES, '_ => scene_action(scene, "selectRow", json!({ "surfaceId": scene.surface_id, "row": row })),', f'_ => scene_action(scene, "selectRow", {MAC}({{ "surfaceId": scene.surface_id, "row": {DV}::from(row) }})),')
SCENE_TESTS = RE + "🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs"
for rel in (SCENES, SCENE_TESTS):
    for name, index in (("scene_action", 2), ("block_list_action", 2), ("canvas_addressed_action", 3)):
        if f"{name}(" in text_of(rel):
            rewrite_calls(rel, name, index, "plain", 0, rel == SCENE_TESTS)
BOUNDARY = [
    (SCENES, "semio_framework::optional_json_to_dsl(Some(Value::Object(args)))", f"Some({DV}::from(Value::Object(args)))", 1),
    (RE + "⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(args))", f"Some({DV}::from(args))", 2),
    (RE + "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(Value::Object(args)))", f"Some({DV}::from(Value::Object(args)))", 2),
    (RE + "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(if effective.is_empty() { None } else { Some(Value::Object(effective)) })", f"(!effective.is_empty()).then(|| {DV}::from(Value::Object(effective)))", 1),
    (RE + "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(args))", f"Some({DV}::from(args))", 3),
    (RE + "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(args)", f"args.map({DV}::from)", 1),
    (RE + "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(frame_args))", f"Some({DV}::from(frame_args))", 1),
    (RE + "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(done_args))", f"Some({DV}::from(done_args))", 1),
    (RE + "🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(Value::Object(base)))", f"Some({DV}::from(Value::Object(base)))", 1),
    (RE + "🐚️Shell/🧪️tests/⚙️settings-general-layout/🦀️.rs", 'semio_framework::optional_json_to_dsl(Some(vector["arguments"].clone()))', f'Some({DV}::from(&vector["arguments"]))', 1),
    (RE + "🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs", 'semio_framework::optional_json_to_dsl(Some(step["arguments"].clone()))', f'Some({DV}::from(&step["arguments"]))', 1),
    (RE + "🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(arguments))", f"Some({DV}::from(arguments))", 1),
    (RE + "🐚️Shell/🧪️tests/🔀️wgpu-document-relay/🦀️.rs", "semio_framework::optional_json_to_dsl(args)", f"args.map({DV}::from)", 1),
    (RE + "🐚️Shell/🧪️tests/🔬️wgpu-command-registry/🦀️.rs", 'semio_framework::optional_json_to_dsl((!case["args"].is_null()).then(|| case["args"].clone()))', f'(!case["args"].is_null()).then(|| {DV}::from(&case["args"]))', 1),
    (RE + "🐚️Shell/🧪️tests/🚗️driver-editor/🦀️.rs", "semio_framework::optional_json_to_dsl(Some(args))", f"Some({DV}::from(args))", 1),
    (RE + "🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs", 'semio_framework::optional_json_to_dsl(Some(item["request"].clone())).unwrap()', f'{DV}::from(&item["request"])', 1),
    (RE + "🖼️IconRenderHost/🧪️tests/📤️export-batch/🦀️.rs", 'semio_framework::optional_json_to_dsl(Some(fixture["invalid"][0]["request"].clone())).unwrap()', f'{DV}::from(&fixture["invalid"][0]["request"])', 1),
    (RE + "🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs", "semio_framework::optional_json_to_dsl(args)", f"args.map({DV}::from)", 1),
    (RE + "🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs", "semio_framework::optional_json_to_dsl(args)", f"args.map({DV}::from)", 1),
]
for rel, old, new, count in BOUNDARY:
    exact(rel, old, new, count)

for path, text in edits.items():
    if not re.search(r"(?<![A-Za-z0-9_])semio_framework::", path.read_text(encoding="utf-8")):
        edits[path] = text.replace(MAC, "semio_framework_plugin::dsl_value!").replace(DV, "semio_framework_plugin::DslValue")
for path in sorted(edits, key=str):
    print(("write " if WRITE else "dry-run ") + str(path.relative_to(ROOT)))
for note in notes:
    print(note)
for problem in problems:
    print("problem:", problem)
print(f"{len(edits)} files, {len(problems)} problems")
if "--diff" in args:
    import difflib
    for path, text in sorted(edits.items(), key=lambda item: str(item[0])):
        rel = str(path.relative_to(ROOT))
        sys.stdout.writelines(difflib.unified_diff(path.read_text(encoding="utf-8").splitlines(True), text.splitlines(True), rel, rel, n=1))
if problems:
    sys.exit(1)
if WRITE:
    for path, text in edits.items():
        path.write_text(text, encoding="utf-8")
