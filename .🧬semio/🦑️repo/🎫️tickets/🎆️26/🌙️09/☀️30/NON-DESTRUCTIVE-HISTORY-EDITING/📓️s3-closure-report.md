# 📓️ S3-CLOSURE Report — deleting every amend/coalesce/bracket path, derived fold footprints

Ticket `26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING`. Executor S3-CLOSURE (Opus), session 3. Contract: `📋️design.md` §13, §15, §17.5,
§17.7, §20 (overrides census D1: no amend on ANY lane). Input: `📓️s3-closure-census.md` §(b)–(d). Scratch: `🗑️generated/s3-closure/`.

## Session 3 — 2026-10-02

Status: IN PROGRESS (started 10-02 12:05; resumed 10-03 05:50 after TREE GREEN core). Sections below are updated at every milestone.

### Step 1 — RED gate (DONE 12:55)

- Root `📜️script.ts` region `//#region 🚫️HistoryClosure`: `HISTORY_CLOSURE_RULES` (8 rules, literal `anchors` prefilter + regex),
  pure `historyClosureFindings(sources, rules)` (rule `functions` = enclosing-fn allowlist for declared whole-document intents),
  `historyClosureSources` (one `git ls-files -co --exclude-standard` over `✏️s 🧰️framework 🌎️hub`, `.rs .ts .tsx .js .py .feature .json`),
  `policyHistoryClosureBreaches`. Command `bun ./📜️script.ts verify history-closure [--self-test|--json]` (VerifyScript route
  `runHistoryClosure`); launch entry `⚖️gate🚫️history-closure` (order 411.425) in `.vscode/🧩️launch.seed.jsonc` + `.vscode/launch.json`.
  NOT in `runGate` yet (red until step 8).
- Rules: `amend-emit` (`Emit::amend`, `amend_config`), `amend-last`, `coalesce-key` (`coalesce_key|coalesceKey|set_coalesce_key`, all
  files incl. fixtures/wire), `preview-contract`, `bracket-verb` (allow: `🖐️gumball-verb-audience.json`, puzzle 3d `🔬️unit` bracket-absence
  law), `host-snapshot-bracket` (`setHostSnapshot`, coordinator 12:40), `edit-literal` (plugin production `protocol::Edit {`),
  `footprint-hand` (`for_one_invertible_item|for_one_item|ArtifactStoreOneItemFootprint { work_items` outside `🏪️store`, non-test).
  `snapshot_edit_set_snapshot` is NOT banned (§20.3: survives for whole-document intents; D3 owners).
- Planted-violation test `🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️history-closure-policy/🟦️.ts` (27 cases + clean;
  banned tokens assembled from fragments so the file itself is clean). Taxonomy scope report clean.
- Superseded and deleted: tool-run predicate `interactivityToolRunAmendFailures` (+ dead `interactivityToolRunBlock`, the dead
  `actions` requirement column ×26 rows, trigger tokens `Emit::amend|coalesce_key`, 8 amend cases in
  `⏯️tool-run/🧪️tests/🔬️interactivity-tool-run-policy/🟦️.ts`), draw gate `coalesce_key: Some(` check + its hostile case
  (`🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts`), puzzle fill fixture `coalesce` fn (2 cases re-anchored).
- Verification: self-tests run via `🗑️generated/s3-closure/selftests.ts` → tool-run 29 PASS, drawing 20 PASS, closure 28 PASS,
  puzzle-fill 32 PASS; `verify history-closure --self-test` PASS (28). Full scan (68 s) census 12:50:
  `amend-emit=9 amend-last=67 coalesce-key=351 preview-contract=3 bracket-verb=12 host-snapshot-bracket=40 edit-literal=45 footprint-hand=90`
  (`🗑️generated/s3-closure/gate-1.json`).

### Step 2 — document lane deletion (LANDED 13:05, re-verified 18:45 after the reboot)

- `OS/🔌️plugin/🦀️.rs`: `Emit::amend` deleted; `artifact_lane_commands` lost `coalesce_key` + the `AmendLast` arm (outside a
  transaction every emission is one plain `Apply`); `tool_transaction_shape_fault` lost `coalesced` (+ `#[allow(too_many_arguments)]`),
  both callers updated; the retained route no longer calls `publication.set_coalesce_key` on the DOCUMENT publication;
  `🔖️UtilityPreviewContract` → `🔖️ToolContract`, `Emit::commit` doc rewritten.
- `OS/🔌️plugin/🛠️tool-machine/🦀️.rs`: the 2 artifact-transaction `emit.coalesce_key = None` resets deleted (the config-press reset
  in `settle_press_config` (S3-CONTROLS) stays until step 5/6).
