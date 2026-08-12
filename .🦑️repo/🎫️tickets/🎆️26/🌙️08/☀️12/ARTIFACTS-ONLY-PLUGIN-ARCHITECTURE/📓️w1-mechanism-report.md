# W1 mechanism report — `ArtifactDeclaration` + builder + `genesis()`

`apa-status: partial` — mechanism landed, one exemplar plugin converted and verified, `.setup()` intentionally NOT deleted yet (see "What remains" below — deleting it now was explicitly out of scope and would have broken 31 plugins at once).

## What changed

### 1. `ArtifactDeclaration` (M1) — new region, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:930-1241`

New region `//#region 🔖️ArtifactDeclaration` / `//#endregion 🔖️ArtifactDeclaration`, inserted immediately after the existing `//#endregion 🔖️ArtifactBuilder` (old :928, now unmoved), inside `pub mod app { … }`. Nothing existing was reordered or edited to make room.

- `ArtifactDeclaration` (:944-964) — the data struct. Every field is module-private (no `pub` anywhere on the struct or its fields); only `register_all` (:1140, `pub(crate)`) ever reads them.
- `ArtifactDeclarationBuilder<State>` (:973-987) — consuming typestate builder, mirroring `PluginBuilder`'s own `NeedsLabel → NeedsVersion → Ready` shape: `NeedsSchema → DeclarationReady`. `.schema(...)` is the one call that unlocks every other method — a declaration missing it is a compile error (the type simply has no other methods until `.schema()` is called), matching the design doc's "malformed declaration is a compile error" requirement.
- `DocumentCodecSpec` (:1127-1138) — a monomorphized non-capturing `fn()` thunk so `.document_codec::<A>()` can store the registration as inert data instead of performing it immediately; the actual call (`plugin_runtime::register_document_codec_for_app::<A>(A::DOCUMENT_SCHEMA)`) only happens inside `register_all`.
- `ArtifactDeclaration::register_all` (:1140-1219, `pub(crate)`) — walks one declaration in the fixed order **schema → inferences → formats → subset validators → composers → languages → document codec → migrations**, then unions `capabilities` into the `Plugin`. Called exactly once per declared artifact, from `PluginBuilder::build()`.
- Curated re-export: added `ArtifactDeclaration` to the existing `pub use app::{ … WindowKindSpec, ArtifactDeclaration, };` list at :10679 — one name appended to an existing list, nothing reordered, nothing UCAS-adjacent touched.

**Ownership check** (:1148-1186, the load-bearing part of `register_all`) — two layers, because on-disk kind strings are **pre-migration** (see below):
1. **Always enforced.** Every composer entry must actually be *about* the declared `kind`: either `writes.artifact_kind == kind` (import direction) or `kind` appears somewhere in `reads` (export direction — an artifact's own composer legitimately *writes* a foreign format when exporting, e.g. note→svg, so a naive "writes must equal kind" would reject every real export entry; I verified this against note's own 7-entry composer table before committing to the check). Subset validators and dialect migrations are always strictly `== kind` (they are never about a foreign dialect).
2. **Enforced once `kind` is canonical.** If `kind` parses as `s.<plugin>.<artifact>` (`ArtifactKindId::parse`), its plugin segment must equal the builder's `plugin_id` — the precise, structural form of "a plugin may only declare artifacts it owns," and the direct countermeasure to the named lowpoly violation (`register_mesh_exporter("3d.mesh", …)` — a call naming a kind lowpoly had no connection to at all).

