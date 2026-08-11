# W4a6 — Flow Extensions: Delete Closed `Contribution` Path

Scope: the 9 flow extension crates under `✏️s/🔌️plugins/🌊️flow/🧩️extensions/` (`bim`, `brep`, `dictionary`,
`list`, `primitive`, `math`, `text`, `draw`, `logic`) and the registry consumer
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`.

## Producer sites (`mod extension_guest` / `bundle()`) — all 9 crates

Each crate's `flow_extension_contribution(app_id, manifest_json)` helper (inside
`#[cfg(feature = "component-guest")] mod extension_guest`, unconditional in `bim`) previously returned
`(Contribution, serde_json::Value)`. Changed to return just `serde_json::Value` (the topic payload) —
deleted the trailing `Contribution::FlowExtension { .. }` tuple element entirely, kept the payload
construction (which already drew from the same local `extension_id`/`label`/`icon_id`/`manifest_json`
values, so no logic duplication introduced).

`bundle()` in each crate: replaced the two destructured `let (flow_contribution, flow_topic_payload) =
flow_extension_contribution(...)` calls with plain `let flow_topic_payload = ...` (same for the
`procedural3d` variant), and removed the two `.contributes(flow_contribution)` /
`.contributes(procedural3d_contribution)` calls from the builder chain. `.contributes_topic("flow.extension",
...)` is now the sole contribution declaration per app_id, unchanged from the prior wave.

Removed `Contribution` from each `use semio_framework::{Contribution, Fault, FaultCode, FaultOrigin};` ->
`use semio_framework::{Fault, FaultCode, FaultOrigin};`.

Applied via a small Python script (`/private/tmp/.../scratchpad/fix_extension_guest.py`, not checked in —
scratch only) asserting exactly one match per pattern per file, so all 9 crates got the identical
transform with no silent skips.

## Test-fixture cleanup — `bim`, `brep`, `draw` only

Re-grepped both the flow plugin subtree and the registry file for any remaining `Contribution::`/
`contribution:` construction (including test fixtures), per the wave instructions. Found that only 3 of
the 9 crates (`bim`, `brep`, `draw`) had a `#[cfg(test)]` test directly constructing
`Contribution::FlowExtension { .. }` via `.contributes(...)` — the other 6 (`list`, `dictionary`, `text`,
`primitive`, `logic`, `math`) have no such test, only the producer-site helper (already handled above).

None of these 3 tests were "closed-shape-only" tests exercising the removed fallback (i.e. no test was
purely testing that `Contribution::FlowExtension` is present) — each test's real purpose is
"bundle extends flow and evaluates via the handler", so instead of deleting them, rewrote them to build
the bundle with `.contributes_topic("flow.extension", serde_json::json!({ .. }))` (mirroring the real
`bundle()` shape) in place of `.contributes(Contribution::FlowExtension { .. })`, and removed the
now-unused `use semio_framework::Contribution;` import from each test fn.

- `bim::extension_bundle_extends_flow_and_evaluates`: also asserted `installed.contributions.len() == 2`
  and `matches!(installed.contributions[0], Contribution::FlowExtension { .. })` — updated to
  `installed.topic_contributions.len() == 2` and `installed.topic_contributions[0].topic == "flow.extension"`.
- `brep::extension_bundle_extends_flow_and_evaluates_box`: didn't assert on `contributions` at all (only
  `extension_manifest().extension_id`), so just swapped the two `.contributes(...)` calls for
  `.contributes_topic(...)`.
- `draw::bundle_contributes_draw_for_flow_and_procedural3d_play`: asserted
  `installed.contributions.len() == 2` and both `matches!(installed.contributions[N], Contribution::FlowExtension { .. })`
  — updated to `installed.topic_contributions.len() == 2` and
  `installed.topic_contributions[N].topic == "flow.extension"` for both indices.

Post-change grep of all 9 extension files + the registry file for `Contribution` (any form) returns
nothing except benign doc-comment mentions of `TopicContribution` in the registry file (renamed the one
docstring that still referenced `Contribution::FlowExtension` by name to stop naming the deleted type).

