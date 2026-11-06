//! 🧪️ Output window — the read-only inferred canvas, its staleness guard and its action roster.

use crate::editor::bitmap::transient::BitmapTransient;
use semio_plugin::SurfaceKind;
use semio_plugin::ToolRunView;
use crate::editor::bitmap::modes::edit::windows::output::*;
use crate::editor::bitmap::modes::edit::windows::output::config::BitmapOutputWindowConfig;
use crate::standards::v1::subsets::any::io::text::snapshot::decode_base64;
use crate::standards::v1::subsets::any::io::text::snapshot::encode_base64;

#[test]
fn the_window_kind_is_a_canvas_with_a_migrated_action_roster() {
    let definition = definition();
    assert_eq!(definition.id, WFC_BITMAP_WINDOW_OUTPUT);
    assert_eq!(definition.body_key, BODY_KEY);
    assert_eq!(definition.surface_kind, SurfaceKind::Canvas2d);
    for action in &definition.actions {
        assert_eq!(action.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated);
    }
}

/// 📨️ A fill completion stays dispatchable without appearing as an empty user command.
#[test]
fn the_output_palette_only_offers_author_actions() {
    let source = include_str!("../../../../../../../../../../../../../🧪️tests/🧫️bitmap-output-actions/🔣️.json");
    let oracle: serde_json::Value = serde_json::from_str(source).expect("portable action contract");
    let definition = definition();
    let public: Vec<String> = definition.actions.iter().filter(|action| action.in_palette).map(|action| action.id.clone()).collect();
    let actual = semio_framework_value::ToValue::to_value(&public);
    let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&actual));
    assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).expect("native action output"), oracle["publicActions"]);
    let internal = definition.actions.iter().find(|action| action.id == oracle["internalAction"].as_str().unwrap()).expect("completion remains registered");
    assert!(!internal.in_palette);
    assert_eq!(internal.semantics.execution.interactive_job, semio_framework::InteractiveJobClassification::Migrated);
}

/// 🪪️ Completed and live previews belong to the exact canonical problem they solved.
#[test]
fn preview_visibility_follows_the_portable_source_revision_contract() {
    use crate::editor::bitmap::modes::edit::tools::fill::{self as fill_tool, payload_from_assignment};
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunState};

    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️bitmap-preview-freshness/🔣️.json")).expect("freshness contract");
    let snapshot = crate::examples::rooms_16::snapshot();
    let config = BitmapOutputWindowConfig::default();
    let count = (snapshot.output.width * snapshot.output.height) as usize;
    let mut observations = Vec::new();
    for case in fixture["cases"].as_array().unwrap() {
        let source = [case["sourceRevisionByte"].as_u64().unwrap() as u8; 32];
        let current = case["currentRevisionByte"].as_u64().map(|byte| [byte as u8; 32]);
        let empty = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &config, None, current);
        let cached = BitmapTransient { output_pixels: Some(encode_base64(&vec![1; count])), contradiction: false, output_width: snapshot.output.width, output_height: snapshot.output.height, source_revision: encode_base64(&source) };
        let cached_visible = render_layers_fingerprint(&snapshot, &cached, &config, None, current) != empty;
        let mut run = ToolRunView::new(fill_tool::TOOL_ID, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, source), ToolRunState::Running);
        run.payload = Some(payload_from_assignment(snapshot.output.width, snapshot.output.height, &[(0, 1)], false, false).encode_json().into_bytes().into());
        let live_visible = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &config, Some(&run), current) != empty;
        assert_eq!(cached_visible, case["visible"].as_bool().unwrap(), "cached {}", case["id"]);
        assert_eq!(live_visible, cached_visible, "live {}", case["id"]);
        observations.push(cached_visible);
    }
    let actual = semio_framework_value::ToValue::to_value(&observations);
    let encoded = semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&actual));
    let expected: Vec<bool> = fixture["cases"].as_array().unwrap().iter().map(|case| case["visible"].as_bool().unwrap()).collect();
    assert_eq!(serde_json::from_str::<Vec<bool>>(&encoded).expect("native visibility output"), expected);
}

#[test]
fn an_unsolved_output_still_renders_its_extent_for_every_example() {
    for snapshot in [crate::examples::rooms_16::snapshot(), crate::examples::flowers_24::snapshot()] {
        let layers = crate::bitmap_layers_json("out", snapshot.output.width, snapshot.output.height, &snapshot.input.palette, &[], &snapshot.pinned);
        assert!(layers.contains("out-extent"), "a pane with no solve is never structurally empty");
        assert!(render(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, Some([0; 32])).is_ok());
    }
}

