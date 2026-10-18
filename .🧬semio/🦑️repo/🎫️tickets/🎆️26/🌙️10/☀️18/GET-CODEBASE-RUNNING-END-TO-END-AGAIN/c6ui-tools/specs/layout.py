from specs.lib import *
F = U + '/📜️declaration/📐️layout/🦀️.rs'
INC1 = 'include!("../../🧪️tests/🔬️targets-wgpu-component-layout-layout-wire-format/🦀️.rs");'
INC2 = 'include!("../../🧪️tests/🔬️targets-wgpu-component-layout-value-round-trip/🦀️.rs");'
LC = [
    ch('🎬', 'action-and-presence', 'action_and_presence', [(12, 200)],
       'Action and presence: the action descriptor, the style spec, the ui state and status, the peer mark and the `UiPresence` with their serde defaults.'),
    ch('🗂', 'context-menu-organizer', 'context_menu_organizer', [(204, 611)],
       'Context menu organizer: the menu reference, the context menu item spec, the ribbon parent categories and the group, separator and budget organizing with the shell menu actions.'),
    ch('📋', 'context-menu-requests-and-panel-tabs', 'context_menu_requests_and_panel_tabs', [(614, 752)],
       'Context menu requests and panel tabs: the context menu hit, selection group, text context, surface target, point, request and response and the framework panel tab ids, labels and icons.'),
    ch('🪟', 'window-layout-tree', 'window_layout_tree', [(756, 1048)],
       'Window layout tree: the window and stack kinds, the stack corner, the window, stack and axis nodes, the layout child and root unions, the named layouts and the layout constructors and window kind collection.'),
    ch('📏', 'window-measure', 'window_measure', [(1052, 1222)],
       'Window measure: the measure select item, the `WindowMeasure` union, its accessors and the measure partition.'),
    ch('🤝', 'window-engagement', 'window_engagement', [(1226, 1418)],
       'Window engagement: the engagement options, inputs, status, possible values, controls and slots, the viewport engagement default and the window options.'),
]
SPEC = dict(
    orig='layout.rs', file=F, children=[], expect_missing=['pub mod layout {', '}'],
    external=mounted(F, orig='layout.rs'),
    patch={'🧭️layout-contract': [(INC1, INC1.replace('"../../', '"../../../')), (INC2, INC2.replace('"../../', '"../../../'))]},
    inner=dict(mod='layout', indent=4, folder='🧭️layout-contract', nest=False, rebase_super=True,
               doc='//! 🧭️ Layout contract module: the action and presence, context menu, window layout, measure and engagement declarations with the shared imports of the former inline module. See [`super`].',
               children=LC),
)
