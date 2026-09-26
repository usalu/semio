#!/usr/bin/env python3
"""📐️ P8 post-publish patch set: cad's five dead verbs become reachable and say what they really do.

`applyTransformation`, `importCadFile`, `saveSelected`, `saveInPlay` and `saveCurrent` were `BatchOnlyPendingRewrite`, so the
shell and every agent got `interactive-job.not-ui-safe` before any handler ran (G10's run 4: cad `importCadFile`). They move
onto cad's one retained factory. Two of their handlers are documented stubs since the composable-artifact migration
(`apply_transformation_mutations` returns nothing; importing an OBJ/STL/STEP object needs the pane model-child seam):
instead of settling silently they now refuse by name, and the object import stub is routed to the cad owner (report
§ Routed). A whole spatial scene still imports (host `LoadDocument`), and the three exports still download.

Parts: A retained routing (tool ids, lanes, raw budget, reducer arm, proofs); B classification; C honest stubs + tests.
Usage: p8-cad.py --dry-run | --write"""
from p8_patch import ROOT, finish, replace

CAD = ROOT / "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor"
EDITOR = CAD / "🦀️.rs"
IO = CAD / "🎮️commands/📥️io/🦀️.rs"
TRANSFORM = CAD / "🎮️commands/🔄️transform/🦀️.rs"
UNIT = CAD / "🧪️tests/🔬️unit/🦀️.rs"

replace("A", EDITOR, '''const CAD_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addNode", "renameNode", "patchCadPlayReference", "addObject", "patchObject", "patchSelection", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection", "engagementSubmit", "engagementPossibleSelect", "worldPointerDown"];''',
        '''const CAD_RETAINED_ARTIFACT_TOOL_IDS: &[&str] = &["addNode", "renameNode", "patchCadPlayReference", "addObject", "patchObject", "patchSelection", "deleteObject", "duplicateObject", "translateSelection", "rotateSelection", "scaleSelection", "engagementSubmit", "engagementPossibleSelect", "worldPointerDown", "applyTransformation"];''')
replace("A", EDITOR, '''    "setContributions",
    "setActiveExample",
];
const CAD_RETAINED_TOOL_IDS: &[&str] = &[''', '''    "setContributions",
    "setActiveExample",
    "importCadFile",
];
/// 📤️ The exports read the scene and hand the host a download; they touch no store.
const CAD_RETAINED_EXPORT_TOOL_IDS: &[&str] = &["saveSelected", "saveInPlay", "saveCurrent"];
const CAD_RETAINED_TOOL_IDS: &[&str] = &[''')
replace("A", EDITOR, '''    "setActiveExample",
    "loadRawRequest",
];
const CAD_RETAINED_COMMAND_SCHEMA''', '''    "setActiveExample",
    "loadRawRequest",
    "applyTransformation",
    "importCadFile",
    "saveSelected",
    "saveInPlay",
    "saveCurrent",
];
const CAD_RETAINED_COMMAND_SCHEMA''')
replace("A", EDITOR, '''    ArtifactToolPublicationContract { tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];''', '''    ArtifactToolPublicationContract { tool_id: "loadRawRequest", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "applyTransformation", lanes: &[ArtifactToolPublicationLane::Artifact] },
    // 🗃️ A whole spatial scene imports like an example switch: the session config plus ONE host `LoadDocument`.
    ArtifactToolPublicationContract { tool_id: "importCadFile", lanes: &[ArtifactToolPublicationLane::Config] },
    ArtifactToolPublicationContract { tool_id: "saveSelected", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "saveInPlay", lanes: &[ArtifactToolPublicationLane::HostOnly] },
    ArtifactToolPublicationContract { tool_id: "saveCurrent", lanes: &[ArtifactToolPublicationLane::HostOnly] },
];''')
replace("A", EDITOR, '''fn cad_retained_raw_bytes(tool_id: &str) -> usize {
    if tool_id == "setContributions" {''', '''fn cad_retained_raw_bytes(tool_id: &str) -> usize {
    if tool_id == "setContributions" || tool_id == "importCadFile" {''')
replace("A", EDITOR, '''    match command {
        CadCommand::LoadRawRequest(payload) => load_raw_request::handle(payload, &doc, &cfg, &mut ctx),
        _ => Err(Fault::from("cad-retained-route-mismatch")),
    }''', '''    if CAD_RETAINED_EXPORT_TOOL_IDS.contains(&command.command_id()) {
        admit_cad_snapshot(snapshot).map_err(Fault::from)?;
        return command.dispatch(&doc, &cfg, &mut ctx);
    }
    match command {
        CadCommand::LoadRawRequest(payload) => load_raw_request::handle(payload, &doc, &cfg, &mut ctx),
        _ => Err(Fault::from("cad-retained-route-mismatch")),
    }''')
