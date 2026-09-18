//! 🖼️ WFC bitmap artifact — the classic OVERLAPPING model: an authored input bitmap is learned as a
//! universe of `N × N` patterns, and an output bitmap is collapsed so that every one of its `N × N`
//! windows is a pattern the input actually contains. The document persists only the PROBLEM (input
//! sample and palette, output extent and periodicity, model parameters, pins, seed); the collapsed
//! bitmap, the contradiction verdict and the entropy map are an INFERENCE
//! (`🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`), never persisted state.
//!
//! The solver itself is not here: it is the shared `semio-s-plugin-wfc-engine` crate every artifact
//! of this plugin calls. This crate is a `🧬️schema` plus a `🚪️io` system plus two surfaces — never
//! an engine.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub use crate::schema::snapshot::WFC_BITMAP_DOCUMENT_SCHEMA;

use crate::schema::snapshot::{BitmapColor, BitmapPinnedPixel};
use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ The canonical surface-id coordinate for this artifact's ONE subset — `s.wfc.bitmap@1/*`.
/// Lives at the ARTIFACT root (not under `✏️editor`/`👁️viewer`) so a viewer file can read it without
/// importing through the sibling mutation-capable surface. `artifact_kind` matches
/// `BitmapSnapshot`/`BitmapDiff`'s own `#[artifact_schema(id = "s.wfc.bitmap")]` attribute and
/// `WFC_BITMAP_DOCUMENT_SCHEMA`'s literal value; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location on disk.
pub const WFC_BITMAP_DIALECT: Dialect = Dialect { artifact_kind: WFC_BITMAP_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`. A bitmap problem is genuinely two-dimensional raster media
/// — its input IS an image and its inferred output is another — so `dimension`/`media_class`/
/// `media_form` say `2d`/`TwoD`/`Raster` rather than the headless `data` shape a pure rule spec
/// would take.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "2d.wfcbitmap".into(),
        name: "WFC Bitmap".into(),
        source_format: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
        component_kind: "wfcbitmap".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster },
        schema: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.txt".into(), "stdio.json".into()],
        import_stdio_kinds: vec!["stdio.txt".into(), "stdio.json".into()],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🎨️CanvasLayers
/// 🚚️ The byte budget one window's `layersJson` may occupy. `Canvas2dScene` keeps `layersJson` in
/// the 32 KiB fixed-capacity surface spine — it has no lane of its own, unlike a paint-2d document
/// sync — so a bitmap whose run-merged rectangles would not fit is rendered at a coarser block size
/// instead of silently failing the whole surface encode.
pub const BITMAP_LAYERS_JSON_BUDGET_BYTES: usize = 24 * 1024;

/// 🎨️ Builds one canvas-2d layer list for a palette-indexed bitmap: a background rectangle for the
/// whole extent, one filled path per horizontal run of equal indices, and one outlined marker per
/// pinned cell. Shared by both surfaces from the ARTIFACT root so the read-only viewer never has to
/// import through the editor to draw the same pixels.
///
/// An empty `indices` slice draws the extent and the pins only — the honest rendering of an output
/// nobody has collapsed yet, and the reason a window is never structurally empty.
pub fn bitmap_layers_json(prefix: &str, width: u32, height: u32, palette: &[BitmapColor], indices: &[u8], pins: &[BitmapPinnedPixel]) -> String {
    let mut block = 1u32;
    loop {
        let layers = bitmap_layers_at_block(prefix, width, height, palette, indices, pins, block);
        let text = pack::json_to_string(&pack::json_array(layers));
        if text.len() <= BITMAP_LAYERS_JSON_BUDGET_BYTES || block >= width.max(height).max(1) {
            return text;
        }
        block *= 2;
    }
}

fn bitmap_rect_layer(id: &str, x: f64, y: f64, w: f64, h: f64, color: [f64; 4]) -> pack::JsonValue {
    let segments = pack::json_array(vec![
        pack::json_object([("kind".to_string(), pack::JsonValue::from("move")), ("to".to_string(), pack::json_array(vec![pack::JsonValue::from(x), pack::JsonValue::from(y)]))]),
        pack::json_object([("kind".to_string(), pack::JsonValue::from("line")), ("to".to_string(), pack::json_array(vec![pack::JsonValue::from(x + w), pack::JsonValue::from(y)]))]),
        pack::json_object([("kind".to_string(), pack::JsonValue::from("line")), ("to".to_string(), pack::json_array(vec![pack::JsonValue::from(x + w), pack::JsonValue::from(y + h)]))]),
        pack::json_object([("kind".to_string(), pack::JsonValue::from("line")), ("to".to_string(), pack::json_array(vec![pack::JsonValue::from(x), pack::JsonValue::from(y + h)]))]),
        pack::json_object([("kind".to_string(), pack::JsonValue::from("close"))]),
    ]);
    let fill = pack::json_object([
        ("kind".to_string(), pack::JsonValue::from("solid")),
        ("color".to_string(), pack::json_array(color.iter().map(|channel| pack::JsonValue::from(*channel)).collect::<Vec<_>>())),
    ]);
    pack::json_object([("id".to_string(), pack::JsonValue::from(id)), ("kind".to_string(), pack::JsonValue::from("path")), ("segments".to_string(), segments), ("fill".to_string(), fill)])
}

fn bitmap_layers_at_block(prefix: &str, width: u32, height: u32, palette: &[BitmapColor], indices: &[u8], pins: &[BitmapPinnedPixel], block: u32) -> Vec<pack::JsonValue> {
    let mut layers = vec![bitmap_rect_layer(&format!("{prefix}-extent"), 0.0, 0.0, f64::from(width), f64::from(height), [0.09, 0.10, 0.12, 1.0])];
    let cells = (width as usize) * (height as usize);
    if indices.len() == cells {
        for y in (0..height).step_by(block as usize) {
            let mut x = 0u32;
            while x < width {
                let index = indices[(y * width + x) as usize];
                let mut run = block.min(width - x);
                while x + run < width && indices[(y * width + x + run) as usize] == index {
                    run += block.min(width - x - run);
                }
                if let Some(color) = palette.get(usize::from(index)) {
                    layers.push(bitmap_rect_layer(&format!("{prefix}-{x}-{y}"), f64::from(x), f64::from(y), f64::from(run), f64::from(block.min(height - y)), color.to_unit_rgba()));
                }
                x += run;
            }
        }
    }
    for pin in pins {
        if pin.x < width && pin.y < height {
            let color = palette.get(pin.color as usize).map_or([1.0, 1.0, 1.0, 1.0], |color| color.to_unit_rgba());
            layers.push(bitmap_rect_layer(&format!("{prefix}-pin-{}-{}", pin.x, pin.y), f64::from(pin.x) + 0.2, f64::from(pin.y) + 0.2, 0.6, 0.6, color));
        }
    }
    layers
}
//#endregion 🎨️CanvasLayers

//#region 🔖️Declaration
/// 🧾️ Defines `s.wfc.bitmap`'s immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.wfc.bitmap.standard.v1", "standard", "1", &[], None),
        ("s.wfc.bitmap.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.wfc.bitmap.schema.artifact", "schema", "s.wfc.bitmap", &[("schema", "s.wfc.bitmap")], None),
        ("s.wfc.bitmap.inference.artifact", "inference", "s.wfc.bitmap.solve", &[("schema", "s.wfc.bitmap.solve")], None),
        ("s.wfc.bitmap.composer.native", "composer", "s.wfc.bitmap@1/*", &[("dialect", "s.wfc.bitmap@1/*")], None),
        ("s.wfc.bitmap.composer.format-1", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.wfc.bitmap.composer.format-2", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.wfc.bitmap.grammar.1", "grammar", "wfc.bitmap", &[("grammar", "wfc.bitmap")], None),
        ("s.wfc.bitmap.grammar.2", "grammar", "wfc.bitmap.op", &[("grammar", "wfc.bitmap.op")], None),
        ("s.wfc.bitmap.codec.document-1", "codec", "s.wfc.bitmap:wfcbitmap", &[("codec", "s.wfc.bitmap"), ("codec-extension", "12:s.wfc.bitmap:wfcbitmap")], None),
        ("s.wfc.bitmap.localization.en", "localization", "Bitmap", &[], Some(("en", "Bitmap"))),
        ("s.wfc.bitmap.localization.de", "localization", "Bitmap", &[], Some(("de", "Bitmap"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.wfc.bitmap")?);
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

/// 🎭️ The two app bindings every declaration of this artifact must be able to absorb.
#[cfg(feature = "component-app-assembly")]
pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::bitmap::BitmapEditor>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::bitmap::BitmapViewer>>>
{
}

#[cfg(feature = "component-app-assembly")]
impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<editor::bitmap::BitmapEditor>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<viewer::bitmap::BitmapViewer>>>
{
}

/// 🌳️ This artifact's declaration tree root — the ONLY registration channel for schema/io/viewer/
/// editor rows. `definition()` above is the separate capability-row channel.
#[cfg(feature = "component-app-assembly")]
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use store::os_io::ArtifactKindId;
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.wfc.bitmap").expect("canonical wfc bitmap kind"), localization: &[], standards: vec![standards::v1::standard::<PA>()] }
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`. The index order
/// `[document, op, pack, spr]` is the one `io()` reads its `NativeCodecs` slots from.
pub fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "wfc.bitmap",
                    extension: Some("wfcbitmap"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.bitmap"),
                },
                dsl::LanguageSpec {
                    id: "wfc.bitmap.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::schema::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.bitmap.op"),
                },
                dsl::LanguageSpec {
                    id: "wfc.bitmap.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.bitmap.pack"),
                },
                dsl::LanguageSpec {
                    id: "wfc.bitmap.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::schema::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("wfc.bitmap.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[cfg(feature = "component-app-assembly")]
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod component;
        #[cfg(feature = "component-app-assembly")]
        pub use component::*;

        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[cfg(feature = "component-app-assembly")]
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod component;
                #[cfg(feature = "component-app-assembly")]
                pub use component::*;

                #[path = "."]
                pub mod schema {
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                    mod component;
                    pub use component::*;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                    pub mod diff;
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
                    pub mod mutations {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                        pub mod binary;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                        pub mod text;
                        #[path = "."]
                        pub mod change_seed {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎲️change-seed/🧪️tests/🎲️reseeds-the-solve-from-7-to-99/🦀️.rs"]
                            mod tests_reseeds_the_solve_from_7_to_99;
                        }
                        #[path = "."]
                        pub mod resize_input {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-input/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-input/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-input/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📐️resize-input/🧪️tests/📐️grows-the-sample-to-6-by-4/🦀️.rs"]
                            mod tests_grows_the_sample_to_6_by_4;
                        }
                        #[path = "."]
                        pub mod set_input_pixels {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-input-pixels/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-input-pixels/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-input-pixels/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️set-input-pixels/🧪️tests/🖌️paints-a-2-by-2-block-of-colour-1/🦀️.rs"]
                            mod tests_paints_a_2_by_2_block_of_colour_1;
                        }
                        #[path = "."]
                        pub mod add_palette_color {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️add-palette-color/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️add-palette-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️add-palette-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🎨️add-palette-color/🧪️tests/🎨️appends-a-third-colour/🦀️.rs"]
                            mod tests_appends_a_third_colour;
                        }
                        #[path = "."]
                        pub mod change_palette_color {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖍️change-palette-color/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖍️change-palette-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖍️change-palette-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖍️change-palette-color/🧪️tests/🖍️recolours-the-second-entry/🦀️.rs"]
                            mod tests_recolours_the_second_entry;
                        }
                        #[path = "."]
                        pub mod remove_palette_color {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️remove-palette-color/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️remove-palette-color/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️remove-palette-color/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧽️remove-palette-color/🧪️tests/🧽️drops-the-unused-third-colour/🦀️.rs"]
                            mod tests_drops_the_unused_third_colour;
                        }
                        #[path = "."]
                        pub mod resize_output {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️resize-output/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️resize-output/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️resize-output/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖼️resize-output/🧪️tests/🖼️shrinks-the-output-and-cascades-a-pin/🦀️.rs"]
                            mod tests_shrinks_the_output_and_cascades_a_pin;
                        }
                        #[path = "."]
                        pub mod change_model {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️change-model/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️change-model/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️change-model/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/⚙️change-model/🧪️tests/⚙️widens-the-window-to-three/🦀️.rs"]
                            mod tests_widens_the_window_to_three;
                        }
                        #[path = "."]
                        pub mod pin_pixel {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-pixel/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-pixel/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-pixel/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📌️pin-pixel/🧪️tests/📌️pins-the-origin-cell-to-colour-1/🦀️.rs"]
                            mod tests_pins_the_origin_cell_to_colour_1;
                        }
                        #[path = "."]
                        pub mod unpin_pixel {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-pixel/🦀️.rs"]
                            mod component;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-pixel/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-pixel/↩️inverse/🦀️.rs"]
                            pub mod inverse;
                            pub use component::*;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📍️unpin-pixel/🧪️tests/📍️releases-the-pinned-origin-cell/🦀️.rs"]
                            mod tests_releases_the_pinned_origin_cell;
                        }
                    }
                    #[path = "."]
                    pub mod inferences {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs"]
                        mod component;
                        pub use component::*;
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
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                        pub mod any;
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                        pub mod any;
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
                                pub mod txt {
                                    #[path = "."]
                                    pub mod v_utf_8 {
                                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs"]
                                        pub mod any;
                                    }
                                }
                                #[path = "."]
                                pub mod json {
                                    #[path = "."]
                                    pub mod v_rfc8259 {
                                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️.rs"]
                                        pub mod any;
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

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod bitmap {
        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod transient {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🦀️.rs"]
            mod component;
            pub use component::*;
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
                    #[path = "."]
                    pub mod input {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🖼️input/🎚️config/🦀️.rs"]
                        pub mod config;
                    }
                    #[path = "."]
                    pub mod output {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🦀️.rs"]
                        mod component;
                        pub use component::*;
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧩️output/🎚️config/🦀️.rs"]
                        pub mod config;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod bitmap {
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
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🖼️input/🦀️.rs"]
                    pub mod input;
                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧩️output/🦀️.rs"]
                    pub mod output;
                }
            }
        }
    }
}

//#region 📚️Examples
/// 📚️ The bundled bitmap problems this subset ships — one compact three-colour plan, one taller
/// four-colour meadow with a ground constraint. Each slug owns its own directory under
/// `🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/`, and the Rust builder there is the authority its
/// `🗣️.dsl.semio` asset is printed from.
#[path = "."]
pub mod examples {
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🚪️rooms-16/🦀️.rs"]
    pub mod rooms_16;
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🌸️flowers-24/🦀️.rs"]
    pub mod flowers_24;

    /// 📇️ Every bundled example, in the order the editor's example picker offers them.
    pub fn sources() -> Vec<semio_framework_plugin::ExampleSource> {
        vec![rooms_16::source(), flowers_24::source()]
    }

    /// 📇️ The same roster as a `&'static` slice, which is the shape `SubsetDeclaration` takes.
    pub fn example_source_slice() -> &'static [semio_framework_plugin::ExampleSource] {
        static EXAMPLES: std::sync::OnceLock<Vec<semio_framework_plugin::ExampleSource>> = std::sync::OnceLock::new();
        EXAMPLES.get_or_init(sources).as_slice()
    }

    #[cfg(test)]
    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️outcome/🦀️.rs"]
    mod tests;
}
//#endregion 📚️Examples

// ---- Shims: flat access from the artifact root, mirroring the sibling artifacts ----
pub mod schema {
    pub use super::standards::v1::subsets::any::schema::*;
}
pub mod diff {
    pub use crate::standards::v1::subsets::any::schema::diff::*;
}
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub mod inferences {
    pub use crate::standards::v1::subsets::any::schema::inferences::*;
}
pub use crate::standards::v1::subsets::any::schema::diff::BitmapDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::BitmapMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::BitmapSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧩️mount-contract/🦀️.rs"]
mod mount_contract;

#[cfg(all(test, feature = "component-app-assembly"))]
#[path = "🧪️tests/🔬️store-fixture/🦀️.rs"]
mod store_fixture;
//#endregion 🧪️Tests
