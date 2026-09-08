//! 🔷️ Lowpoly artifact — the mesh + paint document projection, its patch types, and the ephemeral
//! selection value threaded into the compute session (never part of the persisted document).

extern crate semio_framework_value_derive as value_derive;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<LowpolyMutation, LowpolyConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
extern crate self as semio_s_artifact_lowpoly_lowpoly;

use protocol::{Identified, Patchable};
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;

//#region 🔖️Pixels
pub use crate::schema::mutations::LowpolyMutation;

pub use crate::schema::diff::LowpolyDiff;

pub const LOWPOLY_PAINT_TEXTURE_SIZE: usize = 1024;
pub const LOWPOLY_DOCUMENT_SCHEMA: &str = "lowpoly.document";

/// @emoji 🎨️ An opaque-white RGBA buffer sized for one paint layer.
pub fn empty_paint_pixels() -> Vec<u8> {
    let mut pixels = vec![0u8; LOWPOLY_PAINT_TEXTURE_SIZE * LOWPOLY_PAINT_TEXTURE_SIZE * 4];
    for chunk in pixels.chunks_mut(4) {
        chunk[0] = 255;
        chunk[1] = 255;
        chunk[2] = 255;
        chunk[3] = 255;
    }
    pixels
}

//#endregion 🔖️Pixels

//#region 🔖️Snapshot
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyTransform {
    #[dsl(coord)]
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

impl Default for LowpolyTransform {
    fn default() -> Self {
        Self { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], scale: [1.0, 1.0, 1.0] }
    }
}

/// @emoji 🖌️ One paint layer of an object: compositing metadata plus its persisted RGBA pixel buffer.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyPaintLayer {
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub blend_mode: String,
    #[value(default = "empty_paint_pixels")]
    #[dsl(base64)]
    pub pixels: Vec<u8>,
}

impl LowpolyPaintLayer {
    pub fn new(name: &str) -> Self {
        Self { name: name.into(), visible: true, opacity: 1.0, blend_mode: "normal".into(), pixels: empty_paint_pixels() }
    }
}