replace("A", EDITOR, '''            "setActiveExample",
            "loadRawRequest"
        ]
    }''', '''            "setActiveExample",
            "loadRawRequest",
            "applyTransformation",
            "importCadFile",
            "saveSelected",
            "saveInPlay",
            "saveCurrent"
        ]
    }''')
for verb in ["applyTransformation", "importCadFile", "saveSelected", "saveInPlay", "saveCurrent"]:
    replace("B", EDITOR, f'''            .action_interactive_job("{verb}", InteractiveJobClassification::BatchOnlyPendingRewrite)''', f'''            .action_interactive_job("{verb}", InteractiveJobClassification::Migrated)''')
replace("B", EDITOR, '''            .action_describe("applyTransformation", LocalizedLabel::native("Bakes the staged transformation into the selected objects' geometry.",''',
        '''            .action_describe("applyTransformation", LocalizedLabel::native("Bakes the staged transformation into the selected objects' geometry; refused until composed pane models support it.",''')
replace("B", EDITOR, '''            .action_describe("importCadFile", LocalizedLabel::native("Reads a CAD file (STEP, OBJ, STL) into the model.", "Liest eine CAD-Datei (STEP, OBJ, STL) in das Modell ein."))''',
        '''            .action_describe("importCadFile", LocalizedLabel::native("Replaces the whole CAD scene with a spatial scene file; single STEP, OBJ or STL objects are refused until composed pane models support them.", "Ersetzt die gesamte CAD-Szene durch eine räumliche Szenendatei; einzelne STEP-, OBJ- oder STL-Objekte werden abgelehnt, bis zusammengesetzte Bereichsmodelle sie unterstützen."))''')

replace("C", TRANSFORM, '''    pub fn handle(payload: &ApplyTransformation, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        Ok(Emit::mutations(apply_transformation_mutations(doc.snapshot, &payload.qid)))
    }''', '''    /// 🔄️ Refused by name while baking into composed pane models is unimplemented (`apply_transformation_mutations`
    /// has nothing to bake into since the composable-artifact migration) — never an empty success.
    pub fn handle(payload: &ApplyTransformation, doc: &ArtifactView<'_, CadSnapshot>, _cfg: &ConfigView<'_, CadConfig>, _ctx: &mut CadDispatchCtx) -> Result<Emit<CadMutation, CadConfigMutation>, Fault> {
        let mutations = apply_transformation_mutations(doc.snapshot, &payload.qid);
        if mutations.is_empty() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.apply-transformation-unavailable"), format!("applyTransformation cannot bake \\"{}\\": composed pane models accept no baked transformation yet", payload.qid)));
        }
        Ok(Emit::mutations(mutations))
    }''')
replace("C", IO, '''        if import_cad_object_by_extension(&name_lower, &payload_value).is_some() {
            return Ok(Emit::default());
        }''', '''        if import_cad_object_by_extension(&name_lower, &payload_value).is_some() {
            return Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.import-object-unavailable"), format!("importCadFile cannot place the object \\"{}\\": composed pane models accept no imported object yet", payload.name)));
        }''')
replace("C", IO, '''            return Ok(emit);
        }
        Ok(Emit::default())
    }
}
//#endregion 🔖️ImportCadFile''', '''            return Ok(emit);
        }
        Err(Fault::new(FaultOrigin::App, FaultCode::new("cad.import-unreadable"), format!("importCadFile cannot read \\"{}\\" as a spatial scene or a CAD object", payload.name)))
    }
}
//#endregion 🔖️ImportCadFile''')
replace("C", UNIT, '''    let emit = drive(&app, &scene, "importCadFile", Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })));
    assert!(emit.artifact_mutations.is_empty(), "importCadFile's document write is a documented no-op until the child-dispatch seam lands");
    assert!(emit.config_mutations.is_empty(), "importCadFile no longer touches config once selection moved to the framework");
}''', '''    let fault = drive_with_operation(&app, &scene, "importCadFile", Some(json!({ "payload": obj_data_url, "name": "triangle.obj" })), &CadConfig::default(), None).err().expect("an object import is refused until composed pane models accept it");
    assert_eq!(fault.code.0, "cad.import-object-unavailable");
}''')

replace("C", IO, "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};")
replace("C", TRANSFORM, "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};", "use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultCode, FaultOrigin};")

finish(__doc__)
