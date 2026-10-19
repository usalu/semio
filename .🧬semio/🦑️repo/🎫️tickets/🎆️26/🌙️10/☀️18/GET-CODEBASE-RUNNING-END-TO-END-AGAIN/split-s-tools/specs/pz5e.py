from specs.lib import *
F = '✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs'
c = Cut(F, 'pz5e.rs')
c.child('🍓', 'paste-job', 'paste_job', [(451, 650)], 'Paste job: the reserved clipboard paste tool job of the 5d play app.')
c.child('🥦', 'import-job', 'import_job', [(667, 1187)], 'Import job: the reserved media import tool job of the 5d play app.')
c.child('🌽', 'command-variants', 'command_variants', [(1204, 1368)], 'Command variants: the sole dispatch surface of the 5d play app, one variant per declared action.')
c.child('🍠', 'play-app', 'play_app', [(1393, 1588)], 'Play app: the stateless 5d play app and its action dispatch.')
c.child('🥐', 'retained-command-works', 'retained_command_works', [(1591, 2026)], 'Retained command works: the retained tool ids and the retained works of proximity connect, add node, add brush part and create fastener.')
SPEC = c.spec(keep_empty=('Labels', 'ActionContext', 'ContextMenu', 'CopyPaste', 'Distribution', 'Engine', 'Scene', 'SelectionFlags', 'UnitTests', 'InferredKinds'))
