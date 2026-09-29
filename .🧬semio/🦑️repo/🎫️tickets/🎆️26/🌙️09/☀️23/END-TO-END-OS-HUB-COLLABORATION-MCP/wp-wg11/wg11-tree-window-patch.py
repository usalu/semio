#!/usr/bin/env python3
"""🪟️ WG11 session 14d — set for the first train after the chain (T6): the wgpu tree window observer (ticket 26/09/16
ARTIFACT-TREE-VIRTUALISED-STREAMING packet P5, coordinator 07:4x "full cut").

Measured (WG11 07:3x): the wgpu target PAINTED a `TreeWindow` (immediate `🌳️Tree` painter only) but never REQUESTED one — the
retained layout priced no spacer at all (`📌️mounted_layout` sums materialised rows only), nothing measured a windowed container,
and the Shell stamped no `ViewModel.tree_windows`/`tree_viewport_rows` (`live_view_state` GAP). So a windowed body (Home's table,
a 10 000-row outline, the Marketplace once it is windowed) showed its first-paint window forever: the rows past it were
unreachable.

This set, the ui-crate half:
1. `🌳️Tree/🪟️window/🦀️.rs` — the Rust twin of React's rule: `treeWindowVisibleRowsForViewport`, `treeWindowRequestsForViewport`,
   `capTreeWindowRequests` (`🌳️Tree/🟦️.tsx`), `treeWindowBodyRequestsV1`, `treeWindowServedRequestsV1`,
   `treeWindowReportSignatureV1` (`🗣️Interpreter/🟦️.tsx`), pure; laws against NEUTRAL vectors React's rule wrote
   (`🌳️Tree/🧫️fixtures/🪟️window-requests/🔣️.json` + schema, and the existing `🧬️contract/🧫️fixtures/🪟️tree-window-served.json`):
   React and wgpu request the same windows for the same viewport; no body answer exceeds the guest ledger; scrolling a
   10 000-row list to its end reaches the last row under any guest capacity; and on the retained engine itself
   (`🖱️ui/🧪️tests/🪟️tree-window-streaming/🦀️.rs`) a 10 000-row windowed table, scrolled to its bottom through
   measure → serve → guest answer → republish, materialises and paints its LAST row inside the viewport. React's own law reads the same vectors
   (`🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx`, Ajv against the schema).
2. ONE spacer geometry: `layout::tree_window_row_extent_px`/`tree_window_spacer_px` (moved from the immediate painter), and the
   retained layout now pitches it: a windowed section/item/table is `lead + rows + trail` tall and its first materialised row
   starts `lead` past the header (`FlowStyle::lead`, a main-axis leading offset the flow arranger honours in both directions).
3. `Ui::tree_window_measures` — every windowed container a surface presents, measured against its scroll viewport exactly as
   `treeWindowContainersUnder` measures the DOM (window path keys, spacer-inclusive extent, real row tops; an up-flow tree is
   measured in mirrored space so its leading spacer and row order read like a down-flow one).

Dry run by default; `--write` backs every edited file up under `.🧬semio/🌐hub/s14-wg11-backup/tree-window/` (a new file's backup
is its absence) and applies; `--revert` restores.
"""

import difflib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
WORK = Path("/Users/ueli" + "/Documents/semio/.tmp-ticket/wp-wg11/p5")
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
WGPU = UI / "🎯️targets/🧊️wgpu"
TREE = UI / "🧱️elements/🌳️Tree"
TARGET_MOD = WGPU / "🦀️.rs"
LAYOUT = WGPU / "🧮️layout/🦀️.rs"
FLEX = WGPU / "📐️flex/🦀️.rs"
FLEX_LAWS = UI / "🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs"
COMPONENT = WGPU / "🧩️component/🦀️.rs"
MOUNTED = WGPU / "📌️mounted_layout/🦀️.rs"
ENGINE = WGPU / "⚙️engine/🦀️.rs"
TREE_TARGET = TREE / "🎯️targets/🧊️wgpu/🦀️.rs"
RULE = TREE / "🪟️window/🦀️.rs"
RULE_LAWS = TREE / "🧪️tests/🪟️window/🦀️.rs"
STREAMING_LAWS = UI / "🧪️tests/🪟️tree-window-streaming/🦀️.rs"
FIXTURE = TREE / "🧫️fixtures/🪟️window-requests/🔣️.json"
SCHEMA = TREE / "🧬️schema/🪟️window-requests/🔣️.json"
TS_LAWS = ROOT / "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx"
BACKUP = ROOT / ".🧬semio/🌐hub/s14-wg11-backup/tree-window"

NEW_FILES = {
    RULE: (WORK / "tree-window.rs").read_text(encoding="utf-8") + '\n#[cfg(test)]\n#[path = "../🧪️tests/🪟️window/🦀️.rs"]\nmod tests;\n',
    RULE_LAWS: (WORK / "tree-window-tests.rs").read_text(encoding="utf-8"),
    STREAMING_LAWS: (WORK / "tree-window-streaming-tests.rs").read_text(encoding="utf-8"),
    FIXTURE: (WORK / "window-requests.json").read_text(encoding="utf-8"),
    SCHEMA: (WORK / "window-requests.schema.json").read_text(encoding="utf-8"),
}

