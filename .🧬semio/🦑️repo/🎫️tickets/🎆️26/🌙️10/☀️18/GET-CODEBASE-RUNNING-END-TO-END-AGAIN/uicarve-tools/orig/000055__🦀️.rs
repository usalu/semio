/// 🪟️ Tui layout and window laws: the row text helper, the weighted, auto, padded layouts, the window layouts and chrome silhouettes, the window controls and hits, the wizard and the remount.

fn row_text(buf: &CellBuffer, y: u16) -> String {
    (0..buf.size.width).filter_map(|x| buf.get(x, y)).map(|c| c.ch).filter(|&c| c != '\0').collect()
}

#[test]
fn layout_row_weights_fill_exactly() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(a).set_constraint(Constraint { width: Dimension::Weight(1), ..Default::default() });
    scene.node_mut(b).set_constraint(Constraint { width: Dimension::Weight(2), ..Default::default() });
    solve(&mut scene, Rect::new(0, 0, 30, 10));
    assert_eq!(scene.rect(a).width + scene.rect(b).width, 30);
    assert!(scene.rect(b).width > scene.rect(a).width);
}

#[test]
fn layout_auto_measures_text_width() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, ..Default::default() });
    let text = scene.add(root, Node::new(NodeContent::Text("hi".into())));
    solve(&mut scene, Rect::new(0, 0, 30, 10));
    assert_eq!(scene.rect(text).width, 2);
}

#[test]
fn layout_padding_and_gap() {
    let mut scene = Scene::new();
    let root = scene.root();
    scene.node_mut(root).set_constraint(Constraint { direction: Direction::Row, gap: 1, padding: [1, 1, 1, 1], ..Default::default() });
    let a = scene.add(root, Node::new(NodeContent::Box));
    let b = scene.add(root, Node::new(NodeContent::Box));
    scene.node_mut(a).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
    scene.node_mut(b).set_constraint(Constraint { width: Dimension::Cells(2), ..Default::default() });
    solve(&mut scene, Rect::new(0, 0, 20, 10));
    assert_eq!(scene.rect(a).x, 1);
    assert_eq!(scene.rect(b).x, 4);
    assert_eq!(scene.rect(root).height, 10);
    assert_eq!(scene.rect(a).height, 8);
}

#[test]
fn window_layout_row_of_stacks_tiles_with_one_gutter_cell() {
    let layout = create_default_layout(&["a".to_string(), "b".to_string()], "row", Some(&[1.0, 1.0]), None);
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 100, 10));
    assert_eq!(measures.len(), 2);
    assert_eq!(measures[0].rect.width + 1 + measures[1].rect.width, 100);
    assert_eq!(measures[0].rect.x, 0);
    assert_eq!(measures[1].rect.x, measures[0].rect.width + 1, "the gutter is the splitter between the two stacks");
}

#[test]
fn window_layout_stack_exposes_tabs() {
    let layout = WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            size: None,
            active_window_kind_id: Some("b".into()),
            children: vec![WindowLayoutWindowNode { window_kind_id: "a".into(), title: None, corner: None }, WindowLayoutWindowNode { window_kind_id: "b".into(), title: None, corner: None }],
        }),
        zoomed: None,
    };
    let measures = solve_window_layout(&layout, Rect::new(0, 0, 40, 10));
    assert_eq!(measures.len(), 1);
    assert_eq!(measures[0].window_kind_id, "b");
    assert_eq!(measures[0].stack_tabs, vec!["a", "b"]);
}

/// 🧱 A single top-left corner tab with its close glyph; the focused stack uses the heavy line set and the right edge stays flat at the body hairline.
#[test]
fn window_chrome_recesses_tabs_into_the_top_corners_of_a_closed_shape() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 5);
    let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let mut w = WindowState::new("Puzzle 3D");
    w.focused = true;
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);

    let row0 = row_text(&buf, 0);
    let row1 = row_text(&buf, 1);
    let row2 = row_text(&buf, 2);
    assert!(row0.starts_with('\u{250f}'), "tab top-left corner: {row0:?}");
    assert!(row1.contains("Puzzle 3D"), "title in tab: {row1:?}");
    assert!(!row1.contains('\u{2922}'), "a lone window has nothing to maximize against: {row1:?}");
    assert!(!row1.contains('\u{29C9}'), "the per-tab new-window glyph is gone: {row1:?}");
    assert!(row1.contains('\u{2715}'), "close glyph: {row1:?}");
    assert!(row2.contains('\u{2517}'), "tab bends into body: {row2:?}");
    assert_eq!(row2.chars().last(), Some('\u{2513}'), "flat top-right at body: {row2:?}");
    assert_eq!(row_text(&buf, 4).chars().next(), Some('\u{2517}'));
    assert_eq!(row_text(&buf, 4).chars().last(), Some('\u{251b}'));

    let title_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == 'P').expect("title text rendered");
    let title = buf.get(title_x, 1).unwrap();
    assert_eq!(title.fg, theme.role(Role::ActiveForeground));
    assert_eq!(title.bg, theme.role(Role::ActiveBase), "the active tab is a solid fill");
}

