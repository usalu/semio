/// 🗜️ Closing, collapse and split resize: closing the last, active and only tabs, single child hoisting, inert maximization and the split resize totals, deltas and gutter hits.

/// 📑️ `modeDockTabClassName` caps a tab at `max-w-[12rem]` and truncates its label
/// (`🎛️chrome-control-presentation/🟦️.ts:35`) — a long title must never widen the tab bar.
#[test]
fn dock_tab_label_truncates_at_reacts_twelve_rem_cap() {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let long = "Generation Preview Viewport With A Very Long Window Title";
    let (display, width) = dock_tab_chip(&mut atlas, &theme, long, 2);
    assert!((width - MODE_DOCK_TAB_MAX_WIDTH_PX).abs() < 0.001, "a long tab is pinned to 12rem, got {width}");
    assert!(display.ends_with('…'), "truncation appends an ellipsis, got {display:?}");
    assert!(display.len() < long.len());
    let short = "Main";
    let (display_short, width_short) = dock_tab_chip(&mut atlas, &theme, short, 2);
    assert_eq!(display_short, short, "a short label is never truncated");
    assert!(width_short < MODE_DOCK_TAB_MAX_WIDTH_PX);
    assert_eq!(truncate_label_to_width(&mut atlas, short, theme.font_size_small, 10_000.0), short);
}

/// 🪟️ Closing the LAST tab of a stack collapses the split it lived in — React's
/// `collapseLayout(removeWindowFromLayout(prev, id))` (`🎨️Canvas/🟦️.tsx:1475-1485`). This used to be
/// a silent no-operation (`if windows.len() <= 1 { return false }`).
#[test]
fn closing_the_last_tab_of_a_stack_collapses_the_layout() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    assert!(dock.close_window_in_stack(&vec![0], "a"), "React always allows close");
    assert_eq!(dock.root, stack_with("b"), "the single surviving child is hoisted into the root");
    assert_eq!(dock.active_window_id.as_deref(), Some("b"));
    assert_eq!(dock.active_stack, Some(vec![]));
}

/// 🪟️ React re-focuses `children[0]` of the stack, and the canvas re-focuses `remaining[0]`
/// (`🎨️Canvas/🟦️.tsx:313, 1481`) — not the tab to the left of the one that closed.
#[test]
fn closing_the_active_tab_focuses_the_first_remaining_window() {
    let mut dock = dock_with(stack_tabs(&["a", "b", "c"], "b"), "b");
    assert!(dock.close_window_in_stack(&vec![], "b"));
    assert_eq!(dock.root, stack_tabs(&["a", "c"], "a"));
    assert_eq!(dock.active_window_id.as_deref(), Some("a"));
}

/// 🪟️ Closing the only window leaves React's empty root stack and no active window.
#[test]
fn closing_the_only_window_empties_the_dock() {
    let mut dock = dock_with(stack_tabs(&["a"], "a"), "a");
    assert!(dock.close_window_in_stack(&vec![], "a"));
    assert_eq!(dock.root, DockNode::Stack { windows: vec![], active: String::new() });
    assert_eq!(dock.active_window_id, None);
    assert_eq!(dock.active_stack, None);
    assert!(!dock.close_window("a"), "a window that is not in the layout cannot be closed");
}

/// 🪟️ `collapseLayout` hoists a single-child axis and keeps `only.size ?? node.size`
/// (`🎨️Canvas/🟦️.tsx:225-236`) — the hoisted child carries the ratio it held among its own siblings.
#[test]
fn collapse_hoists_a_single_child_axis_keeping_the_childs_ratio() {
    let inner = DockNode::Row(vec![(stack_with("a"), 0.25), (stack_with("b"), 0.75)]);
    let mut dock = dock_with(DockNode::Column(vec![(inner, 0.4), (stack_with("c"), 0.6)]), "c");
    assert!(dock.close_window_in_stack(&vec![0, 0], "a"));
    assert_eq!(dock.root, DockNode::Column(vec![(stack_with("b"), 0.75), (stack_with("c"), 0.6)]), "the emptied row is hoisted away and b keeps its own 0.75");
    assert_eq!(dock.active_window_id.as_deref(), Some("c"), "closing a non-active window leaves focus alone");
}

/// 🔲️ React drops a stale maximized path the moment the canvas falls back to one window
/// (`🎨️Canvas/🟦️.tsx:1802-1805`), and never offers maximize for a lone window at all.
#[test]
fn maximize_is_inert_and_self_clearing_for_a_single_window_canvas() {
    let mut single = dock_with(stack_tabs(&["a"], "a"), "a");
    single.toggle_maximize(&vec![]);
    assert_eq!(single.maximized_stack, None, "a lone window already fills the canvas");
    let mut two = dock_with(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), "a");
    two.toggle_maximize(&vec![1]);
    assert_eq!(two.maximized_stack, Some(vec![1]));
    assert!(two.close_window_in_stack(&vec![0], "a"));
    assert_eq!(two.maximized_stack, None, "the canvas is down to one window, so the maximized path is dropped");
}