TARGET_MOD_EDITS = [
    (
        '''#[cfg(feature = "wgpu-engine")]
#[path = "../../🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs"]
mod tree_element;
''',
        '''#[cfg(feature = "wgpu-engine")]
#[path = "../../🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs"]
mod tree_element;

/// 🪟️ The tree window request rule — the target-neutral Rust twin of React's (`🌳️Tree/🟦️.tsx` region `🪟️TreeWindow` and the
/// `🗣️Interpreter` observer's body/served rules): one rule, both hosts, the same windows for the same viewport.
#[cfg(feature = "wgpu")]
#[path = "../../🧱️elements/🌳️Tree/🪟️window/🦀️.rs"]
pub mod tree_window;
''',
    )
]

LAYOUT_EDITS = [
    (
        '''use crate::wgpu::component::ui::{UiControlNode, UiTreeActionPlacement, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};''',
        '''use crate::wgpu::component::ui::{UiControlNode, UiTreeActionPlacement, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, UiTreeWindow, UiTreeWindowRowExtent};''',
    ),
    (
        '''    Rect::new(x, 0.0, width, row_height)
}
//#endregion 🌳️TreeRowGeometry''',
        '''    Rect::new(x, 0.0, width, row_height)
}

/// 🪟️ The closed-row pitch one windowed container prices an unmaterialised row at — `treeWindowRowExtentPx`
/// (`🧱️elements/🌳️Tree/🟦️.tsx`): the ONE geometry the immediate painter's spacer bands, the retained layout's spacers and the
/// tree window observer's measurement share.
pub fn tree_window_row_extent_px(extent: UiTreeWindowRowExtent) -> f32 {
    match extent {
        UiTreeWindowRowExtent::Standard => crate::wgpu::widgets::TREE_ROW_HEIGHT,
        UiTreeWindowRowExtent::CompactText => crate::wgpu::chrome::SIZE_TINY * 1.5,
        UiTreeWindowRowExtent::CompactSmallControl => (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::CONTROL_HEIGHT_SMALL_UI_SPACING) as f32,
        UiTreeWindowRowExtent::CompactControl => (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::CONTROL_HEIGHT_UI_SPACING) as f32,
    }
}

/// 🪟️ The `(leading, trailing)` spacer pitch one windowed container paints around its `materialised` children: `offset`
/// unmaterialised rows before them and `total − offset − materialised` after, each [`tree_window_row_extent_px`] tall. An
/// unwindowed container (`None`) pitches nothing. There is no `+N` continuation row — the spacer IS the unloaded rows, which
/// is what makes the scrollbar span the whole list and a scroll land on the real row index.
pub fn tree_window_spacer_px(window: Option<&UiTreeWindow>, materialised: usize) -> (f32, f32) {
    window.map_or((0.0, 0.0), |window| {
        let pitch = tree_window_row_extent_px(window.row_extent);
        (window.leading_rows() as f32 * pitch, window.trailing_rows(materialised) as f32 * pitch)
    })
}
//#endregion 🌳️TreeRowGeometry''',
    ),
]

TREE_TARGET_EDITS = [
    (
        '''use crate::wgpu::component::ui::{UiTreeWindow, UiTreeWindowRowExtent};
use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};
''',
        '''use crate::wgpu::geometry::Rect;
use crate::wgpu::input::{DragAxis, HitKind, HitTarget};
use crate::wgpu::layout::tree_window_spacer_px;
''',
    ),
    (
        '''/// 🪟️ The `(leading, trailing)` spacer pitch one windowed container paints around its materialised
/// children: `offset` unmaterialised rows before them and `total − offset − materialised` after, each
/// priced at exactly one [`TREE_ROW_HEIGHT`]. An unwindowed container (`None`) pitches nothing, so
/// every non-virtualised tree keeps its existing extent to the float. There is deliberately no `+N`
/// continuation row — the spacer IS the representation of the unloaded rows, which is what makes the
/// scrollbar span the whole document rather than the loaded window.
///
/// ⚠️ GAP (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING packet P5): the wgpu target paints a
/// window but never requests one. React's `🗣️Interpreter` mounts a viewport observer that reports
/// `TreeWindowRequest`s back through `ViewModel.tree_windows`; the wgpu shell has no equivalent, so a
/// wgpu tree shows only whatever first-paint window its guest chose and scrolling into a spacer band
/// reveals empty pitch, not streamed rows. Closing that needs a wgpu-side scroll/open observer
/// feeding the same `ViewModel` field — out of scope for this packet.
fn tree_window_row_extent(extent: UiTreeWindowRowExtent) -> f32 {
    match extent {
        UiTreeWindowRowExtent::Standard => TREE_ROW_HEIGHT,
        UiTreeWindowRowExtent::CompactText => crate::wgpu::chrome::SIZE_TINY * 1.5,
        UiTreeWindowRowExtent::CompactSmallControl => (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::CONTROL_HEIGHT_SMALL_UI_SPACING) as f32,
        UiTreeWindowRowExtent::CompactControl => (ui_styling::metrics::chrome::UI_SPACING_COMPACT_PX * ui_styling::metrics::chrome::CONTROL_HEIGHT_UI_SPACING) as f32,
    }
}

fn tree_window_pitch(window: Option<&UiTreeWindow>, materialised: usize) -> (f32, f32) {
    window.map_or((0.0, 0.0), |window| {
        let extent = tree_window_row_extent(window.row_extent);
        (window.leading_rows() as f32 * extent, window.trailing_rows(materialised) as f32 * extent)
    })
}

''',
        '',
    ),
]

