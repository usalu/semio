# 📓️ W3-T2-CONTROLS Report: Scrub Machine, Continuous-Control Hosts, ToolRun Transactions

Executor W3-T2-CONTROLS. Scope: `📓️audit-remaining-tools.md` §7 row 1 (F-1, F-3, F-4, §5.2, §6.2) and design §13.1.
Nothing was committed by me; no ticket or goal was opened or closed. API for other executors: `📓️api-scrub-machine.md`.

Aliases: `FW` = `🧰️framework/🔨️modules`, `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`, `PLG` = `OS/🔌️plugin/🦀️.rs`,
`RE` = `OS/📺️renderer/🧑‍🎨engine/🧱️elements`, `TM` = `FW/🛠️tool-machine`.

## 1. Census (before this work)

| Control | Host behaviour | Guest commit |
|---|---|---|
| React `SliderView`, continuous number `InputView` | lane `{value, gesture, commit}` (no abort, no unmount/blur cancel) | per plugin: amend with key, or one edit per tick |
| React `NumberStepperView` (absolute) | one bare `change {value}` per click/keystroke | one edit per dispatch |
| React declarative `renderUiControl` slider/stepper (test-only path) | bare value per change | – |
| React text `Input` without `commit` | one `change` per keystroke | one edit per keystroke (F-4) |
| wgpu `Slider` | bare `Change {value}` on down, on every move, and on release only when released OVER the slider; keyboard/readout bare | same as React |
| wgpu `NumberStepper` (absolute), number `Input` (no commit) | bare value per click/keystroke | one edit per dispatch |
| gis terrain exaggeration | – | `Emit::amend(…, "gis3d-exaggeration")` static key |
| energy inspector sliders/numbers | – | one described edit per tick (no key) — ledger exhaustion class |
| energy name fields | per keystroke | one edit per keystroke |
| forms `patchQuestions`/`patchStep`/`patchQuestionOptions`/`patchVectorField`/`updateForm` | text per keystroke, numbers per tick | amend with static per-field keys (`patch:{field}:{ids}`, `patch-step:…`, `patch-option:…`, `patch-vector:…`, `change-form-title`) |
| norm `setField` text inputs | per keystroke | `Emit::commit` described edit per change |
| playbook title | palette/agent only (no field) | `Emit::amend(…, "playbook.title")` |
| ToolRun finalize | – | one edit, `transaction: None`, English-only description |

## 2. What landed

### 2.1 F-1: ToolRun rows are tool transactions
- `OS/🔌️plugin/⏯️tool-run/🦀️.rs`: `publish_tool_run` publishes with `Some(tool_run_transaction(app_id, tool_id, actor, run))` and NO
  description; `tool_run_transaction` mints `TransactionRef::mint(actor, {0, now_ms, run}, "<appId>#<toolId>")`.
- `tool_run_transaction_label(app_id, transaction)`: a transaction whose tool is `<appId>#<toolId>` of a declared tool run is
  labelled with the tool's own `LocalizedLabel` (all locales); `PLG::build_history_view` uses it ahead of the mutation-label
  rule, so the row reads the same after a reload (no single-locale description stored any more).
- Law `🧪️tests/🔬️tool-run` `tool_run_finalize_is_one_transaction_labelled_by_its_tool_in_every_locale`.

### 2.2 `ScrubMachine<M>` in `🛠️tool-machine` (Rust + TS twin, schema-first)
- Rust (`TM/🦀️.rs` region `🔖️Scrub`): `SCRUB_GESTURE_ARG/SCRUB_COMMIT_ARG/SCRUB_ABORT_ARG`, `ScrubPhase::parse/gesture/input`,
  `ScrubInput<M>`, `ScrubContext`, `ScrubEvent<M>` (`StatechartEvent`), `ScrubMachine<M>` (a `ToolMachine`, Effect
  `ToolYield<M>`, chart `idle → scrubbing`), M-independent tables + generic definition via an associated const (promoted to
  `'static`), `SCRUB_FINGERPRINT`/`SCRUB_MANIFEST_JSON` pinned against the `statechart!` compilation of the same chart,
  `ScrubHost`, `ScrubState<M>`, `Scrub<M>` (start/resume/send/persist; another press = host abort `captureLost` first),
  `ScrubLedger<M>` (per window; closed-press memory; base or tool change reopens; abort/abort_all/retain_windows/provisional).
- TS twin (`TM/🟦️.ts`): `parseScrubPhase`, `scrubMachine<M>()`, `scrubEvent`, `ScrubHost`, `Scrub`, `ScrubLedger`.
- Schema (`TM/🧬️schema/🔣️.json`): `ScrubGesture`, `ScrubPhase`, `ScrubArgs`, `ScrubInput`, `ScrubOpen`, `ScrubStep`,
  `ScrubScenario`, `ScrubChart`, `ScrubLawFixture` (additive; formatting preserved).
