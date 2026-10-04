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

### S3.4 Resume after the 13:05 usage cut + 17:00 reboot (18:40)

- Every S3.3 edit and the §20.1 config-lane press (below) were on disk intact; the energy config-press law was complete
  (no half edit). Cargo waits on the peer schema-split fix (S3-INFRA): last plugin check (13:0x) red only in
  `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1266-1269` (`semio_framework_schema::{with_schema_export_registry, SchemaFormat,
  registered_referenced_schema_documents}` missing — peer schema split).
- **Config-lane press (design §20.1, coordinator item)** in `OS/🔌️plugin/🛠️tool-machine/🦀️.rs`: `settle_press_config` holds a tick's
  `config_mutations` + `window_config_mutations` per window (each tick replaces; late tick / late release of the press the window
  closed stays silent; `captureLost`/`blur`/`frozen`/`retired` drop with zero trace); render seams read
  `config_overlay_or` / `window_config_overlay_or` (4 seams in `PLG`: render, engagements, window measures, tool measures; context
  menu and poll keep reading what landed); overlays folded with the shared `fold_leaf` and retired through the config store /
  `WindowConfigOwnerRegistry::preview|retire_preview` (new, `🪟️window/🎚️config/🦀️.rs`); release publishes its own config lanes as
  ONE config edit with `coalesce_key` cleared; `A::host_configuration_mutation` verbs ride the same press. Test accessors
  `config_generations`, `rendered_config`. Law (energy):
  `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row`. API note `📓️api-scrub-machine.md` §4.
- Taxonomy (`verify taxonomy report --scope …/🧬️schema/🧬️mutations`): the 6 non-truncated new leaf dirs are clean except the
  pre-existing `directory-kind-unresolved` class of every case dir (596 in energy); the 7 path-budget-truncated dirs
  (`🌡️/🌱️/⛏️change-ground-temperature`, `🛫️/▶️change-run-period-start`, `🛬️/⏹️change-run-period-end`) join the existing
  `projection-member-unresolved` + `mutation-payload-schema-authority-invalid` class of the 153 truncated energy dirs (S3-TAX
  structural identity). Fixture dirs report `scenario root is absent` until the Rust writer fills the placeholder quintets.
- Verification after the resume (no cargo yet — every plugin-crate build queues behind ~15 peer cargos on the shared build-dir):

| Command | Result |
|---|---|
| `… vitest run … "Interpreter/🟦️.tsx" -t "continuous presses|interpreted number steppers|scrub protocol"` | **5/5 pass** |
| `… vitest run … "Interpreter/🟦️.tsx"` (whole suite) | **182/184**; the 2 reds are the known peer Overlay-padding cases |
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🔋️energy` | **725/725 inputs of 304 leaves carry a UI descriptor**; 16 findings, all catalogue state: 13 `leafUncatalogued` (new leaves) + 3 `malformed` (deleted leaves still catalogued) → central `schema generate` |
| Python `ast.parse` of the case's second implementation; JSON parse of oracles, aggregate schema and all 595 descriptor/payload schemas | pass |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib -- scrub_press control_commit` (private target, 19:18) | **14/14 pass** (incl. the 3 new laws: ring drag + cancel, colour field press, refused-draft release) |
| `cargo test -p semio-framework-ui --features wgpu-engine --lib` (whole crate) | **768/768 pass** |
| energy lib-test build (19:01–19:17, 4 attempts) | peer-blocked: lock cycle (cleared by the coordinator), then `🗣️dsl/🦀️.rs:931` `canonicalize` ambiguous (peer, fixed within a minute), then `PLG:3015…39124` `dsl::LanguageSpec` / `preflight_languages` / `register_languages` removed by the peer dsl refactor before its plugin callers were updated — 13 errors, none in my regions |

### S3.5 Open items (19:30)