FLEX_EDITS = [
    (
        '''    TreeRow {
        row: f32,
        height: f32,
        expanded: bool,
        reversed: bool,
    },''',
        '''    TreeRow {
        row: f32,
        height: f32,
        expanded: bool,
        reversed: bool,
        /// 🪟️ The leading spacer of the windowed container this row is the FIRST materialised child of (else `0`).
        lead: f32,
    },''',
    ),
    (
        '''    TableRow {
        height: f32,
        actions: f32,
    },''',
        '''    TableRow {
        height: f32,
        actions: f32,
        /// 🪟️ The windowed table's leading spacer when this row is its first materialised row (else `0`).
        lead: f32,
    },''',
    ),
    (
        '''    pub grows_children: bool,
    pub reverse: bool,
}''',
        '''    pub grows_children: bool,
    pub reverse: bool,
    /// 🪟️ Main-axis empty pitch this child claims BEFORE itself in its parent's flow (after it, in a reversed flow) — a
    /// windowed container's leading spacer, carried by its first materialised row. `0` for every other node.
    pub lead: f32,
}''',
    ),
    (
        '''            grows_children: false,
            reverse: false,
        }''',
        '''            grows_children: false,
            reverse: false,
            lead: 0.0,
        }''',
    ),
    (
        '''            let (main, cross) = if flow.row { (width, height) } else { (height, width) };
            main_sum += main;''',
        '''            let (main, cross) = if flow.row { (width, height) } else { (height, width) };
            main_sum += main + child_flow.lead;''',
    ),
    (
        '''            let (main, cross) = child_flow.flow_size(flow, content_main, content_cross, intrinsic, child, measure);
            base_sum += main;''',
        '''            let (main, cross) = child_flow.flow_size(flow, content_main, content_cross, intrinsic, child, measure);
            base_sum += main + child_flow.lead;''',
    ),
    (
        '''                if flow.reverse {
                    cursor -= main;
                }''',
        '''                if flow.reverse {
                    cursor -= main + child_flow.lead;
                } else {
                    cursor += child_flow.lead;
                }''',
    ),
    (
        '''        LayoutNodeKind::TreeRow { row, height, reversed, .. } => band(height, row, reversed),''',
        '''        LayoutNodeKind::TreeRow { row, height, reversed, lead, .. } => FlowStyle { lead, ..band(height, row, reversed) },''',
    ),
    (
        '''        LayoutNodeKind::TableRow { height, actions } => {
            let (left, right) = if metrics.inline.is_rtl() { (actions, metrics.gap) } else { (metrics.gap, actions) };
            FlowStyle { row: true, reverse: metrics.inline.is_rtl(), gap_main: metrics.gap, align: Align::Center, height: Dim::Length(height), shrink: 0.0, padding: EdgePx { left, right, ..EdgePx::default() }, clips: true, ..FlowStyle::default() }
        }''',
        '''        LayoutNodeKind::TableRow { height, actions, lead } => {
            let (left, right) = if metrics.inline.is_rtl() { (actions, metrics.gap) } else { (metrics.gap, actions) };
            FlowStyle {
                row: true,
                reverse: metrics.inline.is_rtl(),
                gap_main: metrics.gap,
                align: Align::Center,
                height: Dim::Length(height),
                shrink: 0.0,
                padding: EdgePx { left, right, ..EdgePx::default() },
                clips: true,
                lead,
                ..FlowStyle::default()
            }
        }''',
    ),
]

COMPONENT_EDITS = [
    (
        '''    /// `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`.
    ///
    /// ⚠️ GAP (ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING packet P5): the wgpu target
    /// RENDERS a window but never REQUESTS one. React's host owns scroll/open state and reports it
    /// back through `ViewModel.tree_windows`; the wgpu shell has no equivalent observer, so a wgpu
    /// tree only ever shows the first-paint window its guest chose. There is deliberately no `+N`
    /// continuation-row fallback.
''',
        '''    /// `🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`. The retained layout pitches the unmaterialised rows as spacers
    /// (`layout::tree_window_spacer_px`), `Ui::tree_window_measures` reads every windowed container against its scroll
    /// viewport, and the one request rule (`🧱️elements/🌳️Tree/🪟️window/🦀️.rs`, React's `treeWindowServedRequestsV1`) asks
    /// back the rows the viewport needs — the same `ViewModel.tree_windows` React's host reports. There is deliberately no
    /// `+N` continuation-row fallback.
''',
    ),
]