- Fixture `TM/🧫️fixtures/🧫️scrub-law/🔣️.json` (phases, chart incl. fingerprint, 13 scenarios) authored by
  `🧪️w3-t2-controls-scrub-law-fixture.ts` (hand-written expectations; transaction ids minted INDEPENDENTLY with first-party TS
  BLAKE3 + hand LEB128).
- Since then W3-T2-TEXT added a Typing region and W3-T-FLOWCAD a NodeDrag region to the same crate (theirs).

### 2.3 Runtime glue (plugin runtime)
- New module `OS/🔌️plugin/🛠️tool-machine/🦀️.rs` (my `ScrubRuntime`; W3-T2-TEXT later generalized it to `ToolMachineRuntime`
  with typing runs — the scrub path is unchanged). PLG seams (unique-anchor edits):
  - `dispatch_action`: after the registry/classification check, `admit_tool_dispatch` reads `gesture/commit/abort`; an abort is
    settled by the runtime (zero trace, never reaches the app); a press while time travel freezes is dropped and refused
    `timeTravel.frozen`; otherwise the tag rides `ingress` into `dispatch_typed_command_inner`, which binds it to the operation id.
  - time-travel verbs first `freeze_tool_machines()` (`frozen`); windows missing from the roster retire their press (`retired`).
  - `publish_mounted_typed_operation_unit`: at the one point the completion becomes the publication, `settle_tool_operation`
    moves the emit's `artifact_mutations` into the window's press (base = the operation's canonical revision). A tick or empty
    press publishes no artifact edit and logs no row (`command_logged`); the release publishes the committed batch as ONE edit
    with `emit.transaction = Some(ref)`, `coalesce_key`/`description` cleared. Other lanes publish as usual (ui_scope kept).
  - Render seams (render, window engagements, window measures, tool measures): `render_snapshot_or(tool_runs, overlay_or(..))` —
    committed ⊕ provisional leaves, refolded on every press change and on store generation change (`refresh_cache`); displaced
    aliases are retired through `store.retire_snapshot_alias` (never a plain drop).
- `VcsArtifactApp.tool_machines` field + constructor; `semio-framework-tool-machine` dependency of `semio-framework-plugin`.
- Laws `OS/🔌️plugin/🧪️tests/🧪️scrub/🦀️.rs`: overlay fold / untouched committed / refold-only-on-change / displaced aliases /
  bounded operation tags; plus the F-4 lint law.

### 2.4 Hosts
- Lane (`FW/🖱️ui/🎬️scene/🟦️.ts`): `ContinuousGestureLane.abort(reason)` + `open()`; port `abort?(reason)`; cancel is sent after
  the round trip in flight and before later offers; the cancelled press's release is ignored; a value-less `commit()` releases
  only an open press.
- React Interpreter (`RE/🗣️Interpreter/🟦️.tsx`): `useContinuousTriggerLane` sends `{gesture, abort}` and aborts `retired` on
  unmount; `SliderView` aborts on pointer cancel (`captureLost`) and blur (`blur`); `NumberStepperView` absolute changes ride
  the lane (button release / blur = release); declarative `renderUiControl` slider and stepper now use the same press protocol.
- wgpu (`FW/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs`, `WidgetState.scrub_gesture` in `🌳️tree`): a slider press ticks on down/move
  and releases ONCE on pointer up wherever the pointer lets go; pointer cancel → `{gesture, abort:"captureLost"}`; a11y blur on
  a pressed slider → `blur`; keyboard steps, readout edits and stepper clicks/keys are one-shot presses; a number `Input` without
  a commit policy and a stepper's typed value tick per keystroke and release on blur/Enter; Escape cancels the typed press.
- Shared corpus `FW/🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json`: rule `continuousPress` + per-case `press`
  (`released`/`open`); Rust law and TS twin check it.

### 2.5 F-4 input commit-mode lint
- `artifact_app_laws::document_input_commit_findings(projection, is_document_verb)` + `document_input_commit_lint(app, body, view)`
  (Mutation-kind verbs); fixture `OS/🔌️plugin/🧫️fixtures/🧫️input-commit-lint/🔣️.json`; law in `🧪️scrub`. Enforced in energy
  (`no_inspector_field_edits_the_document_per_keystroke`). CLOSURE can wire it into every app's tests.

### 2.6 Plugins
- gis terrain: `set_exaggeration` emits the absolute `change-exaggeration` (`Emit::mutations`); static key deleted.
- energy: inspector sliders/numbers unchanged (absolute `change-*`/`update-*` leaves from `model_edit`) and now scrub; name
  fields `Trigger::Commit` + blur.
- forms: all five verbs emit `Emit::mutations`; static keys deleted; inspector text-like fields commit on blur
  (`controls::input_row`, `row_on`), number fields scrub.
- norm: free-text `setField` inputs commit on blur (`bind_on`); number inputs scrub. No fixture touched.
- playbook: `updatePlaybook` emits one `change-title` per dispatch; amend deleted; describe text (en/de) and law updated.

## 3. Verification

See §3 table (filled below as each run completes).

## 4. Decisions and deviations

1. The scrub glue routes every dispatch carrying `gesture` (design §13.1). Late ticks of a closed press are silent; a moved
   document reopens the press on the new revision instead of dropping the rest of the drag (leaves are absolute).
2. The runtime captures `artifact_mutations` only; `child_emits` pass through (flow F6: W3-T-FLOWCAD extends the seam after §12).
3. Forms keeps `replace-block` (absolute, already history-editable). A field-parametric leaf needs a design decision (one
   generic `change-block-field{blockId, field, value}` vs per-field leaves; heterogeneous field types) — raised, not improvised.
4. `Ring` controls are not presses (React's `RingView` is not a continuous lane either).
5. Fail-closed mutation types (`OrderedMap`, flow) are dropped plainly inside `ToolTransaction`/`ScrubLedger` (as for every tool
   machine since W1-C); overlay snapshots are retired through the store.

## 5. Open items

- Descriptor regeneration: playbook (`updatePlaybook` describe text).
- Forms parametric leaf decision (§4.3). NodeGraph `setSlider` lane: W3-T2-PROCEDURAL moves gesture/commit to top level.
- Derived previews reading the overlay (F-7): W3-T2-PROCEDURAL adds `ArtifactOwnedToolJobContext::provisional()`.

## Session 2 — 2026-10-01

Successor S2-CONTROLS (coordinator `⚪552b484a…`). Ownership per the coordinator's 12:xx message: `PL/🔋️energy/**`,
`PL/📋️forms/**`, `PL/📖️playbook/**`, `PL/🌍️gis/**`, `PL/📕️norm/📇️registry/🧬️contract/🖥️app-surface/**`,
`RE/🗣️Interpreter/🟦️.tsx` (+ its tests). `FW/🖱️ui/**` is S2-W1E's and `OS/🔌️plugin/**` S2-W2A's: requests for them are in
§S2.6. Nothing committed; no ticket tool called. Scratch: `🗑️generated/s2-controls/`. Status at 21:40 (after the 13:30 and
18:00 usage cuts): source of §S2.2–§S2.4 complete; all TS/Python/lint evidence green; Rust: the forms lib-test build compiles
up to ONE error, fixed (§S2.1), re-run blocked by orphan cargos holding the shared build lock (§S2.5/§S2.6).

### S2.1 Repair (rule 21)

- Predecessor's files diffed: tool-machine crate, runtime glue, hosts and the five plugin call sites were intact (no
  half-finished edit found).
- Peer REPO-PATH-BUDGET rename (ticket 26/10/01/REPO-PATH-BUDGET, word-boundary truncation) had stopped half-way in MY
  trees, so none of the five crates could compile: references and directories disagreed in BOTH directions. Repaired with
  plain `mv` + reference rewrites (no git command):
  - by hand: `🪪️document-contract` → `🪪️document` (`🧬️schema/{🧪️tests,🧫️fixtures}`) in forms, gis terrain, gis map, playbook;
    forms config leaves `📸️replace-config` → `📸️replace`, `🧩️set-contributions` → `🧩️set` (+ their fixture dirs; the leaf
    descriptors' `owner` already named the short path);
  - by the ticket input script `🧪️s2-controls-path-budget-repair.py` (unique word-boundary prefix match at the first missing
    segment; a longer sibling dir MOVES to the referenced short name, except command-module dirs whose reference is rewritten;
    a shorter sibling REWRITES the long reference): 23 directory moves (gis terrain/map + energy + forms Try config test/fixture
    dirs, energy/playbook config leaves, energy `rename-*` refusal cases, `set-camera` cases) and 98 reference rewrites in 14
    files (energy root `🦀️.rs` 35, energy `🏛️mutate-energy-model-1` case, energy leaf fixture tests, forms/playbook root `🦀️.rs`
    command path back to `🧩️set-contributions`). Result: 0 dangling `#[path]`/`include_*` references and 0 descriptor `owner`
    mismatches over the five trees + their hub compositions; feature vector cells all resolve. 22 dangling TS twin imports
    (`👁️viewer/🟦️.ts`, `✏️editor/🟦️.ts` → missing per-window `🟦️.ts`) are pre-existing in HEAD, not from the rename.

- Peer `DslValue::Bytes` variant: forms `semio_value_from_dsl` / `dsl_from_semio_value` (forms artifact `🦀️.rs`) did not cover
  it (E0004, the only error of the forms lib-test build at 17:37) — now mapped both ways to `SemioValue::Bytes` (norm
  app-surface already covered it).
- Coordinator 17:26 ask (playbook kernel `UiInputNode`/`UiSliderNode: Default`): not my edit (5 `..Default::default()` lines
  added 17:21 in `OS/📖️playbook/…/🦀️.rs`); S2-W1E derived `Default` at 17:22. `cargo check -p
  semio-framework-artifact-playbook-playbook --target wasm32-unknown-unknown` (17:28) then failed only in S2-W1E's
  `semio-framework-ui` (`🧊️wgpu/🧩️component/🦀️.rs:2251` `wgpu::layout`, `:2307` `wgpu::stepper` not found); log
  `playbook-kernel-wasm-1.txt`; reported to the coordinator.

### S2.2 Forms `change-block-field` (design §17.1) — schema-first

- Leaf `PL/📋️forms/…/✳️any/🧬️schema/🧬️mutations/🎛️change-block-field/`: descriptor `🔣️.json`; payload schema
  `🧬️schema/🔣️.json` = root `oneOf` discriminated by `field` (18 members: label, description, placeholder, text, unit, schema,
  src, accept, fixtureSlug, required, min, max, step (`exclusiveMinimum 0`), default, params (object), condition
  (`definition.json#/$defs/Expression` | null), options (`value` minLength 1), fields (`key` minLength 1)), every member with
  `x-semio-ui` labels en/de, widget, group, order; `blockId` is `role: target` + `ref {question, fields, field}`; the two
  base-dependent Fatal rules are declared in root `x-semio-invariant` (`bounds-ordered`, `default-answers-kind`) and the
  `🧪️refuses` outcome names `bounds-ordered`. Generated by the ticket input script `🧪️s2-controls-forms-change-block-field-schema.py`.
- Lint fix (repo test domain, `🧪️test/🧬️schema/📋️orchestration/🟦️.ts`, aggregate rule): an internally tagged aggregate's leaf
  may be a discriminated root union (W1-D §6.1) — the tag is accepted when the root OR every `oneOf`/`anyOf` member pins
  `<tag>: {const}`. Without it the readers (union path only when the root has no `properties`) and this lint contradicted each
  other for every root-union leaf of an internally tagged aggregate. Relaxation only: it cannot add findings elsewhere.
- Rust (`🦠️mutation/🦀️.rs`): `BlockField` (adjacently tagged `{field, value}`, typed per field, `None` clears) flattened into
  `ChangeBlockField { block_id, change }` → wire `{mutation:"changeBlockField", blockId, field, value}`; `BlockField::{labels,
  read, applied, changes}`; label en `Change <field> of question "<id>"` / de `<Feld> der Frage "<id>" ändern`; target
  `[blockId]`. Diff (`🔺️diff`): Error `mutation.target-missing`; Warning `mutation.no-op`; Fatal `mutation.invariant` (non-finite
  or inverted min/max, step ≤ 0, default the kind cannot answer, params not an object, empty option value / vector key) and
  Fatal `mutation.duplicate-id` (repeated option value / vector key); applies by patching only the owning step's blocks.
  Inverse (`↩️inverse`): the same field set back to the BASE value. Variant + `KINDS` in the aggregate, forms artifact
  `🦀️.rs` wiring, aggregate schema `oneOf`, TS union, proto (`ChangeBlockField {block_id, field, DslValue value}`, oneof 13),
  graphql type + union, text codec (`change-block-field block-id=… change=<json>`), binary tag 12, three new demo cases.
- TS twin (`🦠️mutation/🟦️.ts`): `parseChangeBlockField` (exactly the schema, incl. `parseFormExpr`), `readBlockField`,
  `applyBlockField`, `blockFieldRefusal`, `diagnoseChangeBlockField`.
- Fixtures (`✳️any/🧫️fixtures/🧬️mutations/🎛️change-block-field/`): `🧾️wire-witness` (options list) + three quintets
  `🧪️rejects` (target-missing), `🧪️refuses` (min 150 > max 100 → Fatal invariant), `🧪️no` (required already true → no-op);
  each with its Rust fixture test (`🧪️tests/🧪️<case>/🦀️.rs`: canonical JSON, applies-to-after, declared outcome + level +
  path, base-derived inverse). Unit laws in `🧬️mutations/🧪️tests/🔬️unit`: field round trip for 5 field kinds, 6 Fatal
  refusals via `assert_fatal_never_applies`, `assert_missing_target_is_error`, no-op, `BlockField::changes` order, en/de label,
  wire witness, kinds count 13.
- Oracle registration (`🔮️oracles/🔣️.json`): catalog `forms-1-any` kind + 3 scenarios, `mutationManifests` entry. Python second
  implementation `🧪️tests/🌵️mutate-forms-1/🐍️.py` extended (`located`, `with_field`, `field_refusal`, INVARIANT rejecting); case
  `🥒️.feature` gains the `🎛️change-block-field/🧪️refuses` row in both outlines; Rust case `KINDS`/`GUARD_VECTORS` extended.
  TS `🪪️document` contract count updated to 30 snapshots / 9 diffs.
- Editor (`✏️editor`): `crate::schema::question_edit_mutations(step, before, after)` = one `change-block-field` per differing
  field, or ONE `replace-block` only when id/kind changed (a re-seeded question — the surviving whole-block intent);
  `update_block_operations` (replaces `update_block_operation`) feeds it. Converted: `patchQuestions`, `patchQuestionOptions`,
  `patchVectorField`, `addQuestionOption`, `removeQuestionOption`, `addVectorField`, `removeVectorField`. No field edit emits a
  whole block any more; an unchanged edit emits nothing.
- Laws `✏️editor/🧪️tests/🧪️field-transactions/🦀️.rs` (wired from `✏️editor/🦀️.rs`): a number-field press = one transaction of one
  absolute `change-block-field` (ticks touch nothing, `TransactionRef` tool `#patchQuestions`, en/de row label), a blur cancel =
  zero trace; a history edit freezes an open press (zero trace, further ticks refused `timeTravel.frozen`, late release
  silent); a history edit of the scrubbed max BELOW a later min replays that min as Fatal `mutation.invariant`, `blocking`, worst
  Fatal, `historyEditFinalize` refused `timeTravel.blocked`, exit = zero trace; a consistent edit replays cleanly, overwrite →
  head equals a fresh run of the edited log.

### S2.3 Hosts (React Interpreter, mine)

- Colour `Input` without `commit` is now a continuous press: picker moves (`input` events) tick the lane, the native `change`
  (caught by a `display: contents` wrapper, deferred past React's handler) releases the press; it was one document edit per
  picker move before.
- Releases name no value (`lane.commit()`): a number field's blur used to release with the DOM value, which a controlled field
  may already have reset to the published value — the release could commit the STALE value (reproduced in vitest: released 1
  instead of 125). Now the press ends on the value last offered.
- Press identities are `<key>:<ms>:<serial>` (page-wide serial) for interpreted and declarative lanes: two presses in one
  millisecond shared an id, and the guest would have swallowed the second as a late tick of the closed first.
- `RingView` (orb drag) speaks the press protocol (window `pointerup` → deferred release, `pointercancel` → `captureLost`);
  it dispatched one edit per frame before. No plugin uses `ring` today.
- Tests: new `🗣️Interpreter/🧪️tests/🧪️continuous-presses/🟦️.tsx` (registered in the Interpreter's in-source block): colour
  press, number press + `retired` on unmount, ring press + cancel. Stale `🪪️container-node-ids` stepper case updated to the
  press protocol (it predates the predecessor's stepper conversion; its `onIntent` also returned a number, which wedges the lane
  — §S2.6).

### S2.4 Census (b) — remaining continuous controls

Mine (energy, forms, playbook, gis, norm app-surface): no `Emit::amend`, no artifact-lane static key left. Keyed emits that
remain are view/transient state allowed by §17.5: energy 3d camera (`window_config`), energy/gis viewer cameras, forms Try
window transient. Findings for decision: (1) energy has exactly three WHOLE-RECORD absolute leaves among its 296 —
`update-site`, `update-ground-temperature`, `update-run-period` (every other inspector field already has a field-granular
`change-*` leaf): a history edit of an earlier press on one of their fields is masked on replay by any later press of ANOTHER
field of the same record (the later leaf re-sets the whole record from its own base). Field-granular leaves for these three
(one per field, or one `change-<record>-field` union as §17.1 decided for forms) would fix it — raised, not improvised; (2) norm `commit_snapshot(_fields)` publishes described edits (row label =
the verb's manifest label, not the leaf label; §16.2) — the 16 `set-snapshot` callers live in norm artifact trees (not mine).

Routing list (other owners; not touched):

| Owner | Remaining continuous-control path |
|---|---|
| GRAPHS (queued) | dag `patch-dag-nodes` `Emit::amend(…, "patch-{field}-{ids}")` (slider value/min/max), dag `move-media-node` amend, hub space `moveMediaNode` amend + per-keystroke parameter inputs |
| FLOW | flow `patchFlowWidgets` static key (F6), `generation-values`, `duplicateWidget:{id}`; `RE/🕸️NodeGraph` `useGraphSliderLanes` press id `${surface}:${widget}:${Date.now()}` (same-ms collision) and no blur/captureLost abort |
| PROCEDURAL | gen3d `gesture_coalesce_key` keys (`widget-field:{gesture}`, `graph-slider:{gesture}`) on the artifact lane |
| PUZZLE | puzzle 3d `set-active-example` static artifact key |
| STROKES | remodel per-frame `Emit::amend` (§15); raster inspection inputs per change (F-4); `RE/🖌️Paint2dHost` brush opacity range dispatches per tick (config lane) |
| S2-W1E (`FW/🖱️ui`) | wgpu colour `Input` is a per-keystroke text field (should press like number: tick per key, release blur/Enter, Escape cancels); wgpu `Ring` not a press; lane `send` returning a non-promise non-`undefined` throws in `settled.catch` and wedges the lane; corpus case `input-number-commits-a-number` (predecessor's open red, unverified now) |
| S2-W2A (`OS/🔌️plugin`) | F-4 `document_input_commit_findings` must exempt `color` (now a press) beside number/file + fixture case |

### S2.5 Verification (so far)

| Command | Result |
|---|---|
| `cargo test -p semio-framework-tool-machine` (private target) | 28/28 pass |
| `cargo check -p semio-framework-tool-machine --target wasm32-wasip2` | pass |
| `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` | 33/33 pass |
| `tsc -p 🧪️w3-t2-controls-typecheck-tool-machine.tsconfig.json` | 0 errors |
| `bun test ./…/forms/…/🧬️schema/🧪️tests/🧪️change-block-field/🟦️.ts` (Ajv + fast-check 4000 runs, generator admits 400–1900 of 2000) | 4/4 pass |
| `tsc --strict` on the forms TS twin, aggregate union and test | 0 errors |
| forms TS contract (`testFormsMutationSchemas`, `testFormsDocumentContractOracle` 30/9, `testFormsDesignImport`) via `🗑️generated/s2-controls/forms-contract.ts` | pass |
| Python oracle on the 3 new vectors + apply/inverse (`🗑️generated/s2-controls/forms-oracle.py`) | pass |
| manifest TS reader `mutationInputAudit` on the leaf schema | 0 findings, 37 inputs |
| `schema mutation-inputs --under ✏️s/🔌️plugins/📋️forms` | 5 findings: change-block-field `leafUncatalogued` (central schema generate, coordinator) + 4 pre-existing forms config-leaf findings (catalog names `📸️replace`, disk `📸️replace-config` — rename debris) |
| `schema mutation-payloads --under ✏️s/🔌️plugins/📋️forms` | 3 findings → fixed (params `additionalProperties`, invariant declared, union-aware tag rule) → **0 findings**, 19/19 payloads, 15/15 leaves witnessed |
| `tsc -p 🧪️w3-t2-controls-typecheck-hosts.tsconfig.json` (+ the two Interpreter test files) | 12 errors, all in peer files (registry generated, Shell, store `line`, ui-contract `UiNumber*`); 0 in Interpreter/tests |
| Interpreter vitest (`SEMIO_TEST_LEVEL=long`, W2-B shim config, `Interpreter/🟦️.tsx`) | run 3: 167/170 (fails: 2 Overlay-padding = peer styling tokens, 1 load timeout); run 5 `-t "continuous presses|interpreted number steppers"`: **5/5** (colour, number, ring presses; both stepper cases) |
| `tsc -p 🧪️w3-t2-controls-typecheck-hosts.tsconfig.json` (run 2) | 4 errors, all peer files (store `line`, Shell); 0 in Interpreter/tests |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts "retained-control-commit" "continuous-gesture-lane"` (react package dir, stock config) | **30/30** after updating the lane suite's press-id pattern to `<key>:<ms>:<serial>` (it pinned the old collision-prone `<key>:<ms>`) |
| same, `"Interpreter/🟦️.tsx"` (stock config, run 6) | **175/177**; the 2 reds are the peer Overlay-padding cases (spacing tokens) |
| `cargo test -p semio-framework-plugin --lib` scrub/tool filter | NOT RUN to completion: blocked first by peer store `HistoryPageStack` churn, then a peer fixture rename (`1️⃣standard-1` → `1standard`, now consistent) |
| `cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-forms-forms --lib --no-run -j 4` (private target) | 17:37: 1 error (peer `DslValue::Bytes` arm, fixed); rebuild stalled 17:41→21:37 behind orphan cargos; 22:26 **Finished, 0 errors** (`forms-test-5.txt`), 1 warning in my files' crate (`🧬️schema/🦀️.rs:50` unnecessary qualification, pre-existing line) |
| same without `--no-run` | 22:31 died: disk full (`No space left on device`, 442 MiB free) — reported; 22:58 (after the prune) **227 passed / 26 failed** (`forms-test-7.txt`), triaged below |
| forms TS conformance / contract / lints after the local-`$defs` change (03:06) | `🧪️change-block-field` 4/4; contract 30/9 ok; `schema mutation-payloads` 0 findings (19/19, 15/15 witnessed); `schema mutation-inputs` **0 findings**, census 15 leaves / 29 inputs (the leaf is now catalogued — central generate ran; my config-leaf dir repair also cleared the 4 old findings) |
| focused forms run `-- field_transaction change_block_field authoring_tests block_field` (03:07) | **peer-blocked**: os-kernel red — a peer's `RecordSpecProducer` refactor (`🗣️dsl/🪟️viewport` E0433 `super::schema`, `🗣️dsl/🧬️schema:988` + `🎒️pack` E0618) |
| forms / energy / gis / norm / playbook crate tests, wasm32 checks | PENDING behind the os-kernel break |

Forms lib-test triage (22:58, `forms-test-7.txt`):
- 22 failures share ONE cause: every bundled forms DSL template/example fails to parse — `TextError "expected LBrace, found
  Ident 'id'"` (`📋️default.forms`, `📋️onboarding.forms`, `📋️building-component.forms`, the demo asset): examples, snapshot
  text/binary round trips, csv/xlsx exports, try wizard, builder, inspector `addQuestion` laws. None of these tests touches a
  file I changed. **Peer-blocked: dsl record-list canonical braces** — a non-fleet peer's in-flight `OS/🗣️dsl` change (new
  `🧬️schema/🛬️decoding`, test `🧾️record-list`, 02:28) requires `{…}` around every record of a record list; migrating the
  committed `.dsl.semio` carriers is part of their change (coordinator 02:5x: do not hand-edit carriers; S2-INFRA watches).
- 2 of my new replay laws: `historyEditInput` refused `timeTravel.schema-unavailable` — root cause: the runtime resolves a
  leaf's `$ref`s only through the OS schema-export registry, which publishes no forms `definition.json`, so the condition
  member's `$ref` to `definition.json#/$defs/Expression` failed the strict reader. Fixed: the generator now embeds the
  definition's `Expression` as the leaf's local `$defs/Expression` (read from `📝️definition/🔣️.json` at generation, single
  source); the manifest reader audits the leaf with NO resolver → 0 findings, 37 inputs. The same gap makes `replace-block`,
  `create-block`, `create-step` (all `$ref` into `definition.json`) not time-travel editable today → coordinator gave it to
  S2-AGNOSTIC (G8, generic: publish every referenced schema); those leaves stay untouched.
- 1 of my new laws (`a_history_edit_freezes_an_open_press_with_zero_trace`): the late release after the session settles with
  "did not retire within 30 seconds"; under investigation.
- 1 (`inspection_controls_bind_to_real_typed_commands`): stale test — it synthesizes a `value` only for `change` bindings, but
  the form title field commits on blur since the predecessor (`commit` trigger) → test now feeds `change` and `commit`.
- Passing: all 3 new fixture quintets, the unit laws, the wire witness, the payload law, kinds catalog (13), the codec
  round trips incl. the 3 new demo cases, `a_number_scrub_is_one_transaction_of_one_absolute_field_leaf`.

### S2.6 Needs a coordinator action

- Central schema generate (new leaf scope `s.forms.forms.mutation.change-block-field`); forms descriptor `describe`
  (mutation roster gains the leaf); playbook descriptor (predecessor's `updatePlaybook` text).
- No activation needed for verification; an activation is needed before any browser probe of the React colour/ring presses.
- REPO-PATH-BUDGET debris outside my trees (read-only census `🗑️generated/s2-controls/path-dangling-census.txt`, Rust
  `#[path]`/`include_*` only): draw 36, sequence 4, stdio 4, remodel 2, sourcing 2 dangling references — their owners can reuse
  `🧪️s2-controls-path-budget-repair.py` (edit `ROOTS`).

## Session 3 — 2026-10-02

Successor S3-CONTROLS (coordinator `⚪b7db773a…`). Same ownership as S2-CONTROLS (`TM` scrub region, `OS/🔌️plugin/🛠️tool-machine`,
the scrub glue region of `PLG`, gis/energy/forms/norm app-surface/playbook call sites, `RE/🗣️Interpreter`). Nothing committed; no
ticket tool called. Scratch: `🗑️generated/s3-controls/`. Status: IN PROGRESS (section kept current at every milestone).

### S3.1 Repair (rule 28)

- Files of my trees newer than the S2 section (03:08) are peer edits only: `store::EngineHandles` → `semio_framework_2d::compute::EngineHandles`
  (energy/gis map/playbook editors), the `RecordSpecProducer` spec accessor `(variants[i].1.ordinary)()` (energy editor op codec),
  hand-written `to_value_controlled`/`from_value_controlled` (energy engine ids), energy path-budget directory renames in
  `DIRECTORIES`, `.dsl.semio` asset carriers (record-list braces). No half-finished edit of mine.

### S3.2 Verification

| Command | Result |
|---|---|
| `cargo test -p semio-framework-tool-machine` (private target `target-nde-s3-controls`) | **28/28 pass** (11:01) |
| `cargo check -p semio-framework-tool-machine --target wasm32-wasip2` | **pass** (11:13) |
| `bun test ./🧰️framework/🔨️modules/🛠️tool-machine/🧪️tests/🧪️conformance/🟦️.ts` | **33/33 pass** |
| `bun test ./…/📋️forms/…/🧬️schema/🧪️tests/🧪️change-block-field/🟦️.ts` (Ajv + fast-check; twin now read through a peer's `🌱️value/🔣️json` codec) | **4/4 pass** |
| `SEMIO_TEST_LEVEL=long bun x vitest run --config ../../🧪️tests/🎚️config/🟦️.ts "continuous-gesture-lane" "retained-control-commit"` (react package dir) | **32/32 pass** (11:41; +2 new lane laws, §S3.3) |
| `cargo test -p semio-framework-plugin --lib -- tool_machine:: tool_run_tests::` (private target, 11:21→12:06 under load ~100) | **37 pass / 5 fail**: every scrub law (overlay fold, refold, bounded tags, F-4 lint law, typing laws) and `tool_run_finalize_is_one_transaction_labelled_by_its_tool_in_every_locale` PASS; the 5 reds are tool-run settings/panel/perf laws outside the scrub glue (`🧪️tests/🔬️tool-run/🦀️.rs:1022` overlay append 125 ms under load, `:1440`/`:1596` settings reconfigure 0≠1, `:1456` resident job, `:1726` panel fixture drift) → S3-W2A region |
| `bun x tsc -p 🧪️s3-controls-typecheck-hosts.tsconfig.json` (scene lane, Interpreter, NodeGraph, lane + press suites) | 4 errors, all peer files (store worker ×2, store sync test, Shell); **0 in touched files** |
| `… vitest run … "engine-contract" -t "graph slider"` | **7/7 pass** (incl. new graph-slider cancel law) |
| `bun ./📜️script.ts schema mutation-labels` (G7 + `labelHandwritten`) | **0 findings in energy / gis / forms / playbook**; norm app-surface `commit_snapshot(_fields)` ×2 remain (§20.4 owner S3-NORM) |
| energy / forms crate builds (12:21, 12:32) | **peer-blocked**: `🧬️schema/📇️registry/🦀️.rs:349,511` duplicate `ArtifactSchemaRegistry`/`SchemaDescriptorRegistryError`, `📡️replication/🎮️mutation/🦀️.rs:218` unlinked `semio_framework_schema_state` (reported to `main`) |

### S3.3 Changes (milestone 12:40)

- **Press identity + lane robustness** (`FW/🖱️ui/🎬️scene/🟦️.ts`): `continuousPressIdentity(key)` (`<key>:<ms>:<serial>`, page-wide serial) is the
  one minting point (Interpreter's two local copies deleted); the lane treats any non-promise port answer as settled (a sink
  returning a number used to throw in `settled.catch` and wedge the lane). Laws in `🎚️continuous-gesture-lane`.
- **NodeGraph sliders** (`RE/🕸️NodeGraph/🟦️.tsx`): press id minted by the press's first send (no same-ms collision, no constant
  fallback id that made a second keyboard press a "late tick"), `onSliderAbort` → `captureLost` on pointer cancel, `blur` on blur;
  `beginSliderGesture` deleted. Law in `🔬️engine-contract` ("cancels a graph slider press …").
- **wgpu presses** (`FW/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` press region, `🌳️tree` `WidgetState.scrub_offered`): `Ring` drags are presses
  (ticks, one release wherever the pointer lets go, `captureLost`/a11y-blur cancel); a colour `Input` without commit policy is a
  press like a number field; a blur whose typed draft is refused releases the press on the value it last ticked (React's lane
  releases on the last offered value) instead of leaving the guest press open. New laws in `🧪️scrub-press` (ring drag + cancel,
  colour field, refused draft) — WRITTEN, compile pending (framework peer break).
- **F-4 lint**: `color` joins `number`/`file` as non-findings (both hosts press colour fields); fixture case added.
- **Closure §20.1 (D1) in my trees**: forms `set-try-value` is a plain window-transient write (coalesce key deleted); gis map viewer,
  energy viewer and energy editor cameras publish ONE config edit per settled camera gesture (both hosts dispatch `setCamera` only
  at gesture end — React trailing debounce, wgpu `WorldCameraSync`), coalesce keys + their test assertions deleted. Left for
  S3-CLOSURE's field deletion: the `coalesce_key: None` lines of the store `Edit` literals (gis terrain/map, forms, playbook, energy
  preparation factories) and the two `emit.coalesce_key = None` resets in `OS/🔌️plugin/🛠️tool-machine/🦀️.rs`.
- **§20.6 labels**: energy reduce no longer carries hand-written descriptions (`model_edit` labels from leaves; load-example,
  simulation settings, result field, cameras: no description); gis map viewer camera likewise.
- **Energy field-granular leaves (coordinator GO)**: `update-site`, `update-ground-temperature`, `update-run-period` DELETED (their
  only producer was `model_edit`, i.e. the inspector/simulation verbs; no import or whole-record intent exists — document loads go
  through `ArtifactStore::reset`). Thirteen leaves, schema-first, generated by `🧪️s3-controls-energy-field-leaves.py`:
  `change-site-{latitude,longitude,elevation,time-zone,north-axis}`, `change-ground-temperature-{building-surface,shallow}`
  `{month, newTemperatureC}`, `change-ground-temperature-deep`, `change-run-period-{start-month,start-day,end-month,end-day,year}`;
  hard bounds in the schemas (EnergyPlus `Site:Location` elevation −300…<8900 m, ≥ −273.15 °C), full `x-semio-ui` en/de; a
  run-period edit whose period stops being a calendar interval of its year is `mutation.target-mismatch` (Error) via the new
  `RunPeriod::is_interval`; every registry updated (root module tree, aggregate, KINDS/DIRECTORIES, wire probes, protocol tags
  287–299, both text grammars, GraphQL, protobuf oneof 924–936, aggregate schema, TS twin, oracle catalog, case feature/Python
  second implementation/Rust adapter; 26 vectors). `model_edit` emits one leaf per changed field; run-period leaves are ordered so
  every intermediate period stays an interval. `SetSite` carries `Option` fields: an inspector press names only its `field`
  (the inspector stops re-sending possibly stale sibling values). New law
  `a_history_edit_of_an_earlier_site_press_keeps_a_later_press_of_another_field`. Fixture quintets: placeholders until the Rust
  writer runs (`SEMIO_ENERGY_WRITE_FIXTURES=1`) — blocked by the peer break.
