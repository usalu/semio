// 📐️ Tui geometry, text, cell and ansi laws: the rect operations, the character cells and truncation, the cell buffer writes, the full redraw diff, the run emission and the parser key and mouse decoding.

//#region 🔖️Geometry
#[test]
fn rect_contains_checks_boundaries() {
    let r = Rect::new(2, 2, 3, 3);
    assert!(r.contains(Pos { x: 2, y: 2 }));
    assert!(r.contains(Pos { x: 4, y: 4 }));
    assert!(!r.contains(Pos { x: 5, y: 4 }), "the right edge is exclusive");
    assert!(!r.contains(Pos { x: 1, y: 2 }), "left of the rect is outside");
}

#[test]
fn rect_intersect_returns_overlap_or_default_on_miss() {
    let a = Rect::new(0, 0, 10, 10);
    let b = Rect::new(5, 5, 10, 10);
    assert_eq!(a.intersect(b), Rect::new(5, 5, 5, 5));
    let miss = Rect::new(20, 20, 5, 5);
    assert_eq!(a.intersect(miss), Rect::default());
}

#[test]
fn rect_inset_shrinks_and_clamps_when_margin_exceeds_size() {
    let r = Rect::new(0, 0, 10, 10);
    assert_eq!(r.inset(2), Rect::new(2, 2, 6, 6));
    let small = Rect::new(0, 0, 3, 3);
    let shrunk = small.inset(5);
    assert_eq!(shrunk, Rect { x: 3, y: 3, width: 0, height: 0 }, "an oversized margin clamps size to zero but the origin still shifts by min(margin, size)");
}

#[test]
fn rect_split_top_and_split_bottom_partition_rect() {
    let r = Rect::new(1, 1, 10, 10);
    let (top, rest) = r.split_top(3);
    assert_eq!(top, Rect::new(1, 1, 10, 3));
    assert_eq!(rest, Rect::new(1, 4, 10, 7));
    let (rest2, bottom) = r.split_bottom(4);
    assert_eq!(bottom, Rect::new(1, 7, 10, 4));
    assert_eq!(rest2, Rect::new(1, 1, 10, 6));
    let (top2, rest3) = r.split_top(50);
    assert_eq!(top2.height, 10, "rows beyond the rect's height clamp to the full height");
    assert_eq!(rest3.height, 0);
}
//#endregion 🔖️Geometry

//#region 🔖️Text
#[test]
fn text_char_cells_zero_and_wide() {
    assert_eq!(crate::tui::text::char_cells('a'), 1);
    assert_eq!(crate::tui::text::char_cells('世'), 2);
    assert_eq!(crate::tui::text::char_cells('\u{200b}'), 0, "a zero-width space occupies no cells");
}

#[test]
fn text_cell_width_unicode_goldens() {
    assert_eq!(display_width("Semio"), 5, "ASCII is one cell per scalar");
    assert_eq!(display_width("e\u{0301}"), 1, "combining accents add no cell");
    assert_eq!(display_width("a\u{0000}\u{001b}b"), 2, "C0 controls add no cell");
    assert_eq!(display_width("界面"), 4, "CJK wide scalars occupy two cells");
    assert_eq!(display_width("🙂"), 2, "emoji presentation scalars occupy two cells");
    assert_eq!(crate::tui::text::char_cells('\u{200d}'), 0, "the zero-width joiner adds no cell");
    assert_eq!(display_width("👩\u{200d}💻"), 2, "a ZWJ sequence is one two-cell glyph");
}

#[test]
fn truncate_to_stops_before_splitting_a_wide_char() {
    let (s, w) = truncate_to("a世b", 2);
    assert_eq!(s, "a", "the wide char doesn't fit in the remaining 1 cell, so it's dropped whole");
    assert_eq!(w, 1);
    let (s2, w2) = truncate_to("世", 2);
    assert_eq!(s2, "世");
    assert_eq!(w2, 2);
}
//#endregion 🔖️Text