FLEX_LAW_EDITS = [
    (
        '''        rows.push(fixture.push(LayoutNodeKind::TreeRow { row, height: row, expanded: false, reversed: false }, Some(section), None));''',
        '''        rows.push(fixture.push(LayoutNodeKind::TreeRow { row, height: row, expanded: false, reversed: false, lead: 0.0 }, Some(section), None));''',
    ),
    (
        '''    let row = fixture.push(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed: false }, Some(section), None);
    fixture.solve(320.0, header);
    assert!(close(fixture.rect(toolbar).height, header));
    assert!(fixture.rect(execute).width > 0.0 && fixture.rect(execute).height > 0.0, "the real header Button remains measurable and hittable");
    assert!(close(fixture.rect(row).height, 0.0), "closed form rows stay collapsed");
}
''',
        '''    let row = fixture.push(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed: false, lead: 0.0 }, Some(section), None);
    fixture.solve(320.0, header);
    assert!(close(fixture.rect(toolbar).height, header));
    assert!(fixture.rect(execute).width > 0.0 && fixture.rect(execute).height > 0.0, "the real header Button remains measurable and hittable");
    assert!(close(fixture.rect(row).height, 0.0), "closed form rows stay collapsed");
}

/// 🪟️ LAW (ticket 26/09/23 session 14d, WG11 P5): a windowed section's FIRST materialised row carries the leading spacer — it
/// starts `lead` past the header, its successors keep the row pitch, and the section band spans header + lead + rows + trail, so
/// the scroll extent covers every unmaterialised row. In an up-flow (reversed) section the spacer sits between the header and
/// the first row on the bottom side, exactly mirrored.
#[test]
fn a_windowed_rows_lead_pitches_the_unmaterialised_rows_before_it_in_both_flows() {
    let metrics = TreeRowMetrics::from_theme(&crate::wgpu::theme::Theme::default());
    let row = metrics.row_height;
    let (lead, trail) = (row * 40.0, row * 10.0);
    let height = row + lead + row * 3.0 + trail;
    for reversed in [false, true] {
        let mut fixture = Fixture::new();
        let tree = fixture.push(LayoutNodeKind::Tree { height, header: 0.0, reversed }, None, None);
        let section = fixture.push(LayoutNodeKind::TreeSection { header: row, height, expanded: true, reversed }, Some(tree), None);
        let rows: Vec<usize> = (0..3).map(|index| fixture.push(LayoutNodeKind::TreeRow { row, height: row, expanded: false, reversed, lead: if index == 0 { lead } else { 0.0 } }, Some(section), None)).collect();
        fixture.solve(320.0, height);
        let band = fixture.rect(section);
        assert!(close(band.height, height), "the band spans header + lead + rows + trail: {band:?}");
        let (first, second) = (fixture.rect(rows[0]), fixture.rect(rows[1]));
        if reversed {
            assert!(close(band.y + band.height - (first.y + first.height), row + lead), "up-flow: the spacer sits between the bottom header and the first row: {first:?} in {band:?}");
            assert!(close(first.y - second.y, row), "up-flow successors keep the row pitch upwards");
        } else {
            assert!(close(first.y - band.y, row + lead), "down-flow: the first row starts header + lead into the band: {first:?} in {band:?}");
            assert!(close(second.y - first.y, row), "successors keep the row pitch");
        }
    }
}
''',
    ),
]

