# Snapshot oracle and projection audit

semio_framework_pack_json::to_json_string(&snapshot) accepts the Snapshot-derived ToValue and converts the complete DslValue through the JSON bridge (framework/🔨️modules/🎒️pack/🔤️json/🎛️controlled-and-borrowed-bridge/🦀️.rs:5–6). serde_json::from_str::<serde_json::Value>(&text) independently parses that text. Remodel uses this same full-value bridge in its text snapshot owner, and the sqlite snapshot test compares full restored and original ToValue results. This establishes an oracle recipe, not an independent encoder.

BTreeMap implements DslField map projection (framework/🔨️modules/🗣️dsl/🧬️schema/🪆️binding/🦀️.rs:327–329), so the map itself is not the default UnsupportedOwner. The implementation uses iterator nth for key and value, so traversal work grows with index and is not visibly metered. Nested owners can still refuse: nonempty realistic fixtures and actual execution are necessary. No runtime test was performed.
