# 🪞️ Wave W3-4 mirrors — remodeling non-Rust surfaces follow the landed Rust schema change

Subset root `S` = `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any`.
Rust truth mirrored: `job`/`ReconstructionJob`/`ReconstructionStage` and `replace-job` deleted; `append-content`,
`remove-content` added (variant order … `replace-qc`, `append-content`, `remove-content`, `commit-reconstruction`);
`commit-reconstruction` is a pure document mutation binding complete durable content by id; staging handles gone.
No cargo was run; no Rust file other than the named adapter was touched.

## What changed

### A. JSON Schema / GraphQL / Protobuf / TypeScript mirrors
- `S/🧬️schema/🔣️.json` — `job` property/required and `$defs.ReconstructionJob`/`ReconstructionStage` removed; `$defs.RemodelingContentKind` (sparse|mesh|image) added. Edited textually to keep a peer's odd indentation untouched.
- `S/🧬️schema/📸️snapshot/🔣️.json`, `S/🧬️schema/🔺️diff/🔣️.json` (incl. `$defs.RemodelingArtifact`) — `job` removed.
- `S/🧬️schema/🧬️mutations/🔣️.json` — `oneOf` drops `replace-job`, adds `append-content`, `remove-content` (36 refs, sorted).
- `S/🧬️schema/🧬️mutations/🏁commit-reconstruction/🧬️schema/🔣️.json` — `job` gone; `ReconstructionAssetCommit` = `{id, contentId: string|null}`.
- `S/🧬️schema/{,📸️snapshot/,🔺️diff/}🔗️.graphql` — `job` field, `ReconstructionJob`, `ReconstructionStage` removed. `S/🧬️schema/🧬️mutations/🔗️.graphql` — union/types: `ReplaceJob` removed, `AppendContent`, `RemoveContent`, `enum RemodelingContentKind` added, `CommitReconstruction` without `job`, `ReconstructionAssetCommit.contentId: String`.
- `S/🧬️schema/{,📸️snapshot/,🔺️diff/}🛰️.proto` — `job` field removed (results renumbered 9 / diff 10), `ReconstructionJob`, both `ReconstructionStage` enum+message removed. `S/🧬️schema/🧬️mutations/🛰️.proto` — oneof renumbered 27..36 with `append_content`/`remove_content`, messages `AppendContent`/`RemoveContent`, enum `RemodelingContentKind`, `CommitReconstruction` 2..7, `ReconstructionAssetCommit.content_id`.
- `S/🧬️schema/📸️snapshot/🟦️.ts` — `ReconstructionStage`/`RECONSTRUCTION_STAGES`/`ReconstructionJob`/`RECONSTRUCTION_JOB_SPEC`/snapshot `job` removed; `RemodelingContentKind` + `REMODELING_CONTENT_KINDS` added.
- `S/🧬️schema/🟦️.ts` — artifact `job` + parser + `ReconstructionStage` removed; `locale` field removed from `REMODELING_ARTIFACT_SPEC` (Rust `RemodelingArtifact` has none).
- `S/🧬️schema/🔺️diff/🟦️.ts` — `job` lane removed; stray `locale` lane removed (Rust `RemodelingDiff` has none — this alone fixed 123 pre-existing suite failures).
- `S/🧬️schema/🧬️mutations/🟦️.ts` — the TS twin: payloads/tags/specs for `appendContent`, `removeContent`, reshaped `commitReconstruction`; `RemodelingAnyMutation`, `RemodelingUnsupportedError`, `commitReconstructionOutcome`, `remodelingMutationDiff` and all staging constants deleted. New exports: `REMODELING_CONTENT_ENVELOPES`, `remodelingContentHandle(contentId, chunkCount)`, `remodelingContentHandleParts(value): [string, number] | null`, `remodelingMeshContentHandle(contentId, chunkCount)`, `remodelingMeshContentHandleParts(handle)`, `committedRemodelingAssetHandle(assetId, contentId)`, `decodeRemodelingDurableChunk(encoded): Uint8Array | null`, `remodelingContentIsComplete(store, contentId, kind, chunkCount)`. `remodelingMutationOutcome(base, mutation)` now covers all 36 tags (append/remove/commit with Rust guard order and messages, create-asset content-handle guard, replace-mesh-result `mutation.incomplete-mesh` on incomplete `remodeling-mesh-content:` handles); `applyRemodelingMutation(snapshot, mutation)`.
- `S/🧬️schema/🧪️tests/🧩️suite/🟦️.ts` — uses `remodelingMutationOutcome`; shared commit vector test no longer asserts `job`.

