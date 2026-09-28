#!/usr/bin/env python3
"""📊️ WG11 session 14c — prepared patch for window 3: the wgpu TableRow painter (the renderer half of the WG11 × LB2 agreement;
lands TOGETHER with LB2's SDK half `wp-lb2/lb2-p3-row-actions.py`, which drops the SDK's `row-action-<i>` child buttons).

Measured (tree, 2026-09-28): `🔀️reconcile` painted a `Table` as a bare vertical Stack (no header, no window), a `TableRow` with
children as a bare horizontal Stack (cells dropped) and a childless `TableRow` as ONE Button labelled `cells.join(" · ")`;
`TableRowProps.row_actions` were never painted, and no retained tree row action was ever clickable or announced (paint drew the
icons, no hit path or accessibility node existed). Hence the SDK duplicated every row action as a child button, and Home's
32-row window ran out of its item budget after 27 rows (S18, session 14b).

Contract (existing `TableProps`/`TableRowProps`, no schema change):
  1. `Table` -> the retained `UiNode::Tree` with ONE section keyed by the table's own record key (`window` = `Table.window`); a table
     has no section record, so its rows mount directly under the table node. Every `TableRow` -> a tree item (named by its first
     cell, activated by its record's `Trigger::Activate`, its `RowAction`s the item's trailing actions) and its record mounts as the
     keyed identity row the tree arm already uses. The column grid stays on the document records (`TableProps.columns` /
     `actions_label`, `TableRowProps.cells`) and is read through the node binding (`mounted_layout::document_table`).
  2. ONE grid: `layout::table_column_rect` (equal columns between the leading gap and the trailing actions column) is the header's,
     the painted cells' and the editable row's cell children's (`flex` `LayoutNodeKind::TableRow`: cells flow in equal columns).
     A plain row paints `cells[i]`; an editable row's child i (input, paged draft, read-only surface) paints itself in column i.
  3. ONE action geometry: `layout::tree_row_action_rect` — paint draws the icons there, the pointer router fires the action there
     (`EventRouter::pointer_row_action`), with the action's own versioned binding (`mounted_layout::document_row_action`), never the
     row's `Trigger::Activate`; the accessibility projection announces every Row-placed action as a virtual
     `<rowKey>::row-action::<i>` button (`"<label>: <row name>"`, React's TableView naming) that the mirror activates. Tree rows get
     the same pointer + accessibility path (their icons were painted but dead).
Laws: `🧪️tests/📊️table-row-grid` over the new fixture `🧫️fixtures/📊️table-row-grid` (metrics, rows/columns/editable cells, action
slots, clicks, painted glyph columns, accessibility buttons + activation) + a taffy oracle for the editable row's cells; the two
old reconcile laws re-pinned to the new projection; the flex unit literals gain `header: 0.0`.

Dry run by default; `--apply` writes (every anchor asserted exactly once; refuses when a new file exists); `--revert` restores.
"""

import difflib
import json
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
UI = ROOT / "🧰️framework/🔨️modules/🖱️ui"
WGPU = UI / "🎯️targets/🧊️wgpu"
RECONCILE = WGPU / "🔀️reconcile/🦀️.rs"
MOUNTED = WGPU / "📌️mounted_layout/🦀️.rs"
FLEX = WGPU / "📐️flex/🦀️.rs"
LAYOUT = WGPU / "🧮️layout/🦀️.rs"
PAINT = WGPU / "🖌️paint/🦀️.rs"
EVENTS = WGPU / "⚡️events/🦀️.rs"
ACCESSIBILITY = WGPU / "♿️accessibility/🦀️.rs"
ENGINE = WGPU / "⚙️engine/🦀️.rs"
RECONCILE_LAWS = UI / "🧪️tests/🔬️targets-wgpu-reconcile-unit/🦀️.rs"
FLEX_LAWS = UI / "🧪️tests/🔬️targets-wgpu-flex-unit/🦀️.rs"
GRID_LAWS = UI / "🧪️tests/📊️table-row-grid/🦀️.rs"
GRID_FIXTURE = UI / "🧫️fixtures/📊️table-row-grid/🔣️.json"

