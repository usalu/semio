//! 🎞️ Animate presentation artifact — document entities + `ArtifactKindSpec` (constitutional: general).

// 🌀️ R7 — `fn` in public traits (`Sobject`, `Animation`) is deliberate under O1's universal-async
// ruling; callers cannot assume `Send` from the lint's suggested fix, and R3 answers that structurally
// (every dyn seam here is a `dyn_enum_close!`-generated enum, so `Send` falls out of the concrete
// variant types). Never resolved by `+ Send` on the trait method or by making the method sync.
#![allow(async_fn_in_trait)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<PresentationMutation, PresentationConfigMutation>, Fault>`, the exact signature
// `ArtifactApp::handle` and `app_commands!`'s generated `dispatch` require. `Fault` is a
// framework-owned error type; boxing it here would diverge from the trait it must satisfy, and the
// lint does not fire on the trait impl itself (only on the free functions the taxonomy split creates),
// so this is a pure artefact of decomposition.
#[allow(clippy::result_large_err)]
extern crate self as semio_s_artifact_animate_presentation;

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::schema::mutations::PresentationMutation;

pub use crate::schema::diff::PresentationDiff;

pub const PRESENTATION_DOCUMENT_SCHEMA: &str = "animate.presentation";
pub use crate::snapshot::schema::{default_snapshot, PresentationSnapshot};

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 — the one canonical
/// `(artifact_kind, standard, subset)` coordinate shared by BOTH `✏️editor::animate::AnimatePresentationPlayApp`
/// and `👁️viewer::animate::AnimatePresentationViewer`. Lives at the ARTIFACT level (not under either surface
/// module) specifically so the viewer can read it without ever importing through the editor. Matches
/// `definition()`'s own `s.presentation.schema.artifact` capability descriptor (`s.animate.presentation`) and this
/// file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface id `s.animate.presentation@1/*#editor`
/// / `s.animate.presentation@1/*#viewer`.
pub const ANIMATE_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.animate.presentation", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

//#region 🔖️Domain
/// 📐️ Normalized `x,y,width,height` rect — always reached through a `#[dsl(block)]` field (see
/// {@link FigureTileSource}/{@link FigureTileDraft}), so it declares no `#[dsl(keyword)]` of its own.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    #[dsl(block)]
    pub frame: FigureTileFrame,
    #[value(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[value(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    #[dsl(block)]
    pub crop: FigureTileFrame,
}
//#endregion 🔖️Domain

pub fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource { src: "/🖼️bauteilbörse.png".into(), kind: "figure".into(), frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 }, source_aspect: Some(1222.0 / 896.0), pdf_page: None }
}

pub fn default_presentation_snapshot() -> PresentationSnapshot {
    default_snapshot()
}

//#region 🔖️PresentationBridge
/// 🕸️ Owned CHILD handle types for the composed `s.stdio.semio.presentation`/`s.stdio.semio.animation`
/// documents — ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`animate→C:presentation,animation`):
/// the shared source figure + its named tile crops now live in the composed `presentation` child's
/// slide-deck structure instead of inline `source`/`tiles` fields on `PresentationSnapshot`. `animation`
/// is composed too, per the design mapping's `animate→C:presentation,animation` line, but carries no
/// content today — this artifact's persisted document has no time-based data at all (the Manim-class
/// scene/keyframe engine under `✏️editor/⚙️engine` constructs its scenes in Rust code at
/// render/export time, never from persisted document state — see `animation_child_handle`'s own doc
/// comment for the honest gap this leaves).
pub type PresentationChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot>;
pub type AnimationChild = store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot>;

/// 🪪️ Fixed target-identity roots for the two singleton composed children (this artifact only ever
/// has exactly one presentation deck and one animation set, never a collection of either).
const PRESENTATION_CHILD_ARTIFACT_ID: &str = "animate-presentation-deck-presentation";
const ANIMATION_CHILD_ARTIFACT_ID: &str = "animate-presentation-deck-animation";