/// 🕸️ Deterministic content-addressed CHILD handle for an object's mesh — same `(child_id, target)`
/// for identical geometry, a different pair once the geometry actually changes. Content-addressing
/// (rather than a per-object-stable id) is what lets the parent's own diff/mutation machinery detect
/// "the mesh changed" from the HANDLE alone, without ever comparing embedded content — the handle
/// IS the change signal. Shared by `snapshot_from_mesh_json` and the app's kernel session
/// (`⚙️engine::LowpolyDocument::sync_meshes_to_snapshot`, `add_primitive`), which both need the
/// identical rule so the same geometry always resolves to the same handle.
pub fn mesh_child_handle(object_id: &str, mesh_json: &str) -> store::ArtifactChild<SemioMeshSnapshot> {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    mesh_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("mesh-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "mesh".into() };
    let target = store::os_io::ArtifactRef { artifact_id: format!("{object_id}-mesh"), dialect };
    store::ArtifactChild::new(child_id, target)
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyObject {
    pub id: String,
    pub name: String,
    pub transform: LowpolyTransform,
    pub smooth_shading: bool,
    /// 🕸️ Owned CHILD handle for this object's mesh representation (`s.stdio.semio.mesh`, ticket
    /// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`). The child is its own document; this parent
    /// stores only the two-string handle (`child_id`/`target`), never embedded geometry — replaces
    /// the old opaque `mesh_json: String` this field used to be. Lifecycle via `create-mesh`/
    /// `delete-mesh` (`🧬️mutations/🕸️create-mesh`, `🧬️mutations/🧨delete-mesh`), which replace the
    /// old whole-value `replace-object-mesh`. `LowpolySnapshot`'s hand-rolled `ArtifactDsl`/
    /// `ArtifactPack` below persist ONLY this handle for the mesh slot — the live half-edge-mesh
    /// JSON content itself never lives on this struct at all (see `✏️editor/🖌️session::LowpolyScratch`'s
    /// `mesh_workspace` field, the session-local cache this field's content moved to, round 2 of this
    /// ticket's fix — `LowpolySnapshot`'s hand-rolled codecs already asserted "never `mesh_workspace`"
    /// in this same doc comment before the field was removed; a struct-level round-trip law
    /// (`store::os_store::test_support::assert_document_text_round_trip`) forbids a codec-excluded
    /// field from living on a persisted snapshot type at all — see `📸️snapshot/🦀️.rs`'s
    /// module doc comment).
    ///
    /// ⚠️ Framework limitation (flagged in this ticket's `lowpoly-report.md` for the next fan-out
    /// agent hitting the same shape): `#[derive(ArtifactSchema)]`'s `#[child(kind = "…")]` mechanism
    /// (`🧬️schema/✨️derive/🦀️.rs`) only recognizes a child field declared DIRECTLY on the
    /// struct it derives — it does not recurse into a `Vec<T>` element type's own fields the way
    /// `LowpolySnapshot.objects: Vec<LowpolyObject>` would need. Every W2c precedent (`✳️object`,
    /// `✳️kit`) only ever put its child fields directly on the top-level snapshot struct, never
    /// nested inside a collection element — this is the first subset in the ticket with that shape.
    /// Consequently `#[child(...)]` cannot be applied here (no derive on `LowpolyObject` declares it
    /// as a valid helper attribute) and `LowpolySnapshot::child_slots()` cannot discover this slot.
    /// The type/mutation-vocabulary/persistence layer is still fully real; only the derive-generated
    /// SCHEMA INTROSPECTION table is incomplete for it.
    pub mesh: Option<store::ArtifactChild<SemioMeshSnapshot>>,
    #[value(default)]
    pub paint_layers: Vec<LowpolyPaintLayer>,
}

impl Identified<String> for LowpolyObject {
    fn id(&self) -> &String {
        &self.id
    }
}

/// @emoji 📸️ Persisted lowpoly snapshot: schema plus mesh objects (geometry, transform, shading,
/// paint layers). Non-persistent fields live on [`schema::LowpolyArtifact`](crate::schema::LowpolyArtifact).
pub use crate::schema::snapshot::LowpolySnapshot;

pub use crate::snapshot::schema::snapshot_from_mesh_json;

/// @emoji 🎯️ Ephemeral component selection — never part of the document, threaded into the compute
/// session so mesh operations know their target vertices/edges/faces.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolySelectionTargets {
    pub mesh: bool,
    pub vertex: bool,
    pub edge: bool,
    pub face: bool,
}

impl Default for LowpolySelectionTargets {
    fn default() -> Self {
        Self { mesh: true, vertex: false, edge: false, face: false }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolySelection {
    #[value(default)]
    pub targets: LowpolySelectionTargets,
    #[value(default)]
    pub keys: Vec<String>,
    pub mode: String,
    pub ids: Vec<u32>,
}

impl Default for LowpolySelection {
    fn default() -> Self {
        Self { targets: LowpolySelectionTargets::default(), keys: Vec::new(), mode: "mesh".into(), ids: Vec::new() }
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️Patches
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct LowpolyObjectPatch {
    pub name: Option<String>,
    pub smooth_shading: Option<bool>,
    pub transform: Option<LowpolyTransform>,
    /// 🕸️ Double-`Option`: outer = "this patch touches the `mesh` slot", inner = the new handle,
    /// `None` to clear it — matches `✳️object`'s own `mesh: Option<Option<ArtifactChild<…>>>` diff
    /// convention (`w2c-object-kit-report.md`). Never carries content, only `child_id`/`target`.
    /// Replaces the old `mesh_json: Option<String>` scalar patch field; `dsl::DslRecord` dropped
    /// from this struct (unused in practice — nothing calls its derived DSL machinery, confirmed by
    /// grep — and `store::ArtifactChild<S>` has no `DslField` impl to derive against anyway).
    pub mesh: Option<Option<store::ArtifactChild<SemioMeshSnapshot>>>,
}

impl Patchable<LowpolyObjectPatch> for LowpolyObject {
    fn apply_patch(&mut self, patch: &LowpolyObjectPatch) {
        if let Some(value) = &patch.name {
            self.name = value.clone();
        }
        if let Some(value) = patch.smooth_shading {
            self.smooth_shading = value;
        }
        if let Some(value) = &patch.transform {
            self.transform = value.clone();
        }
        if let Some(value) = &patch.mesh {
            self.mesh = value.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<LowpolyObjectPatch> {
        let patch = LowpolyObjectPatch {
            name: (self.name != other.name).then(|| other.name.clone()),
            smooth_shading: (self.smooth_shading != other.smooth_shading).then_some(other.smooth_shading),
            transform: (self.transform != other.transform).then(|| other.transform.clone()),
            mesh: (self.mesh != other.mesh).then(|| other.mesh.clone()),
        };
        (patch != LowpolyObjectPatch::default()).then_some(patch)
    }
}

/// 🖌️ Applies a paint-layers sub-delta onto one object.
pub fn apply_paint_layers_delta(object: &mut LowpolyObject, delta: &crate::diff::schema::LowpolyPaintLayersDelta) -> protocol::MutationApplyResult<()> {
    let mut layers = object.paint_layers.clone();
    let mut removed = std::collections::BTreeSet::new();
    for (position, index) in delta.removed.iter().copied().enumerate() {
        if !removed.insert(index) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "paint layer is removed more than once").at(["removed".to_string(), position.to_string()]));
        }
        if index as usize >= layers.len() {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "removed paint layer index is out of range").at(["removed".to_string(), position.to_string()]));
        }
    }
    let mut removed_indices: Vec<_> = delta.removed.iter().map(|index| *index as usize).collect();
    removed_indices.sort_unstable_by(|left, right| right.cmp(left));
    for i in removed_indices {
        layers.remove(i);
    }
    for (position, entry) in delta.added.iter().enumerate() {
        let i = entry.index as usize;
        if i > layers.len() {
            return Err(protocol::MutationApplyError::new("mutation.apply.invalid-index", "added paint layer index is out of range").at(["added".to_string(), position.to_string()]));
        }
        layers.insert(i, entry.layer.clone());
    }
    for (position, entry) in delta.patched.iter().enumerate() {
        let i = entry.index as usize;
        let layer = layers.get_mut(i).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.invalid-index", "patched paint layer index is out of range").at(["patched".to_string(), position.to_string()]))?;
        let p = &entry.patch;
        if let Some(value) = &p.name {
            layer.name = value.clone();
        }
        if let Some(value) = p.visible {
            layer.visible = value;
        }
        if let Some(value) = p.opacity {
            layer.opacity = value;
        }
        if let Some(value) = &p.blend_mode {
            layer.blend_mode = value.clone();
        }
    }
    for (position, stroke) in delta.strokes.iter().enumerate() {
        let i = stroke.layer_index as usize;
        let layer = layers.get_mut(i).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.invalid-index", "paint stroke layer index is out of range").at(["strokes".to_string(), position.to_string(), "layerIndex".to_string()]))?;
        for (run_index, run) in stroke.runs.iter().enumerate() {
            let start = run.offset as usize;
            let end = start
                .checked_add(run.bytes.len())
                .filter(|end| *end <= layer.pixels.len())
                .ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.invalid-index", "paint stroke byte range is out of bounds").at(["strokes".to_string(), position.to_string(), "runs".to_string(), run_index.to_string()]))?;
            layer.pixels[start..end].copy_from_slice(&run.bytes);
        }
    }
    object.paint_layers = layers;
    Ok(())
}
//#endregion 🔖️Patches

//#region 🔖️Dialect
/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (`✏️editor`/`👁️viewer`) under this artifact's `✳️any` subset shares — lives at the
/// ARTIFACT level (not under either surface) specifically so `👁️viewer/🦀️.rs` can read it
/// without ever importing through the sibling editor module.
pub const LOWPOLY_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.lowpoly.lowpoly", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🧱️ The two artifact kinds this plugin contributes — lifted out of the old ui crate's manifest
/// builder chain so the app's `🔖️Manifest` region can stitch it in as a single passthrough.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    semio_framework_plugin::ArtifactKindSpec {
        id: "3d.lowpoly".into(),
        name: "3D Lowpoly".into(),
        source_format: "lowpoly.fixture".into(),
        component_kind: "lowpoly".into(),
        dimension: "3d".into(),
        media_capability: semio_framework_plugin::OsMediaCapability::MeshOnly,
        media_type: semio_framework_plugin::MediaType { class: semio_framework_plugin::MediaClass::ThreeD, form: semio_framework_plugin::MediaForm::Mesh },
        schema: "lowpoly.fixture".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
        import_stdio_kinds: vec!["stdio.dwg".into(), "stdio.gltf".into(), "stdio.json".into(), "stdio.las".into(), "stdio.obj".into(), "stdio.ply".into(), "stdio.png".into(), "stdio.stl".into()],
    }
}

