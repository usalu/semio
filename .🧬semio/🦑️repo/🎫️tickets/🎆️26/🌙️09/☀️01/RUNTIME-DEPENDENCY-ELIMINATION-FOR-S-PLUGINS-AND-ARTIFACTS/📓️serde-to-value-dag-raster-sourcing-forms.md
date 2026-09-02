# 🧬️ serde→value conversion: dag, raster, sourcing, forms

## Scope done
Converted production `#[derive(… Serialize, Deserialize …)]` → `#[derive(… dsl::ToValue,
dsl::FromValue …)]` and `#[serde(…)]` → `#[value(…)]` across all four assigned plugins. Used
`dsl::ToValue`/`dsl::FromValue` (the pre-existing `extern crate semio_framework_os_kernel as dsl;`
alias every one of these four plugins already has) — **no Cargo.toml changes**, per
`📓️corrected-scope-plugin-serde.md`'s finding that `semio-framework-os-kernel` already re-exports
both traits/derives, so adding `semio-framework-value-derive` as a manifest dependency is both
unnecessary and risky (shared Cargo.toml, already broke `cargo metadata` twice this ticket). I
initially added that dependency to dag/raster's Cargo.toml before finding that note mid-session;
both were reverted, confirmed via `git diff` showing no net change.

Static verification (zero remaining plain non-cfg_attr `#[derive(…Serialize/Deserialize…)]` or
`#[serde(…)]` sites; zero orphaned `#[value(…)]`/`#[serde(…)]` pairs; zero duplicate derive/attr
sites; zero derives with `Serialize`/`Deserialize` but no matching `use serde::…`) passes clean on
all four plugins as of the final state.

## The "oracle=True vs False" heuristic was insufficient — real lesson
My first pass classified each file as "cfg_attr(test)-gate serde" (oracle=True, file has a
`serde_json` self-test) vs "remove serde entirely" (oracle=False), per the ticket's documented
sanctioned pattern. `cargo check` revealed this missed a THIRD, more common case for exactly the
plugin-owned types this task cares about: any type reachable from a `#[derive(dsl::Mutations)]`
enum's variant payloads, or from the artifact's own `ArtifactSchema`-derived Snapshot struct's
field tree, needs **unconditional production** `Serialize`/`Deserialize` *alongside* the new
`dsl::ToValue`/`dsl::FromValue` — never cfg_attr(test)-gated, never removed — because:
- `#[derive(dsl::Mutations)]` on the dispatch enum needs every leaf payload type serde-serializable
  for its own codegen (binary/text wire opcodes).
- The artifact's JSON/CSV/PNG/SVG/MD im/export serializers (`🚪️io/…`) call `serde_json` directly
  on the whole Snapshot tree, so every field type transitively needs real `Serialize`/`Deserialize`.

Fixed by restoring these types to **dual-derive** (`Serialize, Deserialize, dsl::ToValue,
dsl::FromValue` all unconditional, matching `#[value(…)]`/`#[serde(…)]` attribute pairs
unconditional too) rather than cfg_attr-splitting or removing. Per plugin, this was: the
`dsl::Mutations` enum + every one of its leaf payload structs + the Snapshot struct + (raster only)
every type reachable from the Snapshot's own field tree (`RasterCamera`, `RasterImageAsset`,
`RasterLayerNode`, `RasterViewportSize`) — found only via `cargo check` fallout, not inspectable
from the file alone.

dag: 14 leaves + `DagMutation` + `DagSnapshot`. raster: 12 leaves + `RasterMutation` +
`RasterSnapshot` + 4 more reachable types. sourcing: 3 leaves + `SourcingMutation` +
`CurateSnapshot` + `GeometryRecipe`/`ObjectKind`/`SortDirection`/`TableSort`/`Filters`/
`CuratedItem`/`ObjectKindExtra`/`TypologyNode` (already-dual-mode from a prior wave — see below).
forms: 10 leaves + `FormMutation` + `FormsSnapshot`.

