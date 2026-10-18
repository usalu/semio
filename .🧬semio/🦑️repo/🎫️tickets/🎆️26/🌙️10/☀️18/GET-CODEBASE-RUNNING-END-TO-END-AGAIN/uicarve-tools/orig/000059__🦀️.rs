// 🎨️ Tui widget paint and window silhouette laws: the list, select, tabs, log, input, divider, chip, navbar and footer painting, the terminal widget passthrough and the window stack tab silhouettes and hairlines.

//#region 🔖️Widget
#[test]
fn paint_list_highlights_selected_row_only_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut list = ListState::new(vec!["a".to_string(), "b".to_string()]);
    list.selected = 1;
    list.marks = vec![false, true];
    let rect = Rect::new(0, 0, 10, 2);
    let mut buf = CellBuffer::new(Size { width: 10, height: 2 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::List(list).paint(&theme, rect, &mut buf, true);
    assert_eq!(buf.get(0, 1).unwrap().bg, theme.role(Role::ActiveBase));
    assert_ne!(buf.get(0, 0).unwrap().bg, theme.role(Role::ActiveBase));
    assert!(row_text(&buf, 1).starts_with('\u{2713}'), "a marked row shows a check prefix");
    assert!(row_text(&buf, 0).starts_with(' '), "an unmarked row has a blank prefix");
}

#[test]
fn paint_select_renders_label_and_current_option() {
    let theme = Theme::new(AppearanceName::Dark);
    let select = SelectState { label: "Mode".to_string(), options: vec!["A".to_string(), "B".to_string()], index: 1 };
    let rect = Rect::new(0, 0, 20, 1);
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Select(select).paint(&theme, rect, &mut buf, true);
    assert_eq!(row_text(&buf, 0).trim_end(), "Mode: \u{2039} B \u{203a}");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::Accent), "a focused select uses the accent color");
}

#[test]
fn paint_tabs_bolds_the_active_tab() {
    let theme = Theme::new(AppearanceName::Dark);
    let tabs = TabsState::new(vec!["One".to_string(), "Two".to_string()], 1);
    let rect = Rect::new(0, 0, 20, 1);
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Tabs(tabs).paint(&theme, rect, &mut buf, false);
    let t_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'T').expect("active tab rendered");
    assert_eq!(buf.get(t_x, 0).unwrap().attrs & attr::BOLD, attr::BOLD);
    let o_x = (0..20).find(|&x| buf.get(x, 0).unwrap().ch == 'O').expect("inactive tab rendered");
    assert_eq!(buf.get(o_x, 0).unwrap().attrs & attr::BOLD, 0, "inactive tab is not bold");
}

