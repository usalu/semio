# Wave 2 — `norm` / `din16798` / `1` / `any` — mutations facet migration

Facet: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/📗️din16798/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations`
Crate: `semio-s-plugin-norm`

## Vocabulary derived

`Din16798Snapshot` is a flat, id-less, document-root parameter form: sixty-two persistent scalar
fields (occupancy, ventilation, comfort, heat-recovery, infiltration, cooling, storage and
duct-leakage inputs to a DIN EN 16798-1 compliance check) — no id-keyed collections, no
name/identity field, no nested structured owners. Per `📓️derivation-rules.md` rule 1
("document-level scalars... `change-<field>` per remaining scalar"), every one of the 62 fields
became its own `change-<field>` mutation. None qualified for the `update-<facet>` grouping
exception — each parameter is independently measured/entered on its own spec sheet, never
validated as an atomic multi-field bundle. The single pre-migration generic variant,
`SetSnapshot { snapshot }`, is gone: banned outright per taxonomy/derivation-rules rule 6, with
**no replacement mutation** — whole-document replace is not an in-history mutation; it goes
through `store::ArtifactStore::reset` (file-open/import/load-example), entirely outside
`Din16798Mutation`. `NoMutation`/`CollectionMutation` were never present in this facet (the
snapshot has no collections at all).

All 62 new variants: `ChangeAnnex`, `ChangeOccupancy`, `ChangeComfortCategory`, `ChangeTOpC`,
`ChangeRhPercent`, `ChangeAirSpeedMS`, `ChangeThetaRmC`, `ChangeCo2Ppm`, `ChangeDfPercent`,
`ChangeLAeqDb`, `ChangePersons`, `ChangeIdaClass`, `ChangeVentilationM3H`, `ChangeFloorAreaM2`,
`ChangeBedrooms`, `ChangeDwellingVentilationM3H`, `ChangeOccupants`,
`ChangeResidentialVentilationM3H`, `ChangeSfpWM3S`, `ChangeSfpRequiredClass`,
`ChangeHeatRecoveryEta`, `ChangeHeatRecoveryEtaMin`, `ChangeSystemType`,
`ChangeYearsSinceInspection`, `ChangeHumidificationRequiredKgH`,
`ChangeHumidificationProvidedKgH`, `ChangeFanQVM3S`, `ChangeFanTRunH`,
`ChangeFanEnergyReferenceKwh`, `ChangeNightSetbackK`, `ChangeHrMDotKgS`, `ChangeHrCpJKgk`,
`ChangeHrDeltaTC`, `ChangeHrTH`, `ChangeHrSavingsReferenceKwh`, `ChangeN50HInv`,
`ChangeVolumeM3`, `ChangeInfiltrationAllowanceM3H`, `ChangeCellarAreaM2`,
`ChangeCellarVentilationM3H`, `ChangeHTrWK`, `ChangeHVeWK`, `ChangeThetaEC`, `ChangeThetaSetC`,
`ChangeCoolingDeltaTH`, `ChangeCoolingGainsKwh`, `ChangeCoolingUtilizationFactor`,
`ChangeCoolingReferenceKwh`, `ChangeChillerType`, `ChangeEerActual`, `ChangeQCKwh`,
`ChangeGenerationReferenceKwh`, `ChangeDataCenterSupplyC`, `ChangeHStWK`, `ChangeThetaStC`,
`ChangeThetaAmbC`, `ChangeStorageTH`, `ChangeStorageAllowanceKwh`, `ChangeDhwDeliveryC`,
`ChangeDuctClass`, `ChangeDuctTestPressurePa`, `ChangeDuctLeakageM3SM2`.

Every `SEMANTICS.kind` was computed with a Python replica of `dsl_derive`'s exact `to_kebab`
algorithm (verified against the derive's own compile-time `const _: () = assert!(...)` check
before any Rust was written), so acronym-heavy fields such as `h_tr_w_k` land on the algorithm's
real output (`change-h-tr-wk`, not the "obvious" `change-h-tr-w-k`) rather than an invented slug
that would fail the derive's assert.

## Directory layout — 1 dir repurposed in place, 61 new dirs added

`📦️glue.rs` (plugin-shared, outside this facet's writable boundary) path-includes exactly one
pre-migration triad directory: `📄set-snapshot`. Since glue.rs couldn't be edited, that directory
was **repurposed in place** (same path, rewritten `🦠️mutation`/`🔺️diff`/`↩️inverse` content) to
hold `ChangeAnnex` (`kind: "change-annex"`) instead of being renamed. The other 61 mutations have
no pre-migration slot and are self-wired directly inside `🧬️mutations/🦀️component.rs` via nested
`#[path = "."] pub mod <name> { #[path = "..."] pub mod mutation; ... }` blocks — mirrors this
ticket's `process`/`process3d` precedent (`#[path]` resolves per physical file, not per logical
mod nesting, so new triads work without ever touching `📦️glue.rs`).

