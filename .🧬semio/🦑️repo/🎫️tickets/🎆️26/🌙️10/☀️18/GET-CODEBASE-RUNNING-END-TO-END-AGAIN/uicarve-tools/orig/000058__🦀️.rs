// 📟️ Tui vt, scene and layout laws: the vt screen cursor, wrap, scroll region, sgr, alternate screen and resize, the scene node accessors and removal and the column and window layout operations.

//#region 🔖️Vt
fn vt_screen(w: u16, h: u16) -> crate::tui::vt::VtScreen {
    crate::tui::vt::VtScreen::new(Size { width: w, height: h }, 100)
}

fn vt_row(screen: &crate::tui::vt::VtScreen, y: u16) -> String {
    (0..screen.size.width).filter_map(|x| screen.cell_at(x, y)).map(|c| c.ch).filter(|&c| c != '\0').collect()
}

#[test]
fn vt_cursor_motion_cup_and_cuu() {
    let mut s = vt_screen(10, 5);
    s.feed(b"\x1b[3;4H");
    assert_eq!(s.cursor, Pos { x: 3, y: 2 });
    s.feed(b"\x1b[2A");
    assert_eq!(s.cursor, Pos { x: 3, y: 0 });
    s.feed(b"\x1b[1B\x1b[2C\x1b[1D");
    assert_eq!(s.cursor, Pos { x: 4, y: 1 });
}

#[test]
fn vt_wrap_at_edge() {
    let mut s = vt_screen(4, 3);
    s.feed(b"abcdX");
    assert_eq!(&vt_row(&s, 0)[..4], "abcd");
    assert_eq!(s.cell_at(0, 1).map(|c| c.ch), Some('X'));
}

#[test]
fn vt_scroll_region_decstbm_newline_scrolls_inside() {
    let mut s = vt_screen(5, 5);
    s.feed(b"[1;1HAAAAA[2;1HBBBBB[3;1HCCCCC[4;1HDDDDD[5;1HEEEEE");
    s.feed(b"[2;4r");
    assert_eq!((s.scroll_top, s.scroll_bottom), (1, 3));
    s.feed(
        b"[4;1H
",
    );
    assert_eq!(&vt_row(&s, 0)[..5], "AAAAA");
    assert_eq!(&vt_row(&s, 1)[..5], "CCCCC");
    assert_eq!(&vt_row(&s, 2)[..5], "DDDDD");
    assert_eq!(vt_row(&s, 3).trim(), "");
    assert_eq!(&vt_row(&s, 4)[..5], "EEEEE");
}

#[test]
fn vt_sgr_truecolor_sets_cell_fg_bg() {
    let mut s = vt_screen(5, 2);
    s.feed(b"\x1b[38;2;10;20;30;48;2;40;50;60mZ");
    let cell = s.cell_at(0, 0).expect("cell");
    assert_eq!(cell.ch, 'Z');
    assert_eq!(cell.fg, [10, 20, 30]);
    assert_eq!(cell.bg, [40, 50, 60]);
}

#[test]
fn vt_alt_screen_1049_preserves_primary() {
    let mut s = vt_screen(5, 3);
    s.feed(b"HELLO");
    assert_eq!(&vt_row(&s, 0)[..5], "HELLO");
    s.feed(b"\x1b[?1049h");
    assert!(s.alt_active);
    s.feed(b"ALT");
    assert_eq!(&vt_row(&s, 0)[..3], "ALT");
    s.feed(b"\x1b[?1049l");
    assert!(!s.alt_active);
    assert_eq!(&vt_row(&s, 0)[..5], "HELLO");
}

#[test]
fn vt_resize_clamps_cursor() {
    let mut s = vt_screen(10, 10);
    s.feed(b"\x1b[8;8H");
    assert_eq!(s.cursor, Pos { x: 7, y: 7 });
    s.resize(Size { width: 4, height: 3 });
    assert_eq!(s.size, Size { width: 4, height: 3 });
    assert_eq!(s.cursor, Pos { x: 3, y: 2 });
}
//#endregion 🔖️Vt

//#region 🔖️Scene
#[test]
fn scene_node_mut_setters_update_content_and_visibility() {
    let mut scene = Scene::new();
    let root = scene.root();
    let id = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(id).set_text("hi");
    match &scene.node(id).content {
        NodeContent::Text(s) => assert_eq!(s, "hi"),
        _ => panic!("expected text content"),
    }
    scene.node_mut(id).set_visible(false);
    assert!(!scene.node(id).visible);
}