### B. Grammars
- Snapshot text `📖️.grammar.semio`/`🅰️.g4`/`🔤️.ebnf`: `job-block`, `job-part`, `job-field`, `stage`, `camera-poses-preview` table removed (`poses-header`/`pose-row` kept for the trajectory).
- Snapshot binary `🔠️.abnf`/`🌶️.spicy`/`🥋️.ksy`: top-level field ids now end `7 gcps, 8 results`. `📡️.protocol.semio` carries no field table (unchanged).
- Diff text `📖️.grammar.semio`/`🅰️.g4`/`🔤️.ebnf`: lanes reduced to the 10 real Rust lanes (job and the stale selection/reportTable/frameCursor/camera/layers/locale/active-utility lanes removed; `.g4` gains the missing `lane` rule).
- Mutations `📖️.grammar.semio` and `📝️text/{📖️.grammar.semio,🅰️.g4,🔤️.ebnf}`: `replace-job-op` removed; `append-content-op` (`content-id`, `kind`, optional `mime`, `width`, `height`, `first`, `chunks` list), `remove-content-op` (`content-id`, `from`); `commit-reconstruction-op` without `job`, asset table header `[id:TEXT content-id:TEXT]` rows `name {name | cell}`; `content-kind` replaces `stage`; `cell` added.
- Mutations binary `🔠️.abnf`/`🌶️.spicy`/`🥋️.ksy`: ordinal table 0..35 with `33 append-content`, `34 remove-content`, `35 commit-reconstruction`.

### C. Python reference, feature, Rust adapter
- `S/🧪️tests/📸️mutate-remodeling-1/🐍️.py` — independent `append-content` (refusal ladder invalid-content-chunk → content-gap → content-kind-mismatch → content-conflict → no-op → content-capacity; applier; inverse = `remove-content` from BASE count), `remove-content` (target-missing/content-gap/no-op; applier; inverse = `append-content` of BASE leaves), full `commit-reconstruction` (sparse/mesh/asset completeness from BASE, no-op, applier, inverse = commit of BASE lanes and bindings); own mesh-leaf resolver and content completeness; create-asset guard via content-handle parse + raster envelope; replace-mesh-result incomplete-mesh via content handle; `replace-job` gone; scenario lists updated.
- `S/🧪️tests/📸️mutate-remodeling-1/🥒️.feature` — rows: replace-job* dropped; `append-content`, `append-content-gap`, `append-content-noop`, `remove-content`, `remove-content-missing` added; `create-asset-staging-handle` → `create-asset-content-handle`, `replace-mesh-result-staged` → `replace-mesh-result-unpublished`; narrative rewritten (36 kinds, durable content paragraph, pure commit semantics, shared vector description); outline title "shared commit vector".
- `S/🧪️tests/📸️mutate-remodeling-1/🦀️.rs` — `KINDS` (36), `MUTATE_SCENARIOS`/`INVERSE_SCENARIOS` (137 each, sorted), doc comments only.

### D. Oracle registry `S/🔮️oracles/🔣️.json`
- oracle rationale/candidate reasons/`fixtureCoverage.vectors` 136 rewritten to the truth (all 36 kinds, commit ordinary, no staging); `mutationCatalogs.kinds`/`vectors` (36, new scenario directory names); `mutationManifests` (36; `append-content`/`remove-content` entries; commit variant corrected to `CommitReconstruction`); `fixtureManifests[commit-reconstruction-refused]` sha256/bytes recomputed with hashlib (before/after `sha256:89482afc…8935` 7786 B, mutation `sha256:51c2ef1f…41df5` 242 B) and notes rewritten. `noOracleDecisions[0].rationale` left verbatim (it declares itself a historical record).

