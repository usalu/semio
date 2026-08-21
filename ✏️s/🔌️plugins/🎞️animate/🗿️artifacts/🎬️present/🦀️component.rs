//! 🎞️ Animate present artifact — document entities + `ArtifactKindSpec` (constitutional: general).

use protocol::{Identified, Patchable};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

pub use crate::artifacts::present::schema::mutations::PresentMutation;

pub use crate::artifacts::present::schema::diff::PresentDiff;

pub const PRESENT_DOCUMENT_SCHEMA: &str = "animate.present";
pub use crate::artifacts::present::snapshot::schema::{default_snapshot, PresentSnapshot};

/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1 — the one canonical
/// `(artifact_kind, standard, subset)` coordinate shared by BOTH `✏️editor::animate::AnimatePresentPlayApp`
/// and `👁️viewer::animate::AnimatePresentViewer`. Lives at the ARTIFACT level (not under either surface
/// module) specifically so the viewer can read it without ever importing through the editor. Matches
/// `definition()`'s own `s.present.schema.artifact` capability descriptor (`s.animate.present`) and this
/// file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — canonical surface id `s.animate.present@1/*#editor`
/// / `s.animate.present@1/*#viewer`.
pub const ANIMATE_DIALECT: semio_framework_plugin::Dialect = semio_framework_plugin::Dialect { artifact_kind: "s.animate.present", standard: semio_framework_plugin::StandardId("1"), subset: semio_framework_plugin::SubsetId::ANY };

//#region 🔖️Domain
/// 📐️ Normalized `x,y,width,height` rect — always reached through a `#[dsl(block)]` field (see
/// {@link FigureTileSource}/{@link FigureTileDraft}), so it declares no `#[dsl(keyword)]` of its own.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileSource {
    pub src: String,
    pub kind: String,
    #[dsl(block)]
    pub frame: FigureTileFrame,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_aspect: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdf_page: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraft {
    pub id: String,
    pub name: String,
    #[dsl(block)]
    pub crop: FigureTileFrame,
}
//#endregion 🔖️Domain

pub async fn default_figure_tile_source() -> FigureTileSource {
    FigureTileSource { src: "/🖼️bauteilbörse.png".into(), kind: "figure".into(), frame: FigureTileFrame { x: 0.127, y: 0.1, width: 0.746, height: 0.75 }, source_aspect: Some(1222.0 / 896.0), pdf_page: None }
}

pub async fn default_present_snapshot() -> PresentSnapshot {
    default_snapshot()
}

//#region 🔖️PresentationBridge
/// 🕸️ Owned CHILD handle types for the composed `s.stdio.semio.presentation`/`s.stdio.semio.animation`
/// documents — ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`animate→C:presentation,animation`):
/// the shared source figure + its named tile crops now live in the composed `presentation` child's
/// slide-deck structure instead of inline `source`/`tiles` fields on `PresentSnapshot`. `animation`
/// is composed too, per the design mapping's `animate→C:presentation,animation` line, but carries no
/// content today — this artifact's persisted document has no time-based data at all (the Manim-class
/// scene/keyframe engine under `✏️editor/⚙️engine` constructs its scenes in Rust code at
/// render/export time, never from persisted document state — see `animation_child_handle`'s own doc
/// comment for the honest gap this leaves).
pub type PresentationChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot>;
pub type AnimationChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot>;

/// 🪪️ Fixed target-identity roots for the two singleton composed children (this artifact only ever
/// has exactly one presentation deck and one animation set, never a collection of either).
const PRESENTATION_CHILD_ARTIFACT_ID: &str = "animate-present-deck-presentation";
const ANIMATION_CHILD_ARTIFACT_ID: &str = "animate-present-deck-animation";

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
pub async fn presentation_snapshot_from_source_tiles(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot, Slide, SlideFrame, SlideMaster, SlidePictureImage, SlideShape, STDIO_SEMIOPRESENTATION_DOCUMENT_SCHEMA};

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
pub async fn source_tiles_from_presentation_snapshot(snapshot: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot) -> (FigureTileSource, Vec<FigureTileDraft>) {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::DocBlock;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::presentation::schema::snapshot::SlideShape;

    async fn frame_from(shapes: &[SlideShape]) -> Option<(FigureTileFrame, String, String)> {
        shapes.iter().find_map(|shape| match shape {
            SlideShape::Picture { frame, image } => Some((FigureTileFrame { x: frame.origin.x, y: frame.origin.y, width: frame.width, height: frame.height }, image.asset_id.clone(), image.mime.clone())),
            _ => None,
        })
    }
    async fn text_from_notes(notes: &[DocBlock]) -> String {
        notes
            .iter()
            .find_map(|block| match block {
                DocBlock::Paragraph { runs, .. } => Some(runs.iter().map(|run| run.text.as_str()).collect::<String>()),
                _ => None,
            })
            .unwrap_or_default()
    }

    let source = snapshot.masters.first().and_then(|master| frame_from(&master.shapes)).map(|(frame, src, kind)| FigureTileSource { src, kind, frame, source_aspect: None, pdf_page: None }).unwrap_or_else(default_figure_tile_source);

    let tiles = snapshot
        .slides
        .iter()
        .map(|slide| {
            let crop = frame_from(&slide.shapes).map(|(frame, ..)| frame).unwrap_or(FigureTileFrame { x: 0.0, y: 0.0, width: 1.0, height: 1.0 });
            FigureTileDraft { id: slide.id.clone(), name: text_from_notes(&slide.notes), crop }
        })
        .collect();
    (source, tiles)
}

