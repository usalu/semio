# 📓️ W4 fan-out report — `🎬️sequence` plugin

Agent: W4 fan-out, `🎬️sequence`. Boundary (only writer): `✏️s/🔌️plugins/🎬️sequence/**`.

## Starting condition (measured, before any edit)

```
cd /Users/ueli/Documents/semio
export CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/🎯️target"
cargo check -p semio-s-plugin-sequence --all-targets --keep-going 2>&1 | grep -cE '^error'
```
→ **16 errors** (the `--all-targets` run counted 17 lines matching `^error`, but the final summary line
"error: could not compile ... due to **16** previous errors" is the authoritative count — one match was the
summary line itself). All 16 were in files this agent owns:
- `✏️editor/🦀️component.rs`: 5× missing `imperative_engine::{register_native_imperative_module,
  contributions_json_from_entries, sync_imperative_module_contributions}` imports; missing `App` import in
  `testkit`; `assert_two_instances_converge::<SequencePlayApp, _>` should have been
  `::<EditorApp<SequencePlayApp>, _>` (5 cascading `ArtifactApp`-not-satisfied/`snapshot`/`dispatch` errors
  downstream of that one).
- `🧬️mutations/🦀️component.rs`: `vcs::apply_mutation(...)` with no `vcs` crate alias in `📦️glue.rs`;
  `SequenceStore::new(...)` used without `.expect(...)` despite returning `Result<Self, VcsError>`.

`git log --date=iso -1` / `git status --porcelain` on the specific broken file
(`✏️editor/🦀️component.rs`) confirmed `1d71198c19` (2026-08-17 14:44), before this ticket's start commit
(`101a6b4ea8`, 15:59:36) — pre-existing rot, not a peer's in-flight work, matching `📓️status.md`'s own note.

`cargo nextest run` could not run at all against this baseline (compile failure) — no baseline test count exists.

## What was fixed / built

### Pre-existing compile errors (all in files I own)
- `📦️packages/🦀️rust/📦️glue.rs`: added `extern crate semio_framework_os_kernel as vcs;`.
- `✏️editor/🦀️component.rs`: completed the `imperative_engine::{...}` import list; added `App` to the
  `testkit` module's `use semio_framework_plugin::{...}`; retargeted
  `assert_two_instances_converge::<SequencePlayApp, _>` → `::<semio_framework_plugin::EditorApp<SequencePlayApp>, _>`
  (matches the pattern every other `EditorApp`-wrapped plugin in the repo uses).
- `🧬️mutations/🦀️component.rs`: `SequenceStore::new(...)` → `.expect("valid artifact store fixture")`.

### A second, previously-undetected pre-existing bug (found only because the crate now compiles and its test
suite could run for the first time)
`sequence_snapshot_mutations` (the before/after fixture diff every editor command routes through) emitted a
`DisconnectSteps(edge)` mutation for every edge dropped between snapshots — including edges dropped **because
their step was deleted in the same diff**. `DeleteStep`'s own diff (`🧬️mutations/🗑️delete-step/🔺️diff`) is
already a cascade (it drops every edge touching the deleted step as part of that one mutation), so the
redundant `DisconnectSteps` for the same edge applied against a snapshot where the edge no longer existed,
rejecting the whole batch with `"Edge \"edge-1\" does not exist."`. This broke `RemoveStep`, `DeleteSelection`,
and `NodeGraphEdit`'s delete action — three real editor commands, unreachable by any test until this pass.
Fixed by skipping a `disconnect_steps` emission for an edge whose endpoint is one of the diff's own
newly-deleted steps (`🧬️mutations/🦀️component.rs`, `sequence_snapshot_mutations`).

### Declaration tree (atomic cutover, design.md §1/§2)
- **Subset root** (new file): `🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️component.rs` —
  `pub fn subset() -> SubsetDeclaration`, assembling `schema`/`io`/`viewer`/`editor`/`examples`.
  `editor`/`viewer` are read via `crate::editor::sequence`/`crate::viewer::sequence` (top-level, per recipe §5
  gotcha 1 — pre-existing structural fact, unchanged).