/// ↔️ `applyAxisResizeDelta` conserves the dragged PAIR's total and floors each side at `minPct = 8`
/// (`🎨️Canvas/🟦️.tsx:593-611`).
#[test]
fn split_resize_conserves_the_pair_total_and_floors_at_eight_percent() {
    let mut dock = dock_with(DockNode::Row(vec![(stack_with("a"), 0.25), (stack_with("b"), 0.25), (stack_with("c"), 0.5)]), "a");
    dock.begin_split_drag(&vec![]);
    dock.apply_split_drag(&vec![], 0, 100.0, 1000.0);
    let DockNode::Row(children) = &dock.root else { panic!("row") };
    assert!((children[0].1 - 0.35).abs() < 1e-5);
    assert!((children[1].1 - 0.15).abs() < 1e-5);
    assert!((children[2].1 - 0.5).abs() < 1e-5, "the untouched sibling keeps its share");
    dock.begin_split_drag(&vec![]);
    dock.apply_split_drag(&vec![], 0, 10_000.0, 1000.0);
    let DockNode::Row(children) = &dock.root else { panic!("row") };
    assert!((children[0].1 + children[1].1 - 0.5).abs() < 1e-5, "the pair total survives the clamp");
    assert!((children[1].1 - SPLIT_MIN_FRACTION).abs() < 1e-5, "the squeezed side floors at React's 8 %");
    assert!((children[2].1 - 0.5).abs() < 1e-5, "and the clamp never restretches the rest of the axis");
}

/// 📏️ Authored window layouts retain React's percentage weights, so a physical pointer delta must
/// be converted into that axis's captured weight scale before the split is solved.
#[test]
fn split_resize_moves_the_separator_by_the_pointer_delta_on_react_percentage_weights() {
    let repo = fixture_path!("../../../../../../../../../..");
    let fixture_path = repo.join("🧰️framework/🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/📐️dock-axis-geometry/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(&fixture_path).unwrap_or_else(|error| panic!("read {}: {error}", fixture_path.display()))).expect("dock axis fixture");
    let oracle = &fixture["resizeOracle"];
    let axis_extent = oracle["axisExtentPixels"].as_f64().expect("axis extent") as f32;
    let delta = oracle["deltaPixels"].as_f64().expect("delta pixels") as f32;
    let before: Vec<f32> = oracle["beforeWeights"].as_array().expect("before weights").iter().map(|value| value.as_f64().expect("weight") as f32).collect();
    let after: Vec<f32> = oracle["afterWeights"].as_array().expect("after weights").iter().map(|value| value.as_f64().expect("weight") as f32).collect();
    let mut dock = dock_with(DockNode::Row(before.iter().enumerate().map(|(index, weight)| (stack_with(&format!("window-{index}")), *weight)).collect()), "window-0");
    let canvas = Rect::new(0.0, 0.0, axis_extent, 200.0);
    let before_frame = dock.stack_frame_rects_with_separator(canvas, 0.0)[0].1;
    let before_separator = before_frame.x + before_frame.w;
    let origin = dock.begin_split_drag(&vec![]);
    dock.apply_split_drag_with_origin(&vec![], 0, delta, axis_extent, &origin);
    let after_frame = dock.stack_frame_rects_with_separator(canvas, 0.0)[0].1;
    let after_separator = after_frame.x + after_frame.w;
    let DockNode::Row(children) = &dock.root else { panic!("row") };
    for ((_, actual), expected) in children.iter().zip(after) {
        assert!((*actual - expected).abs() < 0.0001, "weight {actual} != {expected}");
    }
    assert!((after_separator - before_separator - oracle["expectedSeparatorDeltaPixels"].as_f64().expect("separator delta") as f32).abs() < 0.001);
}

/// ↔️ Resize gutters: React's `Resizable` separator is a thin visual line with a fat grab zone; wgpu
/// pins the visual at 6 px and the hit at 20 px, centred on the seam, plus 10 px join-corner squares
/// mirroring `modeJoinCornerSpecsForSeparator` (`🎨️Canvas/🟦️.tsx:515-580`).
///
/// 🎯️ `hit_at` resolves the last COMPLETE frame's registry; a walk that has only just staged its
/// targets reads them back through `staged_hits` (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:331-340`).
#[test]
fn split_resize_gutter_hit_is_twenty_pixels_centred_on_the_seam() {
    let dock = dock_with(even_layout(&["a".into(), "b".into()]), "a");
    let canvas = Rect::new(0.0, 0.0, 400.0, 300.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::new();
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &IconAtlas::default(), input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &HashMap::new(), control_names: None };
    dock.register_resize_hits(&mut ctx, canvas);
    let hit = input.staged_hits().iter().find(|target| target.kind == HitKind::DockSplit).expect("split hit on the seam");
    assert_eq!(hit.control_id.as_deref(), Some("dock.split..0"));
    assert!((hit.rect.w - 20.0).abs() < 0.001, "20 px grab zone");
    assert!((hit.rect.x + hit.rect.w * 0.5 - 200.0).abs() < 0.001, "centred on the seam");
    assert!(hit.rect.contains(200.0, 150.0), "the seam itself is inside the grab zone");
    assert!(!hit.rect.contains(188.0, 150.0), "12 px off the seam is outside it");
}
