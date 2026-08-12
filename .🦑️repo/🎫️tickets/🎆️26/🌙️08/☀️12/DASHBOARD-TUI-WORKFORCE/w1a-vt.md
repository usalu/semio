# W1a — VT Terminal Emulator

## Done
Inserted `// #region 🔖️Vt` (`pub mod vt`) into `🧰️framework/🔨️modules/🖱️ui/⌨️tui/🦀️component.rs` after `Ansi` and before `Event`.

### `VtScreen`
- Primary + alt `CellBuffer`
- Scrollback `VecDeque<Vec<Cell>>` (default cap 10000)
- Cursor, DECSC/DECRC saved cursor, SGR fg/bg/attrs
- DECSTBM scroll region, origin/wrap/cursor-visible/mouse/bracketed-paste modes
- OSC title
- API: `new`, `resize`, `feed`, `blit_to`, `visible_line_count`, `scrollback_len`, `cell_at`

### `VtParser`
Incremental Ground/Escape/Csi/Osc/Dcs/SosPmApc machine covering:
- Printable UTF-8 + wide chars via `text::char_cells`
- CR/LF/BS/TAB/BEL
- CSI cursor/erase/insert/delete/scroll/DECSTBM/SGR (incl. 256 + truecolor)
- DECSET/DECRST `?1049/25/7/1000/1002/1006/2004`
- ESC 7/8, RIS (`ESC c`)
- OSC 0/2 title; DCS ignored to ST

### Tests (existing Tests region)
- `vt_cursor_motion_cup_and_cuu`
- `vt_wrap_at_edge`
- `vt_scroll_region_decstbm_newline_scrolls_inside`
- `vt_sgr_truecolor_sets_cell_fg_bg`
- `vt_alt_screen_1049_preserves_primary`
- `vt_resize_clamps_cursor`

## Verification
```
CARGO_TARGET_DIR=<ticket>/🎯️target-w1a cargo test -p semio-framework-ui --features tui vt_ -- --nocapture
```
Result: **6 passed; 0 failed** (77 other tests filtered out).

Note: `--features tui-terminal` also pulls the parallel W1b Pty region; at verification time that region had an unrelated `openpty` mutability compile error. VT itself is covered by feature `tui` (which `tui-terminal` enables).

Repo MCP was unavailable in this session; work used existing ticket `2026/08/12/DASHBOARD-TUI-WORKFORCE`.
