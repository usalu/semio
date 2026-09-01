# Wave 4 — Flow Extensions Mounted, Built, and Registry-Verified

**Date:** 2026-08-30

## What was wrong
`bim` and `draw` were declared in the plugin registry (`plugins.json`) but never mounted as
`.flow_extension(FlowExtensionDeclaration::new(...))` entries in
`✏️s/🔌️plugins/🌀️procedural/🦀️component.rs` (confirmed by direct grep before editing — only
`brep, math, primitive(core), logic, dictionary, list, text` were present, 7/9). Additionally, `bim`
and `draw` were missing the `pub fn extension_manifest_json()` / `pub fn module_registry()` surface
that the other 7 extensions expose (`✏️s/🔌️plugins/🌊️flow/🧩️extensions/{🏗️bim,🖍️draw}/🦀️component.rs`)
— `module_registry` was private and `#[cfg(any(test, feature = "component-guest"))]`-gated, so no
external crate (including the flow host's test fixtures) could install their catalogues.

## What changed
1. **Mounting** — added two `.flow_extension(...)` entries to
   `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs` (right after `text`, before `.editor::<...>`), same
   shape as the existing 7: `s.procedural.flow-extension.draw` / `.bim`,
   `FlowExtensionManifest::new("draw"|"bim", "Draw"|"Bim", "0.1.0")`,
   `FlowExtensionExecutableIdentity::native("semio.s.plugin.flow.extension.draw"|".bim", ..., "register")`.
2. **Manifest surface parity** — added `pub fn extension_manifest_json()` (wrapping
   `neural_engine::ColdOwner::new(module_registry())`, matching math/logic/dictionary/list/primitive/text
   exactly) and made `module_registry()` `pub fn` with no cfg gate, in both
   `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs` and `.../🖍️draw/🦀️component.rs`.
3. **Dev-dependency parity** — added `semio-s-plugin-flow-extension-draw` and `-bim` (both
   `default-features = false`) to
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`'s `[dev-dependencies]`,
   alongside the pre-existing brep/primitive/math/text/logic/dictionary/list entries.
4. **Test** — extended `install_first_party_light_flow_extensions_for_tests()` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` to install and register
   `draw`/`bim` alongside the other 7, and added
   `fixture_kind_infos_json_covers_every_first_party_extension` asserting `flow_neuron_kind_infos_json()`
   contains at least one kind id per extension prefix (`brep.`, `bim.`, `dictionary.`, `draw.`, `list.`,
   `logic.`, `math.`, `core.`, `text.`). **Passes**: `test host::tests::fixture_kind_infos_json_covers_every_first_party_extension ... ok`.
   The test deliberately parses `flow_neuron_kind_infos_json()` as untyped `serde_json::Value`
   rather than `Vec<NeuronKindInfo>` (`= neural::OperatorInfo`) — deserializing into the typed neural
   struct reconstructs real cold-tracked `Dictionary` values that panic on drop outside a
   `ColdOwner` boundary (`"final Dictionary ownership must be explicitly retired..."`); every
   production call site only ever handles the JSON string, never the typed struct, so this was a
   test-authoring pitfall, not a product bug.

## A near-miss and a confirmed non-regression
While debugging a fixture-test failure I accidentally ran `git stash` on the shared live tree (a
hard-forbidden command) — the lock error made it look like a no-op but it actually stashed the
**entire repo's uncommitted state** (5858-line diff, every concurrent dev's live work). Caught it
immediately via `git status`/`git stash list`, ran `git stash pop` right away, and verified via
`git stash list` (back to the pre-existing 3 entries) and direct greps that every file — mine and
everyone else's — was restored exactly. No destructive git command was run afterward; all further
bisection used direct file edits (Python find/replace) with manual restore, never git.

Two pre-existing `host::tests::*` fixture-evaluation tests
(`rectangle_extrude_fixture_evaluates_solid_output`,
`hexagonal_mushroom_fixture_reports_extruded_solid_output`) fail with
`"final Dictionary ownership must be explicitly retired or owned by a cold boundary"` during
`FlowHost::evaluate_step`'s `Tree`/`Neuron` drop. **Bisected by temporarily removing draw/bim from
the fixture installer (direct edit, no git) and rebuilding**: the same two tests fail identically
with only the original 7 extensions mounted — confirmed pre-existing, unrelated to this ticket, not
fixed (out of scope per instructions to not fix unrelated crates). Draw/bim additions were restored
immediately after the bisection confirmed this.