## Registry consumer — `sync_host_flow_extension_contributions`

Removed the closed-enum fallback branch entirely:

```rust
let closed_manifest_json = match entry.contribution {
    semio_framework::Contribution::FlowExtension { manifest_json, .. } => Some(manifest_json),
    _ => None,
};
if let Some(manifest_json) = topic_manifest_json.or(closed_manifest_json) { ... }
```

is now:

```rust
if let Some(manifest_json) = topic_manifest_json { ... }
```

The open `TopicContribution` read (`entry.topic_contribution`, filtered by `topic == "flow.extension"`,
decoded via `FlowExtensionTopicPayload`) is now the only path — an entry with no matching open
contribution is skipped, same as a malformed one would be today. This was necessary, not just cleanup:
`semio_framework::ProgramContributionEntry` (defined in `🔨️modules/🛂️manifest/🦀️component.rs`) has
**already** had its `contribution: Contribution` field deleted by the parallel framework-type-deletion
agent — the pre-edit code (`entry.contribution`) would not have compiled at all.

## Verification

```
cargo check -p semio-s-plugin-flow-extension-bim -p semio-s-plugin-flow-extension-brep \
  -p semio-s-plugin-flow-extension-dictionary -p semio-s-plugin-flow-extension-list \
  -p semio-s-plugin-flow-extension-primitive -p semio-s-plugin-flow-extension-math \
  -p semio-s-plugin-flow-extension-text -p semio-s-plugin-flow-extension-draw \
  -p semio-s-plugin-flow-extension-logic -p semio-framework-os-flow
```

- `semio-framework-os-flow` (the registry crate): **Finished clean**, zero errors — confirms the
  consumer-side edit is correct in isolation.
- All 9 `semio-s-plugin-flow-extension-*` crates: blocked upstream — `could not compile
  semio-framework-plugin (lib) due to 2 previous errors`, both inside
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (NOT one of my assigned
  files):
  - `error[E0432]: unresolved import semio_framework::Contribution` (line 4,
    `use semio_framework::{kernel::CapabilityRequirement, CommandDefinition, Contribution};`)
  - `error[E0599]: no method named contributes found for struct component::app::Plugin` (line 166,
    `plugin = plugin.contributes(contribution);`)

  This is the same parallel framework-type-deletion agent's in-progress work: `Contribution` has already
  been removed from the `semio_framework` crate root (confirmed independently above via
  `ProgramContributionEntry`), but `🏗️builder/🦀️component.rs` — which defines `ExtensionBundle::contributes()`
  itself, a different file than anything in my assignment — hasn't been updated yet to drop its own
  `Contribution`-typed `contributes()` method. None of my 9 extension crates call `.contributes(...)`
  anymore (only `.contributes_topic(...)`, which doesn't touch `Contribution` and isn't implicated in
  either error), so this block is entirely upstream/unrelated to this wave's edits — noted per the
  "concurrent churn, don't chase" rule rather than fixed. Manually verified brace/paren balance across
  all 9 edited files as a syntax sanity check in lieu of a full `cargo check` pass blocked by the above.

## Files touched

- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📖️dictionary/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📃️list/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🔤️primitive/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧮️math/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📝️text/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🧠️logic/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`

Not touched / out of scope, flagged for awareness: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`
(still defines `ExtensionBundle::contributes(Contribution)` and imports the now-deleted `Contribution`
type — belongs to whichever wave is deleting `Contribution` from the framework/plugin-builder layer).

Also deliberately not touched: `grep -rl Contribution` across the full `✏️s/🔌️plugins/🌊️flow/` subtree also
turns up `🎛️apps/🌊️flow/🦀️component.rs`, `🎛️apps/🌊️flow/🎚️config/🦀️component.rs`, and
`🎛️apps/🌊️flow/🎮️commands/🧩️extension/🦀️component.rs` — these live under `🎛️apps/🌊️flow/`, outside the
`🧩️extensions/` subtree this wave was scoped to, so left alone per the "never touch a file outside your
assigned directories even if it also references Contribution" rule; another agent owns them.