- Runtime contract test (`🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`): `AmendLabel` → `StreamLabel { value, commit }`
  (`Emit::stream_transaction` / `commit_transaction` on `TEST_STREAM_LABEL_TRANSACTION`), `SetLabel` is a plain `Emit::mutations`.
  Converted laws: `streamed_ticks_and_their_commit_are_one_undo_step`, new `plain_emissions_never_coalesce`,
  `a_streamed_gesture_appends_exactly_one_command_log_entry`, `a_long_streamed_gesture_previews_its_newest_operations_only`,
  `a_streamed_gesture_is_one_undo_step_while_each_commit_is_its_own`, `a_streamed_tick_reports_only_its_own_new_operation`.
  Tool tables `🧪️tests/⏳️completion/🦀️.rs` `amendLabel` → `streamLabel`.
- Verification: `cargo check -p semio-framework-plugin --lib` PASS at 18:43 (139 warnings, none in touched regions).
  `cargo test -p semio-framework-plugin --lib -- streamed plain_emissions_never_coalesce` (private target `target-nde-s3-closure`):
  BLOCKED, 58 compile errors, all in peer files from the schema split (`LocalizedLabel`/`Locale`/`Terminology` private,
  `artifact_schema_descriptor_registered` missing, `take_candidate` private). My test file type-checks (only warnings). Reported to
  the coordinator. The converted laws are WRITTEN BUT UNVERIFIED until the lib-test build is green.

### Step 3 — `next_edit` helper (SOURCE COMPLETE 19:20, verification blocked by peer reds)

- Store (`OS/🏪️store/🦀️.rs`, additive, announced to S3-W1G): `ArtifactStoreOneItemLiveAuthority::next_edit<M>(&self, forward, inverse)
  -> Edit<M>`. It mints id = `edit_id()`, line, actor, group, sequence, clock, base version, `<id>#0`, and `description: None`
  (coordinator: §20.6, no hand-written labels). `cargo check -p semio-framework-os-kernel --lib` PASS at 18:58 (43 warnings).
- Runtime: `bounded_config_next_edit` deleted → `authority.next_edit`; the window-config preparation literal (`🔌️plugin/🪟️window/🎚️config`)
  → `next_edit` (the dropped per-mutation meta hooks `dependencies/base_version/author_id/timestamp/undo_policy` have no override anywhere:
  they were always the defaults); the runtime test `TestCountOneItemPreparation` literal → `next_edit`.
- Plugins: script `T/🧪️s3-closure-next-edit-sweep.py` (verifies every literal field is authority default before touching it).
  31 wrapper fns deleted with their calls rewritten, 10 inline literals replaced (41 sites in 33 files). 4 sites done by hand:
  flow `📬️preparation` (identity texts 1/2/8/9/10 + phase 99 + `🔤️bytes` `edit_id_length/edit_id_byte` deleted; texts 11 → 6),
  gismap (`gis2d_one_item_edit` = `next_edit` + Hub stamp override), energy (`energy_model_retained_edit` deleted), stdio zip.
  The description argument of every builder is dropped (row label from the leaf, §20.6). Acceptance: `protocol::Edit {` in plugin
  production code = 0 (left: 2 test literals, `writer 💾️binary` unit test and hub `💡️inference/🧾️wal` test, which die with
  `coalesce_key` in step 6).
- Verification: BLOCKED. `cargo check --manifest-path ✏️s/Cargo.toml -p <34 touched crates> -p semio-s-artifact-space-space --lib`
  fails in the peer crate `semio-framework-pack` (`🎒️pack/🔤️json/🦀️.rs:1516,1527` E0282). Since 19:10 the root kernel is also red from
  the dsl split (`semio_framework_dsl` unresolved, `FieldValue` slice mismatches in `🏪️store` lines 12182/13688/23452, none mine).
  Reported. Owed: re-run both checks plus the plugin unit tests of the 34 crates when green. `semio-hub-space` (root workspace,
  `🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🦀️.rs`) is also owed.

### Step 4 — derived fold footprint — SOURCE COMPLETE 10-03 06:12 (framework verified; plugin batches blocked by peer reds)

Mechanism (schema-first, design §20.5):
- Vocabulary `🧬️schema/🧫️fixtures/🧬️vendor-annotation-vocabulary/🔣️.json` registers `x-semio-inverse-rows`
  (`{fixed:N}` | `{perTarget:{<arrayField>:k}}` (+`fixed`) | `{bounded:N}`; absent = one row). The bounded exclusion is strict-Ajv
  clean (`dependencies.bounded.properties.{fixed,perTarget}: false`; the first `not.anyOf.required` form broke every strict suite at
  load — fixed 10-02 19:45, verified: `semioSchemaAjvV1()` + 9 instance cases PASS, renderer engine-contract (long, `-t gumball`)
  12 passed, architect document-contract + writer window-state exit 0).
