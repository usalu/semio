# 📓️ exec-architect — `✏️s/🔌️plugins/🏛️architect`

Status: **WRITTEN BUT UNVERIFIED** (no cargo result for this crate). Every `cargo check -p semio-s-artifact-architect-program --tests` through the
gate stops in `semio-framework-replication` (not mine): `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🌳️root/🦀️.rs:105-117`
and `.../🩹️update/🦀️.rs:64-148` — `expected RetainedCloneGrant, found Grant` / `expected SharedOwner<String>, found Arc<String>` (a peer's in-flight
retained-clone refactor, 12 errors). Logs: `T/🗑️generated/architect/check-native.log`. Run recipe once the framework is green:
`cd ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program && "$T/🚦️gate.sh" architect -- bash "$T/🗑️generated/architect/run.sh"` (native `check --tests`, wasm32-wasip2 check, `cargo test --lib`).
NOTE: the artifact crate is now its own cargo workspace root (`🏛️program/Cargo.toml`); `cargo -p` from `✏️s/` no longer finds it.

Sum law: `protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await` exists (fw-spine) and is called in every applied leaf test.

## Design decisions
- `ProgramDiff` = per-collection `added/removed/patched/reordered` deltas (66: 64 registers + `knowledge` + `benchmarks` row deltas), singleton
  section edits `ProgramMetaEdit/ProjectDefinitionEdit/GovernanceEdit { set, patch }`, scalar `schema`. Removed: `artifact`, `knowledgePayload`,
  `benchmarksPayload`, whole-value `meta/project/governance`, child handles (`apply` re-derives them from the patched rows).
- One generic algebra (`🔺️diff/🧮️algebra`): `apply` (remove→add→patch→reorder, in place), `absorb` (patch∘patch → one patch, create∘delete → nothing,
  delete∘create → replace, create∘patch → create, patch∘delete → delete, reorder composition), `inverse` (exact negative incl. row positions; a patch
  that sets an `Option` field falls back to remove+add), `between` (sync/import only). `ProgramDiff: MutationDiff + DiffAlgebra` concrete.
- `replace-*` / `connect-*` on an existing edge = whole-row replacement under the same id (`removed=[id]`, `added=[row]`, `reordered` unless last): exact,
  because a patch `Option<T>` field cannot clear an optional field (the old full-row `diff_patch` silently could not; `Option<Option<T>>` does not
  round-trip through `ToValue`). `Patchable::diff_patch` is now changed-only and `None` when inexpressible; only `between` uses it.
- `replace-project/-governance/-meta` keep `ReplaceX(base.section)` as inverse: the absolute setter of the entity the kind replaces (same shape as
  every entity replace); the diff is `Edit::set`. `replace-config`/`replace-presence` likewise keep an absolute `ReplaceX(base)` inverse; their diffs are
  sparse changed-field diffs (`ArchitectConfigDiff`, `ArchitectPresenceDiff`). If the lead wants per-field set kinds instead, that is a new-kinds change.

## Per kind (before → after)
| group | n | before | after |
|---|---|---|---|
| create / delete / rename std | 186 | clean | unchanged leaf code; tests moved to `protocol::apply_diff`, + sum-law test; fixture diff JSON lost `artifact/knowledgePayload/benchmarksPayload` keys |
| disconnect adjacency/trace | 2 | clean | same |
| replace std | 62 | V1-GENERIC-DIFF full-row `diff_patch` | concrete whole-row replacement diff (leaf-owned); fixtures rewritten |
| connect adjacency/trace | 2 | V1-GENERIC-DIFF (existing edge) | existing edge: whole-row replacement; new edge: `added` (unchanged) |
| benchmark / knowledge create,delete,rename,replace | 8 | V1-SNAPSHOT/GENERIC + V3-LEAF-APPLY (clone whole collection, mutate, emit all) | row deltas on `*_payload`; `program_benchmarks`/`program_knowledge` deleted (readers use `&snapshot.*_payload`); 6 new applied fixtures + tests for delete/rename/replace |
| rename project/governance/meta | 3 | V1-SNAPSHOT + V3-LEAF-APPLY | section patch (`patch.code/framework/title`) |
| replace project/governance/meta | 3 | V1-SNAPSHOT + V2-RESTORE | section `set`; inverse = absolute entity setter (see above) |
| editor replace-config / replace-presence | 2 | V1-SNAPSHOT + V2-RESTORE | sparse diff types; inverse absolute setter |
| window configs register/graph/adjacency/report | 4 | V1-SNAPSHOT + V3-HAND | `*WindowConfigDiff { field: Option<..> }`, `impl_whole_record_config!` replaced by `ConfigRecord` + concrete `MutationDiff/DiffAlgebra`; inverse already set-field-back |