EDITS = {
    RECONCILE: [
        (
            """/// 🌳️ Assembles one `Component::TreeItem` record and its whole subtree into the inline""",
            """/// 📊️ A `Component::Table` as the ONE section of the retained `Tree` it paints through, keyed by the table's own record key:
/// a table has no section record, so its rows mount directly under the table node, and `TableProps::window` windows it
/// exactly as it windows a tree section. The column grid stays on the document — `TableProps::columns`/`actions_label` and
/// every row's `TableRowProps::cells` are read through the node binding (`mounted_layout::document_table`) by the layout, the
/// painter, the pointer router and the accessibility projection, so no second copy of a cell travels in the retained spec.
fn table_section(document: &UiDocumentTree, record: &UiNodeRecord, props: &ui_contract::TableProps, controller: &str) -> UiTreeSectionNode {
    UiTreeSectionNode {
        header_toolbar: None,
        window: tree_window(props.window.as_ref()),
        id: record.key.as_str().to_string(),
        label: None,
        default_open: Some(true),
        presence: UiPresence::default(),
        items: record.children.iter().filter_map(|child| document.record(*child)).filter_map(|row| table_row_item(row, controller)).collect(),
    }
}

/// 📊️ One `Component::TableRow` as its tree item: named by its first cell (React's row name), activated by the record's own
/// `Trigger::Activate` binding, its `RowAction`s the item's trailing actions — the one representation every target paints.
fn table_row_item(record: &UiNodeRecord, controller: &str) -> Option<UiTreeItemNode> {
    let ui_contract::Component::TableRow(props) = &record.component else { return None };
    let actions: Vec<UiTreeItemAction> = props.row_actions.iter().map(|action| row_action(action, controller)).collect();
    let mut item = UiTreeItemNode::base(record.key.as_str(), Label::data(props.cells.get(0).map_or(record.key.as_str(), |cell| cell.as_str())));
    item.presence = record_presence(record);
    item.action = record_action(record, ui_contract::Trigger::Activate, controller);
    item.actions = (!actions.is_empty()).then_some(actions);
    item.menu = menu_ref(record);
    Some(item)
}

/// 🌳️ Assembles one `Component::TreeItem` record and its whole subtree into the inline""",
        ),
        (
            """        ui_contract::Component::TreeSection(_) | ui_contract::Component::TreeItem(_) => UiNode::Stack(UiStackNode {""",
            """        ui_contract::Component::TreeSection(_) | ui_contract::Component::TreeItem(_) | ui_contract::Component::TableRow(_) => UiNode::Stack(UiStackNode {""",
        ),
        (
            """        ui_contract::Component::Table(_) => UiNode::Stack(UiStackNode {
            direction: "vertical".into(),
            gap: None,
            padding: None,
            id: Some(record.key.as_str().to_string()),
            presence,
            activate: None,
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            drop_overlay: None,
            menu,
            children: Vec::new(),
        }),
        ui_contract::Component::TableRow(_) if !record.children.is_empty() => UiNode::Stack(UiStackNode {
            direction: "horizontal".into(),
            gap: None,
            padding: None,
            id: Some(record.key.as_str().to_string()),
            presence,
            activate: record_action(record, ui_contract::Trigger::Activate, controller),
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            drop_overlay: None,
            menu,
            children: Vec::new(),
        }),
        ui_contract::Component::TableRow(props) => UiNode::Button(UiButtonNode {
            id: Some(record.key.as_str().to_string()),
            icon_id: IconName::ChevronRight,
            label: Label::data(props.cells.iter().map(|cell| cell.as_str()).collect::<Vec<_>>().join(" · ")),
            action: record_action(record, ui_contract::Trigger::Activate, controller).unwrap_or_else(|| ActionDescriptor { controller_id: controller.to_string(), action: String::new(), args: None }),
            style: None,
            presence,
            menu,
        }),
""",
            """        ui_contract::Component::Table(props) => UiNode::Tree(UiTreeNode {
            presentation: ui_contract::TreePresentation::Standard,
            sections: vec![table_section(document, record, props, controller)],
            presence,
            drop_action: record_action(record, ui_contract::Trigger::Drop, controller),
            menu,
            interaction_domain: None,
        }),
""",
        ),
    ],
    MOUNTED: [
        (
            """        LayoutNodeKind::Tree { reversed, .. } => {
            let section = owner.sections.iter().find(|section| &section.id == key)?;""",
            """        LayoutNodeKind::Tree { reversed, .. } => {
            if document_table(tree, tree.node(id)?.parent?).is_some() {
                let item = owner.sections.iter().find_map(|section| find_tree_item(&section.items, key, 0))?;
                return Some(LayoutNodeKind::TableRow { height: live_tree_item_height(tree, id, item, &metrics, 0), actions: table_actions_width_of(owner, &metrics) });
            }
            let section = owner.sections.iter().find(|section| &section.id == key)?;""",
        ),
        (
            """    let mut height = metrics.for_item(item).row_height;
    if tree_item_detail_node(tree, id).is_some() {
        height += TREE_DETAIL_HEIGHT;
    }
""",
            """    let mut height = metrics.for_item(item).row_height;
    if tree_item_detail_node(tree, id).is_some() {
        height += TREE_DETAIL_HEIGHT;
    }
    if document_table_row(tree, id).is_some() && tree.children(id).any(|child| matches!(tree.node(child).map(|node| &node.spec.0), Some(UiNode::ComponentScene(_)))) {
        height = height.max(TREE_DETAIL_HEIGHT);
    }
""",
        ),
        (
            """fn is_tree_item_detail(tree: &UiTree, child: NodeId, parent: NodeId) -> bool {
    tree_item_detail_node(tree, parent) == Some(child)
}
""",
            """fn is_tree_item_detail(tree: &UiTree, child: NodeId, parent: NodeId) -> bool {
    tree_item_detail_node(tree, parent) == Some(child)
}

/// 📊️ The `TableProps` the mounted `Tree` node `id` renders — a table is a tree whose record is a `Component::Table`
/// (`reconcile::table_section`), and its column grid stays on that record.
pub(crate) fn document_table(tree: &UiTree, id: NodeId) -> Option<&ui_contract::TableProps> {
    let ui_contract::Component::Table(props) = &tree.document()?.record(tree.document_id(id)?)?.component else { return None };
    Some(props)
}

/// 📊️ The `TableRowProps` of the mounted table row `id` — its cells, positional to the table's columns.
pub(crate) fn document_table_row(tree: &UiTree, id: NodeId) -> Option<&ui_contract::TableRowProps> {
    let ui_contract::Component::TableRow(props) = &tree.document()?.record(tree.document_id(id)?)?.component else { return None };
    Some(props)
}

/// 🎬️ The `index`-th `RowAction` the mounted tree or table row `id` declares — the exact versioned binding a click on its icon
/// or its accessibility button fires (the retained item carries only the icon and the legacy descriptor).
pub(crate) fn document_row_action(tree: &UiTree, id: NodeId, index: usize) -> Option<&ui_contract::RowAction> {
    match &tree.document()?.record(tree.document_id(id)?)?.component {
        ui_contract::Component::TreeItem(props) => props.row_actions.get(index),
        ui_contract::Component::TableRow(props) => props.row_actions.get(index),
        _ => None,
    }
}

/// 📊️ A table's trailing actions column: one `layout::tree_row_action_rect` slot per action of its widest materialised row, as
/// React sizes its actions track by the widest action strip.
pub(crate) fn table_actions_width_of(node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    crate::wgpu::layout::table_actions_width(node.sections.iter().flat_map(|section| section.items.iter()).map(|item| item.actions.as_ref().map_or(0, Vec::len)).max().unwrap_or(0), metrics)
}
""",
        ),
        (
            """pub(crate) fn retained_tree_height(tree: &UiTree, id: NodeId, node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    let metrics = metrics.with_presentation(node.presentation);
""",
            """pub(crate) fn retained_tree_height(tree: &UiTree, id: NodeId, node: &UiTreeNode, metrics: &TreeRowMetrics) -> f32 {
    let metrics = metrics.with_presentation(node.presentation);
    if document_table(tree, id).is_some() {
        return metrics.header_height + node.sections.iter().flat_map(|section| section.items.iter()).filter_map(|item| tree.explicit_child(id, &item.id).map(|row| live_tree_item_height(tree, row, item, &metrics, 0))).sum::<f32>();
    }
""",
        ),
        (
            """        let tree_inline_control = matches!(parent_kind, Some(LayoutNodeKind::TreeRow { .. }));""",
            """        let tree_inline_control = matches!(parent_kind, Some(LayoutNodeKind::TreeRow { .. } | LayoutNodeKind::TableRow { .. }));""",
        ),
        (
            """                    UiNode::Tree(tree_node) => LayoutNodeKind::Tree { height: retained_tree_height(tree, id, tree_node, &self.row_metrics), reversed: root_reversed },""",
            """                    UiNode::Tree(tree_node) => {
                        LayoutNodeKind::Tree { height: retained_tree_height(tree, id, tree_node, &self.row_metrics), header: if document_table(tree, id).is_some() { self.row_metrics.header_height } else { 0.0 }, reversed: root_reversed }
                    }""",
        ),
    ],
    FLEX: [
        (
            """    /// 🌳️ A `Tree`, measured from its own spec through `layout`'s shared row geometry rather than
    /// from arena children — a tree's rows carry no children of their own, so aggregating them
    /// measured a whole tree as the sum of its rows' padding.
    Tree {
        height: f32,
        reversed: bool,
    },""",
            """    /// 🌳️ A `Tree`, measured from its own spec through `layout`'s shared row geometry rather than
    /// from arena children — a tree's rows carry no children of their own, so aggregating them
    /// measured a whole tree as the sum of its rows' padding. `header` is a table's column header
    /// band, reserved ahead of its rows (`0` for a tree).
    Tree {
        height: f32,
        header: f32,
        reversed: bool,
    },""",
        ),
        (
            """    TreeDetail {
        height: f32,
    },""",
            """    TreeDetail {
        height: f32,
    },
    /// 📊️ One table row: its materialised cell children flow along the inline axis in equal columns — the grid
    /// `layout::table_column_rect` gives the header and the painted cells — between the leading gap and the trailing
    /// `actions` column its row actions paint in.
    TableRow {
        height: f32,
        actions: f32,
    },""",
        ),
        (
            """        return FlowStyle { absolute: true, inset, width: Dim::Length(metrics.control_width), height: Dim::Length(control_height), ..content };
    }
""",
            """        return FlowStyle { absolute: true, inset, width: Dim::Length(metrics.control_width), height: Dim::Length(control_height), ..content };
    }
    if matches!(parent_kind, Some(LayoutNodeKind::TableRow { .. })) {
        let mut cell = flow_for(kind, None, authored, metrics);
        cell.absolute = false;
        cell.grow = 1.0;
        cell.shrink = 1.0;
        cell.width = Dim::Length(0.0);
        cell.min_width = 0.0;
        cell.height = match kind {
            LayoutNodeKind::Control { height, .. } => Dim::Length(height),
            _ => Dim::Fill,
        };
        return cell;
    }
""",
        ),
        (
            """        LayoutNodeKind::Tree { height, reversed } => band(height, 0.0, reversed),""",
            """        LayoutNodeKind::Tree { height, header, reversed } => band(height, header, reversed),""",
        ),
        (
            """        LayoutNodeKind::TreeDetail { height } => FlowStyle { width: Dim::Fill, height: Dim::Length(height), shrink: 0.0, ..FlowStyle::default() },""",
            """        LayoutNodeKind::TreeDetail { height } => FlowStyle { width: Dim::Fill, height: Dim::Length(height), shrink: 0.0, ..FlowStyle::default() },
        LayoutNodeKind::TableRow { height, actions } => {
            let (left, right) = if metrics.inline.is_rtl() { (actions, metrics.gap) } else { (metrics.gap, actions) };
            FlowStyle { row: true, reverse: metrics.inline.is_rtl(), gap_main: metrics.gap, align: Align::Center, height: Dim::Length(height), shrink: 0.0, padding: EdgePx { left, right, ..EdgePx::default() }, clips: true, ..FlowStyle::default() }
        }""",
        ),
    ],
    LAYOUT: [
        (
            """use crate::wgpu::component::ui::{UiControlNode, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};""",
            """use crate::wgpu::component::ui::{UiControlNode, UiTreeActionPlacement, UiTreeItemNode, UiTreeNode, UiTreeSectionNode};""",
        ),
        (
            """    pub drag_handle_extent: f32,
    pub inline: ui_contract::FlowInline,""",
            """    pub drag_handle_extent: f32,
    pub action_gap: f32,
    pub inline: ui_contract::FlowInline,""",
        ),
        (
            """            drag_handle_extent: crate::wgpu::chrome::ICON_TREE_ROW,
            inline: ui_contract::FlowInline::Ltr,""",
            """            drag_handle_extent: crate::wgpu::chrome::ICON_TREE_ROW,
            action_gap: theme.padding_standard,
            inline: ui_contract::FlowInline::Ltr,""",
        ),
        (
            """    Rect::new(x, (metrics.row_height - metrics.control_height) * 0.5, metrics.control_width.min((row_width - reservation).max(0.0)), metrics.control_height)
}
//#endregion 🌳️TreeRowGeometry""",
            """    Rect::new(x, (metrics.row_height - metrics.control_height) * 0.5, metrics.control_width.min((row_width - reservation).max(0.0)), metrics.control_height)
}

/// 🎬️ A tree or table row's trailing action `slot` (1 = the inline-end-most), relative to the row's own top-left — the one rect
/// `paint` draws a row action's icon at and the pointer router fires it from. `trailing` is the drag handle's reservation when
/// the row shows one.
pub fn tree_row_action_rect(row_width: f32, row_height: f32, slot: usize, trailing: f32, metrics: &TreeRowMetrics) -> Rect {
    let extent = metrics.drag_handle_extent;
    let offset = metrics.gap + trailing + slot as f32 * (extent + metrics.action_gap);
    let x = if metrics.inline.is_rtl() { offset - extent } else { row_width - offset };
    Rect::new(x, (row_height - extent) * 0.5, extent, extent)
}

/// 🎬️ The index of the Row-placed action whose slot holds the row-local point: every action takes one slot from the inline end
/// in reverse authored order, exactly as `paint` counts them, and a Menu-placed one paints and hits nothing in its slot.
pub fn tree_row_action_at(item: &UiTreeItemNode, row_width: f32, row_height: f32, trailing: f32, metrics: &TreeRowMetrics, x: f32, y: f32) -> Option<usize> {
    let actions = item.actions.as_deref()?;
    actions.iter().enumerate().rev().zip(1usize..).find(|((_, action), slot)| action.placement() == UiTreeActionPlacement::Row && tree_row_action_rect(row_width, row_height, *slot, trailing, metrics).contains(x, y)).map(|((index, _), _)| index)
}

/// 📊️ The trailing actions column of a table whose widest row carries `slots` actions — the span its
/// [`tree_row_action_rect`] slots occupy, so the header's actions label, the cells and the icons agree.
pub fn table_actions_width(slots: usize, metrics: &TreeRowMetrics) -> f32 {
    if slots == 0 {
        0.0
    } else {
        metrics.gap + slots as f32 * (metrics.drag_handle_extent + metrics.action_gap)
    }
}

/// 📊️ Column `column` of a `columns`-wide table row (or its header), relative to the row's own top-left: the materialised columns
/// share the row's inline extent equally, `gap` apart, between the leading gap and the `actions` column — the flow `flex`'s
/// `LayoutNodeKind::TableRow` lays an editable row's cell children in.
pub fn table_column_rect(row_width: f32, row_height: f32, column: usize, columns: usize, actions: f32, metrics: &TreeRowMetrics) -> Rect {
    let span = (row_width - metrics.gap - actions - metrics.gap * columns.saturating_sub(1) as f32).max(0.0);
    let width = if columns == 0 { 0.0 } else { span / columns as f32 };
    let offset = metrics.gap + column as f32 * (width + metrics.gap);
    let x = if metrics.inline.is_rtl() { row_width - offset - width } else { offset };
    Rect::new(x, 0.0, width, row_height)
}
//#endregion 🌳️TreeRowGeometry""",
        ),
        (
            """#[cfg(test)]
#[path = "../../../🧪️tests/🌳️tree-row-rects/🦀️.rs"]
mod tree_row_rect_tests;""",
            """#[cfg(test)]
#[path = "../../../🧪️tests/🌳️tree-row-rects/🦀️.rs"]
mod tree_row_rect_tests;

#[cfg(test)]
#[path = "../../../🧪️tests/📊️table-row-grid/🦀️.rs"]
mod table_row_grid_tests;""",
        ),
    ],
    PAINT: [
        (
            """        self.chrome = false;
        self.item = 0;
    }
}""",
            """        self.chrome = false;
        self.item = 0;
    }

    /// 🧮️ Moves to the next part of the SAME phase — a table's next column label or cell: the glyph and measure state restart,
    /// the part index advances.
    fn next_part(&mut self) {
        self.glyph.reset();
        self.measure_byte = 0;
        self.measure_width = 0.0;
        self.item += 1;
    }
}""",
        ),
        (
            """    let mut parent = retained.explicit_child(tree_id, &section.id)?;""",
            """    let mut parent = if crate::wgpu::mounted_layout::document_table(retained, tree_id).is_some() { tree_id } else { retained.explicit_child(tree_id, &section.id)? };""",
        ),
        (
            """fn retained_tree_node_step(
    retained: &UiTree,""",
            """/// 📊️ Advances a table's column header by one glyph (`cursor.item` is the column): each materialised column label over its
/// `layout::table_column_rect`, then `TableProps::actions_label` over the trailing actions column, in the band
/// `LayoutNodeKind::Tree::header` reserves ahead of the rows; then the rows (phase 2).
#[allow(clippy::too_many_arguments, reason = "one retained paint context")]
fn retained_table_header_step(
    retained: &UiTree,
    tree_id: NodeId,
    tree: &UiTreeNode,
    bounds: Rect,
    theme: &Theme,
    reversed: bool,
    inline: ui_contract::FlowInline,
    metrics: &TreeRowMetrics,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    let Some(table) = crate::wgpu::mounted_layout::document_table(retained, tree_id) else { return RetainedNodePaintStep::Fault };
    let header = metrics.header_height;
    let actions = crate::wgpu::mounted_layout::table_actions_width_of(tree, metrics);
    let columns = table.columns.len();
    let (label, rect) = match (table.columns.get(cursor.item), table.actions_label.as_ref()) {
        (Some(label), _) => (label, crate::wgpu::layout::table_column_rect(bounds.w, header, cursor.item, columns, actions, metrics)),
        (None, Some(label)) if cursor.item == columns && actions > 0.0 => (label, Rect::new(if inline.is_rtl() { 0.0 } else { bounds.w - actions }, 0.0, actions, header)),
        _ => {
            if !reversed {
                cursor.row_y += header;
            }
            cursor.advance(2);
            return RetainedNodePaintStep::Pending;
        }
    };
    let y = if reversed { cursor.row_y + retained_tree_content_height(retained, tree_id, tree, metrics) - header } else { cursor.row_y };
    match retained_tree_text_step(label.0.as_str(), Rect::new(bounds.x + rect.x, y, rect.w, header), theme.font_size_small, theme.text_muted, inline, atlas, draw, cursor) {
        RetainedNodePaintStep::Complete => {
            cursor.next_part();
            RetainedNodePaintStep::Pending
        }
        step => step,
    }
}

/// 📊️ Advances one table row's cells by one glyph (`cursor.item` is the column): column `i` paints `TableRowProps::cells[i]` in
/// its `layout::table_column_rect` unless the row materialised a child for it — an editable row's input, paged draft or
/// read-only surface, which paints itself in the same column (`LayoutNodeKind::TableRow`); then the row's trailing actions.
#[allow(clippy::too_many_arguments, reason = "one retained paint context")]
fn retained_table_cells_step(
    retained: &UiTree,
    tree_id: NodeId,
    tree: &UiTreeNode,
    item: &UiTreeItemNode,
    row: Rect,
    font_size: f32,
    theme: &Theme,
    inline: ui_contract::FlowInline,
    metrics: &TreeRowMetrics,
    reversed: bool,
    atlas: &mut FontAtlas,
    draw: &mut DrawList,
    cursor: &mut RetainedNodePaintCursor,
) -> RetainedNodePaintStep {
    let (Some(table), Some(row_id)) = (crate::wgpu::mounted_layout::document_table(retained, tree_id), retained_tree_item_node_at(retained, tree_id, tree, cursor, reversed)) else {
        return RetainedNodePaintStep::Fault;
    };
    let columns = table.columns.len();
    let index = cursor.item;
    if index >= columns {
        cursor.advance(5);
        return RetainedNodePaintStep::Pending;
    }
    let materialised = retained.children(row_id).count();
    let Some(text) = crate::wgpu::mounted_layout::document_table_row(retained, row_id).and_then(|props| props.cells.get(index)).filter(|_| index >= materialised) else {
        cursor.next_part();
        return RetainedNodePaintStep::Pending;
    };
    let column = crate::wgpu::layout::table_column_rect(row.w, row.h, index, columns, crate::wgpu::mounted_layout::table_actions_width_of(tree, metrics), metrics);
    let color = foreground_on_fill(theme, theme.text_element, item.presence.selected, item.presence.state == UiState::Previewed || item.presence.hover);
    let color = if item.presence.state == UiState::Disabled { color.with_alpha(color.a * 0.5) } else { color };
    match retained_tree_text_step(text.as_str(), Rect::new(row.x + column.x, row.y + column.y, column.w, column.h), font_size, color, inline, atlas, draw, cursor) {
        RetainedNodePaintStep::Complete => {
            cursor.next_part();
            RetainedNodePaintStep::Pending
        }
        step => step,
    }
}

/// 📊️ How far the paint cursor moves past one row: a table row's whole laid-out height (`mounted_layout::live_tree_item_height`
/// — a row whose cell hosts a draft or read-only surface is taller), a tree row's own band (its nested rows advance it themselves).
fn retained_tree_row_advance(retained: &UiTree, tree_id: NodeId, tree: &UiTreeNode, item: &UiTreeItemNode, cursor: &RetainedNodePaintCursor, metrics: &TreeRowMetrics, reversed: bool) -> f32 {
    let band = metrics.for_item(item).row_height;
    if crate::wgpu::mounted_layout::document_table(retained, tree_id).is_none() {
        return band;
    }
    retained_tree_item_node_at(retained, tree_id, tree, cursor, reversed).map_or(band, |row| crate::wgpu::mounted_layout::live_tree_item_height(retained, row, item, metrics, 0))
}

fn retained_tree_node_step(
    retained: &UiTree,""",
        ),
        (
            """            if !section.presence.visible() {
                cursor.section += 1;
                cursor.advance(1);
                return RetainedNodePaintStep::Pending;
            }
            let open = retained.tree_section_open(tree_id, &section.id, crate::wgpu::layout::tree_section_default_open(section));
            let header_height = tree_section_header_height(section, &metrics);""",
            """            if !section.presence.visible() {
                cursor.section += 1;
                cursor.advance(1);
                return RetainedNodePaintStep::Pending;
            }
            if crate::wgpu::mounted_layout::document_table(retained, tree_id).is_some() {
                return retained_table_header_step(retained, tree_id, tree, bounds, theme, reversed, inline, &metrics, atlas, draw, cursor);
            }
            let open = retained.tree_section_open(tree_id, &section.id, crate::wgpu::layout::tree_section_default_open(section));
            let header_height = tree_section_header_height(section, &metrics);""",
        ),
        (
            """            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
            let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
            let selected = item.presence.selected;""",
            """            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            if crate::wgpu::mounted_layout::document_table(retained, tree_id).is_some() {
                return retained_table_cells_step(retained, tree_id, tree, item, row, font_size, theme, inline, &metrics, reversed, atlas, draw, cursor);
            }
            let indent = bounds.x + (cursor.depth - 1) as f32 * TREE_INDENT_PER_LEVEL + TREE_TOGGLE_WIDTH;
            let label_x = indent + if item.icon_id.is_some() { TREE_ICON_SIZE + theme.gap_standard } else { 0.0 };
            let selected = item.presence.selected;""",
        ),
        (
            """            let offset = theme.gap_standard + trailing + cursor.item as f32 * (TREE_ICON_SIZE + theme.padding_standard);
            let x = if inline.is_rtl() { bounds.x + offset - TREE_ICON_SIZE } else { bounds.x + bounds.w - offset };
            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            let result = retained_fixed_output(draw, |draw| {
                if let Some(icons) = icons {
                    let on_hover_fill = item.presence.state == UiState::Previewed || item.presence.hover;
                    push_icon(draw, icons, action.icon_id.as_str(), x, row.y + (row.h - TREE_ICON_SIZE) * 0.5, TREE_ICON_SIZE, foreground_on_fill(theme, theme.text_element, item.presence.selected, on_hover_fill));""",
            """            let Some((row, _)) = retained_tree_item_row(retained, tree_id, tree, bounds, cursor, &metrics, reversed) else { return RetainedNodePaintStep::Fault };
            let slot = crate::wgpu::layout::tree_row_action_rect(row.w, row.h, cursor.item, trailing, &metrics);
            let result = retained_fixed_output(draw, |draw| {
                if let Some(icons) = icons {
                    let on_hover_fill = item.presence.state == UiState::Previewed || item.presence.hover;
                    push_icon(draw, icons, action.icon_id.as_str(), row.x + slot.x, row.y + slot.y, slot.w, foreground_on_fill(theme, theme.text_element, item.presence.selected, on_hover_fill));""",
        ),
        (
            """            if item.presence.visible() && (!reversed || !open) {
                cursor.row_y += metrics.for_item(item).row_height;
            }""",
            """            if item.presence.visible() && (!reversed || !open) {
                let advance = retained_tree_row_advance(retained, tree_id, tree, item, cursor, &metrics, reversed);
                cursor.row_y += advance;
            }""",
        ),
    ],
    EVENTS: [
        (
            """use crate::wgpu::layout::{number_stepper_segments, ring_t_at, slider_control_presentation, slider_unit_label, slider_value_at, tree_drag_handle_rect, tree_drag_role, tree_section_header_band, tree_section_header_height, TreeRowMetrics};""",
            """use crate::wgpu::layout::{
    number_stepper_segments, ring_t_at, slider_control_presentation, slider_unit_label, slider_value_at, tree_drag_handle_rect, tree_drag_handle_reservation, tree_drag_role, tree_section_header_band, tree_section_header_height, TreeRowMetrics,
};""",
        ),
        (
            """/// never an ad hoc per-item action.
fn is_plain_stack_container(tree: &UiTree, id: NodeId, node: &Node) -> bool {
    let UiNode::Stack(stack) = &node.spec.0 else { return false };
    stack.activate.is_none() && stack.drop_action.is_none() && !node.flags.contains(NodeFlags::DRAG_SOURCE) && !tree.disclosure_is_interactive(id)
}""",
            """/// never an ad hoc per-item action. A tree or table row carrying Row-placed actions is a target too:
/// its action slots resolve in `EventRouter::pointer_row_action`.
fn is_plain_stack_container(tree: &UiTree, id: NodeId, node: &Node) -> bool {
    let UiNode::Stack(stack) = &node.spec.0 else { return false };
    stack.activate.is_none()
        && stack.drop_action.is_none()
        && !node.flags.contains(NodeFlags::DRAG_SOURCE)
        && !tree.disclosure_is_interactive(id)
        && !tree.authored_tree_item(id).and_then(|item| item.actions.as_deref()).is_some_and(|actions| actions.iter().any(|action| action.placement() == crate::wgpu::UiTreeActionPlacement::Row))
}""",
        ),
        (
            """    fn push_app_command(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction, out: &mut Vec<UiCommand>) {
        if let Some(command) = self.app_command(tree, node, fired) {
            out.push(command);
        }
    }
""",
            """    fn push_app_command(&mut self, tree: &UiTree, node: NodeId, fired: FiredAction, out: &mut Vec<UiCommand>) {
        if let Some(command) = self.app_command(tree, node, fired) {
            out.push(command);
        }
    }

    /// 🎬️ A row action's intent: the row's own address with the ACTION's versioned id from its `RowAction` binding
    /// (`mounted_layout::document_row_action`) — `build_intent` would answer the row's own `Trigger::Activate` binding, its
    /// primary activation, for the same trigger.
    fn row_action_command(&mut self, tree: &UiTree, row: NodeId, index: usize) -> Option<UiCommand> {
        let binding = &crate::wgpu::mounted_layout::document_row_action(tree, row, index)?.action;
        let args = tree.authored_tree_item(row)?.actions.as_deref()?.get(index)?.action.args.clone();
        let current_revision = tree.document().map_or(0, |document| document.revision().0);
        let address = match tree.node(row)?.intent.as_ref() {
            Some(bindings) if intent_is_stale(bindings.address.revision, current_revision) => return None,
            Some(bindings) => bindings.address.clone(),
            None => UiIntentAddress::default(),
        };
        let seq = self.intents.next(&address.surface);
        Some(UiCommand::App { window_id: self.window_id.clone(), intent: UiIntentCommand { address, trigger: binding.trigger, action: binding.action.clone(), args, input: None, seq } })
    }

    /// 🎬️ The index of the Row-placed action whose trailing slot (`layout::tree_row_action_at`, the slots `paint` draws) holds the
    /// pointer on a tree or table row — a click there fires that action, never the row's activation or disclosure.
    fn pointer_row_action(&self, tree: &UiTree, id: NodeId, x: f32, y: f32) -> Option<usize> {
        let item = tree.authored_tree_item(id)?;
        let metrics = crate::wgpu::mounted_layout::retained_tree_row_metrics(tree, id, &self.tree_drag_metrics);
        let band = tree_section_header_band(tree.absolute_rect(id)?, metrics.row_height, self.flow.block.is_reversed());
        let trailing = if self.tree_drag_driver == UiDriverDrag::Handle && tree_drag_role(item).is_some() { tree_drag_handle_reservation(&metrics) } else { 0.0 };
        crate::wgpu::layout::tree_row_action_at(item, band.w, band.h, trailing, &metrics, x - band.x, y - band.y)
    }
""",
        ),
        (
            """                                let is_select = tree.node(active_id).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_)));
                                let disclosed = self.pointer_toggle_disclosure(tree, active_id, *x, *y);
                                let row_also_activates = disclosed && tree.authored_tree_item(active_id).is_some() && tree.node(active_id).is_some_and(|node| matches!(&node.spec.0, UiNode::Stack(stack) if stack.activate.is_some()));
                                if slider_readout {
                                } else if disclosed && !row_also_activates {""",
            """                                let is_select = tree.node(active_id).is_some_and(|node| matches!(node.spec.0, UiNode::Select(_)));
                                let row_action = self.pointer_row_action(tree, active_id, *x, *y);
                                let disclosed = row_action.is_none() && self.pointer_toggle_disclosure(tree, active_id, *x, *y);
                                let row_also_activates = disclosed && tree.authored_tree_item(active_id).is_some() && tree.node(active_id).is_some_and(|node| matches!(&node.spec.0, UiNode::Stack(stack) if stack.activate.is_some()));
                                if slider_readout {
                                } else if let Some(index) = row_action {
                                    commands.extend(self.row_action_command(tree, active_id, index));
                                } else if disclosed && !row_also_activates {""",
        ),
        (
            """        commands.extend(self.close_overlay(tree, target));
        commands
    }

    pub(crate) fn dispatch_accessibility_slider_editor(""",
            """        commands.extend(self.close_overlay(tree, target));
        commands
    }

    /// ♿️ The accessibility mirror's activation of a row's `index`-th action (`accessibility::row_accessibility_action`), fired
    /// exactly as a pointer on its trailing icon fires it.
    pub(crate) fn dispatch_accessibility_row_action(&mut self, tree: &mut UiTree, target: NodeId, index: usize, event: &AccessibilityUiEvent) -> Vec<UiCommand> {
        let enabled = tree.node(target).is_some_and(|node| node.spec.0.presence().state != UiState::Disabled);
        if !enabled || !matches!(event, AccessibilityUiEvent::Activate) {
            return Vec::new();
        }
        self.row_action_command(tree, target, index).into_iter().collect()
    }

    pub(crate) fn dispatch_accessibility_slider_editor(""",
        ),
    ],
    ACCESSIBILITY: [
        (
            """const SLIDER_EDITOR_KEY_SUFFIX: &str = "::editor";
""",
            """const SLIDER_EDITOR_KEY_SUFFIX: &str = "::editor";
const ROW_ACTION_KEY_INFIX: &str = "::row-action::";
""",
        ),
        (
            """pub(crate) fn is_slider_accessibility_editor(record: &ui_contract::UiNodeRecord, key: &str) -> bool {
    matches!(record.component, ui_contract::Component::Slider(_)) && key == format!("{}{SLIDER_EDITOR_KEY_SUFFIX}", record.key.as_str())
}
""",
            """pub(crate) fn is_slider_accessibility_editor(record: &ui_contract::UiNodeRecord, key: &str) -> bool {
    matches!(record.component, ui_contract::Component::Slider(_)) && key == format!("{}{SLIDER_EDITOR_KEY_SUFFIX}", record.key.as_str())
}

/// 🎬️ A tree or table row record's `RowAction`s and the name React composes their buttons' names with (the tree item's label,
/// the table row's first cell).
fn record_row_actions(record: &ui_contract::UiNodeRecord) -> Option<(&str, &ui_contract::UiFixedList<ui_contract::RowAction>)> {
    match &record.component {
        ui_contract::Component::TreeItem(item) => Some((item.label.0.as_str(), &item.row_actions)),
        ui_contract::Component::TableRow(row) => Some((row.cells.get(0).map_or(record.key.as_str(), |cell| cell.as_str()), &row.row_actions)),
        _ => None,
    }
}

/// ♿️ Which of a row's actions the virtual `<rowKey>::row-action::<i>` button of [`accessibility_projection`] activates.
pub(crate) fn row_accessibility_action(record: &ui_contract::UiNodeRecord, key: &str) -> Option<usize> {
    let (_, actions) = record_row_actions(record)?;
    let index = key.strip_prefix(record.key.as_str())?.strip_prefix(ROW_ACTION_KEY_INFIX)?.parse::<usize>().ok()?;
    actions.get(index).filter(|action| action.placement == ui_contract::RowActionPlacement::Row).map(|_| index)
}

/// ♿️ A row's Row-placed actions as the buttons its trailing action icons paint — named `"<label>: <row name>"` as React's
/// `TableView` names them, reachable and activatable, so no row action is pointer-only on this target.
fn row_action_accessibility_nodes(record: &ui_contract::UiNodeRecord, depth: usize, owner: &AccessibilityProjectionNode) -> Vec<AccessibilityProjectionNode> {
    let Some((name, actions)) = record_row_actions(record) else { return Vec::new() };
    let mut nodes = Vec::new();
    for (index, action) in actions.iter().enumerate() {
        if action.placement != ui_contract::RowActionPlacement::Row {
            continue;
        }
        let mut button = owner.clone();
        button.key = format!("{}{ROW_ACTION_KEY_INFIX}{index}", record.key.as_str());
        button.role = "button".to_string();
        button.depth = depth.saturating_add(1);
        button.label = Some(action.label.as_ref().map_or_else(|| name.to_string(), |label| format!("{}: {name}", label.0.as_str())));
        button.description = None;
        button.shortcut = None;
        button.focusable = !record.disabled;
        button.tabbable = !record.disabled;
        button.actionable = !record.disabled;
        button.focused = false;
        button.checked = None;
        button.pressed = None;
        button.selected = None;
        button.expanded = None;
        button.level = None;
        button.value_min = None;
        button.value_max = None;
        button.value_now = None;
        button.value_text = None;
        button.busy = false;
        nodes.push(button);
    }
    nodes
}
""",
        ),
        (
            """        if select_open {
            for virtual_node in select_accessibility_nodes(record, pending.depth, &node) {
                if projection.len() >= UI_DOCUMENT_NODES {
                    break;
                }
                projection.push(virtual_node);
            }
        }
""",
            """        if select_open {
            for virtual_node in select_accessibility_nodes(record, pending.depth, &node) {
                if projection.len() >= UI_DOCUMENT_NODES {
                    break;
                }
                projection.push(virtual_node);
            }
        }
        for virtual_node in row_action_accessibility_nodes(record, pending.depth, &node) {
            if projection.len() >= UI_DOCUMENT_NODES {
                break;
            }
            projection.push(virtual_node);
        }
""",
        ),
    ],
    ENGINE: [
        (
            """            let virtual_slider_editor = record.key.as_str() != node_key && crate::wgpu::accessibility::is_slider_accessibility_editor(record, node_key);
            if record.key.as_str() != node_key && virtual_select_value.is_none() && !virtual_slider_editor {
                return None;
            }""",
            """            let virtual_slider_editor = record.key.as_str() != node_key && crate::wgpu::accessibility::is_slider_accessibility_editor(record, node_key);
            let virtual_row_action = (record.key.as_str() != node_key).then(|| crate::wgpu::accessibility::row_accessibility_action(record, node_key)).flatten();
            if record.key.as_str() != node_key && virtual_select_value.is_none() && !virtual_slider_editor && virtual_row_action.is_none() {
                return None;
            }""",
        ),
        (
            """            commands.extend(match virtual_select_value {
                Some(value) => router.dispatch_accessibility_select_option(tree, target, &value, &event),
                None if virtual_slider_editor => router.dispatch_accessibility_slider_editor(tree, target, &event),
                None => router.dispatch_accessibility(tree, target, &event),
            });""",
            """            commands.extend(match (virtual_select_value, virtual_row_action) {
                (Some(value), _) => router.dispatch_accessibility_select_option(tree, target, &value, &event),
                (None, _) if virtual_slider_editor => router.dispatch_accessibility_slider_editor(tree, target, &event),
                (None, Some(index)) => router.dispatch_accessibility_row_action(tree, target, index, &event),
                (None, None) => router.dispatch_accessibility(tree, target, &event),
            });""",
        ),
    ],
    FLEX_LAWS: [
        (
            """    let tree = fixture.push(LayoutNodeKind::Tree { height: intrinsic, reversed: false }, Some(viewport), None);""",
            """    let tree = fixture.push(LayoutNodeKind::Tree { height: intrinsic, header: 0.0, reversed: false }, Some(viewport), None);""",
        ),
        (
            """    let tree = fixture.push(LayoutNodeKind::Tree { height: header, reversed: false }, None, None);""",
            """    let tree = fixture.push(LayoutNodeKind::Tree { height: header, header: 0.0, reversed: false }, None, None);""",
        ),
    ],
    RECONCILE_LAWS: [
        (
            """    let row = document.record(ui_contract::UiNodeId(0)).expect("row");
    let UiNode::Button(projected) = ui_node_from_record(&document, row, "table.childless", "s.stdio.csv@rfc4180/*#editor") else { panic!("childless row projects as its legacy button") };
    assert!(projected.action.action.is_empty(), "the row action is not borrowed as an implicit destructive row activation");""",
            """    let row = document.record(ui_contract::UiNodeId(0)).expect("row");
    let UiNode::Stack(projected) = ui_node_from_record(&document, row, "table.childless", "s.stdio.csv@rfc4180/*#editor") else { panic!("a childless table row mounts as its keyed identity row") };
    assert!(projected.activate.is_none(), "the row action is not borrowed as an implicit destructive row activation");""",
        ),
    ],
}

