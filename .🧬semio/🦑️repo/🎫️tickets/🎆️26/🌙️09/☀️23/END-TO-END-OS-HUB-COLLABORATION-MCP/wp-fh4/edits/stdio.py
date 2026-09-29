"""🗄️ stdio (P2-Y): every stdio refusal reports the frozen code it means, addressed at its element — profile/class/view/
serialization rules → Invariant, an addressed node that is not there → TargetMissing, a name already taken → DuplicateId.
Helpers that only carried the dropped sentence lose it (`rejected`, `target_error`, the i-json `Refusal`, gltf's
`rejection_outcome`); the stdio contract answers a refused snapshot patch with one shared report (`editing::patch_refusal`);
readers of the removed report text (checked-apply wrappers, test diagnostics) show code + target; the laws assert codes."""
import re

from rsargs import codemod, drop_argument

ART = "🗄️stdio/🗿️artifacts/"
ZIPI = ART + "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320/🧬️schema/🧬️mutations/🦀️.rs"
ZIPB_DIFF = ART + "🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs"
SVGB = ART + "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic/🧬️schema/🧬️mutations/🦀️.rs"
SVGT = ART + "🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny/🧬️schema/🧬️mutations/🦀️.rs"
XMLV = ART + "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/🦀️.rs"
XMLT = ART + "📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
IFC = ART + "🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/"
STEP = ART + "📐️step/🏅️standards/🔖️ap214/🪆️subsets/"
LADDER = STEP + "🧱️base/🚪️io/🪜️ladder/🦀️.rs"
WAV = ART + "🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧬️schema/🧬️mutations/"
PNG = ART + "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🩹️patch-pixels/🦀️.rs"
CONTRACT = "🗄️stdio/📇️registry/🧬️contract/✏️editing/"
JSONI = ART + "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/🦀️.rs"
JSONT = ART + "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
XLSX = ART + "📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"
DOCX = ART + "📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🦀️.rs"
SEMIO = ART + "🧿️semio/🏅️standards/🔖️v1/🪆️subsets/"
DWG = ART + "🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"
GLTF = ART + "🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/"
GLTF_MUT = GLTF + "♾️any/🧬️schema/🧬️mutations/🦀️.rs"
GLTF_TOP = GLTF + "♾️any/🔨️modules/🧬️mutation-support/📚️top-level/🦀️.rs"
PATCHED = [ART + rel + "/🧬️schema/🧬️mutations/🩹️patch-snapshot/🦀️.rs" for rel in (
    "🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any", "📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any", "📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any",
    "📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document", "🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any",
    "🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document", "🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base")]
TARGET_ERROR = [ART + rel + "/🧬️schema/🔺️diff/🦀️.rs" for rel in (
    "🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header", "🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any", "📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base",
    "🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any")]
CLASSES = ["1️⃣cc1", "2️⃣cc2", "3️⃣cc3", "4️⃣cc4", "5️⃣cc5", "6️⃣cc6"]
SEMIO_CHILDREN = [SEMIO + rel + "/🔺️diff/🦀️.rs" for rel in (
    "📦️object/🧬️schema/🧬️mutations/🏷️create-properties", "📦️object/🧬️schema/🧬️mutations/🕸️create-mesh", "📦️object/🧬️schema/🧬️mutations/🧱create-brep",
    "🧰️kit/🧬️schema/🧬️mutations/🏗️create-object", "🧰️kit/🧬️schema/🧬️mutations/🏛️create-model", "🧰️kit/🧬️schema/🧬️mutations/🏷️create-properties")]
IFC_VIEWS = {
    "🏢️cobie": ("Ifc2x3CobieMutation", [("SetFacilityName", "set_facility_name", "building"), ("SetFloorElevation", "set_floor_elevation", "storey"),
                                          ("SetSpace", "set_space", "id"), ("SetTypeAssignment", "set_type_assignment", "id")], "stdio.ifc.2x3.cobie.mutation-rejected", 146),
    "🤝️cv20": ("Ifc2x3Cv20Mutation", [("SetStructuralEntity", "set_structural_entity", "id"), ("SetProjectUnits", "set_project_units", "project"),
                                        ("SetProductPlacement", "set_product_placement", "product")], "stdio.ifc.2x3.cv20.mutation-rejected", 115),
    "🧮️sav": ("Ifc2x3SavMutation", [("SetAnalysisModel", "set_analysis_model", "id"), ("SetLoadGroup", "set_load_group", "id"),
                                      ("SetGroupAssignment", "set_group_assignment", "id")], "stdio.ifc.2x3.sav.mutation-rejected", 151),
}
REJECTED_OUTCOME = "protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new())"