#[test]
fn window_chrome_flattens_the_right_side_when_no_controls_tab_is_wanted() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 6);
    let mut buf = CellBuffer::new(Size { width: 40, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let mut w = WindowState::new("Log");
    w.closable = false;
    w.maximizable = false;
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);

    let row0 = row_text(&buf, 0);
    let row1 = row_text(&buf, 1);
    let row2 = row_text(&buf, 2);
    assert!(row0.starts_with('\u{250c}'), "{row0:?}");
    assert!(row1.contains("Log"), "{row1:?}");
    assert!(!row1.contains('\u{2922}') && !row1.contains('\u{2715}'), "no action glyphs: {row1:?}");
    assert!(row2.contains('\u{2514}'), "bend into body: {row2:?}");
    assert_eq!(row2.chars().last(), Some('\u{2510}'), "flat right corner: {row2:?}");
    assert!(!row_text(&buf, 0).contains('\u{2922}') && !row_text(&buf, 1).contains('\u{2922}'), "no controls tab was requested");
}

/// 📦 A plain flat box replaces the raised corner chrome when it cannot fit.
#[test]
fn window_chrome_hides_both_tabs_when_too_narrow_for_even_the_title() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 4, 5);
    let mut buf = CellBuffer::new(Size { width: 4, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("X");
    assert_eq!(window_chip_layout(&w, rect).mode, crate::tui::chrome::ChromeMode::Flat);
    let window = ChromeState::Window(Box::new(w));
    window.paint(&theme, rect, &mut buf);
    assert_eq!(row_text(&buf, 0), "┌──┐");
    assert_eq!(window.window_control_at(rect, Pos { x: 2, y: 0 }), None);
    assert_eq!(window.window_control_at(rect, Pos { x: 2, y: 1 }), None);
}

#[test]
fn window_control_clicks_resolve_to_close_and_maximize_signals() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 5);
    let mut buf = CellBuffer::new(Size { width: 40, height: 5 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let mut state = WindowState::new("Plugins");
    state.peers = 2;
    let window = ChromeState::Window(Box::new(state));
    window.paint(&theme, rect, &mut buf);
    let maximize_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2922}').expect("maximize glyph rendered");
    let close_x = (0..40).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
    assert_eq!(window.window_control_at(rect, Pos { x: maximize_x, y: 1 }), Some(WidgetSignal::WindowMaximize(0)));
    assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 1 }), Some(WidgetSignal::WindowClose(0)));
    assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 0 }), None, "clicks on the tab's own top edge must not trigger a control");
    assert_eq!(window.window_control_at(rect, Pos { x: close_x, y: 2 }), None, "clicks below the tab row must not trigger a control");
}

#[test]
fn window_hit_resolves_tab_activation_and_new_tab_signals() {
    let rect = Rect::new(0, 0, 60, 8);
    let mut w = WindowState::new("Main").with_stack_tabs(vec!["tab1".into(), "tab2".into()], 1);
    w.new_tab = true;
    let layout = window_chip_layout(&w, rect);
    let window = ChromeState::Window(Box::new(w));
    let group = layout.groups.iter().find(|g| g.corner == crate::tui::layout::WindowStackCorner::TopLeft).expect("top-left group");
    let tab0 = group.tabs.first().expect("first tab geometry");
    let tab1 = group.tabs.get(1).expect("second tab geometry");
    assert_eq!(window.window_hit(rect, Pos { x: tab0.x + 2, y: rect.y + 1 }), Some(WidgetSignal::WindowTabActivated(0)));
    assert_eq!(window.window_hit(rect, Pos { x: tab1.x + 2, y: rect.y + 1 }), Some(WidgetSignal::WindowTabActivated(1)));
    let close0 = tab0.close_x.expect("first tab close geometry");
    let close1 = tab1.close_x.expect("second tab close geometry");
    assert_eq!(window.window_hit(rect, Pos { x: close0, y: rect.y + 1 }), Some(WidgetSignal::WindowClose(0)), "an inactive chip closes itself");
    assert_eq!(window.window_hit(rect, Pos { x: close1, y: rect.y + 1 }), Some(WidgetSignal::WindowClose(1)));
    let controls = layout.groups.iter().flat_map(|g| g.tabs.iter()).find(|tab| tab.kind == crate::tui::chrome::TabKind::Controls).expect("controls chip");
    let new_x = controls.new_x.expect("new-tab glyph geometry");
    assert_eq!(window.window_hit(rect, Pos { x: new_x, y: rect.y + 1 }), Some(WidgetSignal::WindowNewTab(1)));
}

