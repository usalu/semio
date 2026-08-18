# 📓️ W4 fan-out report — `🌿️vcs` plugin

Agent: W4 fan-out, `🌿️vcs`. Boundary (only writer): `✏️s/🔌️plugins/🌿️vcs/**`.

STATUS: source cutover complete. `cargo check -p semio-s-plugin-vcs --all-targets` → **0 errors**
(confirmed after a transient, unrelated peer blocker on the repo-root `Cargo.toml` workspace member
list cleared — see `## verification`). `nextest`/`wasm32-wasip2` runs were launched and are queued
behind the shared `CARGO_TARGET_DIR` lock — `ps aux` at time of writing shows **30+ concurrent cargo
processes** from sibling W4 fan-out agents (puzzle/imperative/dag/sourcing/animate/layout/
reasoning-mindmap/flow/draw/writer/forms/cad/stdio/…) all contending for the same build-directory
lock this whole wave shares. Not a vcs-attributable slowdown. Re-run the two commands below once this
clears; this file will be updated with the real numbers the moment they return.

## Starting condition (measured, before any edit)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/🎯️target"
cargo check -p semio-s-plugin-vcs --all-targets --keep-going 2>&1 | grep -cE '^error'
```
→ **3 errors** (the `--all-targets` run's summary line reads "error: could not compile
`semio-s-plugin-vcs` (lib test) due to **3** previous errors" — authoritative count), all pre-existing,
all in files this agent owns:
- `👁️viewer/🦀️component.rs:102`: `semio_framework::AppRole::Viewer` — `semio_framework` is not a
  dependency of this crate (no such extern crate anywhere in `📦️glue.rs`); the correct path is
  `semio_framework_plugin::AppRole`.
- `🧬️schema/🧬️mutations/🦀️component.rs:43-44` (test `vcs_demo_mutation_round_trips_store`):
  `store::ArtifactStore::<VcsSnapshot, VcsDemoMutation>::new(...)` used bare — the constructor
  returns `Result<Self, VcsError>` and was called without `.expect(...)`.

`git log --date=iso -1` / `git status --porcelain` on both broken files showed no peer edits (clean
porcelain, last real commit `1d71198c19` 2026-08-17 14:44, before this ticket's start commit
`101a6b4ea8` 15:59:36) — pre-existing rot, matching the pattern the sequence fan-out (`📓️w4-sequence-report.md`)
documented for its own crate.

`cargo nextest run` could not run at all against this baseline (compile failure) — no baseline test
count exists.

## What was fixed / built

### Pre-existing compile errors (both in files I own)
- `👁️viewer/🦀️component.rs`: `semio_framework::AppRole::Viewer` → `semio_framework_plugin::AppRole::Viewer`.
- `🧬️schema/🧬️mutations/🦀️component.rs`: `ArtifactStore::new(...)` → `.expect("valid artifact store fixture")`
  (matches the identical `SequenceStore::new(...)` fix in `📓️w4-sequence-report.md`).

### Declaration tree (atomic cutover, design.md §1/§2)
- **Subset root** (new file): `🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` — `pub fn subset() ->
  SubsetDeclaration`, assembling `schema`/`io`/`viewer`/`editor`/`examples`. `editor`/`viewer` read via
  `crate::editor::vcs`/`crate::viewer::vcs` (top-level, per recipe §5 gotcha 1 — pre-existing structural
  fact, unchanged).
- **Standard root** (new file): `🏅️standards/🔖️1/🦀️component.rs` — `pub fn standard() ->
  StandardDeclaration`, mounts subset `any`. `mimes`/`extensions`: **no real MIME registration exists
  anywhere for this artifact** (same finding as sequence's own fan-out) — the old `ArtifactCapability`
  channel only ever claimed a codec id (`vcs.vcs`) and an extension (`vcs`), never a mime type. `mimes:
  ["application/vnd.semio.vcs+json"]` is a documented synthesis (see `## openQuestions`), `extensions:
  ["vcs"]` is the real, carried-over value.
- **Artifact root** (edited): `🗿️artifacts/🌿️vcs/🦀️component.rs` — added `pub fn artifact() ->
  app::declarations::ArtifactDeclaration` (kind `s.vcs.vcs`, one standard). Deleted the OLD `pub fn
  declaration()` (the `ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...)
  .document_codec(...)` chain) outright — no dual channel. `definition()` (old `ArtifactDefinition`/
  capability rows) is **kept**, per debt D1 — not deleted repo-wide until W6, zero callers left for it
  after this pass (harmless).
- **Plugin root** (edited): `🦀️component.rs` — `.declare_artifact(crate::artifacts::vcs::artifact())`
  replaces `.artifact(declaration())` + `.editor::<VcsPlayApp>(...)` + `.viewer::<VcsViewer>(...)` in the
  same edit (atomic — old channel never registered alongside the new one). `.editor_mutation_roster()`/
  `.viewer_mutation_roster()` are **kept** (same orthogonal-opt-in reasoning as `📓️w4-sequence-report.md`).

### Old hand-rolled machinery deleted
- `🚪️io/🦀️component.rs`: deleted `derived_composition::VcsComposerComposition` (`ArtifactComposition`
  impl) and `io_registry` (`ComposerEntry` aggregation) outright. Replaced with `pub fn io() -> IoDeclaration`.
- `🧬️schema/🦀️component.rs` (subset schema root): deleted `derived_construction::VcsBuilderConstruction`,
  `derived_analysis::VcsAnalyzerAnalysis`, and the `derive_artifact_facets!` call (`VcsBuilder`/
  `VcsAnalyzer`/`VcsComposer`). Replaced with `pub type Construction =
  semio_framework_plugin::app::SnapshotBuilder<VcsSnapshot, VcsDemoMutation>;` — trivial-subset shape,
  no custom analysis/composition logic this subset needs beyond the ordinary `Mutation`/`MutationDiff`
  algebra.
- `🧬️schema/💡️inferences/🦀️component.rs`: `ArtifactInferrer` impl retargeted off the deleted
  `VcsBuilder` onto a new local zero-sized marker `VcsInferrer` — `SnapshotBuilder` (the recipe's literal
  suggestion) does NOT work here either, same orphan-rule violation (E0117) the sequence pilot found and
  documented; see `## recipeGaps`.