//#region 🔖️Cell
#[test]
fn cell_buffer_get_returns_none_out_of_bounds() {
    let buf = CellBuffer::new(Size { width: 3, height: 3 }, Cell::blank([0, 0, 0], [0, 0, 0]));
    assert!(buf.get(3, 0).is_none());
    assert!(buf.get(0, 3).is_none());
    assert!(buf.get(2, 2).is_some());
}

#[test]
fn cell_buffer_put_pairs_and_orphans_wide_char_continuations() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 5, height: 1 }, blank);
    buf.put(1, 0, Cell { ch: '?', width: 2, ..blank });
    assert_eq!(buf.get(1, 0).unwrap().width, 2);
    assert_eq!(buf.get(2, 0).unwrap().width, 0, "the wide char auto-populates its continuation cell");

    buf.put(2, 0, Cell { ch: '\0', width: 0, ..blank });
    assert_eq!(buf.get(2, 0).unwrap().width, 0, "a width-0 write next to a wide lead stays paired");

    buf.put(0, 0, Cell { ch: '\0', width: 0, ..blank });
    assert_eq!(buf.get(0, 0).unwrap().width, 1, "column 0 has no lead to the left, so an orphaned continuation becomes a blank");
}

#[test]
fn cell_buffer_put_clamps_wide_char_at_right_edge() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 3, height: 1 }, blank);
    buf.put(2, 0, Cell { ch: '?', width: 2, ..blank });
    assert_eq!(buf.get(2, 0).unwrap().width, 1, "a wide char at the last column clamps to width 1 instead of overflowing");
}

#[test]
fn cell_buffer_put_str_counts_clipped_cells_but_only_draws_inside_clip() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 10, height: 1 }, blank);
    let clip = Rect::new(2, 0, 4, 1);
    let written = buf.put_str(Pos { x: 0, y: 0 }, "abcdefgh", [1, 1, 1], [2, 2, 2], 0, clip);
    assert_eq!(written, 6, "cells advance past the clip's left edge even though nothing is drawn there");
    assert_eq!(buf.get(0, 0).unwrap().ch, ' ', "cells left of the clip stay untouched");
    assert_eq!(buf.get(2, 0).unwrap().ch, 'c');
    assert_eq!(buf.get(5, 0).unwrap().ch, 'f');
    assert_eq!(buf.get(6, 0).unwrap().ch, ' ', "cells right of the clip stay untouched");

    let out_of_row = buf.put_str(Pos { x: 2, y: 1 }, "zzz", [0, 0, 0], [0, 0, 0], 0, clip);
    assert_eq!(out_of_row, 0, "a row outside the clip's y-range writes nothing");
}

#[test]
fn cell_buffer_fill_rect_clips_to_buffer_bounds() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 4, height: 4 }, blank);
    buf.fill_rect(Rect::new(2, 2, 10, 10), Cell { ch: '#', ..blank });
    assert_eq!(buf.get(2, 2).unwrap().ch, '#');
    assert_eq!(buf.get(3, 3).unwrap().ch, '#');
}

#[test]
fn cell_buffer_hline_and_vline_paint_expected_cells() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 5, height: 5 }, blank);
    buf.hline(Pos { x: 1, y: 1 }, 3, '-', [1, 1, 1], [2, 2, 2]);
    assert_eq!(buf.get(1, 1).unwrap().ch, '-');
    assert_eq!(buf.get(3, 1).unwrap().ch, '-');
    assert_eq!(buf.get(0, 1).unwrap().ch, ' ');
    assert_eq!(buf.get(4, 1).unwrap().ch, ' ');

    buf.vline(Pos { x: 2, y: 0 }, 3, '|', [1, 1, 1], [2, 2, 2]);
    assert_eq!(buf.get(2, 0).unwrap().ch, '|');
    assert_eq!(buf.get(2, 2).unwrap().ch, '|');
    assert_eq!(buf.get(2, 3).unwrap().ch, ' ');
}

#[test]
fn diff_full_redraw_when_sizes_differ() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let a = CellBuffer::new(Size { width: 5, height: 5 }, blank);
    let b = CellBuffer::new(Size { width: 6, height: 5 }, blank);
    let runs = diff(&a, &b);
    assert_eq!(runs, (0..5).map(|y| DiffRun { y, x: 0, len: 6 }).collect::<Vec<_>>(), "a resized buffer repaints every row in full");
}
//#endregion 🔖️Cell

