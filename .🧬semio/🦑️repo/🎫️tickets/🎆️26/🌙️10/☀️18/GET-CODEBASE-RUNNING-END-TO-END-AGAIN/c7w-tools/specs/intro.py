from specs.lib import *
F = R + '/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🦀️.rs'
c = Cut(F, 'intro.rs')
c.child('🧾', 'dump-types-paths-and-walk', 'dump_types_paths_and_walk', [(34, 462)],
        'Dump types, paths and walk: the structure, accessibility, frame, mesh dump types, the node path grammar, the visual fields, the dump walk and the window selection.')
c.child('🏗', 'structure-dump-and-table-accessibility', 'structure_dump_and_table_accessibility', [(464, 738)],
        'Structure dump and table accessibility: the structure dump builder, the layer helpers and the table stepper, editable text and button accessibility entries and nodes.')
c.child('♿', 'vfs-feed-world-and-timeline-accessibility', 'vfs_feed_world_and_timeline_accessibility', [(740, 1117)],
        'Vfs, feed, world and timeline accessibility: the vfs, block list, event feed, world3d and graph timeline accessibility entries and nodes.')
c.child('📤', 'editor-accessibility-frame-stats-and-exports', 'editor_accessibility_frame_stats_and_exports', [(1119, 1394)],
        'Editor accessibility, frame stats and exports: the text editor accessibility entries, the accessibility dump, the frame stats and the wasm dump exports.')
SPEC = c.spec()
