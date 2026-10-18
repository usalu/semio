import json, os, re, sys, collections
sys.path.insert(0, 'lib')
import carve, rs, reg, crategen
from carve import R, UI, FW, PKG, rd, wr, remove
spec = carve.Spec('specs/tui.json')
T = UI + '/⌨️tui'
DOC = {'core': ('🧱️', 'tui core: geometry, theme, events, text model, cell buffer, rows and the widget signal types.'),
       'terminal': ('📺️', 'tui terminal I/O: ANSI codec, backend capabilities, native raw-mode backend and pty (feature `native`).'),
       'widgets': ('🧩️', 'tui widget states and the element widgets.'),
       'frame': ('🏛️', 'tui scene graph, layout solver, window chrome and the window frame elements.'),
       'laws': ('🥣️', 'tui cross-crate laws: engine behaviour, overlays, pointer routing, chrome anatomy, render equivalence and the unit corpus.')}

# 1. signal extraction -----------------------------------------------------------------------
wp = T + '/🪀️widget/🦀️.rs'
s = rd(wp)
if '/// 📣️ A widget- or window-chrome-level result' in s:
    a = s.index('/// 📣️ A widget- or window-chrome-level result')
    b = s.index('#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub enum Align')
    block = s[a:b]
    s = s[:a] + s[b:]
    old = '/// 🖥️ The embedded terminal pane lives with the VT it drives; the widget layer only hosts it.\npub use crate::tui::vt::pane::{Scrollbar, SelectionMode, TerminalSearch, TerminalSelection, TerminalState};\n'
    assert old in s
    s = s.replace(old, '')
    s = s.replace('use crate::tui::cell::CellBuffer;\n', 'use crate::tui::cell::CellBuffer;\n', 1)
    s = s.replace('use crate::tui::select::{paint_select, select_on_key, select_on_mouse};\n', 'use crate::tui::select::{paint_select, select_on_key, select_on_mouse};\nuse crate::tui::signal::{CursorSpec, WidgetSignal};\n', 1)
    s = s.replace('use crate::tui::tree::{paint_tree, tree_cursor, tree_on_key, tree_on_mouse, tree_on_paste, tree_set_hover};\n', 'use crate::tui::tree::{paint_tree, tree_cursor, tree_on_key, tree_on_mouse, tree_on_paste, tree_set_hover};\nuse crate::tui::vt::pane::TerminalState;\n', 1)
    assert 'signal::{CursorSpec' in s and 'pane::TerminalState' in s
    wr(wp, s)
    sig = '//! 🔔️ Widget signals and the terminal cursor description shared by every widget, the VT pane and the terminal backend.\n\n' + block.rstrip('\n') + '\n'
    wr(T + '/🔔️signal/🦀️.rs', sig)
    print('signal extracted', len(block.splitlines()), 'lines')

# 2. roots ----------------------------------------------------------------------------------
for k, c in spec.crates.items():
    if not c.get('mounts'): continue
    emoji, doc = DOC[k]
    wr(spec.root_file(c), crategen.root_text(spec, c, emoji, doc))
    print('root', k)

# 3. rewrite --------------------------------------------------------------------------------
stats = collections.Counter()
for k, c in spec.crates.items():
    for f in crategen.crate_files(spec, c):
        if not f.endswith('.rs') or not os.path.exists(f): continue
        if f == spec.root_file(c) and c.get('mounts'): continue
        old = rd(f)
        st = {}
        new = carve.rewrite_text(old, spec, k, st)
        if k == 'terminal': new = new.replace('feature = "tui-terminal"', 'feature = "native"')
        if k == 'host': new = new.replace('feature = "tui-bindgen"', 'feature = "bindgen"')
        if k == 'laws': new = new.replace('feature = "tui-terminal"', 'feature = "native"')
        if new != old:
            wr(f, new); stats[k] += 1
        for a, b in st.items(): stats[a] += b
print(dict(stats))

# 4. remove old roots ---------------------------------------------------------------------------
for old in (T + '/🦀️.rs', UI + '/🎯️targets/⌨️tui/🦀️.rs'):
    remove(old)
print('old roots removed')