## Diff construction

Every `diff()` builds `Din16798Diff` directly from the payload — real handcrafted sparse
construction, one field set to `Some(payload.new_<field>.clone())`, everything else
`..Default::default()`. Never apply-then-capture. `Din16798Diff` already had every snapshot field
mirrored as `Option<T>` (pre-existing, from the old `SetSnapshot`-era sparse-diff machinery), so no
diff-shape changes were needed — only the construction sites.

## Inverses

Every inverse looks up the pre-change value from `base` and returns exactly one `ChangeXxx`
mutation restoring it — `change` is its own inverse partner (per `📓️taxonomy.md`). All 62 fields
are singleton document-root scalars with no missing-target case (unlike an id-keyed
`delete`/`remove`), so none of these inverses can return `Vec::new()` — they always have exactly
one prior value to restore.

## Other in-boundary files fixed (compile breakage / vocabulary-consistency caused by the change)

- `🧬️mutations/📝️text/🦀️component.rs` — **required, not optional**: `store::ArtifactStore<P,
  Mutation>` bounds `Mutation: OpText + OpBinary`, and the old `impl_norm_set_snapshot_ops!` macro
  (plugin-root, unreachable to edit) only implements those traits for a literal `Self::SetSnapshot`
  variant — deleting that variant made the macro invocation itself uncompilable, so it was dropped
  from the dispatch file and this file got a full hand-written replacement instead: a local
  `Din16798MutationDsl` mirror enum (`#[derive(dsl::DslEnum)]`, 62 flat keyworded records, no
  `#[dsl(block)]` needed since every field is a plain scalar/enum) plus `to_dsl`/`from_dsl`
  conversion functions and handcrafted `OpText`/`OpBinary` impls bridging `Din16798Mutation`
  through the mirror — exact shape mirrored from `process3d`'s own already-migrated
  `🧬️mutations/📝️text/🦀️component.rs`.
- `🧬️mutations/💾️binary/🦀️component.rs` — encode/decode wrapper functions unchanged (still pure
  forwards to `OpText`/`OpBinary`), only its two tests updated from `SetSnapshot` to a real
  `ChangeTOpC` sample mutation.
- `🔺️diff/📝️text/🦀️component.rs` — removed the now-dead `diff_set_snapshot` helper (built a
  whole-`Din16798Artifact` replacement `Din16798Diff`, the diff-side half of the banned
  `SetSnapshot`); `apply`/`absorb`/`apply_to_artifact` (already field-sparse, used by every new
  `change-*` diff via the shared `MutationDiff` impl) were untouched — they didn't need to change
  shape, only lose their one caller-that-is-now-gone.

`Din16798Diff` still carries its pre-existing `artifact: Option<Box<Din16798Artifact>>` "whole
replacement" escape hatch and the config-adjacent `selected_check_index` field; neither is written
by any of the 62 new mutations and neither was part of this facet's assigned scope (they predate
this migration and aren't `SetSnapshot`-shaped), so they were left as-is rather than speculatively
removed.

## Tests

