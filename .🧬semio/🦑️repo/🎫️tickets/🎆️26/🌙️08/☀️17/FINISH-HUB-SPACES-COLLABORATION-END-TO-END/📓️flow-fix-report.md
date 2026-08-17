# `semio-framework-os-flow` wasm compile fix — report

## Task

Make `semio-framework-os-flow` compile for `wasm32-unknown-unknown` and produce its wasm-pack
output, per the 18-error defect described in the dispatch. Lease: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/**`
and, only for unavoidable one-line adaptations, framework crates it consumes.

## Root cause

Commit `1d71198c19f13e1ecd4000621c08d00d36eac4a1` (2026-08-17 14:44:08 +0200, "Clean OS host and DSL
wiring by removing linkage shims and narrowing feature gates") ran an unused-import cleanup driven by
**native, non-test `cargo check`** — the default compilation profile. That profile never compiles
`#[cfg(target_arch = "wasm32")]` code (native target) and never compiles `#[cfg(test)]` code (`cargo
check`, not `cargo test`), so the cleanup over-pruned: it stripped `use` lines that were genuinely
dead on that one profile but load-bearing for the two profiles it didn't check. A second, already-staged
pass (present before I started, part of the coordinator's own getrandom-Cargo.toml fix batch, timestamped
~15:57–15:58) trimmed a few more of the same kind, plus began (but didn't finish) a visibility widening
in `📔️registry/🦀️component.rs`.

Every one of the 18 reported wasm errors, plus everything found while completing the requested
verification sweep, traces to this single mechanism.

## Fixes applied (all inside `🌊️flow/**`, all restoring/completing what the over-pruning removed)

1. **`🌉️wasm/🦀️component.rs`** — the entire file is `#[cfg(target_arch = "wasm32")]`-gated, so its
   imports were reduced to just the `dag` alias. Restored, each gated identically to the existing `dag`
   import:
   ```rust
   #[cfg(target_arch = "wasm32")]
   use crate::infinite::canvas;
   #[cfg(target_arch = "wasm32")]
   use crate::host::*;
   #[cfg(target_arch = "wasm32")]
   use crate::drawing::*;
   ```
   This alone accounts for 17 of the original 18 errors (`FlowHost`, `canvas`, `render_scene_json`,
   `export_svg_json`, `export_pdf_json`, `export_dwg_json`, `import_dwg_json`, `trace_bitmap_json`,
   `boolean_segments_json` all unresolved; the reported E0277 "str cannot be known at compile-time" on
   `create_canvas_surface`'s `.map_err(|err| JsValue::from_str(&err))?` was cascade noise from the
   unresolved `canvas` module, not a real bug — `create_canvas_surface`'s error type is `String`, which
   `&err` derefs to `&str` correctly once `canvas` resolves).

2. **`🌿️vcs/🦀️component.rs`** — `create_document_envelope` was dropped from the `os_store` import list
   (the 18th error). Restored, but gated rather than reinstated unconditionally (it's used only inside
   `#[cfg(target_arch = "wasm32")] mod flow_vcs_wasm` and `#[cfg(test)] mod flow_vcs_tests` — confirmed
   by grepping every call site — so an unconditional import would warn-unused on native):
   ```rust
   #[cfg(test)]
   use crate::os_store::ArtifactCommand;
   use crate::os_store::{ArtifactEnvelope, ArtifactStore};
   #[cfg(any(target_arch = "wasm32", test))]
   use crate::os_store::create_document_envelope;
   ```

Verified with the wasm build (`bun … 📜️script.ts wasm`) and native `cargo check -p
semio-framework-os-flow` — see §Verify below. **This closes the ticket's literal ask.**

## Additional fixes (same root cause, discovered completing the ticket's own required
`cargo test -p semio-framework-os-flow --lib` verification step)

`cargo test --lib` doesn't compile `#[cfg(target_arch="wasm32")]` code either, but it *does* compile
`#[cfg(test)]` code, which the same over-pruning commit also broke — 170 test-compile errors on first
run. Since these are squarely inside my `flow/**` lease and block a verification step the ticket
explicitly requires, I fixed the ones sharing the identical root cause:

3. **`🖥️host/🦀️component.rs`** test module used `use crate::*;` instead of `use super::*;` — `crate::*`
   only reaches the crate root's own re-exports (`pub use host::*` etc.), not `host`'s own *private*
   `use` imports (`Dictionary`, `NeuralValue`, `Atom`, `EvalError`, `EvalChannels`, `Neuron`,
   `channel_output`, `HashMap`, `BTreeMap`, `ArtifactCommand`, `DagNodeKind`, …) — those are visible to
   `host::tests` as a descendant module, but only via `super::*`, not `crate::*`. Both sibling test
   modules in this same crate (`vcs`, `📖️playbook`) already use `super::*`; `host` was the outlier.
   Switched, and added `DagPreviewContent`, `computation_node_width`, `slider_widget_height` — three dag
   helpers used only by tests, trimmed from `host`'s own top-level `dag` import by the same commit —
   directly to the test module's own `use dag::{…}` line rather than restoring them to production scope.
   This resolved 152 of the 154 host.rs test errors in one shot.

4. **`🖥️host/🦀️component.rs`** line ~3065, `undo_redo_add_widget` test — two real, independent bugs,
   unmasked once the imports above stopped hiding them behind "cannot find type" noise:
   - `let mut store = FlowStore::new(envelope);` — `ArtifactStore::new` returns `Result<Self, VcsError>`
     (this is the store crate's own committed, stable signature — not something I changed); the test
     never unwrapped it, so every subsequent `store.dispatch()`/`.snapshot()` failed to resolve as a
     method on `Result`. Added `.expect("valid flow store fixture")`.
   - `store.dispatch(ArtifactCommand::Apply { mutations, description: None })` — struct-field-shorthand
     `mutations` referred to a local variable that doesn't exist; the actual local is named `operations`
     (`let operations = flow_fixture_operations(&fixture_before, &host.fixture);` two lines above) — a
     copy-paste rename bug, unrelated to the import cleanup. Fixed to `mutations: operations`.

5. **`🌿️vcs/🦀️component.rs`** `coalesced_layout_drag_produces_one_edit` test — same
   `FlowStore::new(...)` unwrapped-Result bug as #4. Added `.expect("valid flow store fixture")`.

6. **`📔️registry/🦀️component.rs`** — `FlowExtensionRegistryState` had already been widened to
   `pub(crate)` (staged, pre-existing when I started) but its two fields (`registry`, `generation`) were
   left module-private, so `host`'s test code (a different module) couldn't write
   `state.registry = …; state.generation += 1;`. Completed the widening: `pub(crate) registry: …`,
   `pub(crate) generation: u64`.

Net effect: `cargo test -p semio-framework-os-flow --lib` compile errors went **170 → 11**, all now
attributable to two causes that are genuinely outside this lease (below), not to the import-pruning
mechanism.

## What is NOT fixed, and precisely why

**A. `vcs::apply_mutation` — 3 remaining test errors (2 in `vcs.rs`, plus this exact function is also
gone from `✏️s/🔌️plugins/🪐️space`'s `⚙️engine/🪐️space/🎚️config/🦀️component.rs`, confirmed while running
the required space regression check).** This helper is not merely misplaced — it no longer exists
anywhere in the codebase. Its removal is *documented and deliberate*, per `📡️spr/🎮️command/🦀️component.rs`
line 64: "the CRDT-era concurrent-diff merge helper this docstring used to point at is deleted, see
`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`". Reconstructing an equivalent
requires composing `Mutation::diff` + `MutationOutcome::apply_to` correctly against that ticket's new
outcome/merge-policy model — that redesign belongs to that ticket, not to this repair lane. Not touched.

**B. Missing fixture file `📚️examples/🌊️default.flow` — 2 remaining test errors** (`include_str!` in
both `host.rs` and `vcs.rs`, `default_flow_example_dsl_round_trips`-style tests). The directory
`🧰️framework/🛍️products/💻️os/🔨️modules/📚️examples/` does not exist at all. The test's own doc comment
says this file is "the permanent proof" of a hand-crafted DSL-text migration from an old
`🌊️default.flow.json` — I have neither the original JSON nor the DSL grammar knowledge to hand-author a
byte-correct replacement without fabricating fixture content, which CLAUDE.md's "no ugly
migrations/handcraft all assets" rule places squarely on whoever owns that migration. Not touched.

**C. `📖️playbook/🦀️component.rs` test module — 6 `DslValue` vs `serde_json::Value` mismatches**
(`default: Some(json!(...))` expects `DslValue`, not `Value`), discovered in the same `cargo test --lib`
sweep. `playbook` is mounted into the flow crate via `#[path]` but physically lives at
`🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/`, outside `🌊️flow/**`. It is stable (last touched
2026-08-14, not live-edited) but a 6-site type-conversion fix in a foreign file exceeds "unavoidable
one-line import/signature adaptation." Not touched.

**D. `semio-s-plugin-space --lib` regression check — 18 errors, was NOT passing at 210/0 as the ticket
expected.** Entirely pre-existing and unrelated to this lane (I never touched anything under
`✏️s/🔌️plugins/🪐️space/**`; `git log --date=iso` on the erroring file shows last commit 2026-08-17
12:10:50, `git status` shows it clean, i.e. not live-edited during this run). 16 of the 18 errors are the
identical "unused import over-pruned by the same commit" pattern (`Arc`, `OsBackbonePort`,
`LocalStorageBackbonePort`, `SpaceKind`, `SpaceVisibility`, `OsSpaceDocument`, `SpaceConfig`,
`S_SPACE_SCHEMA`, several `fn`s — all "cannot find in this scope" in a test module), and the other 2 are
the same `vcs::apply_mutation` foreign-ticket removal as §A. Flagging with full evidence rather than
fixing — `✏️s/🔌️plugins/**` is outside this lease.

## Verify (real output)

| Command | Result | Log |
|---|---|---|
| `bun 🫀️core/📦️packages/🦀️rust/📜️script.ts wasm` | **succeeded** — `pkg/flow_core_bg.wasm` (42.18 MiB) emitted at both `🫀️core/pkg/` and `📦️packages/🦀️rust/pkg/`, `[INFO]: ✨ Done in 8m 55s` | `🧪️flow-fix-wasm-build-success.txt` |
| `cargo check -p semio-framework-os-flow` (native) | **0 errors**, 0 warnings in this crate (only pre-existing warnings in dependency crates: `semio-framework-plugin`, `semio-s-plugin-stdio`) | `🧪️flow-fix-native-check.txt` |
| `cargo test -p semio-framework-os-flow --lib` | Does **not** compile: **11 errors** remain, all attributable to §A/§B/§C above (0 attributable to the wasm-fix work or to import-pruning left unfixed) | `🧪️flow-fix-test-lib.txt` |
| `cargo check -p semio-framework-surface` | **0 errors** — no regression (1 pre-existing unrelated unused-import warning, not touched) | `🧪️flow-fix-surface-regression.txt` |
| `cargo test -p semio-s-plugin-space --lib` | **Does not compile — 18 errors**, NOT the expected 210/0. Pre-existing, unrelated to this lane (§D) | `🧪️flow-fix-space-regression.txt` |

The wasm build took ~9 minutes and needed 3 attempts: attempt 1 hit the 18 known errors before any fix;
attempt 2 (after the `wasm/component.rs`+`vcs.rs` fix) hit a transient, unrelated compile failure in
`🧰️framework/🔨️modules/🚪️io/🦀️component.rs` (`build_proposed`/`descriptor_of` signature mismatch against
`EntryMap`'s `&'static IoEntry` value type) — that file was mid-edit by another live session (`git
status` showed `MM`, mtime matching the exact build window); attempt 3, after waiting, compiled clean.

## Changed files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`

Not created/modified/deleted anything under `✏️s/🔌️plugins/🗄️stdio/**` or `📜️world.wit` (forbidden), or
anything under `✏️s/🔌️plugins/**` at all.

Logs (all in this ticket folder, `.txt` not `.log`): `🧪️flow-fix-wasm-build-success.txt`,
`🧪️flow-fix-native-check.txt`, `🧪️flow-fix-test-lib.txt`, `🧪️flow-fix-surface-regression.txt`,
`🧪️flow-fix-space-regression.txt`.

## sharedFileRequests

None. Every file touched is inside `🌊️flow/**` (my lease) except the registry-field visibility widening,
which was already staged and incomplete before I started (not a live foreign edit I intercepted).

## What is NOT done (summary)

- `cargo test -p semio-framework-os-flow --lib` does not compile — 11 errors, all outside this lease
  (§A missing `vcs::apply_mutation`, owned by `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`;
  §B missing `📚️examples/🌊️default.flow` fixture asset; §C `📖️playbook` DslValue mismatches).
- `cargo test -p semio-s-plugin-space --lib` does not compile — 18 errors, entirely pre-existing and
  outside this lease (§D), NOT caused by this lane's work. The ticket's expected 210/0 baseline does not
  currently hold; whoever owns `✏️s/🔌️plugins/🪐️space/**` needs to know their test module has the same
  import-pruning damage as flow's did, plus the same `vcs::apply_mutation` cross-cutting removal.
- The ticket's own literal ask — wasm compiles, native check is 0 errors, no regression in
  `semio-framework-surface` — is fully done and verified with real command output above.
