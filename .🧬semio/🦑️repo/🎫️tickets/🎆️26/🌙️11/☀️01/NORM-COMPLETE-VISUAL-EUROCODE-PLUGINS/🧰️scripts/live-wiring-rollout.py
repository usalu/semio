import os, re, sys

ROOT = "/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
ARTIFACTS = {
    "en1990": "⚖️", "en1991": "🏋️", "en1992": "🏛️", "en1993": "🔩️", "en1994": "🧩️",
    "en1995": "🪵️", "en1996": "🪨️", "en1997": "🌍️", "en1998": "🫨️", "en1999": "🪶️",
}
EDITS = {
    "en1990": ("consequenceClass", "2"), "en1991": ("altitude", "321.0"), "en1992": ("title", '"wave3"'),
    "en1993": ("annex", '"de"'), "en1994": ("structureKind", '"wave3"'), "en1995": ("annex", '"de"'),
    "en1996": ("storeys", "7"), "en1997": ("designApproach", '"wave3"'), "en1998": ("annex", '"de"'),
    "en1999": ("annex", '"de"'),
}
SUBSET = "🏅️standards/🔖️1/🪆️subsets/✳️any"


def path(code, *parts):
    return os.path.join(ROOT, ARTIFACTS[code] + code, SUBSET, *parts)


def read(p):
    with open(p, encoding="utf-8") as handle:
        return handle.read()


def write(p, text):
    with open(p, "w", encoding="utf-8") as handle:
        handle.write(text)


def must_replace(text, old, new, where, count=1):
    if old not in text:
        raise SystemExit(f"pattern missing in {where}: {old[:90]!r}")
    return text.replace(old, new, count)