- **Standard root** (new file): `🏅️standards/🔖️1/🦀️component.rs` — `pub fn standard() -> StandardDeclaration`,
  mounts subset `any`. `mimes`/`extensions`: **no real MIME registration exists anywhere for this artifact**
  (unlike stdio's `📜️artifact-definition.json`) — the old `ArtifactCapability` channel only ever claimed a
  codec id (`sequence.sequence`) and an extension (`sequence`), never a mime type. `mimes:
  ["application/vnd.semio.sequence+json"]` is a documented synthesis (see `## openQuestions`), `extensions:
  ["sequence"]` is the real, carried-over value.
- **Artifact root** (edited): `🗿️artifacts/🎬️sequence/🦀️component.rs` — added `pub fn artifact() ->
  app::declarations::ArtifactDeclaration` (kind `s.sequence.sequence`, one standard). Deleted the OLD
  `pub fn declaration()` (the `ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...)
  .document_codec(...)` chain) outright — no dual channel. `definition()` (old `ArtifactDefinition`/capability
  rows) is **kept**, per debt D1 — it is not deleted repo-wide until W6, and this pass has zero callers left for
  it (harmless, matches the ticket's own accepted shape for this stage).
- **Plugin root** (edited): `🦀️component.rs` — `.declare_artifact(crate::artifacts::sequence::artifact())`
  replaces `.artifact(declaration())` + `.editor::<SequencePlayApp>(...)` + `.viewer::<SequenceViewer>(...)`
  in the same edit (atomic — the old channel is not registered alongside the new one).
  `.editor_mutation_roster()`/`.viewer_mutation_roster()` are **kept** — these are an orthogonal opt-in
  (`contributor.list-artifact-mutations`) `SurfaceDeclaration.mutation_roster` does not yet wire live
  (`📓️w1-c-report.md` openQuestion 3, confirmed by reading the commit walk: the field is set `None` by
  `editor_surface`/`viewer_surface` and never read) — keeping them is not a second registration of the
  artifact/schema/io itself, so it does not reintroduce the compatibility layer this ticket forbids.

### Old hand-rolled machinery deleted
- `🚪️io/🦀️component.rs`: deleted `derived_composition::SequenceComposerComposition` (`ArtifactComposition`
  impl) and `io_registry` (`ComposerEntry` aggregation) outright. Replaced with `pub fn io() -> IoDeclaration`
  (below).
- `🧬️schema/🦀️component.rs` (subset schema root): deleted `derived_construction::SequenceBuilderConstruction`,
  `derived_analysis::SequenceAnalyzerAnalysis`, and the `derive_artifact_facets!` call (`SequenceBuilder`/
  `SequenceAnalyzer`/`SequenceComposer`). Replaced with `pub type Construction =
  semio_framework_plugin::app::SnapshotBuilder<SequenceSnapshot, SequenceMutation>;` (W1-C task 3's trivial-
  subset shape — no custom analysis/composition logic this subset needs beyond the ordinary
  `Mutation`/`MutationDiff` algebra).
- `💡️inferences/🦀️component.rs`: `ArtifactInferrer` impl retargeted off the deleted `SequenceBuilderFacets` —
  see `## recipeGaps` for why `SnapshotBuilder` (the recipe's literal suggestion) does not work and what the
  real fix is.
- Deleted (unmounted, forbidden-per-design.md-§1 plugin-level shapes, confirmed zero `#[path]` references in
  `📦️glue.rs`): `🎟️capabilities/`, `🔧️setup/`, `🛂️manifest/` — each was a stub doc-comment-only file, never
  compiled.

### Foreign io leaves rewritten as typed `Serializer`/`Deserializer` (design.md §3)
All 8 leaves under `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/{txt,csv,md,json}/…` and
`🚪️io/📤️export/🧵️serializers/🗿️artifacts/{txt,csv,md,json}/…` rewritten from hand-rolled `deserialize_bytes`/
`serialize_bytes` free functions into real `impl Deserializer<SequenceSnapshot>` / `impl
Serializer<SequenceSnapshot>` marker-struct impls (`CsvIntoSequence`/`SequenceIntoCsv`,
`MdIntoSequence`/`SequenceIntoMd`, `JsonIntoSequence`/`SequenceIntoJson`, `TxtIntoSequence`/`SequenceIntoTxt`),
each declaring `FROM`/`INTO` + `IoFidelity` (`Json = Exact`, `Md = Canonical` — wraps the full `.sequence` DSL
text losslessly, `Csv = Lossy` — drops `edges` entirely, a flat grid has no edge concept; `Txt` = honest
not-yet-implemented stub, unchanged behavior, `Lossy`). Registered via `serializer_entry`/`deserializer_entry`
in the new `io()`:

```rust
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    // entries: 4 serializer_entry + 4 deserializer_entry calls, keyed on SEQUENCE_DIALECT
    // native.codec: store::ArtifactCodec::of::<SequenceSnapshot, SequenceMutation>(...)
}
```

**Bug fixed in passing** (`🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📊️csv/…`): the old `deserialize_bytes`
decoded the incoming bytes as a `SequenceSnapshot` pack directly instead of as a `CsvSnapshot` pack first —
would have silently misbehaved on any real CSV import. The new `CsvIntoSequence::deserialize` decodes the
foreign `CsvSnapshot` via its own `ArtifactPack`, as the `FROM: CSV_DIALECT` coordinate requires.

### Native codec relocation (design.md §1 CORRECTION — unsplit, `🧬️schema` keeps only types)
Physically `mv`'d (plain `mv`, never git) all four facets' `{📝️text,💾️binary}` children from `🧬️schema/` to
`🚪️io/`:
- `📸️snapshot/{📝️text,💾️binary}` — the ONE facet with a real relocation to do: `impl store::ArtifactDsl for
  SequenceSnapshot` + its `TextPrimitives`/`ChildCodecPrimitives` helpers moved into
  `🚪️io/📸️snapshot/📝️text/🦀️component.rs`; `impl store::ArtifactPack for SequenceSnapshot` +
  `BinaryPrimitives` moved into `🚪️io/📸️snapshot/💾️binary/🦀️component.rs`. `🧬️schema/📸️snapshot/🦀️component.rs`
  now holds only the `SequenceSnapshot` struct, `Default`, `default_snapshot()`, and the `SequenceFixture`
  bridge — types and pure transforms only, per design.md rule.
- `🔺️diff/{📝️text,💾️binary}`, `🧬️mutations/{📝️text,💾️binary}`, `💡️inferences/{📝️text,💾️binary}` — moved
  wholesale (grammar/protocol assets + the code they already carried; `mutations`'s real `OpText`/`OpBinary`
  impls for `SequenceMutation` were **already** correctly scoped to these facets, not the schema root — only
  their physical directory moved). No content surgery needed for these three; `diff`'s
  `apply_to_artifact`/`MutationDiff` impl and `mutations`'s codec impls resolve identically regardless of
  which physical file compiles them (Rust resolves inherent/trait impls by type, not by the impl block's file
  path) — confirmed zero cross-file references to the old `schema::{diff,mutations,inferences}::{text,binary}`
  module paths anywhere in the crate except `📦️glue.rs`'s own mount block and 4 shim lines (both updated).
- `📦️glue.rs`: removed the 4 facets' `{text,binary}` sub-mounts from under `schema::{...}`; added equivalent
  mounts under a new `io::{snapshot,diff,mutations,inferences}::{text,binary}` block (right after `io`'s own
  component mount, before `import`/`export`). Updated the 4 "Shims" lines that pointed at the moved paths
  (`op`/`dsl`/`spr`/`diff::text`) to the new `io::` targets — these shims re-export the same (only) subset's
  own children under short-hand plugin-root aliases; kept as-is otherwise (not attempted to eliminate the
  shim pattern wholesale — see `## recipeGaps`).
- Added `#[path]` mounts for the previously-unmounted **subset root** (`subsets::any::component`) and
  **standard root** (`standards::v1::component`) in `📦️glue.rs` — these files existed on disk (I wrote them)
  but were never `mod`-declared until this fix; the crate would not have compiled `standard()`/`subset()`
  otherwise.

### TS mirrors
- `🚪️io/🟦️component.ts`: real `IoEntryDescriptorMirror[]` (8 entries, mirroring the Rust `io()` exactly —
  fidelity values match), replacing the `export {};` stub.
- `📦️packages/🟦️typescript/📦️index.ts`: fixed **pre-existing** stale exports (`🪓️decomposer` and a flat
  artifact-level `🧬️schema`/`🚪️io` target — neither exists anywhere in the current tree, confirmed zero
  matching directories) to the real standard/subset-scoped paths.

## verification

All commands from `/Users/ueli/Documents/semio`,
`CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM/🎯️target"`.

- `cargo check -p semio-s-plugin-sequence --lib --keep-going` → **0 errors**, 2 pre-existing warnings
  (unrelated `unused_mut`/`future-incompat` from `block v0.1.6`, not mine).
- `cargo check -p semio-s-plugin-sequence --all-targets --keep-going` → **0 errors** (down from 16).
- `cargo nextest run -p semio-s-plugin-sequence --no-fail-fast` → **[FILL IN: still running at report-write
  time — see addendum below / re-run before trusting this report as final]**.
- `cargo check -p semio-s-plugin-sequence --target wasm32-wasip2 --lib` → **[FILL IN: cold build in progress
  at report-write time, ~7min precedent from W1-C]**.
- `bun ./📜️script.ts policy` → **[FILL IN: running at report-write time]**.

One blocked mid-session, unrelated, resolved-by-waiting peer collision (not mine, not blocking my final
numbers): `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️component.rs`/`🦀️reconcile.rs` were
` M` (uncommitted, live peer edit) at one point mid-session and caused a transient `semio-framework-ui` E0507
failure on one `cargo check` attempt; a retry a few minutes later compiled clean — sequence's own crate has
zero lines in that file, confirmed by `git status --porcelain` returning empty for every file this agent
touched.

## recipeGaps (for the next ~28 W4 agents)

1. **`ArtifactInferrer` cannot be retargeted onto `semio_framework_plugin::app::SnapshotBuilder<S, M>`** —
   the recipe (§2) says to do exactly this "when you delete the macro." It is a genuine **orphan-rule
   violation (E0117)**: `SnapshotBuilder` is a foreign, non-`#[fundamental]` generic struct, so `impl
   ArtifactInferrer for SnapshotBuilder<LocalSnapshot, LocalMutation>` is illegal regardless of the type
   *parameters* being local — confirmed by actually compiling it. `ArtifactInferrer::infer` takes
   `&Self::Snapshot`, never `&self`, so the impl target is a pure type-level anchor with (grepped) zero live
   callers repo-wide. Fix: a trivial local zero-sized marker struct (`pub struct XInferrer; impl
   ArtifactInferrer for XInferrer { type Snapshot = X; type Inference = XInference; }`), not `SnapshotBuilder`.
2. **The subset root and standard root files are easy to write and forget to mount.** Neither
   `crate::artifacts::<a>::standards::<s>::standard()` nor `…::subsets::<x>::subset()` resolves until you add
   a `#[path]` `mod component; pub use component::*;` for each new root file in `📦️glue.rs` — the file
   existing on disk is not enough. `E0425: cannot find function` is the exact symptom; easy to miss because
   `cargo check --lib` on the file's OWN content looks fine in isolation.
3. **The `owner-mounts-children-glue-scope`/`-glue-shim` policies are stricter than the actual reference
   implementation.** stdio's own W2-P pilot `📦️glue.rs` (read directly, not from its report) still
   centralizes every schema/io mount in `📦️glue.rs` (not delegated to each owner's own root file) and still
   carries the "Shims: keep pre-migration module paths" block verbatim — i.e. the reference exemplar this
   ticket points every fan-out agent at has NOT itself achieved what `owner-mounts-children` fully wants. This
   agent matched that same "additive declaration tree, deep mounts still centralized in glue.rs" shape rather
   than attempting a full mount-tree redistribution + full shim elimination (the latter alone is a ~135
   call-site rename for this one plugin) — the physical `schema`→`io` facet *directory* relocation was done
   (explicitly instructed), but the glue.rs mount *nesting* was only reorganized for the newly-relocated io
   facets and the two new root files, not redistributed down into each owner's own file. Flagging so nobody
   assumes "policy count reaches 0" is an achieved W4 norm anywhere yet — it isn't, on the one real
   reference implementation available to check against.
4. **A pre-existing runtime bug can hide behind a pre-existing compile error indefinitely.** This plugin's
   3 test failures (`sequence_snapshot_mutations` emitting a `DisconnectSteps` for an edge `DeleteStep`
   already cascaded away) were undetectable until the crate compiled for the first time under this pass.
   Worth a blanket warning to the remaining ~28 agents: "0 errors" is necessary but plan real time for
   genuinely NEW test failures once a long-broken crate compiles again, not just format/import fixes.

## sharedFileRequests

None. Every change landed inside `✏️s/🔌️plugins/🎬️sequence/**`.

## openQuestions

1. **`MediaDeclaration.mimes` for standard `1`** (`🏅️standards/🔖️1/🦀️component.rs`) is a documented synthesis
   (`application/vnd.semio.sequence+json`), not a literal carry-over — no real MIME registration exists
   anywhere in the pre-migration code for this artifact (only a codec id `sequence.sequence` and an extension
   `sequence` claim). Flag for whoever eventually wires a real media-type registry for this artifact.
2. **`ArtifactDeclaration.localization: &[]`** — the real en/de localized names (`"Sequence"`/`"Sequenz"`)
   still live on `definition()`'s `ArtifactCapability` rows (kept, unread by the new tree, per debt D1).
   Wiring them into the new field is real follow-up work, not required for this pass (mirrors the stdio
   pilot's identical documented deviation).
3. **txt import/export stay honest stubs** (`TxtIntoSequence`/`SequenceIntoTxt` always `Err`) — unchanged
   behavior from before this pass, now wired as real (if always-failing) `IoEntry` rows rather than dead
   composer-table entries. A real implementation is out of scope here.
4. Per `📓️w1-c-report.md`'s own openQuestion 3, `SurfaceDeclaration.mutation_roster` is declared but never
   read by the commit walk — `.editor_mutation_roster()`/`.viewer_mutation_roster()` were kept on the plugin
   builder as the still-live channel for that capability (see `## What was fixed` above). Whoever eventually
   wires the new field should either delete these two calls at that point or confirm the new field replaces
   them cleanly.