- replication `🎮️mutation`: `MutationLeaf::inverse_rows(&self) -> usize` (required, derived) and `Mutation::inverse_rows(&self)`
  (default 1 = point-invertible, for the 87 hand-written aggregates without leaf schemas); `#[derive(Mutations)]` overrides it per
  variant with the wrapped leaf's derived rows. (A separate `InverseRows` trait was dropped: hand aggregates flow through the generic
  bounded-config / window-config factories, so the bound must be the `Mutation` trait itself.)
- derive (`🗣️dsl/✨️derive/🦀️.rs`): `#[derive(MutationLeaf)]` reads the payload schema's `x-semio-inverse-rows` at compile time
  (`mutation_leaf_inverse_rows`, same exclusions as the vocabulary) and emits `fixed + Σ k·self.<field>.len()` (camelCase → snake_case;
  a `payload = Variant` leaf reads the variant). The derive-generated payload law runs `mutation_inverse_rows_failures::<Snapshot,
  Aggregate>` (`📡️spr/🎮️command`) over EVERY subset of the owner's standard (`…/🪆️subsets/**`, since sibling-subset fixtures replay
  through the `✳️any` aggregate — sequence's quintets live in `🪜️step`/`🔗️dependency`): every fixture case's `inverse(before).len()
  <= inverse_rows()`.
- Store (`🏪️store/🦀️.rs`): `ArtifactStoreOneItemFootprint::for_leaf<P, M: Mutation<P>>(&M, bytes)` and `for_ephemeral_item(bytes)`;
  `for_one_invertible_item` / `for_one_item` DELETED (no caller left anywhere).
- Framework: bounded config admission (`admit_bounded_config_mutation<C, M>` → `for_leaf`), window-config preflight (`for_leaf::<O::State,
  O::Mutation>`), retained-command `ArtifactRetainedWorkCapacity::one_item_footprint` DELETED (gen2d/gen3d preflights call `for_leaf`;
  `generation2d/3d_one_item_footprint` helpers deleted, gen3d fold-contract + work-capacity tests and the dev composition law updated).
  Runtime tests: publication presence/transient/owners footprints → `for_ephemeral_item`, builder-contract count prep → `for_leaf`.

Plugin sweep (scripts in T: `🧪️s3-closure-inverse-rows-annotate.py`, `🧪️s3-closure-footprint-manual.py`, `🧪️s3-closure-footprint-sweep.py`):
- 62 leaf payload schemas annotated. Exact transfers of the former hand counts: layout drag/rotate/scale-frames (targets ×1/×2/×2), draw
  drag/rotate/scale-layers + drag-path-points (targets ×1), wfc bitmap resize-input (fixed 2) / resize-output (bounded 1025), puzzle 5d
  (drag2d ×1, drag3d ×2, rotate3d/scale3d ×1, delete-part/remove-part-grip 65), puzzle 3d (drag/rotate bounded 4096 — the former
  `1 MiB / size_of` became a literal, scale ×1, delete-object/remove-object-vortex 65), puzzle 2d (delete-node 65, drag ×1, scale ×2,
  rotate ×65, remove-node-handle 2), fem 2d/3d move-selection (nodeIds + regionIds / solidIds), flow delete-widget 258.
  Newly declared (were flat 2 = latent refusals): **sequence delete-step bounded 768** (the inverse resets the scene: ≤ 256 steps ×2 +
  256 edges, `SEQUENCE_STORE_MAXIMUM_SCENE_ITEMS`; its artifact lane declared `work_items: 1`), equation delete-node 513 / delete-nodes
  768 (`EQUATION_MAX_NODES/EDGES`), procedural gen2d/gen3d move-nodes (ids ×1), gen3d drag/scale/rotate-transforms (targets ×1), animate
  delete-tiles (ids ×1), energy delete-fenestration 2, wfc grid3d resize-grid fixed 4, wfc grid2d mask-cell 2, stdio semio kit
  remove-design fixed 2; uncapped cascades (record + incident links / positional tail rebuild) bounded 1025 (the wfc bitmap pin-cascade
  precedent: a larger cascade is refused honestly): wfc 2d/3d delete-slot/delete-tile, grid2d/3d delete-tile, grid2d resize-grid,
  dag delete-node, trinity jack delete-node, energy delete-surface, stdio semio table delete-column, mesh delete-primitive/mesh/material,
  graph delete-node, brep delete-edge/shell/solid/vertex/face, kit unbind-representation.
  Audit basis: every leaf inverse (1992 `↩️inverse` files + 1041 inline) classified by growth pattern (`🗑️generated/s3-closure/inverse-*`).
