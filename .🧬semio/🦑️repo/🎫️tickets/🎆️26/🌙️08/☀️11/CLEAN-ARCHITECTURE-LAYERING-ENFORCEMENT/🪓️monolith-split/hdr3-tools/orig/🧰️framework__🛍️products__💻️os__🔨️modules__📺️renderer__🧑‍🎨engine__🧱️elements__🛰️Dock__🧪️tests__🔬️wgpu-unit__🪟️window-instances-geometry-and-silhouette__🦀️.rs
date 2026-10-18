/// 🪟️ Window instances, geometry and silhouette: concrete window instances, stale kind focus, split extents, stack content rects, the icon slot, the silhouette clip, owned chips and the cap depth.

#[test]
fn concrete_window_instances_round_trip_without_kind_collapse() {
    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ExpectedWindow {
        id: String,
        window_kind_id: String,
    }
    #[derive(serde::Deserialize)]
    struct Fixture {
        layout: WindowLayout,
        expected: Vec<ExpectedWindow>,
    }
    let fixture: Fixture = serde_json::from_str(fixture_text!("../../../../../🧱️elements/🛰️Dock/🧪️tests/🧫️fixtures/🪟️concrete-window-instances.json")).expect("concrete window fixture");
    let mut dock = DockState::from_app(&sample_app(&["canvas"], Some(fixture.layout)), Some("canvas-copy"));
    let actual = dock.window_instances();
    assert_eq!(actual, fixture.expected.iter().map(|window| (window.id.clone(), window.window_kind_id.clone())).collect::<Vec<_>>());
    assert_eq!(dock.active_window_id.as_deref(), Some("canvas-copy"));
    let DockNode::Stack { windows, .. } = &dock.root else { panic!("fixture root must be a stack") };
    let labels = HashMap::from([("canvas".to_string(), "Canvas".to_string())]);
    let mut atlas = FontAtlas::builtin();
    let chrome = layout_stack_cap(windows, &labels, &HashMap::new(), &mut atlas, &Theme::default(), Rect::new(0.0, 0.0, 640.0, 480.0), 0);
    assert_eq!(chrome.groups[0].tabs.iter().map(|tab| tab.label.as_str()).collect::<Vec<_>>(), vec!["Canvas", "Canvas"]);
    let payload = DockDragPayload { kind: DockDragKind::Tab, window_id: "canvas-copy".into(), window_kind_id: "canvas".into(), template_id: None, source_path: Vec::new(), tab_index: 1, ghost_label: "Canvas copy".into() };
    assert!(dock.apply_drop(&payload, &DockDropZone::RootSplit { side: DockSide::Right }));
    assert_eq!(dock.window_kind_id("canvas-copy"), Some("canvas"));
    let persisted = serde_json::to_value(dock.to_window_layout()).expect("persisted layout");
    assert!(persisted.to_string().contains("canvas-copy"));
}

#[test]
fn stale_kind_focus_falls_back_to_the_first_authored_instance() {
    let root = DockNode::Row(vec![
        (DockNode::Stack { windows: vec![DockStackTab::instance("canvas-top", "canvas", WindowStackCorner::TopLeft)], active: "canvas-top".into() }, 1.0),
        (DockNode::Stack { windows: vec![DockStackTab::instance("canvas-perspective", "canvas", WindowStackCorner::TopLeft)], active: "canvas-perspective".into() }, 2.0),
    ]);
    let app = sample_app(&["canvas"], Some(WindowLayout { root: dock_node_to_layout_root(&root) }));
    let dock = DockState::from_app(&app, Some("canvas"));
    assert_eq!(dock.active_window_id.as_deref(), Some("canvas-top"));
    assert_eq!(dock.active_stack.as_deref(), Some([0].as_slice()));
}

#[test]
fn split_axis_extent_uses_row_width_not_canvas_max() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = DockNode::Column(vec![(DockNode::Row(vec![(stack_with("a"), 0.5), (stack_with("b"), 0.5)]), 0.5), (stack_with("c"), 0.5)]);
    let canvas = Rect::new(0.0, 0.0, 1000.0, 800.0);
    let separator = Theme::default().gap_standard;
    let row_extent = dock.split_axis_extent(&vec![0], canvas, separator).unwrap();
    assert!((row_extent - (1000.0 - separator)).abs() < 0.1);
    let col_extent = dock.split_axis_extent(&vec![], canvas, separator);
    assert!((col_extent.unwrap() - (800.0 - separator)).abs() < 0.1);
    let nested_extent = dock.split_axis_extent(&vec![0], canvas, separator).unwrap();
    assert!((nested_extent - (1000.0 - separator)).abs() < 0.1);
}