Non-leaf: `behavior` module writers (23) now pure: `apply_template(&template)` → mutations, `build_report_and_record`/`run_analysis_and_record` → `(result, mutation)`,
`import_registers_csv/tsv(&program, ..) -> RegisterImport { snapshot, mutations, touched }` (rows become create/rename/replace/delete mutations folded through
`protocol::apply_diff`), trace queries read `effective_traces(&program)`, `add_trace_link` returns `ConnectTrace`; `set_adjacency/clear_adjacency/absorb_register_mutation` deleted.
`io/🦀️.rs` builder and `apply_program_mutation_outcome(&snapshot, &mutation) -> (snapshot, outcome)` use `protocol::apply_diff` (`MutationOutcome::apply_to` gone).

## Files (main)
`🧬️schema/🗄️registers/🦀️.rs` (RowPatch + exact patch algebra, meta/trace via macro, 5 patch DslRecords), `🧱️kernel/🦀️.rs` (`TraceLinkPatch.label: Option<String>`),
`🧬️schema/🔺️diff/🦀️.rs` + `🧮️algebra/🦀️.rs` (+ tests), 62+2+8+6 leaf `🔺️diff/🦀️.rs`, 272 leaf tests, `🧬️mutations/🦀️.rs`, `🧪️tests/🔬️fixture/🦀️.rs` (6 new mounts),
`🧬️mutations/🧪️tests/🔬️unit`, `✏️editor/🦀️.rs` + commands (template/register/exchange), config/presence/4 window configs + tests, `🚪️io/🦀️.rs`, csv deserializer,
inferences/catalog readers, `🦀️.rs` root (helpers removed), diff facets `🔣️.json/.graphql/.proto/.ts` (hand-synced; graphql/proto keep the existing `item` convention — regenerate when the schema emitter runs),
fixtures: 260 `🔺️diff/🔣️.json`, 6×5 new b/k fixtures, config/presence diff JSON. Generators: `T/🗑️generated/architect/gen_*.py`.

## Tests (NOT RUN)
Written: algebra unit tests (all four required absorb sequences + create∘patch, replace∘patch/delete, associativity, reorder composition, inverse position/Option fallback,
between, malformed apply), `ProgramDiff` absorb/inverse/between/section tests, multi-row + composed-table + singleton sum-law tests, config/presence/window diff law tests,
import-mutation replay tests. Planned command (after framework green): `cargo test -p semio-s-artifact-architect-program --lib` from `🏛️program`.

## Open issues
1. Cannot compile until `semio-framework-replication` is green again (peer). Expect a first-compile fix round (macro-heavy new code; desk-checked only).
2. Positional undo: `DeleteX` of a non-last row is undone by `CreateX` (appends). `DiffAlgebra::inverse` restores position, the mutation inverse cannot; all 66
   committed delete fixtures delete the last row, so the sum law holds on them. Needs a positioned create or a reorder kind.
3. `🧪️tests/🏛️mutate-program-1` feature/py oracle still map the 6 b/k kinds to the `absent-a` vectors (docs there are stale); new applied vectors are covered by the Rust leaf tests.
4. Facet files for `ProgramDiff` are hand-edited, not emitter-generated.

## Wave 3 — positional undo (ruling: delete-of-non-last must restore its position)

Status: WRITTEN, compile status below. Open issue 2 above is closed by this wave.

- **Payloads:** the 62 `create-*`, `create-benchmark-record`, `create-knowledge-record`, `connect-adjacency` and `connect-trace` payloads gained
  `index: Option<usize>` (absent = append; `skip_serializing_if` so every committed mutation fixture stays canonical). `connect-*` uses `index` only when the
  pair/id is new (an existing edge is replaced in place and keeps its position).
- **Diff:** a create with `index < len` emits `added=[row]` + `reordered` = base order with the id inserted at `index`; `index == len`/absent appends;
  `index > len` is `Error mutation.target-missing` (empty diff).
