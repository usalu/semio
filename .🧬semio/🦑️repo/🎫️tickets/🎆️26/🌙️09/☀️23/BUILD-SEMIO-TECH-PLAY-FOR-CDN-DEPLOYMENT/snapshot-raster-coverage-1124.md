# 📦️ Snapshot Raster Coverage Clone (2026-11-24)

Crate: `semio-s-artifact-raster-raster-model` (wasm32-wasip2, release), tree `/Users/ueli/.sinc/play`.
Errors: `E0599 retained_alias_work_demand` / `retained_alias` on `PersistentList<T>` in `🧬️schema/🎯️selection/📦️coverage/🦀️.rs:30,37`.
**Nothing was compiled** (no cargo/rustc run); the release build is the verifier.

## Root cause

`PersistentList::retained_alias_work_demand(&self)` and `PersistentList::retained_alias(&self, effect)` were added to
`🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🦀️.rs` in auto-commit `b4927d2c311` (🚩️751) and removed one commit later
in `01324745f80` (🚩️752). The same 752 hunk also renames `birth_work_demand` → `source_admission_work_demand`, so this looks like a
last-writer-wins overwrite by a peer working from an older copy of the file, not a deliberate API removal. The live repo has the same
framework file (no `retained_alias`), so live raster is broken the same way.

## Current API (snapshot)

`PersistentList<T>` now has no `&self` alias. Its only root-sharing entry points need a lease-backed original:
- `projection_demand() -> RetirementDemand`
- `projection_work_demand(original: RetainedCloneRef<'_, Self>) -> Result<u64, ValueError>`
- `project(original: RetainedCloneRef<'_, Self>, effect) -> RetainedWorkOutcome<Option<(Self, RetainedCloneProgress)>, ValueError>`
- `project_into(..)` (the `ImmutableRootProjection` used by `impl RetainedClone for PersistentList<T>` through `ImmutableRootCloneCursor`)

You can only get a `RetainedCloneRef` from a `RetainedCloneSource` / borrow authority that owns the value. So `Clone::clone(&self)`
cannot call `project`. `#[derive(RetainedClone)]` needs the same lease, so delegating to it does not help either.
Framework free functions `retained_alias*` (pack-json tree) take `RetainedCloneRef` too, so they do not fit here either.

## Sibling pattern used

Structs that are plain `Clone` and hold persistent roots keep them behind `SealedShared`. `SealedShared::clone` adds one counted
alias with no allocation and cannot fail:
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/…/🧬️schema/📸️snapshot/🦀️.rs`: `#[derive(Clone, …)]` with `SealedShared<PersistentList<…>>` fields.
- `✏️s/🔌️plugins/🔋️energy/…/📸️snapshot/🧳️state/🦀️.rs`: `EnergyModelState` `#[derive(Clone, …)]` over `SealedShared<…State>` groups that hold `PersistentList`s.
- `✏️s/🔌️plugins/🖨️raster/…/🧬️schema/🖼️image-payload/🦀️.rs`: `impl Clone` aliases the shared owner handle.

No sibling uses `BlockingTurns` + `RetainedCloneGrant` inside `Clone`. The coverage file was the only one.

## Change

- `RasterSelectionSpans.pages` is now `Option<SealedShared<PersistentList<RasterSpanPage>>>`: `None` for empty, so `Default` still holds without allocating.
  The struct derives `Clone`. The hand-written `Clone` (BlockingTurns/grant/alias) is gone.
- `pages()` now returns `Option<&PersistentList<RasterSpanPage>>`. `get*`, `page_slice*`, `rank_observation_demand` and `iter` read through it, so their signatures are unchanged for callers in the editor, host and select-pixels.
  `RasterSelectionSpanIter.pages` is now `Option<PersistentListIter>`. `PartialEq` uses `same_root` on both present roots.
- `from_pages` is replaced by `admits_pages` (validation before sealing). The builder gains two funded phases after `take_into`:
  `list` → `SealedShared::admit_from(…, SharedIssuer::owned(), …)` → `root` → publish `RasterSelectionSpans` (work 1).
  An empty list publishes `pages: None` right away. Demands, work demands, close demands/work, the `close!` order and `terminal_is_empty` all cover `list` and `root`.
- Tests: `packed_coverage_alias_shares_pages_for_exact_work` is rewritten as `packed_coverage_clone_shares_one_sealed_page_root` (clone equals and shares the same root, page counts from the fixture).
  Builder, codec and receiving tests now read `pages().map_or(0, PersistentList::len)`.

## Files changed

- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️selection/📦️coverage/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️selection/📦️coverage/🧵️builder/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️selection/📦️coverage/🧵️builder/🧪️tests/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️selection/📦️coverage/🔁️codec/🧪️tests/🦀️.rs`
- `/Users/ueli/.sinc/play/✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎯️selection/📦️coverage/🛬️receiving/🧪️tests/🦀️.rs`

## Other callers

Searched `✏️s` and `🧰️framework` for `retained_alias_work_demand` / `retained_alias` used as methods. The only hits were the coverage file and its
receiving test, and both are fixed. The `retained_alias*` names in `🧰️framework/…/🎒️pack/🔤️json` and `📡️replication/…/🔏️seal` are the unrelated free functions.

## Unresolved

- Not compiled. The likeliest compile risks are the bounds of `SharedIssuer::<PersistentList<RasterSpanPage>>::owned()` / `SealedShared::admit_from`
  for a `PersistentList` payload (FEM builds the same type, so they should hold), and glob-import reach in the tests.
- `fixture.json` `alias[].work` (1/4/4) and `alias.test.ts` still model the old funded root-alias work. The Rust side no longer quotes a clone work
  (SealedShared alias costs nothing), so `work` is now only checked by the TypeScript oracle.
- The other fix is to restore the two 13-line methods in the framework `PersistentList` (lost in 🚩️752). That would keep the original
  design (one sealed layer, quoted alias work) and needs no plugin change. It is outside this task's scope (framework is off-limits). If someone
  restores it in live, revert this plugin change to keep snapshot and live aligned.
- Cost: one extra small sealed allocation per non-empty coverage (`SealedShared` around the list, whose pages are already sealed).