#[test]
fn row_layout_stack_content_rects_match_per_window() {
    let layout = create_default_layout(&["flow".into(), "preview".into()], "row", Some(&[68.0, 32.0]), Some(&["Flow".into(), "Preview".into()]));
    let app = sample_app(&["flow", "preview"], Some(layout));
    let dock = DockState::from_app(&app, Some("flow"));
    let canvas = Rect::new(0.0, 0.0, 1200.0, 800.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let labels = HashMap::from([("flow".into(), "Flow".into()), ("preview".into(), "Preview".into())]);
    let placements = dock.stack_body_rects(canvas, &theme, &labels, &mut atlas);
    assert_eq!(placements.len(), 2);
    let flow_rect = placements.iter().find(|(_, _, id)| id == "flow").map(|(_, rect, _)| *rect);
    let preview_rect = placements.iter().find(|(_, _, id)| id == "preview").map(|(_, rect, _)| *rect);
    let flow_rect = flow_rect.expect("flow body rect");
    let preview_rect = preview_rect.expect("preview body rect");
    assert!(flow_rect.w > preview_rect.w);
    assert!((preview_rect.x - (flow_rect.x + flow_rect.w) - theme.gap_standard - theme.padding_standard * 2.0).abs() < 0.01, "📐️ one separator plus both adjacent body insets separate scene content");
    assert!(flow_rect.h > 0.0 && preview_rect.h > 0.0);
}

#[test]
fn dock_tab_content_width_reserves_icon_slot() {
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let label = "Main";
    let with_icon = dock_tab_content_width(&mut atlas, &theme, label);
    let text_only = atlas.measure_text(label, theme.font_size_small).0 + theme.padding_standard * 2.0;
    assert!(with_icon > text_only);
}

#[test]
fn window_silhouette_clip_union_excludes_cap_gap() {
    let silhouette = WindowSilhouette::from_measured_top(Rect::new(10.0, 20.0, 300.0, 200.0), 80.0, 60.0, 32.0);
    assert_eq!(silhouette.content_clip_rects(), vec![Rect::new(10.0, 52.0, 300.0, 168.0), Rect::new(10.0, 20.0, 80.0, 32.0), Rect::new(250.0, 20.0, 60.0, 32.0)]);
    assert_eq!(silhouette.content_bounds(), Rect::new(10.0, 20.0, 300.0, 200.0));
    assert_eq!(silhouette.safe_body_rect(), Rect::new(10.0, 52.0, 300.0, 168.0));
    assert!(!silhouette.content_clip_rects().iter().any(|rect| rect.contains(150.0, 30.0)));
}

#[test]
fn window_silhouette_v1_matches_typescript_fixture_for_merging_bottom_and_containment() {
    let normalized = WindowSilhouette::new(
        Rect::new(0.0, 0.0, 200.0, 100.0),
        WindowSilhouetteEdge::new(24.0, vec![WindowSilhouetteSpan::new(160.0, 220.0), WindowSilhouetteSpan::new(60.25, 90.0), WindowSilhouetteSpan::new(0.0, 60.0), WindowSilhouetteSpan::new(90.25, 120.0)]),
        WindowSilhouetteEdge::new(16.0, vec![WindowSilhouetteSpan::new(80.0, 120.0), WindowSilhouetteSpan::new(0.0, 40.0)]),
    );
    assert_eq!(normalized.top.spans, vec![WindowSilhouetteSpan::new(0.0, 120.0), WindowSilhouetteSpan::new(160.0, 200.0)]);
    let silhouette = WindowSilhouette::new(
        Rect::new(0.0, 0.0, 200.0, 100.0),
        WindowSilhouetteEdge::new(24.0, vec![WindowSilhouetteSpan::new(160.0, 200.0), WindowSilhouetteSpan::new(0.0, 60.0)]),
        WindowSilhouetteEdge::new(16.0, vec![WindowSilhouetteSpan::new(80.0, 120.0), WindowSilhouetteSpan::new(0.0, 40.0)]),
    );
    assert_eq!(silhouette.glass_regions(), vec![Rect::new(0.0, 0.0, 60.0, 24.0), Rect::new(160.0, 0.0, 40.0, 24.0), Rect::new(0.0, 84.0, 40.0, 16.0), Rect::new(80.0, 84.0, 40.0, 16.0)]);
    assert_eq!(silhouette.safe_clearances(), (24.0, 16.0));
    assert!(silhouette.contains(20.0, 12.0));
    assert!(!silhouette.contains(140.0, 12.0));
    assert!(silhouette.contains(100.0, 50.0));
    assert!(!silhouette.contains(60.0, 92.0));
    assert!(silhouette.contains(100.0, 92.0));
}

#[test]
fn dock_stack_glass_and_hits_exist_only_on_owned_chips() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    let bounds = Rect::new(0.0, 0.0, 600.0, 400.0);
    let theme = Theme::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    let icon_ids = HashMap::new();
    let layout = layout_stack_cap(&tabs(&["a", "b"]), &labels, &icon_ids, &mut atlas, &theme, bounds, 0);
    let gap_point = (bounds.x + bounds.w * 0.5, bounds.y + theme.control_height * 0.5);
    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, false);
    assert_eq!(draw.glass_regions.len(), 1, "one top-left corner group chip for both tabs");
    assert!(!draw.glass_regions.iter().any(|region| Rect::new(region.rect[0], region.rect[1], region.rect[2], region.rect[3]).contains(gap_point.0, gap_point.1)));
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a")));
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a.close")));
    assert!(input.staged_hits().iter().any(|hit| hit.control_id.as_deref() == Some("dock.tab..a.focus")));
    assert!(!input.staged_hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.focus.") || id.starts_with("dock.close."))));
    assert!(!input.staged_hits().iter().any(|hit| hit.control_id.as_deref().is_some_and(|id| id.starts_with("dock.stack."))));
    let _ = layout;
}

