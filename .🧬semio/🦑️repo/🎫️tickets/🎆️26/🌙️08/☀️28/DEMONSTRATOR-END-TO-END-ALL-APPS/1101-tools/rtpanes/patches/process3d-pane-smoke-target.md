Apply after the freeze lifts. File: `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📦️packages/🦀️rust/Cargo.toml`

Append next to `process3d_editor_unit`:

[[test]]
name = "process3d_pane_smoke"
path = "../../🧪️tests/🪧️pane-smoke-root/🦀️.rs"

Then change `seq.sh` bearbeiten to `--test process3d_pane_smoke -- bearbeiten_pane`.
Reason: `process3d_editor_unit` has 108 stale errors (command-surface, interaction-behavior, contributions-and-examples use pub(super) items of `🧵️retained-commands` and kit constants through the editor glob) that need production-visibility decisions; the new root only mounts the support harness (2 stale errors fixed) and the pane-smoke module.