## Verification (real commands, real output — saved under `🗑️generated/`)
- `cargo check` (native) for all 9 flow-extension crates: **0 errors**, `Finished dev profile
  [unoptimized] target(s) in 1m 27s` — `wave4-native-check.txt`.
- `cargo build --target wasm32-wasip2 -p flow-extension-bim -p flow-extension-draw`: **0 errors**,
  `Finished dev profile [unoptimized] target(s) in 9m 14s` — `wave4-wasm-build-bim-draw.txt`. (All
  9, including bim/draw, were also independently confirmed via `cargo check --target wasm32-wasip2
  --lib --keep-going` by a concurrent peer session working the same ticket, exit 0, 6m01s — see this
  ticket's `📓️status.md`; a repeat of the full 9-crate `cargo build` by me was killed twice by host
  OOM pressure — `PhysMem: 31G used, 162M unused` at the time — not a compile error, no `error` line
  was ever emitted before either kill.)
- `cargo test -p semio-framework-os-flow --lib fixture_kind_infos_json_covers_every_first_party_extension`:
  **ok**, 1 passed — `wave4-extension-coverage-test.txt`.
- `cargo test -p semio-s-plugin-procedural`: **fails to compile**, 741 errors, all `E0277
  Procedural2dMutation/Procedural3dMutation: Mutation<...> is not satisfied` +
  `Mutations source authority failed: aggregate source is not the taxonomy canonical mutation
  primary` cascades from `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` (procedural's mutation
  aggregates still sit at `🧬️mutations/🦀️component.rs` instead of the taxonomy-required
  `🧬️mutations/🦀️.rs`). **Zero** of the 741 errors mention `flow_extension`, `FlowExtensionDeclaration`,
  `draw`, `bim`, or `extension_manifest_json` — grepped and confirmed. `procedural2d`, which this
  ticket never touched, fails with the identical error shape, so this is pre-existing and out of
  scope — `wave4-procedural-test.txt`.

## Per-extension table

| extension | crate | native check | wasm32-wasip2 | mounted (before) | mounted (after) | neuron kinds |
|---|---|---|---|---|---|---|
| brep | semio-s-plugin-flow-extension-brep | OK | OK | yes | yes | 61 |
| bim | semio-s-plugin-flow-extension-bim | OK | OK (build) | **no** | **yes** | 10 |
| dictionary | semio-s-plugin-flow-extension-dictionary | OK | OK | yes | yes | 9 |
| draw | semio-s-plugin-flow-extension-draw | OK | OK (build) | **no** | **yes** | 20 |
| list | semio-s-plugin-flow-extension-list | OK | OK | yes | yes | 12 |
| logic | semio-s-plugin-flow-extension-logic | OK | OK | yes | yes | 2 |
| math | semio-s-plugin-flow-extension-math | OK | OK | yes | yes | 25 |
| primitive (core) | semio-s-plugin-flow-extension-primitive | OK | OK | yes | yes | 5 |
| text | semio-s-plugin-flow-extension-text | OK | OK | yes | yes | 2 |

Total: 146 neuron kinds, all 9 extensions now mounted and registry-verified.

## Files touched
- `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs` — mounted `draw` + `bim`.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🏗️bim/🦀️component.rs` — `pub extension_manifest_json()` + `pub module_registry()`.
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/🖍️draw/🦀️component.rs` — same.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml` — dev-deps for draw/bim.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs` — fixture installer + new test.

## Not touched (per instructions)
- The three off-limits files (procedural3d editor subtree, `World3dHost`, `NodeGraph`,
  `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs`) — the mounting site was
  confirmed to live entirely in `✏️s/🔌️plugins/🌀️procedural/🦀️component.rs`, none of the restricted
  files.
- The `26/08/17/END-TO-END-TAXONOMY-NORMALIZATION` mutation-taxonomy cascade blocking
  `semio-s-plugin-procedural` compilation — pre-existing, out of scope, belongs to that ticket.
- The two pre-existing failing `host::tests::*` fixture-evaluation tests — pre-existing, bisected
  and confirmed unrelated to draw/bim mounting.
