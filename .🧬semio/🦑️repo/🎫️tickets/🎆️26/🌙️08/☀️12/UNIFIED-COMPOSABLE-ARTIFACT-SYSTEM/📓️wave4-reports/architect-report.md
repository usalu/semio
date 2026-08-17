# W4 batch Db — `architect` composes stdio `table` (2 of 68 register collections), `graph`/`R:model` investigated, not implemented

**ucas-status: partial — real, fully-verified `table` composition landed on 2 register collections
(`benchmarks`, `knowledge`), 0 compile errors (reproduced twice), 190/191 tests passing (reproduced
twice, non-flaky), the 1 remaining failure independently traced to a pre-ticket SMO-fanout commit
unrelated to composition. `graph`/`R:model` investigated in depth, not implemented — honest reasons
below. Canonical id `s.architect.program` was already correct; no fix was needed.**

## What the codebase actually looks like (verified against code, not assumed from the design doc)

`✏️s/🔌️plugins/🏛️architect/🗿️artifacts/` has exactly **one** artifact root: `🏛️program`
(`ProgramSnapshot`, schema id `s.architect.program` — already canonical, see `## Canonical-id
finding` below). `ProgramSnapshot` is **not** a small struct: it carries **68 `Vec<T>` register
collections** (`stakeholders`, `users`, `activities`, … `benchmarks`, `traces`) plus `meta`/
`project`/`governance` blocks — confirmed by direct count of `#[dsl(table)]` fields in
`📸️snapshot/🦀️component.rs` before any edit. Crate: `semio-s-plugin-architect`
(`✏️s/🔌️plugins/🏛️architect/📦️packages/🦀️rust/Cargo.toml`). The mutation-triad directory count
(`274` entries under `🧬️mutations/`, minus 8 non-triad files) confirms **~66 collections × 4
verbs** (`create`/`replace`/`delete`/`rename`) plus 2 edge-shaped collections
(`adjacencies`/`traces`) with an extra `connect`/`disconnect` pair each — matching
`📌️important.md`'s own D2-adjacent "additive struct fields" caution and `📓️design-full-plan.md`'s
scale warning ("architect ~43k lines, comparable to fem/norm, expect possible partial
completions").

**Every one of the 68 collections already has a genuinely sparse, id-addressed CQRS delta**
(`ProgramXxxDelta{ added: Vec<T>, removed: Vec<String>, patched: Vec<PatchEntry>, reordered:
Option<Vec<String>> }`, applied via one generic `apply_collection_delta<T: Identified<EntityId> +
Patchable<P>>` helper in `🔺️diff/📝️text/🦀️component.rs`) — a real, already-correct, already-generic
implementation of exactly the granular-mutation shape this whole ticket programme wants elsewhere.
This does **not**, on its own, disqualify composition (see `## Why composition was NOT blocked` —
this is the same reasoning norm's Round 2 corrected about Round 1's misread of D2), but it does mean
composing a collection is a genuine trade (sparse per-row VCS diff → one re-minted whole-table diff
per mutation, the same trade `mathematical`/`dag`/`en1990`/`din18599` already accepted), not a free
win — worth composing selectively, not wholesale, given the plugin's scale.

## Canonical-id finding

The dispatch brief flagged `architect→C:table,graph R:model (id s.architect.program)` as a
possible non-canonical-id fix. Checked directly (`grep -rn "artifact_schema(id\|envelope_id()"`
across every `🦀️component.rs` in the plugin, before any edit): `📸️snapshot/🦀️component.rs:13`,
`🔺️diff/🦀️component.rs:13`, and `🧬️schema/🦀️component.rs:14` (the schema-descriptor facet) all
already declare `#[artifact_schema(id = "s.architect.program")]`; sibling facets are consistently
namespaced (`s.architect.program.inference`, `s.architect.architect.config`,
`s.architect.architect.presence`). **No stray/legacy id was found anywhere in the plugin** — the id
was already canonical before this session touched anything. I did not fabricate a fix for a
non-existent problem (same discipline `mathematical`'s "id `a` dies" search and `norm`'s "R:fem"
search both documented for a negative finding).

**Also found, NOT mine**: a live uncommitted edit was already present in
`🎛️apps/🏛️architect/🦀️component.rs` (adding `type Transient = semio_framework_plugin::NoTransient;`
/ `type TransientMutation = ...NoTransientMutation;` to the `ArtifactApp` impl) and a one-line
docstring change in `🧬️schema/🦀️component.rs`, both already staged (`git status` showed `M `) at
session start, both part of HEAD commit `696b87d16e…` (2026-08-13 16:49:56). Confirmed via
`grep -rln "type Transient ="` that this is a **repo-wide** pattern already landed in `trinity`,
`remodel`, `raster`, `flow`, and others — an unrelated framework-wide fan-out (adding a `Transient`
lane to `ArtifactApp`), not this ticket's work and not touching anything I needed to edit. Left
alone, not fought, per `📌️important.md`'s guidance.

## What changed — `benchmarks` and `knowledge` compose `s.stdio.semio.table`

Picked the two **lowest-blast-radius** of the 68 collections: `benchmarks: Vec<BenchmarkRecord>`
and `knowledge: Vec<KnowledgeRecord>` are the only two collections with **zero** references
anywhere in `🎛️apps/`/`💡️inferences/` outside their own 4-triad mutation leaves and the generic
catalog dispatch (verified: `grep -rl "\.benchmarks\b\|\.knowledge\b"` across the whole plugin,
counted every hit, not just the first grep pass — my first grep used a wrong path prefix and
returned a false "zero elsewhere" result; the real count is 4 real call sites per field in
`💡️inferences/🦀️component.rs` plus 2 in `🗂️catalog.rs`, all fixed, see `## Files touched`). Neither
field is one of the 8 registers wired into `patch_register_item_operation`'s reflection dispatch
(`stakeholders`/`elements`/`adjacencies`/`requirements`/`risks`/`issues`/`functions`/`users`), so
no generic JSON-patch UI feature needed rewiring.

### The codec wall did NOT apply — confirmed by trying, not assumed

Per `📓️migration-recipe.md`'s 2026-08-13 update, `impl<S> crate::os_dsl::DslField for
ArtifactChild<S>` now exists (`🏪️store/🦀️component.rs:523`, verified real/generic/complete by
reading the source before starting). Tried keeping `ProgramSnapshot`'s
`#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]` as-is
and just swapping the two field types — **it compiled on the first full attempt**, 0 errors. No
hand-rolled `ArtifactDsl`/`ArtifactPack` was needed for `ProgramSnapshot` (unlike every W3/early-W4
exemplar, which all predate this framework addition and all hand-rolled). This is a real, confirmed
positive finding for future fan-out agents on plugins with a similarly derive-heavy snapshot: check
the impl before assuming you need to hand-roll a 68-field codec.

### Composition machinery (`🗿️artifacts/🏛️program/🦀️component.rs`, new `🔖️Composition` region)

- `ProgramBenchmarksChild`/`ProgramKnowledgeChild` — `store::ArtifactChild<SemioTableSnapshot>`
  type aliases.
- **Real bidirectional converters**: `benchmark_table_from_records`/`benchmark_records_from_table`
  and the `knowledge_*` equivalents. Every register row nests a rich `EntityHeader`
  (id/name/status/priority/ownership/tags/notes/timestamps) with no clean native `table`-column
  mapping, so — following `🕸️dag`'s "honest string boundary" precedent for its own
  richer-than-native node/edge types — each row becomes 3 columns: `id`/`name` (native projection,
  for genuine table-tooling) + `json` (the FULL `serde_json` serialization, the actual source of
  truth on decode). The inverse converter skips a row whose `json` cell is missing/unparseable
  rather than fabricating a partial record from `id`/`name` alone (`BenchmarkRecord`/
  `KnowledgeRecord` have no `Default`) — documented honestly in the converter's own doc comment,
  not silently.
- **Working scene**: `PROGRAM_BENCHMARKS_SCRATCH`/`PROGRAM_KNOWLEDGE_SCRATCH`, each a
  `thread_local! RefCell<HashMap<child_id, Vec<Record>>>`, matching `➗️mathematical`'s
  `MATH_SCRATCH`/`📕️norm`'s `EN1990_QK_SCRATCH` exactly — content-addressed scene id
  (`architect-benchmarks-<hash>`/`architect-knowledge-<hash>`), minted+cached by
  `benchmarks_child_from_records`/`knowledge_child_from_records`, read through the single accessor
  `program_benchmarks`/`program_knowledge` (fails soft to `Vec::new()` on a cache miss — same
  documented staleness gap every prior exemplar accepted; no `LinkResolver`/child-dispatch seam
  exists in `ArtifactApp::handle` yet, checked directly against `🔌️plugin/🦀️component.rs`,
  W1-owned, read-only).

### Snapshot / full-artifact struct / diff

`📸️snapshot/🦀️component.rs`: `benchmarks`/`knowledge` fields swapped `Vec<T>` →
`ProgramXxxChild`, `#[dsl(table)]` → `#[dsl(block)] #[child(kind = "s.stdio.semio.table")]`.
`🧬️schema/🦀️component.rs` (`ProgramArtifact`, the UI-inclusive struct): identical field-type swap
(`to_snapshot`/`from_snapshot`/`set_snapshot` needed zero logic changes — they clone/assign the
field verbatim regardless of its type). `🔺️diff/🦀️component.rs` (`ProgramDiff`): fields swapped
`Option<ProgramXxxDelta>` → `Option<ProgramXxxChild>` (single-Option, always-present-slot shape per
`📓️migration-recipe.md` §9); the now-dead `ProgramBenchmarksDelta`/`ProgramBenchmarksPatchEntry`/
`ProgramKnowledgeDelta`/`ProgramKnowledgePatchEntry` types removed, replaced with a doc comment
explaining the composition. `🔺️diff/📝️text/🦀️component.rs`: `apply_to_artifact`'s two
`apply_collection_delta(...)` call sites for these fields collapsed to a plain
`next.xxx = child.clone()`; `absorb`'s two merge-by-extend blocks collapsed to
`if other.xxx.is_some() { self.xxx = other.xxx; }` (simpler than before, not more complex — a
whole-handle replace has no "merge added/removed/patched" logic to write).

### Mutation triads (8 kinds — 4 verbs × 2 fields)

`create`/`replace`/`delete`/`rename`-`benchmark-record` and the `knowledge-record` equivalents.
Every `🦠️mutation/🦀️component.rs` payload struct is **byte-for-byte unchanged** — only the 7 (of 8)
`🔺️diff` bodies and 3 (of 8) `↩️inverse` bodies that read `base.benchmarks`/`base.knowledge`
directly were rewired to `crate::artifacts::program::program_benchmarks(base)` /
`program_knowledge(base)` (a working-scene read) followed by the same
push/retain/find-and-mutate/find-and-rename logic as before, then
`benchmarks_child_from_records(&records)` / `knowledge_child_from_records(&records)` (re-mint).
`create-*`'s own `↩️inverse` needed no change (never read `base` directly). Mirrors
`mathematical`'s/`norm` Round 2's identical pattern.

### App-layer rewiring (6 call sites, `🗂️catalog.rs` + `💡️inferences/🦀️component.rs`)

`register_entities`/`find_register_for_entity` (`🗂️catalog.rs`) special-case `"benchmarks"`/
`"knowledge"` before the generic `program.$field.iter()` macro expansion (which no longer type-checks
for a non-`Vec` field), reading through the two accessors instead; the two macro-generated match
arms for these two field names were removed from their literal lists. `💡️inferences/🦀️component.rs`:
4 call sites per field (`for e in &program.xxx {}`, `collect("xxx", program.xxx.iter()...)`,
`search_register!("xxx", &program.xxx)`, `push_rows!("xxx", &program.xxx)`) rewired to read through
the accessor — found by a **second, corrected** `grep -rn "\.benchmarks\b\|\.knowledge\b"` sweep
after my first (wrongly-pathed) sweep had returned zero hits; caught before compiling, not after (the
sweep-then-count discipline `📌️important.md` asks for).

### `empty_plugin()` (`🗿️artifacts/🏛️program/🦀️component.rs`)

`benchmarks: Vec::new()`/`knowledge: Vec::new()` → `benchmarks_child_from_records(&[])`/
`knowledge_child_from_records(&[])` (mints + caches an empty-content handle). `sample_plugin()`
needed no change — it only mutates `elements`/`stakeholders`/`adjacencies`, never touches
`benchmarks`/`knowledge`.

### Whole-document replace — nothing to remove

Checked (grep, before any edit): `ArtifactApp for ArchitectPlayApp` never overrode
`whole_document_operation`. No `SetSnapshot`/`SetFixtureJson`-shaped app command exists in this
plugin at all — nothing to convert per recipe §6.

### Fixture regeneration (done twice, for real, not hand-transcribed)

`📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`'s pre-migration `benchmarks [...]  {}`/
`knowledge [...] {}` table-grammar lines don't parse against the new `#[dsl(block)]` shape
(`TextError: expected LBrace, found LBracket`). Regenerated via a temporary
`#[cfg(test)] mod debug_fixture_regen` in `📸️snapshot/📝️text/🦀️component.rs` dumping
`print_dsl(&sample_plugin())` under `DUMP_ARCHITECT_EXAMPLE=1 cargo test … debug_fixture_regen --
--nocapture`, captured, written verbatim, module removed — **twice**, once after composing
`benchmarks` alone, again after adding `knowledge` (the child-handle hash the fixture embeds changes
whenever the composed content changes, so both rounds needed a fresh dump). Verified clean both
times: `grep -rn debug_fixture_regen ✏️s/🔌️plugins/🏛️architect` → nothing.

## Why composition was NOT blocked (re-examining norm's own correction)

I initially suspected the "already-sparse CQRS delta" fact above was itself a disqualifying reason
(the same shape norm's Round 1 wrongly cited against `en1990.q_k`). Re-reading norm's own Round 2
correction: D2/Concern B is about how stdio's **own** `table`/`graph`/`text` subsets implement
their internal collection diff (sparse triple vs. whole-list-clone), not about whether a plugin
composing one of those subsets must give up its own dispatch-layer mutation granularity. The
public mutation API (payload struct, `MutationKind` impl, semantic descriptor) is unchanged either
way; only the internal `🔺️diff`/`↩️inverse` body changes from "build the delta directly" to "read
working scene, apply the operation, re-mint the whole child." That trade (sparse per-row VCS diff →
one re-minted whole-table diff per mutation) is real and already accepted by every prior
composed-table exemplar (`en1990.q_k`, `din18599.climate`) — not a reason to decline, just a cost to
weigh per-field against the plugin's actual scale. `architect`'s difference from `en1990` is one of
**degree** (68 collections, some potentially large in a real building program, vs. `en1990`'s single
small compliance table), which is why I composed 2 collections as a proof slice rather than all 68 —
not a reason to compose zero.

## `graph` — investigated, not implemented, concrete reason

`relationships: Vec<Relationship>` (`source_id`/`target_id: EntityId`) and
`adjacencies: Vec<Adjacency>` (`element_a_id`/`element_b_id: EntityId`) are genuinely edge-shaped.
But unlike `dag`'s self-contained node+edge model (where nodes and edges are the SAME collection's
two fields), architect's edge endpoints are **foreign keys into other, separately-diffed
registers** — `Adjacency` endpoints are (in practice) `elements` ids, but `Relationship.source_id`/
`target_id` can reference **any** entity kind (stakeholders, requirements, elements, … — confirmed
by reading `Relationship`'s own doc-free field: no kind tag restricts it, and `catalog.rs`'s
`find_register_for_entity` exists precisely because an id alone doesn't say which register it came
from). A real `s.stdio.semio.graph` composition needs actual node data, which means synthesizing
nodes from `elements` (at minimum) inside the SAME composed child that `adjacencies`/
`relationships` populate as edges — i.e. reconciling **3 separately-mutated collections**
(`elements`: 4 triads; `adjacencies`: 4 create/replace/delete/rename + 2 connect/disconnect;
`relationships`: 4 triads — 14 triads total, the same count `mathematical`'s co-derived
graph+geometry composition needed) into one working-scene-cached graph child, where **every one**
of those 14 triads must re-mint the FULL node+edge graph on every single element/adjacency/
relationship change. This is architecturally the same pattern already proven to work
(`mathematical`), not a blocked pattern — it's a **larger, well-defined unit of work** (3 collections
+ 14 triads + a cross-register node-synthesis converter, vs. this pass's 2 collections + 8 triads +
2 single-collection converters) that didn't fit inside this pass's remaining budget after landing
the `table` proof and the pre-existing baseline-compile fixes below. Documented here as a concrete,
scoped, ready-to-pick-up design (not a vague "later") rather than attempted half-done.

