# Plugin Preference Ownership Implementation

## Outcome

Plugin-owned locale and terminology config, mutation, command, schema, fixture, and catalog projections were removed. Rendering, menus, measurements, and engagements consume the canonical host `ViewModel`. The ownership verifier reports zero misplaced direct or nested preference declarations and validates 98 aggregate/diff contracts; its last recorded run passed 9 direct vectors and 4 nested vectors.

The generic manifest no longer maps domain action, command, mode, or example identifiers to presentation metadata. Plugin declarations now carry their own explicit icons. The declaration inventory and owner locations are in `presentation-metadata-ownership.md`.

The final Block/Puzzle utility source pass removed plugin config ownership and retained codec routes. User utility selection enters the framework `setActiveUtility` route. Plugin workflows that intentionally change tools publish `Effect::SetActiveUtility`, while render and interaction helpers read the host `ViewModel`.

## Current Block/Puzzle Batch

The exact files changed in the final bounded batch were:

- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🌍️world/🦀️.rs`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/🧵️retained/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🔣️.json`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🧬️schema/🛰️.proto`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🗄️retained-jobs/🔣️.json`
- removed `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧰️set-active/🦀️.rs`

## Validation

- `NX_DAEMON=false NX_ISOLATE_PLUGINS=false bun nx run @semio-tech/puzzle-5d-rs:check --output-style=stream` passed on 2026-09-09, including the current Puzzle3d dependency and Puzzle5d source.
- Static residual search found no Puzzle5d config mutation, typed command, config schema, direct dispatcher, or retained expected-mutation representation of active utility. The only remaining `active_utility_by_window_id` reference in Puzzle5d reads canonical `ViewModel` state.
- The earlier ownership report at `🗑️generated/plugin-preferences/ownership-zero.log` passed 9 direct vectors, 4 nested vectors, 98 aggregate/diff contracts, and reported zero misplaced declarations.
- A broad 17-project check in `🗑️generated/plugin-preferences/final-plugin-checks.log` did not complete cleanly because concurrent workspace changes updated `Cargo.lock` and temporarily omitted a framework window-transient initializer. It is not recorded as a passing validation.
- Full Puzzle unit tests were not rerun after the final Puzzle5d edit; the focused Nx check is green and the coordinator's all-artifact/native test lane remains the required final runtime validation.

## Audit Boundary

This lane is safe to audit. No partially removed Block/Puzzle config variant or direct utility command remains. Ongoing WindowConfig partition work may mechanically add `ConfigView.window` at call sites after this handoff. The independent audit should still inspect low-level WASM utility setters and retained fixture oracle assertions to distinguish host integration adapters from plugin-owned state.