EDITABLE_LAW_OLD = r'''#[test]
fn a_declarative_table_row_does_not_turn_its_remove_action_into_row_activation() {
    let row: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 0,
        "key": "row-7",
        "children": [1, 2],
        "component": {
            "type": "tableRow",
            "cells": ["Ada"],
            "rowActions": [{
                "icon": "trash-2",
                "label": "Remove row",
                "action": {
                    "trigger": "activate",
                    "action": { "scope": "s.stdio.csv@rfc4180/*#editor", "name": "remove-row", "version": 1 },
                    "args": { "row": 7, "revision": "0123456789abcdef" }
                }
            }]
        },
        "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("table row fixture");
    let child: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "cell-0",
        "component": { "type": "input", "kind": "text", "value": "Ada", "commit": "blur" },
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Name" }
    }))
    .expect("cell fixture");
    let action: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 2,
        "key": "row-action-0",
        "component": { "type": "button", "icon": "trash-2", "label": "Remove row" },
        "bindings": [{
            "trigger": "activate",
            "action": { "scope": "s.stdio.csv@rfc4180/*#editor", "name": "remove-row", "version": 1 },
            "args": { "row": 7, "revision": "0123456789abcdef" }
        }],
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Remove row" }
    }))
    .expect("row action fixture");
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from("table.action").expect("surface"), revision: ui_contract::UiRevision(0), root: row.id, layout_epoch: 0, node_count: 3 };
    let mut document = UiDocumentTree::new(header).expect("document");
    document.try_upsert_record(row).expect("row record");
    document.try_upsert_record(child).expect("cell record");
    document.try_upsert_record(action).expect("action record");
    {
        let row = document.record(ui_contract::UiNodeId(0)).expect("row");
        let UiNode::Stack(projected) = ui_node_from_record(&document, row, "table.action", "s.stdio.csv@rfc4180/*#editor") else { panic!("a table row with declarative cells projects as a stack") };
        assert!(projected.activate.is_none(), "focusing or activating an editable row must not run its destructive trailing action");
    }
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(1);
    for _ in 0..64 {
        if matches!(tree.step_document_reconcile(&mut cursor, "table.action", "s.stdio.csv@rfc4180/*#editor"), UiDocumentReconcileStep::Complete) {
            break;
        }
    }
    let row = tree.document_node(ui_contract::UiNodeId(0)).expect("row mounted");
    let children = tree.children(row).collect::<Vec<_>>();
    assert_eq!(children.len(), 2);
    let UiNode::Button(action) = &tree.node(children[1]).expect("action mounted").spec.0 else { panic!("the row action mounts as its own button") };
    assert_eq!(action.label.as_str(), "Remove row");
    assert_eq!(action.action.action, "remove-row");
    let Some(DslValue::Object(arguments)) = &action.action.args else { panic!("remove address is retained") };
    assert!(arguments.contains(&("revision".into(), DslValue::String("0123456789abcdef".into()))));
    assert!(arguments.contains(&("row".into(), DslValue::uint(7))));
}
'''