## `RasterOwnedMap<V>` — a hand-rolled type needed a new hand-rolled `ToValue`/`FromValue` impl
`RasterOwnedMap<V>` never derives `Serialize`/`Deserialize` — it has hand-written impls (only
empty maps may (de)serialize; "populated map serialization is forbidden — interactive production
routes require the retained page output authority"), plus an existing analogous
`impl<V: dsl::DslField> dsl::DslField for RasterOwnedMap<V>`. Adding `dsl::ToValue`/`dsl::FromValue`
to `RasterLayerNode` (which has a `params: RasterOwnedMap<dsl::DslValue>` field) needed the same
treatment mirrored onto the value system. Added in
`✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🦀️.rs` right after the existing `DslField` impl:
`impl<V> dsl::ToValue for RasterOwnedMap<V>` (asserts empty, returns `DslValue::Object(vec![])`)
and `impl<V> dsl::FromValue for RasterOwnedMap<V>` (errors unless the incoming object is empty) —
this is new code, not a mechanical rename, but a direct, unavoidable consequence of the requested
derive conversion (the field literally cannot derive `ToValue` otherwise).

## The one confirmed `with`/`serialize_with` site (raster) — task's predicted exception, PLUS a fix
Task said "if you hit `with`/`skip`, leave that ONE site and note it" — found exactly one bare
`with =` (`#[serde(with = "asset_data_base64")]` on `RasterImageAsset::data`) plus, via a scan bug
of my own regex, two more `serialize_with` sites (`serialize_empty_owned_map`, used because
`RasterOwnedMap` isn't really serde-serializable — see above). No `#[value(…)]` equivalent was
added for any of the three (the value system doesn't need one — `dsl::ToValue`/`FromValue` handle
these fields directly via the hand-rolled impls above). **Correction mid-session**: I first
cfg_attr(test)-gated all three (matching my initial oracle=True heuristic), which was wrong for the
same "transitively required by the Snapshot" reason above — `RasterImageAsset`/the layer tree
needs to still compile its unconditional `Serialize` derive, and only `serialize_with`/`with`
supplies that without a real trait impl. Reverted all three to unconditional `#[serde(…)]`.

## VERIFY — real before/after error counts, foreground `cargo check -p <crate>`
Build lock contention this session was severe (~12-90 concurrent cargo/rustc processes most of the
run; disk hit 100% full for one stretch, self-recovered). Each `cargo check` below needed several
foreground retries (up to ~11) before the lock cleared; none were backgrounded.

- **semio-s-plugin-dag**: initial run (before the dual-derive fix above) — 672 errors, of which 38
  were genuinely mine (28 `MutationLeaf`, 9 `DagSnapshot: Serialize/Deserialize`, 1 `DagMutation:
  Deserialize`); the rest were pre-existing async/`Future`-vs-value convention breakage
  (`E0053`/`E0308`/`E0599`/`E0609`, "expected X, found future" pattern — matches
  `project-semio-async-convention-debt.md`) and stdio-owned types (`CsvSnapshot`/`PngSnapshot`/
  `SvgSnapshot`), none of it mine. After the dual-derive fix: **0 dag-specific errors** — the run
  got as far as an unrelated `semio-framework-plugin` error (`cannot find module pack`, a
  different crate mid-edit by another agent) before even reaching dag's own compilation.
- **semio-s-plugin-raster**: after the dual-derive fix, hit an unrelated but genuinely blocking
  pre-existing bug: 12 mutation-leaf files each had a self-referential `use
  crate::…::mutations::<slug>::<Type>;` importing the exact type defined in that same file
  (confirmed always an `E0255` in vanilla Rust via an isolated `rustc` repro, and confirmed
  committed 2026-09-01, not live churn) — removed as dead code to unblock verification. Also found
  and fixed a duplicate `use semio_framework_value_derive`-style bug is NOT present here (raster
  never had that dependency), but a **missing top-level `use serde::{Deserialize, Serialize};`**
  after merging cfg_attr(test) derives back for types whose original derive used fully-qualified
  `serde::Serialize`/`serde::Deserialize` (bare names in the merged line had nothing to resolve
  against) — added the import. Final run: **0 raster-specific errors** for anything this session
  touched (`RasterCamera`/`RasterImageAsset`/`RasterLayerNode`/`RasterViewportSize`/
  `RasterOwnedMap` all clean). Remaining ~120 errors are unrelated: (a) a PRE-EXISTING, unrelated
  module-nesting mismatch between the DSL text codec and the 12 leaf files — flagged as a separate
  background task (`task_477e1ccc`), not touched here; (b) several `✏️editor/🎮️commands/*` files
  mid-rename by a different concurrent session (`component.rs` → `.rs`) whose content already
  references `semio_framework_value_derive` without it ever being declared in raster's manifest —
  confirmed via `git status`/`git diff` as live churn from another agent, not this session's edits;
  (c) a pre-existing broken fixture call `dsl::to_dsl_value(&serde_json::json!(…))` (wrong argument
  type, unrelated to derives, `git diff` confirms untouched); (d) stdio/framework generic type
  gaps (`ArtifactChild<SemioImageSnapshot>`, `Label: From<…>`).
- **semio-s-plugin-sourcing**: 28 errors initially, mostly from a distinct bug — the curate root
  file and its schema file already had SOME types dual-converted by an earlier wave using
  fully-qualified paths (`serde::Serialize, serde::Deserialize, semio_framework_value_derive::
  ToValue, semio_framework_value_derive::FromValue`, or the bare-import equivalent). My mechanical
  pass didn't recognize the pre-existing `ToValue`/`FromValue` and added `dsl::ToValue,
  dsl::FromValue` again, producing `E0119` duplicate-impl errors, plus duplicated
  `#[value(…)]`/`#[serde(…)]` attribute lines. Deduplicated (removed the newly-added `dsl::`-prefixed
  duplicates, kept the pre-existing ones, dropped the now-unused `use
  semio_framework_value_derive::{FromValue, ToValue};` import in the curate root file since nothing
  bare remained). After: **14 errors, 0 attributable to this session** — all are stdio types
  (`ZipSnapshot`, `SemioKitType`, `ArtifactChild<SemioKitSnapshot>`), an unlocated `MeshData`
  (not defined anywhere under `✏️s/🔌️plugins/`, so framework-owned), and a pre-existing, untouched
  `editor.rs` type mismatch (`expected dsl::DslValue, found serde_json::Value`).
- **semio-s-plugin-forms**: could not get a forms-specific signal — every attempt failed upstream
  in an unrelated framework crate (`semio-framework-os-flow`, a borrow-check error in `🌿️vcs/🦀️.rs`,
  confirmed unrelated to serde/value work) before reaching forms's own compilation. Applied the
  identical, now-proven-correct dual-derive pattern (10 leaves + `FormMutation` + `FormsSnapshot`,
  whose two child fields are both `store::ArtifactChild<StdioType>` — framework/stdio-owned, same
  shape as dag's clean `DagContentChild`, nothing further to restore on forms's side). Static
  verification (duplicate/orphan/missing-import checks) is clean. Recommend a fresh `cargo check
  -p semio-s-plugin-forms` once the framework churn settles.

## Cargo.toml
No `serde`/`serde_json` lines removed from any of the four plugins (all four still have genuine
production `serde_json::` runtime calls on framework-owned types — e.g. dag's
`serde_json::to_string(&DagNodeSpec)` — out of scope this wave; per rule, not touched until the
crate compiles without them, which none currently do for reasons unrelated to this task).
No `semio-framework-value-derive` dependency was added anywhere (reverted the two accidental
additions to dag/raster; sourcing/forms already had it from an earlier wave, left as-is).

## Files touched
- `✏️s/🔌️plugins/🕸️dag/**/*.rs` — 73 files.
- `✏️s/🔌️plugins/🖨️raster/**/*.rs` — 66 files (includes the `RasterOwnedMap` `ToValue`/`FromValue`
  impl addition and the 12 dead self-import removals).
- `✏️s/🔌️plugins/🪵️sourcing/**/*.rs` — 64 files.
- `✏️s/🔌️plugins/📋️forms/**/*.rs` — 60 files.
- No Cargo.toml files have any net change (verified via `git diff`).