**Why `kind` is `String`, not `ArtifactKindId` (a deliberate deviation from the design doc's illustrative struct).** I traced note's and raster's *actual* on-disk kind strings before writing this: note's `Dialect.artifact_kind` is `"s.note"` (2 segments), raster's `ArtifactKindSpec.id` is `"2d.raster"` — **neither is canonical `s.<plugin>.<artifact>` grammar**. `ArtifactKindId::parse`'s own doc says this explicitly: *"This wave lands the type and validator only; renaming existing artifact ids to this grammar is a later wave."* Requiring `ArtifactKindId::parse` to succeed in `ArtifactDeclaration::builder()` would have made the mechanism unusable for every one of today's 33 plugins until a repo-wide kind-string migration (UCAS/SMO territory, not mine) lands first — a chicken-and-egg block that would have made "convert one plugin as the proof" impossible. Storing `kind: String` and upgrading to the strict canonical check only where the grammar already parses means the check **tightens itself automatically** as that migration lands, with no second pass needed here. Documented in the field's own doc comment (:944-950) and in `register_all`'s doc (:1148-1154) so nobody mistakes this for a shortcut.

### 2. Composition slots — `.composition::<Snapshot>()` (:1084-1092)

`child_slots`/`link_slots` are `&'static [ChildSlotSpec]` / `&'static [LinkSlotSpec]`, set **only** via `.composition::<Snapshot: ArtifactCompositionFields>()`, which pulls `Snapshot::child_slots()`/`Snapshot::link_slots()` — no other setter exists, matching UCAS's review requirement exactly (a hand-written list would be unwritable, not merely discouraged). **Verified by compiling, not by reasoning**, per the design doc's explicit instruction: `semio-framework-schema` was *already* a dependency of `semio-framework-plugin`'s `Cargo.toml` (added by UCAS's C1 for `ArtifactChildren`, comment at that Cargo.toml explains why) — reachability was a non-issue. `ChildSlotSpec.kind` is confirmed `&'static str` (schema crate doesn't depend on framework-core), exactly as the design doc predicted. No plugin registers composition slots yet (the exemplar conversion below doesn't call `.composition()` — note has no children/links); the fields exist and are typed correctly but currently trigger a `dead_code` warning, which will resolve the moment any plugin calls `.composition::<Snapshot>()`.

### 3. Builder changes — `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs`

