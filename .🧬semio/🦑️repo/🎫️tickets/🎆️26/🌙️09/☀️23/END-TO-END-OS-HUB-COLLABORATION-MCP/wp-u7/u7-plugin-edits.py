"""🧩️ U7 non-stdio example-loader edits: `(file, old block, new block)` per `setActiveExample` reduction. Imported by
`u7-build-set.py`; every `old` is the exact live text (doc + fn) the census printed, every `new` answers a non-empty id
that names no declared example, or a declared example whose asset does not decode, with the SDK's typed
`ExampleRefusal` instead of an empty emit or the genesis document. The empty id keeps each app's own declared meaning.
"""

P = "✏️s/🔌️plugins/"
S1 = "/🏅️standards/🔖️1/🪆️subsets/✳️any/"
R = "semio_framework_plugin::ExampleRefusal"

EDITS = []


def edit(part, rel, old, new):
    EDITS.append({"part": part, "rel": rel, "old": old, "new": new})


edit("vcs", P + "🌿️vcs/🗿️artifacts/🌿️vcs" + S1 + "✏️editor/🎮️commands/📚️example/🦀️.rs",
"""    /// 🧬️ Whole-document replace has no `VcsDemoMutation` representative, so picking an example
    /// builds an `Effect::LoadDocument` and therefore lands outside undo history. An id this app
    /// does not publish is a no-op rather than a fault: the playground navbar dispatches whatever
    /// its combobox holds, including an empty string before the catalogue resolves.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {
        if payload.example_id.is_empty() || payload.example_id == crate::examples::demo::ID {
            return Ok(Emit { effects: vec![vcs_example_document_effect()], ..Default::default() });
        }
        Ok(Emit::default())
    }""",
f"""    /// 🧬️ Whole-document replace has no `VcsDemoMutation` representative, so picking an example
    /// builds an `Effect::LoadDocument` and therefore lands outside undo history. The empty id is the
    /// demo too; an id this app does not publish is the typed `example.unknown` refusal the shell shows.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, VcsSnapshot>, _cfg: &ConfigView<'_, VcsDemoConfig>) -> Result<Emit<VcsDemoMutation, VcsDemoConfigMutation>, Fault> {{
        if payload.example_id.is_empty() || payload.example_id == crate::examples::demo::ID {{
            return Ok(Emit {{ effects: vec![vcs_example_document_effect()], ..Default::default() }});
        }}
        Err({R}::unknown(&payload.example_id).into())
    }}""")

edit("animate", P + "🎞️animate/🗿️artifacts/🎬️presentation" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
"""    if payload.example_id == "demo" || payload.example_id.is_empty() {
        Ok(Emit { effects: vec![crate::editor::animate::reset_presentation_document_effect(&demo_presentation_snapshot()), interaction_select_effect(&[], "replace")], ..Default::default() })
    } else {
        Ok(Emit::default())
    }""",
f"""    if payload.example_id == "demo" || payload.example_id.is_empty() {{
        Ok(Emit {{ effects: vec![crate::editor::animate::reset_presentation_document_effect(&demo_presentation_snapshot()), interaction_select_effect(&[], "replace")], ..Default::default() }})
    }} else {{
        Err({R}::unknown(&payload.example_id).into())
    }}""")

edit("architect", P + "🏛️architect/🗿️artifacts/🏛️program" + S1 + "✏️editor/🎮️commands/📚️example/🦀️.rs",
"""    /// 🧬️ Whole-document replace has no `ProgramMutation` representative (banned outright by the
    /// taxonomy's forbidden vocabulary), so picking an example builds a `Effect::LoadDocument`
    /// through `reset_document_effect` — the same lane `importProgram`/`importRegistersCsv` use —
    /// and therefore lands outside undo history. An id this app does not publish is a no-op rather
    /// than a fault: the playground navbar dispatches whatever its combobox holds.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {
        if payload.example_id.is_empty() || payload.example_id == crate::examples::demo::ID {
            return Ok(Emit { effects: vec![reset_document_effect(&sample_plugin())], ..Default::default() });
        }
        Ok(Emit::default())
    }""",
f"""    /// 🧬️ Whole-document replace has no `ProgramMutation` representative (banned outright by the
    /// taxonomy's forbidden vocabulary), so picking an example builds a `Effect::LoadDocument`
    /// through `reset_document_effect` — the same lane `importProgram`/`importRegistersCsv` use —
    /// and therefore lands outside undo history. The empty id is the sample too; an id this app does
    /// not publish is the typed `example.unknown` refusal the shell shows.
    pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, ProgramSnapshot>, _cfg: &ConfigView<'_, ArchitectConfig>) -> Result<Emit<ProgramMutation, ArchitectConfigMutation>, Fault> {{
        if payload.example_id.is_empty() || payload.example_id == crate::examples::demo::ID {{
            return Ok(Emit {{ effects: vec![reset_document_effect(&sample_plugin())], ..Default::default() }});
        }}
        Err({R}::unknown(&payload.example_id).into())
    }}""")