- **Inverse:** all 62 `delete-*`, the 2 b/k deletes, `disconnect-adjacency`, `disconnect-trace` invert to `CreateX { row, index: Some(position) }` /
  `ConnectX { .., index: Some(position) }` read from `base` (`position()`), so the summed inverse diff `{added:[row], reordered:[base order]}` equals
  `ProgramDiff::inverse` for middle rows too. Create/rename/replace inverses unchanged.
- **Schemas/twins:** 66 per-leaf `🧬️schema/🔣️.json` payload schemas (`index`, integer >= 0, optional), `🧬️mutations/🟦️.ts` (`index?: number`), `.graphql` (`index: Int`),
  `.proto` (`optional uint32 index = 2`). All non-leaf construction sites (templates, import, catalog, graph/element commands, unit tests) pass `index: None`.
- **Law fixtures:** 66 new applied vectors, one per ordered collection, that delete the MIDDLE row of a three-row collection: `<kind>/🗑️delete/🗑️deletes-middle`
  (64 delete kinds incl. benchmark/knowledge, whose child handles are recomputed) plus `🫷️disconnects-middle` (adjacency) and `✂️disconnects-middle` (trace); each has
  before/after/mutation/diff/outcome JSON, a test (round trip, canonical JSON, committed diff, sum law) mounted in `🧪️tests/🔬️fixture/🦀️.rs` as `*_middle`.
  Adjacency rows get distinct endpoints so the reconnect is not read as an upsert of an existing pair. Unit tests added: create at index (middle/last/append/out of range),
  delete of every row of a 3-row stakeholder list, middle knowledge delete, middle trace disconnect.
- **Move/reorder kinds:** the architect plugin has none (no reorder mutation exists), so no extra fixture.
- Generators: `T/🗑️generated/architect/gen_index.py`, `gen_middle.py`.
- Verification: checks only (disk), `run.sh` runs native `check --tests` + wasm check; tests run only with `ARCHITECT_RUN_TESTS=1`.

## Wave 4 — verification + translators (W6)

**Status: BLOCKED on foundation.** `🗑️generated/coord/foundation.status` stayed `RED` (native=101 wasm=101; last seen `RED 19:09:19`) for the whole 60-minute wait
(17:49-18:51) and after. Errors are outside this scope (peer's `owned_retirement` / `next_close_byte_demand` retirement refactor, e.g.
`🖱️ui/🎬️scene/…/📐️math/🦀️.rs:1078`). No cargo call was made under the new rule; the architect crate has still never been compiled and no test has run
(native check, wasm check and `ARCHITECT_RUN_TESTS=1` `cargo test --lib` all pending; command: `cd ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program && "$T/🚦️gate.sh" architect -- env ARCHITECT_RUN_TESTS=1 bash "$T/🗑️generated/architect/run.sh"`). Pass/fail counts: none.

Done in this round (all unverified):
- Frozen outcome codes: `mutation.index-out-of-range` → `mutation.target-missing` in all 66 create/connect diffs, their docs, and the unit test.
- R14: `📃️document/♻️replace/↩️inverse` now returns `[CreateDocument{index: Some(position)}, DeleteDocument]` (replays delete→create, last-to-first); no `ReplaceDocument` restore.
- **W6 finding — `ReplaceDocument` is NOT a whole-document kind and is kept.** `📃️document` is the program's `artifacts: Vec<ArtifactRecord>` register
  (documents *referenced by* a program); `ReplaceDocument { document: ArtifactRecord }` replaces one row of `snapshot.artifacts` by id — an entity replace, like
  `replace-process`. Deleting it would remove a legitimate entity kind (and its create/delete/rename siblings would lose their replace). The whole-artifact reset path
  (`reset_document_effect`) is already used by exchange import and the example command and emits NO mutation rows; nothing in architect builds a whole-document mutation.
  So no union/proto/graphql/TS/oracle entry is removed; coordinator should drop W6 from `audit-translators.md` or rename the finding.

## Wave 5 — positional rows (AMB-1), gate breaches (0), no minted data (AMB-2/3)