- Deleted (unmounted, forbidden-per-design.md-§1 plugin-level shapes, confirmed zero `#[path]` references
  in `📦️glue.rs` before deletion): `🎟️capabilities/`, `🔧️setup/`, `🛂️manifest/` — each was a stub
  doc-comment-only file, never compiled.
- Zero-reference grep confirmed repo-wide (outside this plugin) before deleting: `VcsBuilder`,
  `VcsAnalyzer`, `VcsComposer`, `VcsComposerComposition`, `VcsBuilderConstruction`, `VcsAnalyzerAnalysis`,
  `VcsBuilderFacets`.

### Foreign io leaves rewritten as typed `Serializer`/`Deserializer` (design.md §3)
All 10 leaves under `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{json,csv,xlsx,zip,txt}/…` and
`🚪️io/📤️export/🧵️serializers/🗿️artifacts/{json,csv,xlsx,zip,txt}/…` rewritten from hand-rolled
`serialize`/`serialize_text`/`deserialize`/`deserialize_text`/`deserialize_bytes` free functions into
real `impl Deserializer<VcsSnapshot>` / `impl Serializer<VcsSnapshot>` marker-struct impls
(`JsonIntoVcs`/`VcsIntoJson`, `CsvIntoVcs`/`VcsIntoCsv`, `XlsxIntoVcs`/`VcsIntoXlsx`,
`ZipIntoVcs`/`VcsIntoZip`, `TxtIntoVcs`/`VcsIntoTxt`), each declaring `FROM`/`INTO` + `IoFidelity`:
- `Json = Exact` — a direct `serde_json` field-for-field round trip (mirrors the sequence fan-out's own
  json leaf exactly, same `IoPayload::Binary(pretty-printed json bytes)` shape).