### E. Stories
- `✏️s/🔌️plugins/📸️remodel/📖️stories/🧭️coordination/🧫️fixtures/🧫️model/🟦️.ts` — `RemodelJob`, `job` document lanes and header mentions removed.
- `…/🧫️scene/🟦️.ts` — stage display table removed; camera-pose layer reads `results.trajectory.poses`; `qcStages` is check/value from `results.qc`; pipeline panel = Reconstruction section: `No reconstruction run`, `Stored cameras: N`, the five tool-run chords (en/de).
- `📖️stories/🎭️panels/🧪️.story.tsx`, `📖️stories/🎭️report/🧪️.story.tsx` — story docstrings.
- The acceptance `rg` over `✏️s/🔌️plugins/📸️remodel` (excluding `.rs`) now hits only the generated plugin-level `🔣️.json`.

## Tests run (all outputs in `🗑️generated/W3-4-mirrors/`)
- `cd ✏️s/🔌️plugins/📸️remodel/📦️packages/🟦️typescript && bun ./📜️script.ts test` → `vitest-remodel-3.txt`: 1164 passed / 209 failed of 1373. HEAD baseline (`git archive` of the subset, same suite) → `vitest-baseline-head.txt`: 1018 passed / 330 failed of 1348. Every remaining failure is `re-emits the committed snapshot|diff bytes exactly`, caused by the pre-existing whole-number float lexeme drift (committed fixture bytes write `2`, `floatLexeme` writes `2.0` like `pack::json::write_float`). All decode/apply/diff/outcome/lanes/round-trip tests pass for every vector, including the 5 new ones and the 4 commit vectors.
- `python3 🐍️w3-4-mirror-check.py` → `python-mirror-check-2.txt`: 274/274 planned feature rows pass through the real Python host `Context`, registration drift none; 21/21 committed quartets of changed kinds exact (inverse restores); 28/28 synthesized cases (`🐍️w3-4-mirror-twin-probe.ts`) agree between Python reference and TS twin, 11/11 applied inverses restore.
- `bun 🐍️w3-4-mirror-schema-probe.ts` (ajv 8.20.0) → `json-validate.txt`: 274 snapshots valid against snapshot and artifact schema, 98 diffs valid, 9 new/commit payloads valid; all 6 changed JSON files parse.
- `node_modules/.bin/tsc --noEmit --skipLibCheck --strict … <changed TS>` → `tsc-changed-1.txt` vs `tsc-baseline-head.txt`: no new errors in changed code (remaining errors are pre-existing in the generated parser half of `🧬️schema/🟦️.ts`, `📸️snapshot/🟦️.ts` decoder `unknown`s, and unresolved framework imports of the story scene file).
- `bun 🐍️w3-4-mirror-story-probe.ts` → `story-probe.txt`: pipeline panel (en/de), `qcStages` rows (populated and empty) and the camera-pose layer verified at runtime.

## Commands to register in launch.json
None new (existing remodel TS test target covers the suite). Ticket probes: `python3 🐍️w3-4-mirror-check.py`, `bun 🐍️w3-4-mirror-schema-probe.ts`, `bun 🐍️w3-4-mirror-story-probe.ts`.

## Deviations, foreign edits, open items
- Foreign: removed `locale` from the TS artifact/diff specs and stale lanes from the diff text grammars (both contradicted Rust).
- Not changed: `S/🧬️schema/🧬️mutations/🏁commit-reconstruction/🔣️.json` `outcomeClasses` is still `["error","applied"]` although the Rust diff now warns `mutation.no-op` (siblings spell that `info`); left for the Rust lane because descriptors are derive-checked.
- Not changed (generated): plugin `🔣️.json`/`🛂️.descriptor.semio`, `🧰️framework/…/📚️library/🔣️schema-catalog.json` + `📓️schema-catalog.md` still list `ReconstructionJob`/`replace-job` until regenerated.
- Pre-existing: 209 byte re-emission failures (float lexeme vs committed fixture bytes); TS twin has no inverse functions (it never had).
- Rust quirk mirrored, not fixed: `append-content`'s capacity check counts overlapping leaves twice (`stored[..first+overlap]` bytes plus all payload leaf bytes).
- The TS/Python base64 decoders are more lenient than Rust's on non-canonical padding/whitespace; no vector exercises it.