/// 🌉 REAL bidirectional converter (forward half): `(source, tiles)` -> one composed
/// `SemioPresentationSnapshot`. `source` becomes the deck's single `SlideMaster` (id `"source"`, one
/// `Picture` shape spanning `source.frame`); each tile becomes its own `Slide` referencing that
/// master's image, with the tile's own `crop` as its `Picture` shape's frame and the tile's `name`
/// carried as the slide's own `notes` (the closest lossless slot presentation offers a per-slide
/// display string). `source.kind` is reused verbatim as `SlidePictureImage.mime` — not a real MIME
/// type, but presentation's `Picture` shape has no dedicated "kind" tag of its own, and reusing the
/// nearest string slot losslessly beats inventing a MIME taxonomy that would only need an unmapping
/// table right back. **Lossy**: `source.source_aspect`/`source.pdf_page` have no representable slot
/// in `presentation`'s schema at all and are dropped by this forward conversion — every in-process
/// mutation round-trip still preserves them exactly via the working-scene cache below (never routed
/// through this lossy projection), so this only matters for a genuinely fresh reload with an empty
/// cache, the same class of documented gap every `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` exemplar
/// (lowpoly/cad/writer) has left for its own composed slot.
pub fn presentation_snapshot_from_source_tiles(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideMaster, SlidePictureImage, SlideShape, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

    const SOURCE_MASTER_ID: &str = "source";
    let frame_of = |frame: &FigureTileFrame| SlideFrame { origin: SemioPoint2 { x: frame.x, y: frame.y }, width: frame.width, height: frame.height };
    let image_of = || SlidePictureImage { asset_id: source.src.clone(), mime: source.kind.clone(), bytes: Vec::new() };

    let master = SlideMaster { id: SOURCE_MASTER_ID.into(), shapes: vec![SlideShape::Picture { frame: frame_of(&source.frame), image: image_of() }] };
    let slides = tiles.iter().map(|tile| Slide { id: tile.id.clone(), layout_id: None, shapes: vec![SlideShape::Picture { frame: frame_of(&tile.crop), image: image_of() }], notes: vec![DocBlock::paragraph(tile.name.clone())] }).collect();
    SemioPresentationSnapshot { schema: STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA.into(), masters: vec![master], layouts: Vec::new(), slides }
}

/// 🌉 Inverse of [`presentation_snapshot_from_source_tiles`] — reads the `"source"` master's first
/// `Picture` shape back into a `FigureTileSource` (honestly `source_aspect: None`, `pdf_page: None` —
/// see the forward converter's doc comment) and each `Slide`'s first `Picture` shape + its `notes`'s
/// first paragraph text back into a `FigureTileDraft`. A master/slide with no `Picture` shape at all
/// (never produced by the forward converter, but a composed child can in principle arrive from
/// elsewhere) falls back to `default_figure_tile_source()`/an empty name rather than panicking.
pub fn source_tiles_from_presentation_snapshot(snapshot: &semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot) -> (FigureTileSource, Vec<FigureTileDraft>) {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
    use semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::SlideShape;

    fn frame_from(shapes: &[SlideShape]) -> Option<(FigureTileFrame, String, String)> {
        shapes.iter().find_map(|shape| match shape {
            SlideShape::Picture { frame, image } => Some((FigureTileFrame { x: frame.origin.x, y: frame.origin.y, width: frame.width, height: frame.height }, image.asset_id.clone(), image.mime.clone())),
            _ => None,
        })
    }
    fn text_from_notes(notes: &[DocBlock]) -> String {
        notes
            .iter()
            .find_map(|block| match block {
                DocBlock::Paragraph { runs, .. } => Some(runs.iter().map(|run| run.text.as_str()).collect::<String>()),
                _ => None,
            })
            .unwrap_or_default()
    }

    let source = snapshot.masters.first().and_then(|master| frame_from(&master.shapes)).map_or_else(default_figure_tile_source, |(frame, src, kind)| FigureTileSource { src, kind, frame, source_aspect: None, pdf_page: None });

    let tiles = snapshot
        .slides
        .iter()
        .map(|slide| {
            let crop = frame_from(&slide.shapes).map_or(FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }, |(frame, ..)| frame);
            FigureTileDraft { id: slide.id.clone(), name: text_from_notes(&slide.notes), crop }
        })
        .collect();
    (source, tiles)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the composed `presentation` deck — same
