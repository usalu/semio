# 🧭️ Plan — Non-Destructive History Editing

Binding contract: `📋️design.md`. Fleet rules: `📌️important/📝️.md`. Evidence: `📓️explore-*.md`.
Aliases: `FW` = `🧰️framework/🔨️modules`, `OS` = `🧰️framework/🛍️products/💻️os/🔨️modules`,
`PZ2D` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

## Waves

| Wave | WP | Owner files (exclusive within this fleet) | Depends on |
|---|---|---|---|
| 1 | W1-A replication | `FW/📡️replication/**` except the `MutationLeaf` region of `🎮️mutation/🦀️.rs` | – |
| 1 | W1-B time-travel module | new `FW/⏪️time-travel/**`; root `Cargo.toml`, `package.json`, taxonomy member line | W1-A types (stub locally until landed) |
| 1 | W1-C tool-machine module | new `FW/🛠️tool-machine/**`; root `Cargo.toml`, `package.json`, taxonomy member line | W1-A `TransactionRef` |
| 1 | W1-D input descriptors | `FW/🛂️manifest` (ArgSchema/ActionArgDef/ConfigFieldShape regions + TS twin), `OS/🗣️dsl/✨️derive`, `MutationLeaf` region of `🎮️mutation/🦀️.rs`, `OS/📡️spr/🎮️command` (`SemanticMutation`), vocabulary fixture, repo lint, roster row in `🔌️plugin` | – |
| 1 | W1-E UI contract | `FW/🖱️ui/**` (contract, Slider, Stepper, UIDialog, wgpu widgets), `DialogDefinition` region of `🛂️manifest/🦀️.rs`, wgpu shell dialog request region | – |
| 1 | W1-F puzzle 2d schema | `PZ2D/🧬️schema/**`, `PZ2D/🧫️fixtures/**`, `PZ2D/🧪️tests/**`, `PZ2D/🔮️oracles/**` | W1-D vocabulary (annotate per design §6 now) |
| 1 | W1-G store core | `OS/🏪️store/**`, `OS/📡️spr/📜️history/**`, `OS/🌿️vcs/**`, retained initializer in `🔌️plugin` | W1-A (integrate Supersede after it lands) |
| 2 | W2-A plugin runtime | `OS/🔌️plugin/**`, `FW/🎠️kernel/**` | W1-A..G |
| 2 | W2-B React shell | `OS/📺️renderer/🧑‍🎨engine/🧱️elements/{🏛️ShellHost,🛠️ShellHelpers,…}`, React i18n bundles, keybindings | W2-A wire |
| 2 | W2-C wgpu shell | `OS/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/**` | W2-A wire |
| 2 | W2-D puzzle 2d tool + engine | `PZ2D/✏️editor/**`, `OS/♾️infinite/🎲️board/**`, `🖥️Board2dHost`, wgpu board coalescer | W1-C, W1-F |
| 2 | W2-E hub rules | `OS/🛢️db/**`, `🌎️hub/**` supersede refusal/grading/check-in | W1-A, W1-G |
| 2 | W2-R* input-UI rollout | per-plugin leaf `🧬️schema/🔣️.json` annotations | W1-D lint |
| 3 | W3-* e2e, convergence, audits | browser probes, two-replica laws, gates, reviews | W2 |

## Common acceptance for every WP

- Tests first: a language-agnostic JSON corpus under the owner's `🧫️fixtures`, validated against a JSON Schema
  under `🧬️schema`, consumed by the Rust implementation and by a TS twin that is checked against a third-party
  oracle (ajv / xstate / fast-check / fast-json-patch / jsonschema as fits). New nx targets follow the
  `⏯️tool-run` package pattern; launch rows are regenerated centrally by the coordinator.
- Compile and test evidence in the report with exact commands and pass counts. "WRITTEN BUT UNVERIFIED" is allowed.
- No legacy paths left: every replaced mechanism is deleted in the same WP.

## W1-A — Replication: `Supersede`, `TransactionRef`, `ReplayReport`

1. `HistoryTransition::Supersede(TransitionSupersede)` tag 6 per design §2: codec encode/decode with malformed
   cases, `history_transition_envelope` dependencies = targets, `HistoryFold.supersessions`, fold law (scope,
   last-wins, unknown target error), `HistoryLog`-agnostic.
2. JSON Schema + corpus (accepted + malformed supersede cases, withdrawn and scoped variants). Move the Python
   generator out of the old ticket folder into a proper test case `FW/📡️replication/🧪️tests/🧪️history-transition/🐍️.py`
   (independent encoder) and regenerate the corpus with it; Rust byte-for-byte test; TS encoder + Ajv test.
   Rename the unregistered `🔀️history-transition-v1` fixture/schema dirs to open-pattern names
   (`🧫️fixtures/🧫️history-transition`, `🧬️schema/🔣️history-transition`) and update all references.