#[test]
fn terminal_widget_passes_keys_through_and_opens_search_only_by_command() {
    let mut term = TerminalState::new(Size { width: 8, height: 3 }, 50);
    term.feed(b"line1\nline2\nline3\nline4\n");
    let mut widget = WidgetState::Terminal(term);
    let page = KeyEvent { key: Key::PageUp, mods: 0 };
    assert_eq!(widget.on_key(&page), Some(WidgetSignal::TerminalInput(b"\x1b[5~".to_vec())));
    let slash = KeyEvent { key: Key::Char('/'), mods: 0 };
    assert_eq!(widget.on_key(&slash), Some(WidgetSignal::TerminalInput(b"/".to_vec())));
    if let WidgetState::Terminal(t) = &mut widget {
        t.begin_search();
    }
    let a = KeyEvent { key: Key::Char('a'), mods: 0 };
    assert_eq!(widget.on_key(&a), Some(WidgetSignal::ValueChanged("a".into())));
    let esc = KeyEvent { key: Key::Esc, mods: 0 };
    assert_eq!(widget.on_key(&esc), Some(WidgetSignal::ValueChanged(String::new())));
    let x = KeyEvent { key: Key::Char('x'), mods: 0 };
    assert_eq!(widget.on_key(&x), Some(WidgetSignal::TerminalInput(b"x".to_vec())));
    let mut buf = CellBuffer::new(Size { width: 8, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    widget.paint(&Theme::new(AppearanceName::Dark), Rect::new(0, 0, 8, 3), &mut buf, true);
}

#[test]
fn window_stack_tabs_paint_on_body_top() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 6);
    let mut buf = CellBuffer::new(Size { width: 50, height: 6 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("Main").with_stack_tabs(vec!["a".into(), "b".into()], 1);
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
    let row: String = (0..50).filter_map(|x| buf.get(x, 1).map(|c| c.ch)).collect();
    assert!(row.contains('a') && row.contains('b'), "corner stack tabs missing: {row:?}");
    assert!(row.contains('\u{2715}') && !row.contains('\u{2922}'), "close glyph present, no maximize for a lone window: {row:?}");
}

#[test]
fn window_stack_tabs_respect_bottom_corners() {
    use crate::tui::chrome::WindowStackTabState;
    use crate::tui::layout::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 8);
    let mut buf = CellBuffer::new(Size { width: 50, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    let w = WindowState::new("Main").with_stack_tab_states(vec![WindowStackTabState::new("top", WindowStackCorner::TopLeft), WindowStackTabState::new("bot", WindowStackCorner::BottomRight)], 1);
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
    let top = row_text(&buf, 1);
    let bottom = row_text(&buf, 6);
    assert!(top.contains("top"), "top tab missing: {top:?}");
    assert!(bottom.contains("bot"), "bottom tab missing: {bottom:?}");
}

#[test]
fn window_silhouette_keeps_one_continuous_outline_around_the_notch() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 40, 8);
    let sentinel = Cell { ch: '.', fg: [1, 2, 3], bg: [4, 5, 6], attrs: 0, width: 1 };
    let mut buf = CellBuffer::new(Size { width: 40, height: 8 }, sentinel);
    ChromeState::Window(Box::new(WindowState::new("Commands"))).paint(&theme, rect, &mut buf);
    assert_eq!(buf.get(38, 0).copied(), Some(sentinel), "the notch beside the title chip stays outside the window");
    assert_eq!(buf.get(2, 4).unwrap().bg, theme.surface(Surface::Window));
    let hair = row_text(&buf, 2);
    let bend = hair.rfind('\u{2514}').expect("chip bends into the body");
    let tail = &hair[bend..];
    assert!(tail.chars().all(|ch| ch != ' ' && ch != '.'), "outline breaks across the body cap: {hair:?}");
    assert!(tail.ends_with('\u{2510}'), "body cap does not close on the right: {hair:?}");
    assert_eq!(window_content_padding(&WindowState::new("Commands"), rect), [3, 1, 1, 1]);
}

#[test]
fn window_bottom_notch_is_outside_the_continuous_outline() {
    use crate::tui::chrome::WindowStackTabState;
    use crate::tui::layout::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 50, 8);
    let sentinel = Cell { ch: '.', fg: [1, 2, 3], bg: [4, 5, 6], attrs: 0, width: 1 };
    let mut buf = CellBuffer::new(Size { width: 50, height: 8 }, sentinel);
    let window = WindowState::new("Main").with_stack_tab_states(vec![WindowStackTabState::new("top", WindowStackCorner::TopLeft), WindowStackTabState::new("bot", WindowStackCorner::BottomRight)], 0);
    ChromeState::Window(Box::new(window)).paint(&theme, rect, &mut buf);
    assert_eq!(buf.get(0, 7).copied(), Some(sentinel), "no dangling corner in the bottom-left notch");
    assert_eq!(buf.get(0, 5).unwrap().ch, '\u{2514}');
}