/// `(child_id, target)` for identical `(source, tiles)`, a different pair once the content actually
/// changes, mirroring lowpoly's `mesh_child_handle`/writer's `document_child_handle`.
pub fn presentation_child_handle(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationChild {
    use std::hash::{Hash, Hasher};
    let content = presentation_snapshot_from_source_tiles(source, tiles);
    let content_json = dsl::os_pack::json::to_json_string(&content);
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("presentation-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "presentation".into() };
    let target = store::os_io::ArtifactRef { artifact_id: PRESENTATION_CHILD_ARTIFACT_ID.into(), dialect };
    store::ArtifactChild::new(child_id, target)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the composed `animation` set. Always the SAME
/// handle today (content is always the empty default `SemioAnimationSnapshot`) — honest reflection of
/// the fact that nothing in this plugin yet produces per-tile keyframe/timeline data; composed per the
/// design mapping's `animate→C:presentation,animation` line so the slot exists for a future wave (a
/// natural extension: per-tile camera-pan/transition timing) without another schema migration.
pub fn animation_child_handle() -> AnimationChild {
    use semio_s_artifact_stdio_semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
    let content_json = dsl::os_pack::json::to_json_string(&SemioAnimationSnapshot::default());
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    let content_hash = hasher.finish();
    let child_id = format!("animation-{content_hash:016x}");
    let dialect = store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "animation".into() };
    let target = store::os_io::ArtifactRef { artifact_id: ANIMATION_CHILD_ARTIFACT_ID.into(), dialect };
    store::ArtifactChild::new(child_id, target)
}
//#endregion 🔖️PresentationBridge

//#region 🔖️WorkingScene
// 🌱 Ephemeral, process-side working representation of the composed `presentation` child's live
// `(source, tiles)` content, keyed by `PresentationChild::child_id` and shared across executor
// threads.
type PresentationScratch = std::collections::HashMap<String, (FigureTileSource, Vec<FigureTileDraft>)>;

static PRESENTATION_SCRATCH: std::sync::OnceLock<std::sync::RwLock<PresentationScratch>> = std::sync::OnceLock::new();

fn presentation_scratch() -> &'static std::sync::RwLock<PresentationScratch> {
    PRESENTATION_SCRATCH.get_or_init(|| std::sync::RwLock::new(PresentationScratch::new()))
}

/// 📝 Seeds the scratch cache for a handle's `child_id` — call whenever new `(source, tiles)` content
/// is about to become a document's `presentation` child (every mutation-diff/fixture builder in this
/// plugin does, via [`presentation_child_handle_and_cache`]).
pub fn cache_presentation_working_scene(child_id: &str, source: &FigureTileSource, tiles: &[FigureTileDraft]) {
    presentation_scratch().write().unwrap_or_else(std::sync::PoisonError::into_inner).insert(child_id.to_string(), (source.clone(), tiles.to_vec()));
}

/// 🔎 Reads the cached live `(source, tiles)` for a `presentation` child handle — falls back to
/// `source_tiles_from_presentation_snapshot`'s best-effort (lossy) reconstruction is NOT attempted
/// here (no live child content is reachable from this pure accessor either — see the region doc
/// comment); falls back to `default_figure_tile_source()`/no tiles, never a panic, when nothing has
/// cached this handle yet.
pub fn presentation_working_scene_for_handle(handle: &PresentationChild) -> (FigureTileSource, Vec<FigureTileDraft>) {
    presentation_scratch().read().unwrap_or_else(std::sync::PoisonError::into_inner).get(&handle.child_id).cloned().unwrap_or_else(|| (default_figure_tile_source(), Vec::new()))
}

/// 🔎 Reads the current document's live `(source, tiles)` off its `presentation` child handle — the
/// single read call site every mutation/render/export/inference path in this plugin uses instead of
/// the old `snapshot.source`/`snapshot.tiles` field access.
pub fn presentation_working_scene(snapshot: &PresentationSnapshot) -> (FigureTileSource, Vec<FigureTileDraft>) {
    presentation_working_scene_for_handle(&snapshot.presentation)
}

/// 🏗️ Mints a new content-addressed `presentation` handle AND seeds the scratch cache with its
/// `(source, tiles)` in one call — the standard way every mutation-diff/fixture builder in this
/// plugin creates a `presentation` field value; never construct a handle without also caching, or
/// [`presentation_working_scene`] will read back the empty default.
pub fn presentation_child_handle_and_cache(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationChild {
    let handle = presentation_child_handle(source, tiles);
    cache_presentation_working_scene(&handle.child_id, source, tiles);
    handle
}

/// 🏗️ Builds a full `PresentationSnapshot` from literal `(source, tiles)` — the standard fixture/import
/// constructor replacing the old 3-field `PresentationSnapshot { schema, source, tiles }` struct literal
/// now that `presentation`/`animation` are composed child handles, not plain fields.
pub fn presentation_snapshot_with_tiles(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationSnapshot {
    PresentationSnapshot { schema: PRESENTATION_DOCUMENT_SCHEMA.into(), presentation: presentation_child_handle_and_cache(source, tiles), animation: animation_child_handle() }
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::presentation::create_animate_presentation_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: PRESENTATION_DOCUMENT_SCHEMA.into(),
        name: "Animate Presentation".into(),
        source_format: PRESENTATION_DOCUMENT_SCHEMA.into(),
        component_kind: "panel".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Presentation, form: MediaForm::Deck },
        schema: PRESENTATION_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.md".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.pptx".into(), "stdio.svg".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.md".into(), "stdio.pdf".into(), "stdio.png".into(), "stdio.pptx".into(), "stdio.svg".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport
impl Identified<String> for FigureTileDraft {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    fn apply_patch(&mut self, patch: &FigureTileDraftPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
    }

    fn diff_patch(&self, other: &Self) -> Option<FigureTileDraftPatch> {
        Some(FigureTileDraftPatch { name: (self.name != other.name).then(|| other.name.clone()), crop: (self.crop != other.crop).then(|| other.crop.clone()) })
    }
}
//#endregion 🔖️CollectionSupport

//#region 🧪️Tests
#[cfg(test)]
#[allow(clippy::items_after_test_module)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.animate.presentation")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.animate.presentation")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.animate.presentation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.animate.presentation.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.animate.presentation.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.animate.presentation@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.animate.presentation@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.pptx")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pptx@ecma-376/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pptx@ecma-376/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"animate.presentation:presentation")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "animate.presentation")?)?
                .claim(ArtifactIdentityClaim::codec_extension("animate.presentation", "presentation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Animate Presentation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Animate Presentation")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.animate.presentation.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Animate Presentation")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Animate Presentation")?)?,
        )
}