#[test]
fn a_cached_solve_of_the_wrong_extent_is_discarded_rather_than_reshaped() {
    let snapshot = crate::examples::rooms_16::snapshot();
    let stale = BitmapTransient { output_pixels: Some(encode_base64(&[0, 1, 1, 0])), contradiction: false, output_width: 2, output_height: 2, source_revision: encode_base64(&[0; 32]) };
    assert!(render(&snapshot, &stale, &BitmapOutputWindowConfig::default(), None, Some([0; 32])).is_ok());
    let fresh = BitmapTransient {
        output_pixels: Some(encode_base64(&vec![0u8; (snapshot.output.width * snapshot.output.height) as usize])),
        contradiction: false,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
        source_revision: encode_base64(&[0; 32]),
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
    let empty = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, Some([0; 32]));
    let contradiction = BitmapTransient { output_pixels: None, contradiction: true, output_width: snapshot.output.width, output_height: snapshot.output.height, source_revision: encode_base64(&[0; 32]) };
    let layers = render_layers_fingerprint(&snapshot, &contradiction, &BitmapOutputWindowConfig::default(), None, Some([0; 32]));
    assert!(layers.contains("out-contradiction"), "contradiction must be visually labelled");
    assert_ne!(layers, empty);
}

#[test]
fn a_partial_fill_payload_paints_differently_from_empty_and_finished() {
    use crate::editor::bitmap::modes::edit::tools::fill::{self as fill_tool, payload_from_assignment};
    use semio_plugin::ToolRunView;
    use semio_framework_tool_run::{ToolRunId, ToolRunIdentity, ToolRunState};

    let snapshot = crate::examples::rooms_16::snapshot();
    let empty = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), None, Some([0; 32]));
    let finished = BitmapTransient {
        output_pixels: Some(encode_base64(&vec![1u8; (snapshot.output.width * snapshot.output.height) as usize])),
        contradiction: false,
        output_width: snapshot.output.width,
        output_height: snapshot.output.height,
        source_revision: encode_base64(&[0; 32]),
    };
    let finished_layers = render_layers_fingerprint(&snapshot, &finished, &BitmapOutputWindowConfig::default(), None, Some([0; 32]));
    let partial = payload_from_assignment(snapshot.output.width, snapshot.output.height, &[(0, 1)], false, false);
    let mut run = ToolRunView::new(fill_tool::TOOL_ID, ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 9 }, [0; 32]), ToolRunState::Running);
    run.payload = Some(partial.encode_json().into_bytes().into());
    let partial_layers = render_layers_fingerprint(&snapshot, &BitmapTransient::default(), &BitmapOutputWindowConfig::default(), Some(&run), Some([0; 32]));
    assert_ne!(partial_layers, empty);
    assert_ne!(partial_layers, finished_layers);
}


/// 🏁️ Final pixels ignore every search trace, while partial pixels retain tried colors and discard markers.
#[test]
fn completed_payload_ignores_trace_pixels_and_discard_markers() {
    use crate::editor::bitmap::modes::edit::tools::fill::{BitmapFillPayload, BitmapFillTraceEvent};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../../../🧪️tests/🧫️completed-preview/🔣️.json")).unwrap();
    let mut document = crate::BitmapSnapshot::reference_scene();
    document.output.width = 1;
    document.output.height = 1;
    for case in fixture["cases"].as_array().unwrap() {
        let payload = BitmapFillPayload { pixels: encode_base64(&[case["assigned"].as_u64().unwrap() as u8]), decided: encode_base64(&[1]), width: 1, height: 1, contradiction: false, done: case["done"].as_bool().unwrap(), trace: vec![BitmapFillTraceEvent { index: 0, color: case["traced"].as_u64().unwrap() as u32, discarded: case["discarded"].as_bool().unwrap() }] };
        let layers: serde_json::Value = serde_json::from_str(&layers_from_fill_payload(&document, &payload, &[])).unwrap();
        let layers = layers.as_array().unwrap();
        let painted = layers.iter().find(|layer| layer["id"] == "out-0-0").unwrap();
        let color: Vec<f64> = serde_json::from_value(painted["fill"]["color"].clone()).unwrap();
        assert_eq!(color, document.input.palette[case["expected"].as_u64().unwrap() as usize].to_unit_rgba().to_vec(), "{}", case["id"]);
        assert_eq!(layers.iter().any(|layer| layer["id"] == "out-pin-0-0"), case["showDiscard"].as_bool().unwrap(), "{}", case["id"]);
    }
}