EDITABLE_LAW_NEW = r'''#[test]
fn an_editable_table_row_keeps_one_child_per_cell_and_its_remove_action_as_a_row_action() {
    let table: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 0,
        "key": "table",
        "children": [1],
        "component": { "type": "table", "label": "People", "columns": ["Name"], "actionsLabel": "Actions" },
        "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("table fixture");
    let row: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 1,
        "key": "row-7",
        "children": [2],
        "component": {
            "type": "tableRow",
            "cells": ["Ada"],
            "rowActions": [{
                "icon": "trash-2",
                "label": "Remove row",
                "action": {
                    "trigger": "activate",
                    "action": { "scope": "s.stdio.csv@rfc4180/*#editor", "name": "remove-row", "version": 1 },
                    "args": { "revision": "0123456789abcdef", "row": 7 }
                }
            }]
        },
        "layout": { "kind": "stack", "axis": "horizontal", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {},
        "activity": "idle",
        "accessibility": {}
    }))
    .expect("table row fixture");
    let cell: ui_contract::UiNodeRecord = serde_json::from_value(serde_json::json!({
        "id": 2,
        "key": "cell-0",
        "component": { "type": "input", "kind": "text", "value": "Ada", "commit": "blur" },
        "layout": { "kind": "leaf", "width": "hug", "height": "hug" },
        "style": {},
        "activity": "idle",
        "accessibility": { "label": "Name" }
    }))
    .expect("cell fixture");
    let header = ui_contract::UiDocumentLeaseHeader { generation: 1, surface: ui_contract::SurfaceId::try_from("table.action").expect("surface"), revision: ui_contract::UiRevision(0), root: table.id, layout_epoch: 0, node_count: 3 };
    let mut document = UiDocumentTree::new(header).expect("document");
    document.try_upsert_record(table).expect("table record");
    document.try_upsert_record(row).expect("row record");
    document.try_upsert_record(cell).expect("cell record");
    {
        let row = document.record(ui_contract::UiNodeId(1)).expect("row");
        let UiNode::Stack(projected) = ui_node_from_record(&document, row, "table.action", "s.stdio.csv@rfc4180/*#editor") else { panic!("a table row mounts as its keyed identity row") };
        assert!(projected.activate.is_none(), "focusing or activating an editable row must not run its destructive trailing action");
        let table = document.record(ui_contract::UiNodeId(0)).expect("table");
        let UiNode::Tree(projected) = ui_node_from_record(&document, table, "table.action", "s.stdio.csv@rfc4180/*#editor") else { panic!("a table paints through the retained tree") };
        let [section] = projected.sections.as_slice() else { panic!("a table is ONE section") };
        assert_eq!(section.id, "table", "the section is keyed by the table's own record key");
        let [item] = section.items.as_slice() else { panic!("one row item") };
        assert_eq!((item.id.as_str(), item.label.as_str()), ("row-7", "Ada"), "a row is named by its first cell");
        assert!(item.action.is_none(), "the row action is not the row's activation");
        let Some([remove]) = item.actions.as_deref() else { panic!("the RowAction prop is the row's one trailing action") };
        assert_eq!(remove.label.as_ref().map(|label| label.as_str()), Some("Remove row"));
        assert_eq!(remove.action.action, "remove-row");
        let Some(DslValue::Object(arguments)) = &remove.action.args else { panic!("remove address is retained") };
        assert!(arguments.contains(&("revision".into(), DslValue::String("0123456789abcdef".into()))));
        assert!(arguments.contains(&("row".into(), DslValue::uint(7))));
    }
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(1);
    for _ in 0..64 {
        if matches!(tree.step_document_reconcile(&mut cursor, "table.action", "s.stdio.csv@rfc4180/*#editor"), UiDocumentReconcileStep::Complete) {
            break;
        }
    }
    let row = tree.document_node(ui_contract::UiNodeId(1)).expect("row mounted");
    let children = tree.children(row).collect::<Vec<_>>();
    let [cell] = children.as_slice() else { panic!("an editable row keeps exactly one child per materialised cell") };
    let UiNode::Input(input) = &tree.node(*cell).expect("cell mounted").spec.0 else { panic!("the cell mounts as its own input") };
    assert_eq!(input.id, "cell-0");
}
'''