- 45 durable sites → `for_leaf(mutation, bytes)`: writer, equation, gismap, gis terrain, vcs, animate, demonstrator, sequence (`work_items: 1`
  fixed), fem 2d/3d, process3d ×2, lowpoly ×2, forms, cad ×2, playbook, energy, dag, raster ×2, stdio docx/zip/semio, puzzle 2d/3d/5d
  (artifact + config), block 2d/3d/5d, space core/home, sourcing ×2 (`work_items: 1` on the curation lane fixed), hub space, flow ×2,
  layout (wrapper `LayoutArtifactStorePreparationFactory` deleted, the framework factory derives the same rows), draw, wfc bitmap.
  Deleted hand row counters: `layout_mutation_inverse_rows`, `drawing_inverse_rows`, `bitmap_inverse_rows` + `BITMAP_PIN_CASCADE_ROWS`,
  `PUZZLE2D_DELETE_NODE/ROTATE_SELECTION_INVERSE_ROWS`, `PUZZLE3D_SELECTION/REMOVAL_INVERSE_ROWS`, `PUZZLE5D_REMOVAL_INVERSE_ROWS`, flow
  `inverse_rows` match. Semantic-item validators that returned a footprint now return retained bytes (process3d
  `process3d_mutation_retained_bytes`, sourcing `…_config_retained_bytes`/`…_mutation_retained_bytes`, cad `admit_cad_config/snapshot`).
  Stale "work_items: 1 fail-closed" comments inside definitions removed; docs in norm, dag ×4, hub space law rewritten.
- Gate: `footprint-hand` 91 → **0** (`gate-direct.ts`, 06:14; the `verify` CLI route is currently refused by a peer `📋️project.json`
  (`trinity rewriting:test-snapshot-sqlite` invalid owned command), so the gate ran through its exported functions).
- Verification: `cargo check -p semio-framework-plugin --lib` PASS 06:08 (plugin lib 143 warnings = unchanged), derive crate PASS 06:13.
  Plugin batches (`--target wasm32-wasip2`) BLOCKED by the peer ValueError sweep (`semio-framework-artifact-workflow-workflow` 222 errors,
  stdio mp4/gif sqlite), none of my crates reached. Owed on green: the plugin batches, their unit tests (layout fold law, equation
  `the_store_preparation_derives_its_footprint_from_the_leaf`, cad/process3d/sourcing/gen3d footprint tests, the new sequence law
  `delete_step_inverse_fits_its_declared_fold_footprint` in `🪜️step/…/🗑️delete-step/🧪️tests/🔬️mutation-law` — sequence's committed
  quintets are reject/no-op cases only, so this law pins the multi-row reset the coordinator flagged), and the derived payload laws
  (L3) across all plugins.

### Step 5 — config lane (no amend) — SOURCE COMPLETE 19:55 (wave 5a), verification blocked

