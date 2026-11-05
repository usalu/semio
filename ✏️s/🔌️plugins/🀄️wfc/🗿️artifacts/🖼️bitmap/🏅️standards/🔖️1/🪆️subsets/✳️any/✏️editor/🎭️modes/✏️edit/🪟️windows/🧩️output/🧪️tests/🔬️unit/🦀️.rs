//! 🧪️ Output window — the read-only inferred canvas, its staleness guard and its action roster.

use crate::editor::bitmap::transient::BitmapTransient;
use semio_plugin::SurfaceKind;
use semio_plugin::ToolRunView;
use crate::editor::bitmap::modes::edit::windows::output::*;
use crate::editor::bitmap::modes::edit::windows::output::config::BitmapOutputWindowConfig;
use crate::standards::v1::subsets::any::io::text::snapshot::decode_base64;
use crate::standards::v1::subsets::any::io::text::snapshot::encode_base64;

use super::complete_pinned_indices;

/// 🧷️ A kept bitmap renders from complete authored pins after its transient preview expires.
#[test]
fn complete_pins_follow_the_portable_output_contract() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️bitmap-pinned-output/🔣️.json")).expect("pinned output contract");
    let mut snapshot = crate::examples::rooms_16::snapshot();
    snapshot.output.width = fixture["width"].as_u64().unwrap() as u32;
    snapshot.output.height = fixture["height"].as_u64().unwrap() as u32;
    snapshot.input.palette.truncate(fixture["paletteSize"].as_u64().unwrap() as usize);
    for case in fixture["cases"].as_array().unwrap() {
        snapshot.pinned = case["pins"].as_array().unwrap().iter().map(|pin| crate::schema::snapshot::BitmapPinnedPixel { x: pin[0].as_u64().unwrap() as u32, y: pin[1].as_u64().unwrap() as u32, color: pin[2].as_u64().unwrap() as u32 }).collect();
        let indices = complete_pinned_indices(&snapshot);
        let native = semio_framework_value::ToValue::to_value(&indices);
        let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&native));
        assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).expect("native pinned output"), case["indices"], "{}", case["id"]);
        if let Some(indices) = indices {
            let mut config = BitmapOutputWindowConfig::default();
            config.show_pins = false;
            let actual = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &config, None, None);
            assert_eq!(actual, crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &indices, &[]));
        }
    }
}

#[test]
fn an_unsolved_output_still_renders_its_extent_for_every_example() {
    for snapshot in [crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()] {
        let layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
        assert!(layers.contains("out-extent"), "a pane with no solve is never structurally empty");
        assert!(render(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, None).is_ok());
    }
}

#[test]
fn a_cached_solve_of_the_wrong_extent_is_discarded_rather_than_reshaped() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let stale = BitmapTransient { output_pixels: Some(encode_base64(&[0, 1, 1, 0])), contradiction: false, output_width: 2, output_height: 2 };
    assert!(render(&snapshot, &stale, &BitmapOutputWindowConfig::default(), None, None).is_ok());
    let fresh = BitmapTransient {
        output_pixels: Some(encode_base64(&vec![0u8; (snapshot.output.width * snapshot.output.height) as usize])),
        contradiction: false,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
    };
    let stale_layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
    let fresh_indices = decode_base64(fresh.output_pixels.as_deref().expect("fresh pixels")).expect("fresh pixels decode");
    let fresh_layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &fresh_indices, &snapshot.pinned);
    assert_ne!(stale_layers, fresh_layers, "a fresh solve draws more than a bare extent");
}

#[test]
fn hiding_the_pin_overlay_drops_exactly_the_pin_layers() {
    let snapshot = crate::examples::flowers_24::snapshot();
    let with_pins = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
    let without = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &[]);
    assert!(with_pins.contains("out-pin-0-0"));
    assert!(!without.contains("out-pin-"));
}


#[test]
fn a_contradiction_transient_paints_a_labelled_overlay_not_a_silent_empty() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let empty = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, None);
    let contradiction = BitmapTransient { output_pixels: None, contradiction: true, output_width: snapshot.output.width, output_height: snapshot.output.height };
    let layers = render_layers_fingerprint(&snapshot, &contradiction, &BitmapOutputWindowConfig::default(), None, None);
    assert!(layers.contains("out-contradiction"), "contradiction must be visually labelled");
    assert_ne!(layers, empty);
}

#[test]
fn a_partial_fill_payload_paints_differently_from_empty_and_finished() {
    use crate::editor::bitmap::modes::edit::tools::fill::{self as fill_tool, payload_from_assignment};
    use semio_plugin::ToolRunView;
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunState};

    let snapshot = crate::examples::rooms_16::snapshot();
    let empty = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, None);
    let finished = BitmapTransient {
        output_pixels: Some(encode_base64(&vec![1u8; (snapshot.output.width * snapshot.output.height) as usize])),
        contradiction: false,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
    };
    let finished_layers = render_layers_fingerprint(&snapshot, &finished, &BitmapOutputWindowConfig::default(), None, None);
    let partial = payload_from_assignment(snapshot.output.width, snapshot.output.height, &[(0, 1)], false, false);
    let mut run = ToolRunView::new(fill_tool::TOOL_ID, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 9 }, [0; 32]), ToolRunState::Running);
    run.payload = Some(partial.encode_json().into_bytes().into());
    let partial_layers = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), Some(&run), None);
    assert_ne!(partial_layers, empty);
    assert_ne!(partial_layers, finished_layers);
}