def edit_root(code):
    N = "En" + code[2:]
    p = path(code, "✏️editor", "🦀️.rs")
    s = read(p)
    if "fn render_body(" in s:
        print("skip root", code)
        return
    s = must_replace(s, "windows::{checks, diagrams, properties, viewport};", "windows::{checks, diagrams, outliner, properties, viewport};", p) if "outliner" not in s.split("windows::{")[1].split("}")[0] else s
    a = s.index("    fn render(body_key: &str")
    b = s.index(".map(semio_framework_plugin::built_to_component_tree)", a)
    end = s.index("    }\n", b) + len("    }\n")
    original = s[a:end]
    lines = original.rstrip("\n").split("\n")
    match_at = next(i for i, line in enumerate(lines) if line.strip() == "match body_key {")
    last = max(i for i, line in enumerate(lines) if line == "    }")
    prologue = lines[1:match_at]
    tail = [line[4:] if line.startswith("    ") else line for line in lines[match_at:last]]
    live = [
        f"    let live = semio_s_plugin_norm_app_surface::LiveView::<{N}Family>::new(doc.snapshot.clone(), semio_s_plugin_norm_results_config::current::<checks::ChecksWindowConfigOwner>(cfg).selected_check_index, picked);",
        "    let host = &live.host;",
        "    let selection = &live.selection;",
        "    let selected = selection.checks_selected();",
        "    let focus = live.focus.clone();",
        "    let worst = live.worst.as_deref();",
    ]
    if any("let selected_check =" in line for line in prologue):
        live.append("    let selected_check = selection.check.and_then(|index| host.report().checks.get(index));")
    live += [line[4:] for line in prologue if "let combination" in line]
    body = "\n".join(tail)
    body = must_replace(body, "properties::render(&host, focus.as_deref(), ", "properties::render(host, selection, ", p)
    body = body.replace("outliner::render(&host, focus.as_deref(), ", "outliner::render(host, selection, ")
    body = must_replace(body, "viewport::render(doc.snapshot, host.report(), focus.as_deref(), ", "viewport::render(doc.snapshot, host.report(), selection, ", p)
    outliner_arm = "        outliner::BODY_OUTLINER => outliner::render(host, selection, view_state.locale, CONTROLLER_ID, &semio_framework_plugin::TreeWindows::for_body(view_state, outliner::BODY_OUTLINER)),\n"
    body = body.replace("Self::examples()", f"{N}PlayApp::examples()")
    if "outliner::BODY_OUTLINER" not in body:
        body = must_replace(body, "        properties::BODY_PROPERTIES =>", outliner_arm + "        properties::BODY_PROPERTIES =>", p)
    rendered = (
        "fn render_body(body_key: &str, doc: &ArtifactView<'_, " + N + "Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel, picked: &[String]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {\n"
        + "\n".join(live) + "\n" + body.rstrip() + "\n}\n"
    )
    trait_fns = (
        "    fn render(body_key: &str, doc: &ArtifactView<'_, " + N + "Snapshot>, cfg: &ConfigView<'_, NoConfig>, view_state: &semio_framework_plugin::ViewModel) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::ComponentTree> {\n"
        "        render_body(body_key, doc, cfg, view_state, &[])\n"
        "    }\n\n"
        "    fn render_with_request_context(\n"
        "        _owner: &semio_framework_plugin::ArtifactInstanceOperationOwnerHandle,\n"
        "        body_key: &str,\n"
        "        doc: &ArtifactView<'_, " + N + "Snapshot>,\n"
        "        cfg: &ConfigView<'_, NoConfig>,\n"
        "        view_state: &semio_framework_plugin::ViewModel,\n"
        "        _transient: &semio_framework_plugin::TransientView<'_, semio_framework_plugin::NoTransient>,\n"
        "        interaction: &InteractionView<'_>,\n"
        "        _cx: &mut semio_framework_plugin::StepContext<'_>,\n"
        "    ) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {\n"
        "        render_body(body_key, doc, cfg, view_state, &interaction.selection(&semio_s_plugin_norm_app_surface::interaction_domain_id(VARIANT)).ids).map(Some).map_err(Into::into)\n"
        "    }\n"
    )
    s = s[:a] + trait_fns + s[end:]
    marker = "//#region 🧵️RetainedCommands"
    s = must_replace(s, marker, "//#region 🪄️LiveRender\n" + rendered + "//#endregion 🪄️LiveRender\n\n" + marker, p)
    s = must_replace(s, "            .window_kind_def(properties::definition())", "            .window_kind_def(outliner::definition())\n            .window_kind_def(properties::definition())", p) if "outliner::definition()" not in s else s
    ev = s.index(f"    fn evaluate(document: &{N}Snapshot)")
    close = s.index("    }\n", ev) + len("    }\n")
    if "fn live_report(" not in s:
        s = s[:close] + f"\n    fn live_report(document: &{N}Snapshot) -> semio_s_plugin_norm_compliance::CheckReport {{\n        semio_s_artifact_norm_{code}_inferences::live_report(document)\n    }}\n" + s[close:]
    s = s.replace(f"NormHost::<{N}Family>::from_artifact(", f"NormHost::<{N}Family>::live(")
    s = s.replace("crate::incremental::host_for(", "Host::live(")
    write(p, s)


def edit_mode(code):
    p = path(code, "✏️editor", "🎭️modes", "✏️edit", "🦀️.rs")
    s = read(p)
    if "outliner" in s.replace("properties and outliner left", ""):
        print("skip mode", code)
        return
    s = must_replace(s, "windows::{checks, diagrams, properties, viewport};", "windows::{checks, diagrams, outliner, properties, viewport};", p)
    s = must_replace(s, '                stack(properties::WINDOW_PROPERTIES, "Properties", 0.22),', '                WindowLayoutChild::Axis(WindowLayoutAxisNode { kind: "column".into(), size: Some(0.22), children: vec![stack(outliner::WINDOW_OUTLINER, "Outliner", 0.5), stack(properties::WINDOW_PROPERTIES, "Properties", 0.5)] }),', p)
    s = must_replace(s, "    #[path = \"🪟️windows/🧾️properties/🦀️.rs\"]\n    pub mod properties;", "    #[path = \"🪟️windows/🌲️outliner/🦀️.rs\"]\n    pub mod outliner;\n    #[path = \"🪟️windows/🧾️properties/🦀️.rs\"]\n    pub mod properties;", p)
    s = s.replace("properties: properties", "properties: properties")
    s = s.replace("/// properties 22 %,", "/// outliner above properties 22 %,")
    write(p, s)
    t = path(code, "✏️editor", "🎭️modes", "✏️edit", "🧪️tests", "🔬️unit", "🦀️.rs")
    ts = read(t)
    ts = must_replace(ts, "windows::{checks, diagrams, properties, viewport};", "windows::{checks, diagrams, outliner, properties, viewport};", t)
    ts = must_replace(ts, "all_four_window_kinds", "all_five_window_kinds", t)
    ts = must_replace(ts, "[properties::WINDOW_PROPERTIES,", "[outliner::WINDOW_OUTLINER, properties::WINDOW_PROPERTIES,", t)
    write(t, ts)