- **Owed verification (cargo; framework red from the peer DSL crate extraction, coordinator "TREE GREEN" pending):**
  energy fixture writer (`SEMIO_ENERGY_WRITE_FIXTURES=1 cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-energy-model --lib --
  writes_the_committed_vector_when_requested`) then the full energy lib suite (26 new vectors, field-leaf laws, config-press law,
  scrub laws, structural correspondence) + the case adapter / Python second implementation; `cargo test -p semio-framework-plugin --lib
  -- tool_machine:: tool_run_tests::` (F-4 colour case, config press compile); forms
  (`field_transactions change_block_field block_field try_value` + the frozen law's `[DEBUG]` investigation: "did not retire within 30
  seconds" on the late release), gis terrain/map, playbook crate tests; wasm32-wasip2 checks of the five plugin crates.
- `a_history_edit_freezes_an_open_press_with_zero_trace` (forms) still carries the predecessor's `[DEBUG]` eprintlns; they leave
  with the fix.
- Taxonomy: the 7 truncated energy leaf dirs join S3-TAX's existing `projection-member-unresolved` class (153 in energy).
- Decision recorded (§S2.4 finding 1): resolved by the coordinator's GO — energy whole-record leaves replaced (S3.3).

### S3.6 Coordinator actions (exact)

1. Central `schema generate`: catalogue the 13 new energy leaf scopes (`s.energy.model.mutation.change-site-{latitude,longitude,
   elevation,time-zone,north-axis}`, `…change-ground-temperature-{building-surface,shallow,deep}`, `…change-run-period-{start-month,
   start-day,end-month,end-day,year}`) and drop the 3 deleted (`update-site`, `update-ground-temperature`, `update-run-period`) —
   clears the 16 `schema mutation-inputs` findings under energy.
2. `describe` energy (`set-site` args no longer required; config-lane/label changes), playbook (descriptor still carries the old
   `updatePlaybook` describe text "consecutive edits merge into one undo step"), forms (`change-block-field` roster, if not yet).
3. S3-CLOSURE: with `Emit.coalesce_key`/`Edit.coalesce_key` deletion remove the `coalesce_key: None` lines of the store `Edit`
   literals (gis terrain `✏️editor/🦀️.rs:278`, gis map `:593`, forms `:768`, playbook `:332`, energy `~:1992`); the glue's
   resets are gone (S3.7 note). My trees have no `coalesce_key: Some(…)` left.
4. S3-NORM (§20.4): norm app-surface `commit_snapshot(_fields)` hand labels (`📇️registry/🧬️contract/🖥️app-surface/🦀️.rs:1582,1590`).
5. S3-SPATIAL: fem playback can drop `PLAYBACK_COALESCE_KEY` once its hosts send `{gesture, commit}` (config-lane press, S3.4).
6. Activation before any browser probe of the React/wgpu press changes (colour, ring, NodeGraph sliders, config presses).

### S3.7 Audit `📓️audit-s3-tools.md` C1–C3 (19:55)

- **C1 (energy witnesses)**: the 130 placeholder files are now real quintets, authored by the ticket input script
  `🧪️s3-controls-energy-field-witnesses.py` from the committed `🔢️change-model-version/⛔️refuses` base (default model "BESTEST 600",
  version "1"; the 26 leaf-test scenarios now pin `version: "1"` so the Rust writer reproduces the same base) in the Rust writer's
  exact shape (applied: whole regenerated model + unchanged child handles; refused / no-op: all-null diff, untouched document).
  Independent checks: `bun ./📜️script.ts schema mutation-payloads --under ✏️s/🔌️plugins/🔋️energy` → **0 findings, 600/600 payloads,
  301/301 leaves witnessed, 138 negative witnesses rejected**; the case's Python second implementation (loaded with a stub harness)
  → **26/26 vectors agree** (after-document, status + code, and its own inverse restores the before-document). The Rust leaf laws
  (forward, inverse, canonical, outcome, diff, absorb) are owed with the energy lib run. A dedicated `{}`-body gate is not added:
  the payload lint rejects an empty mutation payload and every leaf test decodes all five files.
- **C2 (forms frozen-press law)**: root cause found by reading the harness — `artifact_app_laws::settle_registered_typed_operation`
  drained ui scopes but never took a UI-progress frame, so a time-travel `HistoryPatch` left pending by `historyEditExit` (the
  pump returns as soon as the stage reads `None`, before taking it) kept `time_travel.has_pending_work()` → `has_pending_typed_operations`
  true for the late release's settle until the 30 s deadline. Fix (S3-W2A region of `PLG`, one loop + docstring): the helper also
  takes every UI-progress frame like the host does (`receipt.ui_scope` keeps the first real scope). The five `[DEBUG]` eprintlns are
  removed. WRITTEN, run owed (TREE GREEN).
- **C3**: `settle_press_config` — a late release and an abort both clear the config lanes and answer `false`; only a live release
  publishes.
- Note: a peer (S3-CLOSURE) already deleted the `coalesce_key` resets in my glue; my trees' only remaining `coalesce_key` text is the
  store `Edit` literals S3-CLOSURE removes with the field.

### S3.8 Owed runs after "TREE GREEN (core)" (2026-10-03 05:47–06:10)

| Command | Result |
|---|---|
| on-disk check after the 21:00 cut | 130/130 energy quintet files non-empty (S3.7 C1 complete); forms frozen law has 0 `[DEBUG]` lines; the settle-helper UI-progress loop is in `PLG` |
| `cargo check -p semio-framework-plugin --lib --features artifact-app-testing` | **pass** (06:08); 0 warnings in `🛠️tool-machine` / my `🪟️window/🎚️config` additions (1 peer warning `🎚️config/🦀️.rs:455`) |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` | **pass** (06:09) |
| `cargo test -p semio-framework-plugin --lib -- tool_machine:: tool_run_tests::` | **peer-blocked** (05:55–06:00): test modules not yet migrated to the DSL crate's `ValueError` (`🧪️tests/🖥️test-app-mutations-document` ×22, `🧪️tests/🧩️composition` ×17, `🔬️app-declarations-fixture` ×10, `🏗️builder/🧪️tests/🔬️schema-stamping` ×8, `🧬️mutation-fixtures-{transaction,surface,dummy}` ×8 each) + `⏪️time-travel/🦀️.rs:2311` (`TimeTravelLabel::MemberEdited`, landed 05:55) — 0 errors in my files |
| energy / forms crate tests (05:47, 06:01) | **peer-blocked** in their stdio dependencies: `🌦️epw/…/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs` (139 errors) and `🎒️zip/…/🌐️iso21320/🧬️schema/🦀️.rs:207-213` + `🎒️zip/…/🧱️base/…/🪶️sqlite/🦀️.rs` — same `ValueError` migration (reported to `main`); 0 errors in energy/forms sources before the dependency stops the build |

### S3.9 Status at the 10:45 resume

- No interrupted edit: C1 (130/130 quintets non-empty; `schema mutation-payloads --under energy` still **0 findings, 600/600,
  301/301 witnessed** at 10:43), C2 (0 `[DEBUG]`, settle-helper UI-progress loop on disk), C3 on disk.
- 06:52 cheap check: the kernel itself was red from the peer DSL `ValueError` migration (`🏪️store/🦀️.rs` ×12, `🚪️io` ×4,
  `🧬️semio` ×3, `📜️space-history` sqlite ×12, snapshot-capability codecs ×2) — no further polling per the coordinator.
- **Owed, in order, once the ✏️s workspace compiles** (one gated cargo each): (1) energy lib suite incl. the 26 vectors (the Rust
  writer must reproduce the authored quintets byte-for-byte modulo key order), `a_history_edit_of_an_earlier_site_press_keeps_a_later_press_of_another_field`,
  `a_config_press_is_one_config_edit_a_cancel_is_none_and_neither_is_a_history_row`, scrub + F-4 laws; (2) plugin
  `tool_machine:: tool_run_tests::` (needs the peer's test-module `ValueError` migration); (3) forms
  `field_transactions change_block_field block_field try_value` (C2 proof); (4) gis terrain/map, playbook, norm registry contract
  crate tests; (5) wasm32-wasip2 checks of the five plugin crates.

### S3.10 CLOSURE-3 — the release is transactional (2026-10-03 11:00–11:40)

Finding: `settle_tool_operation` settled the config press (hold / release, press marked closed) BEFORE the fallible
`scrubs.send(..)?` / `child_scrubs.send(..)?`, and `ScrubLedger::send` itself recorded a release as closed even when the scrub
refused it — a refused release lost the final config value silently. Fix (structural, the audit's "extract one PressLedger"):

- **One press ledger for every lane** (`OS/🔌️plugin/🛠️tool-machine/🦀️.rs`): `ToolMachineRuntime.presses: ScrubLedger<PressLeaf<M,
  CM>>` with `PressLeaf::{Member, Child, Config, WindowConfig}` replaces `scrubs` + `child_scrubs` + the hand-kept config presses
  (`ConfigPress`, `config_presses`, `config_closed`, `config_changed`, `hold_config` / `release_config` / `abort_config` /
  `retain_config_windows`, `settle_press_config` — all deleted). `settle_press` moves every press lane of the emit into ONE
  `presses.send`; only its `Committed` step hands the lanes back (document lanes stamped with the press's ONE `TransactionRef` —
  previously the child ledger minted a second ref —, config lanes as ONE config edit, never a history row). A late input is
  silent on every lane; aborts / freeze / roster retirement are one ledger call each; the document / config / window-config
  overlays refold from `presses.provisional()`. The host-configuration branch of `dispatch_action` (`PLG`) calls the same
  `settle_press` (base = live content revision). Net: no fallible step between hold and release, one closed-press memory.
- **`WindowConfigMutation: Clone`** (`🔌️plugin/🪟️window/🎚️config/🦀️.rs`): the erased value is a private
  `ErasedWindowConfigMutationValue` (`as_any` / `into_any` / `clone_value`, blanket over `Clone + Send + 'static`; every owner
  mutation is `Clone` through `protocol::Mutation`), so a press holds window-config leaves; `begin` checks the type before it
  moves the value, so a mismatch still hands the mutation back.