- `Csv`/`Xlsx`/`Zip = Lossy` — pre-migration behavior preserved verbatim (a bare `serde_json` struct
  coercion from `VcsSnapshot`'s JSON shape into `CsvSnapshot`/`XlsxSnapshot`/`ZipSnapshot`'s own,
  unrelated shape). Only the `schema` field name is shared, so only `schema` survives; everything else
  is dropped/defaulted on export, and import from a real `.csv`/`.xlsx`/`.zip` file will generally fail
  to deserialize into `VcsSnapshot` at all (its required, non-`#[serde(default)]` fields `title`/
  `counter`/`notes`/`status` have no CSV/XLSX/ZIP-side counterpart). **This is not a regression** — the
  pre-migration hand-rolled `serialize`/`deserialize` pair did the exact same bare coercion; redesigning
  a real column/row mapping is a domain decision out of this cutover's scope (flagged in
  `## openQuestions`).
- `Txt` = honest not-yet-implemented stub (unchanged behavior), `Lossy`.

Registered via `serializer_entry`/`deserializer_entry` in the new `io()`:
```rust
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    // entries: 5 serializer_entry + 5 deserializer_entry calls, keyed on VCS_DIALECT
    // native.codec: store::ArtifactCodec::of::<VcsSnapshot, VcsDemoMutation>(...)
}
```

### Native codec relocation (design.md §1 CORRECTION — unsplit, `🧬️schema` keeps only types)
Physically `mv`'d (plain `mv`, never git) all four facets' `{📝️text,💾️binary}` children from
`🧬️schema/` to `🚪️io/`:
- `📸️snapshot/{📝️text,💾️binary}` — the ONE facet with a real relocation to do: `impl
  store::ArtifactDsl for VcsSnapshot` moved into `🚪️io/📸️snapshot/📝️text/🦀️component.rs`; `impl
  store::ArtifactPack for VcsSnapshot` moved into `🚪️io/📸️snapshot/💾️binary/🦀️component.rs` (VcsSnapshot
  derives `dsl::DslRecord`, so both impls call the derive-generated `Self::__dsl_spec()`/
  `__dsl_from_record`/`__dsl_to_record` inherent methods — these resolve identically regardless of the
  impl block's physical file, confirmed by the sequence pilot's identical relocation).
  `🧬️schema/📸️snapshot/🦀️component.rs` now holds only the `VcsSnapshot` struct + `Default` — types only,
  per design.md rule.
- `🔺️diff/{📝️text,💾️binary}`, `🧬️mutations/{📝️text,💾️binary}`, `💡️inferences/{📝️text,💾️binary}` —
  moved wholesale (grammar/protocol assets + the code they already carried; `mutations`'s real
  `OpText`/`OpBinary` impls for `VcsDemoMutation` and `diff`'s `MutationDiff<VcsSnapshot> for VcsDiff`
  impl were **already** correctly scoped to these facets, not the schema root — only their physical
  directory moved, no content surgery). Confirmed zero cross-file references to the old
  `schema::{diff,mutations,inferences}::{text,binary}` module paths anywhere in the crate except
  `📦️glue.rs`'s own mount block and the 4 "Shims" lines (both updated).
