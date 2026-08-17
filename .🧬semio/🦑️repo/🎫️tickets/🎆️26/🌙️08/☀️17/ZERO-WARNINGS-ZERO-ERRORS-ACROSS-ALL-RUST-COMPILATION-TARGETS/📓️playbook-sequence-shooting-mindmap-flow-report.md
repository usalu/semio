# Playbook / Sequence / Shooting / Reasoning-Mindmap / Flow — `(lib)` Warning Report

All five assigned crates verified at **0 warnings / 0 errors** on the `(lib)` target via
individual `cargo check -p <crate>` runs (final re-verification pass after all edits, each run to
completion, all green). `(lib test)` targets were **not** touched — any pre-existing
`Mutation::apply`/`::diff` trait-mismatch errors there are the other session's in-flight migration,
out of scope per the briefing, and were not investigated further for these five crates (none
happened to surface during `(lib)`-target checks, so no such errors were even observed here).

## 1. `semio-s-plugin-playbook`: 4 → 0 warnings, 0 errors
File: `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- Hidden-lifetime warning on `fn compose(sources: &[ComposeSource])`: added explicit `<'_>` —
  `ComposeSource<'a>` itself carries a lifetime param; confirmed via crate-wide grep that
  `ComposeSource<'_>` is the established convention at every other `compose()` site in the repo
  (trinity, remodel checked as examples).
- Unused import `semio_framework_plugin::ArtifactAnalyzer as _`: deleted — `PlaybookAnalyzer::analyze`
  turned out to be an inherent method, not a trait method requiring the trait in scope.

File: `.../🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `fn json_value_to_serde` dead: crate-wide grep found zero callers (not even recursive uses beyond
  itself). `deserialize()` already goes through stdio's own `JsonSnapshot::to_serde_value()` bridge
  method instead. Deleted the function, its doc comment, and the now-unused `JsonValue`/`FromStr`
  imports; updated the module's top doc comment to stop describing the deleted converter.

File: `.../🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `fn serde_to_json_value` dead: same shape — `serialize()` calls `JsonSnapshot::from_value(value)`
  which takes `impl Into<JsonValue>`, and stdio already provides a real
  `impl From<serde_json::Value> for JsonValue`. Confirmed via grep of stdio's own snapshot component
  (`🗄️stdio/.../🔣️json/.../📸️snapshot/🦀️component.rs:46`). Deleted the function and the now-unused
  `JsonMember`/`JsonValue` imports; updated the doc comment accordingly.

## 2. `semio-s-plugin-sequence`: 2 → 0 warnings, 0 errors
File: `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- Same two warnings as playbook, same fixes: `ComposeSource<'_>` explicit lifetime, deleted unused
  `ArtifactAnalyzer as _` import.

## 3. `semio-s-plugin-shooting`: 5 → 0 warnings, 0 errors
File: `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️component.rs`
- "unused doc comment" at line 518: a large doc comment sat directly above a `thread_local! { ... }`
  macro invocation rather than above the `static` item declared inside the macro body — doc comments
  on the macro-call statement itself aren't attached to anything. Fix: moved the doc comment inside
  the `thread_local!` block, directly above `static SHOOTING_EMBLEM_SCRATCH`, where `thread_local!`
  does accept and attach it per-item (idiomatic, not a suppression).

File: `.../🚪️io/🦀️component.rs` (composer file) — same `ComposeSource<'_>` + unused
`ArtifactAnalyzer as _` import pattern as playbook/sequence.

Files: `.../🚪️io/📥️import/.../🔣️json/.../🦀️component.rs` and
`.../🚪️io/📤️export/.../🔣️json/.../🦀️component.rs` — same dead `json_value_to_serde`/
`serde_to_json_value` pattern as playbook, same fix (deleted, confirmed superseded by stdio's real
`to_serde_value`/`From<serde_json::Value>` bridges), same import/doc-comment cleanup.

## 4. `semio-s-plugin-reasoning-mindmap`: 3 → 0 warnings, 0 errors
File: `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- Same `ComposeSource<'_>` + unused `ArtifactAnalyzer as _` import pattern.

File: `.../✏️editor/📌️panels/🔍️inspection/🦀️component.rs`
- **Real bug, not just a lint**: `pub use infinite_board_normal_undirected as graph;` re-exported a
  *private* `extern crate` alias (`E0365: extern crate ... is private and cannot be re-exported`;
  this crate is also in the future-incompat report over it — will hard-error on a future Rust).
  Root cause: `📦️glue.rs` declared `extern crate infinite_canvas as infinite_board_normal_undirected;`
  (private by default, no `pub`) purely to give the real dependency crate `infinite_canvas` a
  semantically-named local alias; nothing else in the crate used that particular alias as a real
  Rust path (its only other appearances anywhere in the crate are inside doc-comment prose, not
  code). Fix: changed the re-export to go straight to the real crate name —
  `pub use infinite_canvas as graph;` — which is public-dependency-visible by construction (2018+
  edition dependencies don't need `extern crate` to be nameable), then deleted the now-fully-unused
  `extern crate infinite_canvas as infinite_board_normal_undirected;` line in `📦️glue.rs` (confirmed
  zero remaining code references via crate-wide grep; left the sibling
  `extern crate infinite_canvas as infinite_board_port_directed;` alias alone since it's genuinely
  used as a real path in `🧬️schema/🦀️component.rs:320`).

## 5. `semio-s-plugin-flow`: 2 → 0 warnings, 0 errors
File: `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- Same `ComposeSource<'_>` + unused `ArtifactAnalyzer as _` import pattern as the other four.

Note on naming: this is `semio-s-plugin-flow` (the "flow" **artifact plugin**, under
`✏️s/🔌️plugins/🌊️flow/`), a different crate from `semio-framework-os-flow` (the flow **OS module**,
under `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/`), which is not in this ticket's five-crate
scope and was left untouched (it still shows ~11 warnings including a real
`private_interfaces`-shaped one — `FlowExtensionRegistryState`/`FLOW_EXTENSION_STATE` — noted here
only for whoever picks that crate up next, not acted on).

## Left alone / not applicable
No `dead_code` items requiring the full test/wasm-gating triage came up in any of these five —
every dead-code hit was the same already-well-understood "hand-rolled JSON converter superseded by
stdio's real bridge method" shape seen elsewhere in this ticket's other reports. No
`#[allow(...)]` used anywhere. No `(lib test)`-target trait-mismatch errors were encountered or
touched for any of the five crates.

## Files touched (all edits, no deletions of files, no new files)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔣️json/🔖️rfc8259/✳️any/🦀️component.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
- `✏️s/🔌️plugins/💡️reasoning/🗿️artifacts/🔌️wires/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️component.rs`
- `✏️s/🔌️plugins/💡️reasoning/📦️packages/🦀️rust/📦️glue.rs`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`