#[test]
fn composed_window_keeps_the_silhouette_when_the_body_paints() {
    let mut tui = crate::tui::engine::Tui::new(Size { width: 60, height: 40 }, Theme::new(AppearanceName::Dark));
    let navbar = NavbarState { left: vec![], center: vec![NavItem { id: "d".into(), label: "dashboard".into(), active: false }], right: vec![] };
    let footer = FooterState { hints: vec![], status: String::new() };
    let layout = create_default_layout(&["w1".into()], "row", None, Some(&["Commands".into()]));
    let mut built = shell(&mut tui.scene, navbar, footer, &layout);
    let (_id, chrome) = built.windows[0].clone();
    let widget = tui.scene.add(chrome, Node::new(NodeContent::Widget(WidgetState::Wizard(WizardState::new(vec!["alpha".into()])))));
    tui.scene.node_mut(widget).set_constraint(Constraint { width: Dimension::Weight(1), height: Dimension::Weight(1), ..Default::default() });
    built.remount(&mut tui.scene, &layout);
    tui.set_focus(Some(widget));
    let _ = tui.render_full();
    let win = tui.scene.rect(chrome);
    let frame = tui.frame();
    let hair = row_text(frame, win.y + 2);
    let bend = hair.rfind('\u{2517}').unwrap_or_else(|| panic!("missing bend {hair:?}"));
    assert!(hair[bend..].chars().all(|ch| ch != ' '), "body overwrote the silhouette: {hair:?}");
    let body = row_text(frame, win.y + 3);
    assert!(body.contains('/'), "wizard should start under the outline, got {body:?}");
    assert_eq!(frame.get(win.x + win.width - 2, win.y).unwrap().bg, tui.theme.surface(Surface::Base));
}

#[test]
fn window_hairline_opens_only_under_the_active_tab() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 60, 8);
    for active in [0usize, 1] {
        let mut w = WindowState::new("Main").with_stack_tabs(vec!["alpha".into(), "beta".into()], active);
        w.focused = true;
        let layout = window_chip_layout(&w, rect);
        let mut buf = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
        ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
        let group = layout.groups.iter().find(|g| g.corner == crate::tui::layout::WindowStackCorner::TopLeft).expect("top-left tabs");
        for tab in &group.tabs {
            let open = tab.index == active;
            for x in (tab.x + 1)..(tab.x + tab.interior_width + 1) {
                let ch = buf.get(x, layout.top_body_y).unwrap().ch;
                if open {
                    assert_eq!(ch, ' ', "active tab {active} must stay open at {x}: {}", row_text(&buf, layout.top_body_y));
                } else {
                    assert_eq!(ch, '\u{2501}', "inactive tab must stay closed at {x} (heavy line set while focused): {}", row_text(&buf, layout.top_body_y));
                }
            }
        }
        let hair = row_text(&buf, layout.top_body_y);
        let bend = hair.rfind('\u{2517}').unwrap_or_else(|| panic!("missing active bend {hair:?}"));
        assert!(hair[bend..].chars().all(|ch| ch != ' '), "silhouette breaks after the active tab: {hair:?}");
        assert!(hair.ends_with('\u{2513}'), "body cap does not close: {hair:?}");
        assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::ActiveBase), "focused silhouette uses the active color");
        assert_eq!(buf.get(rect.width - 1, layout.top_body_y).unwrap().fg, theme.role(Role::ActiveBase));
    }
    let plain = WindowState::new("Main").with_stack_tabs(vec!["alpha".into(), "beta".into()], 0);
    let mut buf = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    ChromeState::Window(Box::new(plain)).paint(&theme, rect, &mut buf);
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::BorderNormal), "an unfocused silhouette uses the normal border");
}