- `📦️glue.rs`: removed the 4 facets' `{text,binary}` sub-mounts from under `schema::{...}`; added
  equivalent mounts under a new `io::{snapshot,diff,mutations,inferences}::{text,binary}` block (right
  after `io`'s own component mount, before `import`/`export`). Updated the 4 "Shims" lines that pointed
  at the moved paths (`op`/`dsl`/`spr`/`pack`/`snapshot::pack`/`diff::text`) to the new `io::` targets.
- Added `#[path]` mounts for the previously-unmounted **subset root** (`subsets::any::component`) and
  **standard root** (`standards::v1::component`) in `📦️glue.rs` — these files did not exist on disk
  before this pass and were never `mod`-declared; the crate would not compile `standard()`/`subset()`
  otherwise (recipeGap #2 from the sequence report, avoided here by writing both mounts up front).

### Cargo.toml
- Added `semio-framework = { path = "../../../../../🧰️framework/📦️packages/🦀️rust", package =
  "semio-framework" }` — required for `semio_framework::io::io_mechanism::{Serializer, Deserializer,
  serializer_entry, deserializer_entry, IoEntry}` and `semio_framework::io_schema::{Dialect, IoError,
  IoFidelity, IoOutcome, IoPayload, IoResult}`. Was previously absent (the crate only depended on
  `semio-framework-os-kernel`/`semio-framework-plugin`/`semio-framework-schema`).

### TS mirrors
- `🚪️io/🟦️component.ts`: real `IoEntryDescriptorMirror[]` (10 entries, mirroring the Rust `io()`
  exactly — fidelity values match), replacing the `export {}`-style `register(): void {}` stub.
- `📦️packages/🟦️typescript/📦️index.ts`: fixed **pre-existing** stale exports (`vcs_snapshot`/
  `vcs_snapshot_text`/`vcs_snapshot_binary`/`vcs_diff`/`vcs_diff_text`/`vcs_diff_binary`/
  `vcs_mutations`/`vcs_mutations_text`/`vcs_mutations_binary` pointing at a flat artifact-level
  `🗿️artifacts/🌿️vcs/🧬️schema/…` target that has not existed since this artifact adopted the
  standard/subset tree, and `vcs_decomposer` pointing at a `🪓️decomposer` directory that does not exist
  anywhere in the current tree) to the real standard/subset-scoped `vcs_schema`/`vcs_io` paths (mirrors
  the sequence fan-out's identical finding/fix).

## verification

All commands from `/Users/ueli/Documents/semio`,
`CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/🎯️target"`.

- `cargo check -p semio-s-plugin-vcs --lib --keep-going` → **initially BLOCKED**, not a vcs-attributable
  error: `error: failed to load manifest for workspace member .../✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust
  ... failed to read .../✏️s/🔌️plugins/🖍️draw/🔄️fsm/📦️packages/🦀️rust/Cargo.toml: No such file or
  directory`. `git status --porcelain -- ✏️s/🔌️plugins/🖍️draw/` showed 84-94 uncommitted `R`/`A`/`D`
  entries (renames/adds/deletes) while blocked — a live peer session's own W4 fan-out on the `🖍️draw`
  plugin, mid-flight, relocating `🔄️fsm` exactly the way this pass relocated `🚪️io`'s facets, and the
  repo-root `Cargo.toml` workspace `members` list (outside every plugin's own boundary) had not yet been
  updated to match. This broke **`cargo` for the entire workspace**, not just `draw` or `vcs` — proven by
  the fact that plain `cargo metadata`/`cargo check -p <anything>` all failed identically at the time.
  Zero lines of this failure touch `✏️s/🔌️plugins/🌿️vcs/**`. Per `📌️important.md`, not chased — polled
  `cargo metadata --no-deps` until it succeeded (~15-20 min), then re-ran.
- `cargo check -p semio-s-plugin-vcs --all-targets --keep-going` → **0 errors** (confirmed after the
  blocker cleared, `Finished \`dev\` profile [unoptimized] target(s) in 5m 51s`, genuine cold-ish build
  through this large dependency graph). Down from the 3 pre-existing errors at baseline. Warnings present
  are pre-existing/unrelated: 1 `unused_imports` in this plugin's own
  `🚪️io/🧬️mutations/📝️text/🦀️component.rs:50` (`use super::*;`, unchanged content, only its physical
  directory moved — out of scope, owned by the separate `26/08/17/ZERO-WARNINGS-…` ticket), plus a
  `testkit`-is-ambiguous future-incompat warning whose two conflicting glob imports
  (`crate::os_spr::*`/`crate::os_pack::*`) both live in `🧰️framework/…/📦️glue.rs`, entirely outside this
  plugin's boundary.
  - ⚠️ **A second, cross-ticket peer landed on this plugin's root file mid-verification**:
    `✏️s/🔌️plugins/🌿️vcs/🦀️component.rs` picked up an uncommitted `.activation(…)`/`.execution(…)`/
    `.requests(…)` addition from the live `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` ticket
    (`git status --porcelain` showed `MM` — modified-and-modified-again — for this one file only; every
    other file this agent touched showed a clean single-layer `M`/`A`/`D`/`R`). Their edit built directly
    on top of this pass's `.declare_artifact(crate::artifacts::vcs::artifact())` call rather than
    reverting it, and the crate still compiles 0-error with both changes present — left in place per the
    "others are live in this tree, do not fight it" rule; not reverted, not further modified.
- `cargo nextest run -p semio-s-plugin-vcs --no-fail-fast` → launched; queued behind the shared
  `CARGO_TARGET_DIR` build-directory lock (30+ concurrent sibling W4 cargo processes observed via `ps
  aux` at time of writing — puzzle/imperative/dag/sourcing/animate/layout/reasoning-mindmap/flow/draw/
  writer/forms/cad/stdio/…). Not yet returned; **not a vcs-attributable slowdown**. Re-run and record the
  exact run/pass/fail counts once the shared lock frees up.
- `cargo check -p semio-s-plugin-vcs --target wasm32-wasip2 --lib` → launched, same shared-lock queue as
  above; not yet returned.
- `bun ./📜️script.ts policy` → ran repo-wide successfully (independent of the cargo blocker). Filtered to
  `🌿️vcs`: **zero** breaches on any of the seven `clean-mechanism/*` policies
  (`owner-mounts-children`/`io-exclusivity`/`subset-isolation`/`module-consumer-count`/
  `io-declaration`/`subset-standalone`/`declaration-tree`). The only breaches this plugin has are
  unrelated, pre-existing taxonomy/grammar policies outside this ticket's seven
  (`taxonomy/dead-example-leaf` ×4, `taxonomy/emoji-prefix-uniqueness` ×6, `taxonomy/plugin-root-shape`
  ×2 — missing `🎮️commands/🦀️component.rs`/`🔨️modules/🦀️component.rs` plugin-root facets, pre-existing,
  not touched by this pass — `handcrafted-grammar/spec-distinctness` ×42, `dsl-migration/
  diff-completeness` ×3, `protocol-migration/command-envelope-completeness` ×1, `pack-migration/
  completeness` ×1, `mutation-migration/semantic-vocabulary` ×1 — none of these attributable to files
  this pass touched, all pre-existing grammar/vocabulary debt for a later ticket).

**`cargo check --all-targets` is real (0 errors, above). `nextest`/`wasm32` are still queued behind
wave-wide build-lock contention** — do not read those two as "verified" yet; everything else in this
report (source cutover, `--all-targets`, policy run, manual code audit) is complete and confirmed.

## recipeGaps (for the next W4 agents)

1. **`ArtifactInferrer` cannot be retargeted onto `semio_framework_plugin::app::SnapshotBuilder<S, M>`,
   confirmed a second time.** Same orphan-rule violation (E0117) `📓️w4-sequence-report.md` documented —
   `SnapshotBuilder` is a foreign, non-`#[fundamental]` generic struct. Fixed the same way: a trivial
   local zero-sized marker struct (`VcsInferrer`). This is now confirmed on two independent plugins;
   later agents should treat it as the norm, not an exception to re-discover.
2. **A DIFFERENT, unrelated ticket can land a live edit on your plugin-root `🦀️component.rs` mid-pass**,
   not just this ticket's own peers. `git status --porcelain` showing `MM` (two uncommitted layers) on a
   single-writer file — instead of the plain `M`/`A`/`D`/`R` every other touched file shows — is the tell.
   Read the added content before assuming it conflicts: here it was additive (`.activation(…)`/
   `.execution(…)`/`.requests(…)` from `26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME`, built directly
   on top of this pass's `.declare_artifact(...)` call, not a revert of it) — confirm with a real
   `cargo check` that both changes coexist cleanly (they did here) before deciding whether to touch it at
   all. Left untouched either way; the plugin-root single-writer rule in `📌️important.md` is about MY
   ticket's own peers, not a guarantee no other ticket ever lands there.
3. **A wave-wide shared-`CARGO_TARGET_DIR` lock can queue your verification behind 30+ concurrent
   sibling builds** with zero relation to your own crate's correctness. `ps aux | grep "cargo check\|
   cargo nextest"` naming a dozen+ *different* plugin crates all mid-build is the tell — this is normal
   once the coordinator fans out the full W4 wave, not a regression to chase. `cargo check --all-targets`
   for one crate can still take 5-6 real minutes once it gets a lock turn purely from cold-building the
   shared framework/stdio dependency graph; budget for it.
- 🚧️ **A workspace-wide peer blocker can appear with zero warning and zero relation to your own files**:
  a sibling W4 fan-out relocating its own plugin's directories (exactly this recipe's own §4.d-style
  move) can leave the repo-root `Cargo.toml` `members` list stale for the whole time its `mv`s are
  in-flight, which breaks `cargo` for literally every crate in the repo, not just theirs. `git status
  --porcelain -- <their-plugin-dir>` is the fast tell (many uncommitted `R`/`A`/`D` lines); confirm the
  failure is workspace-wide (`cargo metadata` or `cargo check -p <anything-unrelated>` fails identically)
  before concluding it's your own regression. Don't touch the root `Cargo.toml` yourself — wait for the
  peer to commit.

## sharedFileRequests

None — every source change landed inside `✏️s/🔌️plugins/🌿️vcs/**`. (The transient workspace-load
blocker above is not a file this agent needs to write; it will clear when the `🖍️draw` fan-out commits.)

## openQuestions

1. **`MediaDeclaration.mimes` for standard `1`** (`🏅️standards/🔖️1/🦀️component.rs`) is a documented
   synthesis (`application/vnd.semio.vcs+json`), not a literal carry-over — no real MIME registration
   exists anywhere in the pre-migration code for this artifact (only a codec id `vcs.vcs` and an
   extension `vcs` claim). Flag for whoever eventually wires a real media-type registry for this artifact.
2. **`ArtifactDeclaration.localization: &[]`** — the real en/de localized names (`"VCS"`/`"VCS"`) still
   live on `definition()`'s `ArtifactCapability` rows (kept, unread by the new tree, per debt D1).
3. **csv/xlsx/zip foreign hops are Lossy to the point of near-uselessness** (see `## What was fixed`) —
   pre-existing behavior, preserved verbatim rather than redesigned; a real column/field mapping for each
   format is legitimate follow-up work, out of this cutover's scope.
4. **txt import/export stay honest stubs** (`TxtIntoVcs`/`VcsIntoTxt` always `Err`) — unchanged behavior
   from before this pass, now wired as real (if always-failing) `IoEntry` rows rather than dead
   hand-rolled functions.
5. Per `📓️w1-c-report.md`'s own openQuestion 3, `SurfaceDeclaration.mutation_roster` is declared but never
   read by the commit walk — `.editor_mutation_roster()`/`.viewer_mutation_roster()` were kept on the
   plugin builder as the still-live channel for that capability. Whoever eventually wires the new field
   should either delete these two calls at that point or confirm the new field replaces them cleanly.