OVERRIDES = {
    (ZIPI, 134): {"variant": "DuplicateId"}, (ZIPI, 137): {"variant": "TargetMissing"}, (ZIPI, 146): {"variant": "DuplicateId"},
    (ZIPB_DIFF, 189): {"skip": True},
    **{(SVGB, line): {"skip": True} for line in (259, 267, 277, 285, 288, 292)},
    **{(SVGT, line): {"skip": True} for line in (229, 240)},
    (XMLV, 202): {"skip": True},
    **{(IFC + view + "/🧬️schema/🧬️mutations/🦀️.rs", line): {"skip": True} for view, (_, _, _, line) in IFC_VIEWS.items()},
    **{(STEP + cc + "/🧬️schema/🧬️mutations/🦀️.rs", line): {"skip": True} for cc, line in zip(CLASSES, (132, 114, 113, 112, 113, 117))},
    **{(path, line): {"skip": True} for path, line in zip(PATCHED, (20, 26, 26, 26, 18, 26, 26))},
    **{(path, line): {"skip": True} for path, line in zip(TARGET_ERROR, (1476, 533, 509, 512))},
    (WAV + "🦀️.rs", 103): {"skip": True}, (WAV + "🩹️patch-data/🦀️.rs", 69): {"skip": True}, (PNG, 49): {"skip": True},
    (JSONI, 285): {"skip": True}, (XLSX, 110): {"skip": True}, (DOCX, 433): {"skip": True},
}

NODE_TARGET = '''/// 🎯️ The report address of the element at `path`: its child indices from the document element down.
fn node_target(path: &[usize]) -> Vec<String> {
    path.iter().map(usize::to_string).collect()
}'''

