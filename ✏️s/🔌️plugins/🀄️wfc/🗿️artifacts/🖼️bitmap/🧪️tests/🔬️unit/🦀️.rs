//! 🧪️ Bitmap artifact root — identity, the shared canvas layer builder, and the committed-fixture
//! generator this artifact's quintets are produced by.

use crate::schema::snapshot::{decode_base64, encode_base64, BitmapColor, BitmapPinnedPixel, BitmapSnapshot};

#[test]
fn the_dialect_agrees_with_the_document_schema() {
    assert_eq!(crate::WFC_BITMAP_DIALECT.artifact_kind, crate::WFC_BITMAP_DOCUMENT_SCHEMA);
    assert_eq!(crate::WFC_BITMAP_DOCUMENT_SCHEMA, "s.wfc.bitmap");
    assert_eq!(crate::WFC_BITMAP_DIALECT.standard.0, "1");
}

#[test]
fn the_artifact_kind_is_the_os_level_two_dimensional_raster_id() {
    let kind = crate::artifact_kind();
    assert_eq!(kind.id, "2d.wfcbitmap");
    assert_eq!(kind.component_kind, "wfcbitmap");
    assert_eq!(kind.dimension, "2d");
    assert_eq!(kind.schema, crate::WFC_BITMAP_DOCUMENT_SCHEMA);
}

#[test]
fn the_capability_definition_builds() {
    assert!(crate::definition().is_ok());
}

#[test]
fn base64_round_trips_every_length_class() {
    for length in 0..64usize {
        let bytes: Vec<u8> = (0..length).map(|index| (index * 37 % 251) as u8).collect();
        let text = encode_base64(&bytes);
        assert_eq!(text.len() % 4, 0, "base64 is padded to a multiple of four");
        assert_eq!(decode_base64(&text).as_deref(), Some(bytes.as_slice()), "length {length}");
    }
}

#[test]
fn base64_refuses_a_malformed_buffer() {
    assert_eq!(decode_base64("AAA"), None, "an unpadded remainder is not base64");
    assert_eq!(decode_base64("A!AA"), None, "a non-alphabet byte is not base64");
    assert_eq!(decode_base64("A=AA"), None, "padding may only sit at the end");
}

#[test]
fn the_layer_builder_always_draws_the_extent_even_with_no_pixels() {
    let layers = crate::bitmap_layers_json("out", 4, 4, &[BitmapColor::opaque(1, 2, 3)], &[], &[]);
    assert!(layers.starts_with('['), "the layer list is a JSON array");
    assert!(layers.contains("out-extent"), "an unsolved output still states its extent");
}

#[test]
fn the_layer_builder_merges_equal_runs_and_marks_pins() {
    let palette = vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)];
    let indices = vec![0, 0, 0, 0, 1, 1, 1, 1];
    let layers = crate::bitmap_layers_json("in", 4, 2, &palette, &indices, &[BitmapPinnedPixel { x: 1, y: 1, color: 1 }]);
    assert_eq!(layers.matches("\"kind\":\"path\"").count(), 4, "extent + two merged runs + one pin");
    assert!(layers.contains("in-pin-1-1"));
}

#[test]
fn the_layer_budget_is_respected_by_coarsening() {
    let palette: Vec<BitmapColor> = (0..8u32).map(|index| BitmapColor::opaque(index * 31, 0, 0)).collect();
    let indices: Vec<u8> = (0..(96 * 96u32)).map(|cell| ((cell * 7 + cell / 96) % 8) as u8).collect();
    let layers = crate::bitmap_layers_json("in", 96, 96, &palette, &indices, &[]);
    assert!(layers.len() <= crate::BITMAP_LAYERS_JSON_BUDGET_BYTES, "a dense sample is coarsened rather than overflowing the surface spine");
}

#[test]
fn every_example_declares_a_distinct_id_and_a_localized_label() {
    let sources = crate::examples::sources();
    assert_eq!(sources.len(), 2);
    assert_ne!(sources[0].id(), sources[1].id());
    assert_eq!(crate::examples::example_source_slice().len(), 2);
}