Owner status at 19:30 (re-grep): every plugin producer of a coalesce key is gone (fem playback, gis/energy viewers, shooting, forms
try-value, process3d cursor/engagement converted by their owners; S3-CONTROLS landed `settle_press_config`: a scrub press holds
config/window-config as the window's provisional config and its release publishes ONE config edit). The last V site, the draw
viewer camera (`🖍️drawing/✳️any/👁️viewer/🦀️.rs:98`), lost its `coalesce_key` here (same shape as the gis/energy viewers).
- Runtime (`OS/🔌️plugin/🦀️.rs`): `Emit.coalesce_key` + its Default deleted, `Emit::amend_config` deleted, `TransactionProposalDraft.coalesce_key`
  deleted (the wire frame sends `coalesce_key: String::new()` until wave 6c deletes `AppFrame::TransactionProposal.coalesce_key`), the
  emit-retirement step for the key deleted, `dispatch_emit_inner` config lane is always one `Apply` (doc-config `config:{key}` AmendLast
  gone), retained route: no `emit.coalesce_key` reset, no `set_coalesce_key` on the CONFIG publication, window-config `begin` without key.
  Docs naming `AmendLast`/`amend_config` rewritten (streamed open edit).
- `OS/🔌️plugin/🪟️window/🎚️config/🦀️.rs`: `dispatch`/`begin` lose `coalesce_key` (trait + impl + store wrappers), the `window:{id}:{key}`
  AmendLast arm and `publication.set_coalesce_key` deleted.
- `OS/🔌️plugin/🛠️tool-machine/🦀️.rs`: the config-press reset in `settle_press_config` deleted.
- Runtime tests: `🧪️time-travel` drops the coalesced-transaction refusal; `🧬️mutation-fixtures-transaction` `CoalescedIncrement` →
  `StreamedIncrement` (`Emit::stream_transaction`, law `a_streamed_tick_extends_cached_history_in_place`) + command-close test, fixture
  and schema (`streamed-increment`); `🔬️app-typed-command-full-operation` byte oracle without key; fixture
  `🧵️retained-command/🔄️full-operation/🧫️fixtures/🔣️.json` re-sealed (7 cases, expectedBytes = description + 1, verified by script) + schema.
- Plugin trees (mechanical): note retained accumulator refusal, puzzle 5d completion retirement step, 13 test assertions (fem 2d/3d gumball,
  layout ×2, trinity rewriting, draw ×2, puzzle 2d ×2, puzzle 3d example-switch ×2, puzzle 5d retirement law format, hub space spawn-app),
  puzzle 3d settle doc.
- Grep after the wave: `coalesce_key|amend_config|Emit::amend` in `✏️s`, `🌎️hub/🧩️compositions` (non-JSON) = 1 (the writer
  `💾️binary` test `Edit` literal, dies in 6b). Framework outside store/spr/replication/manifest: only `GroupMeta.coalesce_key: None` +
  the interim AppFrame field.
- Verification: `cargo check -p semio-framework-plugin --lib` at 19:58 → kernel RED from peers (277 errors: `dsl::Diagnostic`
  unresolved in `🚪️io`, `crate::Fault*` missing in `📡️spr/🧵️channel`), none in files I touched. WRITTEN BUT UNVERIFIED.

### Step 6 — store/wire deletion — 6a + 6b SOURCE COMPLETE 21:10, 6c scheduled (final channel-bump wave)

Announced to the coordinator/S3-W1G before touching `🏪️store/🦀️.rs` (W1-G: no collision; its `[DEBUG]` probes untouched).
- 6a (`OS/🏪️store/🦀️.rs`, script `T/🧪️s3-closure-store-wave6.py`, every anchor asserted): `ArtifactCommand::AmendLast` + `AmendLastInLane`
  deleted (enum, `projection_cause`, `history_transition_kinds`, dispatch arms, `retire_command`), `amend_command` deleted, text codec
  (`CommandHeaderLine::Amend`/`AmendInLane` + print/parse arms) and binary codec arms (ordinals 8 and 12 unassigned; every other variant
  keeps its fixed ordinal, doc says "fixed variant ordinal"), `ArtifactStoreBatchPublication.coalesce_key` + `set_coalesce_key` + its
  retirement/terminal/close-state lines, `batch_amend_target(transaction)` keeps only the TransactionRef branch, the fold no longer
  stamps a key; docs naming `AmendLast`/`amend_command` reworded (AppendTransaction).
- 6b `Edit.coalesce_key` deleted everywhere: replication `Edit` (+ ToValue/FromValue), store edit retirement (strings 8 → 7), owned `.spr`
  edit decoder (field id 7 unassigned, string slots 8 → 7 remapped), `.ops` header (`OpsHeaderLine::Edit.key`), history projection, text
  parse `PendingEdit`, chained edit digest (tag + bytes), canonical-edit walk (fields 12 → 11, children 7..10 remapped), hydration,
  retained config load, member-open retire list, `GroupMeta.coalesce_key`; spr `HistoryEdit` (`.ops` field `key` removed, field ids
  renumbered, binary presence bit 2 now REFUSED as malformed), spr CLI (`(amends …)` marker + hash), manifest
  `TutorialDocumentEventKind::Edit.coalesce_key` + schema projection TS string; 44 `coalesce_key: None` literals removed by regex (19 files).
- Tests: store unit (AmendLast laws → `a_streamed_transaction_grows_one_edit_over_many_ticks`, `a_streamed_edit_revision_is_its_own_from_scratch_digest`,
  stateful-current law streams; absorb/differing-key/committed/empty amend laws deleted; codec list), tool-transaction, flow-vcs
  (`a_streamed_layout_drag_produces_one_edit`), spr history/protocol-laws/CLI, manifest app-label, canonical-edit unit/reader.
- Fixtures re-sealed: `🏪️store/🧫️fixtures/📤️outbound-announcement` (rewritten: every gesture its own edit; 3 vectors, 14 steps) + schema +
  Rust law + TS twin (`💻️os/🧪️tests/📤️outbound-announcement`); `🧵️canonical-edit/🧫️fixtures/🔗️edit-digest-chains.json` re-sealed by
  `T/🧪️s3-closure-reseal-edit-digest-chains.ts` (first PROVED the oracle reproduces all 7 committed digests with the old slot, then
  recomputed without it) + TS twin. `bun` run of `testCanonicalEditFixtures()` → PASS (21:05).
- Unrelated "coalesce" concepts renamed so the token is absolute: `🛎️services` mailbox `coalesce_key` → `mailbox_key` (5), renderer
  `PluginRuntime/🟦️.tsx` turn-queue `coalesceKey` → `turnKey` (9, file-local). Gate allowlists the hub space
  `🔮️ownership/🔣️.json` (sha256-pinned historical source snapshot).
- Verification: WRITTEN BUT UNVERIFIED (kernel red from the Codex DSL extraction; coordinator: wait for TREE GREEN, no polling).

#### Final channel-bump wave (coordinator-run, design §20.1; ready to apply)
`AppFrame::TransactionProposal.coalesce_key` (frame tag 15) is sent empty and never read until then. Exact steps, in one wave with the
`APP_CHANNEL_VERSION` bump:
1. `OS/📡️spr/🧵️channel/🦀️.rs`: delete the field from `AppFrame::TransactionProposal` (≈2873), the encoder arm destructure + `write_str(coalesce_key)`
   (≈3901/3907) and the decoder line `coalesce_key: read_str(..)` (≈4057); bump `APP_CHANNEL_VERSION`.
2. `OS/📡️spr/🧵️channel/🧪️tests/🔬️unit/🦀️.rs` ≈471, ≈869: drop `coalesce_key: …` from both frame literals.
3. `OS/🔌️plugin/🦀️.rs` (`plugin_exchange`, ≈42595): drop `coalesce_key: String::new()` from the `AppFrame::TransactionProposal` literal.
4. `🧰️framework/🛍️products/💻️os/🟦️.ts`: type field (≈2705), encoder `writeStr(out, frame.transactionProposal.coalesce_key)` (≈3414), decoder
   `const coalesce_key = readStr(bytes, pos)` + the returned object (≈3584–3586); the TS channel-version constant.
5. `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️backbone-envelope-io/🟦️.ts` ≈524, ≈603, ≈804: drop `coalesce_key` from the three frame literals (and any
   byte-length expectations the empty string contributed: one zero-length varint byte per frame).
6. Regenerate the frame-worker JS (`📺️renderer/…/🎞️frame-worker/🤖️generated/🟨️.js`) and rebuild/describe every plugin component.
7. Re-run `verify history-closure`: `coalesce-key` must be 0.
8. `AppCommand::LoadDocument` (S3-W2A `📓️api-stepped-document-load.md` §5; every host already loads through `LoadDocumentArchive`):
   `📡️spr/🧵️channel/🦀️.rs` variant (≈2528) + field-page arms (≈1978/1981/2007/2010/2154/2347) + encoder (≈3267) + decoder tag 6 (≈3655)
   + docs (≈83, ≈2581); channel tests (≈98, ≈632, ≈810, ≈893 golden `060101010102`); `🔌️plugin/🦀️.rs` guest arm (≈43966); remaining
   senders first moved to the archive admit/poll/ack loop (S3-W2A): MCP workspace `exchange_one_real` (≈1477, ≈2657, ≈2111), `🏃️run`
   (≈1129, ≈2180 + its unit test ≈70/≈376). TS `💻️os/🟦️.ts`: union member (≈2617), tag `LoadDocument: 6` (≈2896), encoder (≈2959–2963),
   decoder (≈3149–3153), `AppChannelClient.loadDocument` (≈4192); `🧪️backbone-envelope-io` callers (≈1336, ≈1448, ≈1454, ≈1486).
   Tag 6 stays unassigned (no renumbering).
Other wire changes of my waves: none on the guest↔host frame. Persistence/codec changes (no channel impact, detectable): `.spr` `HistoryEdit`
presence bit 2 refused; `.ops` edit field `key` gone; `ArtifactCommand` binary ordinals 8/12 unassigned (decode → "unknown command ordinal").

### Step 7 — plugin sweeps (10-03 06:30)

- `setHostSnapshot` (`host-snapshot-bracket`): 12 → **0**. Every guest decoder is gone; what remained were refusal laws and docs
  naming the refused row. The rule now skips test trees and allowlists the shared refusal fixture
  `🛠️tool-machine/🧫️fixtures/🧫️node-graph-edit-rows/🔣️.json`; the 2 production docs (procedural gen2d/gen3d `✏️node-graph-edit`) say
  "a whole-fixture host-snapshot row". New planted case `host-snapshot-refusal-law-is-silent`.
- §20.6 hand labels: `Emit::commit_config(…, description)` had 0 callers → DELETED (plugin lib check PASS 06:23, 143 warnings).
  `Emit.description` + the label parameter of `Emit::commit` stay until the last callers are gone: 3 stdio `Emit { description:
  Some(format!("Load example {example_id}")) }` (html, md, txt editors — S3-STDIO) + the runtime contract test `TestCommand::CommitLabel`
  (mine, deleted together with the API). Producer `Emit::commit(` callers in plugin trees: 0.
- `bracket-verb` 11: only the GENERATED composition descriptors (`✏️s/…/🧩️compositions/{demonstrator,fem,lowpoly,puzzle}/🔣️.json`
  list `transformBegin/End`, `paintStrokeBegin/End`); no source declares them — coordinator descriptor regeneration.
- `coalesce-key` 14: exactly the AppFrame tag-15 field (spr channel ×4 + its tests ×2, plugin literal ×1, TS twin ×4, backbone-envelope
  test ×3) — the final channel-bump wave above.

### Step 8 — laws, acceptance greps, coordinator actions (10-03 06:35)

Gate (`gate-direct.ts` = `historyClosureFindings(historyClosureSources(root))`, self-test 31 cases PASS):
`amend-emit 0 · amend-last 0 · coalesce-key 14 (bump wave) · preview-contract 0 · bracket-verb 11 (descriptors) · host-snapshot-bracket 0
· edit-literal 0 · release-plain-commit 0 · footprint-hand 0`. The `bun ./📜️script.ts verify history-closure` CLI is refused right now
by a peer `📋️project.json` (`✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/📦️packages/🦀️rust/📋️project.json:test-snapshot-sqlite`,
"Invalid owned command") before any route runs.
Laws:
- **L1** type-level: `ArtifactCommand` has no amend variant (enum + both codecs deleted; ordinals 8/12 unassigned) + gate `amend-last 0`.
- **L2** N ticks + commit = 1 edit, abort = 0: framework laws `a_streamed_tool_transaction_is_one_row_and_its_abort_leaves_none`
  (time-travel), `a_streamed_transaction_grows_one_edit_over_many_ticks` (store), `streamed_ticks_and_their_commit_are_one_undo_step`
  (runtime); the per-family cross-plugin harness is S3-AGNOSTIC's G12.
- **L3** fold footprint: derived payload law over every subset fixture of every derived aggregate (`mutation_inverse_rows_failures`).
- **L4** config edits are never history rows: S3-CONTROLS' energy law `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row`;
  the runtime routes every config emission as one plain `Apply` (no AmendLast arm left).
- **L5** canonical digest + wire schema without `coalesceKey`: digest chains re-sealed (TS oracle PASS), `.spr` presence bit 2 refused,
  `.ops` field gone; the wire frame field is the bump wave.
- **L6** new gate rule `release-plain-commit`: in plugin production code a function named `*pointer_up*`/`*_release`/`release_*`
  may not emit `artifact_mutations:`/`Emit::mutations(`/`Emit::commit(` unless it commits/streams/aborts its transaction (or is a
  node drag / `ToolStep`); 0 findings over 18 such functions; pointer-up match arms all delegate (survey: 0 direct emissions).
  Descriptor bracket verbs → coordinator regeneration.

Coordinator actions:
1. Final channel-bump wave (§6 list: tag-15 field + `AppCommand::LoadDocument` + `AppChannelClient.loadDocument`), then `coalesce-key` = 0.
2. Regenerate the demonstrator, fem, lowpoly and puzzle composition descriptors (`describe`) → `bracket-verb` = 0. The leaf payload schemas
   gained `x-semio-inverse-rows` (62 files): any descriptor that embeds payload schemas is stale too.
3. Then wire `policyHistoryClosureBreaches` into `runGate` (red until 1–2 land).
4. Peer `📋️project.json` (trinity rewriting `test-snapshot-sqlite`) blocks every `verify` CLI route.

Verification owed on green (blocked 06:10–06:30 by the peer ValueError sweep in workflow + stdio sqlite and kernel lib tests):
plugin batches (`--target wasm32-wasip2 --lib`) for the 45 step-4 crates + the 34 step-3 crates + `semio-hub-space`; kernel lib tests
(store unit, tool-transaction, outbound-announcement, canonical-edit, spr history/CLI/protocol-laws, flow-vcs); plugin lib tests
(runtime contract streamed laws, transaction fixtures, full-operation, time-travel, gen3d fold-contract); plugin unit tests of the
touched crates incl. their derived payload laws (L3).

### Audit follow-ups CLOSURE-1 / CLOSURE-2 (10-03 11:15)

- **§20.6 `Emit.description` + `Emit::commit` DELETED** (script `T/🧪️s3-closure-emit-description.py`, 14 files, every anchor asserted).
  Label gate re-run repo-wide first (`mutationLabelReport`): 0 diagnostics, 0 `labelHandwritten`; my all-`.rs` Emit-literal scan:
  0 production producers (only test literals). Runtime: `Emit` field + Default line, `Emit::commit` (= `Emit::mutations`) deleted;
  `authored_row_label` deleted (rows record `verb_label`); `backfilled_edit_label` and the history projection never read
  `edit.description` (verb → single leaf → first leaf (+N)); `artifact_lane_commands`, window-config `dispatch` (trait, impl,
  wrapper) and `PendingChildGroupPublication` lost their description; `TransactionProposalDraft.description` deleted (frame sends
  `description: String::new()` until the bump wave); retained route passes `None` to the store batch APIs; tool-machine resets gone.
  `dispatch_emit_group` keeps a `description` argument ONLY for the agent `transaction_commit` label (`PendingTransaction.label`,
  host-supplied), stored as `Edit.description` but never a row label. Plugins: flow retained child-group contract, note retained
  accumulator, puzzle 5d/3d tests. Tests: builder contract (5 literals, `CommitLabel` → `SetLabel`), completion tables,
  mutation-fixtures dummy/surface/transaction, window-config retained-pack-load (3 dispatch calls), history-label-reload
  (Rust + fixture + schema + TS twin re-sealed: no `description`; cases `undeclared-single-leaf`, `declared-single-leaf`,
  `verb-labelled`, `leaf-labelled`, `undeclared-multi-leaf`).
  Verified: `cargo check -p semio-framework-plugin --lib` PASS 11:05 (147 warnings; the +4 are in peer-edited
  `⏪️time-travel`/`📥️retained` files, none in mine); TS twin `historyLabelReloadOracle` PASS (5 cases).
  Blocked: plugin lib tests (`--no-run`, private target) — 158 errors, all peer (ValueError/IoError sqlite sweep in test fixtures,
  S3-CONTROLS' in-flight tool-machine `scrubs`/`config_presses` rename); plugin crates (wasm32) — peer stdio sqlite red.
  Follow-up (store wave, coordinator): `Edit.description` (store, `.spr` F_EDIT_DESCRIPTION, `.ops`, canonical digest), `ArtifactCommand::Apply.description`,
  `GroupMeta.description`, `begin_*apply_batch(description)`, `AppFrame::TransactionProposal.description` (rides the bump wave),
  the agent `transaction_commit` label.
- **Manifest typegen regenerated** with the framework's own exporter (`cargo test --features typegen exports_typescript_bindings`,
  `SEMIO_TYPEGEN_OUT`, private target): one line changed — `TutorialDocumentEventKind` lost `coalesceKey`; freshness re-check PASS.
- **Gate wired into `runGate`** (`VerifyScript.runGate`, after the mutation-outcome law): self-test must execute cases, then any
  `policyHistoryClosureBreaches` fails the gate (red until the bump wave + descriptor regeneration). `bun ./📜️script.ts verify
  history-closure --json` works again (peer project.json fixed): census unchanged (coalesce-key 14, bracket-verb 11, rest 0).
- **CLOSURE-2 / §20.13 L4 law rewritten**: energy `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row` now
  measures `history_rows_and_mutations` = (ALL `history_snapshot().upserts`, Σ their mutation rows) — no `edit_id` filter. It will
  stay RED until S3-W2A's projection change drops config-lane rows (today a config release is a row); unrun (energy pulls the red
  stdio-epw crate).