EDITS = [
    (ZIPI, 'const CODE_REJECTED: &str = "stdio.zip.iso21320.mutation-outside-profile";\n\n', ""),
    (ZIPB_DIFF, '.map_err(|error| MutationApplyError::new("stdio.zip.serialization.invalid-state", error.to_string()).at(["entries"]))?;',
     '.map_err(|_| MutationApplyError::new(MutationCode::Invariant).at(["entries"]))?;'),
    (SVGB, 'const CODE_REJECTED: &str = "stdio.svg.basic.mutation-outside-profile";', NODE_TARGET),
    (SVGB, f"Some(message) => {REJECTED_OUTCOME},", "Some(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, node_target(parent)),"),
    (SVGB, f"""if let Err(message) = resolve_clip_path(base, id) {{
                        return {REJECTED_OUTCOME};""", """if resolve_clip_path(base, id).is_err() {
                        return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, node_target(path));"""),
    (SVGB, f"Err(message) => return {REJECTED_OUTCOME},", "Err(_) => return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, node_target(path)),"),
    (SVGB, 'return protocol::MutationOutcome::error(CODE_REJECTED, "the inserted shape carries a text element -- SVG Basic 1.1 forbids clipping to text".to_string(), Vec::<String>::new());',
     "return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, vec![clip_path_id.clone()]);"),
    (SVGB, f"""if let Some(message) = subtree_profile_violation(node) {{
                return {REJECTED_OUTCOME};""", """if subtree_profile_violation(node).is_some() {
                return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, vec![clip_path_id.clone()]);"""),
    (SVGB, f"Err(message) => {REJECTED_OUTCOME},", "Err(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, vec![clip_path_id.clone()]),"),
    (SVGT, 'const CODE_REJECTED: &str = "stdio.svg.tiny.mutation-outside-profile";', NODE_TARGET),
    (SVGT, f"Some(message) => {REJECTED_OUTCOME},", "Some(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, node_target(parent)),"),
    (SVGT, '''return protocol::MutationOutcome::error(CODE_REJECTED, format!("attribute '{name}' is forbidden anywhere in SVG Tiny 1.1"), Vec::<String>::new());''',
     "return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, node_target(path));"),
    (XMLV, '''/// 🚫 The fault code every rejected `✳️valid` mutation reports under.
pub const CODE_REJECTED: &str = "stdio.xml.valid.mutation-outside-subset";
''', ""),
    (XMLV, '''// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn rejected(message: String) -> protocol::MutationOutcome<XmlDiff> {
    protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new())
}

''', ""),
    (XMLV, '''Some(message) => rejected(format!("set-snapshot: the replacement document is not XML 1.0 valid — {message}")),''',
     "Some(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, Vec::<String>::new()),"),
    (XMLV, '''rejected("declare-doctype: the document has no document element, so §2.8 gives the DOCTYPE no Name to carry".to_string())''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, ["root"])'''),
    (XMLV, '''rejected("rename-document-element: the document has no document element to rename".to_string())''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, ["root"])'''),
    (XMLV, '''rejected("rename-document-element: the document has no DOCTYPE to keep in step with the new name — declare one first".to_string())''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, ["doctype"])'''),
    (XMLV, '''rejected("set-external-subset: the document has no DOCTYPE to attach an external subset reference to".to_string())''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, ["doctype"])'''),
    (XMLV, '''rejected("declare-entity: the document has no DOCTYPE, so there is no internal subset to declare an entity in".to_string())''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, ["doctype"])'''),
    (XMLV, '''rejected(format!("declare-entity: '{name}' is already declared — XML 1.0 §4.2 binds the FIRST declaration, so a second one is dead markup rather than an edit"))''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::DuplicateId, vec!["doctype".to_string(), name.clone()])'''),
    (XMLV, '''rejected("set-internal-subset: the document has no DOCTYPE, so there is no internal subset to replace".to_string())''',
     '''protocol::MutationOutcome::error(protocol::MutationCode::TargetMissing, ["doctype"])'''),
    (XMLT, '''    assert!(outcome.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "got {:?}", outcome.messages());''',
     '''    assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.invariant"), "got {:?}", outcome.messages());'''),
    (XMLT, '''    assert!(duplicate.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "§4.2 binds the first declaration, so a second one is dead markup");''',
     '''    assert!(duplicate.messages().iter().any(|message| message.code.0 == "mutation.duplicate-id" && message.target == ["doctype", "semio"]), "§4.2 binds the first declaration, so a second one is dead markup");'''),
    (XMLT, '''    assert!(no_doctype.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "there is no internal subset without a DOCTYPE to hold it");''',
     '''    assert!(no_doctype.messages().iter().any(|message| message.code.0 == "mutation.target-missing" && message.target == ["doctype"]), "there is no internal subset without a DOCTYPE to hold it");'''),
    (XMLT, '''        assert!(!outcome.messages().iter().any(|message| message.code.0 == CODE_REJECTED), "{mutation:?} must apply against the fixture: {:?}", outcome.messages());''',
     '''        assert!(!outcome.messages().iter().any(|message| matches!(message.level, protocol::Severity::Error | protocol::Severity::Fatal)), "{mutation:?} must apply against the fixture: {:?}", outcome.messages());'''),
    (LADDER, "/// ▶️ Applies one class edit under `class`'s own ceiling.",
     """/// 🎯️ The address a refused class edit names in its mutation report: the header's file schema, the product chain's
/// entities, or the one representation entity the ladder axis edits.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn class_edit_target(edit: &ClassEdit) -> Vec<String> {
    match edit {
        ClassEdit::FileSchema { .. } => vec!["header".into(), "fileSchema".into()],
        ClassEdit::ProductIdentity { .. } => vec!["entities".into()],
        ClassEdit::Representation { id, .. } | ClassEdit::Demotion { id } => vec!["entities".into(), id.to_string()],
    }
}

/// ▶️ Applies one class edit under `class`'s own ceiling."""),
    (WAV + "🦀️.rs", "return protocol::MutationOutcome::error(issue.code, issue.message, issue.target).absorb_messages(outcome.messages().to_vec());",
     "return protocol::MutationOutcome::error(protocol::MutationCode::Invariant, issue.target).absorb_messages(outcome.messages().to_vec());"),
    (WAV + "🩹️patch-data/🦀️.rs", '''Err(message) => protocol::MutationOutcome::error("stdio.wav.patch-data.invalid-range", message, ["data".into(), "value".into(), payload.index.to_string()]),''',
     '''Err(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, ["data".into(), "value".into(), payload.index.to_string()]),'''),
    (PNG, '''Err(message) => protocol::MutationOutcome::error("stdio.png.patch-pixels.invalid-range", message, ["pixels".into(), self.index.to_string()]),''',
     '''Err(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, ["pixels".into(), self.index.to_string()]),'''),
    (CONTRACT + "🦀️.rs", "pub use patch::{apply_snapshot_patch, apply_snapshot_patch_for_dialect, inverse_snapshot_patch, prepare_snapshot_patch, SnapshotPatch,",
     "pub use patch::{apply_snapshot_patch, apply_snapshot_patch_for_dialect, inverse_snapshot_patch, patch_refusal, prepare_snapshot_patch, SnapshotPatch,"),
    (CONTRACT + "🩹️patch/🦀️.rs", """fn schema_path<S: ToValue>(snapshot: &S, path: &[String])""",
     """/// 🧾️ The mutation report of a snapshot patch this contract refused, addressed at the edit path the refusal names (else
/// the patch's first edit): a path the snapshot does not hold → target missing, every other refusal → invariant.
pub fn patch_refusal<D: Default>(fault: &Fault, patch: &SnapshotPatch) -> kernel::MutationOutcome<D> {
    let named = |path: &Vec<String>| fault.parameters.as_slice().iter().any(|parameter| parameter.name == "path" && parameter.value == pointer(path));
    let target = patch.edits.iter().map(|edit| &edit.path).find(|path| named(path)).or_else(|| patch.edits.first().map(|edit| &edit.path)).cloned().unwrap_or_default();
    if fault.code.0 == "snapshot-edit.path-invalid" {
        kernel::MutationOutcome::error(kernel::MutationCode::TargetMissing, target)
    } else {
        kernel::MutationOutcome::error(kernel::MutationCode::Invariant, target)
    }
}

fn schema_path<S: ToValue>(snapshot: &S, path: &[String])"""),
    (JSONI, '''/// 🚫️ Frozen `MutationMessage` code for every refusal below (`Fatal`, empty diff). The seven-code
/// set is closed and generic; a per-plugin code is never minted.
const CODE_INVARIANT: &str = "mutation.invariant";

/// 🚫️ Frozen code for "the addressed node is not there, or is not the kind this verb writes over".
const CODE_TARGET_MISSING: &str = "mutation.target-missing";

''', ""),
    (JSONI, '''/// 🚫️ One refused I-JSON clause: the frozen code, the prose, and the address it was refused at.
type Refusal = (&'static str, String, Vec<String>);''',
     '''/// 🚫️ One refused I-JSON clause: its `Fatal` report — the frozen code (`mutation.target-missing` for an addressed node that
/// is not there or not the kind the verb writes over, `mutation.invariant` for a value RFC 7493 forbids) at the address it
/// was refused at, with an empty diff.
type Refusal = protocol::MutationOutcome<JsonDiff>;'''),
    (JSONI, "if let Some(offending) = value.chars().find(|c| is_unicode_noncharacter(*c)) {", "if value.chars().any(is_unicode_noncharacter) {"),
    (JSONI, "Err((code, message, target)) => protocol::MutationOutcome::fatal(code, message, target),", "Err(refusal) => refusal,"),
    (JSONT, "message.code.0 == CODE_INVARIANT", 'message.code.0 == "mutation.invariant"'),
    (XLSX, '''Err(message) => protocol::MutationOutcome::error("stdio.xlsx.canonical-edit.invalid", message, ["xmlParts"]),''',
     '''Err(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, ["xmlParts"]),'''),
    (DOCX, '''Err(message) => protocol::MutationOutcome::error("stdio.docx.xml-address.invalid", message, mutation_target(this)),''',
     '''Err(_) => protocol::MutationOutcome::error(protocol::MutationCode::Invariant, mutation_target(this)),'''),
    (SEMIO + "✉️base/🧬️schema/🧬️mutations/🦀️.rs",
     '''.map(|message| format!("{:?} {:?}: {}", message.level, message.code, message.message)).collect()''',
     '''.map(|message| format!("{:?} {:?} at {}", message.level, message.code, message.target.join("/"))).collect()'''),
    (DWG, '''Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),''',
     '''Some(message) => Err(format!("{:?} was rejected: [{}] at {}", mutation, message.code.0, message.target.join("/"))),'''),
    (GLTF_MUT, '''Some(message) => Err(format!("{kind}: the {step} was refused — {} {}", message.code.0, message.message)),''',
     '''Some(message) => Err(format!("{kind}: the {step} was refused — {} at {}", message.code.0, message.target.join("/"))),'''),
    (GLTF_TOP, "pub(crate) fn rejection_outcome(code: &str, path: &str, detail: String) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {",
     "pub(crate) fn rejection_outcome(code: &str, path: &str) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {"),
    (GLTF_TOP, "/// 📨️ Converts a glTF mutation rejection into its protocol severity, target and owned detail.",
     "/// 📨️ Converts a glTF mutation rejection into its protocol severity, frozen code and target."),
]

