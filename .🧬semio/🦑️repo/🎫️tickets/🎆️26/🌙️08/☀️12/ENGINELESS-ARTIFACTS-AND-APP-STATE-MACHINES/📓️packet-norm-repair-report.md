# Norm Repair Report — Green Again

Scope: `✏️s/🔌️plugins/📕️norm` only. Goal: make `semio-s-plugin-norm` (`--all-targets`) compile
clean again after the `⚙️engine`-dissolution merge-collisions.

## Error-count sequence

| Run | File | lib errors | lib test errors | Note |
|---|---|---|---|---|
| 1–3 | scratch-norm-fix-1..3.txt | — | — | Blocked before reaching norm: `semio-s-plugin-stdio` (a **dependency** of norm) failed with 1–2 `E0432`s in `🗿️artifacts/🧿️semio/…/✳️mesh/…` — another session's live registration-conversion WIP (confirmed via `git status` showing unstaged `M`/`A` churn in that exact subtree). Not ours; norm never got type-checked in these runs. |
| 4 | scratch-norm-fix-4.txt | 39 | 47 | stdio cleared momentarily; first real look at norm's own errors: 16×`E0255`, 5×`E0252` (+1 already fixed pre-run), several `E0432`/`E0425`/`E0422`/`E0659`, 6×`E0599`. |
| 5 | scratch-norm-fix-5.txt | 2 | 8 | After the `Family` self-import, `part_2`, `AnnexChoice`/`MonthlyClimate`/`UseClass`, and `::schema::` fixes. Remaining: 1×`E0432` (`BalancingInputs`), 1×`E0425` (`Diagnostic`), 6×`E0599` (`execute`). |
| 6 | scratch-norm-fix-6.txt | **0** | **0** | Clean. |
| 7 (final) | scratch-norm-fix-7-final.txt | **0** | **0** | `Finished \`dev\` profile [unoptimized] target(s) in 30.33s`, exit 0. |

Two later re-runs (`exit_check.txt`, `exit_check2.txt`) hit **fresh, different** stdio errors
(`E0753`/`E0433`/`E0425` in `stdio`'s `png` artifact — `cannot find engine in png`,
`sniff_real_bytes`) — `git status` shows stdio's `📷️png` artifact mid-dissolution (`⚙️engine`
deletion in progress, unstaged) at that moment. This is the same "other session live in stdio"
situation called out in the task, now hitting a different artifact. Norm's own code was not
touched between run 7 and these re-runs. See "Attribution" below.

## Fixes made (all inside `✏️s/🔌️plugins/📕️norm`)

### E0252/E0255 — duplicate imports / duplicate definitions (dedup, kept the destination copy)

1. **`🗿️artifacts/📘️en1996/…/🧬️schema/🦀️component.rs`** (the ticket's confirmed example):
   - L282 `use serde::{Deserialize, Serialize};` duplicated L7 → removed.
   - L280 `use crate::artifacts::en1996::MasonryClass;` duplicated L3's `{MasonryClass, part_2}` → removed.
   - L281 `use crate::document::{AnnexChoice, CheckReport, CheckResult, CheckStatus, Quantity, ClauseId};` partially overlapped L4's `{AnnexChoice, DesignSituation}` (`AnnexChoice` dup, rest new) → merged into L4: `use crate::document::{AnnexChoice, DesignSituation, CheckReport, CheckResult, CheckStatus, Quantity, ClauseId};`, removed L281.
   - Separately, L3's `use crate::artifacts::en1996::{MasonryClass, part_2};` collided (**E0255**) with this same file's own `pub mod part_2 { … }` (compliance-check functions, relocated from the engine) — the *type* `part_2` (from the app-root file) is never referenced unqualified in this file (always `crate::artifacts::en1996::part_2::X`), so the import was narrowed to `use crate::artifacts::en1996::MasonryClass;`, leaving the local `pub mod part_2` as the sole `part_2` in scope (used unqualified at L534/540/546/548 in the test module).

2. **`🗿️artifacts/📘️en1992/…/🧬️schema/🦀️component.rs`**: relocated block re-imported `FireRating`/`TightnessClass` already imported at the top of the file → removed the two duplicate lines, kept the new `crate::document::{…}` line.

3. **`🗿️artifacts/📓️iso16757/…/🧬️schema/🦀️component.rs`**: relocated block re-imported `CatalogueValue` already imported at top → removed the duplicate line.

4. **`🗿️artifacts/📘️en1991/…/🧬️schema/🦀️component.rs`**: relocated block re-imported `FireCurve` already imported at top → removed the duplicate line.

5. **`🗿️artifacts/📘️en1995/…/🧬️schema/🦀️component.rs`**, **`📘️en1997`**, **`📘️en1999`** (schema/component.rs): relocated `use crate::document::{AnnexChoice, …}` each re-imported `AnnexChoice` already imported at the top of file (`use crate::document::AnnexChoice;`) → dropped `AnnexChoice` from the relocated line in each.

6. **`🗿️artifacts/📙️din18599/…/🧬️schema/🦀️component.rs`**: relocated `use crate::artifacts::din18599::{BalancingInputs, MonthlyClimate, UseClass};` re-imported `MonthlyClimate`/`UseClass` already imported at the top (`{MonthlyClimate, UseClass}`) → narrowed to `use crate::artifacts::din18599::BalancingInputs;`.

7. **All 15 `🎛️apps/<slug>/🦀️component.rs`** (`din4108`, `din16798`, `din18599`, `en1990`–`en1999`, `iso16757`, `vdi3805`): each had a **self-import** `use crate::apps::<slug>::<Slug>Family;` at L13 that collided (**E0255**) with the file's own `pub struct <Slug>Family;` defined later in the same file. Removed the self-import in each of the 15 files (verified each struct definition was present in-file before removing).

### E0432/E0433/E0425/E0422/E0659 — path repointing (verified each destination on disk / via compiler suggestion)

8. **`🗿️artifacts/📙️din18599/…/🧬️schema/💡️inferences/🦀️component.rs`**: `BalancingInputs` (a type alias) was imported from `…::schema::{…, BalancingInputs}` but it does not live in the `schema` submodule — it's re-exported at the artifact root. Split into two `use`s: kept the `schema::{part_1..part_12}` list, added `use crate::artifacts::din18599::BalancingInputs;` (rustc's own suggested re-export path).