/// 🕸️ Deterministic content-addressed CHILD handle for the composed `presentation` deck — same
/// `(child_id, target)` for identical `(source, tiles)`, a different pair once the content actually
/// changes, mirroring lowpoly's `mesh_child_handle`/writer's `document_child_handle`.
pub async fn presentation_child_handle(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationChild {
    use std::hash::{Hash, Hasher};
    let content = presentation_snapshot_from_source_tiles(source, tiles);
    let content_json = serde_json::to_string(&content).unwrap_or_default();
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
pub async fn animation_child_handle() -> AnimationChild {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot;
    let content_json = serde_json::to_string(&SemioAnimationSnapshot::default()).unwrap_or_default();
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
/// 🌱 Ephemeral, process-side working representation of the composed `presentation` child's live
/// `(source, tiles)` content — NEVER persisted, NEVER a durable field on `PresentSnapshot` itself
/// (matches the `EngineRep` contract: wholly derived, droppable at any instant, rebuilt from base).
/// Exists because no `LinkResolver`/child-dispatch seam was wired into this plugin's mutation
/// diff/inverse path at authoring time — `protocol::MutationKind::diff`/`inverse` only ever receive
/// `base: &PresentSnapshot`, never an `ArtifactView`/`ChildContentView` (that seam, confirmed real and
/// live-wired as of 2026-08-13 in `🔌️plugin/🦀️component.rs`'s `ArtifactView::with_children`, is
/// reachable from `ArtifactApp::handle`/`render` — the APP layer — but ArtifactView is never plumbed
/// into the pure `MutationKind` trait itself). Mirrors lowpoly's `LowpolyScratch.mesh_workspace` /
/// writer's `WRITER_SCRATCH`, keyed by `PresentationChild::child_id`.
///
/// ⚠️ Same documented staleness gap as every prior exemplar: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, so a handle can in principle go uncached (a fresh process, or an
/// undo past this session's history). `present_working_scene`/`present_working_scene_for_handle` fail
/// SOFT (the default empty source/no tiles) rather than panicking — see this region's callers.
thread_local! {
    static PRESENT_SCRATCH: std::cell::RefCell<std::collections::HashMap<String, (FigureTileSource, Vec<FigureTileDraft>)>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 📝 Seeds the scratch cache for a handle's `child_id` — call whenever new `(source, tiles)` content
/// is about to become a document's `presentation` child (every mutation-diff/fixture builder in this
/// plugin does, via [`presentation_child_handle_and_cache`]).
pub async fn cache_present_working_scene(child_id: &str, source: &FigureTileSource, tiles: &[FigureTileDraft]) {
    PRESENT_SCRATCH.with(|cache| cache.borrow_mut().insert(child_id.to_string(), (source.clone(), tiles.to_vec())));
}

/// 🔎 Reads the cached live `(source, tiles)` for a `presentation` child handle — falls back to
/// `source_tiles_from_presentation_snapshot`'s best-effort (lossy) reconstruction is NOT attempted
/// here (no live child content is reachable from this pure accessor either — see the region doc
/// comment); falls back to `default_figure_tile_source()`/no tiles, never a panic, when nothing has
/// cached this handle yet.
pub async fn present_working_scene_for_handle(handle: &PresentationChild) -> (FigureTileSource, Vec<FigureTileDraft>) {
    PRESENT_SCRATCH.with(|cache| cache.borrow().get(&handle.child_id).cloned()).unwrap_or_else(|| (default_figure_tile_source(), Vec::new()))
}

/// 🔎 Reads the current document's live `(source, tiles)` off its `presentation` child handle — the
/// single read call site every mutation/render/export/inference path in this plugin uses instead of
/// the old `snapshot.source`/`snapshot.tiles` field access.
pub async fn present_working_scene(snapshot: &PresentSnapshot) -> (FigureTileSource, Vec<FigureTileDraft>) {
    present_working_scene_for_handle(&snapshot.presentation)
}

/// 🏗️ Mints a new content-addressed `presentation` handle AND seeds the scratch cache with its
/// `(source, tiles)` in one call — the standard way every mutation-diff/fixture builder in this
/// plugin creates a `presentation` field value; never construct a handle without also caching, or
/// [`present_working_scene`] will read back the empty default.
pub async fn presentation_child_handle_and_cache(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentationChild {
    let handle = presentation_child_handle(source, tiles);
    cache_present_working_scene(&handle.child_id, source, tiles);
    handle
}

/// 🏗️ Builds a full `PresentSnapshot` from literal `(source, tiles)` — the standard fixture/import
/// constructor replacing the old 3-field `PresentSnapshot { schema, source, tiles }` struct literal
/// now that `presentation`/`animation` are composed child handles, not plain fields.
pub async fn present_snapshot_with_tiles(source: &FigureTileSource, tiles: &[FigureTileDraft]) -> PresentSnapshot {
    PresentSnapshot { schema: PRESENT_DOCUMENT_SCHEMA.into(), presentation: presentation_child_handle_and_cache(source, tiles), animation: animation_child_handle() }
}
//#endregion 🔖️WorkingScene

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::present::create_animate_present_app`'s `🔖️Manifest` region.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: PRESENT_DOCUMENT_SCHEMA.into(),
        name: "Animate Present".into(),
        source_format: PRESENT_DOCUMENT_SCHEMA.into(),
        component_kind: "panel".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Presentation, form: MediaForm::Deck },
        schema: PRESENT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg"],
        import_stdio_kinds: vec!["stdio.json", "stdio.md", "stdio.pdf", "stdio.png", "stdio.pptx", "stdio.svg"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️CollectionSupport
impl Identified<String> for FigureTileDraft {
    async fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
pub struct FigureTileDraftPatch {
    pub name: Option<String>,
    #[dsl(block)]
    pub crop: Option<FigureTileFrame>,
}

impl Patchable<FigureTileDraftPatch> for FigureTileDraft {
    async fn apply_patch(&mut self, patch: &FigureTileDraftPatch) {
        if let Some(name) = &patch.name {
            self.name = name.clone();
        }
        if let Some(crop) = &patch.crop {
            self.crop = crop.clone();
        }
    }

    async fn diff_patch(&self, other: &Self) -> Option<FigureTileDraftPatch> {
        Some(FigureTileDraftPatch { name: (self.name != other.name).then(|| other.name.clone()), crop: (self.crop != other.crop).then(|| other.crop.clone()) })
    }
}
//#endregion 🔖️CollectionSupport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn present_snapshot_schema_is_animate_present() {
        assert_eq!(default_present_snapshot().schema, PRESENT_DOCUMENT_SCHEMA);
    }

    #[semio_framework_async_macros::async_test]
    async fn artifact_kind_matches_the_store_schema() {
        assert_eq!(artifact_kind().schema, PRESENT_DOCUMENT_SCHEMA);
        assert_eq!(artifact_kind().id, PRESENT_DOCUMENT_SCHEMA);
    }
}
//#endregion 🧪️Tests
//#region 🔖️Declaration
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.present")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.schema.artifact")?, ArtifactCapabilityKind::schema())
                .descriptor(b"s.animate.present")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.animate.present")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.animate.present.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.animate.present.inference")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.present@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.present@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.pptx")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pptx@ecma-376/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pptx@ecma-376/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.svg")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.svg@1.1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.svg@1.1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.pdf")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.pdf@1.4/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.pdf@1.4/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.md")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.md@commonmark/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.md@commonmark/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.png")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.png@1.2/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.png@1.2/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"animate.present:present")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "animate.present")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "present")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.localization.en")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Animate Present")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Animate Present")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.present.localization.de")?, ArtifactCapabilityKind::localization())
                .descriptor(b"Animate Present")?
                .localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Animate Present")?)?,
        )
}

/// 🗿️ New declaration-tree root (design.md §1/§2 recipe step 6) — replaces the OLD `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).document_codec(...)`
/// chain) outright, no dual channel (mirrors `🎬️sequence`'s identical atomic cutover). `kind` matches
/// `ANIMATE_DIALECT.artifact_kind` / `PresentSnapshot`'s own `#[artifact_schema(id = ...)]`
/// (`"s.animate.present"`), NOT `definition()`'s legacy `ArtifactIdentity` root (`"s.present"`,
/// kept unread by the new tree per debt D1). `localization: &[]` is a documented shortfall — the
/// real en/de localized names still live on `definition()`'s kept capability rows.
pub async fn artifact() -> semio_framework_plugin::app::declarations::ArtifactDeclaration {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.animate.present").expect("canonical animate.present kind"), localization: &[], standards: vec![crate::artifacts::present::standards::v1::standard()] }
}
//#endregion 🔖️Declaration