MOUNTED_EDITS = [
    (
        '''use crate::wgpu::layout::{gap_for_token, padding_for_token, tree_section_header_height, TreeRowMetrics, TREE_ROW_MAX_DEPTH};''',
        '''use crate::wgpu::layout::{gap_for_token, padding_for_token, tree_section_header_height, tree_window_row_extent_px, tree_window_spacer_px, TreeRowMetrics, TREE_ROW_MAX_DEPTH};''',
    ),
    (
        '''use crate::wgpu::tree::{AcceptedLayout, NodeFlags, NodeKey, UiTree};
''',
        '''use crate::wgpu::tree::{AcceptedLayout, NodeFlags, NodeKey, UiTree};
use crate::wgpu::tree_window::{TreeWindowContainerMeasure, TreeWindowRowMeasure};
''',
    ),
    (
        '''            if document_table(tree, tree.node(id)?.parent?).is_some() {
                let item = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0))?;
                return Some(LayoutNodeKind::TableRow { height: live_tree_item_height(tree, id, item, &metrics, 0), actions: table_actions_width_of(owner, &metrics) });
            }''',
        '''            if document_table(tree, tree.node(id)?.parent?).is_some() {
                let item = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0))?;
                let lead = owner.sections.first().map_or(0.0, |section| window_lead(section.window.as_ref(), section.items.first(), key));
                return Some(LayoutNodeKind::TableRow { height: live_tree_item_height(tree, id, item, &metrics, 0), actions: table_actions_width_of(owner, &metrics), lead });
            }''',
    ),
    (
        '''            if matches!(parent_kind, LayoutNodeKind::TreeSection { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed });
            }
            if matches!(parent_kind, LayoutNodeKind::TreeRow { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed });
            }
            let height = live_tree_item_height(tree, id, item, &metrics, 0);
            let expanded = height > 0.0 && tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) && item.items.as_deref().is_some_and(|items| !items.is_empty());
            Some(LayoutNodeKind::TreeRow { row: if expanded { metrics.row_height } else { 0.0 }, height, expanded, reversed })''',
        '''            if matches!(parent_kind, LayoutNodeKind::TreeSection { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed, lead: 0.0 });
            }
            if matches!(parent_kind, LayoutNodeKind::TreeRow { expanded: false, .. }) {
                return Some(LayoutNodeKind::TreeRow { row: 0.0, height: 0.0, expanded: false, reversed, lead: 0.0 });
            }
            let height = live_tree_item_height(tree, id, item, &metrics, 0);
            let expanded = height > 0.0 && tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) && tree_item_has_rows(item);
            let lead = row_window_lead(tree, id, key, owner);
            Some(LayoutNodeKind::TreeRow { row: if expanded { metrics.row_height } else { 0.0 }, height, expanded, reversed, lead })''',
    ),
    (
        '''    if depth >= TREE_ROW_MAX_DEPTH || !tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) {
        return height;
    }
    for child in item.items.iter().flatten() {''',
        '''    if depth >= TREE_ROW_MAX_DEPTH || !tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) {
        return height;
    }
    let (lead, trail) = tree_window_spacer_px(item.window.as_ref(), item.items.as_ref().map_or(0, Vec::len));
    height += lead + trail;
    for child in item.items.iter().flatten() {''',
    ),
    (
        '''    header + section.items.iter().filter_map(|item| tree.explicit_child(id, &item.id).map(|item_id| live_tree_item_height(tree, item_id, item, metrics, 0))).sum::<f32>()
}''',
        '''    let (lead, trail) = tree_window_spacer_px(section.window.as_ref(), section.items.len());
    header + lead + trail + section.items.iter().filter_map(|item| tree.explicit_child(id, &item.id).map(|item_id| live_tree_item_height(tree, item_id, item, metrics, 0))).sum::<f32>()
}

/// 🪟️ Whether a row folds open onto rows: materialised children, or a window declaring rows not materialised yet
/// (`total > 0` with no children is expandable-but-not-yet-streamed, never a leaf).
fn tree_item_has_rows(item: &UiTreeItemNode) -> bool {
    item.items.as_deref().is_some_and(|items| !items.is_empty()) || item.window.is_some_and(|window| window.total > 0)
}

/// 🪟️ The leading spacer a windowed container places before `key`, when `key` is its FIRST materialised child.
fn window_lead(window: Option<&crate::wgpu::component::ui::UiTreeWindow>, first: Option<&UiTreeItemNode>, key: &str) -> f32 {
    match (window, first) {
        (Some(window), Some(first)) if first.id == key => tree_window_spacer_px(Some(window), 0).0,
        _ => 0.0,
    }
}

/// 🪟️ The leading spacer before the retained row `id` (authored key `key`): its parent is a windowed section or item.
fn row_window_lead(tree: &UiTree, id: NodeId, key: &str, owner: &UiTreeNode) -> f32 {
    let Some(NodeKey::Explicit(parent_key)) = tree.node(id).and_then(|node| node.parent).and_then(|parent| tree.node(parent)).map(|parent| &parent.key) else { return 0.0 };
    if let Some(section) = owner.sections.iter().find(|section| &section.id == parent_key) {
        return window_lead(section.window.as_ref(), section.items.first(), key);
    }
    owner.sections.iter().find_map(|section| find_tree_item(&section.items, parent_key, 0)).map_or(0.0, |parent| window_lead(parent.window.as_ref(), parent.items.as_deref().and_then(<[UiTreeItemNode]>::first), key))
}''',
    ),
    (
        '''    if document_table(tree, id).is_some() {
        return metrics.header_height + node.sections.iter().flat_map(|section| section.items.iter()).filter_map(|item| tree.explicit_child(id, &item.id).map(|row| live_tree_item_height(tree, row, item, &metrics, 0))).sum::<f32>();
    }''',
        '''    if document_table(tree, id).is_some() {
        let (lead, trail) = node.sections.first().map_or((0.0, 0.0), |section| tree_window_spacer_px(section.window.as_ref(), section.items.len()));
        return metrics.header_height + lead + trail + node.sections.iter().flat_map(|section| section.items.iter()).filter_map(|item| tree.explicit_child(id, &item.id).map(|row| live_tree_item_height(tree, row, item, &metrics, 0))).sum::<f32>();
    }''',
    ),
    (
        '''/// 🧩️ One admitted node's layout identity:''',
        '''//#region 🪟️TreeWindowMeasure
/// 🪟️ Node visits one measurement pass may spend walking to a surface's trees — the surface's own document ceiling several
/// times over; a tree's rows are walked through its spec, not the arena.
const TREE_WINDOW_MEASURE_NODES: usize = 4 * ui_contract::UI_DOCUMENT_NODES;

/// 🪟️ Every windowed container of `tree`, measured against the scroll viewport its first `Tree` paints in (content origin 0)
/// — the wgpu twin of `treeWindowContainersUnder` (`🗣️Interpreter/🟦️.tsx`): the key is the container's WINDOW PATH (enclosing
/// windowed containers' keys, outermost first, joined by `TREE_WINDOW_PATH_SEPARATOR`), the extent spans its spacers, rows and
/// nested content, and the rows carry their real tops. An up-flow (`reversed`) tree is measured in MIRRORED viewport space, so
/// its leading spacer and its row order read exactly like a down-flow list's. `None` when nothing is windowed.
pub(crate) fn tree_window_measures(tree: &UiTree, theme: &Theme, viewport: (f32, f32), reversed: bool) -> Option<(f64, Vec<TreeWindowContainerMeasure>)> {
    let root = tree.root?;
    let metrics = TreeRowMetrics::from_theme(theme);
    let mut frame = None;
    let mut containers = Vec::new();
    let mut pending = vec![root];
    let mut visits = 0usize;
    while let Some(id) = pending.pop() {
        visits += 1;
        if visits > TREE_WINDOW_MEASURE_NODES {
            break;
        }
        let Some(node) = tree.node(id) else { continue };
        if let UiNode::Tree(owner) = &node.spec.0 {
            let frame = *frame.get_or_insert_with(|| scroll_frame_of(tree, id).unwrap_or(crate::wgpu::geometry::Rect::new(0.0, 0.0, viewport.0, viewport.1)));
            measure_tree_windows(tree, id, owner, &metrics, frame, reversed, &mut containers);
            continue;
        }
        let children: Vec<NodeId> = tree.children(id).collect();
        pending.extend(children.into_iter().rev());
    }
    let frame = frame?;
    (!containers.is_empty()).then(|| (f64::from(frame.h), containers))
}

/// 🪟️ The painted box of the nearest scrolling ancestor of `id` — the viewport its rows scroll through.
fn scroll_frame_of(tree: &UiTree, id: NodeId) -> Option<crate::wgpu::geometry::Rect> {
    let mut cursor = tree.node(id)?.parent;
    while let Some(ancestor) = cursor {
        let node = tree.node(ancestor)?;
        if node.flags.contains(NodeFlags::SCROLLABLE) {
            return tree.absolute_rect(ancestor);
        }
        cursor = node.parent;
    }
    None
}

fn measure_tree_windows(tree: &UiTree, id: NodeId, owner: &UiTreeNode, metrics: &TreeRowMetrics, frame: crate::wgpu::geometry::Rect, reversed: bool, out: &mut Vec<TreeWindowContainerMeasure>) {
    let metrics = metrics.with_presentation(owner.presentation);
    if document_table(tree, id).is_some() {
        let Some(section) = owner.sections.first() else { return };
        if let (Some(window), Some(rect)) = (section.window.as_ref().filter(|window| window.total > 0), tree.absolute_rect(id)) {
            let rows: Vec<NodeId> = section.items.iter().filter_map(|item| tree.explicit_child(id, &item.id)).collect();
            push_window_measure(tree, &section.id, window, section.items.len(), rect, metrics.header_height, &rows, frame, reversed, out);
        }
        return;
    }
    for section in owner.sections.iter().filter(|section| section.presence.visible()) {
        let Some(section_id) = tree.explicit_child(id, &section.id) else { continue };
        if !tree.disclosure_open(section_id).unwrap_or_else(|| crate::wgpu::layout::tree_section_default_open(section)) {
            continue;
        }
        if let (Some(window), Some(rect)) = (section.window.as_ref().filter(|window| window.total > 0), tree.absolute_rect(section_id)) {
            let rows: Vec<NodeId> = section.items.iter().filter_map(|item| tree.explicit_child(section_id, &item.id)).collect();
            push_window_measure(tree, &section.id, window, section.items.len(), rect, tree_section_header_height(section, &metrics), &rows, frame, reversed, out);
        }
        let path = section.window.is_some().then_some(section.id.as_str());
        for item in &section.items {
            if let Some(item_id) = tree.explicit_child(section_id, &item.id) {
                measure_item_windows(tree, item_id, item, path, frame, reversed, 0, out);
            }
        }
    }
}

#[allow(clippy::too_many_arguments, reason = "one recursion frame of the measurement walk: the node, its spec, the path so far and the shared viewport")]
fn measure_item_windows(tree: &UiTree, id: NodeId, item: &UiTreeItemNode, parent_path: Option<&str>, frame: crate::wgpu::geometry::Rect, reversed: bool, depth: usize, out: &mut Vec<TreeWindowContainerMeasure>) {
    if depth >= TREE_ROW_MAX_DEPTH || !item.presence.visible() || !tree.disclosure_open(id).unwrap_or(item.default_open.unwrap_or(false)) {
        return;
    }
    let children = item.items.as_deref().unwrap_or_default();
    let path = item.window.is_some().then(|| match parent_path {
        Some(parent) => format!("{parent}{}{}", ui_contract::TREE_WINDOW_PATH_SEPARATOR, item.id),
        None => item.id.clone(),
    });
    if let (Some(window), Some(path), Some(rect)) = (item.window.as_ref().filter(|window| window.total > 0), path.as_deref(), tree.absolute_rect(id)) {
        let rows: Vec<NodeId> = children.iter().filter_map(|child| tree.explicit_child(id, &child.id)).collect();
        let (lead, trail) = tree_window_spacer_px(Some(window), children.len());
        let nested = lead + trail + rows.iter().filter_map(|row| tree.absolute_rect(*row)).map(|row| row.h).sum::<f32>();
        push_window_measure(tree, path, window, children.len(), rect, (rect.h - nested).max(0.0), &rows, frame, reversed, out);
    }
    let next = path.as_deref().or(parent_path);
    for child in children {
        if let Some(child_id) = tree.explicit_child(id, &child.id) {
            measure_item_windows(tree, child_id, child, next, frame, reversed, depth + 1, out);
        }
    }
}

/// 🪟️ One container's measure: its content band below its own chrome (`band`: a section's header, a table's column header, an
/// item's own row chrome), in viewport space — mirrored for an up-flow tree, where the chrome sits BELOW the rows.
#[allow(clippy::too_many_arguments, reason = "one container's whole geometry, gathered by the two walks above")]
fn push_window_measure(
    tree: &UiTree,
    key: &str,
    window: &crate::wgpu::component::ui::UiTreeWindow,
    length: usize,
    rect: crate::wgpu::geometry::Rect,
    band: f32,
    rows: &[NodeId],
    frame: crate::wgpu::geometry::Rect,
    reversed: bool,
    out: &mut Vec<TreeWindowContainerMeasure>,
) {
    let height = f64::from((rect.h - band).max(0.0));
    let place = |y: f32, h: f32| if reversed { f64::from(frame.h) - (f64::from(y - frame.y) + f64::from(h)) } else { f64::from(y - frame.y) };
    let top = if reversed { place(rect.y, rect.h - band) } else { place(rect.y + band, rect.h - band) };
    let mut measured: Vec<TreeWindowRowMeasure> =
        rows.iter().enumerate().filter_map(|(position, row)| tree.absolute_rect(*row).map(|at| TreeWindowRowMeasure { index: window.offset.saturating_add(position as u32), top: place(at.y, at.h) })).collect();
    measured.sort_by(|left, right| left.top.total_cmp(&right.top).then(left.index.cmp(&right.index)));
    out.push(TreeWindowContainerMeasure {
        key: key.to_owned(),
        total: window.total,
        offset: window.offset,
        length: u32::try_from(length).unwrap_or(u32::MAX),
        top,
        height,
        row_px: f64::from(tree_window_row_extent_px(window.row_extent)),
        rows: measured,
    });
}
//#endregion 🪟️TreeWindowMeasure

#[cfg(test)]
#[path = "../../../🧪️tests/🪟️tree-window-streaming/🦀️.rs"]
mod tree_window_streaming_tests;

/// 🧩️ One admitted node's layout identity:''',
    ),
]