WRITER = P + "✒️writer/🗿️artifacts/✒️writer" + S1 + "✏️editor/🎮️commands/🧺️set-active-example/🦀️.rs"
edit("writer", WRITER,
"""/// 📚️ `crate::examples::demo::ID` is the ONE example id this subset publishes
/// (`📚️examples/🎬️demo`, registered by `subsets::any::examples()`), and the playground navbar boots the
/// pane by dispatching exactly that id — an empty id means the same "load my default example".
/// `dag.jack` is the second authored asset beside it. An id this app does not publish resets to the
/// empty document rather than faulting: the navbar dispatches whatever its combobox holds.
pub fn document_for_example_id(example_id: &str) -> WriterSnapshot {
    match example_id {
        "" => jack_example_document(),
        id if id == crate::examples::demo::ID => jack_example_document(),
        "dag.jack" => dag_jack_example_document(),
        _ => empty_writer_snapshot(),
    }
}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {
    Ok(Emit { effects: vec![reset_document_effect(&document_for_example_id(&payload.example_id))], ..Default::default() })
}""",
f"""/// 📚️ `crate::examples::demo::ID` is the ONE example id this subset publishes
/// (`📚️examples/🎬️demo`, registered by `subsets::any::examples()`), and the playground navbar boots the
/// pane by dispatching exactly that id — an empty id means the same "load my default example".
/// `dag.jack` is the second authored asset beside it. An id this app does not publish is the typed
/// `example.unknown` refusal the shell shows, never an empty document.
pub fn document_for_example_id(example_id: &str) -> Result<WriterSnapshot, Fault> {{
    match example_id {{
        "" => Ok(jack_example_document()),
        id if id == crate::examples::demo::ID => Ok(jack_example_document()),
        "dag.jack" => Ok(dag_jack_example_document()),
        other => Err({R}::unknown(other).into()),
    }}
}}

pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, WriterSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<WriterMutation, NoConfigMutation>, Fault> {{
    Ok(Emit {{ effects: vec![reset_document_effect(&document_for_example_id(&payload.example_id)?)], ..Default::default() }})
}}""")
edit("writer", WRITER, "\nuse crate::schema::empty_writer_snapshot;\n", "\n")
edit("writer", P + "✒️writer/🗿️artifacts/✒️writer" + S1 + "✏️editor/🦀️.rs",
"""                emit.effects.push(reset_document_effect_now(&set_active_example::document_for_example_id(&payload.example_id)));""",
"""                emit.effects.push(reset_document_effect_now(&set_active_example::document_for_example_id(&payload.example_id)?));""")