//#region 🧫️FixtureGenerator
/// 🧫️ The committed quintet generator. Every fixture under
/// `🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations/` is emitted from THIS table, so a
/// hand-edited fixture is a bug by construction: run `cargo test -p semio-s-artifact-wfc-bitmap
/// --features component-app-assembly --lib -- --ignored fixtures` to regenerate them.
#[cfg(test)]
pub mod fixtures {
    use super::*;
    use crate::mutations::{add_palette_color, change_model, change_palette_color, change_seed, pin_pixel, remove_palette_color, resize_input, resize_output, set_input_pixels, unpin_pixel, BitmapMutation};
    use crate::schema::snapshot::{BitmapInput, BitmapOutputSpec, BitmapOverlappingModel, WFC_BITMAP_DOCUMENT_SCHEMA};

    /// 🧪️ The shared scene every case starts from: a 4 × 3 two-colour checker, a 6 × 4 output, an
    /// `N = 2` model, no pins. Small enough to read in a diff, structured enough that the
    /// overlapping model has real patterns to learn.
    pub fn base() -> BitmapSnapshot {
        BitmapSnapshot {
            schema: WFC_BITMAP_DOCUMENT_SCHEMA.into(),
            seed: 7,
            input: BitmapInput {
                width: 4,
                height: 3,
                palette: vec![BitmapColor::opaque(0, 0, 0), BitmapColor::opaque(255, 255, 255)],
                pixels: encode_base64(&[0, 1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 0]),
            },
            output: BitmapOutputSpec { width: 6, height: 4, periodic: false },
            model: BitmapOverlappingModel { pattern_size: 2, symmetry: 1, periodic_input: true, ground: None },
            pinned: Vec::new(),
        }
    }

    /// 🎨️ The base scene plus an unused third palette entry — the only state in which removing a
    /// colour is legal at all.
    pub fn base_with_spare_colour() -> BitmapSnapshot {
        let mut snapshot = base();
        snapshot.input.palette.push(BitmapColor::opaque(200, 60, 60));
        snapshot
    }

    /// 📌️ The base scene plus the two pins the pin/unpin and output-shrink cases need.
    pub fn base_with_pins() -> BitmapSnapshot {
        let mut snapshot = base();
        snapshot.pinned = vec![BitmapPinnedPixel { x: 0, y: 0, color: 1 }, BitmapPinnedPixel { x: 4, y: 3, color: 0 }];
        snapshot
    }

    pub fn cases() -> Vec<(&'static str, &'static str, BitmapSnapshot, BitmapMutation)> {
        vec![
            ("🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99", base(), change_seed(99)),
            ("📐️resize-input", "📐️grows-the-sample-to-6-by-4", base(), resize_input(6, 4)),
            ("🖌️set-input-pixels", "🖌️paints-a-2-by-2-block-of-colour-1", base(), set_input_pixels(1, 0, 2, 2, encode_base64(&[1, 1, 1, 1]))),
            ("🎨️add-palette-color", "🎨️appends-a-third-colour", base(), add_palette_color(2, BitmapColor::opaque(200, 60, 60))),
            ("🖍️change-palette-color", "🖍️recolours-the-second-entry", base(), change_palette_color(1, BitmapColor::opaque(240, 230, 200))),
            ("🧽️remove-palette-color", "🧽️drops-the-unused-third-colour", base_with_spare_colour(), remove_palette_color(2)),
            ("🖼️resize-output", "🖼️shrinks-the-output-and-cascades-a-pin", base_with_pins(), resize_output(2, 2, true)),
            ("⚙️change-model", "⚙️widens-the-window-to-three", base(), change_model(3, 4, false, Some(0))),
            ("📌️pin-pixel", "📌️pins-the-origin-cell-to-colour-1", base(), pin_pixel(0, 0, 1)),
            ("📍️unpin-pixel", "📍️releases-the-pinned-origin-cell", base_with_pins(), unpin_pixel(0, 0)),
        ]
    }
}
//#endregion 🧫️FixtureGenerator