- **`ScrubLedger::send` transactional** (Rust `🧰️framework/🔨️modules/🛠️tool-machine/🦀️.rs` + TS twin `🟦️.ts`): the open state is
  restored and nothing is recorded closed when the scrub refuses; the press stays open until a retry or a host abort decides.

Laws:

| Law | Where | Result |
|---|---|---|
| `the_scrub_ledger_never_refuses_and_late_inputs_change_nothing` — exhaustive: every order of ≤ 4 inputs (2 presses × 2 tools × 2 revisions, empty / non-empty ticks, releases, host abort; 137 560 sends) answers a step, never a refusal; a late input leaves the ledger byte-equal; a live release always closes its press | tool-machine unit | **pass** (`cargo test -p semio-framework-tool-machine --lib`: 33/33, 11:20) |
| TS twin: random ledger inputs across windows, tools and revisions are never refused; a released press answers `idle` to a late tick / release with open state + overlay unchanged (fast-check, 400 runs) | tool-machine conformance | **pass** (`bun test`: 34/34, 11:12) |
| `every_lane_of_a_press_rides_one_ledger_step` — a tick holds document + config leaves as overlays over untouched committed values; the release commits both lanes in ONE step; a late release is silent on every lane; a config-only press leaves the document overlay empty; an abort leaves zero trace and its late release stays silent | plugin glue (`🔌️plugin/🧪️tests/🧪️scrub`) | WRITTEN — compiles (0 errors in my files); the lib-test target is **peer-blocked** by the test-fixture `ValueError` migration (129 errors in `🧪️tests/🖥️test-app-mutations-document`, `🧩️composition`, `🔬️app-declarations-fixture`, `🧬️mutation-fixtures-*`, `🏗️builder/🧪️tests/🔬️schema-stamping`) |