#[test]
fn window_bottom_hairline_opens_only_under_the_active_tab() {
    use crate::tui::chrome::WindowStackTabState;
    use crate::tui::layout::WindowStackCorner;
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 60, 8);
    let mut w = WindowState::new("Main").with_stack_tab_states(
        vec![WindowStackTabState::new("one", WindowStackCorner::BottomLeft), WindowStackTabState::new("two", WindowStackCorner::BottomLeft)],
        1,
    );
    w.focused = true;
    let layout = window_chip_layout(&w, rect);
    let mut buf = CellBuffer::new(Size { width: 60, height: 8 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    ChromeState::Window(Box::new(w)).paint(&theme, rect, &mut buf);
    let y = layout.bottom_body_y.expect("bottom hairline");
    let group = layout.groups.iter().find(|g| g.corner == WindowStackCorner::BottomLeft).expect("bottom tabs");
    assert!(group.tabs.len() >= 2, "two bottom tabs");
    for tab in &group.tabs {
        let open = tab.index == 1;
        for x in (tab.x + 1)..(tab.x + tab.interior_width + 1) {
            let ch = buf.get(x, y).unwrap().ch;
            if open {
                assert_eq!(ch, ' ', "active bottom tab must stay open at {x}: {}", row_text(&buf, y));
            } else {
                assert_eq!(ch, '\u{2501}', "inactive bottom tab must stay closed at {x} (heavy line set while focused): {}", row_text(&buf, y));
            }
        }
    }
    assert_eq!(buf.get(0, rect.height - 1).unwrap().fg, theme.role(Role::ActiveBase));
}

#[test]
fn paint_log_shows_the_tail_when_following() {
    let theme = Theme::new(AppearanceName::Dark);
    let mut log = LogState::new(10);
    for i in 0..5 {
        log.push(&format!("line{i}"));
    }
    let rect = Rect::new(0, 0, 10, 3);
    let mut buf = CellBuffer::new(Size { width: 10, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Log(log).paint(&theme, rect, &mut buf, false);
    assert!(row_text(&buf, 0).starts_with("line2"), "{:?}", row_text(&buf, 0));
    assert!(row_text(&buf, 1).starts_with("line3"));
    assert!(row_text(&buf, 2).starts_with("line4"));
    assert_eq!(buf.get(9, 2).unwrap().ch, '\u{2503}', "five lines in three rows draw a scroll bar whose thumb sits at the bottom while following");
}

#[test]
fn paint_input_shows_placeholder_and_draws_cursor_when_focused() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 20, 1);
    let empty_input = InputState { value: String::new(), cursor: 0, placeholder: "type here".to_string() };
    let mut buf = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Input(empty_input).paint(&theme, rect, &mut buf, false);
    assert_eq!(row_text(&buf, 0).trim_end(), "type here");
    assert_eq!(buf.get(0, 0).unwrap().fg, theme.role(Role::MutedForeground));

    let filled = InputState { value: "hi".to_string(), cursor: 2, placeholder: "ph".to_string() };
    let mut buf2 = CellBuffer::new(Size { width: 20, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Input(filled).paint(&theme, rect, &mut buf2, true);
    assert_eq!(buf2.get(2, 0).unwrap().ch, ' ', "the painter draws no caret glyph; the terminal cursor marks the caret");
    assert_eq!(buf2.get(0, 0).unwrap().ch, 'h', "the caret never hides a glyph");
    let spec = WidgetState::Input(InputState { value: "hi".to_string(), cursor: 2, placeholder: "ph".to_string() }).cursor(rect).expect("a focused input places the terminal cursor");
    assert_eq!(spec.pos, Pos { x: 2, y: 0 });
}

#[test]
fn paint_divider_centers_its_label() {
    let theme = Theme::new(AppearanceName::Dark);
    let divider = DividerState { label: Some("Hi".to_string()) };
    let rect = Rect::new(0, 0, 10, 1);
    let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Divider(divider).paint(&theme, rect, &mut buf, false);
    let row = row_text(&buf, 0);
    assert!(row.contains("Hi"));
    assert_eq!(row.chars().next().unwrap(), '\u{2500}', "the hairline still draws under the label");
}

#[test]
fn paint_chip_reflects_on_and_off_colors() {
    let theme = Theme::new(AppearanceName::Dark);
    let rect = Rect::new(0, 0, 10, 1);
    let on = ChipState { label: "x".to_string(), on: true };
    let mut buf_on = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Chip(on).paint(&theme, rect, &mut buf_on, false);
    assert_eq!(buf_on.get(0, 0).unwrap().bg, theme.role(Role::Accent));

    let off = ChipState { label: "x".to_string(), on: false };
    let mut buf_off = CellBuffer::new(Size { width: 10, height: 1 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    WidgetState::Chip(off).paint(&theme, rect, &mut buf_off, false);
    assert_eq!(buf_off.get(0, 0).unwrap().bg, theme.surface(Surface::Panel));
}
//#endregion 🔖️Widget