def edit_properties(code):
    p = path(code, "✏️editor", "🎭️modes", "✏️edit", "🪟️windows", "🧾️properties", "🦀️.rs")
    s = read(p)
    if "NormSelection" in s:
        print("skip properties", code)
        return
    s = must_replace(s, "{render_inspector, InspectorOptions, OutlinerState}", "{render_inspector, InspectorOptions, NormSelection}", p)
    s = must_replace(s, "selected_entity: Option<&str>", "selection: &NormSelection", p)
    s = must_replace(s, "OutlinerState { selected: selected_entity.map(str::to_string).into_iter().collect(), ..OutlinerState::default() }", "selection.outliner_state()", p)
    s = s.replace("`selected_entity` opens its rows.", "the selection opens its rows and marks the field paths of the selected check.")
    write(p, s)


def edit_viewport(code):
    p = path(code, "✏️editor", "🎭️modes", "✏️edit", "🪟️windows", "🧊️viewport", "🦀️.rs")
    s = read(p)
    if "NormSelection" in s:
        print("skip viewport", code)
        return
    s = must_replace(s, "selected_entity: Option<&str>", "selection: &NormSelection", p)
    old = re.search(r"    let selected: Vec<String> = scene\.entities\.iter\(\)\.filter\(\|e\| selected_entity.*?\.collect\(\);\n", s, re.S)
    if not old:
        raise SystemExit(f"selected line missing in {p}")
    s = s.replace(old.group(0), "    let selected = selection.highlights(scene.entities.iter().map(|e| (e.entity.kind, e.entity.id.as_str())));\n")
    s = must_replace(s, "use semio_s_plugin_norm_app_surface::viewport3d::picking::entity_address;\n", "", p)
    s = must_replace(s, "use semio_s_plugin_norm_app_surface::interaction_domain_id;", "use semio_s_plugin_norm_app_surface::{interaction_domain_id, NormSelection};", p)
    s = s.replace("`selected_entity` is the document entity whose scene entities are highlighted", "`selection` is the resolved selection whose scene entities are highlighted")
    write(p, s)


def field_meta_expression(code):
    s = read(path(code, "✏️editor", "🎭️modes", "✏️edit", "🪟️windows", "🧾️properties", "🦀️.rs"))
    m = re.search(r"Some\(([\w:]*field_meta[\w:]*)\)", s)
    if not m:
        raise SystemExit(f"field meta missing in properties of {code}")
    return m.group(1)


def create_outliner(code, template_code="en1993"):
    target_dir = path(code, "✏️editor", "🎭️modes", "✏️edit", "🪟️windows", "🌲️outliner")
    if os.path.exists(os.path.join(target_dir, "🦀️.rs")):
        print("skip outliner", code)
        return
    N, TN = "En" + code[2:], "En" + template_code[2:]
    template = read(path(template_code, "✏️editor", "🎭️modes", "✏️edit", "🪟️windows", "🌲️outliner", "🦀️.rs"))
    text = template.replace(template_code, code).replace(TN, N).replace("EN " + template_code[2:], "EN " + code[2:])
    text = text.replace(f"crate::field_meta::{code}_field_meta", field_meta_expression(code))
    os.makedirs(os.path.join(target_dir, "🧪️tests", "🔬️unit"), exist_ok=True)
    write(os.path.join(target_dir, "🦀️.rs"), text)
    test = f'''use crate::modes::edit::windows::outliner::*;
use crate::unit_tests::context;
use semio_framework_value::DslValue;

#[semio_framework_async_macros::async_test]
async fn definition_declares_this_windows_body_key() {{
    assert_eq!(definition().body_key, BODY_OUTLINER);
    assert_eq!(definition().id, WINDOW_OUTLINER);
}}

#[semio_framework_async_macros::async_test]
async fn lists_the_first_entity_of_the_document_with_a_select_row() {{
    let mut app = context::app_with_registry().await;
    let tree = semio_framework_value::ToValue::to_value(&app.snapshot().expect("projection"));
    let id = semio_s_artifact_norm_{code}_scene::ENTITY_KINDS
        .iter()
        .find_map(|kind| match tree.get(kind.collection) {{
            Some(DslValue::Array(items)) => items.first().and_then(|item| match item.get("id") {{
                Some(DslValue::String(id)) => Some(id.clone()),
                _ => None,
            }}),
            _ => None,
        }})
        .expect("the reference document has an entity");
    let rendered = context::render(&mut app, BODY_OUTLINER).await;
    assert!(rendered.contains(&id) && rendered.contains("select"), "outliner lists {{id}}: {{rendered}}");
    context::close(&mut app);
}}
'''
    write(os.path.join(target_dir, "🧪️tests", "🔬️unit", "🦀️.rs"), test)


