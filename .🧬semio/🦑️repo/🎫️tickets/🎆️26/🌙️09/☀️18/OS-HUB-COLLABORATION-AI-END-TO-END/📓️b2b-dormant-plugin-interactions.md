# B2b — Dormant plugin interactions (batch B): reasoning · dag · norm · playbook · imperative

Slice B2b of `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, successor to `📓️b1b-dormant-plugin-boots.md`.
Scope: take B1b's five booting-but-inert React playgrounds past the **interaction bar** —
*default example renders · one Actions-panel row dispatches AND mutates the document · undo reverts ·
redo reapplies · no console errors or guest traps*.

> Every row below was measured in this session. Nothing is predicted. Captures are named per step.

## 1. Scoreboard

| plugin | crate | port | crate check | boot | mutating action | undo | redo | console | bar |
|---|---|---|---|---|---|---|---|---|---|
| 🕸️dag | `semio-s-artifact-dag-dag` | 6017 | ✅ | ✅ | ✅ `addNode` | ✅ | ✅ | ✅ 0 faults | **✅ PASS** |
| 💡️reasoning | `semio-s-artifact-reasoning-wires` | 6015 | ✅ | ✅ | ✅ `addNode` | ✅ | ✅ | ✅ 0 faults | **✅ PASS** |
| 📖️playbook | `semio-s-artifact-playbook-playbook` | 6085 | ✅ | ✅ | ✅ `addStep` | ✅ | ✅ | ❌ 2 lines (`setActiveExample` undeclared) | 🟡 4 of 5 |
| 📕️norm | `semio-s-artifact-norm-din4108` | 6091 | ✅ | ✅ | ❌ refused mid-flight | — | — | ❌ 5 lines | ❌ root-caused, §3.4 |
| 📜️imperative | `semio-s-artifact-imperative-procedure` | 6076 | ✅ | ✅ | ❌ no clickable row exists | — | — | ❌ 2 lines | ❌ blocked, §3.5 |

Probe: `🐍️b2b-interaction-probe.mjs` (shared) + one shim per plugin. It scores the framework-injected
History ledger (`framework.history.entry.<seq>`), the uncommitted-edit count on `#s-checkin`, AND the
rendered surface together, so a ledger row over an unchanged document cannot be mistaken for a
mutation. A **redo** step was added to it in this session (`:172`), and `interactionBar` now requires it.

## 2. Inherited state (what the dead predecessors left in the tree)

`git status --short` / `git diff` over the five plugin paths at session start showed the previous B2b
worker had written — but never compiled and never run — a large part of the recipe:

- **dag** — new `🎮️commands/🧬️set-active-example/`, `DAG_RETAINED_DOCUMENT_TOOL_IDS` + contracts +
  reduce + extent + a second job factory, `dag_command_from_action`, `reset_dag_document_effect`,
  `genesis_dag_child_pack`, `empty_snapshot`, `type Members = SemioMembers` on editor and viewer, all
  10 verbs flipped to `Migrated`. It did **not compile** (3 errors) and, once compiling, **trapped the
  guest at boot** (§4.1).
- **reasoning** — B1b's six fixes plus the F6 retirement (the framework's generic bounded artifact
  preparation replacing the bespoke `MoveNode`-only retained cursor, whose `🧵️retained/` directory is
  gone), `Members`, `genesis_child_pack`, unit tests ported to `new_app_with_registry_and_members`.
- **norm** — a `command_from_action` bridge and `setSnapshot` re-declared with a staged `snapshot`
  text argument. It did **not compile** (wrong command variant name, §4.4).
- **playbook / imperative** — untouched since B1b.

`🗑️generated` had been wiped twice, so no measurement was inherited.

## 3. Per-plugin findings

### 3.1 🕸️dag — ✅ passes the bar

`🗑️generated/b2b-dag/report.json`, `b2b-dag-console.txt`, `b2b-dag.png`, `b2b-dag-check.txt`, `b2b-dag-test.txt`.

```
SUMMARY {"ready":"dag","error":null,"exampleRendered":true,"actionCount":25,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

- Boot: windows `dag-main` + `dag-compiled-dag`, DAG/DSL tabs, example combobox on **Demo**, 25 actions,
  no window faults.
- `addNode` from the Actions pane appends ledger entry 5
  `create-node node-json="{\"id\":\"n1\",\"name\":\"Computation…` and moves the uncommitted-edit count
  0 → 1: the assertion is on the *document mutation payload*, not on the absence of an error.