for plugin, crate_path, label in (("mathematical", "➗️mathematical/🗿️artifacts/➗️equation" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs", "equation"), ("sequence", "🎬️sequence/🗿️artifacts/🎬️sequence" + S1 + "✏️editor/🎮️commands/📚️example/🦀️.rs", "sequence")):
    indent = "" if plugin == "mathematical" else "    "
    old_doc = {
        "mathematical": "/// publish is an empty emit rather than a fault: the playground navbar dispatches whatever its\n/// combobox holds, including ids belonging to other apps.\n",
        "sequence": "    /// An id this app does not publish is an empty emit rather than a fault: the playground navbar\n    /// dispatches whatever its combobox holds, including ids belonging to other apps.\n",
    }[plugin]
    new_doc = {
        "mathematical": "/// publish is the typed `example.unknown` refusal the shell shows, and an asset that no longer parses\n/// `example.undecodable` — never an empty emit.\n",
        "sequence": "    /// An id this app does not publish is the typed `example.unknown` refusal the shell shows, and an\n    /// asset that no longer parses `example.undecodable` — never an empty emit.\n",
    }[plugin]
    edit(plugin, P + crate_path, old_doc, new_doc)
    edit(plugin, P + crate_path,
f"""{indent}    if !example_id.is_empty() && example_id != crate::examples::demo::ID {{
{indent}        return Ok(Emit::default());
{indent}    }}
{indent}    let document = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT)
{indent}        .map_err(|error| Fault::from(format!("{label} example {{}} is not parsable: {{error:?}}", crate::examples::demo::ID)))?;""",
f"""{indent}    if !example_id.is_empty() && example_id != crate::examples::demo::ID {{
{indent}        return Err({R}::unknown(example_id).into());
{indent}    }}
{indent}    let document = crate::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| {R}::undecodable(crate::examples::demo::ID, format!("{{error:?}}")))?;""")

edit("wfc", P + "🀄️wfc/🗿️artifacts/🖼️bitmap" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
"""    let Some(next) = example_snapshot(example_id) else { return Ok(Emit::default()) };""",
f"""    let next = example_snapshot(example_id).ok_or_else(|| {R}::unknown(example_id))?;""")
edit("wfc", P + "🀄️wfc/🗿️artifacts/◻️2d" + S1 + "✏️editor/🦀️.rs",
"""                return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc2d.example.unknown"), format!("wfc 2d has no example '{example_id}'")));""",
f"""                return Err({R}::unknown(example_id).into());""")
edit("wfc", P + "🀄️wfc/🗿️artifacts/🔲️grid2d" + S1 + "✏️editor/🦀️.rs",
"""            let next = example_document(example_id).ok_or_else(|| Fault::from(format!("wfc-grid2d-unknown-example:{example_id}")))?;""",
f"""            let next = example_document(example_id).ok_or_else(|| {R}::unknown(example_id))?;""")
edit("wfc", P + "🀄️wfc/🗿️artifacts/🔲️grid2d" + S1 + "👁️viewer/🦀️.rs",
"""                return Err(Fault::from(format!("wfc-grid2d-view-unknown-example:{example_id}")));""",
f"""                return Err({R}::unknown(example_id).into());""")
edit("wfc", P + "🀄️wfc/🗿️artifacts/🧱️grid3d" + S1 + "✏️editor/🦀️.rs",
"""                return Err(Fault::from(format!("wfc.grid3d.example.unknown '{example_id}'")));""",
f"""                return Err({R}::unknown(example_id).into());""")
edit("wfc", P + "🀄️wfc/🗿️artifacts/🧊️3d" + S1 + "✏️editor/🦀️.rs",
"""        other => Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("wfc3d.example.unknown"), format!("wfc3d has no example '{other}'"))),""",
f"""        other => Err({R}::unknown(other).into()),""")

GEN2D = P + "🌀️procedural/🗿️artifacts/🌀️generation2d" + S1 + "✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
edit("procedural", GEN2D,
"""fn example_document(example_id: &str) -> Option<Generation2dSnapshot> {
    if example_id.is_empty() {
        return Some(empty_generation2d_snapshot());
    }
    if example_id == crate::examples::demo::ID {
        return crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).ok();
    }
    None
}""",
f"""fn example_document(example_id: &str) -> Result<Generation2dSnapshot, Fault> {{
    if example_id.is_empty() {{
        return Ok(empty_generation2d_snapshot());
    }}
    if example_id == crate::examples::demo::ID {{
        return crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| {R}::undecodable(example_id, format!("{{error:?}}")).into());
    }}
    Err({R}::unknown(example_id).into())
}}""")
edit("procedural", GEN2D,
"""    let Some(target) = example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };""",
"""    let target = example_document(&payload.example_id)?;""")

GEN3D = P + "🌀️procedural/🗿️artifacts/🧊️generation3d" + S1 + "✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
edit("procedural", GEN3D,
"""/// screen). An id the dialect never published is still a no-op rather than a blank, because the
/// picker can only ever offer declared ids and a typo must not destroy a graph
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).""",
"""/// screen). An id the dialect never published is the typed `example.unknown` refusal, and a bundled
/// example whose asset no longer decodes `example.undecodable` — a typo must not destroy a graph, and
/// neither may pass in silence (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).""")
edit("procedural", GEN3D,
"""    } else if is_generation3d_example_id(&payload.example_id) {
        example_snapshot(&payload.example_id).unwrap_or_default()
    } else {
        return Ok(Emit::default());
    };""",
f"""    }} else if is_generation3d_example_id(&payload.example_id) {{
        example_snapshot(&payload.example_id).ok_or_else(|| {R}::undecodable(&payload.example_id, "the bundled example asset does not decode"))?
    }} else {{
        return Err({R}::unknown(&payload.example_id).into());
    }};""")
edit("procedural", P + "🌀️procedural/🗿️artifacts/🧊️generation3d" + S1 + "👁️viewer/🎮️commands/🎨️set-active-example/🦀️.rs",
"""        return Err(Fault::from("generation3d-view-active-example-unknown"));""",
f"""        return Err({R}::unknown(&payload.example_id).into());""")

edit("flow", P + "🌊️flow/🗿️artifacts/🌊️flow" + S1 + "✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs",
"""        <FlowSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).map_err(|error| Fault::from(error.to_string()))?
    } else {
        return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("flow.example-unknown"), format!("setActiveExample has no example \\"{}\\"", payload.example_id)));
    };""",
f"""        <FlowSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).map_err(|error| {R}::undecodable(demo::ID, error.to_string()))?
    }} else {{
        return Err({R}::unknown(&payload.example_id).into());
    }};""")

for gis, kind in (("🏔️gisterrain", "terrain"), ("🗺️gismap", "map")):
    rel = P + "🌍️gis/🗿️artifacts/" + gis + S1 + "✏️editor/🎮️commands/🎨️example/🦀️.rs"
    snapshot = "GisTerrainSnapshot" if kind == "terrain" else "GisMapSnapshot"
    edit("gis", rel,
f"""    let source = example_catalogue().into_iter().find(|source| source.id() == example_id).ok_or_else(|| Fault::from(format!("gis {kind} example '{{example_id}}' is not in the catalogue")))?;""",
f"""    let source = example_catalogue().into_iter().find(|source| source.id() == example_id).ok_or_else(|| {R}::unknown(example_id))?;""")
    tail = "" if kind == "terrain" else "?;"
    edit("gis", rel,
f"""<{snapshot} as store::ArtifactDsl>::parse_dsl(&source.document_json()).map_err(|error| Fault::from(format!("gis {kind} example '{{example_id}}' does not parse: {{error:?}}"))){tail}""",
f"""<{snapshot} as store::ArtifactDsl>::parse_dsl(&source.document_json()).map_err(|error| {R}::undecodable(example_id, error.to_string()).into()){tail}""" if kind == "terrain" else
f"""<{snapshot} as store::ArtifactDsl>::parse_dsl(&source.document_json()).map_err(|error| {R}::undecodable(example_id, error.to_string()))?;""")

edit("shooting", P + "🎥️shooting/🗿️artifacts/🎥️shooting" + S1 + "✏️editor/🎮️commands/📄️document/🦀️.rs",
"""        let next = if payload.example_id.is_empty() {
            Some(crate::empty_shooting_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_DEFAULT_ID || payload.example_id == "base" || payload.example_id == "demo" {
            Some(crate::standards::v1::subsets::any::schema::default_snapshot())
        } else if payload.example_id == SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT || payload.example_id == "forest-left" {
            crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::examples::hexagonal_cut_concrete_forest_left::PRIMARY_TEXT)
                .ok()
        } else {
            None
        };
        match next {
            Some(snapshot) => Ok(Emit { effects: vec![crate::editor::shooting::reset_document_effect(&snapshot)], ..Default::default() }),
            None => Ok(Emit::default()),
        }""",
f"""        let next = if payload.example_id.is_empty() {{
            crate::empty_shooting_snapshot()
        }} else if payload.example_id == SHOOTING_EXAMPLE_DEFAULT_ID || payload.example_id == "base" || payload.example_id == "demo" {{
            crate::standards::v1::subsets::any::schema::default_snapshot()
        }} else if payload.example_id == SHOOTING_EXAMPLE_HEXAGONAL_CUT_CONCRETE_FOREST_LEFT || payload.example_id == "forest-left" {{
            crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(crate::examples::hexagonal_cut_concrete_forest_left::PRIMARY_TEXT).map_err(|error| {R}::undecodable(&payload.example_id, format!("{{error:?}}")))?
        }} else {{
            return Err({R}::unknown(&payload.example_id).into());
        }};
        Ok(Emit {{ effects: vec![crate::editor::shooting::reset_document_effect(&next)], ..Default::default() }})""")

edit("demonstrator", P + "🎪️demonstrator/🗿️artifacts/🎪️playground" + S1 + "✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs",
"""        <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).map_err(|error| Fault::from(error.to_string()))?.schema
    } else {
        return Ok(Emit::default());
    };""",
f"""        <PlaygroundSnapshot as store::ArtifactDsl>::parse_dsl(demo::PRIMARY_TEXT).map_err(|error| {R}::undecodable(demo::ID, error.to_string()))?.schema
    }} else {{
        return Err({R}::unknown(&payload.example_id).into());
    }};""")

FEM = P + "🏗️fem/🗿️artifacts/"
FEMS = "/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📚️set-active-example/🦀️.rs"
edit("fem", FEM + "◻️2d" + FEMS,
"""/// 📚️ The bundled example's own `ExampleSource` id (`📚️examples/🎬️demo`) loads that fixture; any other
/// id resets to an empty document. The id is not a local literal because the shell's navbar switcher""",
"""/// 📚️ The bundled example's own `ExampleSource` id (`📚️examples/🎬️demo`) loads that fixture and the empty id
/// an empty document; any other id is the typed `example.unknown` refusal, and a fixture that no longer
/// parses `example.undecodable`. The id is not a local literal because the shell's navbar switcher""")
edit("fem", FEM + "◻️2d" + FEMS,
"""    let document = if payload.example_id == crate::examples::demo::ID {
        Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).unwrap_or_else(|_| crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot())
    } else {
        crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot()
    };""",
f"""    let document = if payload.example_id == crate::examples::demo::ID {{
        Fem2dSnapshot::parse_dsl(crate::editor::fem2d::FEM2D_EXAMPLE_DSL).map_err(|error| {R}::undecodable(crate::examples::demo::ID, format!("{{error:?}}")))?
    }} else if payload.example_id.is_empty() {{
        crate::standards::v1::subsets::any::schema::empty_fem2d_snapshot()
    }} else {{
        return Err({R}::unknown(&payload.example_id).into());
    }};""")
edit("fem", FEM + "🧊️3d" + FEMS,
"""    let text = match payload.example_id.as_str() {
        id if id == crate::examples::demo::ID => Some(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT),
        id if id == crate::examples::concrete_forest::ID => Some(crate::examples::concrete_forest::PRIMARY_TEXT),
        id if id == crate::examples::house::ID => Some(crate::examples::house::PRIMARY_TEXT),
        _ => None,
    };
    let document = text.map(|text| <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text).unwrap_or_default()).unwrap_or_default();
    eprintln!("[DEBUG] fem3d setActiveExample id={} nodes={} elements={} solids={}", payload.example_id, document.nodes.len(), document.elements.len(), document.solids.len());
""",
f"""    let text = match payload.example_id.as_str() {{
        "" => None,
        id if id == crate::examples::demo::ID => Some(crate::standards::v1::subsets::any::schema::snapshot::text::FEM3D_EXAMPLE_TEXT),
        id if id == crate::examples::concrete_forest::ID => Some(crate::examples::concrete_forest::PRIMARY_TEXT),
        id if id == crate::examples::house::ID => Some(crate::examples::house::PRIMARY_TEXT),
        other => return Err({R}::unknown(other).into()),
    }};
    let document = match text {{
        Some(text) => <Fem3dSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| {R}::undecodable(&payload.example_id, error.to_string()))?,
        None => Fem3dSnapshot::default(),
    }};
""")

WIRES = P + "💡️reasoning/🗿️artifacts/🔌️wires" + S1 + "✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs"
edit("reasoning", WIRES,
"""    let next = if payload.example_id.as_str() == WIRES_PLAY_EXAMPLE_METABOLISM_ID {
        metabolism_wires_example_snapshot().map_err(|error| {
            let message = if error.target.is_empty() { error.message.clone() } else { format!("{} at {}", error.message, error.target.join(".")) };
            Fault::new(FaultOrigin::App, FaultCode::new(error.code), message)
        })?
    } else {
        empty_wires_snapshot()
    };""",
f"""    let next = if payload.example_id.as_str() == WIRES_PLAY_EXAMPLE_METABOLISM_ID {{
        metabolism_wires_example_snapshot().map_err(|error| {R}::undecodable(WIRES_PLAY_EXAMPLE_METABOLISM_ID, error.message))?
    }} else if payload.example_id.is_empty() {{
        empty_wires_snapshot()
    }} else {{
        return Err({R}::unknown(&payload.example_id).into());
    }};""")
edit("reasoning", WIRES, "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};\n", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};\n")

edit("forms", P + "📋️forms/🗿️artifacts/📋️forms" + S1 + "✏️editor/🎮️commands/📥️set-active-example/🦀️.rs",
"""            _ => return Err(Fault::from("forms.template.unknown")),
        };
        forms_dsl::parse_dsl(text).map_err(|error| Fault::from(format!("forms.template.invalid: {error}")))?""",
f"""            other => return Err({R}::unknown(other).into()),
        }};
        forms_dsl::parse_dsl(text).map_err(|error| {R}::undecodable(&payload.example_id, error.to_string()))?""")

edit("cad", P + "📐️cad/🗿️artifacts/📐️cad" + S1 + "✏️editor/🎮️commands/🗺️model-definition/🦀️.rs",
"""            <CadSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| Fault::from(format!("cad.example.invalid: registered example `{example_id}` does not parse: {error:?}")))?
        } else {
            return Err(Fault::from(format!("cad.example.unknown: `{example_id}` is not an example this app ships")));
        };""",
f"""            <CadSnapshot as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| {R}::undecodable(example_id, error.to_string()))?
        }} else {{
            return Err({R}::unknown(example_id).into());
        }};""")

NORM = P + "📕️norm/🗿️artifacts/"
NORMS = S1 + "✏️editor/🎮️commands/🎨️set-active-example/🦀️.rs"
edit("norm", NORM + "⚖️en1990" + NORMS,
"""/// 🎨️ Replaces the live document with the named example's `PRIMARY_TEXT`, or clears it when the id is empty.""",
"""/// 🎨️ Replaces the live document with the named example's `PRIMARY_TEXT`, or clears it when the id is empty; an id
/// this editor does not declare is the typed `example.unknown` refusal the shell shows.""")
edit("norm", NORM + "⚖️en1990" + NORMS,
"""        id if id == crate::standards::v1::subsets::any::examples::fatigue_failing::ID => crate::standards::v1::subsets::any::examples::fatigue_failing::PRIMARY_TEXT.to_string(),
        _ => return Ok(Emit::default()),""",
f"""        id if id == crate::standards::v1::subsets::any::examples::fatigue_failing::ID => crate::standards::v1::subsets::any::examples::fatigue_failing::PRIMARY_TEXT.to_string(),
        other => return Err({R}::unknown(other).into()),""")
edit("norm", NORM + "⚡️din18599" + NORMS,
"""/// 🎨️ Replaces the live document with a named example's `PRIMARY_TEXT` DSL asset.""",
"""/// 🎨️ Replaces the live document with a named example's `PRIMARY_TEXT` DSL asset; an id this editor does not
/// declare is the typed `example.unknown` refusal the shell shows.""")
edit("norm", NORM + "⚡️din18599" + NORMS,
"""        "cooled-office" => crate::examples::cooled_office::PRIMARY_TEXT,
        _ => return Ok(Emit::default()),""",
f"""        "cooled-office" => crate::examples::cooled_office::PRIMARY_TEXT,
        other => return Err({R}::unknown(other).into()),""")

for plugin, rel, snapshot, empty, label, noun in (
    ("playbook", P + "📖️playbook/🗿️artifacts/📖️playbook" + S1 + "✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs", "PlaybookSnapshot", "crate::empty_playbook_snapshot()", "playbook-example-unparsable", "playbook"),
    ("imperative", P + "📜️imperative/🗿️artifacts/📜️procedure" + S1 + "✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs", "ProcedureSnapshot", "ProcedureSnapshot::default()", "imperative-example-unparsable", "procedure"),
):
    doc_noun = "demo document" if plugin == "playbook" else "demo program"
    edit(plugin, rel,
f"""/// `.dsl.semio` asset IS this app's canonical {doc_noun}; every other id loads the empty {noun}.""",
f"""/// `.dsl.semio` asset IS this app's canonical {doc_noun}; the empty id loads the empty {noun}, and any other id
/// is the typed `example.unknown` refusal the shell shows.""")
    edit(plugin, rel,
f"""        <{snapshot} as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| Fault::from(format!("{label}: {{error}}")))?
    }} else {{
        {empty}
    }};""",
f"""        <{snapshot} as store::ArtifactDsl>::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map_err(|error| {R}::undecodable(crate::examples::demo::ID, error.to_string()))?
    }} else if payload.example_id.is_empty() {{
        {empty}
    }} else {{
        return Err({R}::unknown(&payload.example_id).into());
    }};""")

edit("remodel", P + "📸️remodel/🗿️artifacts/📸️remodeling" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
"""/// 🎬️ Replaces the document's declared state with the named example's. An unknown id, or an example
/// whose committed text no longer parses, is a no-op rather than a fault: the picker is a navigation
/// affordance, not a destructive verb.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {
    let Some(text) = example_text(&payload.example_id) else { return Ok(Emit::default()) };
    let Ok(next) = crate::snapshot::text::parse_dsl(text) else { return Ok(Emit::default()) };""",
f"""/// 🎬️ Replaces the document's declared state with the named example's. The empty id keeps the open
/// document; an unknown id is the typed `example.unknown` refusal and an example whose committed text no
/// longer parses `example.undecodable` — the picker never changes nothing in silence.
pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, NoConfig>) -> Result<Emit<RemodelingMutation, NoConfigMutation>, Fault> {{
    if payload.example_id.is_empty() {{
        return Ok(Emit::default());
    }}
    let text = example_text(&payload.example_id).ok_or_else(|| {R}::unknown(&payload.example_id))?;
    let next = crate::snapshot::text::parse_dsl(text).map_err(|error| {R}::undecodable(&payload.example_id, format!("{{error:?}}")))?;""")

TRW = P + "🔱️trinity/🗿️artifacts/♻️rewriting" + S1 + "✏️editor/"
edit("trinity", TRW + "🎮️commands/🎯️set-active-example/🦀️.rs",
"""/// 🎬️ The ids this verb answers. The navbar example picker dispatches a REGISTERED example id, and
/// `demo` is the only example this subset registers, so that id must resolve or every navbar pick is
/// inert. `default` names the blank rule `resetRule` builds, for the catalogue's own rows.
pub(crate) fn set_active_example_document(example_id: &str) -> Option<RewritingSnapshot> {
    match example_id {
        "default" | "blank" => Some(crate::editor::rewriting::default_rule_state()),
        id if id == crate::examples::demo::ID => RewritingSnapshot::parse_dsl(crate::examples::demo::PRIMARY_TEXT).ok(),
        _ => None,
    }
}""",
f"""/// 🎬️ The ids this verb answers. The navbar example picker dispatches a REGISTERED example id, and
/// `demo` is the only example this subset registers, so that id must resolve or every navbar pick is
/// inert. `default` names the blank rule `resetRule` builds, for the catalogue's own rows; the empty id
/// keeps the open rule (`None`), and any other id — or a demo asset that no longer parses — is the typed
/// `ExampleRefusal` the shell shows.
pub(crate) fn set_active_example_document(example_id: &str) -> Result<Option<RewritingSnapshot>, Fault> {{
    match example_id {{
        "" => Ok(None),
        "default" | "blank" => Ok(Some(crate::editor::rewriting::default_rule_state())),
        id if id == crate::examples::demo::ID => RewritingSnapshot::parse_dsl(crate::examples::demo::PRIMARY_TEXT).map(Some).map_err(|error| {R}::undecodable(id, format!("{{error:?}}")).into()),
        other => Err({R}::unknown(other).into()),
    }}
}}""")
edit("trinity", TRW + "🎮️commands/🎯️set-active-example/🦀️.rs",
"""pub(crate) fn set_active_example(example_id: &str) -> Emit<RewriteRuleMutation, NoConfigMutation> {
    match set_active_example_document(example_id) {
        Some(next) => Emit { effects: vec![crate::editor::rewriting::reset_document_effect(&next)], ..Default::default() },
        None => Emit::default(),
    }
}""",
"""pub(crate) fn set_active_example(example_id: &str) -> Result<Emit<RewriteRuleMutation, NoConfigMutation>, Fault> {
    Ok(match set_active_example_document(example_id)? {
        Some(next) => Emit { effects: vec![crate::editor::rewriting::reset_document_effect(&next)], ..Default::default() },
        None => Emit::default(),
    })
}""")
edit("trinity", TRW + "🦀️.rs",
"""        TrinityRewritingCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),""",
"""        TrinityRewritingCommand::SetActiveExample { example_id } => commands::set_active_example(example_id)?,""")
edit("trinity", TRW + "🦀️.rs",
"""            TrinityRewritingCommand::SetActiveExample { example_id } => crate::editor::rewriting::commands::set_active_example(example_id),""",
"""            TrinityRewritingCommand::SetActiveExample { example_id } => crate::editor::rewriting::commands::set_active_example(example_id)?,""")
edit("trinity", TRW + "🧪️tests/🔬️unit/🦀️.rs",
"""crate::editor::rewriting::commands::set_active_example_document(crate::examples::demo::ID).expect(""",
"""crate::editor::rewriting::commands::set_active_example_document(crate::examples::demo::ID).expect("the demo example seats").expect(""")

TRJ = P + "🔱️trinity/🗿️artifacts/🔌️jack" + S1 + "✏️editor/"
edit("trinity", TRJ + "🎮️commands/🎯️set-active-example/🦀️.rs",
"""pub(crate) fn set_active_example(example_id: &str) -> Emit<TrinityGraphMutation, NoConfigMutation> {
    match fixture_dsl_for_preset(example_id).and_then(|dsl| JackSnapshot::parse_dsl(dsl).ok()) {
        Some(next) => {
            Emit { effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() }
        }
        None => Emit::default(),
    }
}""",
f"""/// 🎯️ Seats a preset fixture by id: the empty id keeps the open graph, an id no preset names is the typed
/// `example.unknown` refusal, and a preset fixture that no longer parses `example.undecodable`.
pub(crate) fn set_active_example(example_id: &str) -> Result<Emit<TrinityGraphMutation, NoConfigMutation>, Fault> {{
    if example_id.is_empty() {{
        return Ok(Emit::default());
    }}
    let dsl = fixture_dsl_for_preset(example_id).ok_or_else(|| {R}::unknown(example_id))?;
    let next = JackSnapshot::parse_dsl(dsl).map_err(|error| {R}::undecodable(example_id, format!("{{error:?}}")))?;
    Ok(Emit {{ effects: vec![crate::editor::jack::reset_document_effect(&next)], ..Default::default() }})
}}""")
edit("trinity", TRJ + "🦀️.rs",
"""        TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),
        TrinityJackCommand::SetFixtureJson { json } => commands::set_fixture_json(json),""",
"""        TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id)?,
        TrinityJackCommand::SetFixtureJson { json } => commands::set_fixture_json(json),""")
edit("trinity", TRJ + "🦀️.rs",
"""            TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id),
            TrinityJackCommand::SetViewport { viewport, .. } => return commands::set_viewport(viewport, view_state),""",
"""            TrinityJackCommand::SetActiveExample { example_id } => commands::set_active_example(example_id)?,
            TrinityJackCommand::SetViewport { viewport, .. } => return commands::set_viewport(viewport, view_state),""")

edit("dag", P + "🕸️dag/🗿️artifacts/🕸️dag" + S1 + "✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs",
"""/// 🧬️ The subset registers exactly one example (`crate::examples::demo`, `ID = "demo"`), whose asset
/// IS this app's canonical default document; every other id loads the empty graph.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {
    let next = if payload.example_id.as_str() == crate::examples::demo::ID { crate::default_snapshot() } else { crate::empty_snapshot() };""",
f"""/// 🧬️ The subset registers exactly one example (`crate::examples::demo`, `ID = "demo"`), whose asset
/// IS this app's canonical default document; the empty id loads the empty graph, and any other id is the
/// typed `example.unknown` refusal the shell shows.
pub fn handle(payload: &SetActiveExample, _doc: &ArtifactView<'_, DagSnapshot>, _cfg: &ConfigView<'_, DagConfig>) -> Result<Emit<DagMutation, DagConfigMutation>, Fault> {{
    let next = match payload.example_id.as_str() {{
        id if id == crate::examples::demo::ID => crate::default_snapshot(),
        "" => crate::empty_snapshot(),
        other => return Err({R}::unknown(other).into()),
    }};""")

edit("draw", P + "🖍️draw/🗿️artifacts/🖍️drawing" + S1 + "✏️editor/🎮️commands/🖼️set-active-example/🦀️.rs",
"""        let source = examples().iter().find(|source| source.id() == payload.example_id).ok_or_else(|| Fault::new(FaultOrigin::App, FaultCode::new("drawing.example.unknown"), format!("Drawing declares no example '{}'", payload.example_id)))?;
        DrawingSnapshot::parse_dsl(&source.document_json()).map_err(|_| Fault::new(FaultOrigin::App, FaultCode::new("drawing.example.parse"), format!("Drawing example '{}' does not parse as a drawing document", payload.example_id)))?""",
f"""        let source = examples().iter().find(|source| source.id() == payload.example_id).ok_or_else(|| {R}::unknown(&payload.example_id))?;
        DrawingSnapshot::parse_dsl(&source.document_json()).map_err(|error| {R}::undecodable(&payload.example_id, error.to_string()))?""")

edit("raster", P + "🖨️raster/🗿️artifacts/🖨️raster" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
"""/// 🎬️ Loads one registered example over the open document. An id this subset does not register is a
/// no-op rather than a fault — the navbar switcher is free-text on the wire, and an unknown id must
/// not destroy the open document.""",
"""/// 🎬️ Loads one registered example over the open document. The empty id keeps the open document; an id
/// this subset does not register is the typed `example.unknown` refusal — an unknown id must not destroy
/// the open document, and must not pass in silence either.""")
edit("raster", P + "🖨️raster/🗿️artifacts/🖨️raster" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
"""    let Some(mut example) = raster_example_document(&payload.example_id) else {
        return Ok(Emit::default());
    };""",
f"""    if payload.example_id.is_empty() {{
        return Ok(Emit::default());
    }}
    let mut example = raster_example_document(&payload.example_id).ok_or_else(|| {R}::unknown(&payload.example_id))?;""")

edit("note", P + "🗒️note/🗿️artifacts/🗒️note" + S1 + "✏️editor/🎮️commands/🗃️set-active-example/🦀️.rs",
"""        other => return Err(Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("note.example.unknown"), format!("note has no example '{other}'"))),""",
f"""        other => return Err({R}::unknown(other).into()),""")

for dim, (first, second) in (("◻️2d", ("BLOCK2D_EXAMPLE_LEFT", "BLOCK2D_EXAMPLE_RIGHT")), ("🖐️5d", ("BLOCK5D_EXAMPLE_FOREST_LEFT", "BLOCK5D_EXAMPLE_CAPSULE")), ("🧊️3d", ("BLOCK3D_EXAMPLE_CAPSULE", "BLOCK3D_EXAMPLE_FOREST_LEFT"))):
    rel = P + "🧱️block/🗿️artifacts/" + dim + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"
    text = (__import__("pathlib").Path("/Users/ueli/Documents/semio") / rel).read_text()
    lines = [line for line in text.split("\n") if line.startswith(f"        {first} => ") or line.startswith(f"        {second} => ")]
    old = "\n".join(lines) + "\n        _ => None,\n    };\n    match example {\n        Some(document) => Ok(Emit::mutations(replace_document_operations(doc.snapshot, &document))),\n        None => Ok(Emit::default()),\n    }"
    new_lines = [line.replace(".ok(),", f".map(Some).map_err(|error| {R}::undecodable(&payload.id, format!(\"{{error:?}}\")))?,") for line in lines]
    new = "\n".join(new_lines) + f"\n        \"\" => None,\n        other => return Err({R}::unknown(other).into()),\n    }};\n    match example {{\n        Some(document) => Ok(Emit::mutations(replace_document_operations(doc.snapshot, &document))),\n        None => Ok(Emit::default()),\n    }}"
    edit("block", rel, old, new)

edit("sourcing", P + "🪵️sourcing/🗿️artifacts/🗂️curation" + S1 + "✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs",
"""        id => crate::standards::v1::subsets::any::examples().iter().find(|example| example.id() == id).map(semio_framework_plugin::ExampleSource::document).ok_or_else(|| Fault::from("sourcing.example.unknown"))?,
    };
    let next = <CurationSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| Fault::from(error.to_string()))?;""",
f"""        id => crate::standards::v1::subsets::any::examples().iter().find(|example| example.id() == id).map(semio_framework_plugin::ExampleSource::document).ok_or_else(|| {R}::unknown(id))?,
    }};
    let next = <CurationSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|error| {R}::undecodable(&payload.example_id, error.to_string()))?;""")