def edit_root_tests(code):
    N = "En" + code[2:]
    p = path(code, "✏️editor", "🧪️tests", "🔬️unit", "🦀️.rs")
    s = read(p)
    s = s.replace(f"NormHost::<{N}Family>::from_artifact(", f"NormHost::<{N}Family>::live(")
    s = s.replace("Host::from_artifact(", "Host::live(")
    s = s.replace("crate::incremental::host_for(", "crate::Host::live(")
    s = s.replace("assert_eq!(definition.window_kinds.len(), 2);", "assert_eq!(definition.window_kinds.len(), 5);")
    s = re.sub(r"for body_key in \[properties::BODY_PROPERTIES, viewport", "for body_key in [outliner::BODY_OUTLINER, properties::BODY_PROPERTIES, viewport", s)
    if "render_right_after_an_edit_shows_updated_checks_without_evaluate" not in s:
        field, value = EDITS[code]
        value_literal = value.replace('"', '\\"')
        test = f'''
//#region 🛰️LiveWiring
#[semio_framework_async_macros::async_test]
async fn render_right_after_an_edit_shows_updated_checks_without_evaluate() {{
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, {N}Command::SetField(set_field::SetField {{ path: "{field}".into(), value_json: "{value_literal}".into() }})).await;
    let edited = app.snapshot().expect("projection");
    let expected = semio_s_artifact_norm_{code}_inferences::evaluate(&edited);
    assert_eq!(<{N}Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&edited), expected, "the retained session equals the full fold");
    let worst = expected.checks.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).expect("the edited document still has checks");
    let rendered = context::render(&mut app, checks::BODY_CHECKS).await;
    assert!(!rendered.contains("No checks computed"), "no evaluate command was dispatched: {{rendered}}");
    let escaped = worst.title.en.replace('\\\\', "\\\\\\\\").replace('"', "\\\\\\"");
    assert!(rendered.contains(&escaped), "the checks window shows the edited report: {{rendered}}");
    context::close(&mut app);
}}

#[test]
fn a_picked_entity_selects_its_governing_check_in_every_window() {{
    let report = semio_s_artifact_norm_{code}_inferences::evaluate(&{N}Snapshot::default());
    let entity = report.checks.iter().find(|check| !check.subject.entity_id.is_empty() && check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable).map(|check| check.subject.entity_id.clone()).expect("a check with an entity");
    let live = semio_s_plugin_norm_app_surface::LiveView::<{N}Family>::new({N}Snapshot::default(), None, &[entity.clone()]);
    assert!(live.selection.contains(&entity));
    assert_eq!(live.focus.as_deref(), Some(entity.as_str()));
    assert!(live.selection.checks_selected().is_some());
    assert!(live.selection.outliner_state().selected.contains(&entity));
}}
//#endregion 🛰️LiveWiring
'''
        marker = "semio_framework_plugin::history_edit_acceptance_law!"
        if marker in s:
            i = s.index(marker)
            s = s[:i] + test.lstrip("\n") + "\n" + s[i:]
        else:
            s = s.rstrip("\n") + "\n" + test
    write(p, s)


def main(codes):
    for code in codes:
        edit_root(code)
        edit_mode(code)
        edit_properties(code)
        edit_viewport(code)
        create_outliner(code)
        edit_root_tests(code)
        print("done", code)


if __name__ == "__main__":
    main(sys.argv[1:] or list(ARTIFACTS))