**Status: WRITTEN, UNVERIFIED by cargo** (foundation.status still `RED 23:29:36 native=101 wasm=101`; no cargo call made). Gate (`bun ./📜️script.ts verify mutation-outcome-law`,
log `T/🗑️generated/architect/gate-run.log`, run twice): **0 breaches under `🏛️architect`** (the 4 of gate-run-4 are gone; the run's remaining 36 are other plugins).

**AMB-1.** No diff carries an `order`/`reordered` list any more. Every collection delta is `removed:[{id,index}]`, `inserted:[{index,row}]`, `moved:[{id,from,to}]`,
`modified:[{id,patch}]` (shared `ListRemoval`/`ListRelocation`, per-register `Program<X>Insertion`, `Program<X>PatchEntry` kept as the modification row; 66 deltas).
- `🧬️schema/🔺️diff/🧮️algebra/🦀️.rs` rewritten after norm-a's API: `Parts::{commit, absorb, inverse}` with index-coordinate arithmetic (`rank`/`nth_free`); `absorb` coalesces per id
  (insert∘remove → nothing, remove∘insert → replacement, insert∘move → insert at final slot, move∘move → one move, move∘remove → remove at the base index, patch∘patch → one
  patch, insert∘patch → changed insert); `inverse(base)` reads base rows one by one (removed ← inserted at their after index, inserted ← removed read from `base[index]`,
  moved ← swapped, modified ← `patch.restore(base row)`; a patch that sets an optional field falls back to remove+insert of the base row at coordinates fixed by the delta
  itself). Per-delta constructors `Delta::insertion(index,row)`, `removal(&base,index)`, `replacement(&base,index,row)`. The algebra entry points are `commit/absorb/inverse`
  (no `apply`/`between`); `DiffAlgebra::between`, `Edit::between`, `diff_patch` differencing (now `None`) and `PatchRow::diff_row` are gone.
- Leaves: create → `Delta::insertion(index.unwrap_or(len), row)` (`index>len` → Error `mutation.target-missing`); delete/disconnect → `Delta::removal(&base, position)`;
  replace / connect-on-existing → `Delta::replacement(&base, position, row)` (same coordinate, no order list); rename → `modified`. Delete inverses (`Create/Connect{index: Some(position)}`)
  were already base-read and now sum exactly to `ProgramDiff::inverse` (`inserted` at the base index). Document replace keeps its delete+create inverse.
- Facets: diff `🔣️.json` ($defs rebuilt), `.graphql`, `.proto`, `.ts` updated (ListRemoval/ListRelocation/Insertion types). Fixtures: all 326 `🔺️diff/🔣️.json` (incl. the 66 middle-row and 6 b/k
  vectors) converted by script `gen_positional_fixtures.py`, each replay-verified in Python (old semantics replay == after snapshot == new `commit` semantics); native-roles delta fixture converted.
- Tests: algebra unit tests rewritten (four required absorb sequences + create∘patch, create∘move, move∘move, replace∘patch/delete, inverse incl. Option fallback and middle rows,
  malformed-commit refusals, 400-case randomized insert/remove/move/patch/replace chain proving absorb ≡ sequential and inverse restores); diff unit tests and registers round-trip tests
  (use `RowPatch::restore`) updated; middle-row sum-law fixtures and unit tests stand.

**Gate (4 → 0).** R9 `::apply(` (algebra fn renamed `commit`); R9 `apply_diff(` in `🧬️mutations/🦀️.rs` → `apply_program_mutation_outcome` moved to `🚪️io/🦀️.rs` `mutation_bridge`
(callers: mutate-program-1 harness, graph tool test); R12 `to_vec` clone in the old algebra (gone); R14 `ReplaceConfig` inverse → config kind redesigned: `replace-config` became
`set-config` (`SetConfig { search_query?, search_history_json?, last_result_json?, last_analysis_json? }`), whose inverse is the same kind carrying the base value of exactly the touched
fields; handlers call `snapshot(&base, next)` which names only differing fields; fixtures/contracts/oracle manifest/feature/adapter renamed.

**AMB-2/AMB-3.** Benchmark/knowledge leaves emit row deltas only; the child handles are re-derived in `ProgramDiff::apply` (`*_child_from_records`). Every inverse reads `base`
row by row; none applies or simulates a diff.

Pending when foundation is GREEN: `ARCHITECT_RUN_TESTS=1 bash T/🗑️generated/architect/run.sh` (native check, wasm check, `cargo test --lib`); expect a first compile-fix round
(the macro/algebra code and ~340 regenerated tests are desk-checked only).

## Wave 6 — one positional list delta (`protocol::list_delta`)

**Status: WRITTEN, UNVERIFIED by cargo** (foundation.status still `RED`, last line seen `RED 23:55:59 native=101 wasm=101`; no cargo call made, so nothing compiled or ran).

**Local algebra deleted.** `🧬️schema/🔺️diff/🧮️algebra/` (the `Parts`/`CollectionDelta` copy, its free `commit/absorb/inverse` fns and its unit tests incl. the 400-case randomized chain test) and the
`mod algebra` line are gone, together with the `impl_collection_delta!` macro and the shared `ListRemoval`/`ListRelocation` types. The randomized chain test is dropped, not re-pointed: the
framework's `🪡️list-delta/🧪️tests/🔬️unit` runs 6000 randomized insert/remove/move/patch sequences against the same code.

**What replaced it.**
- `🧬️schema/🔺️diff/🦀️.rs` (regenerated by `T/🗑️generated/architect/gen_diff3.py`): 66 `protocol::list_delta! { pub Program<X>Delta { removal: Program<X>Removal, insertion: Program<X>Insertion,
  relocation: Program<X>Relocation, modification: Program<X>PatchEntry, row, patch, list: Vec<row>, key: String = |row| row.header.id.0.clone() } }` invocations (traces key `row.id.0.clone()`),
  test-only serde derives passed through `$(#[$meta])*`. The wire shape is unchanged (`removed:[{id,index}]`, `inserted:[{index,row}]`, `moved:[{id,from,to}]`, `modified:[{id,patch}]`),
  so the 326 fixtures stay as they are.
- `ProgramDiff::apply` forwards the central applier's `ApplyCapability` into every `delta.commit_onto(&next.<field>, capability)` (knowledge/benchmarks re-derive their child handles afterwards);
  `absorb` / `inverse` / `is_empty` call the delta's own `absorb` / `inverse(&base.<field>)` / `is_empty` through four small macros over one collection list.
- Singleton sections (`ProgramMetaEdit`, `ProjectDefinitionEdit`, `GovernanceEdit`) keep `{set, patch}`. Because a patch is now written only under the capability (`RowPatch::commit_into`), a later patch no longer
  folds into an earlier `set`: `compose` keeps both (`set` applied first, `patch` after; patch∘patch via `RowPatch::absorb`), `undo` is `replacing(base)` when `set` is present, else `patching(patch.inverse(base))`.
- `🧬️schema/🗄️registers/🦀️.rs`: `impl_patchable!` now implements the framework `RowPatch<Entity>` (`commit_into`, later-wins `absorb`, `inverse` capturing exactly the carried fields, `is_empty`); the
  local `RowPatch`/`Patchable` traits, `diff_row`/`diff_patch`/`restore` are gone. The 65 patch round-trip tests commit through `protocol::apply_diff` over a one-row program (`patch_through_program!`/`patch_through_table!`).
- Leaves: replace / connect-on-existing build `let mut delta = Delta::removal(&base.c, position); delta.absorb(Delta::insertion(position, row));` (66 files; was `Delta::replacement`).
- Facets (`🔣️.json`, `.graphql`, `.proto`, `.ts`) rebuilt by `gen_facets3.py`: per-register `Program<X>Removal` / `Program<X>Relocation` replace the shared `ListRemoval` / `ListRelocation`; the unused `ProgramStringList` is gone.
- Diff unit tests: the four absorb sequences (patch∘patch, create∘delete, delete∘create, patch∘delete) stay, now with `ProgramStakeholdersRemoval`; the section test asserts that `set` and `patch` both survive composition;
  new test `a_middle_row_removal_inverts_to_a_reinsertion_at_its_base_index` (three rows, delete row 1, `inverse(&base)` inserts at index 1, `apply_diff` restores the base, inverse law).

**Known limitation (documented, not worked around).** A patch field is `Option<T>`, so a patch cannot clear an optional field. The earlier Option-fallback inside the local `inverse` (remove+insert of the base row) is lost with the
local copy; replace-*/connect-on-existing already travel as whole-row replacement, so only rename (non-optional `name`) produces patches and the limitation is not reachable from a leaf.

**Gate.** `bun ./📜️script.ts verify mutation-outcome-law` (run after all Wave 6 edits): 1 breach in the whole repo, an R12 in `📏️layout/…/🔺️diff/🦀️.rs:757` (not architect); **0 under `🏛️architect`**.
