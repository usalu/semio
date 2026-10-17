F = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs'
SPEC = dict(
    orig='unit.rs.txt',
    file=F,
    nest=False,
    children=[
        dict(folder='⚖️laws-and-rendering', mod='laws_and_rendering', ranges=[(449, 791)],
             doc='//! ⚖️ Retained static laws, rendering, context menu, pack and operation tests of the puzzle 5d editor. See [`super`].'),
        dict(folder='🗂️manifest-and-window-actions', mod='manifest_and_window_actions', reexport=False, ranges=[(793, 1344)],
             doc='//! 🗂️ Command envelope, clipboard, manifest and window action contract tests of the puzzle 5d editor. See [`super`].'),
        dict(folder='🔌️in-port-and-lane-verbs', mod='in_port_and_lane_verbs', ranges=[(1346, 1732)],
             doc='//! 🔌️ Kit in-port, clipboard maximum and window and config lane verb tests of the puzzle 5d editor. See [`super`].'),
        dict(folder='☑️option-groups-and-edits', mod='option_groups_and_edits', ranges=[(1734, 2240)],
             doc='//! ☑️ Window option group, artifact lane verb and document edit tests of the puzzle 5d editor. See [`super`].'),
        dict(folder='📤️import-export-and-menus', mod='import_export_and_menus', ranges=[(2242, 2796)],
             doc='//! 📤️ Example, export, import, dialog, context menu, paste and part kind tests of the puzzle 5d editor. See [`super`].'),
    ],
    patch={'⚖️laws-and-rendering': [('        include_str!("../../../🧩️entities/🦀️.rs"),\n        include_str!("../../../🧭️view/🦀️.rs"),\n        include_str!("../../../🗂️catalog/🦀️.rs"),\n        include_str!("../../../🕹️interaction/🦀️.rs"),\n        include_str!("../../../🖌️session/🦀️.rs"),\n        include_str!("../../../🧵️retained/🦀️.rs"),\n        include_str!("../../../👆️gestures/🦀️.rs"),\n        include_str!("../../../🧰️kit/🦀️.rs"),\n        include_str!("../../../🦀️.rs"),', '        include_str!("../../../🧩️entities/🦀️.rs"),\n        include_str!("../../../🧭️view/🦀️.rs"),\n        include_str!("../../../🗂️catalog/🦀️.rs"),\n        include_str!("../../../🕹️interaction/🦀️.rs"),\n        include_str!("../../../🖌️session/🦀️.rs"),\n        include_str!("../../../🖌️session/📋️clipboard/🦀️.rs"),\n        include_str!("../../../🖌️session/📌️paste/🦀️.rs"),\n        include_str!("../../../🖌️session/📥️snapshot-import/🦀️.rs"),\n        include_str!("../../../🧵️retained/🦀️.rs"),\n        include_str!("../../../👆️gestures/🦀️.rs"),\n        include_str!("../../../👆️gestures/🎛️selection-and-weights/🦀️.rs"),\n        include_str!("../../../👆️gestures/🩹️patches/🦀️.rs"),\n        include_str!("../../../👆️gestures/🔗️fastener-edits/🦀️.rs"),\n        include_str!("../../../👆️gestures/🧲️proximity-connect/🦀️.rs"),\n        include_str!("../../../🧰️kit/🦀️.rs"),\n        include_str!("../../../🧰️kit/➕️add-node-and-brush-part/🦀️.rs"),\n        include_str!("../../../🧰️kit/🧱️board-events-and-fasteners/🦀️.rs"),\n        include_str!("../../../🧰️kit/📚️example-and-brush-mesh/🦀️.rs"),\n        include_str!("../../../🧰️kit/💾️store-preparation/🦀️.rs"),\n        include_str!("../../../🦀️.rs"),')]},
    expect_extra=['        include_str!("../../🖌️session/📋️clipboard/🦀️.rs"),', '        include_str!("../../🖌️session/📌️paste/🦀️.rs"),', '        include_str!("../../🖌️session/📥️snapshot-import/🦀️.rs"),', '        include_str!("../../👆️gestures/🎛️selection-and-weights/🦀️.rs"),', '        include_str!("../../👆️gestures/🩹️patches/🦀️.rs"),', '        include_str!("../../👆️gestures/🔗️fastener-edits/🦀️.rs"),', '        include_str!("../../👆️gestures/🧲️proximity-connect/🦀️.rs"),', '        include_str!("../../🧰️kit/➕️add-node-and-brush-part/🦀️.rs"),', '        include_str!("../../🧰️kit/🧱️board-events-and-fasteners/🦀️.rs"),', '        include_str!("../../🧰️kit/📚️example-and-brush-mesh/🦀️.rs"),', '        include_str!("../../🧰️kit/💾️store-preparation/🦀️.rs"),'],
    external=[],
)