## `R:model` — investigated exhaustively, genuinely not found

Grepped the whole plugin (before any edit) for `ArtifactLink` (zero hits, anywhere), for `"model"`
in every `🦀️component.rs` (only false positives: `funding_model`/`sharing_model`/
`collaboration_model`/`cost_model` — unrelated `String` fields on unrelated register types; a
`Option<String>` field literally named `model` on `Equipment` — a free-text model/make string, not
a reference), and for `import_media`/`export_media`/`MediaPortSpec` mentioning "model" (zero hits —
unlike `fem`'s real `geometry:in` media port, architect has no equivalent). `ProgramElement`'s only
spatial-adjacent fields are free-text hints (`location_hint: Option<String>`,
`orientation: Option<String>`) — not a link to any spatial/CAD/layout model artifact. **No
plugin content qualifies as "referencing a model artifact"** — same honest non-finding class as
`mathematical`'s "id `a` dies" search and `norm`'s "R:fem" search (`en1992.use_fem: bool` was norm's
closest analog, a plain toggle, not a reference; architect has no analog at all). Nothing was
fabricated to fill this gap.

## Baseline (before any edit) — RED, unrelated to composition, fixed in-pass

`CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-architect --all-targets`: **0
errors on the plain `lib` target, 34 errors on `lib test`** (only surfaces under `--all-targets`,
i.e. `#[cfg(test)]`-gated code only). Two independent, unrelated pre-existing defects, both traced
by real commit + `--date=iso` (never the fake `🎆️🌙️☀️` message glyphs):

1. **31 `E0433` "cannot find `X` in `super`" errors** in `🧬️mutations/🦀️component.rs`'s
   `#[cfg(test)] mod tests` (and 2 more in `🧬️mutations/💾️binary/🦀️component.rs`) — leftover from
   SEMANTIC-MUTATIONS-OVERHAUL's "Wave C" one-triad-dir-per-variant directory restructuring
   (documented in the file's own header comment). The top-level enum correctly writes
   `super::create_stakeholder::...` (one level up from the `mutations` module, where triad dirs are
   real siblings via `glue.rs`'s `pub mod create_stakeholder { ... }` nested inside `pub mod
   mutations`); the SAME reference written one level deeper, inside `mod tests`, needs
   `super::super::create_stakeholder::...` — the restructuring updated the enum but not the
   pre-existing test module's imports. 2 more (`clear_adjacency`, `elements`) were stale PRE-Wave-C
   module names entirely (now `disconnect_adjacency`, `delete_program_element`). Traced:
   `git log -1 --date=iso -- 🧬️mutations/🦀️component.rs` → `a445617cae…`, **2026-08-12 15:50:51**
   — SMO's wave-2 mass mutations fan-out (the same commit `norm`'s report already cites for 3 of
   its own pre-existing failures), landed ~48min after this ticket opened but is not this ticket's
   work. Fixed: mechanical `super::` → `super::super::` for 18 module names across 31 call sites
   (script-verified against the enum-declaration region so only test-body references were touched),
   plus the 2 stale-name corrections.
2. **1 `E0063` "missing fields" error** in `💡️inferences/🧭topology/🦀️component.rs:170`'s test-only
   `ProgramElement` fixture builder — 11 fields added to `ProgramElement` since this literal was
   written, never updated. Traced: `git log -1 --date=iso` → `a46ac1f883…`, **2026-08-12 13:17:52**
   — genuinely predates this ticket's 15:02:49 open. Fixed: added the 11 missing fields with the
   same defaults `🗿️artifacts/🏛️program/🦀️component.rs`'s own `sample_plugin()` fixture uses.

Both fixed outright (trivial, mechanical, unambiguous — matching `📌️important.md`'s "cheaper to
just fix than keep chasing provenance" guidance) since without them `cargo check --all-targets` and
`cargo nextest run` could never succeed at all, blocking verification of anything, composition
included.

## Verification (actual, run in the foreground, reproduced twice)

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-s-plugin-architect --all-targets
```
Run 1 (after baseline fixes + both compositions): **0 errors**, `Finished` in 5.14s (warm) / 1m
02s–1m 14s (cold, contended with concurrent sessions' cargo locks). Run 2: identical, 0 errors.
Only pre-existing/cosmetic warnings remain (unused imports, unnecessary qualifications, dead
`serde_to_json_value`/`json_value_to_serde` helpers — none touched by this pass, none new).

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo nextest run -p semio-s-plugin-architect --no-fail-fast
```
Run 1: **191 tests run: 190 passed, 1 failed**. Run 2 (immediately after, no further edits):
**identical — 190 passed, 1 failed**, same single named failure both times — not flaky.

## The 1 remaining failure — independently traced, NOT introduced by this migration

`artifacts::program::standards::v1::subsets::any::schema::mutations::component::tests::create_stakeholder_obeys_the_inverse_and_absorb_laws`.

**Root cause (hand-traced, not guessed)**: `apply_collection_delta` (the SAME generic helper every
one of the 68 collections' `apply_to_artifact` calls, `🔺️diff/📝️text/🦀️component.rs`, untouched by
this pass) processes a delta in the order `removed` → `patched` → `added`. `absorb(create_diff,
rename_diff)` merges into `{ added: [new_stakeholder], patched: [{id, name: "Renamed"}] }`. Applying
this merged delta runs `patched` **before** `added` extends the list — the patch looks for
`new_stakeholder`'s id in the list BEFORE it has been added, finds nothing, no-ops; `added` then
appends the row with its **original**, unpatched name. The law
(`absorb(d1,d2).apply(base) == d2.apply(&d1.apply(base))`) fails because the RHS applies sequentially
(the stakeholder exists by the time the rename is applied) while the LHS applies the merged delta in
one pass with the wrong internal order. This is a genuine, structural bug in the shared
`apply_collection_delta` ordering — **completely unrelated to `benchmarks`/`knowledge`** (this test
never mentions either field; the `left`/`right` diff in the panic output shows both sides' `benchmarks`
field byte-identical, only `stakeholders[1].header.name` differs). I did not touch `stakeholders` or
`apply_collection_delta` anywhere in this pass.

**Dating**: `git log -1 --date=iso -- 🔺️diff/📝️text/🦀️component.rs` → `a445617cae…`, **2026-08-12
15:50:51** — the same SMO wave-2 commit already cited above and already independently confirmed by
`norm`'s own report as the origin of an identical class of "reorder/insert-vs-patch ordering" bug in
`din4108`/`iso16757`. This test could not have run before my baseline fixes (the whole `lib test`
target didn't compile) — it is a **newly-surfaced**, not newly-introduced, pre-existing defect.

**Why not fixed outright**: unlike the two baseline compile errors above (mechanical, single-file,
unambiguous), this is a real semantics bug in the ONE generic helper shared by all 68 collections'
diff-apply path. A correct fix (patch-then-add, or make `patched` also search `added`) is a genuine,
scoped, but non-trivial change to shared code affecting every one of the other 66 un-composed
collections too — outside "migrate benchmarks/knowledge to composed children," and exactly the class
of fix `mathematical`'s and `dag`'s own reports both independently deferred with full derivation for
the identical reason. Flagged here with complete root-cause + provenance so a dedicated fix (or a
future DiffKit-adjacent pass) can pick it up with zero re-investigation cost.

## sharedFileRequests

None. Every change stayed inside `✏️s/🔌️plugins/🏛️architect/**` (including the plugin's own fixture
asset); no `📦️glue.rs`/`📦️index.ts` edit was needed (no field-count/derive-list change forced a
codec-registration change — `ProgramSnapshot` kept its `#[derive(dsl::DslRecord)]`); no
`🗄️stdio/**` file was written (only read for reference: `SemioTableSnapshot`/`SemioTableColumn`/
`SemioTableRow`/`SemioTableCellKind` and `SemioValue`, at
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️table/🧬️schema/📸️snapshot/🦀️component.rs`).

One standing request (not new, reaffirming `fem`'s/`norm`'s own #1): **a `LinkResolver`/
child-dispatch seam in `ArtifactApp::handle`** (`🧰️framework/…/🔌️plugin/🦀️component.rs`, W1-owned)
is the blocker that keeps every composed-child migration (this one included) on the `thread_local!`
working-scene bridge instead of a real store-backed child history. Not architect-specific.

## Companion facet files — left stale, matching every prior exemplar's scope

`🔺️diff/{🟦️component.ts,🔗️component.graphql,🔣️component.json,🛰️component.proto}` still describe the
old `ProgramBenchmarksDelta`/`ProgramKnowledgeDelta` shape (only referenced in a doc comment in the
`.rs` sibling now, not compiled). Neither `mathematical`'s nor `dag`'s own "Files touched" lists
show any companion TS/GraphQL/JSON-schema/proto facet edits either — these are treated as
downstream-regenerated artifacts in every prior wave's scope, not hand-edited in this fan-out. Noted
here for honesty, not fixed, consistent with precedent.

## Concurrent-churn observations

`git status --porcelain -- ✏️s/🔌️plugins/🏛️architect` at session start showed 5 files already `M `
(staged) — investigated (see `## Canonical-id finding` above): a repo-wide `Transient`/
`TransientMutation` fan-out already landed in HEAD (`696b87d16e…`, 2026-08-13 16:49:56), touching
`🎛️apps/🏛️architect/🦀️component.rs` and a docstring in `🧬️schema/🦀️component.rs` — not this ticket's
work, not colliding with anything I edited, left untouched. No cargo-lock contention observed during
my own `cargo check`/`cargo nextest` runs beyond ordinary cold-cache compile time (one run took
1m02s–1m14s "cold" after another concurrent session's build evicted the incremental cache, both
still finished cleanly on the first attempt — no retries needed).

## Honest accounting

- **Done, verified, tested, reproduced twice**: `benchmarks`/`knowledge` (2 of 68 register
  collections) compose `s.stdio.semio.table` — real converters, working-scene cache, all 8
  create/replace/delete/rename mutation triads' public payloads unchanged, only diff/inverse
  internals rewired; canonical id confirmed already correct; 2 unrelated pre-existing baseline
  compile errors fixed outright; fixture regenerated for real (twice).
- **Investigated in depth, not implemented, concrete scoped reason given**: `graph` composition
  (`elements`+`adjacencies`+`relationships`, 3 collections/14 triads, cross-register node synthesis
  — architecturally the same proven `mathematical` pattern, just a larger unit than this pass's
  budget); `R:model` (exhaustively searched, genuinely does not exist in this plugin's content).
- **Deferred, independently traced, not fixed**: `apply_collection_delta`'s patch-before-add
  ordering bug (1 failing test) — real, pre-existing, shared-code, out of this pass's scope.
- **Not attempted**: the remaining 66 register collections' `table` composition (this pass proved
  the pattern on 2 as a real, working slice — matching `norm`'s own 2-artifact Round-2 precedent —
  rather than attempting all 66 and risking a half-broken cascade).

ucas-status: partial