3. Fold law step fixture `🧫️fixtures/🧫️supersede-fold/🔣️.json` (events → expected supersessions per final
   alternative), Rust test + TS fold twin test (TS twin of the fold's supersession resolution).
4. `TransactionRef` on `MutationMeta` and `MutationEnvelope` in every codec (Rust + TS twin + batch codecs +
   fixtures); round-trip tests.
5. `MutationReplayOutcome`, `ReplayReport`, `blocks_finalize()` in `⚔️conflict` with ToValue/FromValue + TS twin.
6. Fix every compile error the new variant/fields cause in consumers across the workspace (exhaustive matches in
   store, spr, hub, CLI, tests) with the minimal correct arm; coordinate with W1-G for the store (send it a
   message when your types land).

## W1-B — `⏪️time-travel` module

Pure session reducer per design §4 with ids, events, effects, refusals (`timeTravel.illegal`,
`timeTravel.stale`, `timeTravel.blocked`, `timeTravel.empty`), Rust owner `FW/⏪️time-travel/🦀️.rs` + TS twin
`🟦️.ts`, JSON Schema `🧬️schema/🔣️.json`, lifecycle-law fixture, Rust unit tests (`🧪️tests/🔬️unit/🦀️.rs`), TS
conformance with ajv + xstate + fast-check (`🧪️tests/🧪️conformance/🟦️.ts`, bun:test), package scaffolding exactly
like `⏯️tool-run` (Cargo.toml, 📋️project.json targets `test`,`test-quick`,`test-long`,`test-exhaustive`,`check`,
📜️script.ts), root Cargo.toml member + workspace dependency, taxonomy `members-of-modules` entry. Must compile for
native and `wasm32-wasip2`. Localized labels for every stage/refusal (en, de) via the same pattern `⏯️tool-run` uses.
Use `protocol` (replication) types for `MutationId`, `InputReplacement`, `ReplayReport` once W1-A lands.

## W1-C — `🛠️tool-machine` module

Design §5: `ToolYield`, `ToolTransaction` reducer, `ToolMachine` trait over `semio-framework-machine`
(`🔄️machine`) statecharts, `ToolMachineRunner`, transaction id minting, `TransactionRef` (from replication).
Rust + TS twin, schema, `🧫️fixtures/🧫️transaction-law/🔣️.json`, Rust unit tests + one example statechart test
(press → drag → release yields one committed transaction; escape aborts with zero trace), TS conformance with
xstate + fast-check + ajv. Package scaffolding like `⏯️tool-run`; target-neutral.

## W1-D — Input descriptors

Design §6 end to end: `x-semio-ui` meta-schema + vocabulary registration; `ArgSchema`/`ArgPresentation`/
`ActionArgDef`/`ActionArgControl` extensions (Rust + TS twin + JSON schema projection), deletion of
`ConfigFieldShape`/`ConfigFieldSpec`/`CommandFieldSpec` and their uses; `MutationLeaf::PAYLOAD_SCHEMA` via the
derive; `SemanticMutation::{payload_schema, payload_value, with_payload_value}` via `#[derive(Mutations)]`;
reader `mutation_input_defs(schema, resolver)` (Rust + TS twin) with local `$defs` and registry `$ref`
resolution and inference rules; input-label glossary (field name → en/de) covering at least the 200 most frequent
input names (see `📓️explore-mutation-input-schemas.md` §2.4b); `WireMutationRosterEntry.inputs`; repo lint
`schema-mutation-input-ui` with a census mode that prints per-plugin missing counts. Language-agnostic fixture:
`{leafSchema, expectedInputs}` corpus read by Rust and TS readers (byte-equal canonical JSON) and validated by a
third-party validator (npm `jsonschema`, Python `jsonschema`).

## W1-E — UI contract

`SliderProps.snaps` (+ tick rendering), `NumberStepper` builder, number `Input` precision, a `vector_input` builder
recipe (Group of labelled numbers), `reference_list` recipe for selection pickers (chips + remove + "use
selection" button), `DialogDefinition.choices` (`DialogChoice{id, label, description?, action, tone, destructive}`)
with React `UIDialog` rendering (focus order, `aria-describedby`) and wgpu `ChromeDialogRequest` rendering of
choices AND staged args; React `Slider` reads snaps from the record; wgpu slider snapping + ticks. Conformance
corpus cases (`🧫️fixtures/🧪️conformance`) for slider-with-snaps, stepper, dialog-choices; run in Rust, React and
wgpu conformance suites. Localized strings via the existing mechanisms (no `is_de`, no `FrozenLabel`).

## W1-F — Puzzle 2d schema

New leaves `drag-selection`, `rotate-selection`, `scale-selection` (design §8) with the full leaf anatomy (payload,
`🔺️diff`, `↩️inverse`, `🧬️schema/🔣️.json`, descriptor, text grammar, binary tag, TS twin, aggregate, `KINDS`,
oracles manifest, fixture quintets incl. partial/target-missing/no-op vectors, Rust leaf tests, Python second
implementation rows, third-party TS/Python cases). `x-semio-ui` on EVERY puzzle 2d leaf input (en/de labels,
widgets, bounds, snaps: dx/dy `snapSource {config: gridFactor}`, angle as dial in degrees display, factor slider
0.1..10 log scale, targets as `Reference{kind: node|targetRegion, domain: vortex, many: true}`).

## W1-G — Store core

Design §3 items 1–9 (Supersede command integration after W1-A lands; start with the effective-forwards accessor,
`EditReplay` stepper with Merge/Report modes, prefix ring, revision accumulator, read API, persistence).
Tests: store laws with the demo/severity fixtures — interior supersede equals fresh replay of the edited log;
Warning/Error/Fatal downstream reported per mutation; withdraw; scoped vs unscoped; two-store convergence under
shuffled arrival; cancel at every step leaves the store untouched; `.ops`/`.spr` round trip; convergence early
exit equals no early exit; language-agnostic replay corpus `🧫️fixtures/🧫️supersede-replay/🔣️.json`
(initial + ops + supersessions → state + per-mutation outcomes) checked by Rust and an independent TS or Python
replay using `fast-json-patch`/`jsonpatch`.

## W2-A — Plugin runtime (`OS/🔌️plugin/**`, `FW/🎠️kernel/**`)

Integration map: `📓️scout-w2a-plugin-runtime.md` (read it fully first). Coordinator decisions on its open points:
all `historyEdit*` verbs are intercepted host-driven at the head of `dispatch_action` (like `toolRun*`), commit
included; `timeTravel` rides `HistoryPatch` AND `log_generation` is bumped on every time-travel stage/progress
change so the patch is always delivered; the new `dispatch_action` arm is `Box::pin`ned and the stack test re-run;
descriptor regeneration is a coordinator chore after you land.

1. `TimeTravelLedger<A>` beside `tool_runs` driving the `⏪️time-travel` reducer (driver contract in
   `📓️w1-b-report.md`): positions resolved at the session base; `BaseMoved` on every store generation change;
   effects: `ShowPreview` (store `state_before(target, accepted)` + apply the draft, keep the draft's own outcome),
   `StartReplay` (store Report-mode `EditReplay` stepper driven per reactor turn ≤ 4 ms, `ReplayProgressed`/
   `ReplayCompleted`), `CancelReplay`, `OpenFinalizePrompt`, `CommitOverwrite`/`CommitAlternative` (store
   `commit_finished_replay` fast path; `Stale` → replay again), `Close`.
2. Reserved verbs (manifest `history_action_definitions` with en/de describe + use_when + arg schemas; MCP-visible;
   viewer-rejected): `historyEditBegin{mutationId}`, `historyEditInput{path, value}` (generic: `payload_value` →
   path edit → schema validation with the W1-D reader → `with_payload_value` → `Draft`), `historyEditUseSelection
   {path}` (current interaction selection of the input's `Reference.domain`), `historyEditWithdraw`,
   `historyEditAccept`, `historyEditDiscard`, `historyEditFinalize`, `historyEditCommit{choice, name?}`,
   `historyEditBack`, `historyEditExit`, `historyEditCancelReplay`.
3. One `render_snapshot_or(committed)` at every render seam (session preview wins over tool-run overlay only while
   a session is active; a session cannot start while a tool run holds provisional state); freeze artifact-lane,
   child, transaction and history verbs with `timeTravel.frozen`; interaction and view verbs keep working.
4. `Emit.transaction: Option<TransactionRef>` (+ `Emit::commit_transaction(runner_output)` helper for
   `ToolMachineRunner` results) passed to the store's transaction slot (W1-G).
5. History model: rows keyed by transaction id (fallback edit id) expandable to per-mutation rows `{mutationId,
   label (MutationKind label, en/de), severity, messages, superseded, withdrawn, editable, position}`; `editable` =
   op has an input schema and does not emit foreign steps. `HistoryEntry`/`HistoryPatch` (kernel Rust + TS twin +
   schema) gain `editId`, `transaction`, `mutations[]`, `timeTravel{stage, target, targetLabel, done, total, worst,
   blocking, fault}`; rows keyed by `editId` not `seq`.
6. `ui_history_panel` (single producer for both hosts): banner section (stage text, target label, progress + Cancel,
   worst severity, "Next problem", Finalize, Exit), editor section (controls built from `mutation_input_defs`
   with the W1-E builders: slider + snaps, stepper, toggle, select, vector, reference list + use selection; Accept,
   Discard, Withdraw; the draft's own outcome), windowed rows (tone + icon + text per severity, actions Edit and
   Revert). Respect UI limits (512 B text, 32-item lists, node budgets, arena).
7. Framework-injected finalize dialog with `choices`: New alternative (name arg, localized default), Overwrite
   (destructive), Back.
8. Tests (toy app, `artifact-app-testing`): begin → preview = state before target + draft, downstream not
   applied; accept → replay → per-mutation outcomes; Fatal/Error block finalize; withdraw resolves; overwrite →
   one `Supersede`, head == fresh fold of the edited log; alternative → Branch + scoped Supersede; exit discards
   (zero trace); freeze; BaseMoved re-replay; cancel; rows per transaction; stack headroom test.

## W2-B — React shell

Delete `frameworkUtilitiesHistoryTab` and every hand-built history row path; render `framework.body.history`
through the interpreter; persistent time-travel band (`role=status`, `aria-live=polite`) from
`HistoryPatch.timeTravel` with Finalize / Exit / Cancel-replay buttons dispatching the reserved verbs; per-window
"time travel" indicator; `ui.timeTravel.*` keys in `UiTranslationSchema` + en/de bundles; remappable chords
`ui.timeTravel.accept|discard|exit` in `SHELL_KEYBINDINGS`; kernel TS twin consumption; tests in the React
suites (explicit include list!) + typecheck.

## W2-C — wgpu shell

Same behaviour in the wgpu shell: history body already Rust-rendered; time-travel band next to
`ShellTransientNotice`; per-window chip; keybindings; finalize dialog through W1-E's choices/args rendering.

## W2-D — Puzzle 2d tool + engine

Board engine (`OS/♾️infinite/🎲️board/**`): one gesture record per gesture `{gestureId, kind: drag|rotate|scale,
targets, dx, dy | pivot, angle | pivot, factor, proximity pairs}`, `select` + terminal row tagged with the gesture
id and flushed as ONE dispatch; both coalescers (TS `Board2dHost`, Rust wgpu). Guest (`PZ2D/✏️editor/**`): the
select utility is a `ToolMachine` (`🛠️tool-machine`) yielding `drag-selection`/`rotate-selection`/
`scale-selection` plus recorded proximity `connect-handles` in one `ToolTransaction` published via
`Emit.transaction`; `translateSelection`, HUD `move`, keyboard nudge, gumball, inspector `delta` reuse the same
leaves; the scratch-fixture diff is removed for gestures; no ambient reads in replay. Tests: one drag = one edit =
one history row labelled from the leaf, carrying `TransactionRef`; cancel = zero trace; two drags = two
transactions; engine and coalescer tests (TS + Rust).

## W2-E — Hub

`OS/🛢️db` + `🌎️hub`: supersede admission (no ownership rule), grading by the replacement ops' conflict targets,
check-in / `replay_envelopes_onto_pair` folding supersessions; refused-supersede handling in the sync actor;
fixtures in `⚔️concurrent-write`; hub unit tests.

## W2-R — Input-UI rollout (per plugin group)

Driven by the W1-D lint census: every mutation leaf input of every plugin gets a resolvable `x-semio-ui`
(labels en/de, widget, bounds, snaps, references) or a glossary entry; lint reaches zero per group.

### W2-R brief (common to every rollout group)

Scope: the leaf payload schemas (`<leaf>/🧬️schema/🔣️.json`) under your group's paths. Census/lint:
`bun ./📜️script.ts schema mutation-inputs [--census] --under <path> [--json]` (cwd and details in `📓️w1-d-report.md`).
For every flagged input add `x-semio-ui` per design §6 and the manifest `$defs/InputUi` meta-schema: `label {en, de}`
(correct domain German — DIN/Eurocode terminology for norm/energy/architect), `description {en, de}` when the name is
not self-explanatory, widget/presentation, `role: target` + `ref {kind, domain, granularity}` for entity ids (kinds from
the artifact's own entity vocabulary), `options` labels for enums, hard bounds (`minimum`/`maximum`) only when
semantically certain, `step`/`precision`/`unit`/`displayUnit`/`displayFactor` where the domain has them, snaps only
when meaningful. Annotate the fields of root-union variants too (reader support is landing). Preserve each file's
existing formatting (script-assisted edits like `🧪️w1-f-annotate-puzzle2d-inputs.py`; keep inputs scripts in the ticket
root as `🧪️w2-r-<group>-*.py|ts`). Never edit the glossary, the reader, the corpus, Rust/TS code or payload structure;
never run cargo. Verify: strict lint for your scope reaches 0 findings except root-union `malformed` and
`refUnresolved` (W1-D follow-up owns those); every committed fixture payload in your scope still validates against
its updated leaf schema (Python `jsonschema` checker). Report `📓️w2-r-<group>-report.md` with before/after census.

## Wave 3 (after W2-A/B/C/D land)

| WP | What | Owner files |
|---|---|---|
| W3-E2E | Coordinator boots puzzle 2d React (6012) and wgpu (6112) from the main session (nohup, supervisor), an executor drives a Playwright battery: drag → one history row with the leaf label → Edit → time-travel band + preview before the drag → slider/stepper edit of dx with grid snaps → Accept → replay progress → outcomes → Finalize → dialog → overwrite / new alternative → head equals expectation; withdraw path; fatal path (delete the dragged node upstream → drag reports target-missing → edit targets via "use selection"); console `[DEBUG]`-free; screenshots en + de. | ticket-local probe scripts |
| W3-T* | Tool-machine conversions per plugin (one executor per plugin): gestures become `ToolMachine`s yielding parametric leaves in one `ToolTransaction` (puzzle 3d, puzzle 5d, draw (delete vendored `fsm` copy → `🔄️machine` + `🛠️tool-machine`), note (`drag-blocks`), shooting (`drag/rotate/scale-assets` net per transaction), lowpoly, fem gumball, flow widgets, cad interactions). | per-plugin editor trees + new leaves |
| W3-G | Gates: descriptor regeneration (`describe` per plugin), launch.json regeneration + `check-generated`, taxonomy report on every new dir, `verify dependencies literal-external`, `verify layering`, docstring/debug-tag checks on touched files, `schema mutation-inputs` strict wired into `schema check`, rust warnings on touched crates. | generated artifacts |
| W3-R | Read-only Sonnet audits of every WP diff: correctness vs design, legacy leftovers, AGENTS.md rules, test coverage gaps → findings fixed by the owning executor. | – |

### W2-S parity brief (common to every parity group)

Lint: `bun ./📜️script.ts schema mutation-payloads [--census] [--under <path>] [--json]` (cwd
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`); classes invalid / undescribed / opaque / layout / aggregate /
unmapped / unresolved. Baseline: `🗑️generated/w2s/census-before.tsv`, root causes `🗑️generated/w2s/aggregate-rootcauses.txt`.
Repo-wide rule (approved): leaf root = the `payload_value()` shape exactly as Rust emits it (camelCase; when the Rust side
is the outlier, fix its `#[value(rename_all)]` together with fixtures, TS twins, text grammars, proto/graphql — all at
once, no compat); internally tagged aggregates: each leaf declares `<tag>: {const: <wire name>}` in properties + required,
aggregate = `oneOf` of leaf `$ref`s; externally tagged: aggregate branch `{type: object, required: [Wire],
additionalProperties: false, properties: {Wire: {$ref: leaf}}}`; adjacently tagged: `{required: [tag, content],
properties: {tag: {const}, content: {$ref: leaf}}}`; `Option` fields → `type: [T, "null"]` where value_derive emits null.
Keep every `x-semio-ui` annotation (move it with renamed properties; stubs you complete get full en/de annotations per
the W2-R brief). Orphan fixtures: re-home to the renamed leaf (read-only `git log --follow`) or delete. Target: 0
findings in your scope for both `schema mutation-payloads` and `schema mutation-inputs`. Rust changes: gated `cargo
check` + the leaf/fixture tests of each crate you changed. Report `📓️w2-s-<group>-report.md`.

## W2-W — Wire-witness conversion (design §11), launched after W2-S publishes its recipe and B2/A finish parity

Recipe: `📓️w2-s-report.md` (conversion section). Breakdown: `🗑️generated/w2s/unwitnessed-breakdown.txt`.
Groups: W-pdf (pdf 1.7 + 1.4: 144) · W-step-ifc (step ap214, ifc 2x3, ifc 4: 65) · W-office (docx, pptx, xlsx: 76) ·
W-text (svg, xml, json, html, md, txt, csv, tsv: 96) · W-media (gif, jpg, tiff, png, bmp, avi, mp4, mp3, wav: 82) ·
W-geometry (obj, dxf, las, ply, stl, dwg, glTF: 65) · W-misc (semio, bcf, zip, epw, deflate, binary: 73) ·
W-norm-1 (en1991: 81) · W-norm-2 (en1996, en1990: 80) · W-norm-3 (din16798, en1993: 72) · W-norm-4 (en1992, en1998,
en1997: 71) · W-norm-5 (en1999, din18599, plugin-level: 37). Each: feature tables → wire-form params, adapters decode
generically (delete hand mapping), Rust + Python/TS adapters updated, `unwitnessed` 0 in scope, both lints 0,
feature cases green (contract/oracle/subject/parity phases), gated cargo per crate.

### W2-W brief (common to every wire-witness conversion group)

Recipe: `📓️w2-s-report.md` "Follow-up 3", F10 (worked example svg `set-declaration`). Design §11. Inventory:
`🗑️generated/w2s-stdio/f10-inventory.tsv`. For every `🥒️.feature` row in scope: `params` becomes exactly the leaf wire
payload (`payload_value()` shape: camelCase, Options as Rust emits, flat Apply content for `payload = Apply` leaves, no
tag); every adapter's per-kind params→op match is deleted and replaced by the generic
`<Aggregate>::from_payload_value(kind, DslValue::from(params))` (Rust) — Python/TS second implementations and oracles read
the same wire params; `no-mutation`/non-mutation rows are removed or reclassified per the recipe; every leaf still
`unwitnessed` gets a payload-only wire witness `🧫️fixtures/🧬️mutations/<leaf>/🧾️wire-witness/🦠️mutation/🔣️.json` checked by
the crate's payload round-trip law. Target in scope: `schema mutation-payloads` 0 (incl. `unwitnessed`), `schema
mutation-inputs` 0, every touched feature case green in its phases (contract/oracle/subject/parity), crate lib tests
incl. `semio_payload_law_*` green (gated cargo, plugin crates `--target wasm32-wasip2` for checks). Report
`📓️w2-w-<group>-report.md`.

### W3-T brief (common to every tool-machine conversion executor)

Goal (user requirement): every tool gesture is a state machine (`🛠️tool-machine` `ToolMachine` over `🔄️machine`
statecharts) that yields mutations inside ONE `ToolTransaction`; tools themselves are never history-editable, their
yielded mutations are (time travel edits their inputs). Reference implementation: puzzle 2d select tool (W2-D,
`📓️w2-d-report.md` §1: statechart `🖱️select/🦀️.rs`, runner persisted in the window transient, `phase` stream/commit/abort,
host aborts blur/captureLost/frozen/baseMoved/retired, preview painted from transient only, `Emit::commit_transaction`,
no `coalesce_key` on commits, one gesture = one edit = one history row labelled from the leaf's `SemanticMutation::label`
(en + de), cancel = zero trace). API: `📓️w1-c-report.md`, design §5 + §7 + §10 (wire contract: `Emit.transaction`).
Per plugin:
1. Census the plugin's gestures (pointer drags, gumballs, brushes/strokes, marquee, transform brackets, keyboard nudges,
   inspector deltas, HUD submits) and how they commit today (absolute final-state ops, per-tick `Emit::amend`, scratch +
   `Emit::commit`, vendored statecharts, host-only brackets). Write it in your report first.
2. Parametric leaves: gestures yield RELATIVE, selection-scoped leaves carrying intent (targets + offset/angle/pivot/factor,
   stroke points, …) — model on puzzle 2d `drag-selection`/`rotate-selection`/`scale-selection` and 🎥️shooting
   `DragAssets`. New leaves are schema-first: `<leaf>/🧬️schema/🔣️.json` with full `x-semio-ui` (labels en/de, widget,
   bounds, step, precision, units, snaps, `role: target` + `ref`), invariants as schema bounds or declared
   `x-semio-invariant`, outcome codes only from the 9-code vocabulary (design addenda; Error target-missing /
   target-referenced / target-mismatch, Warning no-op / partial / clamped, Fatal duplicate-id / invariant / mutation.apply.*,
   Info cascade), Rust leaf + `#[derive(Mutations)]` wiring, TS twin with `parse<Type>()`, committed wire witness
   (quintet or `🧾️wire-witness`), payload law, labels via `SemanticMutation::label` en + de. Replace (not wrap) the old
   absolute ops where they only existed for gestures; delete dead code/leaves/fixtures (no legacy, no compat).
3. Tool machine: statechart + `ToolMachineRunner`, persisted in the window transient when a gesture spans dispatches
   (stable-id configuration), host aborts leave zero trace, preview from transient only, one `Emit::commit_transaction`.
   Delete scratch structs / vendored machine copies / per-tick amend paths that the machine replaces.
4. Hosts: React + wgpu hosts send stream/commit/abort phases; both must behave the same (shared corpus when a coalescer
   exists).
5. Tests (TDD, language-agnostic first): one gesture = one edit = one row with `TransactionRef`; cancel = zero trace; two
   gestures = two transactions; replay determinism (editing the leaf's inputs through time travel re-applies correctly —
   use `state_before`/`begin_report_replay` laws like puzzle 2d); third-party oracle where the plugin has one.
   Lints `schema mutation-inputs` and `schema mutation-payloads` must stay 0 for your scope; `cargo check` wasm32-wasip2
   for the plugin; plugin crate tests; TS tests for the hosts you touch.
Rules: fleet rules in `📌️important/📝️.md` (private `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-nde-<you>`,
gated foreground builds, compile-atomic edits, re-read before edit, no git-modifying commands, don't close the ticket,
never sweep `🗑️generated/`, `[DEBUG] ` prefix for temporary logs and remove them). Don't regenerate descriptors /
launch.json / central schema generate (coordinator does it once at the end) — list what needs regeneration.
Report: `📓️w3-t-<plugin>-report.md`.

## Session 2 — successor roster (coordinator `⚪552b484a…`, 2026-10-01 12:00)

Every session-1 agent died ~08:00. One successor per in-flight WP; same owner files as session 1 (table above + the W3
briefs). Common duties: fleet rules 1–24 (`📌️important/📝️.md`, esp. 21 repair-first, 22 moved layout, 23 no channel bump /
no activation), append `## Session 2` to the predecessor's report, private `CARGO_TARGET_DIR=…/target-nde-s2-<wp>`.
Product acceptance (dev's goal, binding for every WP): every history mutation is editable; editing enters time-travel mode;
the edited mutation shows with downstream NOT applied; accept/discard; inputs render from schema UI metadata (slider + snaps,
stepper, min/max, select, vector, reference + "use selection"); accept replays downstream with per-mutation success /
warning / error / fatal; warnings stay visible in history (also after finalize and reload); fatal/error block finalize until
edited or withdrawn ("Next problem"); after a clean review the user can finalize or keep editing other mutations; finalize
prompts New alternative vs Overwrite; tools are state machines yielding mutations in ONE transaction; artifact-agnostic.

| WP (session 2) | Owner files | Assignment |
|---|---|---|
| S2-W2A runtime | `OS/🔌️plugin/**` (shared regions with S2-W1G §15 and S2-FLOWCAD §12 — region-scoped compile-atomic edits), `FW/🎠️kernel/**` | finish the locale-neutral edit-verb pass (labels survive reload; `🧪️w2-a-edit-verb-*.py` scripts), compile native + wasm32-wasip2, time-travel laws + plugin suite; prove acceptance items: multi-mutation drafts from Reviewing, persisted warnings after finalize + reload, Next problem, progress + cancel |
| S2-W2B React | React shell elements, i18n bundles, keybindings | repair + typecheck (fake `PluginWasmHandle.readAppDocumentIdentity`), adopt any kernel wire change from S2-W2A, re-verify R2-2/R2-4/R2-6 fixes, React UX/a11y pass over the whole time-travel flow |
| S2-W2C wgpu | `🐚️Shell/🎯️targets/🧊️wgpu/**` | finish (5) native folder re-attach parity; full wgpu parity of band/editor/dialog/alternatives/peers/notices; wasm32 check; wgpu probe prerequisites |
| S2-W2D puzzle 2d | `PZ2D/✏️editor/**`, `OS/♾️infinite/🎲️board/**`, `🖥️Board2dHost`, wgpu coalescer | re-run the verification its follow-up left pending (`🗑️generated/w2d-test-*.txt`), wgpu shortest-decimal replay, "use selection" for drag-selection targets, nudge/HUD/inspector reuse |
| S2-W1G store | `OS/🏪️store/**`, `OS/📡️spr/📜️history/**`, `OS/🌿️vcs/**`, retained initializer + `tool_transaction_shape_fault` region | design §15 transaction-scoped amend (store + runtime shape rule + TS twin + laws), follow-up-4 open items; signal S2-STROKES when §15 lands |
| S2-PUZZLE | puzzle 3d/5d trees | verification + rotate-pivot parity, attraction/fastener re-solve, paged relocate scan (progress + cancel), derive cfg warning, §7 commands |
| S2-DRAW | draw + note trees | continue from `📓️w3-t-draw-note-report.md` |
| S2-SPATIAL | shooting, fem 2d/3d, lowpoly trees + `World3dHost` | compile + verify fem move-selection leaves + gumball machines, lowpoly, delete the bracket rule; then the World3dHost live consumer for generation3d (old W3-T-GEN3D) |
| S2-FLOWCAD | flow + cad trees, composed-child framework seam (§12) | §12 composed-child transactions end to end (parent + child stores, history rows with `store`, time travel on member store, parent re-derive) |
| S2-LAYOUT | layout tree + `Canvas2dGumballOverlay` | continue from `📓️w3-t-layout-report.md` |
| S2-CONTROLS | `🛠️tool-machine` ScrubMachine + glue, gis/energy/forms/norm/playbook control call sites | verification of the source-complete scrub work + fixes |
| S2-TEXT | typing-run machine + text plugin trees (§13.2) | reconstruct (`🗑️generated/w3-t2-text/`), finish, write `📓️w3-t2-text-report.md` |
| S2-STROKES | strokes / wfc / remodel trees | continue; remodel on §15 after S2-W1G |
| S2-PROCEDURAL | procedural incl. generation3d trees | reconstruct (`🗑️generated/w3-t2-procedural/`), finish, write `📓️w3-t2-procedural-report.md` |
| S2-TAX | `📚️library/🔣️taxonomy.json` (Edit tool only), taxonomy engine | structural leaf identity from mutation catalogs; restore the sealed `📽️nested-cargo-package-projection` catalog; workspace-contract package-move re-target |
| S2-CODES | outcome-code fixtures/adapters | drop the "warn" alias (70 fixtures), generation3d/workflow checked-apply adapters, remodel oracle, deferred reruns |
| queued | norm-2, norm-3, media, stdio cases, GRAPHS, CLOSURE, E2E, W3-G gates, W3-R audits | launched as slots free |

## Session 3 — successor roster (coordinator `⚪b7db773a…`, 2026-10-02 11:00)

Session 2's fleet was cut ~03:45. One successor per WP with the same owner files; fleet rules 1–31. Focus: verify → fix →
close (rule 29). Product acceptance unchanged (Session 2 paragraph above). Reports: append `## Session 3 — 2026-10-02`.

| WP (session 3) | Model | Assignment |
|---|---|---|
| S3-INFRA | opus | activation blocker (a): wgpu frame-worker "browser import is not schema-owned" (`🌱️value/🧬️schema/🌳️intrinsic`, `🎠️kernel/🫧️transient`) — schema-owned fix, prove with the `generate-frame-worker` target; audit wave A I-1..I-3; I2 `Bytes` arms compile proof; peer-break watch (workspace load, taxonomy load, guest gate) |
| S3-PUZZLE | opus | activation blocker (b) FIRST: puzzle 3d `📚️examples/🌲️concrete-forest/🦀️.rs:31:125` panics in the descriptor probe (wasm); then S2.5 verification list + resume list |
| S3-W2A | opus | owed plugin/kernel laws (history_patch, history_notices, history_edit, time_travel, composed_child_history), native + wasm32-wasip2 checks, §8 open items, acceptance items (multi-mutation drafts from Reviewing, persisted warnings after finalize + reload, Next problem, progress + cancel, G9 runtime adoption) |
| S3-W1G | opus | owed kernel laws + per-test suite, plugin suite, FU4 hub bin laws; follow-up 4 (hub forced rebuild + rebootstrap test, refused-step text en/de, TS timestamp twin); §15 status proof |
| S3-W1E | opus | S2.5 runtime law, S2.5b wgpu renderer compile + tests, S2.5c corpus law; display facets for the shell staged-arg slider (agree region with S3-W2C) |
| S3-W2B | opus | audit B-1 (§18 law on staged vector axes) + B-2 (ShellHost reveal/focus test); React typecheck + suites; S2.4 open items |
| S3-W2C | opus | S2.9 open items, native folder re-attach parity (5), staged-arg slider facets in `staged_arg_row`, wgpu tests + wasm32 check, `--renderer=wgpu` probe prerequisites |
| S3-W2D | opus | S2.6 verification, S2.7 items, wgpu shortest-decimal replay proof |
| S3-AGNOSTIC | opus | §6 "Next" list, G7/G8/G12 gates to a verdict, per-plugin acceptance table, routed findings |
| S3-DRAW / S3-SPATIAL / S3-FLOWCAD / S3-LAYOUT / S3-CONTROLS / S3-TEXT / S3-STROKES / S3-PROCEDURAL / S3-GRAPHS | opus | the "verification still to run" + "open" sections of their session-2 report sections, then remaining conversions |
| S3-GAP | sonnet | read-only: goal-level gap analysis on the code (G1..G14 re-evaluated + new) → `📓️s3-gap.md` |
| S3-CLOSURE-CENSUS | sonnet | read-only: census of every remaining non-machine gesture / artifact-lane amend / coalesce key / brackets / preview contracts + CLOSURE brief → `📓️s3-closure-census.md` |
| queued | – | S3-CODES-TAX (C-1..C-5, T-1, T-2), S3-NORM (owed cargo + 47 rule-1 breaches), S3-STDIO (WP-2/WP-3, cases-2, media), S3-E2E (after serve), S3-CLOSURE, S3-GATES, audit wave B, coordinator regenerations |