#[test]
fn shell_window_wizard_body_paints_options_after_remount() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 60, height: 16 }, Theme::new(AppearanceName::Dark));
    let navbar = NavbarState { left: vec![], center: vec![], right: vec![] };
    let footer = FooterState { hints: vec![], status: String::new() };
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["wizard".into()]));
    let mut built = shell(&mut tui.scene, navbar, footer, &layout);
    let (_id, chrome) = built.windows[0].clone();
    let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(WizardState::new(vec!["dev".into(), "build".into(), "test".into()])))));
    tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    built.remount(&mut tui.scene, &layout);
    tui.set_focus(Some(widget));
    let _ = tui.render_full();
    let rect = tui.scene.rect(widget);
    assert!(rect.width > 4 && rect.height > 1, "wizard rect collapsed: {rect:?}");
    let mut buf = CellBuffer::new(Size { width: 60, height: 16 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    match &tui.scene.node(widget).content {
        NodeContent::Widget(w) => w.paint(&Theme::new(AppearanceName::Dark), rect, &mut buf, true),
        _ => panic!("expected wizard widget"),
    }
    let body: String = (0..4).map(|row| buf.row_text(rect.y + row)).collect::<Vec<_>>().join("\n");
    assert!(body.contains("dev") && body.contains("build"), "wizard options missing from paint: {body:?}");
}

#[test]
fn mount_window_layout_reparents_windows_and_preserves_widget_state() {
    let mut scene = Scene::new();
    let root = scene.root();
    let canvas = scene.add(root, Node::new(NodeContent::Chrome(ChromeState::Canvas)));
    let w1_chrome = scene.add(canvas, Node::new(NodeContent::Chrome(ChromeState::Window(Box::new(WindowState::new("w1"))))));
    let term = scene.add(w1_chrome, Node::new(NodeContent::Widget(WidgetState::Terminal(TerminalState::new(Size { width: 10, height: 5 }, 100)))));
    if let Some(WidgetState::Terminal(t)) = scene.node_mut(term).widget() {
        t.feed(b"hello");
    }
    let layout = even_window_layout(&["w1".into()]);
    let mut mount_root = None;
    mount_window_layout(&mut scene, canvas, &layout, &[("w1".into(), w1_chrome)], &mut mount_root);
    assert!(mount_root.is_some());
    assert_ne!(scene.node(w1_chrome).parent, Some(canvas), "window chrome must leave the canvas direct-child list");
    if let Some(WidgetState::Terminal(t)) = scene.node_mut(term).widget() {
        let has_h = (0..t.screen.size.height).any(|y| (0..t.screen.size.width).any(|x| t.screen.cell_at(x, y).map(|c| c.ch) == Some('h')));
        assert!(has_h);
    }
}

#[test]
fn wizard_keys_filter_navigate_back_and_activate() {
    let mut widget = WidgetState::Wizard(WizardState::new(vec!["build".into(), "dev".into(), "test".into()]));
    let down = KeyEvent { key: Key::Down, mods: 0 };
    assert_eq!(widget.on_key(&down), Some(WidgetSignal::SelectionChanged(1)));
    let e = KeyEvent { key: Key::Char('e'), mods: 0 };
    let v = KeyEvent { key: Key::Char('v'), mods: 0 };
    assert_eq!(widget.on_key(&e), Some(WidgetSignal::ValueChanged("e".to_string())));
    assert_eq!(widget.on_key(&v), Some(WidgetSignal::ValueChanged("ev".to_string())));
    let WidgetState::Wizard(w) = &widget else { unreachable!() };
    assert_eq!(w.filter(), "ev");
    let enter = KeyEvent { key: Key::Enter, mods: 0 };
    assert_eq!(widget.on_key(&enter), Some(WidgetSignal::Activated(1)));
    let mut empty = WidgetState::Wizard(WizardState::new(vec!["only".into()]));
    let backspace = KeyEvent { key: Key::Backspace, mods: 0 };
    assert_eq!(empty.on_key(&backspace), None, "Backspace on an empty filter never leaves the wizard, so a held key cannot walk out");
    let alt_backspace = KeyEvent { key: Key::Backspace, mods: crate::tui::event::mods::ALT };
    assert_eq!(empty.on_key(&alt_backspace), Some(WidgetSignal::NavigateBack));
}

#[test]
fn tui_dispatch_emits_window_close_signal_on_click() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 40, height: 40 }, Theme::new(AppearanceName::Dark));
    let navbar = NavbarState { left: vec![], center: vec![], right: vec![] };
    let footer = FooterState { hints: vec![], status: String::new() };
    let layout = even_window_layout(&["plugins".to_string()]);
    let built = shell(&mut tui.scene, navbar, footer, &layout);
    tui.render_full();
    let (_, window_id) = built.windows[0].clone();
    let rect = tui.scene.rect(window_id);
    let mut buf = CellBuffer::new(Size { width: rect.width, height: rect.height }, Cell::blank([0, 0, 0], [0, 0, 0]));
    ChromeState::Window(Box::new(WindowState::new("plugins"))).paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, rect.width, rect.height), &mut buf);
    let close_x = (0..rect.width).find(|&x| buf.get(x, 1).unwrap().ch == '\u{2715}').expect("close glyph rendered");
    let signals = tui.dispatch(&Event::Mouse(MouseEvent { kind: MouseKind::Down(MouseButton::Left), pos: Pos { x: rect.x + close_x, y: rect.y + 1 }, mods: 0, clicks: 1 }));
    assert_eq!(signals, vec![(window_id, WidgetSignal::WindowClose(0))]);
}
