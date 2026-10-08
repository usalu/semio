# 🔍️ Audit: Architect Program Mutations Against The Diff-Only Laws

Read-only audit of `✏️s/🔌️plugins/🏛️architect` (artifact `🏛️program`, plus the architect editor and window-config leaves in the same plugin). Design and violation codes: `📋️design.md`. No source file was edited, no cargo/bun build or test was run, and no git modifying command was used. Scan scripts live in `🗑️generated/architect-audit/` (`scan.py`, `classify.py`, `render.py`); their JSON outputs are `kinds.json` and `leaves.json`.

Path prefixes used below:

- `S/` = `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/`
- `P/` = `✏️s/🔌️plugins/🏛️architect/`
- `F/` = `🧰️framework/` (repo root)

## Scope And Counts

| Item | Count |
|---|---|
| `impl MutationKind<ProgramSnapshot, ProgramMutation>` leaves (`S/🧬️schema/🧬️mutations/**`) | 266 |
| `impl MutationKind<ArchitectConfig / ArchitectPresence>` leaves (`S/✏️editor/**`) | 2 |
| Hand-written `impl protocol::Mutation<W>` window-config leaves (`S/✏️editor/🎭️modes/✏️edit/🪟️windows/**`) | 4 |
| Total leaves | 272 (268 `MutationKind` + 4 hand-written `Mutation`) |
| Leaves with no V1-V3 code (clean) | 188 (all `MutationKind`) |
| Leaves with at least one V1-V3 code | 84 |

## Summary By Violation Code

| Code | Leaves | Where |
|---|---|---|
| `V1-SNAPSHOT-DIFF` | 20 | 8 child-collection, 3 singleton rename, 3 singleton replace, 2 editor config/presence, 4 window configs |
| `V1-GENERIC-DIFF` | 72 | 8 child-collection (whole collection) + 64 full-row generic patches (62 replace, 2 connect) |
| `V2-DIFF-DERIVED-INVERSE` | 0 | no inverse calls `diff(` or a shared diff-walking helper |
| `V2-RESTORE-INVERSE` | 5 | 3 singleton replace (project, governance, meta), 2 editor replace-config/presence |
| `V2-EMPTY-INVERSE` | 0 | every inverse builds at least one mutation; the `None => Vec::new()` branches (194 kinds) sit on the target-missing path where the diff itself rejects |
| `V3-LEAF-APPLY` | 11 | 8 child-collection + 3 singleton rename: a clone of base is mutated inside the leaf. No leaf calls `.apply(` and none takes `&mut ProgramSnapshot` |
| `V3-HAND-MUTATION` | 4 | 4 window configs: hand-written `impl protocol::Mutation`, bypassing `#[derive(dsl::Mutations)]`. No direct `apply` call (see note 2) |
| `V4-LAW-UNTESTED` (strict: no absorb-summed L3 test) | 272 | no test in the plugin calls `MutationDiff::absorb` on a diff |
| `V4-LAW-UNTESTED` (sequential applied-path check missing) | 6 | benchmark-record and knowledge-record delete, replace, rename: the only fixture is the rejected `🚫️absent-a` case |

Out-of-leaf findings (outside the leaf count above): 3 non-leaf apply call sites and 23 editor functions taking `&mut ProgramSnapshot`. Listed in the non-leaf table below.

## Leaf Groups

| Group | Leaves | Codes |
|---|---|---|
| child-collection: `🏁️benchmark-record` and `📚️knowledge-record` x create, delete, replace, rename | 8 | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY |
| singleton rename: `🏙️project`, `🏛️governance`, `🏷️meta` `/🏷️rename` | 3 | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY |
| singleton replace: `🏙️project`, `🏛️governance`, `🏷️meta` `/♻️replace` | 3 | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE |
| full-row generic patch: 62 `♻️replace` + `🧲️adjacency/🧲️connect` + `🧵️trace/🧵️connect` | 64 | V1-GENERIC-DIFF |
| editor whole-snapshot: `replace-config`, `replace-presence` | 2 | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE |
| window configs (register, report, graph, adjacency) | 4 | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION |
| clean: 62 create, 62 delete, 62 rename, 2 disconnect | 188 | none |

The clean kinds have sparse `*Delta` diffs built from the payload and reads of base, and concrete inverses built from base. Their inverse is never derived from the forward diff.

## Shared Generic Helpers

