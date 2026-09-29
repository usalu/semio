#!/usr/bin/env python3
"""🧊️ WG11 session 14d — second renderer-wgpu product set for the first train after the chain (T6), AFTER
`wg11-renderer-product-patch.py`: product roots behind four more red laws (WG11 overlay builds 4–5,
`.🧬semio/🌐hub/s14-wg11-captures/reds-6-panics.txt`).

1. `svg_export_matches_neutral_three_and_sharp_fixture_and_retires_before_delivery` — the SVG exporter culled every front face:
   `front_facing` is three's `Projector.checkBackfaceCulling`, which judges winding in NDC (y up), but it ran on the y-down SVG
   coordinates `project` writes, where the winding is mirrored. The fixture's camera-facing triangle painted no fill at all.
   The sign now reads the mirrored space; law: a clockwise copy of the same triangle paints only its outline.
2. `shared_layout_contract_drives_paint_hit_accessibility_and_mutation_badge_geometry` — the label track priced every scalar at `0.43 × text-xs`, while React's `auto` column holds `text-2xs` chips measured by the
   browser (`🕰️HistoryTable/🟦️.tsx`): 85.8 px against React's 80.9. The painter now measures each chip, the `checkpoint`
   placeholder and the mutation badge with the face it pens at `text-2xs` (React's size), keeps the measured track on the
   scene, and the hit test and accessibility read that one painted geometry. The checkpoint-hit fixture's `x` points (read only
   by the wgpu law — React's twin clicks DOM regions) sat on a boundary no React geometry produces (157): React's selectable
   checkpoint spans the `auto` label column (`Head` 23.33 px + `px-1.5`×2 + `px-single`×2 = 41.70) and the 96 px graph column
   from 23.19 → it ends at 160.89; `graph-select` is now 160.5 and the inert boundary sample 162.5 (clear of this train's
   quantized-size track, 161.9).
3. `footer_bands_match_the_react_chrome_band_fixture` — `navbarCenteredLeftV1` and its Rust twin round the clamped left to a
   whole pixel, which can cross a sub-pixel band edge (656.85 → 657 overhangs the free band by 0.15 px), breaking the shared
   contract `centeredItemNeverCrossesOccupiedChrome`. Both twins now snap to the nearest whole pixel INSIDE the band (the
   unrounded clamp when no whole pixel fits); the shared fixture gains the two sub-pixel vectors both laws read, and the footer
   band law expects the twin's own placement instead of re-deriving a rounded ideal within half a pixel.
   The GraphTimeline wiring law also reads its controls from the MOUNTED scene (engine-minted host id, 09-21) through the
   helpers `wg11-renderer-test-drift-patch.py` adds — this set lands AFTER that one.
4. `a_pane_chip_press_over_an_engine_surface_belongs_to_the_shell` — a retained projection body row is keyed
   `<surface>/<id>` (`PanelProjection::key`), so the pointer-owner rule, which matched the bare `framework.worldOrbit.projection`
   prefix, handed a press on a Projection row to the world surface under the pane. The rule reads the surface-local id.

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/renderer-product-2/` and applies;
`--revert` restores the backups.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
ENGINE = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
SVG = ENGINE / "🖼️IconRenderHost/🎯️targets/🧊️wgpu/📤️export/📐️svg/🦀️.rs"
SVG_LAWS = ENGINE / "🖼️IconRenderHost/🧪️tests/📤️svg-export/🦀️.rs"
SCENES = ENGINE / "🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"
GRAPH_LAWS = ENGINE / "🎞️Scenes/🧪️tests/🔬️wgpu-graph-timeline/🦀️.rs"
WIRING_LAWS = ENGINE / "🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs"
SHELL = ENGINE / "🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
NAVBAR = UI / "🧱️elements/🔝️Navbar/🟦️.tsx"
BAND_FIXTURE = UI / "🧫️fixtures/🔝️navbar-centered-band/🔣️.json"
PANEL_LAWS = ENGINE / "🐚️Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs"
CHECKPOINT_FIXTURE = ENGINE / "🌳️GraphTimelineHost/🧫️fixtures/🎯️checkpoint-hit/🔣️.json"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/renderer-product-2"

SVG_EDITS = [
    (
        '''fn front_facing(points: [[f32; 3]; 3]) -> bool {
    (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]) - (points[2][1] - points[0][1]) * (points[1][0] - points[0][0]) < 0.0
}''',
        '''/// 🔄️ three's `Projector.checkBackfaceCulling` judges winding in NDC, y up (`< 0` is a front face); [`project`] writes y-down
/// SVG space, which mirrors every winding, so a front face is positive here.
fn front_facing(points: [[f32; 3]; 3]) -> bool {
    (points[2][0] - points[0][0]) * (points[1][1] - points[0][1]) - (points[2][1] - points[0][1]) * (points[1][0] - points[0][0]) > 0.0
}''',
    )
]

SVG_LAW_EDITS = [
    (
        '''#[test]
fn svg_export_cancellation_retires_the_exact_asset_without_publication() {''',
        '''/// 🔄️ LAW (ticket 26/09/23 session 14d, WG11): the exporter culls exactly the faces three's `Projector` culls — winding judged
/// in NDC, not in the y-down SVG space — so the fixture's camera-facing triangle wound clockwise paints no fill, only its outline.
#[test]
fn svg_export_culls_a_back_face_by_its_ndc_winding() {
    let mut fixture = fixture();
    fixture["geometry"]["indices"] = serde_json::json!([0, 2, 1]);
    let mut export = IconSvgExport::new(&request(&fixture), asset(0x5356_4703, &fixture)).expect("SVG export");
    let mut steps = 0;
    while !export.advance().expect("bounded SVG step") {
        steps += 1;
        assert!(steps < 256, "SVG export exceeded the bounded fixture ladder");
    }
    let markup = String::from_utf8(export.take_svg().expect("one SVG delivery")).unwrap();
    assert!(!markup.contains("fill:rgb("), "a back face paints no fill: {markup}");
    assert_eq!(markup.matches("<path ").count(), 1, "only the outline stroke remains");
    export.cancel();
    while !export.close_step() {}
    assert!(export.terminal_is_empty());
}

#[test]
fn svg_export_cancellation_retires_the_exact_asset_without_publication() {''',
    )
]

SCENES_EDITS = [
    (
        '''    graph_timeline_accessibility: SceneAccessibilityPresentation<GraphTimelineAccessibilityControl>,
''',
        '''    graph_timeline_accessibility: SceneAccessibilityPresentation<GraphTimelineAccessibilityControl>,
    /// 📏️ The label track this surface's last GraphTimeline paint measured — the one geometry its hit test reads.
    graph_timeline_label_track: f32,
''',
    ),
    (
        '''fn graph_timeline_label_text_width(text: &str, theme: &Theme) -> f32 {
    text.chars().count() as f32 * theme.font_size_small * 0.43
}

fn graph_timeline_label_track_width(columns: &[HistoryColumnJson], theme: &Theme) -> f32 {
    let chip_padding = theme.root_rem_pixels * 0.375;
    let chip_gap = theme.root_rem_pixels * 0.25;
    columns
        .iter()
        .map(|column| {
            let content = if column.labels.is_empty() {
                graph_timeline_label_text_width("checkpoint", theme)
            } else {
                column.labels.iter().map(|label| graph_timeline_label_text_width(label, theme) + chip_padding * 2.0).sum::<f32>() + chip_gap * column.labels.len().saturating_sub(1) as f32
            };
            content + theme.padding_standard * 2.0
        })
        .fold(0.0, f32::max)
}''',
        '''/// 🏷️ React's checkpoint chips, their `checkpoint` placeholder and the mutation badge are `text-2xs` (`🕰️HistoryTable/🟦️.tsx`).
const GRAPH_TIMELINE_CHIP_FONT_PX: f32 = ui_styling::metrics::typography::TEXT2XS_PX as f32;

/// 📏️ One chip label's width in the face the painter pens it with.
fn graph_timeline_chip_text_width(atlas: &mut ui_wgpu::wgpu::FontAtlas, text: &str) -> f32 {
    atlas.measure_text(text, GRAPH_TIMELINE_CHIP_FONT_PX).0
}

/// 📏️ The label track React's `auto` grid column resolves to: the widest row's chips (each `px-1.5` around its measured label,
/// `gap-1` apart) or its `checkpoint` placeholder, inside the row's `px-single` padding.
fn graph_timeline_label_track_width(columns: &[HistoryColumnJson], theme: &Theme, atlas: &mut ui_wgpu::wgpu::FontAtlas) -> f32 {
    let chip_padding = theme.root_rem_pixels * 0.375;
    let chip_gap = theme.root_rem_pixels * 0.25;
    columns
        .iter()
        .map(|column| {
            let content = if column.labels.is_empty() {
                graph_timeline_chip_text_width(atlas, "checkpoint")
            } else {
                column.labels.iter().map(|label| graph_timeline_chip_text_width(atlas, label) + chip_padding * 2.0).sum::<f32>() + chip_gap * column.labels.len().saturating_sub(1) as f32
            };
            content + theme.padding_standard * 2.0
        })
        .fold(0.0, f32::max)
}

/// 📏️ [`graph_timeline_label_track_width`] of one scene's own columns.
pub(crate) fn graph_timeline_scene_label_track(scene: &UiComponentSceneNode, theme: &Theme, atlas: &mut ui_wgpu::wgpu::FontAtlas) -> f32 {
    let columns: Vec<HistoryColumnJson> = scene.graph_timeline.as_ref().and_then(|history| serde_json::from_str(&history.columns_json).ok()).unwrap_or_default();
    graph_timeline_label_track_width(&columns, theme, atlas)
}

/// 📏️ The label track `host_id`'s last paint measured (`0` before its first paint).
fn graph_timeline_painted_label_track(host_id: &str) -> f32 {
    SCENE_STATE.with(|cell| cell.borrow().get(host_id).map_or(0.0, |state| state.graph_timeline_label_track))
}''',
    ),
    (
        '''fn graph_timeline_layout(bounds: Rect, columns: &[HistoryColumnJson], theme: &Theme) -> GraphTimelineLayout {''',
        '''fn graph_timeline_layout(bounds: Rect, columns: &[HistoryColumnJson], theme: &Theme, label_track: f32) -> GraphTimelineLayout {''',
    ),
    (
        '''    let label_track_width = graph_timeline_label_track_width(columns, theme).min((inner.w - graph_column_width).max(0.0));''',
        '''    let label_track_width = label_track.min((inner.w - graph_column_width).max(0.0));''',
    ),
    (
        '''pub(crate) fn graph_timeline_accessibility_controls(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme) -> Vec<GraphTimelineAccessibilityControl> {
    let Some(history) = scene.graph_timeline.as_ref() else { return Vec::new() };
    let Ok(columns) = serde_json::from_str::<Vec<HistoryColumnJson>>(&history.columns_json) else { return Vec::new() };
    let layout = graph_timeline_layout(bounds, &columns, theme);''',
        '''pub(crate) fn graph_timeline_accessibility_controls(scene: &UiComponentSceneNode, bounds: Rect, theme: &Theme, label_track: f32) -> Vec<GraphTimelineAccessibilityControl> {
    let Some(history) = scene.graph_timeline.as_ref() else { return Vec::new() };
    let Ok(columns) = serde_json::from_str::<Vec<HistoryColumnJson>>(&history.columns_json) else { return Vec::new() };
    let layout = graph_timeline_layout(bounds, &columns, theme, label_track);''',
    ),
    (
        '''    let theme = ctx.theme;
    stage_graph_timeline_accessibility_controls(&scene.host_id, graph_timeline_accessibility_controls(scene, bounds, theme));
    let Some(history) = &scene.graph_timeline else {
        return render_placeholder("graph-timeline", bounds, ctx);
    };
    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let layout = graph_timeline_layout(bounds, &columns, theme);''',
        '''    let theme = ctx.theme;
    let columns: Vec<HistoryColumnJson> = scene.graph_timeline.as_ref().and_then(|history| serde_json::from_str(&history.columns_json).ok()).unwrap_or_default();
    let label_track = graph_timeline_label_track_width(&columns, theme, ctx.atlas);
    mutate_scene_state(&scene.host_id, |state| state.graph_timeline_label_track = label_track);
    stage_graph_timeline_accessibility_controls(&scene.host_id, graph_timeline_accessibility_controls(scene, bounds, theme, label_track));
    if scene.graph_timeline.is_none() {
        return render_placeholder("graph-timeline", bounds, ctx);
    }
    let layout = graph_timeline_layout(bounds, &columns, theme, label_track);''',
    ),
    (
        '''        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.65, theme.font_size_small, foreground_on_fill(theme, theme.text_muted, false, hovered));
        } else {
            for label in &column.labels {
                let chip_w = (graph_timeline_label_text_width(label, theme) + theme.root_rem_pixels * 0.75).min((inner.x + labels_col_w - pad - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                let chip_h = theme.control_height_small;
                ctx.draw.push_rounded([label_x, y + (row_h - chip_h) * 0.5, chip_w, chip_h], theme.accent, theme.border_radius);
                draw_text(ctx, label, label_x + theme.root_rem_pixels * 0.375, y + row_h * 0.5 + theme.font_size_small * 0.35, theme.font_size_small, theme.active_foreground);''',
        '''        if column.labels.is_empty() {
            draw_text(ctx, "checkpoint", label_x, y + row_h * 0.5 + GRAPH_TIMELINE_CHIP_FONT_PX * 0.35, GRAPH_TIMELINE_CHIP_FONT_PX, foreground_on_fill(theme, theme.text_muted, false, hovered));
        } else {
            for label in &column.labels {
                let chip_w = (graph_timeline_chip_text_width(ctx.atlas, label) + theme.root_rem_pixels * 0.75).min((inner.x + labels_col_w - pad - label_x).max(0.0));
                if chip_w <= 0.0 {
                    break;
                }
                let chip_h = theme.control_height_small;
                ctx.draw.push_rounded([label_x, y + (row_h - chip_h) * 0.5, chip_w, chip_h], theme.accent, theme.border_radius);
                draw_text(ctx, label, label_x + theme.root_rem_pixels * 0.375, y + row_h * 0.5 + GRAPH_TIMELINE_CHIP_FONT_PX * 0.35, GRAPH_TIMELINE_CHIP_FONT_PX, theme.active_foreground);''',
    ),
    (
        '''            let badge_w = graph_timeline_label_text_width(&label, theme) + theme.root_rem_pixels * 0.75;
            let badge_h = layout.avatar_size;
            ctx.draw.push_rounded([description_x, y + (row_h - badge_h) * 0.5, badge_w, badge_h], fill, theme.border_radius);
            draw_text(ctx, &label, description_x + theme.root_rem_pixels * 0.375, y + row_h * 0.5 + theme.font_size_small * 0.35, theme.font_size_small, text);''',
        '''            let badge_w = graph_timeline_chip_text_width(ctx.atlas, &label) + theme.root_rem_pixels * 0.75;
            let badge_h = layout.avatar_size;
            ctx.draw.push_rounded([description_x, y + (row_h - badge_h) * 0.5, badge_w, badge_h], fill, theme.border_radius);
            draw_text(ctx, &label, description_x + theme.root_rem_pixels * 0.375, y + row_h * 0.5 + GRAPH_TIMELINE_CHIP_FONT_PX * 0.35, GRAPH_TIMELINE_CHIP_FONT_PX, text);''',
    ),
    (
        '''    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let layout = graph_timeline_layout(bounds, &columns, theme);
    let inner = layout.inner;
    if x < inner.x || x >= inner.x + layout.selectable_width || y < inner.y || y >= inner.y + inner.h {''',
        '''    let columns: Vec<HistoryColumnJson> = serde_json::from_str(&history.columns_json).unwrap_or_default();
    let layout = graph_timeline_layout(bounds, &columns, theme, graph_timeline_painted_label_track(&scene.host_id));
    let inner = layout.inner;
    if x < inner.x || x >= inner.x + layout.selectable_width || y < inner.y || y >= inner.y + inner.h {''',
    ),
]

GRAPH_LAW_EDITS = [
    (
        '''fn paint_author_scene(scene: &UiComponentSceneNode) -> ui_wgpu::wgpu::DrawList {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, &mut atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_graph_timeline(scene, Rect::new(0.0, 0.0, 400.0, 100.0), &mut ctx);
    }
    draw
}''',
        '''fn paint_author_scene(scene: &UiComponentSceneNode) -> ui_wgpu::wgpu::DrawList {
    paint_timeline_scene(scene, Rect::new(0.0, 0.0, 400.0, 100.0), &mut ui_wgpu::wgpu::FontAtlas::builtin())
}

fn paint_timeline_scene(scene: &UiComponentSceneNode, bounds: Rect, atlas: &mut ui_wgpu::wgpu::FontAtlas) -> ui_wgpu::wgpu::DrawList {
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut input = ui_wgpu::wgpu::InputState::<ActionDescriptor>::default();
    let theme = Theme::default();
    let mut scroll = HashMap::new();
    let mut collapsed = HashMap::new();
    let mut selects = HashMap::new();
    {
        let mut ctx = crate::interpreter::framework_widget_context(&mut draw, None, atlas, None, &mut input, &theme, &mut scroll, &mut collapsed, &mut selects, None, 0.0);
        render_graph_timeline(scene, bounds, &mut ctx);
    }
    draw
}''',
    ),
    (
        '''    let theme = Theme::default();
    let layout = graph_timeline_layout(bounds, &columns, &theme);
    let close = |actual: f32, expected: &serde_json::Value| (actual - expected.as_f64().unwrap() as f32).abs() < 0.001;''',
        '''    let theme = Theme::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::shaped_default();
    let mut scene = author_scene(json!([]));
    scene.host_id = "timeline-layout".into();
    scene.controller_id = "timeline-layout-controller".into();
    scene.graph_timeline.as_mut().unwrap().columns_json = fixture["columns"].to_string();
    let painted = paint_timeline_scene(&scene, bounds, &mut atlas);
    let label_track = graph_timeline_label_track_width(&columns, &theme, &mut atlas);
    assert_eq!(graph_timeline_painted_label_track(&scene.host_id), label_track, "the paint keeps the track it measured — the geometry hit test and accessibility read");
    let layout = graph_timeline_layout(bounds, &columns, &theme, label_track);
    let close = |actual: f32, expected: &serde_json::Value| (actual - expected.as_f64().unwrap() as f32).abs() < 0.001;''',
    ),
    (
        '''    let mut scene = author_scene(json!([]));
    scene.host_id = "timeline-layout".into();
    scene.controller_id = "timeline-layout-controller".into();
    scene.graph_timeline.as_mut().unwrap().columns_json = fixture["columns"].to_string();
    let y = layout.inner.y + layout.row_height * 0.5;''',
        '''    let y = layout.inner.y + layout.row_height * 0.5;''',
    ),
    (
        '''    let control = graph_timeline_accessibility_controls(&scene, bounds, &theme).into_iter().next().expect("checkpoint accessibility");
    assert_eq!(control.rect, Rect::new(layout.inner.x, layout.inner.y, layout.selectable_width, layout.row_height));

    let painted = paint_author_scene(&scene);
    let warning''',
        '''    let control = graph_timeline_accessibility_controls(&scene, bounds, &theme, label_track).into_iter().next().expect("checkpoint accessibility");
    assert_eq!(control.rect, Rect::new(layout.inner.x, layout.inner.y, layout.selectable_width, layout.row_height));

    let warning''',
    ),
    (
        '''    let theme = Theme::default();
    let controls = graph_timeline_accessibility_controls(&scene, bounds, &theme);''',
        '''    let theme = Theme::default();
    let controls = graph_timeline_accessibility_controls(&scene, bounds, &theme, graph_timeline_scene_label_track(&scene, &theme, &mut ui_wgpu::wgpu::FontAtlas::shaped_default()));''',
    ),
    (
        '''    let bounds = Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32);
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::builtin();''',
        '''    let bounds = Rect::new(values[0].as_f64().unwrap() as f32, values[1].as_f64().unwrap() as f32, values[2].as_f64().unwrap() as f32, values[3].as_f64().unwrap() as f32);
    let mut draw = ui_wgpu::wgpu::DrawList::default();
    let mut atlas = ui_wgpu::wgpu::FontAtlas::shaped_default();''',
    ),
]

WIRING_EDITS = [
    (
        '''    let controls = crate::scenes::graph_timeline_accessibility_controls(scene, bounds, &Theme::default());
    let control = controls.iter().find(|control| control.key == expected["key"].as_str().unwrap()).expect("expected checkpoint").clone();
    let node = seed_scene_window_with(window_id, scene_node);
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained GraphTimeline target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    crate::scenes::stage_graph_timeline_accessibility_controls(host_id, controls);''',
        '''    let node = seed_scene_window_with(window_id, scene_node);
    let retained = retained_scene(window_id, node);
    let label_track = crate::scenes::graph_timeline_scene_label_track(&retained, &Theme::default(), &mut ui_wgpu::wgpu::FontAtlas::shaped_default());
    let controls = crate::scenes::graph_timeline_accessibility_controls(&retained, bounds, &Theme::default(), label_track);
    let control = controls.iter().find(|control| control.key == rebased_scene_key(expected["key"].as_str().unwrap(), host_id, &retained.host_id)).expect("expected checkpoint").clone();
    assert!(crate::scenes::mount_scene_identity(&retained_scene_target(window_id, node).expect("retained GraphTimeline target")));
    let generation = UI_ENGINE.with(|cell| cell.borrow().surface_generation(window_id)).unwrap();
    crate::scenes::stage_graph_timeline_accessibility_controls(&retained.host_id, controls);''',
    ),
    (
        '''    assert!(dispatch_accessibility_event(window_id, generation, button.node_id, "history-host.history.stale", ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(), "a stale key is inert");''',
        '''    assert!(
        dispatch_accessibility_event(window_id, generation, button.node_id, &rebased_scene_key("history-host.history.stale", host_id, &retained.host_id), ui_wgpu::wgpu::AccessibilityUiEvent::Activate, &mut input).is_none(),
        "a stale key is inert"
    );''',
    ),
    (
        '''    crate::scenes::stage_graph_timeline_accessibility_controls(host_id, Vec::new());
    crate::scenes::seal_graph_timeline_accessibility_candidates(923);''',
        '''    crate::scenes::stage_graph_timeline_accessibility_controls(&retained.host_id, Vec::new());
    crate::scenes::seal_graph_timeline_accessibility_candidates(923);''',
    ),
]

SHELL_EDITS = [
    (
        '''/// 🎯️ Rust twin of `navbarCenteredLeftV1`, including its pixel rounding.
pub(crate) fn shell_chrome_centered_band(width: f32, occupied: &[ShellChromeBandSpan], desired_width: f32) -> ShellChromeCenteredBand {
    let free = shell_chrome_free_band(width, occupied);
    let centered_width = desired_width.max(0.0).min(free.width());
    let latest = free.right - centered_width;
    let left = if latest <= free.left { free.left } else { ((width - centered_width) * 0.5).clamp(free.left, latest).round() };''',
        '''/// 🎯️ Rust twin of `navbarCenteredLeftV1`, including its pixel rounding — to the nearest whole pixel INSIDE the band, so a
/// sub-pixel band edge is never crossed (the unrounded clamp when no whole pixel fits).
pub(crate) fn shell_chrome_centered_band(width: f32, occupied: &[ShellChromeBandSpan], desired_width: f32) -> ShellChromeCenteredBand {
    let free = shell_chrome_free_band(width, occupied);
    let centered_width = desired_width.max(0.0).min(free.width());
    let latest = free.right - centered_width;
    let left = if latest <= free.left {
        free.left
    } else {
        let (first, last) = (free.left.ceil(), latest.floor());
        let left = ((width - centered_width) * 0.5).clamp(free.left, latest);
        if first > last {
            left
        } else {
            left.round().clamp(first, last)
        }
    };''',
    ),
    (
        '''                        || id.starts_with(WORLD_PROJECTION_PANE_PARENT)
                        || id.starts_with(WINDOW_UTILITY_RAIL_PARENT)''',
        '''                        || id.starts_with(WORLD_PROJECTION_PANE_PARENT)
                        || id.split_once('/').is_some_and(|(_, local)| local.starts_with(WORLD_PROJECTION_PANE_PARENT))
                        || id.starts_with(WINDOW_UTILITY_RAIL_PARENT)''',
    ),
    (
        '''    /// surface's. [`Self::pointer_press_belongs_to_shell_chrome`] is this answer read as a bool.
    pub fn pointer_hit_owner(''',
        '''    /// surface's. [`Self::pointer_press_belongs_to_shell_chrome`] is this answer read as a bool. A retained pane body's rows are
    /// keyed `<surface>/<id>` (`PanelProjection::key`), so it is their surface-local id that names them chrome.
    pub fn pointer_hit_owner(''',
    ),
]

NAVBAR_EDITS = [
    (
        '''/** 🎯️ Left offset of a centered item: the bar's own centre whenever the item fits there without
 * crossing {@link navbarFreeBandV1}'s edges, and otherwise the nearest position inside the band. An item
 * wider than the band starts at the band's left edge — its own `max-width` is what makes it fit, so this
 * is a fixed point rather than a step that re-measures into a different answer. */
export function navbarCenteredLeftV1(width: number, band: NavbarSpanV1, contentWidth: number): number {
  const latest = band.right - contentWidth;
  if (latest <= band.left) return band.left;
  return Math.round(Math.min(Math.max((width - contentWidth) / 2, band.left), latest));
}''',
        '''/** 🎯️ Left offset of a centered item: the bar's own centre whenever the item fits there without
 * crossing {@link navbarFreeBandV1}'s edges, and otherwise the nearest position inside the band. An item
 * wider than the band starts at the band's left edge — its own `max-width` is what makes it fit, so this
 * is a fixed point rather than a step that re-measures into a different answer. The offset snaps to the
 * nearest whole pixel INSIDE the band, so a sub-pixel band edge is never crossed (the unrounded position
 * when no whole pixel fits). */
export function navbarCenteredLeftV1(width: number, band: NavbarSpanV1, contentWidth: number): number {
  const latest = band.right - contentWidth;
  if (latest <= band.left) return band.left;
  const [first, last] = [Math.ceil(band.left), Math.floor(latest)];
  const left = Math.min(Math.max((width - contentWidth) / 2, band.left), latest);
  return first > last ? left : Math.min(Math.max(Math.round(left), first), last);
}''',
    )
]

PANEL_LAW_EDITS = [
    (
        '''        let layout = shell.footer_chrome_layout(&mut atlas, &theme, width, 0.0, btn_h);
        let ideal = (width - span) * 0.5;
        let expected = ideal.clamp(layout.center.free.left, (layout.center.free.right - span).max(layout.center.free.left));
        assert!((first.2.x - expected).abs() < 0.5, "📑️ {name} uses the free footer band: first={first:?}, last={last:?}, span={span}, expected={expected}");''',
        '''        let layout = shell.footer_chrome_layout(&mut atlas, &theme, width, 0.0, btn_h);
        let free = [ShellChromeBandSpan { left: 0.0, right: layout.center.free.left }, ShellChromeBandSpan { left: layout.center.free.right, right: width }];
        let expected = shell_chrome_centered_band(width, &free, span).centered.left;
        assert!((first.2.x - expected).abs() < 0.01, "📑️ {name} sits where the shared centred-band rule (`navbarCenteredLeftV1`, whole pixels inside the band) places it: first={first:?}, last={last:?}, span={span}, expected={expected}");''',
    )
]

BAND_PLACEMENTS = [
    {
        "name": "a sub-pixel band never lets the rounded placement cross its right edge",
        "width": 1440,
        "band": {"left": 331.85, "right": 745.775},
        "contentWidth": 88.925,
        "left": 656,
    },
    {
        "name": "a sub-pixel band never lets the rounded placement cross its left edge",
        "width": 1000,
        "band": {"left": 700.4, "right": 1000},
        "contentWidth": 200,
        "left": 701,
    },
]

CHECKPOINT_FIXTURE_EDITS = [
    (
        '''    { "id": "graph-select", "region": "graph", "row": 1, "x": 156.5, "action":''',
        '''    { "id": "graph-select", "region": "graph", "row": 1, "x": 160.5, "action":''',
    ),
    (
        '''    { "id": "description-boundary-inert", "region": "description", "row": 1, "x": 157, "action": null },''',
        '''    { "id": "description-boundary-inert", "region": "description", "row": 1, "x": 162.5, "action": null },''',
    ),
]

EDITS = {SVG: SVG_EDITS, SVG_LAWS: SVG_LAW_EDITS, SCENES: SCENES_EDITS, GRAPH_LAWS: GRAPH_LAW_EDITS, WIRING_LAWS: WIRING_EDITS, SHELL: SHELL_EDITS, NAVBAR: NAVBAR_EDITS, PANEL_LAWS: PANEL_LAW_EDITS, CHECKPOINT_FIXTURE: CHECKPOINT_FIXTURE_EDITS}


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def band_fixture_after(source: str) -> str:
    anchor = '''      "contentWidth": 200,
      "left": 700
    }
  ],'''
    if source.count(anchor) != 1:
        sys.exit("navbar band fixture: placements tail anchor moved")
    rows = "".join(
        ",\n    {\n"
        + ",\n".join(f'      "{key}": {json.dumps(value, separators=(", ", ": ")).replace("{", "{ ").replace("}", " }") if isinstance(value, dict) else json.dumps(value, ensure_ascii=False)}' for key, value in row.items())
        + "\n    }"
        for row in BAND_PLACEMENTS
    )
    after = source.replace(anchor, anchor[: -len("\n  ],")] + rows + "\n  ],")
    placements = json.loads(after)["placements"]
    if placements[-len(BAND_PLACEMENTS):] != BAND_PLACEMENTS:
        sys.exit("navbar band fixture: the appended placements do not read back")
    return after


def plans():
    planned = []
    for path, edits in EDITS.items():
        source = path.read_text(encoding="utf-8")
        planned.append((path, source, replaced(path, source, edits)))
    fixture = BAND_FIXTURE.read_text(encoding="utf-8")
    planned.append((BAND_FIXTURE, fixture, band_fixture_after(fixture)))
    return planned


def main():
    files = list(EDITS) + [BAND_FIXTURE]
    if "--revert" in sys.argv:
        for path in files:
            backup = BACKUP / path.relative_to(ROOT)
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print(f"REVERTED: {len(files)} files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    if write:
        for path, before, _ in planned:
            backup = BACKUP / path.relative_to(ROOT)
            backup.parent.mkdir(parents=True, exist_ok=True)
            backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(planned)} files (crates: semio-framework-os-renderer-wgpu; ui TS Navbar + fixture)")


if __name__ == "__main__":
    main()
