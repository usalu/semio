import json, os
U = '🧰️framework/🔨️modules/🖱️ui'
T = U + '/⌨️tui'
E = U + '/🧱️elements'
def el(name, mod):
    return {'name': mod, 'file': '%s/%s/🎯️targets/⌨️tui/🦀️.rs' % (E, name)}
def tm(dirname, mod=None):
    return {'name': mod or dirname.split('/')[-1][2:] if False else mod, 'file': '%s/%s/🦀️.rs' % (T, dirname)}
crates = {
 'core': {'pkg': 'semio-framework-ui-tui-core', 'id': 'ui-tui-core', 'dir': T + '/🧱️core', 'desc': 'semio tui core: geometry, theme, events, text model, cell buffer, rows and the widget signal types',
   'mounts': [{'name': 'geometry', 'file': T + '/📐️geometry/🦀️.rs'}, {'name': 'theme', 'file': T + '/🎨️theme/🦀️.rs'}, {'name': 'event', 'file': T + '/📡️event/🦀️.rs'},
              {'name': 'text', 'file': T + '/📝️text/🦀️.rs'}, {'name': 'cell', 'file': T + '/🔲️cell/🦀️.rs'}, {'name': 'rows', 'file': T + '/📜️rows/🦀️.rs'}, {'name': 'signal', 'file': T + '/🔔️signal/🦀️.rs'}]},
 'vt': {'pkg': 'semio-framework-ui-tui-vt', 'id': 'ui-tui-vt', 'dir': T + '/📟️vt', 'root': T + '/📟️vt/🦀️.rs', 'desc': 'semio tui virtual terminal: parser, screen, pane, history, key encoder'},
 'terminal': {'pkg': 'semio-framework-ui-tui-terminal', 'id': 'ui-tui-terminal', 'dir': T + '/📺️terminal', 'desc': 'semio tui terminal I/O: ANSI codec, backend capabilities, native raw-mode backend and pty (feature native)',
   'mounts': [{'name': 'ansi', 'file': T + '/🔡️ansi/🦀️.rs'}, {'name': 'backend', 'file': T + '/🔌️backend/🦀️.rs'},
              {'name': 'pty', 'file': T + '/🚇️pty/🦀️.rs', 'cfg': '#[cfg(feature = "native")]'},
              {'name': 'windows_abi', 'file': T + '/🪟️windows/🦀️.rs', 'cfg': '#[cfg(all(feature = "native", windows))]', 'vis': ''}]},
 'widgets': {'pkg': 'semio-framework-ui-tui-widgets', 'id': 'ui-tui-widgets', 'dir': T + '/🧩️widgets', 'desc': 'semio tui widget states and the element widgets',
   'mounts': [{'name': 'widget', 'file': T + '/🪀️widget/🦀️.rs'},
     el('🪙️Chip', 'chip'), el('➖️Divider', 'divider'), el('✏️Input', 'input'), el('🏷️Label', 'label'), el('📃️List', 'list'), el('🪵️Log', 'log'), el('🔽️Select', 'select'), el('📊️Table', 'table'), el('📑️Tabs', 'tabs'),
     el('💬️Dialog', 'dialog'), el('🖱️ContextMenu', 'menu'), el('⌨️Command', 'palette'), el('💡️ChromeControlHint', 'tooltip'), el('🌳️Tree', 'tree'), el('📜️Scrollable', 'scrollable'), el('📶️Progress', 'progress'), el('🔀️Toggle', 'toggle'), el('🧙️Wizard', 'wizard')]},
 'frame': {'pkg': 'semio-framework-ui-tui-frame', 'id': 'ui-tui-frame', 'dir': T + '/🏛️frame', 'desc': 'semio tui scene graph, layout solver, window chrome and the window frame elements',
   'mounts': [{'name': 'chrome', 'file': T + '/🖥️chrome/🦀️.rs'}, {'name': 'layout', 'file': T + '/📏️layout/🦀️.rs'}, {'name': 'scene', 'file': T + '/🎬️scene/🦀️.rs'},
              el('🔚️Footer', 'footer'), el('🔝️Navbar', 'navbar'), el('🪟️Window', 'window')]},
 'engine': {'pkg': 'semio-framework-ui-tui-engine', 'id': 'ui-tui-engine', 'dir': T + '/⚙️engine', 'root': T + '/⚙️engine/🦀️.rs', 'desc': 'semio tui retained-mode engine'},
 'laws': {'pkg': 'semio-framework-ui-tui-laws', 'id': 'ui-tui-laws', 'dir': T + '/🧪️tests/🥣️companion', 'desc': 'semio tui cross-crate laws: engine behaviour, overlays, pointer routing, chrome anatomy, render equivalence and the legacy unit corpus', 'tests_only': True,
   'mounts': [{'name': n, 'file': T + '/🧪️tests/' + d + '/🦀️.rs', 'cfg': '#[cfg(test)]', 'vis': ''} for n, d in (('text_elements', '🔬️text-elements'), ('render_equivalence', '🔬️render-equivalence'), ('engine_behaviour', '🔬️engine'), ('chrome_anatomy', '🔬️chrome'), ('overlays', '🔬️overlays'), ('pointer_routing', '🔬️pointer-routing'))]},
 'unit-laws': {'pkg': 'semio-framework-ui-tui-unit-laws', 'id': 'ui-tui-unit-laws', 'dir': T + '/🧪️tests/🔬️unit', 'root': T + '/🧪️tests/🔬️unit/🦀️.rs', 'desc': 'semio tui unit corpus: layout, window, table, parser, theme, geometry, text, cell, ANSI, vt and widget laws', 'tests_only': True},
 'vt-laws': {'pkg': 'semio-framework-ui-tui-vt-laws', 'id': 'ui-tui-vt-laws', 'dir': T + '/📟️vt/🧪️tests/🔬️unit', 'root': T + '/📟️vt/🧪️tests/🔬️unit/🦀️.rs', 'desc': 'semio tui virtual terminal laws: screen sequences, history reflow, pane input and pointer, encoders and the shared corpus', 'tests_only': True},
 'host': {'pkg': 'semio-framework-ui-tui-host', 'id': 'ui-tui-host', 'dir': T + '/🏃️host', 'root': T + '/🏃️host/🦀️.rs', 'desc': 'semio tui wasm host for browser terminals (feature bindgen)'},
}
def r(frm, crate, to): return {'from': frm, 'crate': crate, 'to': to}
rules = []
for m in ('geometry', 'theme', 'event', 'text', 'cell', 'rows'): rules.append(r('tui::' + m, 'core', m))
for it in ('WidgetSignal', 'CursorShape', 'CursorSpec'): rules.append(r('tui::widget::' + it, 'core', 'signal::' + it))
rules.append(r('tui::vt', 'vt', ''))
for it in ('Scrollbar', 'SelectionMode', 'TerminalSearch', 'TerminalSelection', 'TerminalState'): rules.append(r('tui::widget::' + it, 'vt', 'pane::' + it))
for m in ('ansi', 'backend', 'pty'): rules.append(r('tui::' + m, 'terminal', m))
rules.append(r('tui::component::windows_abi', 'terminal', 'windows_abi'))
for m in ('widget', 'chip', 'divider', 'input', 'label', 'list', 'log', 'select', 'table', 'tabs', 'dialog', 'menu', 'palette', 'tooltip', 'tree', 'scrollable', 'progress', 'toggle', 'wizard'): rules.append(r('tui::' + m, 'widgets', m))
for m in ('chrome', 'layout', 'scene', 'footer', 'navbar', 'window'): rules.append(r('tui::' + m, 'frame', m))
rules.append(r('tui::signal', 'core', 'signal'))
rules.append(r('tui::engine', 'engine', ''))
rules.append(r('tui::host', 'host', ''))
spec = {'tree': 'tui', 'roots': ['crate'], 'crates': crates, 'rules': rules}
json.dump(spec, open(os.path.join(os.path.dirname(os.path.abspath(__file__)), 'tui.json'), 'w'), indent=1, ensure_ascii=False)
print('ok', len(crates), len(rules))