#[test]
fn scene_widget_and_chrome_return_none_for_mismatched_content() {
    let mut scene = Scene::new();
    let root = scene.root();
    let id = scene.add(root, Node::new(NodeContent::Box));
    assert!(scene.node_mut(id).widget().is_none());
    assert!(scene.node_mut(id).chrome().is_none());
}

#[test]
fn scene_hit_finds_deepest_visible_node_topmost_child_wins() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Stack, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    solve(&mut scene, Rect::new(0, 0, 10, 10));
    assert_eq!(scene.hit(Pos { x: 5, y: 5 }), Some(b), "the last-added, topmost child wins on overlap");
    scene.node_mut(b).set_visible(false);
    assert_eq!(scene.hit(Pos { x: 5, y: 5 }), Some(a), "an invisible node is skipped by hit-testing");
    assert_eq!(scene.hit(Pos { x: 20, y: 20 }), None, "outside every rect");
}

#[test]
fn scene_remove_detaches_subtree_from_parent() {
    let mut scene = Scene::new();
    let root = scene.root();
    let a = scene.add(root, Node::new(NodeContent::Box));
    scene.add(a, Node::new(NodeContent::Box));
    scene.remove(a);
    assert!(!scene.node(root).children().contains(&a));
}

#[test]
#[should_panic(expected = "stale NodeId")]
fn scene_node_panics_after_its_id_is_removed() {
    let mut scene = Scene::new();
    let root = scene.root();
    let a = scene.add(root, Node::new(NodeContent::Box));
    scene.remove(a);
    scene.node(a);
}
//#endregion 🔖️Scene

//#region 🔖️Layout
#[test]
fn layout_column_direction_stacks_children_vertically() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Column, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(a).set_constraint(Constraint { height: Dimension::Cells(3), ..Default::default() });
    scene.node_mut(b).set_constraint(Constraint { height: Dimension::Weight(1), ..Default::default() });
    solve(&mut scene, Rect::new(0, 0, 10, 10));
    assert_eq!(scene.rect(a), Rect::new(0, 0, 10, 3));
    assert_eq!(scene.rect(b), Rect::new(0, 3, 10, 7));
}

#[test]
fn layout_distribute_weight_remainder_goes_to_earliest_child_on_ties() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    let c = scene.add(root, Node::new(NodeContent::Box));
    for id in [a, b, c] {
        scene.node_mut(id).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
    }
    solve(&mut scene, Rect::new(0, 0, 10, 5));
    let widths = [scene.rect(a).width, scene.rect(b).width, scene.rect(c).width];
    assert_eq!(widths.iter().sum::<u16>(), 10);
    assert_eq!(widths[0], 4, "equal fractional remainders are broken by original order");
}

#[test]
fn window_layout_split_resize_move_zoom_and_tabs() {
    use crate::tui::layout::{WindowLayout, WindowLayoutRoot, WindowLayoutStackNode, WindowLayoutWindowNode, activate_stack_tab, cycle_stack_tab, move_window_to_stack, resize_window, solve_window_layout, split_window, zoom_window};
    let mut layout =
        WindowLayout { root: WindowLayoutRoot::Stack(WindowLayoutStackNode { size: None, active_window_kind_id: Some("a".into()), children: vec![WindowLayoutWindowNode { window_kind_id: "a".into(), title: None, corner: None }] }), zoomed: None };
    assert!(split_window(&mut layout, "a", "row", "b", None));
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
    assert_eq!(measures.len(), 2);
    assert!(resize_window(&mut layout, "a", 0.5));
    assert!(move_window_to_stack(&mut layout, "b", "a"));
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
    assert_eq!(measures.len(), 1);
    assert_eq!(measures[0].stack_tabs, vec!["a", "b"]);
    assert!(activate_stack_tab(&mut layout, "b"));
    assert!(cycle_stack_tab(&mut layout, "b", 1));
    zoom_window(&mut layout, Some("a"));
    let zoomed = solve_window_layout(&layout, Rect::new(0, 0, 80, 12));
    assert_eq!(zoomed.len(), 1);
    assert_eq!(zoomed[0].window_kind_id, "a");
    assert_eq!(zoomed[0].rect, Rect::new(0, 0, 80, 12));
    zoom_window(&mut layout, None);
}

//#endregion 🔖️Layout