- Added `artifacts: Vec<ArtifactDeclaration>` field (:20) and `pub fn artifact(mut self, declaration: ArtifactDeclaration) -> Self` on `PluginBuilder<Ready>` (:82-89), repeatable, matching `PluginBuilder`'s existing fluent style.
- `build()` (:149-172): after constructing `Plugin::new(...)`, walks `self.artifacts` and calls `declaration.register_all(&plugin_id, plugin)` for each, in declaration order, **before** capabilities/commands/artifact_kinds/apps are threaded through (those still apply afterward, unioning correctly since `register_all` returns the `Plugin` it was handed).
- **`setup: Option<fn()>` was NOT deleted**, despite the design doc's literal instruction (`"Delete the setup: Option<fn()> field, PluginBuilder::setup(), and the if let Some(setup) = self.setup { setup(); } at :143-145"`). I measured before acting: `grep -rn '\.setup(' ✏️s/🔌️plugins/` → **31 plugin call sites**. Deleting the method would have broken every one of those 31 crates simultaneously, directly contradicting the same packet's own SCOPE DISCIPLINE section (*"If `.setup()` cannot be removed without breaking every plugin at once, then: add `.artifact()` … convert ONE plugin … A landed mechanism plus one working exemplar beats a half-migrated tree"*). I kept `setup`/`.setup()`/the `build()` call exactly as they were, and documented the retirement plan inline on the field (:20-24) and on `.setup()`'s own doc comment. Both `.setup()` and `.artifact()` coexist on `PluginBuilder<Ready>`; `build()` runs `.setup()` first, then walks `.artifact()` declarations.

### 4. M4 — `genesis()` replacing `ArtifactApp::seed` — `🔌️plugin/🦀️component.rs`

- Trait method: old `fn seed(_store: &mut ArtifactStore<Self::Snapshot, Self::Mutation>) {}` (old :4792) → new `fn genesis() -> Vec<Self::Mutation> { Vec::new() }` (now :5107-5109).
- Call site, `VcsArtifactApp::with_registry` (now :5701-5706): old `A::seed(&mut store);` → 
  ```rust
  let genesis_mutations = A::genesis();
  if !genesis_mutations.is_empty() {
      store
          .dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, description: Some("genesis".to_string()) })
          .expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
  }
  ```
  This is real dispatch through `ArtifactStore::dispatch(ArtifactCommand::Apply { .. })` — the exact same command variant every user edit goes through — not a bespoke code path. This removes the only place an app touched a store directly, per the design doc.
- **One override exists repo-wide and it CANNOT be losslessly ported** — see "sharedFileRequests" below; I did not touch it (SMO's held lane).

### 5. `register_mesh_exporter` / `register_app_io` removal — measured, not assumed

Grepped `register_mesh_exporter|register_app_io|register_mesh_importer|register_solid_exporter|register_solid_importer|register_dwg_import_handler|register_2d_export_handlers|register_mesh_dwg_*` across the full 11,159-line file. **Zero function definitions found** — one docstring mention of `register_app_io` (a comment pointing forward to future work, not code) at the `AppCommands` region. Nothing to remove: the premise that copies of this family live in `🔌️plugin/🦀️component.rs` did not hold when measured. Recorded here rather than silently skipped, per the evidence-discipline protocol (measurement over plausible story).

### 6. Exemplar plugin conversion — `🗒️note` (chosen over raster; both were viable, note has richer IO to prove the export-direction ownership check against)

- `✏️s/🔌️plugins/🗒️note/🦀️component.rs` — `plugin()` now calls `.artifact(crate::artifacts::note::engine::declaration())` instead of `.setup(crate::artifacts::note::engine::register)`. `.setup()` is kept for exactly one call: `crate::apps::note::config::schema::register_app_schema` — app-scope config/presence schema, the one §6 function `ArtifactDeclaration` deliberately has no field for (see mapping table below).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` (this file is triple-mounted into note's crate: directly as `crate::artifacts::note::standards::v1::engine`, and re-exported as `crate::artifacts::note::engine` via a glob `pub use super::standards::v1::engine::*;` in `📦️glue.rs:570` — I traced this before writing the `.composers()` call to make sure I was pointing at the *real*, 7-entry composer table, not an orphaned duplicate). `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inferences()` (all confirmed to have zero other call sites repo-wide before deleting) replaced by `declaration()` (data) and a private `pilot_languages()` helper (`OnceLock`-backed `&'static [dsl::LanguageSpec]`, since `dsl::passthrough_hooks` isn't `const fn` so the 5 language specs can't be a `const` array).
- Three **unrelated, pre-existing** compile errors in note's own crate, found only because this was the first time `semio-s-plugin-note --all-targets` ever linked far enough to reach them (previously always masked by `semio-s-plugin-stdio` being red — see Concurrent-churn below). Fixed as trivial, compiler-suggested, single-line changes, evidence they predate this session pasted below:
  - `🎛️apps/🗒️note/🎮️commands/🗃️fixture/🦀️component.rs:69` — `HistoryView<'static>` → `HistoryView` (the struct lost its lifetime parameter at some earlier point; this one caller was never updated).
  - `🗿️artifacts/🗒️note/…/🧬️mutations/❌️delete-block/↩️inverse/🦀️component.rs:10` and `…/🧺️delete-blocks/↩️inverse/🦀️component.rs:15` — `CreateBlock.block` is `Box<NoteBlockNode>`; both inverse leaves still constructed it as a bare `NoteBlockNode`. `Box::new(...)` added at both call sites.

## Exhaustive declaration-field ↔ registration-function mapping (§6 census)

| §6 function | `ArtifactDeclaration` field | covered? |
|---|---|---|
| `register_artifact_schema_descriptor` | `schema: Option<ArtifactSchemaDescriptor>` | ✅ |
| `register_artifact_inference_descriptor` | `inferences: Vec<ArtifactInferenceDescriptor>` | ✅ |
| `register_composer_entries` | `composers: &'static [ComposerEntry]` | ✅ |
| `register_format_descriptors` | `formats: Vec<FormatDescriptor>` | ✅ |
| `register_subset_validator` | `subset_validators: &'static [SubsetValidatorEntry]` | ✅ |
| `register_language` | `languages: &'static [LanguageSpec]` | ✅ |
| `register_document_codec` / `register_document_codec_for_app` | `document_codec: Option<DocumentCodecSpec>` | ✅ |
| `register_dialect_migration` | `migrations: Vec<DialectMigration>` | ✅ |
| — (no registrar; derived from the type) | `child_slots`/`link_slots` via `.composition::<Snapshot>()` | ✅ (composition slots, not a §6 registrar) |
| — (plugin-level, not artifact-level) | `capabilities: Vec<CapabilityRequirement>` | ✅ |
| **`register_app_schema_descriptor`** | **none** | ❌ **loudly** — app-scope (config/presence schema for an `ArtifactApp` owner), not artifact-scope. `ArtifactDeclaration` is about what an *artifact* registers; config/presence belong to the *app* that owns it. No equivalent app-level declaration builder exists yet. Note's exemplar keeps this call live via `.setup()` (see above) — it is NOT silently dropped. |
| **`register_linked_flow_extension_installer`** | **none** | ❌ **loudly** — flow's own extension-installer registry (`semio-framework-os-flow`), used only by `🌊️flow` (7 call sites, all in flow's own crate, an SMO "between waves" lane I have no access to). Structurally similar to a §6 registrar but scoped to exactly one plugin's own extension mechanism, not a general artifact concern. Flagged, not built — no plugin outside flow needs it, and flow is not mine to enter. |
| `set_io_fallback_dispatcher` | none | excluded by design — 0 call sites in `✏️s/🔌️plugins/`, host/OS-boot-only per the census's own note (§6 confirms this explicitly). |
| `register_studio_port` | none | excluded by design — not a framework SDK fn at all; `pub(crate)` to `🪐️space`'s own `🏠️home` app, registers into a plugin-local static, per the census's own note. |

**7 of 9 real §6 artifact-scoped registrars have a field. The 2 without one are named and justified above, not silently missing** — satisfying the "escape hatch survives" failure-avoidance criterion from the dispatch.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — new `🔖️ArtifactDeclaration` region (:930-1241), `genesis()` (:5107-5109, was `seed` at old :4792), `with_registry` call site (:5701-5706), one name added to the `pub use app::{…}` re-export list (:10679). 338 insertions / 8 deletions (`git diff --stat`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — `artifacts` field + `.artifact()` + `build()` walk, `setup` kept. 31 lines changed.
- `✏️s/🔌️plugins/🗒️note/🦀️component.rs` — `plugin()` converted to `.artifact(declaration())` + narrowed `.setup()`. 9 lines.
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️component.rs` — `register()`/`register_pilot_languages()`/`register_artifact_schema()`/`register_artifact_inferences()` → `declaration()` + `pilot_languages()`. 151 lines changed (net smaller: 4 wrapper fns collapsed to 1 declarative one + 1 helper).
- `✏️s/🔌️plugins/🗒️note/🎛️apps/🗒️note/🎮️commands/🗃️fixture/🦀️component.rs` — 1-line unrelated pre-existing fix (`HistoryView<'static>` → `HistoryView`).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/❌️delete-block/↩️inverse/🦀️component.rs` — 1-line unrelated pre-existing fix (`Box::new`).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧺️delete-blocks/↩️inverse/🦀️component.rs` — 1-line unrelated pre-existing fix (`Box::new`).

Nothing created, nothing deleted at the file level. `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🦀️component.rs`'s own `pub mod io_registry { entries()/compose()/register() }` (its :258-279) is now fully orphaned (zero call sites repo-wide, confirmed by grep) — left in place rather than deleted, since removing it is unrelated cleanup outside this wave's scope; flagged here for whoever next touches note.

## Verification — commands run, real output

**1. `cargo check -p semio-framework-plugin --all-targets`** (RUSTC_WRAPPER disabled):
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-framework-plugin --all-targets
warning: `semio-framework-plugin` (lib) generated 40 warnings (all pre-existing except one new `dead_code` on child_slots/link_slots, expected — see composition slots section above)
warning: `semio-framework-plugin` (lib test) generated 50 warnings (38 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 1.06s
```
**0 errors.**

**2. `cargo test -p semio-framework-plugin --lib`**:
```
test result: FAILED. 147 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```
The 3 failures are `component::derived_artifact_children_tests::derived_composer_reads_defaults_to_composition_reads_for_a_leaf_with_no_children`, `component::plugin_runtime::plugin_builder_contract_tests::composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles`, `component::plugin_runtime::plugin_builder_contract_tests::group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child` — **all three are inside `//#region 🔖️CompositionTests` (:8880+) and `derived_artifact_children_tests` (:10900+), UCAS's own C1 composition/group-undo/children test suite, explicitly forbidden territory** (`Emit.child_emits`, group undo, `ArtifactChildren`). Verified NOT caused by my diff: `git diff --stat` on this file shows exactly my two edits (the new region, +338/-8 net at the ArtifactDeclaration insertion point and the genesis site) — nothing near lines 8880-10950. Verified `genesis()` is behaviorally inert for these tests specifically: `grep -n 'fn seed(\|fn genesis('` finds exactly one `fn genesis(` in the whole file (the trait default I added); `TestApp` (the fixture these three tests use) does not override it, so it uses the default `Vec::new()` — byte-identical behavior to the old default no-op `seed()`. These are pre-existing UCAS-side bugs, not mine.

**3. `cargo check -p semio-s-plugin-note --all-targets`** (the exemplar):
- First 5 attempts (spanning several minutes): red, but with the error CONTENT converging (6 → 3 → 1 → 1 → then a clean pass through to `semio-s-plugin-note` itself), all errors exclusively inside `semio-s-plugin-stdio` (`SemioMutation`/`SemioSubsetSnapshot` gaining `Table`/`Graph` variants mid-rename — UCAS's stdio roster, explicitly documented as "not frozen" in `📓️status.md`). Zero mentions of any `🔌️plugins/🗒️note` path in any of these 5 outputs — grep-verified each time (`grep -c "🔌️plugins/🗒️note" <output>` → 0 in all 5). Retry-and-wait protocol followed (5 attempts, exceeding the "up to 3×" minimum) rather than patching stdio.
- 6th attempt: stdio compiled clean, exposing **3 real errors inside note's own crate** — none in code I wrote; all three in unrelated files (see "Files touched"), confirmed pre-existing by `stat -f '%Sm'` (mtimes ~14:55/15:21, hours before this session's work) and `git log --oneline -3` (last commits at flags well before mine). Fixed all three (trivial, compiler-suggested, single-line). Re-ran:
```
$ RUSTC_WRAPPER="" CARGO_TARGET_DIR=".../🎯️target" cargo check -p semio-s-plugin-note --all-targets
warning: `semio-s-plugin-note` (lib) generated 9 warnings (run `cargo fix --lib -p semio-s-plugin-note` to apply 6 suggestions)
warning: `semio-s-plugin-note` (lib test) generated 12 warnings (7 duplicates)
    Finished `dev` profile [unoptimized] target(s) in 14.58s
```
**0 errors.** This is the real, compiler-verified proof that `.artifact()` + `ArtifactDeclaration` works end-to-end on a live plugin.

**4. `cargo test -p semio-s-plugin-note --lib`**:
```
test result: FAILED. 81 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
```
Two failures, both **freshly uncovered pre-existing bugs, unrelated to M1** — this crate's test suite had never run to completion before (blocked first by stdio, then by the 3 compile errors above), so neither failure has a "before" baseline; they are newly visible, not newly caused:
- `apps::note::panels::document::tests::renders_document_tree` — `assertion failed: json.contains("Welcome")`, a panel-content fixture mismatch unrelated to registration.
- `artifacts::note::standards::v1::subsets::any::schema::mutations::component::tests::block_lifecycle_inverse_law_create_delete_duplicate` — `delete_blocks`'s inverse restores two blocks (`Table`/`Math`) in swapped order. Traced this to `🧺️delete-blocks/↩️inverse/🦀️component.rs`'s own reinsertion loop (sorts removed entries by original `(parent_id, index)` then reinserts sequentially — a classic multi-item-reinsertion index-shift bug), **not** to my one-line `Box::new()` fix, which only changes how the value is wrapped, never its value or insertion order. Left unfixed — a real domain bug in note's block-tree inverse law, out of M1's scope.

## sharedFileRequests

**`✏️s/🔌️plugins/🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs:183-185`** — `🌿️vcs` is SMO's "between waves" held lane (per `📌️important.md`'s protocol table); I did not touch it. Its `VcsDemoApp` overrides `fn seed(store: &mut ArtifactStore<VcsSnapshot, VcsDemoMutation>) { seed_vcs_demo_history(store); }`. Since the trait method it overrides no longer exists (renamed to `genesis() -> Vec<Self::Mutation>`), **`semio-s-plugin-vcs` will fail to compile** with `E0407: method 'seed' is not a member of trait 'ArtifactApp'` the moment anyone checks it. This is a **direct, unavoidable, and correctly-scoped consequence** of the M4 change the dispatch explicitly asked for — not a mistake, but it needs SMO's attention before they next gate on vcs.

This is also a genuine design gap worth flagging precisely, not just a rename: `seed_vcs_demo_history` (`🌿️vcs/🎛️apps/🌿️vcs/🦀️component.rs:82-107+`) builds **rich multi-command history** — `ArtifactCommand::Apply`, `CommitCheckpoint`, `CreateAlternative`, `CheckoutCheckpoint`, interleaved and depending on IDs returned by earlier steps (`last_checkpoint_id`, `active_alternative_id`). `genesis() -> Vec<Self::Mutation>` (a flat list of mutations, applied via one `ArtifactCommand::Apply`) **cannot express this** — it has no way to name a checkpoint/alternative or thread an id from one step's result into the next. This is exactly the case the OLD `seed`'s own doc comment called out ("only apps whose fixture is itself a rich history … need this"), and it is now the one case `genesis()` cannot cover. SMO will need either a widened genesis contract or a distinct "demo fixture" mechanism outside the `ArtifactApp` trait entirely for this one app. I have not proposed a fix — this needs a design decision, not a patch, and vcs is not mine to enter.

No other shared-file requests. `🖥️host/🦀️component.rs` (M5, UCAS's `IoRouter`) was never entered — M5 was not in my dispatch's "WHAT TO BUILD." `🎠️kernel`/`🚪️io` `Registrar` placement (M2) was never entered — also not in my dispatch's scope (confirmed by re-reading the dispatch prompt's "WHAT TO BUILD" section, which covers only M1, composition slots, builder changes, and M4).

## Concurrent-churn observations

1. **`semio-s-plugin-stdio` was red for the first ~5 of 6 `cargo check -p semio-s-plugin-note` attempts**, converging 6→3→1→1→0 stdio-side errors across retries as UCAS actively fixed non-exhaustive `SemioMutation`/`SemioSubsetSnapshot` matches (newly added `Table`/`Graph` variants). Matches `📓️status.md`'s own live account of this exact defect (D2). Zero of my own paths ever appeared in any of these outputs. Not touched.
2. **`✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`** gained an unrelated `use protocol::SemanticMutation;` import (mtime 19:33, well inside my session's window) from a session I did not initiate — almost certainly SMO's semantic-mutations work touching the same test module I read (but did not edit) for the `block_lifecycle_inverse_law_create_delete_duplicate` test. No conflict: different lines, both compile together, confirmed by the clean `cargo check -p semio-s-plugin-note` run above.
3. **`semio-framework-plugin`'s 3 pre-existing test failures** (composition/group-undo territory) reproduced identically across two separate `cargo test` runs taken ~20 minutes apart — stable, not flaky, confirming they are a real UCAS-side defect rather than a race.

## What remains before `.setup()` can be deleted repo-wide

1. **31 plugin crates still call `.setup(...)`** (measured: `grep -rn '\.setup(' ✏️s/🔌️plugins/ --include='*.rs' | wc -l`). Each needs its own `register()` → `declaration()` conversion following exactly the pattern this report demonstrates on note: replace the side-effecting free function with one returning `ArtifactDeclaration`, wire it through `.artifact(...)` in the plugin root, and — critically — check whether the plugin calls `register_app_schema_descriptor` (keep via a narrowed `.setup()`, as note does) or `register_linked_flow_extension_installer` (flow only; same treatment).
2. **Kind-string canonicalization** (UCAS/SMO territory, tracked separately) needs to land before the ownership check's *strict* layer (plugin-segment match) activates for any given artifact. Until then, the *loose* layer (composer must produce-or-consume the declared kind) is the only ownership guarantee in effect for non-canonical kinds — real, but weaker than the design doc's illustrative "writes must equal decl.kind" until that migration lands. This is now self-describing in code (see `kind`'s field doc) so nobody mistakes today's behavior for the end state.
3. **`🌿️vcs`'s `seed` override** (see sharedFileRequests) needs a design decision — not a mechanical port — before vcs can adopt `genesis()`. It is the one real gap `genesis() -> Vec<Self::Mutation>` has against the shape `seed(&mut ArtifactStore)` covered.
4. **M2 (`Registrar` seal) and M3 (SDK curated re-exports / Cargo purge) were not built** — out of this dispatch's scope by its own "WHAT TO BUILD" section. Until M2 lands, `register_all`'s calls into `semio_framework::register_*`/`store::register_*`/`dsl::register_language` are still reachable by any code with the right `use` path, same as before this wave; the mechanism makes registration *declarative and ownership-checked*, not yet *capability-sealed*.
5. **Composition slots have no consumer yet** — `child_slots`/`link_slots` are correctly typed and wired to `.composition::<Snapshot>()`, but no plugin calls it (note has no children/links to declare) and no `register_*` function consumes them (UCAS's composition runtime reads `ArtifactCompositionFields` directly off the snapshot type, not through a declaration). The `dead_code` warning on these two fields will persist until either a plugin with real children calls `.composition()`, or UCAS's runtime is repointed to read them from here instead.

## Honest pass/fail

- M1 `ArtifactDeclaration` + typestate builder: **built, compiles, exhaustively field-mapped.**
- Composition slots via `.composition::<Snapshot>()`: **built, compiles, unexercised** (no plugin needs it yet).
- Builder `.artifact()` + `build()` walk + ownership check: **built, compiles.**
- `.setup()` deletion: **not done, deliberately** (would have broken 31 plugins; scope discipline honored).
- M4 `genesis()`: **built, compiles, one known un-portable override** (vcs, reported, not fixed — SMO's lane).
- `register_mesh_exporter`/`register_app_io` removal: **nothing to remove** (measured, not present in this file).
- Exemplar plugin (`🗒️note`): **converted, compiler-verified 0 errors on `--all-targets`**, 2 freshly-uncovered unrelated pre-existing test failures reported not fixed.
- `semio-framework-plugin` itself: **0 compile errors, 3 pre-existing UCAS-side test failures** (not mine, verified not caused by my diff).