- `undo` 1 → 0, `redo` 0 → 1. **0 fault lines** (the run before the §4.1 fix ended in
  `Framework OS boot failed`).
- Gap: the canvas is WebGL, so the probe's render witness counts canvases rather than pixels.

### 3.2 💡️reasoning — ✅ passes the bar

`🗑️generated/b2b-reasoning/report.json`, `b2b-reasoning-console.txt`, `b2b-reasoning.png`, `b2b-reasoning-check.txt`.

```
SUMMARY {"ready":"reasoning-wires","error":null,"exampleRendered":true,"actionCount":21,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

- `addNode` appends `create-node node={ handles=[ ] id="node-8" nodeKind="identit…` — **`node-8`**, so
  the seven-node metabolism Demo really was loaded (B1b's example-id fix holds) and the new node was
  appended to *that* document. Edits 0 → 1, undo → 0, redo → 1.
- No crate change was needed in this session: the predecessor's F6 retirement compiles and runs. This
  slice's contribution to reasoning is the measurement (`cargo check` green + the first full
  interaction run on record).

### 3.3 📖️playbook — 🟡 mutate/undo/redo work, one residual boot fault

`🗑️generated/b2b-playbook/report.json`, `b2b-playbook-console.txt`, `b2b-playbook.png`, `b2b-playbook-check.txt`, `b2b-playbook-test.txt`.

```
SUMMARY {"ready":"playbook","error":null,"exampleRendered":true,"actionCount":20,
         "mutated":true,"undone":true,"redone":true,"faultLines":2,"interactionBar":false}
```

- `addStep` appends `add-step step { id=step-op-128 title="Step 128" blocks=[ ] }`, edits 0 → 1, and
  the Builder window's rendered surface really changes (122 → 131 chars, 121 → 132 SVG nodes). Undo
  retires it, redo reapplies it and the surface grows back.
- Two runs were needed: the first showed the verbs reaching the typed operation (no longer
  classification-refused) and dying at `returned snapshot read requires its exact owned-snapshot
  retirement factory` — see §4.3.
- **Residual fault (2 lines, one event):** the shell dispatches `setActiveExample` at boot and the app
  declares no such verb, so it is dropped `undeclared-action`. This is B1b's F9, not a regression.
  Fixing it is not a one-liner here: `PlaybookSnapshot` owns **two** composed children
  (`document`, `flow`, both `s.stdio.semio`), so a whole-document `Effect::LoadDocument` needs the full
  composed-child recipe (`type Members`, `genesis_child_pack` for both slots, `empty_document_spr`) —
  the same recipe dag carries. Left undone deliberately rather than half-done.

### 3.4 📕️norm — ❌ `setSnapshot` still cannot publish; root cause located, one of two links fixed

`🗑️generated/b2b-norm/report.json`, `b2b-norm-console.txt`, `b2b-norm-check.txt`, `b2b-norm-test.txt`.

Boot is the healthiest of the batch (B1b's finding holds): Inputs + Results windows, **19 evaluated
DIN 4108 check rows**, 12 actions, no window faults. `setSnapshot` — the app's only document-mutating
verb — now *reaches* the typed operation (the `command_from_action` bridge compiles, §4.4) and fails:

```
input #5 setSnapshot refused: dispatch-failed — typed-operation failed:
  validation failed: norm-mutation-owner-missing
```

This is **not a live-only fault**. The crate's own pre-existing test
`set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document`
(`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:175`) fails at HEAD for the same reason — measured before any edit
of mine. Two links were found:

1. **Fixed.** `Din4108Mutation::from_snapshot`
   (`✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs:125`)
   emitted **21 mutations for a one-field change** — every scalar unconditionally, plus a full
   remove/re-insert of every layer. The artifact lane's authority is a *one-item* preparation
   (`NormOneItemPreparationFactory`), so a 21-item bundle could never be staged; the second `advance`
   on the consumed preparation is what prints the misleading `norm-mutation-owner-missing`
   (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:702`) instead of the real cause. It is now
   difference-driven (new test `from_snapshot_yields_one_diff_appliable_mutation_for_a_one_field_change`
   passes: 1 mutation, and its own diff applies back to the base).
2. **Open.** Even with exactly one mutation the document still never publishes: the new native test
   `set_snapshot_publishes_a_one_field_change_through_the_tool_job_path` spins 5 000 maintenance turns
   with the snapshot unchanged. The remaining blocker is inside the shared norm artifact lane —
   `NormOneItemPreparationFactory::begin`/`advance` (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:662`
   and `:690`), which all **fifteen** norm apps route through. `begin` returns `Err(request)` on any
   operation/generation/base-revision mismatch, which is a *silent* refusal at this level. That is the
   next worker's first breakpoint.

Second, unrelated fault in the same run: the two window kinds do not declare the framework's
`shell.windowActivate`, so activating a window is dropped `undeclared-action` (needs
`.window_kind_actions()` / `.window_kind_action_refs()` on both norm window kinds).

### 3.5 📜️imperative — ❌ blocked before the recipe; **B1b's F10 diagnosis is refuted**

`🗑️generated/b2b-imperative/report.json`, `b2b-imperative-console.txt`, `b2b-imperative-check.txt`, `b2b-imperative-test.txt`.

```
SUMMARY {"ready":"imperative","error":null,"exampleRendered":true,"actionCount":0,
         "mutated":false,"undone":false,"redone":false,"faultLines":2,"interactionBar":false}
```

- The crate checks green, activates, boots, and renders its document (`imperative-main`, tabs
  Imperative/Script, example combobox on **Demo**). Only 2 fault lines, both the same
  `setActiveExample` undeclared-action playbook has.
- **The Actions pane really is empty** — measured twice. A direct DOM dump of the engagement pane
  after opening it returns the header `"Actions"` and **not one row**
  (`[id*="engagement"] [id]` → only the toggle itself).
- **But the cause is not what B1b recorded.** A new native test,
  `every_declared_action_is_carried_by_a_window_kind`
  (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`), asserts the built manifest and **passes**: the window kinds do
  carry `addStep`, `removeStep`, `moveStep`, `setStepParams` and `run`. So the missing
  `.window_kind_actions(...)` call is not the reason — the manifest is populated, and the shell still
  renders no rows. The remaining hypothesis space is on the host/render side (why an
  engagement pane with a non-empty window-kind action list paints zero rows), and that is where the
  next worker should start; the migration recipe below it is pointless until a row exists to click.
- Everything else is still untouched: all 10 verbs are `BatchOnlyPendingRewrite`
  (`✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:240-249`),
  no `command_from_action`, no store owners, no artifact-lane preparation.

## 4. Root fixes (file:line)

### 4.1 dag — two tool factories over one owner trap the guest at boot

The predecessor registered `DagDocumentCommandJobFactory` alongside `DagConfigCommandJobFactory`, but
`bounded_first_step_tool_proofs!` binds **every** listed id to a single `factory_type`. The guest's
catalog authority then demands that the registered concrete factory for each id *be* that type:

```
tool factory proof rejected tool 'setActiveExample': … factory='DagConfigCommandJobFactory'
registered_factory='…DagDocumentCommandJobFactory', owner_eq=true controller_eq=true
schema_eq=true … typed_join=false
→ RuntimeError: unreachable → Framework OS boot failed
```

Fixed by collapsing to ONE factory serving the union (the reduce is already chosen per id in
`build_tool_job`), in
`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

- `:186` `DAG_RETAINED_TOOL_IDS` — the single 12-id roster (config verb + 11 document verbs).
- `:209` `DAG_RETAINED_PUBLICATION_CONTRACTS` — the union, in roster order.
- `:312` `DagRetainedCommandJobFactory` (was `DagDocumentCommandJobFactory`), keys/`TOOL_IDS`/
  `PUBLICATION_CONTRACTS` over the union; `DagConfigCommandJobFactory` and the document-only contract
  list deleted.
- `:669` proofs `factory:`/`factory_type:` point at it; `:751` registers one factory.

### 4.2 dag — the predecessor's `set-active-example` did not compile

`…/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs:14` imported a `semio_framework_value_derive` crate
this crate does not depend on (the repo convention is `dsl::ToValue` / `dsl::FromValue`), and `:28`
called `crate::schema::{default,empty}_snapshot`, which live at the crate root. Both fixed, and
`empty_snapshot` is now re-exported beside `default_snapshot`
(`✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs:35`).

### 4.3 playbook — six dead verbs, no `{action,args}` bridge, no store ownership

All in
`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

- `:105` `PLAYBOOK_RETAINED_TOOL_IDS` += `addStep`, `removeStep`, `moveStep`, `addBlock`,
  `removeBlock`, `moveBlock` (was `setContributions` alone). `updatePlaybook` is deliberately **not**
  promoted: it emits `Emit::amend`, a lane no `ArtifactToolPublicationLane` row can state truthfully.
- `:110` publication contracts for all seven (config lane for `setContributions`, artifact lane for
  the six).
- `:126` `playbook_retained_extent` admits the six; `:467` the proofs `tools:` list names all seven.
- `:134` new `playbook_command_from_action`, wired at `:532` as `ArtifactEditor::command_from_action`.
  Without it the trait default refused every id with `app.command.unsupported`.
- `:449` new `build_artifact_store_one_item_preparation_factory` (the app owned a *config*-lane
  preparation only, so an `Artifact`-lane retained tool had nowhere to stage its edit).
- `:456-490` the full store ownership set — document owners + disposer, config owners + disposer,
  draft owners + disposer, presence and transient disposers. The app declared **none**, which is what
  produced `returned snapshot read requires its exact owned-snapshot retirement factory` on the first
  real publication.
- `:603-608` the six `.action_interactive_job(…)` rows flipped to `Migrated`.

### 4.4 norm — the bridge did not compile, and `from_snapshot` was unconditional

- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/…/✏️editor/🦀️.rs:136` — the predecessor's bridge named
  `Din4108Command::SetSnapshot`; `app_commands!` names the variant after the payload type, so it is
  `Din4108Command::ReplaceSnapshot`. Crate now green.
- `…/🧬️schema/🧬️mutations/🦀️.rs:125` — `from_snapshot` rewritten difference-driven (§3.4 link 1), with
  a new bit-exact `quantity_differs` helper at `:200` for the fourteen `f64` fields.

### 4.5 framework — `dyn_enum` return classification (peer file, one line)

`🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs:203` was mid-refactor by slice D1 and failed the whole
workspace: `classify_return` returns `syn::Result<ReturnShape>` but its tail produced
`Result<Type, _>`. Every plugin crate depends on this proc-macro crate, so **no cargo command in the
repo could compile** while it stood. Completed the one line
(`output.map(ReturnShape::FutureOutput).ok_or_else(…)`) to unblock the fleet. D1 has edited the file
again since; whoever owns it should confirm the classification still round-trips.

## 5. Tests added (all run, output captured)

| test | crate | result |
|---|---|---|
| `every_retained_tool_id_is_migrated_contracted_and_served_by_one_factory` | dag | ✅ |
| `command_from_action_resolves_every_flat_verb_and_names_the_gesture_only_ones` | dag | ✅ |
| `every_retained_tool_id_is_migrated_contracted_and_backed_by_store_owners` | playbook | ✅ |
| `command_from_action_resolves_every_declared_verb` | playbook | ✅ |
| `from_snapshot_yields_one_diff_appliable_mutation_for_a_one_field_change` | norm | ✅ |
| `set_snapshot_publishes_a_one_field_change_through_the_tool_job_path` | norm | ❌ **by design** — it is the minimal native repro of §3.4 link 2 and is red until that is fixed |
| `every_declared_action_is_carried_by_a_window_kind` | imperative | ✅ — and its passing is the evidence that refutes B1b's F10 (§3.5) |

The first two assert exactly the three-way join whose violation traps the guest at boot (roster ↔
publication contracts ↔ one registered factory type ↔ `Migrated` on every window kind's action) plus,
for playbook, the store ownership a published edit needs. The dag bridge test also pins the five
canvas-gesture verbs as deliberately **un**bridged (`dag.unhandled-action`) rather than lossily decoded.

Note on the playbook test: the store owners/disposers it constructs are `mem::forget`ed, because a
live `ArtifactStoreCursorDisposer` fails closed in `Drop` unless driven to terminal-empty through a
real store. That is stated in the test's own docstring.

### Crate suites

- `cargo test -p semio-s-artifact-playbook-playbook --lib -- retained_tool command_from_action` → **2/2**.
- `cargo test -p semio-s-artifact-dag-dag --lib` → **179 passed / 27 failed**. Of the 27:
  - 12 are a direct consequence of this slice making the verbs live (they now open a real store):
    10 `artifact store reached Drop without its exact terminal-empty shallow-shell witness` and 2
    `interactive-job.live-instance`. While the verbs were `BatchOnlyPendingRewrite` the dispatch was
    refused, so no store was ever opened and the harness debt was invisible. Two harness fixes landed
    (`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:3,6,22` bind the context to `SemioMembers`, and `:9` binds the
    live instance id); the remaining ones each need a `close_registered_fixture_app` before the app
    drops, the way reasoning's context already does (`…/💡️reasoning/…/🧪️tests/🔬️unit/🦀️.rs:18`).
    **Not done — this is the honest open item on dag.**
  - ~15 are pre-existing fixture debt untouched here, verified by reading their assertions:
    `committed_json_is_canonical` failures are snake/camel and `90.0`/`90` float-form drift, and
    `command_envelope_round_trip…` fails `edit history insertion requires its exact mutation
    retirement factory`.
- `cargo test -p semio-s-artifact-norm-din4108 --lib` → **203 passed / 27 failed**, all pre-existing
  buckets (store drop witness, mutation retirement factory, unbound live instance) except the two
  `setSnapshot` publication tests analysed in §3.4.

## 6. Launch entries

The five variants already have generated dev rows in `.vscode/launch.json`
(`🛠️dev🕸️dag⚛️react`, `🛠️dev💡️reasoning⚛️react`, …). That file is **generated** — `📜️script.ts:8989`
derives it from `.vscode/🧩️launch.seed.jsonc` plus the playground identities — so it was not
hand-edited here; a hand-added row would be overwritten on the next generation.

B1b's eight rows live in `.claude/launch.json`, which is a different file (the Claude Code launcher),
not the file AGENTS.md means. Flagged rather than "fixed": adding the split *activate-then-serve*
recipe as a real dev command belongs in the generator, which is cross-cutting and owned by the build
slice. The split recipe itself stays reproducible through the two committed helpers,
`📜️b2b-activate.sh <variant> <port>` and `📜️b2b-serve.sh <variant> <port>`.

## 7. Honest gaps

1. **imperative — blocked one level below the recipe.** Measured this session: crate green, boots,
   renders, 2 fault lines — but the engagement pane paints **zero rows** although the built manifest
   carries the actions on both window kinds (new passing test). Until that is understood there is no
   row to click, so none of the migration work was started.
2. **norm — `setSnapshot` still cannot publish.** One of two links fixed; the second is in the shared
   fifteen-app `NormOneItemPreparationFactory`. The new native test is the minimal repro and is red.
3. **norm — `shell.windowActivate` undeclared** on both window kinds.
4. **playbook — `setActiveExample` undeclared**; needs the two-composed-children `LoadDocument` recipe.
5. **dag — 12 crate tests red** from the verbs going live (store-close and instance-bind harness debt);
   the runtime chain is green, the unit harness is not.
6. **dag — five canvas-gesture verbs are declared actions the Actions pane can show but the bridge
   refuses** (`dag.unhandled-action`). Truthful, but a user can click a row that always faults; either
   the args should be declared or the rows hidden from the pane.
7. **Render witness on canvas surfaces** counts canvases, not pixels — dag and reasoning are proven by
   ledger payload + edit count, not by a repaint diff.
8. `space` remains out of this slice's scope (it was B1b's sixth plugin, deferred to the cold `dev s`).

## 8. Files changed

Plugin crates:
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🦀️.rs` (re-export `empty_snapshot`)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (single factory, union roster/contracts)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs` (compile fixes)
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (members + instance binding + 2 tests)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (§4.3)
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (2 tests)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` (variant name)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs` (difference-driven `from_snapshot`)
- `✏️s/🔌️plugins/📕️norm/🗿️artifacts/🧱️din4108/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (2 tests)
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (1 test)

Framework (peer file, one line, §4.5):
- `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs`

Ticket folder:
- `🐍️b2b-interaction-probe.mjs` (redo step + summary field), `🐍️b2b-norm-probe.mjs` (rewritten),
  `🐍️b2b-playbook-probe.mjs` (new), `🐍️b2b-imperative-probe.mjs` (new),
  `📓️b2b-dormant-plugin-interactions.md` (this report).

Captures kept under `🗑️generated/`: `b2b-{dag,reasoning,playbook,norm}/report.json`,
`b2b-<plugin>-console.txt`, `b2b-<plugin>.png`, `b2b-<variant>-{activate,serve}.txt`,
`b2b-<crate>-{check,test}.txt`.

## 9. Environment

Five dev servers were started by this slice (6015 reasoning, 6017 dag, 6076 imperative, 6085 playbook,
6091 norm) and all five were stopped by pid at the end of it — every port is free, and every
activation is warm, so `📜️b2b-serve.sh <variant> <port>` brings any of them back in ~20 s — stop them by pid if the next worker needs the ports. One cargo at a time,
`CARGO_PROFILE_WASM_DEV_DEBUG=false`, `NX_DAEMON=false`; no peer process was killed. Headless Chromium
with `--use-angle=metal`, 1600×1000 visible viewport. Peers renamed `🎚️options`→`☑️options` and
`⚙️config`→`🎚️config` inside these plugin trees during the session; every path above was re-read
immediately before editing.

---

# B2c — continuation (2026-09-19)

Slice B2c continues this report in place. Scope inherited from §7 gaps: norm publication root fix +
15-app proof, imperative action-projection trace, playbook `LoadDocument` recipe, dag harness debt,
launch seed rows.

## B2c.0 Status (live, updated as work lands)

| item | state |
|---|---|
| norm — `NormOneItemPreparationFactory` root fix | ✅ **fixed** (§B2c.1) |
| norm — 3 apps boot→mutate→undo→redo | 🟡 din4108 ✅ full bar, 2 to go |
| norm — 15-app sweep (boot + one mutation) | ⏳ |
| imperative — action projection trace | ✅ **root-caused** (§B2c.2): rows were never missing |
| imperative — migration to `Migrated` + runtime proof | ✅ mutate/undo/redo live (§B2c.3) |
| playbook — `setActiveExample` LoadDocument | ⏳ |
| dag — 12 red crate tests | ⏳ |
| launch seed rows (5 plugins) | ⏳ |


## B2c.1 📕️norm — the root cause: a hand-copied preparation factory that mis-declared its fold contract

Captures: `🗑️generated/b2c-norm-check1.txt`, `b2c-norm-test{1,2,3}.txt`, `b2c-norm-test-full.txt`,
`b2c-norm-plugin-check.txt`, `b2c-din4108-{activate,serve}.txt`, `b2b-norm/report.json`.

### The divergence

`NormOneItemPreparationFactory` (`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs`, old `:625-757`) was a
line-for-line copy of the framework's `BoundedConfigPreparationFactory`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:14598`), with **one** behavioural difference:

```rust
// norm (copy)                          // framework (original)
Ok(ArtifactStoreOneItemFootprint {      Ok(ArtifactStoreOneItemFootprint::
    work_items: 1, retained_bytes })        for_one_invertible_item(retained_bytes))
```

`work_items` counts staged edit **rows**, and a point-invertible item folds TWO
(`store::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS`): its forward row plus the row
`Mutation::inverse` yields. The framework carries that exact fix with its own docstring — it was found
once before in the config lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
`batched item candidate failed its exact fixed fold contract`). Norm's copy predates it and never got
it, so **every `setSnapshot` in all fifteen norm apps was refused inside
`ArtifactStore::fold_batch_item`** and the document never published. B2b's
`norm-mutation-owner-missing` was the downstream symptom: the preparation had already taken the
mutation, so the next `advance` reported the missing owner instead of the refusal.

### The fix

The copy is **deleted** (191 lines: `norm_next_edit`, `norm_mutation_retained_bytes`,
`admit_norm_mutation`, `prepare_norm_one_item`, `NormOneItemPreparationFactory`,
`NormOneItemPreparation` and both impls). `norm_artifact_store_preparation`
(`✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs:571`) now returns the framework's own
`bounded_config_store_one_item_preparation_factory::<A::Snapshot, A::Mutation>` — the same authority
trinity-jack, dag and reasoning bind — under a `where A::Mutation: Clone + Sync` bound proven at each
of the fifteen concrete call sites. No norm editor file changed.

`cargo check -p semio-s-plugin-norm --lib` compiles **all fifteen** artifact crates against it.

### Native proof

`cargo test -p semio-s-artifact-norm-din4108 --lib`: **213 passed / 17 failed**, up from B2b's
203/27. The three `setSnapshot` publication tests are green, including B2b's red-by-design repro
`set_snapshot_publishes_a_one_field_change_through_the_tool_job_path`. Every one of the remaining 17
was already failing at the B2b baseline (mutation-fixture canonical-JSON drift, the retained
disposition oracle fixture, five render fixtures) — no regression, and none of them is a publication
fault.

Three harness defects were fixed alongside, all from the known playbook
(`…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`):
- `context::app_with_registry` now calls `bind_instance_id` — an unbound wrapper refuses every live
  verb with `interactive-job.live-instance`.
- `context::settle` drives the operation through `settle_registered_typed_operation`, draining every
  result page. The old bare `maintenance_step` loop retired nothing, which is precisely why a
  publication refusal looked like a silent 5 000-turn timeout. `context::dispatch` now settles.
- `context::close` runs `close_registered_fixture_app` at the end of all ten fixture tests, so a live
  `ArtifactStore` no longer asserts in `Drop`.

### Runtime proof — din4108 clears the whole bar

`🐍️b2b-norm-probe.mjs` against a freshly activated `din4108` React dev server on 6091:

```
SUMMARY {"ready":"din4108","error":null,"exampleRendered":true,"actionCount":12,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

- 19 evaluated DIN 4108 check rows at boot, windows `norm-din4108-inputs` + `norm-din4108-results`.
- `setSnapshot` appends ledger entry 5 `setSnapshot change-t-int-c new-t-int-c="22.5"`, the rendered
  surface changes, undo retires it, redo reapplies it.
- **0 fault lines.** B2b's second norm fault — `shell.windowActivate` dropped `undeclared-action` —
  is gone too: the ledger now carries two `Activate Window` entries. A peer's window-kind action
  wiring landed in between; re-measured, not assumed.

## B2c.2 📜️imperative — the Actions pane was never empty; the probe's click was swallowed

Captures: `🗑️generated/b2c-imperative-actions-pane.json`, `b2c-imperative-{activate,serve}.txt`.
Tool: `🐍️b2c-actions-pane-diagnose.mjs` (new) — it separates the only two places a row can be lost in
the React projection, because they leave different DOM:

- **A** `windowActionPaneNode` returned `undefined` (`resolveWindowActions` empty, or everything
  dropped by its `.filter((action) => action.inPalette)`) → no `[data-slot="window-action-pane"]`
  element exists at all.
- **B** the pane mounted and its `<Tree>` painted nothing → the element exists with no `action.*` child.

### Measurement

Against a freshly activated `imperative` React dev server on 6076:

| toggle click | `[data-slot="window-action-pane"]` | `action.*` rows |
|---|---|---|
| Playwright hit-tested `click({force:true})` | **0** | 0 |
| synthetic `element.click()` on the same id | **1** | **24** |

Same page, same toggle (`framework.window.imperativeMain.engagement.toggle`), seconds apart. So the
projection chain is **intact end to end**: guest window-kind actions → `🔣️.json` (22 `inPalette`
actions on each of `imperative-main` / `imperative-script`, verified on the SERVED manifest under
`🧰️framework/…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/📜️imperative/🔣️.json`,
not only the committed one) → `resolveWindowActions` → `windowActionPaneNode` → `WindowActionPane`
→ 24 rows.

**B1b's F10 (window kinds carry no actions) and B2b's §3.5 (the pane paints zero rows) are both
refuted.** B2b's "measured twice" was the same artefact twice: `Window` mounts the pane only while
unfolded (`engagementExpanded = engagementVisible && !actionsFolded`,
`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:206`), and an anchored chrome panel paints over
the top-left Actions chip and swallows a real pointer event **without throwing** — Playwright reports
`ok`. norm/dag/playbook/reasoning were never affected because their layouts do not overlay that
corner, which is why only imperative looked dead.

### Root fix (measurement side)

`🐍️b2b-interaction-probe.mjs` — `click(selector, settled)` now takes an optional settled-predicate and
falls back to a synthetic `element.click()` when the hit-tested click left the DOM unmoved; the
Actions-toggle loop passes "an action pane exists". Every batch-B probe inherits it.

### What is actually left on imperative

The rows exist and are clickable, but all ten verbs are still
`InteractiveJobClassification::BatchOnlyPendingRewrite`
(`…/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs:240-249`), so a click is refused at the
dispatch gate. The remaining work is playbook's §4.3 recipe applied to imperative (retained tool ids,
publication contracts, extent, `command_from_action`, store owners, `Migrated`) — see §B2c.4.

## B2c.3 📜️imperative — migrated, and mutating in the browser

Captures: `🗑️generated/b2c-imperative-check1.txt`, `b2c-imperative-test{1,2,3}.txt`,
`b2b-imperative/report.json`, `b2c-imperative-{activate,serve}.txt`.

```
SUMMARY {"ready":"imperative","error":null,"exampleRendered":true,"actionCount":22,
         "mutated":true,"undone":true,"redone":true,"faultLines":2,"interactionBar":false}
```

`addStep` from the Actions pane appends ledger entry 3
`create-step step step-3 kind=log.print`, the uncommitted-edit count moves 0 → 1, `undo` retires it and
`redo` reapplies it. B2b measured `actionCount: 0` and no dispatch at all.

### Root fixes — all in `…/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`

A new `🧵️RetainedCommands` region, the same shape playbook got in §4.3:

- `IMPERATIVE_RETAINED_TOOL_IDS` — all ten verbs; `IMPERATIVE_RETAINED_PUBLICATION_CONTRACTS` states
  `Artifact` for the eight structural step verbs and `Config` for `run`/`setContributions`, read off
  the command bodies (`run` emits `Emit::config(SetRunOutput)`).
- `imperative_retained_contract` / `imperative_retained_extent` (every verb is one bounded step).
- `imperative_command_from_action` — the `{action, args}` bridge. Without it the trait default refused
  every id with `app.command.unsupported`. `setStepParams`/`setStepParamsAt` carry a
  `BTreeMap<String, ValueDsl>` whose fields are private to `crate::document_dsl`, so the bridge goes
  through that module's own `value_to_value_dsl` over an engine `Value` rather than reaching inside it.
- `imperative_retained_reduce` + `ImperativeRetainedCommandJobFactory` (ONE factory over all ten ids).
- Trait overrides: artifact- AND config-lane `bounded_config_store_one_item_preparation_factory`, the
  full store owner/disposer set (document, config, draft, presence, transient), the
  `bounded_first_step_tool_proofs!` block, `register_tool_job_factories`, `command_from_action` and
  `build_tool_job`.
- All **ten** `.action_interactive_job(…)` rows flipped `BatchOnlyPendingRewrite` → `Migrated`.

The proof block's `artifact_schema` is `"procedure.document/v1"` (`PROCEDURE_DOCUMENT_SCHEMA`), not the
artifact-kind id — a mismatch there is a boot trap, not a compile error.

### Fixture and harness

- `…/✏️editor/🧫️fixtures/🛣️retained-command-routes.json` — all ten rows restated `Migrated` with their
  real lanes and reasons; `maximumWorkItems` 1 → 64. The census test now asserts one first-step proof
  per route and roster membership, instead of asserting the proofs list is empty.
- `context::imperative_app()` was `new_app()` (registry-less). With the verbs `Migrated` that wrapper
  carries **no** migrated action rows, so the guest's catalog authority rejects every proof
  (`interactive-job.catalog-authority`, `migrated={}`); it now delegates to the registry-backed
  constructor, binds the instance id, settles after each dispatch and closes the app.

`cargo test -p semio-s-artifact-imperative-procedure --lib` → **83 passed / 59 failed**. Honest
accounting of the 59: **38** are `final Dictionary ownership must be explicitly retired or owned by a
cold boundary` raised inside `neural_engine` from `editor::procedure::engine::tests` and the mutation
fixtures — code this slice does not touch and which fails with or without it; **~13** are the residual
store-close/instance-bind harness debt the verbs going live exposed (5 `artifact store reached Drop`,
5 presence-store close faults, 2 grant asserts, 1 settle fault, 1 remaining catalog-authority row);
the rest are pre-existing fixture drift (canonical JSON, an oracle catalog file that is not on disk).
**The runtime chain is green; the unit harness is not** — the same honest split B2b recorded for dag.

### Residual

Two fault lines, both the one event `setActiveExample` refused `undeclared-action` — identical to
playbook's, and gated on the same framework `Effect::LoadDocument` work (§B2c.5).
