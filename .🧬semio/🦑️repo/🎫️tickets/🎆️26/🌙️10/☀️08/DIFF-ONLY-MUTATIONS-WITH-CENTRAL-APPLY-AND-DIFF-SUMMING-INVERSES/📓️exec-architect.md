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
  `index > len` is `Error mutation.index-out-of-range` (empty diff).
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