### TREE GREEN core run (10-03 11:39–12:03)

- `cargo test -p semio-framework-os-kernel --lib` (private target): **1254 passed, 2 failed** →
  - `outbound_announcement_tests::every_locally_authored_operation_is_announced_exactly_once`: MINE — the Rust law still demanded
    `steps >= 22` after my 14-step re-seal (TS twin already said 14). Fixed to `>= 14`; re-run **PASS** (11:45).
  - `sqlite_snapshot_space_history_baseline::…_uses_same_caller_ledger` ("zero physical allowance must refuse before native
    construction"): PEER — the new, uncommitted `🏪️store/📜️space-history/…/🪶️sqlite` work, not mine.
  Includes my converted laws `a_streamed_transaction_grows_one_edit_over_many_ticks`, `a_streamed_edit_revision_is_its_own_from_scratch_digest`,
  canonical-edit, tool-transaction and spr laws (all ok), and **L3 in core: 10 derived payload laws `semio_payload_law_*` ok**
  (`mutation_inverse_rows_failures` runs inside each).
- `cargo test -p semio-framework-plugin --lib --no-run`: **BLOCKED by peers**, 6 errors, none mine:
  `store::ReplayTurnBudget` missing (`⏪️time-travel/🦀️.rs:36-37`, `🧪️tests/🧪️time-travel:1632`, a 0-arg method called with 1 at
  `⏪️time-travel:2076`), `ArtifactStoreInitializationEditAdmission::{Oversized, Duplicate}` missing (`🔌️plugin/🦀️.rs:17046`) — a
  store/time-travel change in flight. So the runtime contract laws (streamed/plain/one-row), transaction fixtures, full-operation,
  time-travel and history-label-reload stay UNRUN.
- `cargo test -p semio-framework-os-flow --lib -- flow_vcs` (`a_streamed_layout_drag_produces_one_edit`): **BLOCKED by a peer**,
  `semio-framework-os-infinite` `DagExpandedPaths` missing (`♾️infinite/🎲️board/…/🕸️dag/🦀️.rs:2243`).
- L4 (energy law over ALL rows): unrun — energy pulls the red stdio-epw crate; expected RED until S3-W2A's projection change.
- ✏️s plugin crate checks: still waiting on the stdio peer.

## Session 4 — 2026-10-04

Continued by S4-GATES in `📓️s4-gates-report.md` (one report for the four inherited WPs, rule 34).