9. **`🗿️artifacts/📔️vdi3805/…/🧬️schema/🦀️component.rs` + `…/💡️inferences/🦀️component.rs`**: `use schema::ArtifactSchema;` and every bare `schema::…` path (`ArtifactSchemaDescriptor`, `FacetLeaves`, `ArtifactInferenceDescriptor`) were **ambiguous/mis-resolved** (`E0659`/`E0425`/`E0422`). Root cause: the relocated `ComplianceHelpers`/inference block adds `use crate::artifacts::vdi3805::*;`, a wildcard glob that (uniquely to vdi3805) also pulls in `crate::artifacts::vdi3805::schema` — i.e. *this file's own enclosing module* — which collides with the crate-root `extern crate semio_framework_schema as schema;` (declared in `📦️glue.rs`). For `use` declarations this is a genuine ambiguity error (`E0659`); for bare expression-position paths elsewhere in the same file it silently resolves to the wrong (local, self-referential) `schema`, giving "not found in schema" (`E0425`/`E0422`). Fixed by prefixing every such reference with a leading `::` (`::schema::ArtifactSchema`, `::schema::ArtifactSchemaDescriptor`, `::schema::FacetLeaves`, `::schema::ArtifactInferenceDescriptor`) — `::name` forces crate-root/extern-prelude resolution, bypassing the glob entirely. Did **not** touch the wildcard import itself or any `crate::…::schema::…`-qualified path (those correctly mean the local submodule and were left alone).

