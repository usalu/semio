# WP-T12: Outcome Switch Landing, T11 Frozen Follow-Ups, Inventory And Lib-Test Debt

Slice: T12 (session 11). Captures: **`.🧬semio/🌐hub/s11-t12-captures/`** (re-measured; see below). Inputs: `.tmp-ticket/wp-t12/*`. Ports 8060–8069 / 6560–6569.

> ⚠️ **12:19–12:35 low-disk sweep.** An external cleanup deleted the whole `.tmp-ticket/wp-t12/` folder (inputs, `generated/`, `backup/`, `target/`). The 43 inputs were restored byte-exact from the git index (read with `git show :path`, no git state touched); the jack additions made after the last auto-stage (vectors 35–38, decisions, harness arms) were rebuilt from the committed fixtures and this report. `generated/editor-survey.json` (derived from T11's deleted `wp-t11/generated/editor-paths-0.json`) is gone and not rebuildable; its registrations are committed. Every capture this report cites is re-measured by `wp-t12/remeasure.sh` into `.🧬semio/🌐hub/s11-t12-captures/<step>.txt` (started 12:47); capture names below that end in `-N.txt` refer to the lost originals, their re-measurement is the step of the same topic in that folder. **Re-measurement complete: §8.**
Inherits: T10 (`📓️wp-t10.md`, `wp-t10/`), T11 (`📓️wp-t11.md`, `wp-t11/`), T5, T8, T9.

## Status

| Item | State | Evidence |
|------|-------|----------|
| 0. **S15 guest defects (coordinator priority, 07:1x)**: stdio ×9, curation ×2, trinity ×2, norm ×2, gis viewer, generation2d viewer, vcs viewer | **all fixed at the root, each with a law** (§S): stdio kit verbs registered + artifact publication authority (9 live laws), curation numeric args, norm snapshot decode, generation2d viewer owners (+6 preventive viewers, + draw), gis viewer camera lane, vcs viewer document row, trinity patchNodes/rail verbs. Native lib tests green per crate; **one wasm32-wasip2 check of all 15 touched guest plugins EXIT 0 (95 s, fleet mutex)**. Every package recorded in `wp-w1/requests/t12.txt` (items 4–9) | `wp-s15` matrix, `wp-w1/requests/s15.txt`, `check-s15-wasm-1.txt` |
| 1a. Landing: T10 outcome switch (`switch.py` + `manifest-align.py --write`), compile-atomic | **landed, green** (§1). Standalone bridges: the check sweep was stopped (5/37 green) because every bridge is regenerated for state-lane surfaces (§3) and rebuilt by the next inventory | `switch-apply.txt`, `check-switch-1.txt`, `check-switch-wasm-1.txt`, `tsc-switch-2.txt`, `check-switch-bridges-2.txt` |
| 1b. Landing: T11 frozen follow-ups | **landed, green, "T12 landing done" 01:08** (§2): binary one name `replace-byte-range`; dwg ac1018 duplicate leaves deleted; jpg/tiff baseline leaf fields `pub`. W2 messaged | `check-followups-1.txt`, `check-followups-oracle-1.txt`, `check-followups-wasm-1.txt` |
| 2. pdf 1.7 base runtime-only 44 | **done** (§6): the 44 kinds production dispatch offered without a manifest row are declared; lopdf writes one before/after pair per kind (60/60 observable, byte-reproducible, reader probe re-qualified 120/120 directions); 44 laws apply each kind, WRITE it with the subset's writer, read it back and require lopdf's after-document, then require the inverse to write back to the before-document: **44/44, crate lib 502/502**. Two laws needed two steps (a shading pattern's shading is its own collection item) — finding F3 | `pdf-lopdf-vectors-1/2/3.txt`, `pdf-lib.txt` |
| 3. Editor/viewer state-lane vocabularies (T11 §1: 39 real) | **done for 35 + gis camera; trinity rewriting blocked (F1)** (§3). Re-measured after the sweep: harness build EXIT 0, `editor-measure` EXIT 0, law::vector 5/5. Earlier state: Vectors: **86/86 measured, 0 findings** after the fix below (`editor-measure-2.txt`). Landed + checked: kernel `mutation_report_json` (`check-kernel-helper-1.txt`), `law::vector` in the stdio test-oracle crate (unit tests **5/5**, `test-law-vector-1.txt`). Harness built; vectors measured. **Bug found and fixed:** architect presence/config and imperative config wiped their record to defaults on an identical re-apply (§3a; `check-noop-1.txt`, W2 request filed). Since landed: 35 vocabularies registered (589 files), all 36 bridges regenerated, inventory clean; trinity's 5 wait on a taxonomy move + private paths (F1) | `editor-survey.json`, `editor-vectors.json`, `editor-measure-1.txt` |
| 4. Inventories + contract rerun (target 0 high) | **high 141 → 54, every T12-owned row gone** (§4). The 54 left: 31 story/benchmark fixture imports, 13 wgpu Shell test-layout rows, 8 sequence carrier-fixture rows, 1 fem3d digest (peers, attributed in §4) and trinity rewriting's window vocabulary (F1, a taxonomy decision). Medium 108: 107 surveyed `justified-reference-gap` + cad's deferral. Earlier step (12:51) was 141 → 126: state-lane fixtures keyed by surface (8 wildcard rows gone), stdio step cc6's stale hand-written subset bridge removed (3 outcome rows), gisterrain terrain config declared (capability without manifest gone), generation2d update-camera fixture re-copied byte-verbatim from its physical vector (2 digest rows), trinity jack's 4 state lanes registered. Remaining 126 split by owner in §4 | `contract` / `inventory` steps in the captures folder, `contract-4-rows-1251.json` |
| 5. remodeling wall-clock laws (5) | **fixed by root cause, 1301/1301 under load 18–21** (§5) | `lib-remodel-1.txt` (before: 2 red), `lib-remodel-serial-1.txt`, `remodel-probe-1.txt`, `lib-remodel-2.txt` |
| 6. raster concurrency flake | **fixed by root cause, 5/5 runs 228/228** (was 227/1): standalone control credits come from an explicit `RasterStandaloneControlPool`; production uses the process pool, each saturation law owns a private pool, and a snapshot root hands its pool to the inner retirement (`wp-t12/raster-pool.py`) | `check-raster-pool-1.txt`, `lib-raster-1.txt`, `lib-raster-2-5.txt` |
| 7. Lib tests of every touched plugin | **re-measured after the sweep** (`remeasure.txt`, `followup.txt` in the captures folder): see §8 for the per-crate table. One red was mine: the 00:41 outcome switch rewrote leaf `outcomeClasses` but left the literal copies in 5 `🔬️structural-correspondence` laws (curation, writer, gisterrain, imperative, space) — settled from the declarations (`wp-t12/structural-outcomes.py`) | `test-curation.txt` (red), `followup-lib-*.txt` |
| 8. Coordinator add-on: per-plugin `test quick` for gis, lowpoly, mathematical, wfc, fem, architect (audit P1-7) | **re-measured 14:3x: 6/6 green** (lowpoly and wfc after W2's descriptor refresh; gis after the fix below). gis was red once, and it was mine: `gismap_viewer_never_mutates` drove the generic `assert_viewer_never_mutates`, whose `expect("viewer adapter command succeeds")` cannot hold for a viewer whose only verb is a retained window-config write (S15's gis camera lane, `handle` refuses by design). The gis surface law now states the guarantee over the seam that decides it, exactly as the energy viewer's does: every verb the map viewer declares is `Migrated` (`followup-quick-gis.txt`, 10/10). Earlier run (03:xx): **gis green** (native codec receipts 2 + hostile 8; nextest 10/10; 758 s, almost all build; the 09-23 document-id fixture drift is gone). **lowpoly red, 2/3: `descriptor_is_fresh`** — the committed `🛂️.descriptor.semio` (09-24 16:32) differs from the live one in 4 bytes: `semantics.effects.destructive` of two actions and one `policy.approval` (peer action-semantics edits since; the 09-23 `ENOENT .tmp-wp-c3` is gone). Regenerating descriptors is W2's describe-all, so it is left to the rebuild. **mathematical green** (nextest 4/4, 623 s; the 09-23 budget timeout was build time). **wfc 27/28: `descriptor_is_fresh`** — the committed descriptor predates G10's landed `InferenceCommitBinding` (the inference `payload` object gained a field: 6 vs 5 members at byte 13 184); same class as lowpoly, refreshed by W2's describe-all. **fem green** (5/5, 545 s), **architect green** (3/3, 355 s). Net: 4/6 green; the 09-23 budget timeouts were build time under fleet load, not tests; lowpoly/wfc wait only on W2's descriptor refresh | `test-quick-<plugin>.txt` |

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

## 4. Contract: 141 → 54 high, and who owns the 54

Measured by `test-contract` (captures `contract.txt` 14:30, `followup-contract.txt`; rows `contract-rows.json`,
`followup-contract-rows.json`). What removed the 87 rows T12 owned, each by fixing the evidence or the declaration,
never by exempting a row:

| rows | fix |
|---:|---|
| 44 `runtime-only-mutation` pdf 1.7 base | §6 |
| 12 state-lane wildcard/outcome/capability rows | §3 (surface-keyed fixtures, cc6 stale bridge removed, gisterrain terrain config declared) |
| 9 `mutation-without-fixture` os.config ui-preferences | §7a: catalog `os-config-ui-preferences-1-any` over the 18 committed per-kind quintets + the claiming host case |
| 7 `fixture-generator-unregistered` jpg | Pillow generator ledger (`wp-t12/jpg-pillow-generator.py`) |
| 5 `mutation-without-fixture` cad object kinds | §7d: the fixture rule now honours the catalog's own `deferredKinds` (the debt stays visible once, as cad's medium `mutation-kinds-deferred`); F2 |
| 3 energy/remodel coverage rows | `mutationCoverageBreaches` counts a row named by a registered vector scenario id for that vector's kind |
| 3 layout `rotate-frame` | §7c |
| 2 generation2d digests | fixtures re-copied byte-verbatim from the physical vector |
| 1 remodel `mutation-kind-undeclared` (136 rows) | §7b |
| 1 trinity jack vocabulary | its 4 state lanes registered |

**The 54 that remain are not T12's:**

| rows | owner | what it is |
|---:|---|---|
| 31 `production-fixture-dependency` | remodel stories (12), fem stories (6), block stories (5), ui stories/elements (3), os dev benchmarks + `🧑‍💻dev` script (3), infinite + IconRenderHost stories (2) | `📖️stories/…/🧪️.story.tsx` and dev benchmarks import `🧫️fixtures` data; the rule wants it under `📚️examples` or out of fixtures — each owner's move |
| 13 wgpu Shell test layout | WG7/WG8 (`🐚️Shell/🧪️tests/📂️wgpu-document-relay`, `🎯️targets/🧊️wgpu/🦀️.rs:29097`) | 11 inline test bodies, 1 non-canonical `#[path]`, 1 case name |
| 8 sequence carrier fixtures | sequence owner | generator oracle `serde-json-sequence-carrier-reader` and profile `semantic-sequence-carrier-v1` referenced but not registered |
| 1 fem3d digest | fem owner | `expected.results.json` rehashed after a benchmark change |
| 1 trinity rewriting window vocabulary | trinity owner (F1) | needs a taxonomy move before it can be registered |

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

## 6. pdf 1.7 base: the 44 kinds nobody declared

Ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE widened `PdfMutation` from 16 to 60 kinds (page boxes and user unit,
content operators and annotations by index, fonts/images/forms/graphics states/shadings/patterns/colour spaces/property
lists, embedded files, outlines, named destinations, page labels, output intents, AcroForm, optional content, viewer
settings, metadata, document id, encryption, catalog extras). Production dispatch offered all 60; the manifest owned
16, and the 44 had one test each (`semantic_identity_is_owned_by_this_leaf`).

- **Declared** (schema-first): 44 manifest rows, outcomes and variant read from each leaf's own `🔣️.json`.
- **Independent evidence**: the lopdf engine (`🏭️generator/🔁️codec`, depends on `lopdf` and nothing else) got an
  arrangement and a COS edit per kind (resources on the first page, catalog entries and name trees, trailer `/ID`,
  RC4-128 encryption) and a projection that renders every page dictionary, the catalog, the trailer `/ID` and whether the
  file was encrypted. 60/60 pairs observable; the 16 existing pairs regenerate byte-identically; two consecutive runs
  diff equal after one fix — lopdf fills the 16 arbitrary `/U` padding bytes (ISO 32000-1 Algorithm 5) at random, the
  engine zeroes them through the COS API. Reader probe re-qualified: (before,before) equal 60/60, (before,after)
  unequal 60/60. 44 fixture manifests from the generator's own `manifests` command.
- **Laws** (`🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors`, #[cfg(test)]): decode lopdf's before/after through the
  subset's reader, derive the payload from what lopdf changed, apply, **write with the subset's writer, read back**,
  require lopdf's after-document on every typed lane (the retained COS carrier is writer freedom), then apply the
  inverse steps and require the before-document. **42/44 green on the first run**; set-pattern and remove-pattern needed
  two steps because the reader lifts a shading pattern's inline shading into its own collection item (`P9Shading`) —
  the laws now apply SetShading+SetPattern / RemovePattern+RemoveShading; the single-step behaviour is finding F3.
  Final: **44/44, crate lib 502/502** (`pdf-lopdf-vectors-3.txt`, `pdf-lib.txt`).
- Nothing in production changed (no stdio rebuild; `wp-w1/requests/t12.txt` item 13).

## 7. Other rows and defects closed this afternoon

**7a. os.config ui-preferences.** The 18 per-kind quintets existed; no catalog registered them and no case claimed them.
Catalog `os-config-ui-preferences-1-any` (9 kinds × `sets-*`/`keeps-*`), host case `🎨️mutate-os-config-ui-preferences`
(feature, Rust + TS adapters: each `sets-*` row must move exactly its declared member, each `keeps-*` row must raise one
`mutation.no-op` Warning and move nothing, inverses restore the before-record, a round trip reads locale, driver scale and
keybinding off the typed value), taxonomy member name. TS subject 28/28; Rust subject in §8.
**The identity sibling's TS adapter was broken**: it read `🧪️tests/<name>` fixture dirs that do not exist (6/6 errored)
and both adapters still asserted a session `token` (`session-ada-0001`, `user-ada`) the `Identity` record no longer has
(it carries `email`); both fixed against the committed record.

**7b. remodel.** `mutate-remodeling-1` named its rows `<kind>-realworld`, `-noop`, `-missing` …, which the catalog does
not declare (1 high row listing 100 scenarios). Every row now carries the registered scenario id of the vector its
`vector` column runs (`wp-t12/remodel-row-ids.py`: 136 mutate + 136 inverse rows, unambiguous and unique per role; the
vector-less commit-reconstruction rows keep their kind id), and both adapters' lists follow. Python oracle 275/275 at
exhaustive; Rust subject in §8.

**7c. layout `rotate-frame`** (added 09-23 without evidence): quintet from move-frame's committed before-document, leaf
vector test (#[cfg(test)] mount), catalog vector, two case rows, both adapters (`wp-t12/layout-rotate-frame.py`).

**7d. Test platform** (`🧪️test/🟦️.ts`, tsc 0 errors; platform suite 115/117 — the 2 reds are F7):
- `mutationCoverageBreaches`: a row named by a registered vector scenario id covers that vector's kind (exact match
  through `vectors[]`, never a prefix).
- `mutationFixtureBreaches`: a kind the capability's catalog defers is not reported a second time; law
  `a kind the capability's catalog lists in deferredKinds is left to mutation-kinds-deferred, and only that kind`
  (9/9 in its block).

**7e. My own fallout.** The 00:41 outcome switch left 17 literal outcome copies in 5 structural-correspondence laws
(§8); settled from the declarations. My S15 gis camera lane made the plugin's generic viewer law unsatisfiable (§Status 8);
restated as the energy viewer's retained-route law.

## 8. Re-measured tests (after the 12:19 sweep)

`remeasure.sh` (12:47–14:36) and `followup.sh` (15:24–) in `wp-t12/`, captures `<step>.txt` / `followup-<step>.txt`.

| check | result |
|---|---|
| native `cargo check` of the 134 switch crates `--lib --tests` | EXIT 0 |
| native check of the 16 S15 plugin crates | EXIT 0 |
| wasm32-wasip2 check: replication, cad + the 16 S15 plugins (fleet mutex) | EXIT 0 |
| tsc on the test platform + the new host case | 0 errors |
| stdio test-oracle crate `--features oracles`; `law::vector` | EXIT 0; 5/5 |
| stdio kit crates (7) | green |
| curation lib | red (outcome literals, §7e) → **154/154** |
| norm (3 crates), S15 viewers (8 crates, `viewer` filter), gismap, vcs, trinity jack + rewriting, architect + imperative | green |
| raster lib ×5 | 5/5 green |
| remodeling lib; remodel probe | green; EXIT 0 |
| vector harness build; editor measure | EXIT 0; EXIT 0 |
| writer lib; gisterrain lib; space lib | **176/176; 55/55; 51/51** |
| pdf lib; lopdf vector laws | **502/502; 44/44** |
| layout lib | red once (my rotate-frame test's include paths carried move-frame's emoji; the codemod replaced `move-frame` before `🕹️move-frame`) → fixed in file and script → **399/399** incl. the 4 rotate-frame vector laws |
| test quick gis, lowpoly, mathematical, wfc, fem, architect | 6/6 green (gis after §Status 8) |
| case `🎨️mutate-os-config-ui-preferences` (Rust + TS) | **56/56, parity 28/28** |
| case `🎚️mutate-os-config-identity` (Rust + TS) | **12/12, parity 6/6** (TS was 0/6 before §7a) |
| case `📸️mutate-remodeling-1` (Rust + Python) | **550/550, parity 275/275** |
| case `📐️mutate-layout-1` | Rust subject 53/53; Python oracle was 30/53 — 11 kinds read the retired `<leaf>/🧪️tests/<vector>` fixture paths; fixed to the feature's `🧫️fixtures` vectors → **105/106, parity 52/53** (`identity-round-trip` is subject-only by the adapters' own design: the example is `.dsl.semio` and the reference carries no DSL codec) |
| Python oracles with the same rot (`wp-t12/oracle-vector-paths.py` + hand edits) | wires 0/21 → **20/21**, space 0/9 → **8/9**, procedure 0/9 → **8/9** (also the wire's `{owner: null, slot: null}` root path), shooting 0/63 → **62/63** (also a NameError: `_leaf_root` bound `document` and read `fixture`), process3d 1/33 → **32/33**, dag 1/29 → **28/29**, home 1/3 → **2/3**, vcs 1/13 → **12/13** (plus two diff members `VcsDiff` no longer has), gismap 36/37 → **37/37** (example read through its `asset://` URI); each remaining row is the subject-only round trip. writer 6/9 → case **17/18, parity 8/9**: the snapshot's new `text` member, and `edit-text`'s no-op branch is now adjudicated because the body is carried (feature narrative updated); the last row is the placeholder-grammar refusal |
| lopdf pdf engine rebuilt; 60 pairs regenerated | identical to the committed fixtures |
| platform suite (bun) | 115/117 (F7) |

### 8b. Repo-wide oracle sweep

Because three Python references of plugins I had touched turned out 0/N (retired fixture paths), I ran every
case's reference role once (`test-oracle` at exhaustive, 506 cases, 7 090 rows) — **`📓️wp-t12-oracle-sweep.md`**
has the per-case table. After the fixes above: Python 111 cases, 37 with non-passing rows; Rust 90 / 6;
TypeScript 128 / 2. The remainder is owners' work, bucketed there (document shape drift, scenario id drift,
behaviour disagreements, declared refusals, oracle-only runs of byte-decoding references).

## Findings

- **F1 trinity rewriting** — its window vocabulary sits at `✏️editor/🪟️window/🎚️config`, outside the surface grammar
  `<✏️editor|👁️viewer>[/🎭️modes/<mode>[/🪟️windows/<window>]]/<lane>`; registering it needs the owner to move the window
  config under `🎭️modes/<mode>/🪟️windows/<window>/🎚️config` (a production taxonomy move).
- **F2 cad object kinds** — no wire vector can exist while the pane's child materialization is absent from every codec;
  the real fix is a persisted child resolver (`store::LinkResolver`/`ChildStoreFactory`), after the freeze.
- **F3 pdf set-pattern** — accepts a pattern whose shading id names no shading; the writer emits it with an
  unresolvable shading (re-read as `shading: ""`) and the outcome is still `applied` (outcomeClasses `["applied"]`).
  remove-pattern leaves the lifted `<id>Shading` behind, so a round trip writes one extra `/Shading` resource. Proposed:
  refuse (`rejected`) and remove the owned shading — leaf outcome and lift/lower decisions for the pdf owner.
- **F4** `artifact_app_laws::assert_viewer_never_mutates` cannot serve retained-route viewers (energy, gis): a refusal
  emits nothing and should satisfy "never mutates". Not edited now: every guest links that crate and W2 is restaging.
- **F5** the os.config host cases say `🎚️config` "has no crate of its own"; `semio-framework-os-config` exists since
  09-08. Moving all four cases to `🎚️config/🧪️tests` in one pass needs a taxonomy member kind for that owner.
- **F6** (earlier) ~80 stdio editors expose editable-kit verbs without bridges (bcf `set-cell` is a no-op); JSON
  `set-node` always writes strings (S15 staged `"\"S15\""`); unused imports in stdio set-snapshot inverse files; the
  Pillow ledger says 11.3.0 while the generator ran 12.2.0.
- **F7** the platform test `every committed case satisfies the frozen contract` asserts zero rows of every priority, so
  it is red on the 107 surveyed medium rows; `clean safety` hit its 5 s timeout under fleet load.
- **F8 jack's Python reference** still decodes the retired hex-member `trinity.jack.dsl v1` carrier, while the
  committed scene (`🖼️assets/🎬️demo/🗣️.dsl.semio`, 09-24) is the nested record notation: 17 rows error with
  `non-hexadecimal number found in fromhex()`, and `spec-vector-create-node` disagrees (the vector declares a
  refusal, the reference applies). A reader/printer for the record notation is the fix (iso16757/vdi3805 refuse by
  clause for the same notation).
- **F9** the oracle phase is the only place these references run; nothing in the contract notices a reference that
  reads a retired path — 9 cases read retired fixture paths (most of them 0/N). A contract rule that resolves every `shared://`/`asset://`
  URI a reference builds against the case's plan would catch the whole class statically.

## Processes (pids)

- 12:47 `remeasure.sh` (nohup, mine; ALL_DONE 14:36); 15:24 `followup.sh` (nohup, mine; ALL_DONE 16:15); both left no process.
- One overlap against my own rule: at 13:24–13:29 the pdf laws (foreground) ran while the remeasure's queued wasm32 check
  got the fleet mutex — two T12 cargos for ~5 min. No other overlap.
- The 16:0x wait loop was moved to background by the tool timeout; it only polled a file and exited.
- `inventory` in the follow-up exited 1 on ONE bridge (animate): a peer's 15:52 edit of
  `🔌️plugin/🖥️host/🧬️component-codec/🦀️.rs` did not compile at that moment (E0407/E0432); every other inventory
  row matched (0 differences) and the contract used animate's 14:28 inventory. Re-run alone at 16:3x once the
  peer's edit compiled: animate 3 surfaces, 0 differences (`inventory-animate.txt`) — the inventory is clean.

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
- S15 (§S) and state lanes (§3): see `wp-w1/requests/t12.txt` items 4–12 and `editor-register.py`'s 589 files.
- pdf (§6), under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/`:
  `🏭️generator/🔁️codec/🦀️.rs`, `🏭️generator/🔁️codec/🏗️generate/🦀️.rs`, `🏭️generator/📜️script.ts`,
  `🔮️oracles/🔣️.json`, `🧬️schema/🧬️mutations/🦀️.rs` (test mount), new `🧬️schema/🧬️mutations/🧪️tests/⚖️lopdf-vectors/🦀️.rs`,
  44 new `🧫️fixtures/<leaf>/{⬅️before,➡️after}.pdf`.
- os.config (§7a): `🧰️framework/🛍️products/💻️os/🎚️config/🔮️oracles/🔣️.json`; new
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/🎨️mutate-os-config-ui-preferences/{🥒️.feature,🦀️.rs,🟦️.ts}`;
  `…/🧪️tests/🎚️mutate-os-config-identity/{🥒️.feature,🦀️.rs,🟦️.ts}`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (one member name).
- remodel (§7b): `✏️s/🔌️plugins/📸️remodel/…/✳️any/🧪️tests/📸️mutate-remodeling-1/{🥒️.feature,🦀️.rs,🐍️.py}`.
- layout (§7c): the 11 files listed by `layout-rotate-frame.py`, the rotate-frame test's include paths, `📐️mutate-layout-1/🐍️.py` (oracle paths).
- test platform (§7d): `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts`, `…/🧪️test/🧬️schema/🔣️.json`, `…/🧪️test/🧪️tests/🧪️test-platform/🟦️.ts`.
- structural laws (§7e): `🧪️tests/🔬️structural-correspondence/🦀️.rs` of sourcing-curation, writer, gis-gisterrain, imperative-procedure, space-space.
- gis: `✏️s/🔌️plugins/🌍️gis/🧪️tests/🔬️surface/🦀️.rs`.
- Python oracles: `📡️mutate-wires-1`, `🪐️mutate-s-space-1`, `🛟️mutate-procedure-1`, `🎥️mutate-shooting-1` (`🐍️.py` each).
- Ticket inputs: `wp-t12/{remeasure.sh,followup.sh,catalog-problems.ts,remodel-row-ids.py,structural-outcomes.py,oracle-vector-paths.py,layout-rotate-frame.py,pdf-kinds.json,tsconfig.t12.json}`.