//#region 🔖️Ansi
#[test]
fn emit_runs_writes_cursor_move_and_truecolor_sgr() {
    let blank = Cell::blank([0, 0, 0], [0, 0, 0]);
    let mut buf = CellBuffer::new(Size { width: 5, height: 1 }, blank);
    buf.put(2, 0, Cell { ch: 'Q', fg: [10, 20, 30], bg: [40, 50, 60], attrs: attr::BOLD, width: 1 });
    let runs = vec![DiffRun { y: 0, x: 2, len: 1 }];
    let mut patch = AnsiPatch::default();
    emit_runs(&buf, &runs, &mut patch);
    assert!(patch.0.contains("\x1b[1;3H"), "cursor moves to the 1-indexed row/col");
    assert!(patch.0.contains(";38;2;10;20;30"), "foreground truecolor sgr");
    assert!(patch.0.contains(";48;2;40;50;60"), "background truecolor sgr");
    assert!(patch.0.contains(";1"), "bold attribute sgr");
    assert!(patch.0.ends_with('Q'));
}

#[test]
fn ansi_setup_and_teardown_sequences_contain_expected_escapes() {
    assert!(setup_sequence().contains("\x1b[?1049h"), "enters the alternate screen");
    assert!(setup_sequence().contains("\x1b[?25l"), "hides the cursor");
    assert!(teardown_sequence().contains("\x1b[?1049l"), "restores the primary screen");
    assert!(teardown_sequence().contains("\x1b[?25h"), "shows the cursor");
}

#[test]
fn parser_ctrl_char_maps_to_char_with_ctrl_mod() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&[0x01, 0x03], &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('a'), mods: crate::tui::event::mods::CTRL }), Event::Key(KeyEvent { key: Key::Char('c'), mods: crate::tui::event::mods::CTRL })]);
}

#[test]
fn parser_alt_prefixed_char_sets_alt_mod() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1bj", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Char('j'), mods: crate::tui::event::mods::ALT })]);
}

#[test]
fn parser_backtab_and_focus_gained_lost() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[Z\x1b[I\x1b[O", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::BackTab, mods: 0 }), Event::FocusGained, Event::FocusLost]);
}

#[test]
fn parser_tilde_navigation_and_function_keys() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[3~\x1b[15~", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::Delete, mods: 0 }), Event::Key(KeyEvent { key: Key::F(5), mods: 0 })]);
}

#[test]
fn parser_ss3_function_keys() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1bOP\x1bOQ", &mut events);
    assert_eq!(events, vec![Event::Key(KeyEvent { key: Key::F(1), mods: 0 }), Event::Key(KeyEvent { key: Key::F(2), mods: 0 })]);
}

#[test]
fn parser_mouse_scroll_drag_and_release_kinds() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(b"\x1b[<64;5;5M\x1b[<97;5;5M\x1b[<32;5;5M\x1b[<0;5;5m", &mut events);
    let kinds: Vec<MouseKind> = events
        .iter()
        .map(|e| match e {
            Event::Mouse(m) => m.kind,
            _ => panic!("expected mouse event"),
        })
        .collect();
    assert_eq!(kinds, vec![MouseKind::Scroll { dx: 0, dy: -1 }, MouseKind::Scroll { dx: 0, dy: 1 }, MouseKind::Drag(MouseButton::Left), MouseKind::Up(MouseButton::Left)]);
}

#[test]
fn parser_control_keys_enter_tab_backspace() {
    let mut parser = AnsiParser::new();
    let mut events = Vec::new();
    parser.feed(&[0x0d, 0x0a, 0x09, 0x7f, 0x08], &mut events);
    assert_eq!(
        events,
        vec![
            Event::Key(KeyEvent { key: Key::Enter, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Char('j'), mods: crate::tui::event::mods::CTRL }),
            Event::Key(KeyEvent { key: Key::Tab, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Backspace, mods: 0 }),
            Event::Key(KeyEvent { key: Key::Backspace, mods: 0 }),
        ]
    );
}
//#endregion 🔖️Ansi