10. **`🗿️artifacts/📔️vdi3805/…/🧬️schema/🦀️component.rs`**: the same wildcard (`use crate::artifacts::vdi3805::*;`) also pulls in `crate::artifacts::vdi3805::dsl` — a real, intentionally-mounted submodule (`📦️glue.rs` L619: `pub mod dsl { pub use …snapshot::text::*; }`, the native-text grammar constants) — which shadows the crate-root `extern crate semio_framework_os_kernel as dsl;` for **unqualified** `dsl::…` references at this file's top level. This broke `derive_artifact_facets!`'s macro-generated `Vec<dsl::Diagnostic>` (the macro is defined in `semio_framework_plugin` and is not `$crate`-hygienic for that path, so it resolves against the *call site's* scope) — reported as `E0425: cannot find type Diagnostic in module dsl`, because `dsl` silently resolved to the local grammar submodule instead of the extern crate. Since the macro body isn't ours to edit, added an explicit `use ::dsl;` in the same scope: an explicit single-item import always wins over a same-named glob-imported item (no ambiguity, unlike the `schema`/extern-prelude case), restoring `dsl` to the crate alias for this scope. Left a docstring explaining why, since this is non-obvious.

11. **`🗿️artifacts/📔️vdi3805/…/🧬️schema/🦀️component.rs`**: `const ANNEX: AnnexChoice = …;` was private but needed cross-module by `…/💡️inferences/🦀️component.rs`'s `evaluate`-path (production code, not test) → made it `pub const ANNEX` (matching the existing `pub fn clause` in the same block) and added `ANNEX` to the sibling file's existing `use …schema::{clause, …}` import list.

12. **`🗿️artifacts/📘️en1997/…/🧬️schema/💡️inferences/🦀️component.rs`**: `AnnexChoice` (struct field type, L110) and `CheckStatus` (test assertion) were unresolved — both used but never imported. The file already has a top-level `use crate::document::CheckReport;` in the same "ComplianceReport" relocated block; extended it to `use crate::document::{AnnexChoice, CheckReport, CheckStatus};` (compiler's own suggested import, applied at the pre-existing import site rather than inline per rustc's raw suggestion, to keep one import site per module as elsewhere in the file).

### E0599 — missing trait import

13. **`🗿️artifacts/📓️iso16757/…/🧬️schema/🦀️component.rs`**: `mod compliance_helpers_tests { … }` calls `runtime.execute(…)` on `DefaultScriptRuntime` five times (L759, L1139, L1147, L1151, L1154) without the `ScriptRuntime` trait (which declares `execute`) in scope. Added `use crate::artifacts::iso16757::standards::v1::subsets::any::schema::component::part_5::ScriptRuntime;` to the test module's imports — this is rustc's own suggested path, and matches how the file is mounted in `📦️glue.rs` (`mod component;` under `…schema::`, confirmed by reading the `#[path]` wiring directly rather than inferring it).

## Attribution

- **Ours (fixed):** all 13 items above, entirely inside `✏️s/🔌️plugins/📕️norm`.
- **Not ours (c) — upstream/another session, left untouched:**
  - `semio-s-plugin-stdio`'s `🗿️artifacts/🧿️semio/…/✳️mesh/…` — `E0432` on `demo_mesh_snapshot`/`geometry`/`triples` (runs 1–3). `git status` showed unstaged `M`/`A` activity throughout that exact subtree (mesh schema/inferences/snapshot/diff), consistent with a live registration-conversion in progress.
  - `semio-s-plugin-stdio`'s `🗿️artifacts/📷️png/…` — `E0753`/`E0433`/`E0425` (`cannot find engine in png`, `sniff_real_bytes`), seen in two post-fix re-runs. `git status` shows `📷️png/…/⚙️engine/🦀️component.rs` mid-deletion (unstaged ` D`) plus modified `🚪️io`/`🧬️schema`/`🧬️mutations` files at the moment of the failing run — another session dissolving stdio's own `png` engine live, the same pattern norm went through earlier tonight, now happening one artifact over. **Not norm's regression; norm's own files were unchanged between the clean run and these.**

No norm-side line was left unfixed that traced back through a deleted `⚙️engine` path — every `E0433`/`E0432`/`E0425`/`E0422` inside norm resolved to one of the 13 fixes above; no remaining norm-side error requires cross-artifact guessing (the `en1990`-only `standards::v1::subsets::any::engine` nesting difference the task warned about didn't surface as a live error — no norm file referenced it incorrectly by the time I reached it).

## Final verbatim result

See `scratch-norm-fix-7-final.txt` for the full log. Tail:

```
warning: `semio-s-plugin-norm` (lib) generated 237 warnings (run `cargo fix --lib -p semio-s-plugin-norm` to apply 207 suggestions)
warning: `semio-s-plugin-norm` (lib test) generated 278 warnings (234 duplicates) (run `cargo fix --lib -p semio-s-plugin-norm --tests` to apply 26 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 30.33s
```

`find ✏️s/🔌️plugins/📕️norm -path "*🗿️artifacts*" -name "⚙️engine" -type d` → 0 results (confirmed both before and after — dissolution was already structurally complete; no `⚙️engine` directory was restored or left behind).

Dangling `#[path]` check in `📕️norm/📦️packages/🦀️rust/📦️glue.rs`: every `#[path = "…"]` target resolved to an existing file on disk → 0 dangling.

**Caveat on repeatability:** because `semio-s-plugin-stdio` is a dependency of norm and is being live-edited by another session tonight, a bare re-run of the definition-of-done command can transiently fail on *stdio's* code (not norm's) depending on timing. Run 7 (`scratch-norm-fix-7-final.txt`) is the authoritative clean evidence — `Finished` line present, all-targets, 0 errors. No norm file changed after that run.