Note on "a refused release": the scrub chart cannot refuse (every `Tick` lands in `scrubbing`, every `Commit` yields `Commit` as
its last yield, the ledger only resumes states it persisted) — the exhaustive law pins this, so the refusal arm is the type's
defensive contract (restore, never close). A release refused DOWNSTREAM of settlement (store admission of the published edit)
surfaces as the operation's `Fault` with both lanes unpublished — never silent; keeping such a press open would need a two-phase
release through the publication ladder (open item).

Verification (11:12–11:40): `cargo check -p semio-framework-plugin --lib --tests --features artifact-app-testing` → lib target
**pass**, 0 warnings in `🛠️tool-machine`, `🧪️scrub`, `🧪️typing` (the typing law now drives `presses`); 1 pre-existing peer warning
`🎚️config/🦀️.rs:481` (`let mut publication`); `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2` → **pass**
(11:38). Owed on TREE GREEN: the glue laws above with `tool_machine:: tool_run_tests::` and the energy config-press law
(end-to-end through the unified ledger).

### S3.11 Owed runs after "TREE GREEN (core tests, 11:38)"

| Command | Result |
|---|---|
| `cargo test -p semio-framework-plugin --lib --features artifact-app-testing -- tool_machine:: tool_run_tests::` (11:39–11:45) | **39 passed, 4 failed**. All 7 `tool_machine::` laws pass, incl. the new `every_lane_of_a_press_rides_one_ledger_step` and the typing laws now driving `presses`. The 4 reds are the documented baseline (`📓️w2-a-report.md` §6.4, wp-c13): `tool_run_panel_of_a_running_run_is_the_shell_fixture` (`generation: 0` vs fixture `0.0`), `tool_run_reconfigure_resume…`, `tool_run_settings_changed…`, `tool_run_window_settings_reads…` (the 8-turn budget runs out while 255 tick-overlay retirements are pending). None touches the press path: `set_target` dispatches straight on the config store, and `follow_tool_machines` returns at once with no press open |