#[test]
fn dock_cap_depth_is_control_plus_padding_with_inset_actions_and_active_fill() {
    let mut dock = DockState::from_app(&sample_app(&["a", "b"], None), Some("a"));
    dock.root = stack_tabs(&["a", "b"], "a");
    dock.active_stack = Some(Vec::new());
    let bounds = Rect::new(10.0, 20.0, 600.0, 400.0);
    let mut theme = Theme::default();
    theme.control_height = 24.0;
    theme.padding_standard = 8.0;
    theme.navbar_height = 52.0;
    let cap_depth = 40.0;
    assert_eq!(dock_cap_depth(&theme), cap_depth);
    assert_ne!(dock_cap_depth(&theme), theme.navbar_height);
    assert_eq!(stack_tab_bar_rect(bounds, &theme).h, cap_depth);

    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut draw = DrawList::default();
    let labels = HashMap::from([("a".into(), "A".into()), ("b".into(), "B".into())]);
    let icon_ids = HashMap::new();
    let action_count = dock_tab_actions(dock.show_maximize(), false).len();
    let layout = layout_stack_cap(&tabs(&["a", "b"]), &labels, &icon_ids, &mut atlas, &theme, bounds, action_count);
    assert!(layout.groups.iter().flat_map(|group| &group.tabs).all(|tab| tab.rect.h == cap_depth));
    let silhouette = stack_window_silhouette(bounds, &theme, &layout);
    assert_eq!(silhouette.safe_body_rect().y, bounds.y + cap_depth);
    let active_rect = layout.groups.iter().flat_map(|group| &group.tabs).find(|tab| tab.window_id == "a").expect("active tab").rect;

    let mut ctx = DockRenderContext { draw: &mut draw, atlas: &mut atlas, icons: &icons, input: &mut input, theme: &theme, window_labels: &labels, window_icon_ids: &icon_ids, control_names: None };
    dock.paint_chrome(&mut ctx, bounds, false);
    let selected = [theme.selected.r, theme.selected.g, theme.selected.b, theme.selected.a];
    assert!(
        draw.layers.iter().flat_map(|layer| layer.ui_instances.iter()).any(|instance| instance.rect == [active_rect.x, active_rect.y, active_rect.w, active_rect.h] && instance.color == selected),
        "globally active selected tab owns the full cap fill"
    );

    let select = input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some("dock.tab..a")).expect("select hit");
    assert_eq!((select.rect.y, select.rect.h), (bounds.y, cap_depth));
    for suffix in ["focus", "close", "drag"] {
        let id = format!("dock.tab..a.{suffix}");
        let action = input.staged_hits().iter().find(|hit| hit.control_id.as_deref() == Some(id.as_str())).unwrap_or_else(|| panic!("missing {id}"));
        assert_eq!((action.rect.y, action.rect.h), (bounds.y + theme.padding_standard, theme.control_height));
    }
}