Extended the existing `🧪️Tests` region in `🧬️mutations/🦀️component.rs` (no new test files): an
`every_mutation()` fixture with one realistic sample value per field (all 62), a
`round_trip()` helper (`vcs::apply_mutation` forward, reversed `inverse()` backward, asserts
restoration of `base`), `every_variant_registers_an_approved_semantic_descriptor` (iterates all 62,
asserts `kinds().len() == 62`), `every_variant_round_trips_via_inverse` (all 62, not a sampled
subset), and three `protocol::os_spr::testkit::assert_mutation_inverse_law` /
`assert_mutation_diff_absorb_law` pairs on the three most structurally distinct variants per the
recipe's guidance: the repurposed enum-typed slot (`change-annex`), a typical `f64` scalar
(`change-t-op-c`), and a `String` scalar (`change-occupancy`). `protocol::os_spr::testkit` and
`dsl::Mutations` are both reachable with **zero new Cargo dependency** (same finding as
`process3d`'s report: `📦️glue.rs` already declares `extern crate semio_framework_os_kernel as
dsl;`/`as protocol;`, and the kernel crate's own root re-exports lift both). Note: the bare
`protocol::testkit` path used by `process3d`'s report is now **ambiguous** crate-wide (`os_pack`
also re-exports a `testkit` module via the same glob `pub use crate::os_pack::*;` in the framework
kernel's own `📦️glue.rs` — confirmed unrelated to this session, another concurrent one touching
`🧰️framework/**`) — used the unambiguous `protocol::os_spr::testkit::...` path instead, which
resolves cleanly with zero warnings.

`🧬️mutations/📝️text/🦀️component.rs`'s own `🧪️Tests` region got five named `OpText` round-trip
tests (`change-annex`, `change-t-op-c`, `change-occupancy`, `change-persons`,
`change-sfp-required-class` — one per distinct field type: enum, f64, String, u32, u8) plus an
`every_variant_op_text_round_trips` loop test over all 62, replacing the single stale
`set_snapshot_op_text_round_trips` test.

## Verify

`cargo check -p semio-s-plugin-norm --message-format=short`: 91 errors repo-wide, **zero inside
`🗿️artifacts/📗️din16798/`** (one pre-existing, unrelated warning in `🚪️io/🦀️component.rs`, present
before this session touched anything). `cargo check -p semio-s-plugin-norm --tests`: 118 errors
repo-wide; inside this facet's own writable boundary, exactly **5 errors, all in
`🎛️apps/📗️din16798/**`** (outside `🗿️artifacts/📗️din16798/`, expected/sanctioned app-boundary
breakage — see `sharedFileRequests`) and **zero warnings** anywhere in
`🧬️mutations/**`/`🔺️diff/📝️text/**` after fixing one self-inflicted issue found this way (see
below). Every other error in both runs is confirmed **not** caused by this session: `vdi3805`,
`en1999`, `en1997`, `en1998`, `en1996`, `din18599`, `en1991`, `en1995`, `en1994`, `en1993`,
`en1992`, `en1990`, `din4108`, `iso16757` all show the *same family* of `SetSnapshot`/`encode_op`/
`decode_op`-missing errors this session never wrote to — other concurrent sessions mid-migration on
their own sibling `norm` facets in this same wave, per house policy on concurrent workspace churn
(poll/verify scope, don't chase another session's WIP); none of those files were touched.

**Self-inflicted issue found and fixed** (via the `--tests` pass, matching `process3d`'s own
experience that `--tests` catches things a plain `cargo check` doesn't): the dispatch file's law
tests used the bare `protocol::testkit::...` path, which is ambiguous per the note above —
6 warnings, not a compile error, but fixed anyway by switching to `protocol::os_spr::testkit::...`.
Also cleaned three trivial "unnecessary qualification"/"unused import" warnings introduced by the
generator script in `🧬️mutations/📝️text/🦀️component.rs` (a fully-qualified `AnnexChoice` path and
an `OpText` path that duplicated an existing `use`, plus one genuinely-unused `Din16798Snapshot`
import in that file's own test module). Reconfirmed clean on the next run.

`cargoCheck: green` reflects this facet's own writable boundary (`🗿️artifacts/📗️din16798/`):
zero errors, zero warnings in every file this session created or edited. `lawTestsPass` is
reported `false` — **not** because any law failed, but because `cargo test -p semio-s-plugin-norm`
cannot build the crate at all right now (the 5 expected `🎛️apps/📗️din16798/**` errors below block
every test binary, not just this facet's), so the inverse/absorb laws, round-trip laws, and
`OpText` round-trip tests written above have not actually been *executed* yet — only statically
verified via `cargo check --tests` (which type-checks but does not run `#[test]` fns). Per house
policy against claiming an unrun test passes, this is reported honestly rather than assumed from
the code review. Once the `sharedFileRequests` below are applied (by this plugin's reconciliation
pass), `cargo test -p semio-s-plugin-norm` should be able to actually run these tests.

## `sharedFileRequests` — exact changes needed once a later pass can touch shared files

1. **`✏️s/🔌️plugins/📕️norm/🎛️apps/📗️din16798/🦀️component.rs:107`** (`import_media`) — builds
   `Din16798Mutation::SetSnapshot { snapshot }` for whole-document import. This has **no 1:1
   replacement mutation** (banned per taxonomy) — the fix is structural: route through
   `store::ArtifactStore::reset` (or whatever store-facing reset entry point `ArtifactApp::handle`
   exposes for media import), not a `Mutation`.
2. **`✏️s/🔌️plugins/📕️norm/🎛️apps/📗️din16798/🎮️commands/📤️set-snapshot/🦀️component.rs:20,41`** —
   the entire `set-snapshot` app command's `handle()` builds `Din16798Mutation::SetSnapshot {
   snapshot: payload.snapshot.clone() }`; its test at line 41 asserts the same shape. Same
   structural note as #1 — this command's whole reason to exist (client sends a full replacement
   document) is banned as an in-history mutation, so it needs to route through `store::reset`
   too, or be retired in favor of per-field `change-*` commands if the app UI is being redesigned
   away from whole-document push.
3. **`✏️s/🔌️plugins/📕️norm/🎛️apps/📗️din16798/🎮️commands/🧮️evaluate/🦀️component.rs:23,38`** — the
   `evaluate` command currently commits the *entire* input snapshot via `SetSnapshot` even though,
   from the field list, `evaluate` most likely only recomputes derived/check-result state (not
   modeled in `Din16798Snapshot` at all — the checks live in a separately-computed `CheckReport`,
   not a persistent field). Needs inspection: if `evaluate` genuinely mutates zero snapshot fields,
   the fix is to stop emitting any artifact mutation at all (config-only or effect-only `Emit`); if
   it does touch specific fields, decompose into the corresponding `ChangeXxx` mutations, one per
   field, in the same `Emit::mutations(vec![...])`.
4. **`📦️glue.rs`** (this plugin's, not framework's) — cosmetic only, once touchable: rename the
   repurposed `📄set-snapshot` triad directory's path-include block to `🔧change-annex` (behavior
   already correct at every layer — wire tag, `SEMANTICS.kind`, descriptor registration — only the
   on-disk directory name and the `set_snapshot` mod alias are stale).
5. Grammar (`🧬️mutations/📖️component.grammar.semio`, at the `🧬️mutations/` root — not the
   `📝️text/` one, which is accurate and unchanged) was left untouched (recipe step f, explicitly
   "not blocking"): the current file was not inspected against the new vocabulary in this pass;
   listing the 62 `change-*` keywords honestly is a larger, separate task.

## Files touched

Created (62 triads × 3 leaves × 2 files [`.rs` + `.ts` stub] = 372 files, one triad repurposing the
pre-existing `📄set-snapshot` path instead of adding a new directory): `🔧change-occupancy`,
`🔧change-comfort-category`, `🔧change-t-op-c`, `🔧change-rh-percent`, `🔧change-air-speed-ms`,
`🔧change-theta-rm-c`, `🔧change-co2-ppm`, `🔧change-df-percent`, `🔧change-l-aeq-db`,
`🔧change-persons`, `🔧change-ida-class`, `🔧change-ventilation-m3-h`, `🔧change-floor-area-m2`,
`🔧change-bedrooms`, `🔧change-dwelling-ventilation-m3-h`, `🔧change-occupants`,
`🔧change-residential-ventilation-m3-h`, `🔧change-sfp-wm3-s`, `🔧change-sfp-required-class`,
`🔧change-heat-recovery-eta`, `🔧change-heat-recovery-eta-min`, `🔧change-system-type`,
`🔧change-years-since-inspection`, `🔧change-humidification-required-kg-h`,
`🔧change-humidification-provided-kg-h`, `🔧change-fan-qvm3-s`, `🔧change-fan-t-run-h`,
`🔧change-fan-energy-reference-kwh`, `🔧change-night-setback-k`, `🔧change-hr-m-dot-kg-s`,
`🔧change-hr-cp-j-kgk`, `🔧change-hr-delta-tc`, `🔧change-hr-th`,
`🔧change-hr-savings-reference-kwh`, `🔧change-n50-h-inv`, `🔧change-volume-m3`,
`🔧change-infiltration-allowance-m3-h`, `🔧change-cellar-area-m2`,
`🔧change-cellar-ventilation-m3-h`, `🔧change-h-tr-wk`, `🔧change-h-ve-wk`, `🔧change-theta-ec`,
`🔧change-theta-set-c`, `🔧change-cooling-delta-th`, `🔧change-cooling-gains-kwh`,
`🔧change-cooling-utilization-factor`, `🔧change-cooling-reference-kwh`, `🔧change-chiller-type`,
`🔧change-eer-actual`, `🔧change-qc-kwh`, `🔧change-generation-reference-kwh`,
`🔧change-data-center-supply-c`, `🔧change-h-st-wk`, `🔧change-theta-st-c`,
`🔧change-theta-amb-c`, `🔧change-storage-th`, `🔧change-storage-allowance-kwh`,
`🔧change-dhw-delivery-c`, `🔧change-duct-class`, `🔧change-duct-test-pressure-pa`,
`🔧change-duct-leakage-m3-sm2` (each with `🦠️mutation/🦀️component.rs`+`.ts`,
`🔺️diff/🦀️component.rs`+`.ts`, `↩️inverse/🦀️component.rs`+`.ts`).

Rewrote: `🧬️mutations/🦀️component.rs` (dispatch enum + tests), `🧬️mutations/📝️text/🦀️component.rs`
(OpText/OpBinary codec + tests), `🧬️mutations/📄set-snapshot/🦠️mutation/🦀️component.rs`,
`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`,
`🧬️mutations/📄set-snapshot/↩️inverse/🦀️component.rs` (repurposed for `ChangeAnnex`),
`🧬️mutations/💾️binary/🦀️component.rs` (tests only), `🔺️diff/📝️text/🦀️component.rs` (removed dead
`diff_set_snapshot` helper).

Not touched: `📦️glue.rs`, plugin-root `🦀️component.rs`, `🎛️apps/**` (per boundary — see
`sharedFileRequests`), any other plugin or artifact, `🧰️framework/**`, root `📜️script.ts`.