for path in PATCHED:
    EDITS.append((path, "Err(error) => protocol::MutationOutcome::error(error.code, String::new(), Vec::<String>::new()),",
                  "Err(error) => editing::patch_refusal(&error, &self.patch),"))

for view, (enum, leaves, code, _) in IFC_VIEWS.items():
    arms = "\n".join(f"        | {enum}::{variant}({module}::{variant} {{ {field}: id, .. }})" if field != "id" else f"        | {enum}::{variant}({module}::{variant} {{ id, .. }})"
                     for variant, module, field in leaves).replace("        | ", "        ", 1)
    EDITS += [
        (IFC + view + "/🧬️schema/🧬️mutations/🦀️.rs",
         f'''fn rejected(message: String) -> protocol::MutationOutcome<Ifc2x3Diff> {{
    protocol::MutationOutcome::error("{code}", message, Vec::<String>::new())
}}''',
         f'''/// 🚫️ The report of an edit this view refuses — a `mutation.invariant` addressed at the instance the edit names (the
/// header for the view definition, the whole document for a snapshot).
fn rejected(this: &{enum}) -> protocol::MutationOutcome<Ifc2x3Diff> {{
    let target = match this {{
        {enum}::SetSnapshot(_) => Vec::new(),
        {enum}::SetViewDefinition(_) => vec!["header".to_string()],
{arms} => vec!["instances".to_string(), id.to_string()],
    }};
    protocol::MutationOutcome::error(protocol::MutationCode::Invariant, target)
}}'''),
        (IFC + view + "/🧬️schema/🧬️mutations/🦀️.rs", "Err(message) => rejected(message),", "Err(_) => rejected(this),"),
        (IFC + view + "/🧬️schema/🦀️.rs",
         '''message: if message.target.is_empty() { message.message.clone() } else { format!("{} at {}", message.message, message.target.join("/")) },''',
         '''message: if message.target.is_empty() { message.code.0.clone() } else { format!("{} at {}", message.code.0, message.target.join("/")) },'''),
    ]

