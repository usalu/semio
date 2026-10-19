from specs.lib import *
F = R + '/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/📥️staging/🖐️pointer/⌨️keyboard-and-palette/🦀️.rs'
c = Cut(F, 'kbd.rs').impl("impl<'ctx> PointerCtx<'ctx> {", [344], at=8)
c.child('🔁', 'deferred-menus', 'deferred_and_menus', [(7, 343)],
        'Deferred actions and menus: the deferred action flush, the ui chain settling, the settle pump steps, the owed refresh and the context menu.')
c.child('⌨', 'keyboard', 'keyboard_handling', [(344, 814)],
        'Keyboard handling: the synchronous and asynchronous keyboard handlers and the keyboard ring advance.')
c.child('🔎', 'pump-activation', 'pump_cross_and_activation', [(816, 1192)],
        'Pump cross and activation: the editing check, the cross settle pump, the extension invocation, the widget drag values, the focused input commit, the tree drag finish, the panel tab, the search and find activation, the app keybindings and the shortcut dispatch.')
SPEC = c.spec()
