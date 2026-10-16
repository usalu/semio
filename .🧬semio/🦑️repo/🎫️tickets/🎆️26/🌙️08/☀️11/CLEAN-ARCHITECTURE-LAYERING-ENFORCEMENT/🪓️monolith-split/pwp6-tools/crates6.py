import sys; sys.path.insert(0, '.')
# crate (key) assignment of method homes; key = suffix of semio-framework-plugin-<key>
HOME_CRATE = {
 'core': 'instance-app', 'pluginapp': 'instance-app',
 'constructor': 'instance-stores', 'stores': 'instance-stores', 'roster': 'instance-stores', 'ingress': 'instance-stores', 'interaction': 'instance-stores',
 'history': 'instance-history', 'hostevent': 'instance-history', 'txn': 'instance-history', 'free': 'instance-history',
 'children': 'instance-children',
 'pending': 'instance-jobs', 'tools': 'instance-jobs', 'toolmachine': 'instance-jobs',
 'media': 'instance-docs', 'ingest': 'instance-docs',
 'timetravel': 'time-travel-turns', 'toolrun': 'tool-run-turns',
 'maintenance': 'instance-ladders', 'close': 'instance-ladders',
}
FILE_USECASE = {  # source file -> use-case crate
 '🧵️instance/🚪️ingress/🦀️.rs': 'instance-commands', '🧵️instance/📨️dispatch/🦀️.rs': 'instance-commands', '🧵️instance/🧷️command-routes/🦀️.rs': 'instance-commands', '🧵️instance/🤝️group/🦀️.rs': 'instance-commands',
 '🧵️instance/🧾️typed-operations/🦀️.rs': 'instance-typed-ops', '🧵️instance/🛠️tool-machine/🦀️.rs': 'instance-typed-ops',
 '🧵️instance/🖼️render/🦀️.rs': 'instance-render',
 '🪜️close-ladder/🦀️.rs': 'instance-ladders', '🪜️envelope-ladder/🦀️.rs': 'instance-ladders', '🪜️maintenance-ladder/🦀️.rs': 'instance-ladders', '🧵️instance/🔄️full-operation/🦀️.rs': 'instance-ladders', '🪟️window/🫧️transient/🔁️document-replacement/🦀️.rs': 'instance-ladders', '♻️lifecycle/🏪️stores/🦀️.rs': 'instance-ladders',
 '⏪️time-travel/🧵️instance/🦀️.rs': 'time-travel-turns', '⏯️tool-run/🧵️instance/🦀️.rs': 'tool-run-turns',
 '🧵️instance/⏳️operation-progress/🦀️.rs': 'instance-ladders', '🎮️command/🎟️lease/🦀️.rs': 'instance-commands', '🎮️command/📥️ingress/🦀️.rs': 'instance-commands',
 '🧵️instance/🔌️plugin-app-impl/🦀️.rs': 'instance-app', '🧵️instance/🦀️.rs': 'instance-app', '🪆️child/👁️read/🧪️test-view/🦀️.rs': 'instance-children',
}