for cc, rung in zip(CLASSES, (None, 3, 4, 5, 6, None)):
    number = cc[-1]
    mutations = STEP + cc + "/🧬️schema/🧬️mutations/🦀️.rs"
    EDITS += [
        (mutations, "Err(message) => rejected(message),", "Err(_) => rejected(edit),"),
        (mutations, f'''pub(crate) fn rejected(message: String) -> protocol::MutationOutcome<StepDiff> {{
    protocol::MutationOutcome::error("stdio.step.cc{number}.mutation-rejected", message, Vec::<String>::new())
}}''', '''pub(crate) fn rejected(edit: &ClassEdit) -> protocol::MutationOutcome<StepDiff> {
    protocol::MutationOutcome::error(protocol::MutationCode::Invariant, ladder::class_edit_target(edit))
}'''),
        (mutations, '''Some(message) => Err(format!("{:?} was rejected: [{}] {}", mutation, message.code.0, message.message)),''',
         '''Some(message) => Err(format!("{:?} was rejected: [{}] at {}", mutation, message.code.0, message.target.join("/"))),'''),
    ]
    tests = STEP + cc + "/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"
    if rung is not None:
        EDITS.append((tests, f'''    let message = &outcome.messages().first().expect("rung {rung} is above this class").message;
    assert!(message.contains("rung {rung}") && message.contains("ceiling of {rung - 1}"), "the refusal must name both rungs: {{message}}");''',
                      f'''    let message = outcome.messages().first().expect("rung {rung} is above this class");
    assert_eq!((message.code.0.as_str(), message.target.clone()), ("mutation.invariant", vec!["entities".to_string(), "13".to_string()]), "the class ceiling refuses the representation it would have written");'''))