//#region 🖨️FixtureEmitter
/// 🖨️ Re-indents a compact JSON string WITHOUT reordering its keys — `serde_json`'s own pretty
/// printer would sort them, and a fixture whose key order no longer matches its struct's field
/// order is exactly the drift the canonical-form tests exist to catch.
fn reindent_json(text: &str) -> String {
    let mut out = String::with_capacity(text.len() * 2);
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for character in text.chars() {
        if in_string {
            out.push(character);
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == '"' {
                in_string = false;
            }
            continue;
        }
        match character {
            '"' => {
                in_string = true;
                out.push(character);
            }
            '{' | '[' => {
                depth += 1;
                out.push(character);
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
            }
            '}' | ']' => {
                depth -= 1;
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
                out.push(character);
            }
            ',' => {
                out.push(character);
                out.push('\n');
                out.push_str(&"  ".repeat(depth));
            }
            ':' => out.push_str(": "),
            other if other.is_whitespace() => {}
            other => out.push(other),
        }
    }
    out.push('\n');
    collapse_empty_containers(&out)
}

/// 🧹️ Folds `[\n   \n]` and `{\n   \n}` back to `[]`/`{}` — an empty container printed across
/// three lines is noise in every committed fixture and in every review of one.
fn collapse_empty_containers(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find(['[', '{']) {
        let opener = rest.as_bytes()[start] as char;
        let closer = if opener == '[' { ']' } else { '}' };
        out.push_str(&rest[..=start]);
        let tail = &rest[start + 1..];
        let inner = tail.trim_start_matches([' ', '\n']);
        if inner.starts_with(closer) {
            out.push(closer);
            rest = &inner[1..];
        } else {
            rest = tail;
        }
    }
    out.push_str(rest);
    out
}

fn subset_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any")
}

fn write_fixture(directory: &std::path::Path, leaf: &str, body: &str) {
    let path = directory.join(leaf);
    std::fs::create_dir_all(path.parent().expect("fixture leaf has a parent")).expect("fixture directory");
    std::fs::write(&path, body).expect("fixture write");
}

/// 🧫️ Regenerates every committed quintet and both example assets from the Rust authorities above.
/// Ignored by default: it WRITES into the source tree, which is the point — the committed files are
/// a print of this table and never hand-edited.
#[test]
#[ignore]
fn emit_committed_fixtures() {
    use crate::mutations::apply_bitmap_mutation;
    let root = subset_root();
    for (kind, case, before, mutation) in fixtures::cases() {
        let outcome = <crate::BitmapMutation as protocol::Mutation<BitmapSnapshot>>::diff(&mutation, &before);
        let mut after = before.clone();
        apply_bitmap_mutation(&mut after, &mutation).unwrap_or_else(|error| panic!("{kind}/{case} applies: {error}"));
        let directory = root.join("🧫️fixtures/🧬️mutations").join(kind).join(case);
        write_fixture(&directory, "📸️snapshot/⬅️before/🔣️.json", &reindent_json(&dsl::json::to_json_string(&before)));
        write_fixture(&directory, "📸️snapshot/➡️after/🔣️.json", &reindent_json(&dsl::json::to_json_string(&after)));
        write_fixture(&directory, "🦠️mutation/🔣️.json", &reindent_json(&dsl::json::to_json_string(&mutation)));
        write_fixture(&directory, "🔺️diff/🔣️.json", &reindent_json(&dsl::json::to_json_string(outcome.diff())));
        let messages: Vec<String> = outcome
            .messages()
            .iter()
            .map(|message| {
                let level = dsl::json::to_json_string(&message.level);
                format!("{{\"level\":{level},\"code\":\"{}\"}}", message.code.0)
            })
            .collect();
        let body = if messages.is_empty() { "{\"status\":\"applied\"}".to_string() } else { format!("{{\"status\":\"applied\",\"messages\":[{}]}}", messages.join(",")) };
        write_fixture(&directory, "🎯️outcome/🔣️.json", &reindent_json(&body));
    }
    for (slug, text) in [
        ("🚪️rooms-16", crate::schema::snapshot::text::print_dsl(&crate::examples::rooms_16::snapshot())),
        ("🌸️flowers-24", crate::schema::snapshot::text::print_dsl(&crate::examples::flowers_24::snapshot())),
    ] {
        let path = root.join("📚️examples").join(slug).join("🖼️assets").join(slug).join("🗣️.dsl.semio");
        std::fs::create_dir_all(path.parent().expect("asset parent")).expect("asset directory");
        std::fs::write(&path, text).expect("asset write");
    }
}
//#endregion 🖨️FixtureEmitter