Still owed (stdio peer, epw/zip): energy lib suite incl. the config-press law, forms `field_transactions`, gis/playbook/norm crate tests,
and wasm32 checks of the plugin crates.

### S3.12 Playbook on the child lane (design §20.15; coordinator 11:50) — design

Finding (`parentLeafReadsChild`, 8): `add-step`, `remove-step`, `move-step`, `update-step`, `add-block`, `remove-block`,
`move-block`, `replace-block` are parent-lane leaves of `PlaybookSnapshot` that read the steps off the `flow` child's local owner
(`playbook_working_scene`) and re-mint BOTH content-addressed children (`document`, `flow`) on every edit; the parent pack also
carries the steps as JSON text (`PlaybookPackRecord.steps`) — a materialisation of child content in the parent.

Design (decided, opinionated):

1. **One source of truth, the `flow` child** (`s.stdio.semio@v1/flow`): a step is a node (`id`, `kind = "step"`, `label` = title,
   params `blocksJson` and, when set, `description`); step order is the chain of `sequence` edges `seq-<a>-<b>` (`next` → `prev`),
   not the node vector — `insert-node` appends and the inverse of `remove-node` re-appends, so vector order cannot survive an undo;
   a chain can. Readers walk the chain from its head (nodes outside it follow in vector order).
2. **The `document` child slot is deleted.** It was a write-only narrative projection (no reader; already stale after every
   `change-title`); keeping it would need a second child lane per edit. A narrative is a read-side export, composed on read.
3. **Writers are child-lane leaves.** The verbs emit `ChildEmit::of::<SemioFlowSnapshot>("flow", id, leaves)` with stdio-flow
   leaves: add step = `insert-node` + chain edges; remove = chain edges + `remove-node`; move = chain rewiring only (node
   identity kept); block edits = `set-node-param blocksJson` (absolute per step). Retained route: the bounded reducer reads the
   children from the job context (`ArtifactView::with_children`), publication lane `Child`. The 8 parent-lane leaves, their
   fixtures, registries (enum, kinds, binary protocol, text grammar ×4, JSON/proto/GraphQL/TS, oracles, case adapter) are deleted;
   `change-title` stays the one parent-lane leaf.
4. **Readers compose on read**: `playbook_composed_spec(snapshot, children)` (title + chain steps) feeds every window, the viewer,
   the interaction topology and `import_media`; an uncomposed child is a named fault, never a local-owner fallback. The working
   scene (`PlaybookWorkingScene`, `attach_playbook_steps`, `seed_playbook_scene_json`, the scene-owner law) is deleted.
5. **Stable child ids + genesis catalogue**: the child id is minted once (`playbook-flow` for a new playbook,
   `playbook-demo-flow` for the demo) and never re-minted; `genesis_child_pack` answers only those ids from the plugin's own
   catalogue (the empty playbook's flow, the demo's flow DSL asset) — no parent content is read.
6. **Laws**: child-leaf vectors per verb (exact leaves, applied order, undo of a middle remove restores order), and
   `composed_reload_law!("playbook", …)` beside the existing G12 law.

Known interim gaps (framework seams owned by S3-AGNOSTIC, pending its gated wave): txt/json export and import serialise the parent
only (no steps) until serializers receive `ArchiveChildren`; inference sees no steps until it receives the child packs as
`dependencies` — the same state flow and sequence are in.


## Session 4 — 2026-10-04

Continued by S4-TOOLS-B in `📓️s4-tools-b-report.md` (one report for both inherited WPs, fleet rule 34).
