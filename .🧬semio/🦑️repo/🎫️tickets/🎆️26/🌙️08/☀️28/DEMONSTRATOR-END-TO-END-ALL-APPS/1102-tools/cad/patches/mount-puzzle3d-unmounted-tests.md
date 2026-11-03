Append to `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (a test file, editable in the freeze; owner = puzzle lane), after the last `mod`:

#[path = "../🤖️agent-lane/🦀️.rs"]
mod agent_lane;

#[path = "../🧵️retained-wiring/🦀️.rs"]
mod retained_wiring;

(both use `use super::*;`, `crate::context::editor_source()` and framework `artifact_app_laws::*`; expect stale-API fixes on first compile)
