//! ✒️ Writer play app — the main window: the jack/text editor surface (writer's only window kind).

use crate::apps::writer::config::WriterConfig;
use crate::apps::writer::modes::edit::windows::main::options;
use crate::apps::writer::terminology::WriterPlayLabels;
use crate::artifacts::writer::schema::{jack_editor_placeholders, jack_newline_gate_offsets, jack_symbol_at_offset, language_completions_json, selectable_spans_for_jack, tokenize_language, JackSymbolKind};
use crate::artifacts::writer::schema::inferences::{language_diagnostics_json, language_tokens_json};
use crate::artifacts::writer::{writer_text, WriterSnapshot};
use semio_framework_plugin::{build_text_editor_scene, LocalizedLabel, SurfaceKind, TextEditorScene, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const WRITER_PLAY_WINDOW_KIND: &str = "writer-main";
pub const WRITER_PLAY_BODY_MAIN: &str = "writer.play.main";
const WRITER_PLAY_SURFACE_MAIN: &str = "writer.play";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::writer::create_writer_app`. `options.measures`
/// stays empty here on purpose: writer's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: WRITER_PLAY_WINDOW_KIND.into(),
        label: LocalizedLabel::native("Jack", "Jack"),
        body_key: WRITER_PLAY_BODY_MAIN.into(),
        surface_kind: SurfaceKind::TextEditor,
        icon_id: "document-jack".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        // 🕹️ Populated post-hoc by `create_writer_app`'s `.window_kind_interactions(..)` call — a
        // bare literal `WindowKindDefinition` (not built via `AppBuilder::window_kind_def`) has no
        // other way to reach the manifest's `InteractionRef`s.
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub fn window_measures(config: &WriterConfig, labels: &WriterPlayLabels) -> Vec<WindowMeasure> {
    vec![options::font_size::measure(config, labels), options::line_height::measure(config, labels), options::tab_size::measure(config, labels), options::line_numbers::measure(config, labels)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &WriterSnapshot, config: &WriterConfig) -> UiNode {
    let is_jack = document.language_id == "jack";
    let text = writer_text(document);
    let selection = config.editor_selection.clone().unwrap_or(crate::apps::writer::config::WriterEditorSelection { start: 0, end: 0 });
    let cursor = selection.end;
    let selection_json = Some(json!({ "start": selection.start, "end": selection.end }).to_string());

    let grammar_tokens = tokenize_language(&text, &document.language_id);
    let lsp_tokens = language_tokens_json(document);
    let tokens_json = lsp_tokens.clone().or_else(|| serde_json::to_string(&grammar_tokens).ok());
    eprintln!(
        "[DEBUG] writer.main tokens path language_id={} lsp_tokens={} grammar_fallback={}",
        document.language_id,
        lsp_tokens.is_some(),
        tokens_json.is_some()
    );
    eprintln!("[DEBUG] writer.main tokens_json={}", tokens_json.as_deref().unwrap_or("none"));

    let diagnostics_json = language_diagnostics_json(document, config.lint_signal);

    let selectable_spans_json = is_jack.then(|| serde_json::to_string(&selectable_spans_for_jack(&text, &grammar_tokens)).unwrap_or_else(|_| "[]".into()));
    let placeholders_json = is_jack.then(|| serde_json::to_string(&jack_editor_placeholders(&text, cursor)).unwrap_or_else(|_| "[]".into()));
    let newline_gates_json = is_jack.then(|| serde_json::to_string(&jack_newline_gate_offsets(&text)).unwrap_or_else(|_| "[]".into()));

    // 🕹️ Tree-row/editor hover cross-highlighting DISSOLVED (ticket
    // 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM): the `ast` domain's hover now lives in the
    // framework's own `InteractionState`/presence broadcast, not in `WriterConfig` — `render` has no
    // `InteractionView` (only `handle`/`copy_fragment`/`cut_operations` do), so this scene no longer
    // computes a server-side `hoverJson`; the client renders `ast` hover highlights itself from live
    // interaction/presence state.
    let hover_json: Option<String> = None;

    let caret_symbol = if is_jack && selection.start == selection.end { jack_symbol_at_offset(&text, selection.start) } else { None };
    let (selection_occurrences, rename_json): (Vec<(usize, usize)>, Option<String>) = match &caret_symbol {
        Some(symbol) if symbol.kind == JackSymbolKind::Variable => {
            let occurrences_json: Vec<Value> = symbol.occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
            let rename = json!({ "name": symbol.name, "occurrences": occurrences_json }).to_string();
            (symbol.occurrences.clone(), Some(rename))
        }
        _ => (Vec::new(), None),
    };

    let occurrences_json = is_jack.then(|| {
        let selection: Vec<Value> = selection_occurrences.iter().map(|(s, e)| json!({ "start": s, "end": e })).collect();
        json!({
            "hover": "[]",
            "selection": serde_json::to_string(&selection).unwrap_or_else(|_| "[]".into()),
        })
        .to_string()
    });

    let extra_carets_json = (!selection_occurrences.is_empty()).then(|| serde_json::to_string(&selection_occurrences.iter().map(|(s, _)| *s).collect::<Vec<_>>()).unwrap_or_else(|_| "[]".into()));

    let completions_json = language_completions_json(&text, &document.language_id, cursor);

    build_text_editor_scene(
        WRITER_PLAY_SURFACE_MAIN,
        crate::apps::writer::WRITER_PLAY_APP_ID,
        TextEditorScene {
            buffer: text.clone(),
            language: Some(document.language_id.clone()),
            selection_json,
            tokens_json,
            diagnostics_json,
            completions_json,
            occurrences_json,
            overlays_json: config.editor_settings.show_line_numbers.then(|| json!({ "lineNumbers": true }).to_string()),
            placeholders_json,
            extra_carets_json,
            selectable_spans_json,
            settings_json: Some(serde_json::to_string(&config.editor_settings).unwrap_or_else(|_| "{}".into())),
            camera_json: Some(json!({ "x": config.camera.x, "y": config.camera.y, "zoom": config.camera.zoom }).to_string()),
            hover_json,
            newline_gates_json,
            rename_json,
        },
    )
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::writer::testkit::{main_window_measures, new_app, render as render_body};
    use semio_framework_plugin::PluginApp;

    #[test]
    fn renders_text_editor_scene() {
        let mut app = new_app();
        assert!(render_body(&mut app, WRITER_PLAY_BODY_MAIN).contains("text-editor"));
    }

    #[test]
    fn scene_emits_placeholders_selectable_spans_and_newline_gates_for_jack() {
        let mut app = new_app();
        let node = app.render(WRITER_PLAY_BODY_MAIN, Some(&crate::artifacts::writer::dsl::jack_example_json()), &semio_framework_plugin::ViewModel::default()).expect("render");
        let json = serde_json::to_string(&node).unwrap();
        assert!(json.contains("placeholdersJson"));
        assert!(json.contains("selectableSpansJson"));
        assert!(json.contains("newlineGatesJson"));
    }

    #[test]
    fn window_measures_expose_font_line_height_tab_and_toggle() {
        let mut app = new_app();
        let measures = main_window_measures(&mut app);
        assert_eq!(measures.len(), 4);
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Toggle { id, .. } if id == "writer-line-numbers-measure")));
    }

    #[test]
    fn definition_declares_the_text_editor_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, WRITER_PLAY_BODY_MAIN);
        assert!(matches!(definition.surface_kind, SurfaceKind::TextEditor));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }
}
//#endregion 🧪️Tests