/// 🗿️ New declaration-tree root (design.md §1/§2 recipe step 6) — replaces the OLD `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).document_codec(...)`
/// chain) outright, no dual channel (mirrors `🎬️sequence`'s identical atomic cutover). `kind` matches
/// `ANIMATE_DIALECT.artifact_kind` / `PresentationSnapshot`'s own `#[artifact_schema(id = ...)]`
/// (`"s.animate.presentation"`), NOT `definition()`'s legacy `ArtifactIdentity` root (`"s.presentation"`,
/// kept unread by the new tree per debt D1). `localization: &[]` is a documented shortfall — the
/// real en/de localized names still live on `definition()`'s kept capability rows.
pub fn artifact<PA>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::animate::AnimatePresentationPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::animate::AnimatePresentationViewer>>>,
{
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.animate.presentation").expect("canonical animate.presentation kind"), localization: &[], standards: vec![crate::standards::v1::standard::<PA>()] }
}
//#endregion 🔖️Declaration

#[path = "."]
        pub mod standards {
            #[path = "."]
            pub mod v1 {
                #[path = "🏅️standards/🔖️1/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "."]
                pub mod subsets {
                    #[path = "."]
                    pub mod any {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod topology {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧭topology/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                }
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "."]
                                pub mod resize_source_frame {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔲resize-source-frame/🧪️tests/📖️no-ops-when-the-457983/🦀️.rs"]
                                    mod tests_no_ops_when_the_frame_is_already_identical;
                                }
                                #[path = "."]
                                pub mod replace_source {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️replace-source/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️replace-source/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️replace-source/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️replace-source/🧪️tests/📖️no-ops-when-the-fc0000/🦀️.rs"]
                                    mod tests_no_ops_when_the_source_is_already_identical;
                                }
                                #[path = "."]
                                pub mod create_tile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🆕create-tile/🧪️tests/🚫️rejects-a-f79685/🦀️.rs"]
                                    mod tests_rejects_a_duplicate_tile_id;
                                }
                                #[path = "."]
                                pub mod delete_tile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-tile/🧪️tests/🚫️rejects-deleting-e5f4c5/🦀️.rs"]
                                    mod tests_rejects_deleting_a_missing_tile;
                                }
                                #[path = "."]
                                pub mod delete_tiles {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧹delete-tiles/🧪️tests/🚫️rejects-when-96c380/🦀️.rs"]
                                    mod tests_rejects_when_every_addressed_tile_is_missing;
                                }
                                #[path = "."]
                                pub mod rename_tile {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-tile/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-tile/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-tile/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✏️rename-tile/🧪️tests/📖️no-ops-when-the-1d4320/🦀️.rs"]
                                    mod tests_no_ops_when_the_tile_already_has_that_name;
                                }
                                #[path = "."]
                                pub mod resize_tile_crop {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️resize-tile-crop/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️resize-tile-crop/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️resize-tile-crop/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/✂️resize-tile-crop/🧪️tests/🚫️rejects-a-zero-fe5a5d/🦀️.rs"]
                                    mod tests_rejects_a_zero_width_crop;
                                }
                                #[path = "."]
                                pub mod reorder_tiles {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔀reorder-tiles/🧪️tests/🚪️no-ops-when-the-82057e/🦀️.rs"]
                                    mod tests_no_ops_when_the_tile_is_already_at_that_index;
                                }
                                #[path = "."]
                                pub mod replace_tiles {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔁replace-tiles/🧪️tests/📖️no-ops-when-the-a5dcbf/🦀️.rs"]
                                    mod tests_no_ops_when_the_collection_is_already_empty;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "."]
                            pub mod snapshot {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod diff {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/📝️text/🦀️.rs"]
                                pub mod text;
                                pub use text::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🔺️diff/💾️binary/🦀️.rs"]
                                pub mod binary;
                            }
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod inferences {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💡️inferences/📝️text/🦀️.rs"]
                                pub mod text;
                            }
                            #[path = "."]
                            pub mod import {
                                #[path = "."]
                                pub mod deserializers {
                                    #[path = "."]
                                    pub mod artifacts {
                                        #[path = "."]
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
                                        pub mod pptx {
                                            #[path = "."]
                                            pub mod v_ecma_376 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎞️pptx/🔖️ecma-376/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod svg {
                                            #[path = "."]
                                            pub mod v1_1 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs"]
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
                                        pub mod pdf {
                                            #[path = "."]
                                            pub mod v1_4 {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.4/✳️any/🦀️.rs"]
                                                    mod component;
                                                    pub use component::*;
                                                }
                                            }
                                        }
                                        #[path = "."]
                                        pub mod md {
                                            #[path = "."]
                                            pub mod v_commonmark {
                                                #[path = "."]
                                                pub mod any {
                                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📝️md/🔖️commonmark/✳️any/🦀️.rs"]
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
            pub use crate::standards::v1::subsets::any::io::mutations::text::*;
        }
        pub mod dsl {
            pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
        }
        pub mod spr {
            pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
        }
        pub mod diff {
            pub use crate::standards::v1::subsets::any::io::diff::text::*;
            pub use crate::standards::v1::subsets::any::schema::diff::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::diff::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::diff::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::diff::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::diff::binary::*;
            }
        }
        pub mod mutations {
            pub use crate::standards::v1::subsets::any::schema::mutations::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::mutations::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::mutations::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::mutations::binary::*;
            }
        }
        pub mod snapshot {
            pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            pub mod schema {
                pub use crate::standards::v1::subsets::any::schema::snapshot::*;
            }
            pub mod text {
                pub use crate::standards::v1::subsets::any::io::snapshot::text::*;
            }
            pub mod pack {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
            pub mod binary {
                pub use crate::standards::v1::subsets::any::io::snapshot::binary::*;
            }
        }

        #[path = "."]
        pub mod examples {
            #[path = "."]
            pub mod demo {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                mod component;
                pub use component::*;
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
                mod tests;
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod animate {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod engine {
            //! ⚙️ This app's own stateful behaviour (compiler/🎞️slide/video-export at the root, plus the
            //! Manim-class animation core and headless video renderer as sibling `<topic>/🦀️.rs`
            //! files) — a non-taxonomy, editor-only facet (ticket
            //! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET packet recipe step 4: only editor-side
            //! files reference it, so it moved wholesale into `✏️editor/⚙️engine/`).
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🦀️.rs"]
            mod component;
            pub use component::*;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎞️animation/🦀️.rs"]
            pub mod animation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📷️camera/🦀️.rs"]
            pub mod camera;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎛️config/🦀️.rs"]
            pub mod config;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📐️geometry/🦀️.rs"]
            pub mod geometry;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/⏱️rate/🦀️.rs"]
            pub mod rate;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🦀️.rs"]
            pub mod scene;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔤️text/🦀️.rs"]
            pub mod text;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎥️video/🦀️.rs"]
            pub mod video;
        }

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
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs"]
        pub mod terminology;
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌉️wasm/🦀️.rs"]
        pub mod wasm;

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🀄️add-tile/🦀️.rs"]
            pub mod add_tile;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👇️canvas-pointer-down/🦀️.rs"]
            pub mod canvas_pointer_down;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧹️clear-tiles/🦀️.rs"]
            pub mod clear_tiles;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📋️copy-prompt/🦀️.rs"]
            pub mod copy_prompt;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚮️delete-selection/🦀️.rs"]
            pub mod delete_selection;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-tile/🦀️.rs"]
            pub mod delete_tile;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⌨️engagement-input/🦀️.rs"]
            pub mod engagement_input;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📤️engagement-submit/🦀️.rs"]
            pub mod engagement_submit;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎥️export-video-from-deck/🦀️.rs"]
            pub mod export_video_from_deck;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/⏸️no-operation/🦀️.rs"]
            pub mod no_operation;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/✂️patch-tile-crops/🦀️.rs"]
            pub mod patch_tile_crops;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-tiles/🦀️.rs"]
            pub mod rename_tiles;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/↩️reset-grid/🦀️.rs"]
            pub mod reset_grid;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱️seed-grid/🦀️.rs"]
            pub mod seed_grid;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs"]
            pub mod set_active_example;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🖼️set-frame/🦀️.rs"]
            pub mod set_frame;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📥️set-source/🦀️.rs"]
            pub mod set_source;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod main {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🖊️main/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🖊️main/🪟️windows/🖼️tile-editor/🦀️.rs"]
                    pub mod tile_editor;
                }
            }
        }

        #[path = "."]
        pub mod panels {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs"]
            pub mod artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🦀️.rs"]
            pub mod catalogue;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs"]
            pub mod inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod animate {
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️tile-editor/🦀️.rs"]
                    pub mod tile_editor;
                }
            }
        }
    }
}

//#region 📚️Examples
pub use standards::v1::subsets::any::examples;
//#endregion 📚️Examples