ENGINE_EDITS = [
    (
        '''    pub fn viewport(&self, window_id: &str) -> Option<(f32, f32)> {
        self.windows.get(window_id).map(|window| window.viewport)
    }
''',
        '''    pub fn viewport(&self, window_id: &str) -> Option<(f32, f32)> {
        self.windows.get(window_id).map(|window| window.viewport)
    }

    /// 🪟️ Every windowed Tree container `window_id` presents, measured against the scroll viewport it paints in — the host half
    /// of the tree window observer (`mounted_layout::tree_window_measures`, the twin of React's `treeWindowContainersUnder`):
    /// `(viewport height, containers)`, or `None` when the surface presents no windowed container.
    pub fn tree_window_measures(&self, window_id: &str) -> Option<(f64, Vec<crate::wgpu::tree_window::TreeWindowContainerMeasure>)> {
        let window = self.windows.get(window_id).filter(|window| window.closing.is_none())?;
        let tree = if window.presented_ready { &window.presented_tree } else { &window.tree };
        crate::wgpu::mounted_layout::tree_window_measures(tree, &self.theme, window.viewport, window.router.flow().block == ui_contract::FlowBlock::Up)
    }
''',
    )
]

TS_LAW_EDITS = [
    (
        '''  describe("🧠️ a window the guest answers short", () => {''',
        '''  const requestsFixture = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧫️fixtures/🪟️window-requests/🔣️.json"), "utf8"));
  const requestsSchema = JSON.parse(readFileSync(join(dirname(fileURLToPath(source.url)), "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🧬️schema/🪟️window-requests/🔣️.json"), "utf8"));
  const { default: Ajv } = await import("ajv");
  const { TREE_WINDOW_OVERSCAN_ROWS, treeWindowRequestsForViewport, treeWindowVisibleRowsForViewport } = await import("@semio-tech/ui-react");

  describe("🪟️ the neutral viewport vectors both hosts answer (WG11 P5)", () => {
    it("the vectors satisfy their schema and were written against this host's body budget", () => {
      const validate = new Ajv({ allErrors: true, strict: true }).compile(requestsSchema);
      expect(validate(requestsFixture), JSON.stringify(validate.errors)).toBe(true);
      expect(requestsFixture.budget).toBe(TREE_WINDOW_BODY_NODE_BUDGET);
      expect(requestsFixture.overscan).toBe(TREE_WINDOW_OVERSCAN_ROWS);
    });
    for (const law of requestsFixture.cases) {
      it(law.name, () => {
        const containers = law.containers.map((container: AnyRecord) => ({ ...container, rows: container.rows.map(([index, top]: [number, number]) => ({ index, top })) }));
        expect([...treeWindowVisibleRowsForViewport(containers, 0, law.viewportHeight).entries()].map(([key, rows]: [string, AnyRecord]) => ({ key, ...rows }))).toEqual(law.visible);
        expect(treeWindowRequestsForViewport(containers, 0, law.viewportHeight, TREE_WINDOW_OVERSCAN_ROWS)).toEqual(law.requests);
        expect(treeWindowBodyRequestsV1(containers, law.viewportHeight)).toEqual(law.body);
      });
    }
  });

  describe("🧠️ a window the guest answers short", () => {''',
    )
]

