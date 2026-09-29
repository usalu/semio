//! 🪟️ LAW (ticket 26/09/23 session 14d, WG11 P5, coordinator 07:4x): a 10 000-row windowed table streams to its END on the wgpu
//! retained engine — the retained layout pitches the unmaterialised rows as spacers (so the scroll extent spans the whole list),
//! `tree_window_measures` reads the mounted rows against the scroll viewport, the one request rule asks the window the viewport
//! needs, and a guest answering that window (capped by its own ledger) republishes; scrolled to the bottom, the LAST row is
//! materialised and painted inside the viewport, and no request ever exceeds the body budget.
use crate::wgpu::arena::NodeId;
use crate::wgpu::mounted_layout::{layout_tree_now, tree_window_measures};
use crate::wgpu::reconcile::{UiDocumentReconcileCursor, UiDocumentReconcileStep};
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{UiDocumentTree, UiTree};
use crate::wgpu::tree_window::{served_requests, TreeWindowServedMemory};
use ui_contract::{EdgeSpace, LayoutSpec, ScrollAxes, ScrollLayout, Sizing, SurfaceId, UiDocumentLeaseHeader, UiNodeId, UiNodeRecord, UiRevision, TREE_WINDOW_BODY_NODE_BUDGET};

const SURFACE: &str = "wg11.streaming";
const TOTAL: u32 = 10_000;
const VIEWPORT: (f32, f32) = (480.0, 480.0);
const GUEST_CAPACITY: u32 = 60;

fn record(value: serde_json::Value) -> UiNodeRecord {
    serde_json::from_value(value).expect("a streaming fixture record deserializes against the contract")
}

/// 📃️ The guest's answer to one window: a scroll root holding one table whose rows `[offset, offset + rows)` are materialised.
fn document(generation: u64, offset: u32, rows: u32) -> UiDocumentTree {
    let rows: Vec<u32> = (offset..offset + rows).collect();
    let mut root = record(serde_json::json!({ "id": 0, "key": "stream", "component": { "type": "container" }, "layout": { "kind": "leaf", "width": "fill", "height": "fill" }, "style": {}, "activity": "idle", "accessibility": {}, "children": [1] }));
    root.layout = LayoutSpec::Scroll(ScrollLayout { axes: ScrollAxes::Vertical, padding: EdgeSpace::default(), sizing: Sizing::Fill });
    let table = record(serde_json::json!({
        "id": 1, "key": "rows",
        "component": { "type": "table", "label": "Rows", "columns": ["Name"], "window": { "total": TOTAL, "offset": offset, "rowExtent": "standard" } },
        "layout": { "kind": "stack", "axis": "vertical", "gap": "none", "padding": { "all": "none" }, "align": "stretch", "justify": "start", "wrap": false, "grow": false },
        "style": {}, "activity": "idle", "accessibility": {},
        "children": (0..rows.len()).map(|position| position + 2).collect::<Vec<_>>()
    }));
    let mut document = UiDocumentTree::new(UiDocumentLeaseHeader { generation, surface: SurfaceId::try_from(SURFACE).expect("surface id"), revision: UiRevision(generation), root: UiNodeId(0), layout_epoch: generation, node_count: rows.len() + 2 })
        .expect("the streaming header admits");
    document.try_upsert_record(root).expect("root admits");
    document.try_upsert_record(table).expect("table admits");
    for (position, row) in rows.iter().enumerate() {
        document
            .try_upsert_record(record(serde_json::json!({ "id": position + 2, "key": format!("row-{row}"), "component": { "type": "tableRow", "cells": [format!("Row {row}")] }, "layout": { "kind": "leaf", "width": "hug", "height": "hug" }, "style": {}, "activity": "idle", "accessibility": {} })))
            .expect("row admits");
    }
    document
}

fn publish(tree: &mut UiTree, generation: u64, offset: u32, rows: u32) -> NodeId {
    tree.publish_document(document(generation, offset, rows));
    let mut cursor = UiDocumentReconcileCursor::default();
    cursor.rearm(generation);
    let complete = (0..65_536).any(|_| match tree.step_document_reconcile(&mut cursor, SURFACE, "wg11") {
        UiDocumentReconcileStep::Pending => false,
        UiDocumentReconcileStep::Complete => true,
        UiDocumentReconcileStep::Fault(fault) => panic!("the streaming document must mount, got {fault:?}"),
    });
    assert!(complete, "reconcile terminates");
    let root = tree.root.expect("mounted root");
    assert!(layout_tree_now(tree, root, Theme::default(), VIEWPORT.0, VIEWPORT.1), "layout pass");
    root
}

fn scroll_to(tree: &mut UiTree, root: NodeId, y: f32) {
    let node = tree.node_mut(root).expect("the scroll root");
    node.state.scroll_offset = (0.0, y.max(0.0));
}

#[test]
fn a_ten_thousand_row_table_streams_to_its_last_row() {
    let mut tree = UiTree::new();
    let mut generation = 1;
    let (mut offset, mut rows) = (0u32, 48u32);
    let root = publish(&mut tree, generation, offset, rows);
    let table = tree.children(root).next().expect("the table node");
    let extent = tree.accepted_layout(table).expect("the table is laid out").height;
    assert!(extent >= TOTAL as f32 * 24.0, "the spacers make the table span every row: {extent}");
    let mut memory: Vec<(String, TreeWindowServedMemory)> = Vec::new();
    for target in [0.0, extent * 0.5, extent] {
        let mut settled = false;
        for _ in 0..16 {
            let root = tree.root.expect("mounted root");
            scroll_to(&mut tree, root, (target - VIEWPORT.1).min(target));
            let (viewport, containers) = tree_window_measures(&tree, &Theme::default(), VIEWPORT, false).expect("the windowed table is measured");
            let (asked, kept) = served_requests(&containers, viewport, &memory, TREE_WINDOW_BODY_NODE_BUDGET);
            memory = kept;
            let cost: usize = asked.iter().map(|request| 1 + request.rows as usize).sum();
            assert!(cost <= TREE_WINDOW_BODY_NODE_BUDGET, "a request never exceeds the guest ledger: {cost}");
            let request = asked.iter().find(|request| request.key == "rows").expect("the table asks a window");
            let answer_offset = request.offset.min(TOTAL - request.rows.max(1));
            let answer_rows = request.rows.min(TOTAL - answer_offset).min(GUEST_CAPACITY);
            settled = (answer_offset, answer_rows) == (offset, rows);
            if settled {
                break;
            }
            (offset, rows) = (answer_offset, answer_rows);
            generation += 1;
            let root = publish(&mut tree, generation, offset, rows);
            scroll_to(&mut tree, root, (target - VIEWPORT.1).min(target));
        }
        assert!(settled, "the window settles at scroll {target}");
    }
    assert_eq!(offset + rows, TOTAL, "scrolled to the end, the window holds the last row");
    let root = tree.root.expect("mounted root");
    let table = tree.children(root).next().expect("the table node");
    let last = tree.children(table).last().expect("materialised rows");
    let frame = tree.absolute_rect(root).expect("the scroll viewport");
    let painted = tree.absolute_rect(last).expect("the last row is laid out");
    assert!(painted.y >= frame.y - 0.5 && painted.y + painted.h <= frame.y + frame.h + 0.5, "the last row paints inside the viewport: {painted:?} in {frame:?}");
}