// 🧱️ `mesh_artifact_kind()` (the shared `3d.mesh` interchange kind) REMOVED — ticket
// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`. `3d.mesh` is a duplicate kind id being deleted
// repo-wide: mesh is now canonically `s.stdio.semio@v1/mesh` (composed via `LowpolyObject.mesh:
// Option<store::ArtifactChild<SemioMeshSnapshot>>` above, never a standalone `ArtifactKindSpec`).
// gis's `🏔️gisterrain` declared the identical id independently — that removal belongs to whichever
// agent owns gis, not this one. Registration call site removed from
// `✏️editor/🦀️.rs`'s `create_lowpoly_app()` (`.artifact_kind(mesh_artifact_kind())`).
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from a
/// plugin `.setup()` callback. Mesh/DWG format handling for `"3d.lowpoly"` flows entirely through
/// `.composers(...)` below (the typed `standards::v1::subsets::any::io::io_registry::entries()` table)
/// — the former `register_mesh_exporter`/`register_mesh_importer`/`register_mesh_dwg_*`
/// calls were never carried into this file; they would have duplicated this composer registration
/// rather than adding anything. Relocated from
/// `⚙️engine/🦀️.rs` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g3): `⚙️engine`
/// was removed from the taxonomy and `declaration()` describes the artifact, not engine behaviour, so
/// its home is the artifact root alongside `artifact_kind()`.
/// `crate::apps::lowpoly::config::schema::register_app_schema()` is the one exception, still called
/// from `💠️lowpoly/🦀️.rs`'s own `.setup()`: it registers the `LowpolyPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.lowpoly.lowpoly.standard.v1", "standard", "1", &[], None),
        ("s.lowpoly.lowpoly.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.lowpoly.lowpoly.schema.artifact", "schema", "s.lowpoly.lowpoly", &[("schema", "s.lowpoly.lowpoly")], None),
        ("s.lowpoly.lowpoly.inference.artifact", "inference", "s.lowpoly.lowpoly.inference", &[("schema", "s.lowpoly.lowpoly.inference")], None),
        ("s.lowpoly.lowpoly.composer.native", "composer", "s.lowpoly.lowpoly@1/*", &[("dialect", "s.lowpoly.lowpoly@1/*")], None),
        ("s.lowpoly.lowpoly.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-2", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.lowpoly.lowpoly.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.lowpoly.lowpoly.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.lowpoly.lowpoly.composer.format-6", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.lowpoly.lowpoly.composer.format-7", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-8", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.lowpoly.lowpoly.composer.format-9", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.lowpoly.lowpoly.grammar.1", "grammar", "lowpoly.document", &[("grammar", "lowpoly.document")], None),
        ("s.lowpoly.lowpoly.grammar.2", "grammar", "lowpoly.op", &[("grammar", "lowpoly.op")], None),
        ("s.lowpoly.lowpoly.grammar.3", "grammar", "lowpoly.diff", &[("grammar", "lowpoly.diff")], None),
        ("s.lowpoly.lowpoly.grammar.4", "grammar", "lowpoly.pack", &[("grammar", "lowpoly.pack")], None),
        ("s.lowpoly.lowpoly.grammar.5", "grammar", "lowpoly.spr", &[("grammar", "lowpoly.spr")], None),
        ("s.lowpoly.lowpoly.codec.document-1", "codec", "lowpoly.document:lowpoly", &[("codec", "lowpoly.document"), ("codec-extension", "16:lowpoly.document:lowpoly")], None),
        ("s.lowpoly.lowpoly.localization.en", "localization", "Lowpoly", &[], Some(("en", "Lowpoly"))),
        ("s.lowpoly.lowpoly.localization.de", "localization", "Niedrigpolygon", &[], Some(("de", "Niedrigpolygon"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.lowpoly.lowpoly")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::schema::lowpoly_artifact_schema_descriptor())
        .inferences([crate::standards::v1::subsets::any::schema::inferences::lowpoly_artifact_inference_descriptor()])
        .composers(crate::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::lowpoly::LowpolyPlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "lowpoly.document",
                    extension: Some("lowpoly"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::document_dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::document_dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("lowpoly.document"),
                },
                dsl::LanguageSpec {
                    id: "lowpoly.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("lowpoly.op"),
                },
                dsl::LanguageSpec {
                    id: "lowpoly.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("lowpoly.diff"),
                },
                dsl::LanguageSpec {
                    id: "lowpoly.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("lowpoly.pack"),
                },
                dsl::LanguageSpec {
                    id: "lowpoly.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("lowpoly.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn object_patch_apply_mutates_and_inverse_restores_all_fields() {
        let mesh_workspace = "{}".to_string();
        let original_mesh = mesh_child_handle("obj-1", &mesh_workspace);
        let mut object = LowpolyObject { id: "obj-1".into(), name: "Original".into(), transform: LowpolyTransform::default(), smooth_shading: false, mesh: Some(original_mesh), paint_layers: vec![LowpolyPaintLayer::new("Base")] };
        let original = object.clone();
        let new_mesh_workspace = "{\"changed\":true}".to_string();
        let new_mesh = mesh_child_handle("obj-1", &new_mesh_workspace);
        let patch = LowpolyObjectPatch { name: Some("Renamed".into()), smooth_shading: Some(true), transform: Some(LowpolyTransform { position: [1.0, 2.0, 3.0], ..LowpolyTransform::default() }), mesh: Some(Some(new_mesh.clone())) };
        object.apply_patch(&patch);
        assert_eq!(object.name, "Renamed");
        assert!(object.smooth_shading);
        assert_eq!(object.transform.position, [1.0, 2.0, 3.0]);
        assert_eq!(object.mesh, Some(new_mesh));
        let inverse = object.diff_patch(&original).expect("patch changed state");
        object.apply_patch(&inverse);
        assert_eq!(object, original);
    }

    #[semio_framework_async_macros::async_test]
    async fn snapshot_from_mesh_json_builds_single_object_with_base_layer() {
        let mesh_json = "{}".to_string();
        let snapshot = snapshot_from_mesh_json(&mesh_json, "obj-42", "Widget");
        assert_eq!(snapshot.schema, LOWPOLY_DOCUMENT_SCHEMA);
        assert_eq!(snapshot.objects.len(), 1);
        assert_eq!(snapshot.objects[0].id, "obj-42");
        assert_eq!(snapshot.objects[0].name, "Widget");
        assert_eq!(snapshot.objects[0].mesh, Some(mesh_child_handle("obj-42", &mesh_json)));
        assert_eq!(snapshot.objects[0].paint_layers.len(), 1);
        assert_eq!(snapshot.objects[0].paint_layers[0].name, "Base");
    }

    #[semio_framework_async_macros::async_test]
    async fn lowpoly_selection_defaults_target_whole_mesh() {
        let targets = LowpolySelectionTargets::default();
        assert!(targets.mesh);
        assert!(!targets.vertex && !targets.edge && !targets.face);
        let selection = LowpolySelection::default();
        assert_eq!(selection.mode, "mesh");
        assert!(selection.ids.is_empty());
    }
}

#[cfg(test)]
#[semio_framework_async_macros::async_test]
async fn artifact_schema_descriptor_leaves_parse_and_field_states_match_snapshot_json() {
    use schema::{parse_state_class_kebab, ArtifactSchemaFields};
    let descriptor = crate::schema::lowpoly_artifact_schema_descriptor();
    assert_eq!(descriptor.id, "s.lowpoly.lowpoly");
    let schema: dsl::os_pack::json::Value = dsl::os_pack::json::from_json_str(descriptor.snapshot.json_schema).expect("snapshot json");
    assert_eq!(schema["title"], "LowpolySnapshot");
    let properties = schema["properties"].as_object().expect("properties");
    let mut json_states: Vec<(String, _)> = properties
        .iter()
        .map(|(name, prop)| {
            let raw = prop["x-semio-state"].as_str().expect("state");
            (name.to_string(), parse_state_class_kebab(raw).expect("parse"))
        })
        .collect();
    json_states.sort_by(|a, b| a.0.cmp(&b.0));
    let mut derived: Vec<(String, _)> = LowpolySnapshot::field_states().await.iter().map(|(n, c)| ((*n).to_string(), *c)).collect();
    derived.sort_by(|a, b| a.0.cmp(&b.0));
    assert_eq!(derived, json_states);
    assert_eq!(crate::schema::LowpolyArtifact::artifact_schema_id().await, "s.lowpoly.lowpoly");
}
//#endregion 🧪️Tests

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "."]
                        pub mod schema {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod bounds {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📦bounds/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;

                                #[path = "."]
                                pub mod create_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱️create-object/🧪️tests/⛵️inserts-obj-mast-c60a92/🦀️.rs"]
                                    mod tests_inserts_obj_mast_between_hull_and_fin;
                                }
                                #[path = "."]
                                pub mod delete_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💀️delete-object/🧪️tests/🚫️removes-obj-fin-b0cb61/🦀️.rs"]
                                    mod tests_removes_obj_fin_without_touching_the_order;
                                }
                                #[path = "."]
                                pub mod reorder_objects {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀️reorder-objects/🧪️tests/🔀️moves-obj-fin-in-cfb501/🦀️.rs"]
                                    mod tests_moves_obj_fin_in_front_of_obj_hull;
                                }
                                #[path = "."]
                                pub mod rename_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-object/🧪️tests/🏷️retitles-obj-hull/🦀️.rs"]
                                    mod tests_retitles_obj_hull;
                                }
                                #[path = "."]
                                pub mod change_object_smooth_shading {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔘️change-object-smooth-shading/🧪️tests/🟢️turns-on-smooth-e6be9e/🦀️.rs"]
                                    mod tests_turns_on_smooth_shading_for_obj_hull;
                                }
                                #[path = "."]
                                pub mod move_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/↗️move-object/🧪️tests/📍️translates-obj-bb774a/🦀️.rs"]
                                    mod tests_translates_obj_hull_along_x_and_z;
                                }
                                #[path = "."]
                                pub mod rotate_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔄️rotate-object/🧪️tests/🔄️yaws-obj-hull-3de70a/🦀️.rs"]
                                    mod tests_yaws_obj_hull_about_the_y_axis;
                                }
                                #[path = "."]
                                pub mod scale_object {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️scale-object/🧪️tests/📐️halves-obj-hull-f4206f/🦀️.rs"]
                                    mod tests_halves_obj_hull_uniformly;
                                }
                                #[path = "."]
                                pub mod create_mesh {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🧪️tests/🕸️attaches-a-mesh-226325/🦀️.rs"]
                                    mod tests_attaches_a_mesh_child_handle_to_obj_fin;
                                }
                                #[path = "."]
                                pub mod delete_mesh {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧨delete-mesh/🧪️tests/✂️detaches-the-mesh-67db32/🦀️.rs"]
                                    mod tests_detaches_the_mesh_child_handle_from_obj_hull;
                                }
                                #[path = "."]
                                pub mod insert_paint_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➕️insert-paint-layer/🧪️tests/🪜️stacks-a-detail-8006c2/🦀️.rs"]
                                    mod tests_stacks_a_detail_layer_above_the_base_layer;
                                }
                                #[path = "."]
                                pub mod remove_paint_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/➖️remove-paint-layer/🧪️tests/➖️drops-the-detail-630aca/🦀️.rs"]
                                    mod tests_drops_the_detail_layer_at_index_1;
                                }
                                #[path = "."]
                                pub mod rename_paint_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔖️rename-paint-layer/🧪️tests/🏷️retitles-the-b8f732/🦀️.rs"]
                                    mod tests_retitles_the_base_layer_to_undercoat;
                                }
                                #[path = "."]
                                pub mod change_paint_layer_visible {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/👁️change-paint-layer-visible/🧪️tests/🙈️hides-the-base-b24cf7/🦀️.rs"]
                                    mod tests_hides_the_base_layer;
                                }
                                #[path = "."]
                                pub mod change_paint_layer_opacity {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌫️change-paint-layer-opacity/🧪️tests/🌫️fades-the-base-3cee87/🦀️.rs"]
                                    mod tests_fades_the_base_layer_to_half;
                                }
                                #[path = "."]
                                pub mod change_paint_layer_blend_mode {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎛️change-paint-layer-blend-mode/🧪️tests/✖️switches-the-base-152ae9/🦀️.rs"]
                                    mod tests_switches_the_base_layer_to_multiply;
                                }
                                #[path = "."]
                                pub mod edit_paint_layer {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️edit-paint-layer/🧪️tests/🖌️paints-red-over-389630/🦀️.rs"]
                                    mod tests_paints_red_over_the_second_half_of_the_base_layer;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            #[path = "."]
                            pub mod export {
                                #[path = "."]
                                pub mod serializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod las {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/☁️las/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod ply {
                                            #[path = "."]
                                            pub mod v1_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧱️ply/🔖️1.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod txt {
                                            #[path = "."]
                                            pub mod v_utf_8 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod png {
                                            #[path = "."]
                                            pub mod v1_2 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📷️png/🔖️1.2/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod json {
                                            #[path = "."]
                                            pub mod v_rfc8259 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod dwg {
                                            #[path = "."]
                                            pub mod v_ac1018 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🖊️dwg/🔖️ac1018/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod stl {
                                            #[path = "."]
                                            pub mod v_ascii {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔺️stl/🔖️ascii/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod gltf {
                                            #[path = "."]
                                            pub mod v2_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod obj {
                                            #[path = "."]
                                            pub mod v3_0 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🗿️obj/🔖️3.0/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // ---- Shims: keep pre-migration module paths resolving for external callers ----
        pub mod schema {
            pub use super::standards::v1::subsets::any::schema::*;
        }
        pub mod io {
            pub use super::standards::v1::subsets::any::io::*;
        }
        pub mod op {
            pub use crate::standards::v1::subsets::any::schema::mutations::text::*;
        }
        pub mod document_dsl {
            pub use crate::standards::v1::subsets::any::schema::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::schema::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::schema::diff::text::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
        }
        pub mod snapshot {
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::schema::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod lowpoly {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs"]
            pub mod schema;
        }

        #[path = "."]
        pub mod presence {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🦀️.rs"]
            mod component;
            pub use component::*;

            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/👥️presence/🧬️schema/🦀️.rs"]
            pub mod schema;
        }
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
        pub mod engine;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🖌️session/🦀️.rs"]
        pub mod session;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧭️view/🦀️.rs"]
        pub mod view;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/➕️add-primitive/🦀️.rs"]
            pub mod add_primitive;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️camera/🦀️.rs"]
            pub mod camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️chrome/🦀️.rs"]
            pub mod chrome;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💬️engagement/🦀️.rs"]
            pub mod engagement;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧫️fixture/🦀️.rs"]
            pub mod fixture;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔷️mesh-edit/🦀️.rs"]
            pub mod mesh_edit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖌️paint/🦀️.rs"]
            pub mod paint;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✏️patch-object/🦀️.rs"]
            pub mod patch_object;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗂️selection/🦀️.rs"]
            pub mod selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌞️sun/🦀️.rs"]
            pub mod sun;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧲️transform/🦀️.rs"]
            pub mod transform;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️utility/🦀️.rs"]
            pub mod utility;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧵️uv/🦀️.rs"]
            pub mod uv;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️model/🦀️.rs"]
                    pub mod model;
                }
            }

            #[path = "."]
            pub mod paint {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🎨️paint/🪟️windows/🖼️uv/🦀️.rs"]
                    pub mod uv;
                }
            }
        }

        #[path = "."]
        pub mod options {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🖌️paint-params-brush/🦀️.rs"]
            pub mod paint_params_brush;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🧽️paint-params-eraser/🦀️.rs"]
            pub mod paint_params_eraser;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🗂️select/🦀️.rs"]
            pub mod select;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/👁️show-edges/🦀️.rs"]
            pub mod show_edges;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🧲️snap/🦀️.rs"]
            pub mod snap;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🛠️options/🌞️sun/🦀️.rs"]
            pub mod sun;
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod document;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗂️layers/🦀️.rs"]
            pub mod layers;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod lowpoly {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🌐️model/🦀️.rs"]
                    pub mod model;
                }
            }
        }
    }
}