EDITS.append((STEP + "6️⃣cc6/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs",
              '''    let message = &outcome.messages().first().expect("MANIFOLD_SOLID_BREP is a solid, not a representation").message;
    assert!(message.contains("not a *_SHAPE_REPRESENTATION type"), "the refusal must say what is wrong: {message}");''',
              '''    let message = outcome.messages().first().expect("MANIFOLD_SOLID_BREP is a solid, not a representation");
    assert_eq!((message.code.0.as_str(), message.target.clone()), ("mutation.invariant", vec!["entities".to_string(), "13".to_string()]), "an off-ladder type is refused on the representation it would have written");'''))

JSON_REFUSAL = re.compile(r"Err\(\(\s*(CODE_TARGET_MISSING|CODE_INVARIANT),")


def json_refusals(text: str) -> str:
    """🔁️ Every `Err((CODE_*, prose, target))` of the i-json lowering → `Err(fatal(<frozen variant>, target))`."""
    edits = []
    for match in JSON_REFUSAL.finditer(text):
        tuple_open = match.start() + len("Err(")
        spans, end = codemod.arguments(text, tuple_open)
        variant = "TargetMissing" if match.group(1) == "CODE_TARGET_MISSING" else "Invariant"
        target = text[spans[2][0]:spans[2][1]].strip()
        edits.append((match.start(), end + 1, f"Err(protocol::MutationOutcome::fatal(protocol::MutationCode::{variant}, {target}))"))
    for start, stop, new in sorted(edits, reverse=True):
        text = text[:start] + new + text[stop:]
    return text


def target_errors(text: str) -> str:
    """🎯️ Every `target_error(code, prose, target)` → `MutationApplyError::new(<frozen variant>).at(target)` (duplicate →
    DuplicateId, else Invariant); the helper, which only carried the prose, goes."""
    text = text.replace('''fn target_error(code: &'static str, message: &'static str, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code, message).at(target)''', '''fn target_error(code: MutationCode, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code).at(target)''')
    text = re.sub(r'target_error\("duplicate-[a-z-]+", "[^"]*", ', "target_error(MutationCode::DuplicateId, ", text)
    text = re.sub(r'target_error\("[a-z-]+", "[^"]*", ', "target_error(MutationCode::Invariant, ", text)
    text = re.sub(r"target_error\(MutationCode::(\w+), ", r"MutationApplyError::new(MutationCode::\1).at(", text)
    text = text.replace('''// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn target_error(code: MutationCode, target: Vec<String>) -> MutationApplyError {
    MutationApplyError::new(code).at(target)
}

''', "")
    return codemod.add_import(text)[0] if "MutationCode::" in text else text


def semio_children(text: str) -> str:
    """🪪️ A child identity the subset refuses is a bool question once the reason text is gone."""
    return re.sub(r"if let Err\(message\) = (crate::[\w:]+::validate_semio_child_identity\([^)]*\)) \{", r"if \1.is_err() {", text)


def gltf_rejections(text: str) -> str:
    """✂️ `rejection_outcome` callers: no detail argument."""
    return drop_argument(text, "rejection_outcome", 2, 3)


def gltf_files() -> list[str]:
    from pathlib import Path
    root = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-s20-overlay-faults-p2/✏️s/🔌️plugins")
    return sorted(str(path.relative_to(root)) for path in (root / GLTF).rglob("🦀️.rs") if "rejection_outcome(" in path.read_text())


TRANSFORMS = ([(JSONI, json_refusals)] + [(path, target_errors) for path in TARGET_ERROR] + [(path, semio_children) for path in SEMIO_CHILDREN]
              + [(path, gltf_rejections) for path in gltf_files()])

RENAMES = [(SEMIO + "📦️object", {"mutation.child-identity": "Invariant"})]