EDITS[RECONCILE_LAWS].append((EDITABLE_LAW_OLD, EDITABLE_LAW_NEW))

GRID_LAWS_SOURCE = r'''//! 📊️ LAW: a table is ONE column grid on the wgpu target — its header, its painted cells and an editable row's cell children
//! share `layout::table_column_rect`, and a row's actions are its `RowAction` props only: painted, hit and announced at
//! `layout::tree_row_action_rect`, each firing its own versioned binding.
//!
//! The defect: the reconcile painted a childless `TableRow` as ONE button labelled `cells.join(" · ")` and a row with children as a
//! bare horizontal stack (cells dropped, no header), and never painted `TableRowProps::row_actions` — so the SDK duplicated every
//! row action as a `row-action-<i>` child button and Home's 32-row window ran out of its item budget after 27 rows (ticket
//! 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP session 14, the WG11 × LB2 TableRow agreement).
//!
//! Oracle: `🖱️ui/🧫️fixtures/📊️table-row-grid/🔣️.json`; the editable row's cell rects are also solved by taffy.

use crate::wgpu::accessibility::{accessibility_projection, row_accessibility_action};
use crate::wgpu::arena::NodeId;
use crate::wgpu::chrome::UiDriverDrag;
use crate::wgpu::draw::{DrawList, KIND_GLYPH};
use crate::wgpu::events::{AccessibilityUiEvent, EventRouter, PointerButton, UiCommand, UiEvent};
use crate::wgpu::layout::{table_actions_width, table_column_rect, tree_row_action_rect, TreeRowMetrics};
use crate::wgpu::mounted_layout::layout_tree_now;
use crate::wgpu::paint::{paint_node_step_with_driver, RetainedNodePaintCursor, RetainedNodePaintStep};
use crate::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{UiDocumentTree, UiTree};
use serde_json::Value;
use ui_contract::{SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision};

const GENERATION: u64 = 1;

fn law() -> Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📊️table-row-grid/🔣️.json")).expect("📊️ the table-row-grid fixture parses")
}

fn number(value: &Value, key: &str) -> f32 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture number {key}")) as f32
}

fn close(left: f32, right: f32) -> bool {
    (left - right).abs() <= 1e-3
}

/// 📃️ Publishes the fixture document, reconciles it to completion and lays it out in the fixture viewport — the path a live
/// window takes (document → arena → mounted layout).
fn mounted(law: &Value) -> (UiTree, NodeId) {
    let source = &law["document"];
    let nodes = source["nodes"].as_array().expect("fixture nodes");
    let header = UiDocumentLeaseHeader {
        generation: GENERATION,
        surface: SurfaceId::try_from(source["surface"].as_str().expect("fixture surface")).expect("fixture surface id"),
        revision: UiRevision(source["revision"].as_u64().expect("fixture revision")),
        root: UiNodeId(source["root"].as_u64().expect("fixture root")),
        layout_epoch: source["layoutEpoch"].as_u64().expect("fixture layout epoch"),
        node_count: nodes.len(),
    };
    let mut document = UiDocumentTree::new(header).expect("fixture header admits");
    for node in nodes {
        let record: UiNodeRecord = serde_json::from_value(node.clone()).expect("fixture record deserializes against the contract");
        document.try_upsert_record(record).expect("fixture record admits");
    }
    let surface = source["surface"].as_str().expect("fixture surface");
    let controller = source["controller"].as_str().expect("fixture controller");
    let mut tree = UiTree::new();
    tree.publish_document(document);
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(GENERATION);
    let mut complete = false;
    for _ in 0..4096 {
        match tree.step_document_reconcile(&mut cursor, surface, controller) {
            UiDocumentReconcileStep::Pending => {}
            UiDocumentReconcileStep::Complete => {
                complete = true;
                break;
            }
            UiDocumentReconcileStep::Fault(fault) => panic!("the table document must mount cleanly, got {fault:?}"),
        }
    }
    assert!(complete, "document reconcile terminates inside its node budget");
    let root = tree.root.expect("mounted table root");
    let viewport = &law["viewport"];
    assert!(layout_tree_now(&mut tree, root, Theme::default(), number(viewport, "width"), number(viewport, "height")), "layout pass");
    (tree, root)
}

fn mounted_node(tree: &UiTree, record: &Value) -> NodeId {
    let id = record["record"].as_u64().expect("fixture record id");
    tree.document_node(UiNodeId(id)).unwrap_or_else(|| panic!("record {id} mounted"))
}

fn fired(commands: &[UiCommand]) -> Vec<String> {
    commands
        .iter()
        .filter_map(|command| match command {
            UiCommand::App { intent, .. } => Some(intent.action.name.as_str().to_string()),
            _ => None,
        })
        .collect()
}

fn glyphs_in(draw: &DrawList, x: f32, y: f32, width: f32, height: f32) -> usize {
    draw.layers
        .iter()
        .flat_map(|layer| layer.ui_instances.iter())
        .filter(|instance| (instance.params[2] - KIND_GLYPH).abs() < 0.01)
        .filter(|instance| {
            let (center_x, center_y) = (instance.rect[0] + instance.rect[2] * 0.5, instance.rect[1] + instance.rect[3] * 0.5);
            center_x >= x && center_x <= x + width && center_y >= y && center_y <= y + height
        })
        .count()
}

#[test]
fn the_grid_metrics_are_the_ones_the_fixture_was_pinned_against() {
    let law = law();
    let pinned = &law["metrics"];
    let theme = Theme::default();
    let metrics = TreeRowMetrics::from_theme(&theme);
    assert!(close(metrics.row_height, number(pinned, "rowHeightPx")));
    assert!(close(metrics.gap, number(pinned, "gapPx")));
    assert!(close(metrics.drag_handle_extent, number(pinned, "actionIconPx")));
    assert!(close(metrics.action_gap, number(pinned, "actionGapPx")));
    assert!(close(theme.control_height_small, number(pinned, "controlHeightSmallPx")));
}

#[test]
fn rows_columns_and_editable_cells_are_laid_out_on_one_grid() {
    let law = law();
    let expected = &law["expected"];
    let (tree, _) = mounted(&law);
    let metrics = TreeRowMetrics::from_theme(&Theme::default());
    let width = number(&law["viewport"], "width");
    let actions = table_actions_width(expected["actionSlots"].as_array().expect("action slots").len(), &metrics);
    assert!(close(actions, number(expected, "actionsWidth")), "actions column {actions}");
    let columns = expected["columns"].as_array().expect("columns");
    for (index, column) in columns.iter().enumerate() {
        let rect = table_column_rect(width, metrics.row_height, index, columns.len(), actions, &metrics);
        assert!(close(rect.x, number(column, "x")) && close(rect.w, number(column, "width")), "column {index}: ({}, {})", rect.x, rect.w);
    }
    for row in expected["rows"].as_array().expect("rows") {
        let (x, y, w, h) = tree.mounted_layout(mounted_node(&tree, row)).expect("row layout");
        assert!(close(x, 0.0) && close(y, number(row, "y")) && close(w, width) && close(h, number(row, "height")), "{}: ({x}, {y}, {w}, {h})", row["key"]);
    }
    for cell in expected["editableCells"].as_array().expect("editable cells") {
        let (x, y, w, h) = tree.mounted_layout(mounted_node(&tree, cell)).expect("cell layout");
        assert!(close(x, number(cell, "x")) && close(y, number(cell, "y")) && close(w, number(cell, "width")) && close(h, number(cell, "height")), "{}: ({x}, {y}, {w}, {h})", cell["key"]);
    }
}

#[test]
fn taffy_lays_the_editable_rows_cells_where_the_grid_does() {
    use taffy::style_helpers::length;
    let law = law();
    let metrics = TreeRowMetrics::from_theme(&Theme::default());
    let width = number(&law["viewport"], "width");
    let actions = number(&law["expected"], "actionsWidth");
    let cells = law["expected"]["editableCells"].as_array().expect("editable cells");
    let mut solver: taffy::TaffyTree<()> = taffy::TaffyTree::new();
    solver.disable_rounding();
    let leaves = cells
        .iter()
        .map(|cell| solver.new_leaf(taffy::Style { flex_grow: 1.0, flex_shrink: 1.0, size: taffy::geometry::Size { width: length(0.0), height: length(number(cell, "height")) }, ..taffy::Style::default() }).expect("taffy leaf"))
        .collect::<Vec<_>>();
    let row = solver
        .new_with_children(
            taffy::Style {
                display: taffy::style::Display::Flex,
                flex_direction: taffy::style::FlexDirection::Row,
                align_items: Some(taffy::style::AlignItems::Center),
                gap: taffy::geometry::Size { width: length(metrics.gap), height: length(0.0) },
                padding: taffy::geometry::Rect { left: length(metrics.gap), right: length(actions), top: length(0.0), bottom: length(0.0) },
                size: taffy::geometry::Size { width: length(width), height: length(metrics.row_height) },
                ..taffy::Style::default()
            },
            &leaves,
        )
        .expect("taffy row");
    solver.compute_layout(row, taffy::geometry::Size::MAX_CONTENT).expect("taffy solves the row");
    for (cell, leaf) in cells.iter().zip(leaves) {
        let solved = solver.layout(leaf).expect("taffy layout");
        assert!(
            close(solved.location.x, number(cell, "x")) && close(solved.location.y, number(cell, "y")) && close(solved.size.width, number(cell, "width")) && close(solved.size.height, number(cell, "height")),
            "taffy {}: ({}, {}, {}, {})",
            cell["key"],
            solved.location.x,
            solved.location.y,
            solved.size.width,
            solved.size.height
        );
    }
}

#[test]
fn every_action_slot_sits_at_the_inline_end_of_its_row() {
    let law = law();
    let metrics = TreeRowMetrics::from_theme(&Theme::default());
    let width = number(&law["viewport"], "width");
    for slot in law["expected"]["actionSlots"].as_array().expect("action slots") {
        let rect = tree_row_action_rect(width, metrics.row_height, slot["slot"].as_u64().expect("slot") as usize, 0.0, &metrics);
        assert!(close(rect.x, number(slot, "x")) && close(rect.y, number(slot, "y")) && close(rect.w, number(slot, "size")) && close(rect.h, number(slot, "size")), "slot {}: ({}, {}, {}, {})", slot["slot"], rect.x, rect.y, rect.w, rect.h);
    }
}

#[test]
fn a_click_on_an_action_icon_fires_that_action_and_anywhere_else_on_the_row_its_activation() {
    let law = law();
    let (mut tree, root) = mounted(&law);
    for click in law["expected"]["clicks"].as_array().expect("clicks") {
        let (x, y) = (number(click, "x"), number(click, "y"));
        let mut router = EventRouter::new("main");
        router.dispatch(&mut tree, root, &UiEvent::PointerDown { x, y, button: PointerButton::Primary, modifiers: Default::default() });
        let commands = router.dispatch(&mut tree, root, &UiEvent::PointerUp { x, y, button: PointerButton::Primary, modifiers: Default::default() });
        assert_eq!(fired(&commands), vec![click["fires"].as_str().expect("fires").to_string()], "click ({x}, {y}): {}", click["why"]);
    }
}

#[test]
fn the_table_paints_its_header_and_plain_cells_once_in_their_columns_and_leaves_editable_cells_to_their_children() {
    let law = law();
    let (tree, root) = mounted(&law);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let layout = tree.accepted_layout(root).expect("table layout");
    let absolute = tree.absolute_rect(root).expect("table bounds");
    let mut draw = DrawList::default();
    let mut cursor = RetainedNodePaintCursor::default();
    let mut complete = false;
    for _ in 0..16_384 {
        match paint_node_step_with_driver(&tree, root, absolute.x - layout.x, absolute.y - layout.y, &theme, &mut atlas, None, false, UiDriverDrag::Handle, false, ui_contract::FlowInline::Ltr, &mut draw, &mut cursor) {
            RetainedNodePaintStep::Pending => {}
            RetainedNodePaintStep::Complete => {
                complete = true;
                break;
            }
            RetainedNodePaintStep::Fault => panic!("table paint fault"),
        }
    }
    assert!(complete, "the table paints inside its fixed budget");
    for text in law["expected"]["paintedText"].as_array().expect("painted text") {
        let value = text["text"].as_str().expect("text");
        let glyphs = glyphs_in(&draw, number(text, "x"), number(text, "y"), number(text, "width"), number(text, "height"));
        assert_eq!(glyphs, value.chars().filter(|scalar| !scalar.is_whitespace()).count(), "{value} paints once, in its column");
    }
    let band = &law["expected"]["unpaintedBand"];
    assert_eq!(glyphs_in(&draw, 0.0, number(band, "y"), number(&law["viewport"], "width"), number(band, "height")), 0, "{}", band["why"]);
}

#[test]
fn every_row_action_is_an_announced_button_that_fires_its_own_binding() {
    let law = law();
    let (mut tree, _) = mounted(&law);
    let projection = accessibility_projection(&tree);
    for expected in law["expected"]["accessibility"].as_array().expect("accessibility") {
        let key = expected["key"].as_str().expect("key");
        let node = projection.iter().find(|node| node.key == key).unwrap_or_else(|| panic!("{key} is projected"));
        assert_eq!((node.role.as_str(), node.label.as_deref(), node.actionable), (expected["role"].as_str().expect("role"), expected["label"].as_str(), true), "{key}");
    }
    let activation = &law["expected"]["accessibilityActivation"];
    let key = activation["key"].as_str().expect("activation key");
    let record_id = UiNodeId(activation["record"].as_u64().expect("activation record"));
    let index = {
        let record = tree.document().and_then(|document| document.record(record_id)).expect("row record");
        assert_eq!(row_accessibility_action(record, &format!("{}::row-action::9", record.key.as_str())), None, "a key naming no action activates nothing");
        row_accessibility_action(record, key).expect("the virtual key names one of the row's actions")
    };
    let row = mounted_node(&tree, activation);
    let mut router = EventRouter::new("main");
    let commands = router.dispatch_accessibility_row_action(&mut tree, row, index, &AccessibilityUiEvent::Activate);
    assert_eq!(fired(&commands), vec![activation["fires"].as_str().expect("fires").to_string()]);
}
'''