| Helper | file:line | Callers | Violation it carries |
|---|---|---|---|
| `program_benchmarks(snapshot)` (body: `snapshot.benchmarks_payload.clone()`) | `P/🗿️artifacts/🏛️program/🦀️.rs:106` | 4 kinds, 7 call sites (4 diff, 3 inverse) | feeds clone-then-mutate in the benchmark child-collection diffs (V1-SNAPSHOT-DIFF, V3-LEAF-APPLY) |
| `program_knowledge(snapshot)` (body: `snapshot.knowledge_payload.clone()`) | `P/🗿️artifacts/🏛️program/🦀️.rs:171` | 4 kinds, 7 call sites | same, for knowledge |
| `benchmarks_child_from_records(records)` | `P/🗿️artifacts/🏛️program/🦀️.rs:97` | 4 diff files | V1-GENERIC-DIFF (whole collection in a child handle) |
| `knowledge_child_from_records(records)` | `P/🗿️artifacts/🏛️program/🦀️.rs:163` | 4 diff files | V1-GENERIC-DIFF |
| `impl_patchable!` macro, generated `diff_patch`, `PatchRow::diff_row` | `S/🧬️schema/🗄️registers/🦀️.rs:57` (macro), `:64` (diff_patch), `:30` and `:42` (diff_row, `*out = Some(other.clone())`) | 67 entity impls; 64 leaf kinds call `diff_patch` | V1-GENERIC-DIFF: every field becomes `Some`, so each replace/connect is a full-row patch |
| `ProgramDiff::apply` via `apply_to_artifact` | `S/🧬️schema/🔺️diff/🦀️.rs:1826` and `:1596` | framework apply path; used by `io` and `editor` non-leaf sites | builds `ProgramArtifact::from_snapshot(base.clone())` and mutates it (the diff's own applier) |
| `apply_collection_delta` | `S/🧬️schema/🔺️diff/🦀️.rs:2693` | `ProgramDiff::apply` | the collection apply helper; checks at `:2701`, `:2709`, `:2722`, `:2725` |
| `absorb_register_mutation` (apply at `:840`) | `S/✏️editor/🦀️.rs:838` | 8 call sites in `upsert_relationship`, `upsert_adjacency`, `upsert_knowledge`, `upsert_benchmark` | non-leaf V3-LEAF-APPLY: `mutation.diff(program).diff().apply(program)` then `*program = next` |
| `apply_program_mutation_outcome` / `inverse_program_mutation_steps` | `S/🧬️schema/🧬️mutations/🦀️.rs:603` / `:612` | test harness `S/🧪️tests/🏛️mutate-program-1/🦀️.rs` | public seam that applies through `MutationOutcome::apply_to` (see L5) |
| `MutationOutcome::apply_to(&mut P)` | `F/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:1316` | `apply_program_mutation_outcome` | framework apply seam taking `&mut P`, reachable from outside the central applier |
| `impl_whole_record_config!` (framework `MutationDiff`: `apply` returns `self.clone()`, `absorb` replaces) | `F/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:12860` | 4 window configs | V1-SNAPSHOT-DIFF (whole record as diff) |

## Diff Types

| Diff type | file:line | Shape | `MutationDiff::apply` | `MutationDiff::absorb` | `DiffAlgebra` |
|---|---|---|---|---|---|
| `ProgramDiff` | `S/🧬️schema/🔺️diff/🦀️.rs:12` | 73 optional fields: 64 sparse `*Delta` fields (added, removed, patched, reordered), and 9 whole-value fields: `artifact` (`:14`), `schema` (`:16`), `meta` (`:18`), `project` (`:20`), `governance` (`:164`), `knowledge_payload` (`:151`), `benchmarks_payload` (`:157`), child handles `knowledge` (`:154`) and `benchmarks` (`:160`) | implemented, `:1826` | implemented, `:1829`: field-wise. Extends delta vectors, last-wins for whole fields and `reordered` | not implemented anywhere in the plugin |
| `ArchitectConfig` (as its own diff) | `S/✏️editor/🎚️config/🦀️.rs:92` | whole snapshot | implemented, `Ok(self.clone())`, ignores base | `*self = other` | no |
| `ArchitectPresence` (as its own diff) | `S/✏️editor/👥️presence/🦀️.rs:32` | whole snapshot | implemented, `Ok(self.clone())`, ignores base | `*self = other` | no |
| window config records (4) | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/*/🎚️config/🦀️.rs` at `:110`, `:114`, `:119`, `:109` (`impl_whole_record_config!`) | whole one-field record | framework macro, same as above | framework macro | no |

Framework trait facts (repo root `F/`): `MutationDiff::apply(&self, base: &P)` is a plain public trait method (`F/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:104`). No `ApplyCapability` type exists in the framework crates searched, so the L5 gate is not implemented. `DiffAlgebra` is declared (`:155`) with no implementor in the plugin.

## Law Status

| Law | Status |
|---|---|
| L1 declarative sparse diff | fails for 84 leaves: 72 V1-GENERIC-DIFF and 20 V1-SNAPSHOT-DIFF (the same 84 leaves that carry a V1 code) |
| L2 concrete inverse | fails for 5 leaves (V2-RESTORE-INVERSE). No inverse is derived from a diff |
| L3 inverse diffs sum to the negative | No test sums inverses with `absorb`. A sequential apply-and-compare check exists for 266 leaves. See the absorb concerns below |
| L4 central apply only | 11 leaves clone base and mutate it (V3-LEAF-APPLY). Non-leaf code applies diffs in 3 places and writes `&mut ProgramSnapshot` in 23 editor functions (non-leaf table) |
| L5 impossible by design | not implemented: `apply` is public, no capability type |

### Absorb Concerns (code reading, not run)

`ProgramDiff::absorb` concatenates `patched` lists (for example `:1851`, repeated for every collection) without coalescing same-id entries. `apply_collection_delta` rejects those duplicates (`:2725`, "patched more than once"). Read against the L3 `absorb` law (`absorb(d1,d2).apply(base) == d2.apply(d1.apply(base))`), the following sequences look like they fail. These are inferences from the code and have not been run:

- two patches to the same entity: duplicate entry, rejected at `:2725`;
- create then delete of the same id: `added` and `removed` both contain it, rejected by the removed-target check at `:2701`;
- delete then create of the same id: rejected by the added-identity check at `:2709`;
- patch then delete of the same id: rejected by the conflicting-target check at `:2722`.

## V4 Test Coverage Detail

- Every leaf has exactly one committed fixture directory (266 for `S/🧫️fixtures/🧬️mutations/`). The program-level exhaustive harness `S/🧪️tests/🏛️mutate-program-1/🦀️.rs:451-465` applies each kind and then its inverse steps in sequence and compares to base. It uses `apply_program_mutation_outcome`, which goes through `MutationOutcome::apply_to`, not the central applier.
- The six benchmark and knowledge delete, replace and rename leaves have only the `🚫️absent-a` fixture, so their inverse is checked only as empty. Their applied-path inverse law is untested. The create leaves have an applied `🌱️creates-a` fixture.
- The editor leaves: contract vectors for config and presence (`S/✏️editor/🎚️config/🧪️tests/🔬️contract-vectors/🦀️.rs:21`, and the presence analogue) fold the inverse over the state. The window-ownership test (`S/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🧪️tests/🔬️window-ownership/🦀️.rs:36-69`) checks only `inverse(...)[0]`.
- No `absorb` call on a diff exists in the plugin (`rg '\.absorb\('` finds only a comment at `S/🧬️schema/🔺️diff/🦀️.rs:2714`).

## Notes And Judgment Calls

1. The 64 full-row generic patches are coded V1-GENERIC-DIFF because `diff_row` writes every field. A typed full-row patch is not a sparse diff under L1. If the team accepts typed full-row patches, V1-GENERIC-DIFF drops to 8 leaves and the violating-leaf count drops from 84 to 20.
2. The four window configs are coded V3-HAND-MUTATION on the bypass criterion only. They do not call `apply`. Their diffs are whole one-field records, so they are also coded V1-SNAPSHOT-DIFF.
3. `ArchitectConfig` and `ArchitectPresence` as diff types are whole snapshots (`apply` ignores base). That is the L1 "never carries a whole after-snapshot" breach in its plainest form.
4. The non-leaf sites are outside the L4 leaf wording, but they breach the intent of L4 and L5. Decide whether the editor import path and `ArtifactBuilder` seam should move to the central applier.
5. The 266 `ProgramSnapshot` leaves were classified by rule (grep-level scan of each kind's `🔺️diff` and `↩️inverse` files, output in `kinds.json`). Representative files were read in full: validation-record create, project rename and replace, benchmark create, delete, replace and rename, knowledge create and replace, change-record replace and rename, adjacency and trace connect, adjacency disconnect, the config and presence leaves, and the four window configs. Helper bodies (`program_*`, `diff_row`, `apply_collection_delta`) were read directly.

## Violating Kinds

One row per leaf with at least one V1-V3 code (84 rows). Paths are relative to `S/`. The codes are as listed; V4 applies to every leaf (see the appendix).

| # | file:line | kind | codes | evidence |
|---|---|---|---|---|
| 1 | `S/🧬️schema/🧬️mutations/ℹ️information/♻️replace/🦀️.rs:20` | `ReplaceInformationRequirement` (ℹ️information/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 2 | `S/🧬️schema/🧬️mutations/♻️sustainability/♻️replace/🦀️.rs:20` | `ReplaceSustainabilityRequirement` (♻️sustainability/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 3 | `S/🧬️schema/🧬️mutations/♿️accessibility/♻️replace/🦀️.rs:20` | `ReplaceAccessibilityRequirement` (♿️accessibility/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 4 | `S/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🦀️.rs:20` | `ReplaceConflict` (⚔️conflict/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 5 | `S/🧬️schema/🧬️mutations/⚖️option/♻️replace/🦀️.rs:20` | `ReplaceOptionEvaluation` (⚖️option/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 6 | `S/🧬️schema/🧬️mutations/⚙️function/♻️replace/🦀️.rs:20` | `ReplaceFunction` (⚙️function/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 7 | `S/🧬️schema/🧬️mutations/⚠️risk/♻️replace/🦀️.rs:20` | `ReplaceRisk` (⚠️risk/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 8 | `S/🧬️schema/🧬️mutations/✅️decision/♻️replace/🦀️.rs:20` | `ReplaceDecision` (✅️decision/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 9 | `S/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/🦀️.rs:20` | `ReplaceValidationRecord` (✔️validation-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 10 | `S/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/🦀️.rs:20` | `ReplacePriorityRecord` (⭐️priority-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 11 | `S/🧬️schema/🧬️mutations/🌊️flow/♻️replace/🦀️.rs:20` | `ReplaceFlowRequirement` (🌊️flow/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 12 | `S/🧬️schema/🧬️mutations/🌿️environmental/♻️replace/🦀️.rs:20` | `ReplaceEnvironmentalRequirement` (🌿️environmental/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 13 | `S/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/🦀️.rs:20` | `ReplaceWorkshop` (🎓️workshop/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 14 | `S/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/🦀️.rs:20` | `ReplaceScenario` (🎬️scenario/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 15 | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🦀️.rs:20` | `ReplaceBenchmarkRecord` (🏁️benchmark-record/♻️replace) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection, `iter_mut()` overwrites the row in the clone, then emits the whole collection (`*_payload` + child handle) |
| 16 | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/🦀️.rs:19` | `CreateBenchmarkRecord` (🏁️benchmark-record/🌱️create) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection via `program_*(base)`, `push()`es the row, then emits the whole collection (`*_payload` + child handle) |
| 17 | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/🦀️.rs:20` | `RenameBenchmarkRecord` (🏁️benchmark-record/🏷️rename) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection, `iter_mut()` renames the row in the clone, then emits the whole collection (`*_payload` + child handle) |
| 18 | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/🦀️.rs:19` | `DeleteBenchmarkRecord` (🏁️benchmark-record/🗑️delete) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection, `retain()`s away the row, then emits the whole collection (`*_payload` + child handle) |
| 19 | `S/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🦀️.rs:20` | `ReplaceActivity` (🏃️activity/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 20 | `S/🧬️schema/🧬️mutations/🏗️infrastructure/♻️replace/🦀️.rs:20` | `ReplaceInfrastructureRequirement` (🏗️infrastructure/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 21 | `S/🧬️schema/🧬️mutations/🏙️project/♻️replace/🦀️.rs:19` | `ReplaceProject` (🏙️project/♻️replace) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff emits the whole singleton section (`ProgramDiff.<section> = Some(payload.<section>.clone())`); inverse restores the whole section from `base.<section>.clone()` |
| 22 | `S/🧬️schema/🧬️mutations/🏙️project/🏷️rename/🦀️.rs:18` | `RenameProject` (🏙️project/🏷️rename) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | `let mut value = base.<singleton>.clone()` mutated in the leaf, then emitted as the whole singleton section |
| 23 | `S/🧬️schema/🧬️mutations/🏛️governance/♻️replace/🦀️.rs:19` | `ReplaceGovernance` (🏛️governance/♻️replace) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff emits the whole singleton section (`ProgramDiff.<section> = Some(payload.<section>.clone())`); inverse restores the whole section from `base.<section>.clone()` |
| 24 | `S/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/🦀️.rs:18` | `RenameGovernance` (🏛️governance/🏷️rename) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | `let mut value = base.<singleton>.clone()` mutated in the leaf, then emitted as the whole singleton section |
| 25 | `S/🧬️schema/🧬️mutations/🏢️organizational/♻️replace/🦀️.rs:20` | `ReplaceOrganizationalRequirement` (🏢️organizational/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 26 | `S/🧬️schema/🧬️mutations/🏷️meta/♻️replace/🦀️.rs:19` | `ReplaceMeta` (🏷️meta/♻️replace) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff emits the whole singleton section (`ProgramDiff.<section> = Some(payload.<section>.clone())`); inverse restores the whole section from `base.<section>.clone()` |
| 27 | `S/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/🦀️.rs:18` | `RenameMeta` (🏷️meta/🏷️rename) | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | `let mut value = base.<singleton>.clone()` mutated in the leaf, then emitted as the whole singleton section |
| 28 | `S/🧬️schema/🧬️mutations/🐛️issue/♻️replace/🦀️.rs:20` | `ReplaceIssue` (🐛️issue/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 29 | `S/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🦀️.rs:20` | `ReplaceApprovalRecord` (👍️approval-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 30 | `S/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/🦀️.rs:20` | `ReplaceStakeholder` (👥️stakeholder/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 31 | `S/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/🦀️.rs:20` | `ReplaceQualityRecord` (💎️quality-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 32 | `S/🧬️schema/🧬️mutations/💪️resilience/♻️replace/🦀️.rs:20` | `ReplaceResilienceRequirement` (💪️resilience/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 33 | `S/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🦀️.rs:20` | `ReplaceAssumption` (💭️assumption/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 34 | `S/🧬️schema/🧬️mutations/💰️cost/♻️replace/🦀️.rs:20` | `ReplaceCostRequirement` (💰️cost/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 35 | `S/🧬️schema/🧬️mutations/📃️document/♻️replace/🦀️.rs:20` | `ReplaceDocument` (📃️document/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 36 | `S/🧬️schema/🧬️mutations/📅️schedule/♻️replace/🦀️.rs:20` | `ReplaceScheduleRequirement` (📅️schedule/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 37 | `S/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/🦀️.rs:20` | `ReplaceGrowthPlan` (📈️growth-plan/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 38 | `S/🧬️schema/🧬️mutations/📊️performance/♻️replace/🦀️.rs:20` | `ReplacePerformanceCriterion` (📊️performance/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 39 | `S/🧬️schema/🧬️mutations/📋️operational/♻️replace/🦀️.rs:20` | `ReplaceOperationalRequirement` (📋️operational/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 40 | `S/🧬️schema/🧬️mutations/📌️requirement/♻️replace/🦀️.rs:20` | `ReplaceRequirement` (📌️requirement/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 41 | `S/🧬️schema/🧬️mutations/📍️site/♻️replace/🦀️.rs:20` | `ReplaceSiteContext` (📍️site/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 42 | `S/🧬️schema/🧬️mutations/📐️template-record/♻️replace/🦀️.rs:20` | `ReplaceTemplateRecord` (📐️template-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 43 | `S/🧬️schema/🧬️mutations/📑️report-record/♻️replace/🦀️.rs:20` | `ReplaceReportRecord` (📑️report-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 44 | `S/🧬️schema/🧬️mutations/📒️audit/♻️replace/🦀️.rs:20` | `ReplaceAuditEvent` (📒️audit/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 45 | `S/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/🦀️.rs:20` | `ReplaceKnowledgeRecord` (📚️knowledge-record/♻️replace) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection, `iter_mut()` overwrites the row in the clone, then emits the whole collection (`*_payload` + child handle) |
| 46 | `S/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/🦀️.rs:19` | `CreateKnowledgeRecord` (📚️knowledge-record/🌱️create) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection via `program_*(base)`, `push()`es the row, then emits the whole collection (`*_payload` + child handle) |
| 47 | `S/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/🦀️.rs:20` | `RenameKnowledgeRecord` (📚️knowledge-record/🏷️rename) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection, `iter_mut()` renames the row in the clone, then emits the whole collection (`*_payload` + child handle) |
| 48 | `S/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/🦀️.rs:19` | `DeleteKnowledgeRecord` (📚️knowledge-record/🗑️delete) | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | clones the whole base collection, `retain()`s away the row, then emits the whole collection (`*_payload` + child handle) |
| 49 | `S/🧬️schema/🧬️mutations/📜️regulatory/♻️replace/🦀️.rs:20` | `ReplaceRegulatoryRequirement` (📜️regulatory/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 50 | `S/🧬️schema/🧬️mutations/📡️communication/♻️replace/🦀️.rs:20` | `ReplaceCommunicationRequirement` (📡️communication/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 51 | `S/🧬️schema/🧬️mutations/📦️resource/♻️replace/🦀️.rs:20` | `ReplaceResource` (📦️resource/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 52 | `S/🧬️schema/🧬️mutations/📶️status-record/♻️replace/🦀️.rs:20` | `ReplaceStatusRecord` (📶️status-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 53 | `S/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🦀️.rs:20` | `ReplaceChangeRecord` (🔀️change-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 54 | `S/🧬️schema/🧬️mutations/🔄️process/♻️replace/🦀️.rs:20` | `ReplaceProcess` (🔄️process/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 55 | `S/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/🦀️.rs:20` | `ReplaceSearchFilter` (🔍️search-filter/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 56 | `S/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🦀️.rs:20` | `ReplaceAccessRule` (🔑️access-rule/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 57 | `S/🧬️schema/🧬️mutations/🔒️privacy/♻️replace/🦀️.rs:20` | `ReplacePrivacyRequirement` (🔒️privacy/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 58 | `S/🧬️schema/🧬️mutations/🔢️quantity/♻️replace/🦀️.rs:20` | `ReplaceQuantityRequirement` (🔢️quantity/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 59 | `S/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🦀️.rs:20` | `ReplaceAnalysisRecord` (🔬️analysis-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 60 | `S/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/🦀️.rs:20` | `ReplaceRelationship` (🕸️relationship/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 61 | `S/🧬️schema/🧬️mutations/🗄️storage/♻️replace/🦀️.rs:20` | `ReplaceStorageRequirement` (🗄️storage/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 62 | `S/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/🦀️.rs:20` | `ReplaceMeetingRecord` (🗓️meeting-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 63 | `S/🧬️schema/🧬️mutations/🗳️survey/♻️replace/🦀️.rs:20` | `ReplaceSurvey` (🗳️survey/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 64 | `S/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🦀️.rs:20` | `ReplaceDeliveryConstraint` (🚚️delivery-constraint/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 65 | `S/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🦀️.rs:20` | `ReplaceConstraintRecord` (🚧️constraint-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 66 | `S/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🦀️.rs:20` | `ReplaceComplianceRecord` (🛂️compliance-record/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 67 | `S/🧬️schema/🧬️mutations/🛎️service/♻️replace/🦀️.rs:20` | `ReplaceServiceRequirement` (🛎️service/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 68 | `S/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🦀️.rs:20` | `ReplaceEquipment` (🛠️equipment/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 69 | `S/🧬️schema/🧬️mutations/🛡️security/♻️replace/🦀️.rs:20` | `ReplaceSecurityRequirement` (🛡️security/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 70 | `S/🧬️schema/🧬️mutations/🤝️collaboration/♻️replace/🦀️.rs:20` | `ReplaceCollaborationRecord` (🤝️collaboration/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 71 | `S/🧬️schema/🧬️mutations/🦺️safety/♻️replace/🦀️.rs:20` | `ReplaceSafetyRequirement` (🦺️safety/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 72 | `S/🧬️schema/🧬️mutations/🧑️user/♻️replace/🦀️.rs:20` | `ReplaceUserProfile` (🧑️user/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 73 | `S/🧬️schema/🧬️mutations/🧠️human/♻️replace/🦀️.rs:20` | `ReplaceHumanFactorRequirement` (🧠️human/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 74 | `S/🧬️schema/🧬️mutations/🧩️flexibility/♻️replace/🦀️.rs:20` | `ReplaceFlexibilityRequirement` (🧩️flexibility/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 75 | `S/🧬️schema/🧬️mutations/🧭️wayfinding/♻️replace/🦀️.rs:20` | `ReplaceWayfindingRequirement` (🧭️wayfinding/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 76 | `S/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/🦀️.rs:20` | `ReplaceProgramElement` (🧱️program-element/♻️replace) | V1-GENERIC-DIFF | diff via `existing.diff_patch(new)`: shared `impl_patchable!`/`diff_row` writes `Some(other)` for every field (full-row patch, not sparse) |
| 77 | `S/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/🦀️.rs:20` | `ConnectAdjacency` (🧲️adjacency/🧲️connect) | V1-GENERIC-DIFF | existing row patched via `existing.diff_patch(new)`: shared `impl_patchable!` writes `Some(other)` for every field (full-row patch) |
| 78 | `S/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/🦀️.rs:19` | `ConnectTrace` (🧵️trace/🧵️connect) | V1-GENERIC-DIFF | existing row patched via `existing.diff_patch(new)`: shared `impl_patchable!` writes `Some(other)` for every field (full-row patch) |
| 79 | `S/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🦀️.rs:14` | `ReplaceConfig` (editor/config/replace) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff type is the whole config snapshot (`MutationOutcome::new(self.config.clone())`; no-op returns `base.clone()`); inverse restores the whole snapshot `ReplaceX { config: base.clone() }` |
| 80 | `S/✏️editor/👥️presence/🧬️schema/🧬️mutations/📸️replace-presence/🦀️.rs:14` | `ReplacePresence` (editor/presence/replace) | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | diff type is the whole presence snapshot (`MutationOutcome::new(self.presence.clone())`; no-op returns `base.clone()`); inverse restores the whole snapshot `ReplaceX { presence: base.clone() }` |
| 81 | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🦀️.rs:27` | `ArchitectRegisterWindowConfigMutation` (window/📋️register) | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | hand-written `impl protocol::Mutation` (bypasses `#[derive(dsl::Mutations)]`); `type Diff` is the whole one-field window config (`active_register`), delegated to the framework whole-record `MutationDiff` macro; no direct `apply` call |
| 82 | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config/🦀️.rs:22` | `ArchitectReportWindowConfigMutation` (window/📓️report) | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | hand-written `impl protocol::Mutation` (bypasses `#[derive(dsl::Mutations)]`); `type Diff` is the whole one-field window config (`selected_report_id`), delegated to the framework whole-record `MutationDiff` macro; no direct `apply` call |
| 83 | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🦀️.rs:23` | `ArchitectGraphWindowConfigMutation` (window/🕸️graph) | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | hand-written `impl protocol::Mutation` (bypasses `#[derive(dsl::Mutations)]`); `type Diff` is the whole one-field window config (`viewport`), delegated to the framework whole-record `MutationDiff` macro; no direct `apply` call |
| 84 | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config/🦀️.rs:22` | `ArchitectAdjacencyWindowConfigMutation` (window/↔️adjacency) | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | hand-written `impl protocol::Mutation` (bypasses `#[derive(dsl::Mutations)]`); `type Diff` is the whole one-field window config (`adjacency_kind_filter`), delegated to the framework whole-record `MutationDiff` macro; no direct `apply` call |

## Non-Leaf Apply Sites And Mutable Snapshot Writers


Outside the leaf count. Rows 1-3 are apply call sites. Rows 4-26 are editor functions that take `&mut ProgramSnapshot`; only `absorb_register_mutation` calls `apply`, the others write fields directly (for example `program.adjacencies.push` at `S/✏️editor/🦀️.rs:213`, `program.elements.push` at `:740`).

| # | file:line | function | what it does |
|---|---|---|---|
| 1 | `S/🚪️io/🦀️.rs:361` | `ProgramBuilderConstruction::mutate` | calls `protocol::MutationDiff::apply(outcome.diff(), &self.snapshot)` (second applier outside the central one) |
| 2 | `S/🚪️io/🦀️.rs:368` | `ProgramBuilderConstruction::absorb` | calls `<ProgramDiff as MutationDiff>::apply(&diff, &self.snapshot)` and writes `self.snapshot` |
| 3 | `S/✏️editor/🦀️.rs:840` | `absorb_register_mutation` | `mutation.diff(program).diff().apply(program)` then `*program = next` (`&mut ProgramSnapshot`, errors swallowed by `if let Ok`); 8 call sites |
| 4 | `S/✏️editor/🦀️.rs:205` | `set_adjacency` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 5 | `S/✏️editor/🦀️.rs:218` | `clear_adjacency` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 6 | `S/✏️editor/🦀️.rs:240` | `apply_template` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 7 | `S/✏️editor/🦀️.rs:556` | `build_report_and_record` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 8 | `S/✏️editor/🦀️.rs:586` | `run_analysis_and_record` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 9 | `S/✏️editor/🦀️.rs:646` | `import_registers_csv` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 10 | `S/✏️editor/🦀️.rs:670` | `import_registers_tsv` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 11 | `S/✏️editor/🦀️.rs:677` | `import_rows` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 12 | `S/✏️editor/🦀️.rs:708` | `remove_register_item` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 13 | `S/✏️editor/🦀️.rs:719` | `upsert_register_row` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 14 | `S/✏️editor/🦀️.rs:735` | `upsert_element` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 15 | `S/✏️editor/🦀️.rs:770` | `upsert_stakeholder` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 16 | `S/✏️editor/🦀️.rs:804` | `upsert_requirement` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 17 | `S/✏️editor/🦀️.rs:838` | `absorb_register_mutation` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 18 | `S/✏️editor/🦀️.rs:858` | `upsert_relationship` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 19 | `S/✏️editor/🦀️.rs:896` | `upsert_adjacency` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 20 | `S/✏️editor/🦀️.rs:934` | `upsert_knowledge` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 21 | `S/✏️editor/🦀️.rs:966` | `upsert_benchmark` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 22 | `S/✏️editor/🦀️.rs:1022` | `trace_chain` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 23 | `S/✏️editor/🦀️.rs:1047` | `trace_links_for` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 24 | `S/✏️editor/🦀️.rs:1053` | `trace_impact` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 25 | `S/✏️editor/🦀️.rs:1076` | `add_trace_link` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |
| 26 | `S/✏️editor/🦀️.rs:1081` | `embed_requirement_traces` | takes `&mut ProgramSnapshot` in editor code (direct field writes or apply; see notes) |

## Appendix: All 272 Leaves

Every leaf with its codes and V4 status. V4 strict is `no` for every row because no absorb-summed L3 test exists. Paths are relative to `S/`.

| # | leaf | file:line | group | V1-V3 codes | V4 seq. L3 (applied inverse steps vs base) | V4 strict (absorb-summed L3) |
|---|---|---|---|---|---|---|
| 1 | `ℹ️information/♻️replace` `ReplaceInformationRequirement` | `S/🧬️schema/🧬️mutations/ℹ️information/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 2 | `ℹ️information/🌱️create` `CreateInformationRequirement` | `S/🧬️schema/🧬️mutations/ℹ️information/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 3 | `ℹ️information/🏷️rename` `RenameInformationRequirement` | `S/🧬️schema/🧬️mutations/ℹ️information/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 4 | `ℹ️information/🗑️delete` `DeleteInformationRequirement` | `S/🧬️schema/🧬️mutations/ℹ️information/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 5 | `♻️sustainability/♻️replace` `ReplaceSustainabilityRequirement` | `S/🧬️schema/🧬️mutations/♻️sustainability/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 6 | `♻️sustainability/🌱️create` `CreateSustainabilityRequirement` | `S/🧬️schema/🧬️mutations/♻️sustainability/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 7 | `♻️sustainability/🏷️rename` `RenameSustainabilityRequirement` | `S/🧬️schema/🧬️mutations/♻️sustainability/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 8 | `♻️sustainability/🗑️delete` `DeleteSustainabilityRequirement` | `S/🧬️schema/🧬️mutations/♻️sustainability/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 9 | `♿️accessibility/♻️replace` `ReplaceAccessibilityRequirement` | `S/🧬️schema/🧬️mutations/♿️accessibility/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 10 | `♿️accessibility/🌱️create` `CreateAccessibilityRequirement` | `S/🧬️schema/🧬️mutations/♿️accessibility/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 11 | `♿️accessibility/🏷️rename` `RenameAccessibilityRequirement` | `S/🧬️schema/🧬️mutations/♿️accessibility/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 12 | `♿️accessibility/🗑️delete` `DeleteAccessibilityRequirement` | `S/🧬️schema/🧬️mutations/♿️accessibility/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 13 | `⚔️conflict/♻️replace` `ReplaceConflict` | `S/🧬️schema/🧬️mutations/⚔️conflict/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 14 | `⚔️conflict/🌱️create` `CreateConflict` | `S/🧬️schema/🧬️mutations/⚔️conflict/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 15 | `⚔️conflict/🏷️rename` `RenameConflict` | `S/🧬️schema/🧬️mutations/⚔️conflict/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 16 | `⚔️conflict/🗑️delete` `DeleteConflict` | `S/🧬️schema/🧬️mutations/⚔️conflict/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 17 | `⚖️option/♻️replace` `ReplaceOptionEvaluation` | `S/🧬️schema/🧬️mutations/⚖️option/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 18 | `⚖️option/🌱️create` `CreateOptionEvaluation` | `S/🧬️schema/🧬️mutations/⚖️option/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 19 | `⚖️option/🏷️rename` `RenameOptionEvaluation` | `S/🧬️schema/🧬️mutations/⚖️option/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 20 | `⚖️option/🗑️delete` `DeleteOptionEvaluation` | `S/🧬️schema/🧬️mutations/⚖️option/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 21 | `⚙️function/♻️replace` `ReplaceFunction` | `S/🧬️schema/🧬️mutations/⚙️function/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 22 | `⚙️function/🌱️create` `CreateFunction` | `S/🧬️schema/🧬️mutations/⚙️function/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 23 | `⚙️function/🏷️rename` `RenameFunction` | `S/🧬️schema/🧬️mutations/⚙️function/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 24 | `⚙️function/🗑️delete` `DeleteFunction` | `S/🧬️schema/🧬️mutations/⚙️function/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 25 | `⚠️risk/♻️replace` `ReplaceRisk` | `S/🧬️schema/🧬️mutations/⚠️risk/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 26 | `⚠️risk/🌱️create` `CreateRisk` | `S/🧬️schema/🧬️mutations/⚠️risk/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 27 | `⚠️risk/🏷️rename` `RenameRisk` | `S/🧬️schema/🧬️mutations/⚠️risk/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 28 | `⚠️risk/🗑️delete` `DeleteRisk` | `S/🧬️schema/🧬️mutations/⚠️risk/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 29 | `✅️decision/♻️replace` `ReplaceDecision` | `S/🧬️schema/🧬️mutations/✅️decision/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 30 | `✅️decision/🌱️create` `CreateDecision` | `S/🧬️schema/🧬️mutations/✅️decision/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 31 | `✅️decision/🏷️rename` `RenameDecision` | `S/🧬️schema/🧬️mutations/✅️decision/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 32 | `✅️decision/🗑️delete` `DeleteDecision` | `S/🧬️schema/🧬️mutations/✅️decision/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 33 | `✔️validation-record/♻️replace` `ReplaceValidationRecord` | `S/🧬️schema/🧬️mutations/✔️validation-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 34 | `✔️validation-record/🌱️create` `CreateValidationRecord` | `S/🧬️schema/🧬️mutations/✔️validation-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 35 | `✔️validation-record/🏷️rename` `RenameValidationRecord` | `S/🧬️schema/🧬️mutations/✔️validation-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 36 | `✔️validation-record/🗑️delete` `DeleteValidationRecord` | `S/🧬️schema/🧬️mutations/✔️validation-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 37 | `⭐️priority-record/♻️replace` `ReplacePriorityRecord` | `S/🧬️schema/🧬️mutations/⭐️priority-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 38 | `⭐️priority-record/🌱️create` `CreatePriorityRecord` | `S/🧬️schema/🧬️mutations/⭐️priority-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 39 | `⭐️priority-record/🏷️rename` `RenamePriorityRecord` | `S/🧬️schema/🧬️mutations/⭐️priority-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 40 | `⭐️priority-record/🗑️delete` `DeletePriorityRecord` | `S/🧬️schema/🧬️mutations/⭐️priority-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 41 | `🌊️flow/♻️replace` `ReplaceFlowRequirement` | `S/🧬️schema/🧬️mutations/🌊️flow/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 42 | `🌊️flow/🌱️create` `CreateFlowRequirement` | `S/🧬️schema/🧬️mutations/🌊️flow/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 43 | `🌊️flow/🏷️rename` `RenameFlowRequirement` | `S/🧬️schema/🧬️mutations/🌊️flow/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 44 | `🌊️flow/🗑️delete` `DeleteFlowRequirement` | `S/🧬️schema/🧬️mutations/🌊️flow/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 45 | `🌿️environmental/♻️replace` `ReplaceEnvironmentalRequirement` | `S/🧬️schema/🧬️mutations/🌿️environmental/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 46 | `🌿️environmental/🌱️create` `CreateEnvironmentalRequirement` | `S/🧬️schema/🧬️mutations/🌿️environmental/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 47 | `🌿️environmental/🏷️rename` `RenameEnvironmentalRequirement` | `S/🧬️schema/🧬️mutations/🌿️environmental/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 48 | `🌿️environmental/🗑️delete` `DeleteEnvironmentalRequirement` | `S/🧬️schema/🧬️mutations/🌿️environmental/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 49 | `🎓️workshop/♻️replace` `ReplaceWorkshop` | `S/🧬️schema/🧬️mutations/🎓️workshop/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 50 | `🎓️workshop/🌱️create` `CreateWorkshop` | `S/🧬️schema/🧬️mutations/🎓️workshop/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 51 | `🎓️workshop/🏷️rename` `RenameWorkshop` | `S/🧬️schema/🧬️mutations/🎓️workshop/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 52 | `🎓️workshop/🗑️delete` `DeleteWorkshop` | `S/🧬️schema/🧬️mutations/🎓️workshop/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 53 | `🎬️scenario/♻️replace` `ReplaceScenario` | `S/🧬️schema/🧬️mutations/🎬️scenario/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 54 | `🎬️scenario/🌱️create` `CreateScenario` | `S/🧬️schema/🧬️mutations/🎬️scenario/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 55 | `🎬️scenario/🏷️rename` `RenameScenario` | `S/🧬️schema/🧬️mutations/🎬️scenario/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 56 | `🎬️scenario/🗑️delete` `DeleteScenario` | `S/🧬️schema/🧬️mutations/🎬️scenario/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 57 | `🏁️benchmark-record/♻️replace` `ReplaceBenchmarkRecord` | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/♻️replace/🦀️.rs:20` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | **no** | no |
| 58 | `🏁️benchmark-record/🌱️create` `CreateBenchmarkRecord` | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/🌱️create/🦀️.rs:19` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | yes | no |
| 59 | `🏁️benchmark-record/🏷️rename` `RenameBenchmarkRecord` | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/🏷️rename/🦀️.rs:20` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | **no** | no |
| 60 | `🏁️benchmark-record/🗑️delete` `DeleteBenchmarkRecord` | `S/🧬️schema/🧬️mutations/🏁️benchmark-record/🗑️delete/🦀️.rs:19` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | **no** | no |
| 61 | `🏃️activity/♻️replace` `ReplaceActivity` | `S/🧬️schema/🧬️mutations/🏃️activity/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 62 | `🏃️activity/🌱️create` `CreateActivity` | `S/🧬️schema/🧬️mutations/🏃️activity/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 63 | `🏃️activity/🏷️rename` `RenameActivity` | `S/🧬️schema/🧬️mutations/🏃️activity/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 64 | `🏃️activity/🗑️delete` `DeleteActivity` | `S/🧬️schema/🧬️mutations/🏃️activity/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 65 | `🏗️infrastructure/♻️replace` `ReplaceInfrastructureRequirement` | `S/🧬️schema/🧬️mutations/🏗️infrastructure/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 66 | `🏗️infrastructure/🌱️create` `CreateInfrastructureRequirement` | `S/🧬️schema/🧬️mutations/🏗️infrastructure/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 67 | `🏗️infrastructure/🏷️rename` `RenameInfrastructureRequirement` | `S/🧬️schema/🧬️mutations/🏗️infrastructure/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 68 | `🏗️infrastructure/🗑️delete` `DeleteInfrastructureRequirement` | `S/🧬️schema/🧬️mutations/🏗️infrastructure/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 69 | `🏙️project/♻️replace` `ReplaceProject` | `S/🧬️schema/🧬️mutations/🏙️project/♻️replace/🦀️.rs:19` | singleton-full-replace | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | yes | no |
| 70 | `🏙️project/🏷️rename` `RenameProject` | `S/🧬️schema/🧬️mutations/🏙️project/🏷️rename/🦀️.rs:18` | singleton-clone-mutate | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | yes | no |
| 71 | `🏛️governance/♻️replace` `ReplaceGovernance` | `S/🧬️schema/🧬️mutations/🏛️governance/♻️replace/🦀️.rs:19` | singleton-full-replace | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | yes | no |
| 72 | `🏛️governance/🏷️rename` `RenameGovernance` | `S/🧬️schema/🧬️mutations/🏛️governance/🏷️rename/🦀️.rs:18` | singleton-clone-mutate | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | yes | no |
| 73 | `🏢️organizational/♻️replace` `ReplaceOrganizationalRequirement` | `S/🧬️schema/🧬️mutations/🏢️organizational/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 74 | `🏢️organizational/🌱️create` `CreateOrganizationalRequirement` | `S/🧬️schema/🧬️mutations/🏢️organizational/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 75 | `🏢️organizational/🏷️rename` `RenameOrganizationalRequirement` | `S/🧬️schema/🧬️mutations/🏢️organizational/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 76 | `🏢️organizational/🗑️delete` `DeleteOrganizationalRequirement` | `S/🧬️schema/🧬️mutations/🏢️organizational/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 77 | `🏷️meta/♻️replace` `ReplaceMeta` | `S/🧬️schema/🧬️mutations/🏷️meta/♻️replace/🦀️.rs:19` | singleton-full-replace | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | yes | no |
| 78 | `🏷️meta/🏷️rename` `RenameMeta` | `S/🧬️schema/🧬️mutations/🏷️meta/🏷️rename/🦀️.rs:18` | singleton-clone-mutate | V1-SNAPSHOT-DIFF, V3-LEAF-APPLY | yes | no |
| 79 | `🐛️issue/♻️replace` `ReplaceIssue` | `S/🧬️schema/🧬️mutations/🐛️issue/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 80 | `🐛️issue/🌱️create` `CreateIssue` | `S/🧬️schema/🧬️mutations/🐛️issue/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 81 | `🐛️issue/🏷️rename` `RenameIssue` | `S/🧬️schema/🧬️mutations/🐛️issue/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 82 | `🐛️issue/🗑️delete` `DeleteIssue` | `S/🧬️schema/🧬️mutations/🐛️issue/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 83 | `👍️approval-record/♻️replace` `ReplaceApprovalRecord` | `S/🧬️schema/🧬️mutations/👍️approval-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 84 | `👍️approval-record/🌱️create` `CreateApprovalRecord` | `S/🧬️schema/🧬️mutations/👍️approval-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 85 | `👍️approval-record/🏷️rename` `RenameApprovalRecord` | `S/🧬️schema/🧬️mutations/👍️approval-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 86 | `👍️approval-record/🗑️delete` `DeleteApprovalRecord` | `S/🧬️schema/🧬️mutations/👍️approval-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 87 | `👥️stakeholder/♻️replace` `ReplaceStakeholder` | `S/🧬️schema/🧬️mutations/👥️stakeholder/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 88 | `👥️stakeholder/🌱️create` `CreateStakeholder` | `S/🧬️schema/🧬️mutations/👥️stakeholder/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 89 | `👥️stakeholder/🏷️rename` `RenameStakeholder` | `S/🧬️schema/🧬️mutations/👥️stakeholder/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 90 | `👥️stakeholder/🗑️delete` `DeleteStakeholder` | `S/🧬️schema/🧬️mutations/👥️stakeholder/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 91 | `💎️quality-record/♻️replace` `ReplaceQualityRecord` | `S/🧬️schema/🧬️mutations/💎️quality-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 92 | `💎️quality-record/🌱️create` `CreateQualityRecord` | `S/🧬️schema/🧬️mutations/💎️quality-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 93 | `💎️quality-record/🏷️rename` `RenameQualityRecord` | `S/🧬️schema/🧬️mutations/💎️quality-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 94 | `💎️quality-record/🗑️delete` `DeleteQualityRecord` | `S/🧬️schema/🧬️mutations/💎️quality-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 95 | `💪️resilience/♻️replace` `ReplaceResilienceRequirement` | `S/🧬️schema/🧬️mutations/💪️resilience/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 96 | `💪️resilience/🌱️create` `CreateResilienceRequirement` | `S/🧬️schema/🧬️mutations/💪️resilience/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 97 | `💪️resilience/🏷️rename` `RenameResilienceRequirement` | `S/🧬️schema/🧬️mutations/💪️resilience/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 98 | `💪️resilience/🗑️delete` `DeleteResilienceRequirement` | `S/🧬️schema/🧬️mutations/💪️resilience/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 99 | `💭️assumption/♻️replace` `ReplaceAssumption` | `S/🧬️schema/🧬️mutations/💭️assumption/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 100 | `💭️assumption/🌱️create` `CreateAssumption` | `S/🧬️schema/🧬️mutations/💭️assumption/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 101 | `💭️assumption/🏷️rename` `RenameAssumption` | `S/🧬️schema/🧬️mutations/💭️assumption/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 102 | `💭️assumption/🗑️delete` `DeleteAssumption` | `S/🧬️schema/🧬️mutations/💭️assumption/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 103 | `💰️cost/♻️replace` `ReplaceCostRequirement` | `S/🧬️schema/🧬️mutations/💰️cost/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 104 | `💰️cost/🌱️create` `CreateCostRequirement` | `S/🧬️schema/🧬️mutations/💰️cost/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 105 | `💰️cost/🏷️rename` `RenameCostRequirement` | `S/🧬️schema/🧬️mutations/💰️cost/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 106 | `💰️cost/🗑️delete` `DeleteCostRequirement` | `S/🧬️schema/🧬️mutations/💰️cost/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 107 | `📃️document/♻️replace` `ReplaceDocument` | `S/🧬️schema/🧬️mutations/📃️document/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 108 | `📃️document/🌱️create` `CreateDocument` | `S/🧬️schema/🧬️mutations/📃️document/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 109 | `📃️document/🏷️rename` `RenameDocument` | `S/🧬️schema/🧬️mutations/📃️document/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 110 | `📃️document/🗑️delete` `DeleteDocument` | `S/🧬️schema/🧬️mutations/📃️document/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 111 | `📅️schedule/♻️replace` `ReplaceScheduleRequirement` | `S/🧬️schema/🧬️mutations/📅️schedule/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 112 | `📅️schedule/🌱️create` `CreateScheduleRequirement` | `S/🧬️schema/🧬️mutations/📅️schedule/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 113 | `📅️schedule/🏷️rename` `RenameScheduleRequirement` | `S/🧬️schema/🧬️mutations/📅️schedule/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 114 | `📅️schedule/🗑️delete` `DeleteScheduleRequirement` | `S/🧬️schema/🧬️mutations/📅️schedule/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 115 | `📈️growth-plan/♻️replace` `ReplaceGrowthPlan` | `S/🧬️schema/🧬️mutations/📈️growth-plan/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 116 | `📈️growth-plan/🌱️create` `CreateGrowthPlan` | `S/🧬️schema/🧬️mutations/📈️growth-plan/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 117 | `📈️growth-plan/🏷️rename` `RenameGrowthPlan` | `S/🧬️schema/🧬️mutations/📈️growth-plan/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 118 | `📈️growth-plan/🗑️delete` `DeleteGrowthPlan` | `S/🧬️schema/🧬️mutations/📈️growth-plan/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 119 | `📊️performance/♻️replace` `ReplacePerformanceCriterion` | `S/🧬️schema/🧬️mutations/📊️performance/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 120 | `📊️performance/🌱️create` `CreatePerformanceCriterion` | `S/🧬️schema/🧬️mutations/📊️performance/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 121 | `📊️performance/🏷️rename` `RenamePerformanceCriterion` | `S/🧬️schema/🧬️mutations/📊️performance/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 122 | `📊️performance/🗑️delete` `DeletePerformanceCriterion` | `S/🧬️schema/🧬️mutations/📊️performance/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 123 | `📋️operational/♻️replace` `ReplaceOperationalRequirement` | `S/🧬️schema/🧬️mutations/📋️operational/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 124 | `📋️operational/🌱️create` `CreateOperationalRequirement` | `S/🧬️schema/🧬️mutations/📋️operational/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 125 | `📋️operational/🏷️rename` `RenameOperationalRequirement` | `S/🧬️schema/🧬️mutations/📋️operational/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 126 | `📋️operational/🗑️delete` `DeleteOperationalRequirement` | `S/🧬️schema/🧬️mutations/📋️operational/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 127 | `📌️requirement/♻️replace` `ReplaceRequirement` | `S/🧬️schema/🧬️mutations/📌️requirement/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 128 | `📌️requirement/🌱️create` `CreateRequirement` | `S/🧬️schema/🧬️mutations/📌️requirement/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 129 | `📌️requirement/🏷️rename` `RenameRequirement` | `S/🧬️schema/🧬️mutations/📌️requirement/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 130 | `📌️requirement/🗑️delete` `DeleteRequirement` | `S/🧬️schema/🧬️mutations/📌️requirement/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 131 | `📍️site/♻️replace` `ReplaceSiteContext` | `S/🧬️schema/🧬️mutations/📍️site/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 132 | `📍️site/🌱️create` `CreateSiteContext` | `S/🧬️schema/🧬️mutations/📍️site/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 133 | `📍️site/🏷️rename` `RenameSiteContext` | `S/🧬️schema/🧬️mutations/📍️site/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 134 | `📍️site/🗑️delete` `DeleteSiteContext` | `S/🧬️schema/🧬️mutations/📍️site/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 135 | `📐️template-record/♻️replace` `ReplaceTemplateRecord` | `S/🧬️schema/🧬️mutations/📐️template-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 136 | `📐️template-record/🌱️create` `CreateTemplateRecord` | `S/🧬️schema/🧬️mutations/📐️template-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 137 | `📐️template-record/🏷️rename` `RenameTemplateRecord` | `S/🧬️schema/🧬️mutations/📐️template-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 138 | `📐️template-record/🗑️delete` `DeleteTemplateRecord` | `S/🧬️schema/🧬️mutations/📐️template-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 139 | `📑️report-record/♻️replace` `ReplaceReportRecord` | `S/🧬️schema/🧬️mutations/📑️report-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 140 | `📑️report-record/🌱️create` `CreateReportRecord` | `S/🧬️schema/🧬️mutations/📑️report-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 141 | `📑️report-record/🏷️rename` `RenameReportRecord` | `S/🧬️schema/🧬️mutations/📑️report-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 142 | `📑️report-record/🗑️delete` `DeleteReportRecord` | `S/🧬️schema/🧬️mutations/📑️report-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 143 | `📒️audit/♻️replace` `ReplaceAuditEvent` | `S/🧬️schema/🧬️mutations/📒️audit/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 144 | `📒️audit/🌱️create` `CreateAuditEvent` | `S/🧬️schema/🧬️mutations/📒️audit/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 145 | `📒️audit/🏷️rename` `RenameAuditEvent` | `S/🧬️schema/🧬️mutations/📒️audit/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 146 | `📒️audit/🗑️delete` `DeleteAuditEvent` | `S/🧬️schema/🧬️mutations/📒️audit/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 147 | `📚️knowledge-record/♻️replace` `ReplaceKnowledgeRecord` | `S/🧬️schema/🧬️mutations/📚️knowledge-record/♻️replace/🦀️.rs:20` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | **no** | no |
| 148 | `📚️knowledge-record/🌱️create` `CreateKnowledgeRecord` | `S/🧬️schema/🧬️mutations/📚️knowledge-record/🌱️create/🦀️.rs:19` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | yes | no |
| 149 | `📚️knowledge-record/🏷️rename` `RenameKnowledgeRecord` | `S/🧬️schema/🧬️mutations/📚️knowledge-record/🏷️rename/🦀️.rs:20` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | **no** | no |
| 150 | `📚️knowledge-record/🗑️delete` `DeleteKnowledgeRecord` | `S/🧬️schema/🧬️mutations/📚️knowledge-record/🗑️delete/🦀️.rs:19` | child-collection | V1-SNAPSHOT-DIFF, V1-GENERIC-DIFF, V3-LEAF-APPLY | **no** | no |
| 151 | `📜️regulatory/♻️replace` `ReplaceRegulatoryRequirement` | `S/🧬️schema/🧬️mutations/📜️regulatory/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 152 | `📜️regulatory/🌱️create` `CreateRegulatoryRequirement` | `S/🧬️schema/🧬️mutations/📜️regulatory/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 153 | `📜️regulatory/🏷️rename` `RenameRegulatoryRequirement` | `S/🧬️schema/🧬️mutations/📜️regulatory/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 154 | `📜️regulatory/🗑️delete` `DeleteRegulatoryRequirement` | `S/🧬️schema/🧬️mutations/📜️regulatory/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 155 | `📡️communication/♻️replace` `ReplaceCommunicationRequirement` | `S/🧬️schema/🧬️mutations/📡️communication/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 156 | `📡️communication/🌱️create` `CreateCommunicationRequirement` | `S/🧬️schema/🧬️mutations/📡️communication/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 157 | `📡️communication/🏷️rename` `RenameCommunicationRequirement` | `S/🧬️schema/🧬️mutations/📡️communication/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 158 | `📡️communication/🗑️delete` `DeleteCommunicationRequirement` | `S/🧬️schema/🧬️mutations/📡️communication/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 159 | `📦️resource/♻️replace` `ReplaceResource` | `S/🧬️schema/🧬️mutations/📦️resource/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 160 | `📦️resource/🌱️create` `CreateResource` | `S/🧬️schema/🧬️mutations/📦️resource/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 161 | `📦️resource/🏷️rename` `RenameResource` | `S/🧬️schema/🧬️mutations/📦️resource/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 162 | `📦️resource/🗑️delete` `DeleteResource` | `S/🧬️schema/🧬️mutations/📦️resource/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 163 | `📶️status-record/♻️replace` `ReplaceStatusRecord` | `S/🧬️schema/🧬️mutations/📶️status-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 164 | `📶️status-record/🌱️create` `CreateStatusRecord` | `S/🧬️schema/🧬️mutations/📶️status-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 165 | `📶️status-record/🏷️rename` `RenameStatusRecord` | `S/🧬️schema/🧬️mutations/📶️status-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 166 | `📶️status-record/🗑️delete` `DeleteStatusRecord` | `S/🧬️schema/🧬️mutations/📶️status-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 167 | `🔀️change-record/♻️replace` `ReplaceChangeRecord` | `S/🧬️schema/🧬️mutations/🔀️change-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 168 | `🔀️change-record/🌱️create` `CreateChangeRecord` | `S/🧬️schema/🧬️mutations/🔀️change-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 169 | `🔀️change-record/🏷️rename` `RenameChangeRecord` | `S/🧬️schema/🧬️mutations/🔀️change-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 170 | `🔀️change-record/🗑️delete` `DeleteChangeRecord` | `S/🧬️schema/🧬️mutations/🔀️change-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 171 | `🔄️process/♻️replace` `ReplaceProcess` | `S/🧬️schema/🧬️mutations/🔄️process/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 172 | `🔄️process/🌱️create` `CreateProcess` | `S/🧬️schema/🧬️mutations/🔄️process/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 173 | `🔄️process/🏷️rename` `RenameProcess` | `S/🧬️schema/🧬️mutations/🔄️process/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 174 | `🔄️process/🗑️delete` `DeleteProcess` | `S/🧬️schema/🧬️mutations/🔄️process/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 175 | `🔍️search-filter/♻️replace` `ReplaceSearchFilter` | `S/🧬️schema/🧬️mutations/🔍️search-filter/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 176 | `🔍️search-filter/🌱️create` `CreateSearchFilter` | `S/🧬️schema/🧬️mutations/🔍️search-filter/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 177 | `🔍️search-filter/🏷️rename` `RenameSearchFilter` | `S/🧬️schema/🧬️mutations/🔍️search-filter/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 178 | `🔍️search-filter/🗑️delete` `DeleteSearchFilter` | `S/🧬️schema/🧬️mutations/🔍️search-filter/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 179 | `🔑️access-rule/♻️replace` `ReplaceAccessRule` | `S/🧬️schema/🧬️mutations/🔑️access-rule/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 180 | `🔑️access-rule/🌱️create` `CreateAccessRule` | `S/🧬️schema/🧬️mutations/🔑️access-rule/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 181 | `🔑️access-rule/🏷️rename` `RenameAccessRule` | `S/🧬️schema/🧬️mutations/🔑️access-rule/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 182 | `🔑️access-rule/🗑️delete` `DeleteAccessRule` | `S/🧬️schema/🧬️mutations/🔑️access-rule/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 183 | `🔒️privacy/♻️replace` `ReplacePrivacyRequirement` | `S/🧬️schema/🧬️mutations/🔒️privacy/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 184 | `🔒️privacy/🌱️create` `CreatePrivacyRequirement` | `S/🧬️schema/🧬️mutations/🔒️privacy/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 185 | `🔒️privacy/🏷️rename` `RenamePrivacyRequirement` | `S/🧬️schema/🧬️mutations/🔒️privacy/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 186 | `🔒️privacy/🗑️delete` `DeletePrivacyRequirement` | `S/🧬️schema/🧬️mutations/🔒️privacy/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 187 | `🔢️quantity/♻️replace` `ReplaceQuantityRequirement` | `S/🧬️schema/🧬️mutations/🔢️quantity/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 188 | `🔢️quantity/🌱️create` `CreateQuantityRequirement` | `S/🧬️schema/🧬️mutations/🔢️quantity/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 189 | `🔢️quantity/🏷️rename` `RenameQuantityRequirement` | `S/🧬️schema/🧬️mutations/🔢️quantity/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 190 | `🔢️quantity/🗑️delete` `DeleteQuantityRequirement` | `S/🧬️schema/🧬️mutations/🔢️quantity/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 191 | `🔬️analysis-record/♻️replace` `ReplaceAnalysisRecord` | `S/🧬️schema/🧬️mutations/🔬️analysis-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 192 | `🔬️analysis-record/🌱️create` `CreateAnalysisRecord` | `S/🧬️schema/🧬️mutations/🔬️analysis-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 193 | `🔬️analysis-record/🏷️rename` `RenameAnalysisRecord` | `S/🧬️schema/🧬️mutations/🔬️analysis-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 194 | `🔬️analysis-record/🗑️delete` `DeleteAnalysisRecord` | `S/🧬️schema/🧬️mutations/🔬️analysis-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 195 | `🕸️relationship/♻️replace` `ReplaceRelationship` | `S/🧬️schema/🧬️mutations/🕸️relationship/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 196 | `🕸️relationship/🌱️create` `CreateRelationship` | `S/🧬️schema/🧬️mutations/🕸️relationship/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 197 | `🕸️relationship/🏷️rename` `RenameRelationship` | `S/🧬️schema/🧬️mutations/🕸️relationship/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 198 | `🕸️relationship/🗑️delete` `DeleteRelationship` | `S/🧬️schema/🧬️mutations/🕸️relationship/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 199 | `🗄️storage/♻️replace` `ReplaceStorageRequirement` | `S/🧬️schema/🧬️mutations/🗄️storage/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 200 | `🗄️storage/🌱️create` `CreateStorageRequirement` | `S/🧬️schema/🧬️mutations/🗄️storage/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 201 | `🗄️storage/🏷️rename` `RenameStorageRequirement` | `S/🧬️schema/🧬️mutations/🗄️storage/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 202 | `🗄️storage/🗑️delete` `DeleteStorageRequirement` | `S/🧬️schema/🧬️mutations/🗄️storage/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 203 | `🗓️meeting-record/♻️replace` `ReplaceMeetingRecord` | `S/🧬️schema/🧬️mutations/🗓️meeting-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 204 | `🗓️meeting-record/🌱️create` `CreateMeetingRecord` | `S/🧬️schema/🧬️mutations/🗓️meeting-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 205 | `🗓️meeting-record/🏷️rename` `RenameMeetingRecord` | `S/🧬️schema/🧬️mutations/🗓️meeting-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 206 | `🗓️meeting-record/🗑️delete` `DeleteMeetingRecord` | `S/🧬️schema/🧬️mutations/🗓️meeting-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 207 | `🗳️survey/♻️replace` `ReplaceSurvey` | `S/🧬️schema/🧬️mutations/🗳️survey/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 208 | `🗳️survey/🌱️create` `CreateSurvey` | `S/🧬️schema/🧬️mutations/🗳️survey/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 209 | `🗳️survey/🏷️rename` `RenameSurvey` | `S/🧬️schema/🧬️mutations/🗳️survey/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 210 | `🗳️survey/🗑️delete` `DeleteSurvey` | `S/🧬️schema/🧬️mutations/🗳️survey/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 211 | `🚚️delivery-constraint/♻️replace` `ReplaceDeliveryConstraint` | `S/🧬️schema/🧬️mutations/🚚️delivery-constraint/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 212 | `🚚️delivery-constraint/🌱️create` `CreateDeliveryConstraint` | `S/🧬️schema/🧬️mutations/🚚️delivery-constraint/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 213 | `🚚️delivery-constraint/🏷️rename` `RenameDeliveryConstraint` | `S/🧬️schema/🧬️mutations/🚚️delivery-constraint/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 214 | `🚚️delivery-constraint/🗑️delete` `DeleteDeliveryConstraint` | `S/🧬️schema/🧬️mutations/🚚️delivery-constraint/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 215 | `🚧️constraint-record/♻️replace` `ReplaceConstraintRecord` | `S/🧬️schema/🧬️mutations/🚧️constraint-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 216 | `🚧️constraint-record/🌱️create` `CreateConstraintRecord` | `S/🧬️schema/🧬️mutations/🚧️constraint-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 217 | `🚧️constraint-record/🏷️rename` `RenameConstraintRecord` | `S/🧬️schema/🧬️mutations/🚧️constraint-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 218 | `🚧️constraint-record/🗑️delete` `DeleteConstraintRecord` | `S/🧬️schema/🧬️mutations/🚧️constraint-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 219 | `🛂️compliance-record/♻️replace` `ReplaceComplianceRecord` | `S/🧬️schema/🧬️mutations/🛂️compliance-record/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 220 | `🛂️compliance-record/🌱️create` `CreateComplianceRecord` | `S/🧬️schema/🧬️mutations/🛂️compliance-record/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 221 | `🛂️compliance-record/🏷️rename` `RenameComplianceRecord` | `S/🧬️schema/🧬️mutations/🛂️compliance-record/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 222 | `🛂️compliance-record/🗑️delete` `DeleteComplianceRecord` | `S/🧬️schema/🧬️mutations/🛂️compliance-record/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 223 | `🛎️service/♻️replace` `ReplaceServiceRequirement` | `S/🧬️schema/🧬️mutations/🛎️service/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 224 | `🛎️service/🌱️create` `CreateServiceRequirement` | `S/🧬️schema/🧬️mutations/🛎️service/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 225 | `🛎️service/🏷️rename` `RenameServiceRequirement` | `S/🧬️schema/🧬️mutations/🛎️service/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 226 | `🛎️service/🗑️delete` `DeleteServiceRequirement` | `S/🧬️schema/🧬️mutations/🛎️service/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 227 | `🛠️equipment/♻️replace` `ReplaceEquipment` | `S/🧬️schema/🧬️mutations/🛠️equipment/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 228 | `🛠️equipment/🌱️create` `CreateEquipment` | `S/🧬️schema/🧬️mutations/🛠️equipment/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 229 | `🛠️equipment/🏷️rename` `RenameEquipment` | `S/🧬️schema/🧬️mutations/🛠️equipment/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 230 | `🛠️equipment/🗑️delete` `DeleteEquipment` | `S/🧬️schema/🧬️mutations/🛠️equipment/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 231 | `🛡️security/♻️replace` `ReplaceSecurityRequirement` | `S/🧬️schema/🧬️mutations/🛡️security/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 232 | `🛡️security/🌱️create` `CreateSecurityRequirement` | `S/🧬️schema/🧬️mutations/🛡️security/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 233 | `🛡️security/🏷️rename` `RenameSecurityRequirement` | `S/🧬️schema/🧬️mutations/🛡️security/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 234 | `🛡️security/🗑️delete` `DeleteSecurityRequirement` | `S/🧬️schema/🧬️mutations/🛡️security/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 235 | `🤝️collaboration/♻️replace` `ReplaceCollaborationRecord` | `S/🧬️schema/🧬️mutations/🤝️collaboration/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 236 | `🤝️collaboration/🌱️create` `CreateCollaborationRecord` | `S/🧬️schema/🧬️mutations/🤝️collaboration/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 237 | `🤝️collaboration/🏷️rename` `RenameCollaborationRecord` | `S/🧬️schema/🧬️mutations/🤝️collaboration/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 238 | `🤝️collaboration/🗑️delete` `DeleteCollaborationRecord` | `S/🧬️schema/🧬️mutations/🤝️collaboration/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 239 | `🦺️safety/♻️replace` `ReplaceSafetyRequirement` | `S/🧬️schema/🧬️mutations/🦺️safety/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 240 | `🦺️safety/🌱️create` `CreateSafetyRequirement` | `S/🧬️schema/🧬️mutations/🦺️safety/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 241 | `🦺️safety/🏷️rename` `RenameSafetyRequirement` | `S/🧬️schema/🧬️mutations/🦺️safety/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 242 | `🦺️safety/🗑️delete` `DeleteSafetyRequirement` | `S/🧬️schema/🧬️mutations/🦺️safety/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 243 | `🧑️user/♻️replace` `ReplaceUserProfile` | `S/🧬️schema/🧬️mutations/🧑️user/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 244 | `🧑️user/🌱️create` `CreateUserProfile` | `S/🧬️schema/🧬️mutations/🧑️user/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 245 | `🧑️user/🏷️rename` `RenameUserProfile` | `S/🧬️schema/🧬️mutations/🧑️user/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 246 | `🧑️user/🗑️delete` `DeleteUserProfile` | `S/🧬️schema/🧬️mutations/🧑️user/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 247 | `🧠️human/♻️replace` `ReplaceHumanFactorRequirement` | `S/🧬️schema/🧬️mutations/🧠️human/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 248 | `🧠️human/🌱️create` `CreateHumanFactorRequirement` | `S/🧬️schema/🧬️mutations/🧠️human/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 249 | `🧠️human/🏷️rename` `RenameHumanFactorRequirement` | `S/🧬️schema/🧬️mutations/🧠️human/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 250 | `🧠️human/🗑️delete` `DeleteHumanFactorRequirement` | `S/🧬️schema/🧬️mutations/🧠️human/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 251 | `🧩️flexibility/♻️replace` `ReplaceFlexibilityRequirement` | `S/🧬️schema/🧬️mutations/🧩️flexibility/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 252 | `🧩️flexibility/🌱️create` `CreateFlexibilityRequirement` | `S/🧬️schema/🧬️mutations/🧩️flexibility/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 253 | `🧩️flexibility/🏷️rename` `RenameFlexibilityRequirement` | `S/🧬️schema/🧬️mutations/🧩️flexibility/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 254 | `🧩️flexibility/🗑️delete` `DeleteFlexibilityRequirement` | `S/🧬️schema/🧬️mutations/🧩️flexibility/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 255 | `🧭️wayfinding/♻️replace` `ReplaceWayfindingRequirement` | `S/🧬️schema/🧬️mutations/🧭️wayfinding/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 256 | `🧭️wayfinding/🌱️create` `CreateWayfindingRequirement` | `S/🧬️schema/🧬️mutations/🧭️wayfinding/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 257 | `🧭️wayfinding/🏷️rename` `RenameWayfindingRequirement` | `S/🧬️schema/🧬️mutations/🧭️wayfinding/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 258 | `🧭️wayfinding/🗑️delete` `DeleteWayfindingRequirement` | `S/🧬️schema/🧬️mutations/🧭️wayfinding/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 259 | `🧱️program-element/♻️replace` `ReplaceProgramElement` | `S/🧬️schema/🧬️mutations/🧱️program-element/♻️replace/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 260 | `🧱️program-element/🌱️create` `CreateProgramElement` | `S/🧬️schema/🧬️mutations/🧱️program-element/🌱️create/🦀️.rs:19` | clean | none | yes | no |
| 261 | `🧱️program-element/🏷️rename` `RenameProgramElement` | `S/🧬️schema/🧬️mutations/🧱️program-element/🏷️rename/🦀️.rs:20` | clean | none | yes | no |
| 262 | `🧱️program-element/🗑️delete` `DeleteProgramElement` | `S/🧬️schema/🧬️mutations/🧱️program-element/🗑️delete/🦀️.rs:19` | clean | none | yes | no |
| 263 | `🧲️adjacency/🧲️connect` `ConnectAdjacency` | `S/🧬️schema/🧬️mutations/🧲️adjacency/🧲️connect/🦀️.rs:20` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 264 | `🧲️adjacency/🫷️disconnect` `DisconnectAdjacency` | `S/🧬️schema/🧬️mutations/🧲️adjacency/🫷️disconnect/🦀️.rs:19` | clean | none | yes | no |
| 265 | `🧵️trace/✂️disconnect` `DisconnectTrace` | `S/🧬️schema/🧬️mutations/🧵️trace/✂️disconnect/🦀️.rs:19` | clean | none | yes | no |
| 266 | `🧵️trace/🧵️connect` `ConnectTrace` | `S/🧬️schema/🧬️mutations/🧵️trace/🧵️connect/🦀️.rs:19` | full-row-generic-patch | V1-GENERIC-DIFF | yes | no |
| 267 | `editor/config/replace` `ReplaceConfig` | `S/✏️editor/🎚️config/🧬️schema/🧬️mutations/📸️replace-config/🦀️.rs:14` | editor-whole-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | yes | no |
| 268 | `editor/presence/replace` `ReplacePresence` | `S/✏️editor/👥️presence/🧬️schema/🧬️mutations/📸️replace-presence/🦀️.rs:14` | editor-whole-snapshot | V1-SNAPSHOT-DIFF, V2-RESTORE-INVERSE | yes | no |
| 269 | `window/📋️register` `ArchitectRegisterWindowConfigMutation` | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/📋️register/🎚️config/🦀️.rs:27` | window-hand-mutation | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | yes | no |
| 270 | `window/📓️report` `ArchitectReportWindowConfigMutation` | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/📓️report/🎚️config/🦀️.rs:22` | window-hand-mutation | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | yes | no |
| 271 | `window/🕸️graph` `ArchitectGraphWindowConfigMutation` | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🦀️.rs:23` | window-hand-mutation | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | yes | no |
| 272 | `window/↔️adjacency` `ArchitectAdjacencyWindowConfigMutation` | `S/✏️editor/🎭️modes/✏️edit/🪟️windows/↔️adjacency/🎚️config/🦀️.rs:22` | window-hand-mutation | V1-SNAPSHOT-DIFF, V3-HAND-MUTATION | yes | no |
