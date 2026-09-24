# WP-T12: Outcome Switch Landing, T11 Frozen Follow-Ups, Inventory And Lib-Test Debt

Slice: T12 (session 11). Captures: **`.🧬semio/🌐hub/s11-t12-captures/`** (re-measured; see below). Inputs: `.tmp-ticket/wp-t12/*`. Ports 8060–8069 / 6560–6569.

> ⚠️ **12:19–12:35 low-disk sweep.** An external cleanup deleted the whole `.tmp-ticket/wp-t12/` folder (inputs, `generated/`, `backup/`, `target/`). The 43 inputs were restored byte-exact from the git index (read with `git show :path`, no git state touched); the jack additions made after the last auto-stage (vectors 35–38, decisions, harness arms) were rebuilt from the committed fixtures and this report. `generated/editor-survey.json` (derived from T11's deleted `wp-t11/generated/editor-paths-0.json`) is gone and not rebuildable; its registrations are committed. Every capture this report cites is re-measured by `wp-t12/remeasure.sh` into `.🧬semio/🌐hub/s11-t12-captures/<step>.txt` (started 12:47); capture names below that end in `-N.txt` refer to the lost originals, their re-measurement is the step of the same topic in that folder.
Inherits: T10 (`📓️wp-t10.md`, `wp-t10/`), T11 (`📓️wp-t11.md`, `wp-t11/`), T5, T8, T9.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. **S15 guest defects (coordinator priority, 07:1x)**: stdio ×9, curation ×2, trinity ×2, norm ×2, gis viewer, generation2d viewer, vcs viewer | **all fixed at the root, each with a law** (§S): stdio kit verbs registered + artifact publication authority (9 live laws), curation numeric args, norm snapshot decode, generation2d viewer owners (+6 preventive viewers, + draw), gis viewer camera lane, vcs viewer document row, trinity patchNodes/rail verbs. Native lib tests green per crate; **one wasm32-wasip2 check of all 15 touched guest plugins EXIT 0 (95 s, fleet mutex)**. Every package recorded in `wp-w1/requests/t12.txt` (items 4–9) | `wp-s15` matrix, `wp-w1/requests/s15.txt`, `check-s15-wasm-1.txt` |
| 1a. Landing: T10 outcome switch (`switch.py` + `manifest-align.py --write`), compile-atomic | **landed, green** (§1). Standalone bridges: the check sweep was stopped (5/37 green) because every bridge is regenerated for state-lane surfaces (§3) and rebuilt by the next inventory | `switch-apply.txt`, `check-switch-1.txt`, `check-switch-wasm-1.txt`, `tsc-switch-2.txt`, `check-switch-bridges-2.txt` |
| 1b. Landing: T11 frozen follow-ups | **landed, green, "T12 landing done" 01:08** (§2): binary one name `replace-byte-range`; dwg ac1018 duplicate leaves deleted; jpg/tiff baseline leaf fields `pub`. W2 messaged | `check-followups-1.txt`, `check-followups-oracle-1.txt`, `check-followups-wasm-1.txt` |
| 2. pdf 1.7 base runtime-only 44 | pending | |
| 3. Editor/viewer state-lane vocabularies (T11 §1: 39 real) | **in progress** (§3). Vectors: **86/86 measured, 0 findings** after the fix below (`editor-measure-2.txt`). Landed + checked: kernel `mutation_report_json` (`check-kernel-helper-1.txt`), `law::vector` in the stdio test-oracle crate (unit tests **5/5**, `test-law-vector-1.txt`). Harness built; vectors measured. **Bug found and fixed:** architect presence/config and imperative config wiped their record to defaults on an identical re-apply (§3a; `check-noop-1.txt`, W2 request filed). Not yet: registration generator run, bridges regenerated, cases run; trinity's 5 wait on a taxonomy move + private paths | `editor-survey.json`, `editor-vectors.json`, `editor-measure-1.txt` |
| 4. Inventories + contract rerun (target 0 high) | **in progress** (§4). Contract high rows **141 → 126** so far (12:51): state-lane fixtures keyed by surface (8 wildcard rows gone), stdio step cc6's stale hand-written subset bridge removed (3 outcome rows), gisterrain terrain config declared (capability without manifest gone), generation2d update-camera fixture re-copied byte-verbatim from its physical vector (2 digest rows), trinity jack's 4 state lanes registered. Remaining 126 split by owner in §4 | `contract` / `inventory` steps in the captures folder, `contract-4-rows-1251.json` |
| 5. remodeling wall-clock laws (5) | **fixed by root cause, 1301/1301 under load 18–21** (§5) | `lib-remodel-1.txt` (before: 2 red), `lib-remodel-serial-1.txt`, `remodel-probe-1.txt`, `lib-remodel-2.txt` |
| 6. raster concurrency flake | **fixed by root cause, 5/5 runs 228/228** (was 227/1): standalone control credits come from an explicit `RasterStandaloneControlPool`; production uses the process pool, each saturation law owns a private pool, and a snapshot root hands its pool to the inner retirement (`wp-t12/raster-pool.py`) | `check-raster-pool-1.txt`, `lib-raster-1.txt`, `lib-raster-2-5.txt` |
| 7. Lib tests of every touched plugin | pending | |
| 8. Coordinator add-on: per-plugin `test quick` for gis, lowpoly, mathematical, wfc, fem, architect (audit P1-7) | **done** — **gis green** (native codec receipts 2 + hostile 8; nextest 10/10; 758 s, almost all build; the 09-23 document-id fixture drift is gone). **lowpoly red, 2/3: `descriptor_is_fresh`** — the committed `🛂️.descriptor.semio` (09-24 16:32) differs from the live one in 4 bytes: `semantics.effects.destructive` of two actions and one `policy.approval` (peer action-semantics edits since; the 09-23 `ENOENT .tmp-wp-c3` is gone). Regenerating descriptors is W2's describe-all, so it is left to the rebuild. **mathematical green** (nextest 4/4, 623 s; the 09-23 budget timeout was build time). **wfc 27/28: `descriptor_is_fresh`** — the committed descriptor predates G10's landed `InferenceCommitBinding` (the inference `payload` object gained a field: 6 vs 5 members at byte 13 184); same class as lowpoly, refreshed by W2's describe-all. **fem green** (5/5, 545 s), **architect green** (3/3, 355 s). Net: 4/6 green; the 09-23 budget timeouts were build time under fleet load, not tests; lowpoly/wfc wait only on W2's descriptor refresh | `test-quick-<plugin>.txt` |

## S. S15 guest defects

W2 release order (w2-final.sh): stdio, gis, animate (done .ok), architect (queued), block … trinity vcs wfc writer. Rule: edit a package only before its `START release` or after its `.ok`.

| defect | package (release state at edit) | root cause | fix | law | checks |
|---|---|---|---|---|---|
| sourcing/curation, demonstrator/curation: `curationSetCount` journals a row, no edit | sourcing-curation (sourcing + demonstrator not started) | the manifest declared `delta`/`value` as TEXT args; the rail staged `"1"`, the bridge reads numbers only → `NoOp` | `ActionArgDef::number` for both | `the_rail_stages_numeric_curation_counts_into_one_curated_edit` (schema is Number + numeric staging reaches the command) | native `--lib --tests` EXIT 0; law 3/3 with the arg-bridge laws (`check-curation-1.txt`, `test-curation-1.txt`) |
| procedural/generation2d viewer: guest panic `ordered-map root must be explicitly retired before drop` | procedural-generation2d (procedural not started) | the viewer did not declare `build_document_store_owners`, so the framework's generic bounded owners dropped the snapshot's `OrderedMap` roots plainly; the editor declares the artifact's own owners. Same gap found in 6 more viewers (raster, process3d, wires, writer, jack, rewriting; draw deferred: its catalog-b release is running) | every one of those viewers declares its artifact's owner catalogue (one function each) | `the_viewer_opens_and_closes_its_document_through_the_artifacts_owners`: **red before (the exact guest panic reproduced natively), green after** | native `--lib --tests` 7 crates EXIT 0; viewer-filtered lib tests 5+7+7+7+4+4+5 all pass (`test-gen2d-viewer-before/after.txt`, `check-viewers-1/2.txt`, `test-viewers-1.txt`) |
| norm din18599, en1990: `setSnapshot` refused `expected Enum/Float, found Absent at 1:1` | norm contract + 2 editors (norm not started) | manifest declares `snapshot` = document JSON (like 13 siblings), but the `text` arm of `norm_command_from_action!` passed that JSON to the DSL-text parser | the arm takes the artifact's `decode_*_snapshot_json`, decodes the declared JSON and carries it as the document's own escaped DSL text (payload/wire unchanged); undeclared `text` key dropped; `escape_op_text_field` is production code (`wp-t12/norm-set-snapshot.py`) | `the_declared_snapshot_argument_carries_the_documents_json` ×2 (committed ➡️after fixture through the rail args) | native `--lib --tests` EXIT 0; 7/7 (`check-norm-1.txt`, `test-norm-1.txt`) |
| stdio ×9 (csv, tsv `set-cell`; txt, md, html `replace-text`; json base/i-json, xml base/valid `set-node`): refused `interactive-job.missing-factory` | stdio editors (stdio released .ok before the edit; the editor code compiles only under the stdio plugin's `component-app-assembly`, so no in-flight component build links it) | the window kits mint the three verbs as `Migrated` Mutations and each editor bridged them in `command_from_action`, but no editor registered them as retained routes, so `qualified_tool_proof` found no app/framework registration. The live law then found a second gap: no editor owned an artifact one-item publication authority, so the `Artifact` lane failed `interactive-job.publication-authority-missing` | per editor: `<X>_KIT_ACTION_ID` in the retained roster, an `Artifact`-lane publication contract, the verb in `bounded_first_step_tool_proofs!`, extent 1, the reducer and `handle` share one pure `<x>_emit`, `build_tool_job` binds the command's own id; `build_artifact_store_one_item_preparation_factory`; text kits admit 32 KiB wires (under the 64 KiB guest ceiling); md/html answer an unparseable buffer with `stdio.<x>.invalid-text` instead of an empty emit (`wp-t12/stdio-kit-verbs.py`, `stdio-kit-publication.py`, `stdio-kit-laws.py`) | `the_kit_verb_edits_the_document_through_its_exact_retained_factory` ×9: registered app, example loaded as the host applies `LoadDocument`, verb dispatched with rail-staged text args, settled through the host publication loop, read back from the snapshot (red first: `publication-authority-missing`); `replace_text_refuses_text_that_is_not_the_artifacts_dsl` (html; md parses any text as markdown) | native `cargo check -p semio-s-plugin-stdio` EXIT 0; lib tests of the 7 crates all green (`check-stdio-kit-1.txt`, `test-stdio-kit-1/2.txt`, `test-csv-kit-1.txt`) |
| vcs viewer paints an empty history tree (0 text) | vcs-vcs viewer (vcs in W2's rest batch, not started; no other component links it) | diagnosed: the viewer's only window rendered the checkpoint forest alone, and a document without checkpoints (every freshly opened viewer) is an empty roster, for which `TreeWindowKit` paints no row at all — not a projection gap | the document itself is the tree's root row (`$`, its title, the language-neutral `—` when untitled) with the checkpoint forest beneath | `a_document_without_checkpoints_still_paints_its_own_row` (row + projection text) and the updated nesting law | vcs artifact lib 126/126, `cargo check -p semio-s-plugin-vcs` EXIT 0 (`test-vcs-viewer-1.txt`) |
| gis viewer drops `setCamera` (`no window kind declares it`) | gis-gismap viewer (gis released .ok before the edit; no batch-b component links it) | the read-only map window declared no actions, but the `TiledMapHost` dispatches `setCamera` after every pan of ANY map window; the viewer had no config lane to hold a camera (`NoConfig`) | the energy viewer's pattern: the map window declares `setCamera` (Migrated view action, `camera` arg); a viewer-owned window config `gis.mapviewerwindowcfg` (`GisMapViewerWindowConfig { camera: {x,y,zoom} }`, schema-first `🔣️.json` + `set-camera` leaf) retained per window; one retained route on the WindowConfig lane only (ViewEmit untouched, `handle` refuses loudly); render publishes the retained camera or the host's fit-to-world default. **Freeze note:** additive local-only window-config record, no artifact pack/codec/ABI change | `a_dispatched_pan_is_retained_by_its_window_and_rendered_back` (registered viewer, host-shaped args, settle, render reads it back), `the_map_window_declares_the_hosts_camera_verb`, the factory join law, refusal/codec/inverse/no-op laws | `cargo check -p semio-s-plugin-gis` EXIT 0; gismap lib 285/285 (`test-gis-viewer-1/2.txt`, `test-gis-lib-1.txt`) |
| trinity jack + rewriting: `patchNodes` (live nodeIds) moves nothing; `textSelect` refused `missing start`; `nodeGraphEdit` refused `missing operationsJson` from the rail | trinity-jack, trinity-rewriting (trinity not started; writer, which embeds both, had finished its batch-b build) | `patch_nodes` answered an empty emit whenever no requested id named a live node (S15's harvested id was not a jack node id; a comma list parsed as ONE id), so the press journalled nothing and said nothing; `textSelect`/`textEdit` are the text host's gesture verbs and `nodeGraphEdit` the node-graph host's (each needs a caret range, a buffer or a `surfaceId` + `operations`), yet all three were `in_palette` rail rows — `textEdit` pressed there would even have emptied the query | `patchNodes`: empty `nodeIds` = the `ast`/`graph` selection, comma/space lists split, every non-applicable request refused by name (`mutation.target-missing`, `app.command.invalid-args`, graph-validation detail), `nodeIds` optional in the form; the three gesture verbs `in_palette: false` (still declared for their hosts), `nodeGraphEdit` out of the context menu's transform group | `patch_nodes_from_the_rail_renames_the_selection_or_the_listed_nodes` / `…patches_the_selection_or_the_listed_nodes` (registered app, real `interactionSelect`, rail-shaped args, settle, read back), the two refusal laws, `the_text_gesture_verbs_are_kept_off_the_rail`, `the_node_graph_gesture_verb_is_kept_off_the_rail_and_the_transform_group` | jack lib 216/216, rewriting lib 160/160, `cargo check -p semio-s-plugin-trinity -p semio-s-plugin-writer` EXIT 0 (`test-trinity-jack-1.txt`, `test-trinity-rewriting-1.txt`) |
| draw viewer owners (deferred until draw's `.ok`) | draw-drawing viewer (draw `.ok` 08:18, W2 idle) | the viewer used the framework's generic bounded owners instead of the drawing's own catalogue (the gap that panicked the generation2d viewer) | declares `crate::spr::drawing_document_store_owners()` like the editor | `the_viewer_opens_and_closes_its_document_through_the_artifacts_owners` — **green with and without the fix** on the default drawing (red-check `test-draw-viewer-before.txt`), so this one is preventive, not a reproduced fault | draw viewer tests 5/5 (`test-draw-viewer-1.txt`) |

## 1. T10 outcome switch

- `wp-t12/switch.py` is `wp-t10/switch.py` with its capture path redirected to `wp-t12/generated/` (T10's evidence stays intact). The dry run on the current tree: **2,544 files, 0 problems** (T10 measured 2,545 on 09-24; one leaf fewer on disk). Applied unchanged at 00:41.
- Residue scan before applying (`wp-t12/residue.py`, `residue-0.txt`): no leaf descriptor outside the T10 table speaks the severity vocabulary; every Rust site naming `MutationOutcomeClass::{Info,Warning,Error,Fatal}` is in the switch's reach (36 bridges, replication, derive, spr, 14 hand descriptors).
- `wp-t12/manifest-align.py --write` (T10's with the capture redirected): **1,898 rows aligned, 682 already agreed**; re-run afterwards: 2,580 same. The 1 unresolved row was binary `splice` (fixed in §2).
- TS: `tsc` on the test platform failed once (`outcomeClasses: [...outcomes]` was a `Set<string>`). Fix: `isMutationOutcomeClass` type guard beside `MUTATION_OUTCOME_CLASSES`; the scaffolder's outcome set is `Set<MutationOutcomeClass>`; `outcomeClassesOf` and the vector reader use the guard. `tsc-switch-2.txt`: 0 errors.
- Compile gate:
  - native `cargo check --lib`, ONE invocation, 134 `-p`: replication, dsl-derive, os-kernel, os-config, every artifact crate holding a changed leaf JSON (97, mapped by `wp-t12/crates-of.py`), 36 plugin crates. **EXIT 0 in 6 m 59 s**, 520 warning lines (`check-switch-1.txt`).
  - wasm32-wasip2 through the fleet mutex: replication, plugin stdio, gis, cad. **EXIT 0 in 3 m 32 s** (`check-switch-wasm-1.txt`).
  - `--tests` for replication + dsl-derive + os-kernel (spr unit test, derive fixture): first run hit a peer race (H9's `pending_is_empty` landed in replication mid-compile, then H9's plugin-crate edit briefly broke `semio-framework-plugin`); I stopped my loop (pids 71069/74971, mine) and reran after H9 compiled: **green** (`check-switch-bridges-2.txt`).
  - 37 standalone bridge workspaces (each its own `[workspace]`): 5 green before I stopped the sweep; every bridge is regenerated (§3) and rebuilt by the inventory.

## 2. T11 frozen follow-ups

**Binary: one name.** The leaf was `replace-byte-range` (the derive requires `semanticKind == to_kebab(variant)` and a verb-entity kind), but the wire tag (`#[value(rename = "splice")]`), DSL keyword, grammars, catalog, manifest row, fixture directory, feature, adapter and oracle said `splice`. Now everything says `replace-byte-range` / wire `replaceByteRange` / `ReplaceByteRange`; `splice` survives only for the diff's byte operation (`ByteSplice`, `BinaryDiff.splices`, `Vec::splice`). `wp-t12/binary-one-name.py`, 14 changes:
- aggregate: rename attribute and its excuse removed; `KINDS`;
- leaf: keyword `replace-byte-range`, label `("replace-byte-range", "Bytebereich ersetzen")`;
- text grammar/EBNF/ANTLR, TS union, proto enum (`REPLACE_BYTE_RANGE`);
- vocabulary unit test: the retired `splice` keyword must now fail to parse;
- catalog kinds, manifest row (`id`/`operation` `replace-byte-range`, outcomes `applied, rejected` from the leaf table), fixture `replace-byte-range-applied`, fixture dir `✂️splice` → `✂️replace-byte-range`;
- feature rows/ids (`zero-length-replacement`, …, `@id-invalid-replace-byte-range`);
- adapter: kind arms, the subject's import of the non-existent `splice` module (host compile debt) → `replace_byte_range`, and registration by Scenario Outline base id (`vector`, `invalid-replace-byte-range`) instead of hand-kept id lists;
- oracle: kind arm, helper `replace_range`, unit test names.

**dwg ac1018 duplicate leaves** (`wp-t12/dwg-ac1018-leaves.py`): AC1018's vocabulary is `pub use` of AC1024's. Deleted its `📸️set-snapshot` and `🏷️set-version-info` leaf directories and the crate-root mounts of the duplicated set-snapshot `diff`/`inverse`/`mutation` helpers (no caller). The one AC1018-specific fixture case (auxiliary save counter) moved to the vocabulary-level `🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs` (include paths re-rooted). `schema generate` rerun: the two dead scopes are gone (it also picked up two peer scopes, `framework.trace-record`, `s.stdio.json.rfc8259.geojson`).

**jpg/tiff baseline subject hosts** (`wp-t12/leaf-fields-pub.py`): leaf payload fields are the schema's public contract; the 20 `pub(crate)` fields in 15 jpg/tiff baseline leaves (exactly T11's E0451 list) are `pub`.

**Compile gate:** native `--lib --tests` stdio binary/dwg/jpg/tiff: EXIT 0 (91 warnings); stdio test-oracle crate (standalone) `--features oracles --lib --tests`: EXIT 0; wasm32-wasip2 plugin stdio + the four artifacts: EXIT 0 (2 m 25 s).

## 3. Editor/viewer state-lane vocabularies

**What exists.** T11 found 39 real editor/viewer vocabularies (config, presence, transient lanes; `wp-t12/generated/editor-survey.json`) that production dispatches through the plugin's lane stores and that no catalog registers (`unregistered-mutation-vocabulary`). T11 landed the schema half (`MutationManifest.surface`, `surfaceProblem`, surface-keyed inventory cache).

**Design (schema-first, one authority per fact):**
- **Manifest** at the surface owner (`<subset>/<surface>/🔮️oracles/🔣️.json`) with `surface: "<surface>"`; catalog with the subset's profile dirs; a surveyed no-oracle decision per lane (generic state containers — electron-store/confy, y-protocols/liveblocks, zustand/im — cannot express the typed per-kind change, no-op guard or inverse) → `justified-reference-gap` (medium) instead of a high row; one `fixture/v2` manifest per vector.
- **Vectors** as quintets under `<surface>/🧫️fixtures/<leaf>/<status-emoji><kind>-<status>/` (outside `🧫️fixtures/🧬️mutations`, so the v1 leaf-local vector registry does not claim them). 33 converted from the owners' committed `🔁️mutation-contracts.json`/`🔁️mutations.json`, 66 hand-authored (`wp-t12/editor-vectors-authored.py`); the production diff and diagnostics are measured by a ticket-local harness (`wp-t12/vector-harness`, `editor-measure.py`) and recorded only when the produced snapshot equals the hand-written after, the inverse restores the before and the diagnostics match the declared class.
- **One case per vocabulary** at the hosting subset (`<subset>/🧪️tests/<lane-emoji>mutate-<catalog>`; a case under the surface would be `case-above-subset`), registered by Scenario Outline base id (`mutate`, `inverse`, `keep` for no-op vectors).
- **Production bridge** per vocabulary, one line: `<aggregate>_report_json` → `store::os_store::test_support::mutation_report_json::<S, M>` (new kernel helper, the generic form of the gis/equation/energy hand copies). The case host links only the artifact crate and the stdio test-oracle crate, so the laws live in `semio_s_plugin_stdio_test_oracle::law::vector` (new, unit-tested).
- **Bridges** (`wp-t12/bridges.py`, derived from `wp-t5/bridges.py`): a 6-column `COORDINATES` with the surface, a 5th `list-mutations` argument, the surface echoed in the inventory, editor/viewer aggregates listed, and the document answer excluding every leaf below a `✏️editor`/`👁️viewer` segment. A dry generation into the scratchpad kept every existing aggregate row; the only dropped coordinates are the ones T10/T11 deleted on purpose (gisterrain window config, the stdio sibling subsets).
- **Platform:** `wildcard-subset-owner` / `unsplit-artifact-subset` no longer apply to state-lane manifests (their mutations change an editor surface, not a document subset); tsc 0 errors.

**Landed (11:0x).** The gis viewer's new camera lane joined the set (survey index 39, two vectors measured 88/88 with 0
findings, no-oracle decision authored like the energy viewer's). `editor-register.py` wrote **589 files for 35
vocabularies** (committed quintets, one `🔮️oracles/🔣️.json` per surface with catalog + state-lane manifest + decision +
fixture manifests, one exhaustive case per vocabulary, one `<aggregate>_report_json` bridge per aggregate);
`cargo check` of the 20 owning crates `--lib --tests` with the gating features: EXIT 0. Trinity's 5 stay unregistered
(no authored vectors: private module paths and the `🪟️window` taxonomy they sit in). `manifests.ts` now sees 200
manifests, 35 of them state lanes. `bridges.py` learned that a state-lane aggregate below a
`#[cfg(feature = "component-app-assembly")]` mount needs that feature on the bridge's dependency (gis, procedural,
fem, block, wfc bridges); all 36 bridges regenerated.

### 3a. Found by the vector harness: whole-record lanes wiped on a no-op

`ArchitectPresence`, `ArchitectConfig` and `ImperativeConfig` are whole-record lanes (the diff type IS the record, so
`apply` replaces the record). Their five leaves answered an unchanged value with `MutationOutcome::empty()` plus a warned
`mutation.no-op`; the store applies every op's diff (`replay_mutations`), so the default diff replaced the record with
its defaults — re-applying the value a collaborator already had erased their presence/search/run output. Measured: the
no-op vectors produced `{activeRegister: "elements", graphCamera…0/0/1}` instead of the unchanged record (`editor-measure-1.txt`).
Fix (`wp-t12/whole-record-noop.py`): the leaves return `MutationOutcome::new(base.clone())` with the same warning.
`cargo check -p semio-s-artifact-architect-program -p semio-s-artifact-imperative-procedure --lib --tests`: EXIT 0.
Guest rebuild requested in `wp-w1/requests/t12.txt`. The generation3d config vectors were wrong on my side (the preview
camera is `{position,target,fov}`, not six scalars) and were corrected, not the code.

## 5. Remodeling wall-clock laws

**Measured first.** The ticket-local probe (`wp-t12/remodel-probe`, production `BoundedStillDecoder`, debug build, serial)
times every PNG worker step: 512×512 uniform 519 steps, worst 2.1 ms; 4096×64 max-row 581 steps, worst 0.74 ms — the
T11 "PNG laws fail serially" no longer holds (the decoder is now the framework's `PngScanlineDecoder` at 4 096 inflate
units per step). The full lib run at load 9–12 failed 2 other laws on ONE step each (texture bake step 1574, a feature
detect microstep at 8.33 ms); both pass 3/3 run alone at the same load. So the per-step work is bounded and the red rows
were single descheduled readings.

**Root cause: the laws asserted a stricter property than the runtime.** Production's `StepOverrunLedger`
(`⏱️trace`, `SUSTAINED_OVERRUN_QUARANTINE_STEPS` = 4, documented there as the one authority every target can run,
since `wasi:clocks` and browsers have no thread-CPU clock) records every over-ceiling step and quarantines only a run of
four consecutive ones. The 29 worker-step laws asserted every single wall reading < 8 ms. They now admit each measured
step through one test helper that applies exactly that ledger rule (`engine/🧪️tests/🔬️step-ceiling`, codemod
`wp-t12/remodel-step-ceiling.py`): the ceiling stays 8 ms, a step that genuinely costs more fails after four steps, a
descheduled reading is recorded and forgotten. Law for the helper itself:
`a_descheduled_step_is_recorded_and_a_sustained_run_fails`. Trade-off stated plainly: a law whose loop has fewer than
four steps can no longer fail on one slow step — exactly as production would not quarantine it.

**Result:** `cargo test -p semio-s-artifact-remodel-remodeling --lib` 1301/1301 (load 18–21), helper law 1/1. Test-only
change (the helper is `#[cfg(test)]`); no guest rebuild needed.

## Processes (pids)

- native switch check 57805/57807 (exited 0)
- wasm switch check (mutex) exited 0
- bridge loop run 1: 71069 + cargo 74971, stopped by me (peer race)
- bridge loop run 2: 80205 + orphan cargo 89516, stopped by me (kernel `--tests` green; bridges superseded by regeneration)
- inventory run 1: 89919/89921 (+ bridge cargo 96931, rustc 99905), stopped by me after writer 4/4 (bridges are regenerated first)
- test-quick chain: 4320 (exited, ALL_DONE)
- vector-harness builds: foreground/detached, exited (build-harness-1 failed on feature-gated editor modules, -2 green)
- 03:1x network outage cut the turn; no orphaned T12 process found at 03:26

## Files changed (T12)

- Switch: 2,544 files listed in `wp-t12/switch-files.txt`; 153 manifests in `wp-t12/manifest-files.txt`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts` (guard).
- Binary: `💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/` — see §2 list.
- dwg: `🖊️dwg/🦀️.rs`, `4️⃣ac1018/…/🧬️mutations/🦀️.rs`, new `…/🧬️mutations/🧪️tests/🔬️fixture/🦀️.rs`, deleted `…/📸️set-snapshot/`, `…/🏷️set-version-info/`.
- jpg/tiff: 15 leaf `🦀️.rs` under the two `🧱️baseline` subsets.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json` (regenerated).