EDITS = {
    TARGET_MOD: TARGET_MOD_EDITS,
    LAYOUT: LAYOUT_EDITS,
    TREE_TARGET: TREE_TARGET_EDITS,
    FLEX: FLEX_EDITS,
    FLEX_LAWS: FLEX_LAW_EDITS,
    COMPONENT: COMPONENT_EDITS,
    MOUNTED: MOUNTED_EDITS,
    ENGINE: ENGINE_EDITS,
    TS_LAWS: TS_LAW_EDITS,
}


def replaced(path: Path, source: str, edits) -> str:
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def plans():
    for path in NEW_FILES:
        if path.exists():
            sys.exit(f"{path.relative_to(ROOT)} exists already — landed already")
    json.loads(NEW_FILES[FIXTURE])
    json.loads(NEW_FILES[SCHEMA])
    planned = [(path, None, content) for path, content in NEW_FILES.items()]
    for path, edits in EDITS.items():
        source = path.read_text(encoding="utf-8")
        planned.append((path, source, replaced(path, source, edits)))
    after = dict((path, content) for path, _, content in planned)
    if "tree_window_pitch(" in after[TREE_TARGET].replace("fn tree_window_pitch", ""):
        after[TREE_TARGET] = after[TREE_TARGET].replace("tree_window_pitch(", "tree_window_spacer_px(")
        planned = [(path, before, after[path]) for path, before, _ in planned]
    return planned


def main():
    if "--revert" in sys.argv:
        for path in list(NEW_FILES) + list(EDITS):
            backup = BACKUP / path.relative_to(ROOT)
            if path in NEW_FILES:
                if path.exists():
                    path.unlink()
                continue
            if not backup.exists():
                sys.exit(f"no backup for {path.relative_to(ROOT)}")
            shutil.copyfile(backup, path)
        print("REVERTED: new files removed, edited files restored from backups")
        return
    write = "--write" in sys.argv
    planned = plans()
    for path, before, after in planned:
        sys.stdout.writelines(difflib.unified_diff((before or "").splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=0))
    if write:
        for path, before, _ in planned:
            if before is not None:
                backup = BACKUP / path.relative_to(ROOT)
                backup.parent.mkdir(parents=True, exist_ok=True)
                backup.write_bytes(before.encode("utf-8"))
        for path, _, after in planned:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(after, encoding="utf-8")
    print(f"\n{'WRITTEN' if write else 'DRY RUN'}: {len(NEW_FILES)} new + {len(EDITS)} edited files (crate: semio-framework-ui; TS law: React renderer engine 🗣️Interpreter)")


if __name__ == "__main__":
    main()