def binding(scope, name, args):
    return {"trigger": "activate", "action": {"scope": scope, "name": name, "version": 1}, "args": args}


def row_action(icon, label, scope, name, args):
    return {"icon": icon, "label": label, "action": binding(scope, name, args)}


LEAF = {"kind": "leaf", "width": "hug", "height": "hug"}
VERTICAL = {"kind": "stack", "axis": "vertical", "gap": "none", "padding": {"all": "none"}, "align": "stretch", "justify": "start", "wrap": False, "grow": True}
HOME = "framework.home"
CSV = "s.stdio.csv@rfc4180/*#editor"


def record(identity, key, component, children=None, bindings=None, accessibility=None, layout=None):
    value = {"id": identity, "key": key}
    if children:
        value["children"] = children
    value["component"] = component
    if bindings:
        value["bindings"] = bindings
    value.update({"layout": layout or LEAF, "style": {}, "activity": "idle", "accessibility": accessibility or {}})
    return value


FIXTURE = {
    "provenance": {
        "what": "The language-neutral oracle for a windowed table on the wgpu target: a published Table document (a plain row with two row actions and an activation, a plain row with one, an editable row with one child per materialised cell) -> the header band, every row's rect, every cell column, every trailing action slot, what a pointer on each fires, the text the table itself paints and the row-action buttons the accessibility projection announces.",
        "why": "The wgpu reconcile painted a childless TableRow as ONE button labelled with its joined cells and a TableRow with children as a bare horizontal stack (cells dropped, no header); TableRowProps.rowActions were never painted, so the SDK duplicated every row action as a row-action child button and Home's 32-row window overflowed its item budget after 27 rows. Ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP session 14, WG11 x LB2 TableRow agreement.",
        "rule": "A table is ONE column grid: the header, the painted cells and an editable row's cell children share layout::table_column_rect; row actions are the RowAction props only, painted, hit and announced at layout::tree_row_action_rect, each firing its own versioned binding.",
        "implementations": {"rust": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/📊️table-row-grid/🦀️.rs", "thirdParty": "taffy (flexbox) solves the editable row's cells in the same law"},
    },
    "metrics": {"source": "🖱️ui/🎨️styling tokens x uiSpacingCompactPx 3.2: treeRowUiSpacing 7.5, gapStandard 1, paddingStandard 1, iconTiny 3.75, controlHeightSmall 5", "rowHeightPx": 24.0, "gapPx": 3.2, "actionIconPx": 12.0, "actionGapPx": 3.2, "controlHeightSmallPx": 16.0},
    "viewport": {"width": 480.0, "height": 240.0},
    "document": {
        "surface": "home.spaces",
        "controller": HOME,
        "revision": 3,
        "root": 0,
        "layoutEpoch": 0,
        "nodes": [
            record(0, "spaces", {"type": "table", "label": "Spaces", "columns": ["Name", "Kind"], "actionsLabel": "Actions", "window": {"total": 3, "offset": 0, "rowExtent": "standard"}}, children=[1, 2, 3], layout=VERTICAL),
            record(
                1,
                "space:alpha",
                {"type": "tableRow", "cells": ["Alpha", "Space"], "rowActions": [row_action("settings", "Settings", HOME, "open-space-settings", {"space": "alpha"}), row_action("trash-2", "Delete", HOME, "delete-space", {"space": "alpha"})]},
                bindings=[binding(HOME, "open-space", {"space": "alpha"})],
            ),
            record(2, "space:beta", {"type": "tableRow", "cells": ["Beta", "Space"], "rowActions": [row_action("trash-2", "Delete", HOME, "delete-space", {"space": "beta"})]}, bindings=[binding(HOME, "open-space", {"space": "beta"})]),
            record(3, "row-2", {"type": "tableRow", "cells": ["Ada", "42"]}, children=[4, 5]),
            record(
                4,
                "cell-0",
                {"type": "input", "kind": "text", "value": "Ada", "commit": "blur"},
                bindings=[{"trigger": "commit", "action": {"scope": CSV, "name": "set-cell", "version": 1}, "args": {"column": 0, "revision": "0123456789abcdef", "row": 2}}],
                accessibility={"label": "Name"},
            ),
            record(5, "cell-1", {"type": "text", "value": "42"}),
        ],
    },
    "expected": {
        "actionsWidth": 33.6,
        "columns": [{"x": 3.2, "width": 220.0}, {"x": 226.4, "width": 220.0}],
        "rows": [{"record": 1, "key": "space:alpha", "y": 24.0, "height": 24.0}, {"record": 2, "key": "space:beta", "y": 48.0, "height": 24.0}, {"record": 3, "key": "row-2", "y": 72.0, "height": 24.0}],
        "editableCells": [{"record": 4, "key": "cell-0", "x": 3.2, "y": 4.0, "width": 220.0, "height": 16.0}, {"record": 5, "key": "cell-1", "x": 226.4, "y": 0.0, "width": 220.0, "height": 24.0}],
        "actionSlots": [{"slot": 1, "x": 461.6, "y": 6.0, "size": 12.0}, {"slot": 2, "x": 446.4, "y": 6.0, "size": 12.0}],
        "clicks": [
            {"x": 467.6, "y": 36.0, "fires": "delete-space", "why": "slot 1 of space:alpha holds its LAST action"},
            {"x": 452.4, "y": 36.0, "fires": "open-space-settings", "why": "slot 2 of space:alpha holds its first action"},
            {"x": 100.0, "y": 36.0, "fires": "open-space", "why": "the row itself fires its Trigger::Activate binding"},
            {"x": 467.6, "y": 60.0, "fires": "delete-space", "why": "slot 1 of space:beta holds its only action"},
            {"x": 452.4, "y": 60.0, "fires": "open-space", "why": "an empty slot is the row"},
        ],
        "paintedText": [
            {"text": "Name", "x": 3.2, "y": 0.0, "width": 220.0, "height": 24.0},
            {"text": "Kind", "x": 226.4, "y": 0.0, "width": 220.0, "height": 24.0},
            {"text": "Alpha", "x": 3.2, "y": 24.0, "width": 220.0, "height": 24.0},
            {"text": "Space", "x": 226.4, "y": 24.0, "width": 220.0, "height": 24.0},
            {"text": "Beta", "x": 3.2, "y": 48.0, "width": 220.0, "height": 24.0},
            {"text": "Space", "x": 226.4, "y": 48.0, "width": 220.0, "height": 24.0},
        ],
        "unpaintedBand": {"y": 72.0, "height": 24.0, "why": "the editable row's cells are its children, which paint themselves; the table paints none of their text"},
        "accessibility": [
            {"key": "space:alpha::row-action::0", "role": "button", "label": "Settings: Alpha"},
            {"key": "space:alpha::row-action::1", "role": "button", "label": "Delete: Alpha"},
            {"key": "space:beta::row-action::0", "role": "button", "label": "Delete: Beta"},
        ],
        "accessibilityActivation": {"record": 1, "key": "space:alpha::row-action::1", "fires": "delete-space"},
    },
}

NEW_FILES = {GRID_LAWS: GRID_LAWS_SOURCE, GRID_FIXTURE: json.dumps(FIXTURE, ensure_ascii=False, indent=2) + "\n"}


def replaced(path, source, edits):
    for old, new in edits:
        count = source.count(old)
        if count != 1:
            sys.exit(f"anchor occurs {count}x in {path.parent.name}/{path.name}: {old[:100]!r}")
        source = source.replace(old, new)
    return source


def main():
    revert = "--revert" in sys.argv
    apply = "--apply" in sys.argv or revert
    for path in NEW_FILES:
        if revert and not path.exists():
            sys.exit(f"{path} absent — nothing to revert")
        if not revert and path.exists():
            sys.exit(f"{path} exists — landed already")
    plans = []
    for path, edits in EDITS.items():
        before = path.read_text(encoding="utf-8")
        after = replaced(path, before, [(new, old) for old, new in reversed(edits)] if revert else edits)
        plans.append((path, before, after))
    for path, before, after in plans:
        sys.stdout.writelines(difflib.unified_diff(before.splitlines(True), after.splitlines(True), f"{path.parent.name}/{path.name}", f"{path.parent.name}/{path.name} (patched)", n=1))
    for path, content in NEW_FILES.items():
        print(f"\n{'--- REMOVE' if revert else '+++ NEW'} {path.relative_to(ROOT)} ({len(content)} chars, {content.count(chr(10))} lines)")
    if apply:
        for path, _, after in plans:
            path.write_text(after, encoding="utf-8")
        for path, content in NEW_FILES.items():
            if revert:
                path.unlink()
                path.parent.rmdir()
            else:
                path.parent.mkdir(parents=True, exist_ok=False)
                path.write_text(content, encoding="utf-8")
    print(f"\n{'REVERTED' if revert else 'APPLIED' if apply else 'DRY RUN'}: {len(plans)} files edited, {len(NEW_FILES)} new")


if __name__ == "__main__":
    main()
