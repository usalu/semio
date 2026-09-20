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
| norm — din4108 boot→mutate→undo→redo | ✅ full bar, 0 faults (§B2c.1) |
| imperative — action projection trace | ✅ **root-caused** (§B2c.2): rows were never missing |
| imperative — migration to `Migrated` + runtime proof | ✅ mutate/undo/redo live (§B2c.3) |
| playbook — `setActiveExample` LoadDocument recipe | ✅ **done, full bar** (§S5.4) |
| imperative — `setActiveExample` LoadDocument recipe | ✅ **done, full bar** (§S5.3) |
| dag — 12 red crate tests | 🟡 **a peer's live edit** (§B2c.6): every mount site now closes; not re-done here |
| `imperative-extension-{text,math,logic,effect,control}` build+install | ✅ **26/26 installed current** (§B2c.7); C1b §12.3's premise refuted |

> **Launch rows** (B2b §6's open item): re-measured, nothing to do. `.vscode/launch.json` carries
> **249** generated `🛠️dev…` rows, including exactly one `⚛️react` row for each of `🕸️dag`,
> `💡️reasoning`, `📖️playbook`, `📜️imperative` and every one of the fifteen `📕️norm` standards. The
> file is generated from `.vscode/🧩️launch.seed.jsonc` by `📜️script.ts`, so a hand-added row would be
> overwritten; the split activate-then-serve recipe stays in `📜️b2c-activate.sh`/`📜️b2c-serve.sh`.
>
> **Session 4 environment.** Load average **150–165** with ~15 peer cargo processes on a 10-core
> machine for the whole session; every cargo of this slice sat in
> `cargo::core::compiler::prebuild_lock_exclusive` → `flock` for 10–30 minutes before its first
> `rustc` (verified with `sample <pid>`, not guessed — 10 live `rustc` processes at the same moment,
> so the build dir was working, not deadlocked). No peer process was killed; the only process this
> slice ever killed was its own queued `cargo check`, replaced by a single two-`-p` `cargo test` so
> that both crates compile in ONE lock acquisition instead of two. Disk 84 GiB free.
>
> **Session 4 (2026-09-19 23:20→)** resumes here. Session 3's edits were verified present in the tree
> before any new work: `norm_artifact_store_preparation` returns the framework factory
> (`📕️norm/🖥️app-surface/🦀️.rs:577-581`, the 191-line copy gone) and imperative carries
> `IMPERATIVE_RETAINED_TOOL_IDS` / `imperative_command_from_action` /
> `ImperativeRetainedCommandJobFactory` with all ten `.action_interactive_job(…)` rows on `Migrated`
> (`📜️procedure/…/✏️editor/🦀️.rs:113,170,254,548-557`). Nothing was redone.


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
playbook's, and gated on the same framework `Effect::LoadDocument` work. **Closed in §B2c.4.**

## B2c.4 📖️playbook + 📜️imperative — the composed-children `setActiveExample` recipe

Both plugins carried the identical last fault: the shell dispatches `setActiveExample` at boot, the
app declares no such verb, the event is dropped `undeclared-action`, and the probe's `faultLines`
stays at 2 so `interactionBar` is false even though mutate/undo/redo all work. Neither is a one-liner
because **both snapshots own two composed `s.stdio.semio` children** — playbook `document`+`flow`,
imperative `flow`+`text` — so a whole-document `Effect::LoadDocument` needs the full member recipe,
not just a command body.

### Peer dependency, honoured not redone

F1 owns the framework side (`📓️f1-load-document-archive-replacement.md` §4–5): an app mints its
replacement log with `store::empty_document_spr(...)`, whose `HistoryLog` has `composition: None`,
and parent hydration refuses it outright before any replacement leg runs. F1's fix stamps the log in
the framework (`stamp_load_document_effects` on the plain dispatch lane,
`stamp_polled_load_document_effects` on the refresh-poll lane, beside the mounted typed-operation
ladder that already stamped). Verified present in the tree before building on it
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:19780+` typed refusal record,
`stamp_load_document_effects`). **No framework file was touched by this slice** — `setActiveExample`
is declared as a retained tool, so it rides the mounted ladder F1 says was always stamped, and the
two new stamping lanes are F1's to prove live.

### The recipe, applied twice (dag's `🕸️dag/…/✏️editor/🦀️.rs` is the reference that already passes)

Seven parts per plugin, all of which must agree or the guest traps at boot rather than failing softly:

1. a new `🎮️commands/🧬️set-active-example/🦀️.rs` payload + handler, emitting `Effect::LoadDocument`
   built from `store::empty_document_spr` (never a minted `create_document_envelope`);
2. `reset_<app>_document_effect` beside the app struct;
3. the verb appended to the retained roster, to the publication contracts as
   `ArtifactToolPublicationLane::HostOnly` (it publishes through neither the artifact nor the config
   store), to the extent function, and to the `bounded_first_step_tool_proofs!` `tools:` list;
4. the `{action,args}` bridge row (`exampleId`/`example_id`/`id`/`value`, defaulting to the demo id);
5. `type Members = semio_s_artifact_stdio_semio::SemioMembers` on **both** the editor and the viewer;
6. `genesis_child_pack` answering **both** slots, plus `build_document_store_initialization_job`
   (the trait default refuses the envelope → `artifact-store.persisted-initializer-refused`);
7. the manifest `.action_with(ActionDefinition::new("setActiveExample", …))` +
   `.action_interactive_job("setActiveExample", Migrated)`.

Parts 5 and 6 cascade: `type Members` changes the app's runtime adapter type, so the plugin's
`dyn_enum_close!` fleet rows and (playbook only) the `PlaybookApplication` trait bound both have to
name `SemioMembers` as the second parameter of `VcsArtifactApp<…>` or the subset declaration no
longer satisfies `From<VcsArtifactApp<EditorApp<App>, SemioMembers>>`.

### Files changed (both plugins)

| file | change |
|---|---|
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs` | **new** — payload + handler |
| `…/📖️playbook/🦀️.rs` | module mount; `genesis_playbook_child_pack` (both slots); `PlaybookApplication` bounds carry `SemioMembers` |
| `…/📖️playbook/…/✳️any/✏️editor/🦀️.rs` | `reset_playbook_document_effect`; roster/contract/extent/bridge/proofs rows; `type Members`; `genesis_child_pack`; `build_document_store_initialization_job`; manifest action + `Migrated` |
| `…/📖️playbook/…/✳️any/👁️viewer/🦀️.rs` | `type Members` |
| `…/📖️playbook/…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | context bound to `SemioMembers` + `new_app_with_registry_and_members`; new row in `every_command()`; wire-keyword arm; **new test** |
| `✏️s/🔌️plugins/📖️playbook/🦀️.rs` | `dyn_enum_close!` fleet rows carry `SemioMembers` |
| `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧬️set-active-example/🦀️.rs` | **new** — payload + handler |
| `…/📜️procedure/🦀️.rs` | module mount; `genesis_procedure_child_pack` (both slots) |
| `…/📜️procedure/…/✳️any/✏️editor/🦀️.rs` | the same seven parts as playbook |
| `…/📜️procedure/…/✳️any/👁️viewer/🦀️.rs` | `type Members` |
| `…/📜️procedure/…/✳️any/✏️editor/🧫️fixtures/🛣️retained-command-routes.json` | eleventh route (`setActiveExample`, `host-only`) |
| `…/📜️procedure/…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | context bound to `SemioMembers`; `every_command()` row; census count 10 → 11; wire-keyword arm; **new test** |
| `✏️s/🔌️plugins/📜️imperative/🦀️.rs` | `dyn_enum_close!` fleet rows carry `SemioMembers` |

### Tests added

`set_active_example_is_host_only_and_both_composed_children_mint_genesis_packs`, one per crate. Each
one asserts the four things that fail at four *different* runtime boundaries and none of which is a
compile error: the verb is in the retained roster (else the dispatch gate refuses it), its publication
contract is exactly `HostOnly` (else it claims a store lane it never writes), the built manifest
declares it `Migrated` (else the shell drops it `undeclared-action`), and **both** composed child
slots mint a non-empty genesis pack off the real demo asset while a foreign child id is refused (else
the archive's closure leg refuses the whole replacement). The demo asset is parsed in the test rather
than a synthetic snapshot, so a broken example file fails here instead of in a browser.

(measuring — crate checks, then activate/serve/probe)

## B2c.6 🕸️dag — the 12 red crate tests are a live peer's edit, not this slice's

Measured before touching anything: `git diff --stat` on `✏️s/🔌️plugins/🕸️dag` shows **nine
uncommitted files**, eight of them the very `🧪️tests/🔬️unit/🦀️.rs` harnesses B2b §5 named, and the
ninth the editor root gaining a `📚️examples/🎬️demo-session` taxonomy mount. Their mtimes are
2026-09-19 23:52–23:54, i.e. *after* this session started at 23:20, and the last commit touching dag
is `03b1a41483` at 23:41. A static sweep of every `#[test]`/`async_test` body under `🕸️dag` that
mounts an app now finds **zero** without a `close`/`close_registered_fixture_app` call — B2b's exact
prescription, already applied by that peer.

This slice therefore did **not** edit dag: re-doing a peer's in-flight harness repair is how two
workers lose each other's work. The remaining action is a measurement (run the suite and record the
delta from B2b's 179/27), which is queued behind the same build-dir lock as everything else this
session; if it does not land, the honest statement is that the fix is in the tree and unverified by
me, with the owner being whoever is editing dag right now.

## B2c.7 🧩️ The five `imperative-extension-*` installs — C1b §12.3's premise is stale; the real fault is a *filtered* sync

Captures: `🗑️generated/b2c-extension-census.txt` (before), `b2c-extension-sync.txt`,
`b2c-extension-census-after.txt`. New census tool `🐍️b2c-extension-install-census.ts`, repair
`🐍️b2c-extension-install-sync.ts` (both in the ticket folder).

C1b §12.3 recorded the five as having "**no built output in this tree at all**". Re-measured per
extension rather than assumed: **all 26 extension crates have built output and all 26 are installed.**
Every one of the five carries `🔌️plugin-modules/<dir>/🌉️bridge.js` and a component `.core.wasm` built
on 2026-09-19. What was actually wrong is one field narrower, and the census makes it visible in one
line per row:

| | built `.core.wasm` | installed `.core.wasm` | install meta |
|---|---|---|---|
| the other 21 | 2026-09-19 | 2026-09-17…19 | `1789777075xxx` |
| the five `imperative-extension-*` | 2026-09-19 | **2026-08-17** | `1788828112xxx` |

**Root cause.** `syncBuiltExtensionsToInstallRoot`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📥️installation/🟦️.ts:99`) walks only the
registry rows its CALLER hands it, and the build pipeline hands it the *session's* targets
(`🏗️build/🏃️execution/🟦️.ts:149`). A filtered session — the collab prebuild resolves `space` + `writer`
and their transitive closure — never names an imperative extension, so its install-root copy stayed
pinned to whichever unfiltered run first published it while the build root moved on by a month. That
is why "a targeted build of one of those five" looked like the only cure: a targeted build puts that
row in `targets`, which is the only thing the sync ever reads.

**Repair, with no crate rebuilt.** `🐍️b2c-extension-install-sync.ts` passes the **unfiltered**
registry to the same production function. 26/26 republished; the after-census shows
`installedWasmAt == builtWasmAt` for every row, including all five:

```
SUMMARY {"extensions":26,"missingBuiltOutput":[],"missingInstall":[],"wasmOlderThanSource":[]}
```

Honest scope: this proves the five are built, current, and installed from the current build output.
It does **not** prove the Aug-17 wasm they were serving was functionally wrong, and it does not
rebuild the crates from source — `wasmOlderThanSource` is false for all 26, so nothing in the tree
asks for a rebuild. The durable defect (a filtered sync silently leaving other rows behind) is stated
here rather than patched, because `📥️installation/🟦️.ts` is the build slice's file, not this slice's.

# Session 5 (2026-09-20) — B2c resume

Inherited state verified in the tree before any work (not assumed):

- `norm` — §B2c.1's fix is committed and present: `norm_artifact_store_preparation` returns the
  framework's `bounded_config_store_one_item_preparation_factory`; din4108 already measured at the
  full bar with 0 faults. Nothing redone.
- `imperative` — §B2c.2/§B2c.3 are committed: ten verbs `Migrated`, bridge, factory, Actions pane
  proven to carry 24 rows.
- `playbook` + `imperative` — §B2c.4's `setActiveExample` recipe is **written but was never
  compiled**: session 4 died at 00:09 with `b2c-playbook-check{2}.txt`, `b2c-playbook-plugin-check.txt`
  and `b2c-imperative-check4.txt` all **0 bytes** (killed mid-flight), and the last completed capture
  `b2c-playbook-check1.txt` ends in a hard error
  (`A: From<VcsArtifactApp<EditorApp<PlaybookPlayApp>, SemioMembers>>` unsatisfied at
  `📖️playbook/…/✳️any/🦀️.rs:71`). So session 5's first job is to compile and then run it.
- `extension install census` — §B2c.7 closed, 26/26 current. Nothing to do.
- `dag` — the 12 red tests are a peer's live harness edit (§B2c.6); the open item is a measurement.

## S5.1 The `setActiveExample` recipe compiles (the item session 4 never got to)

`🗑️generated/b2c-s5-check1.txt` —
`cargo check -p semio-s-plugin-playbook -p semio-s-plugin-imperative --lib` (both crates in ONE
build-dir lock acquisition):

```
Finished `dev` profile [unoptimized] target(s) in 8m 03s
```

**Zero errors.** Session 4's last recorded error (`A: From<VcsArtifactApp<EditorApp<PlaybookPlayApp>,
SemioMembers>>` unsatisfied) is gone: `PlaybookApplication`'s own bounds now name `SemioMembers`
(`📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:341-352`) and the `dyn_enum_close!` fleet rows in both
plugin roots carry it as the second `VcsArtifactApp` parameter. Both artifact crates
(`semio-s-artifact-playbook-playbook`, `semio-s-artifact-imperative-procedure`) and both plugin crates
type-check with all seven parts of §B2c.4's recipe in place. Checked, not assumed, on the tree as of
2026-09-20 01:30.

Peer edits present in the same files and left alone: the playbook editor root's `🪢️TaxonomyMounts`
block (`👥️presence/🧬️schema` + `📚️examples/🎬️demo-session`) and `🧩️extensions/🌀️procedural/🦀️.rs`'s
`retire_cold` ownership repair (mtimes 00:11/00:14, i.e. another slice) — both compile.

## S5.2 Environment — the wasm-dev build dir is deadlocked (measured, not guessed)

`📜️b2c-activate.sh playbook 6085` reached
`nx run @semio-tech/plugin-registry:session-playbook` and then stopped dead in its component build.
The cargo it spawned (`cargo rustc … -p semio-s-plugin-playbook --target wasm32-wasip2 --profile
wasm-dev`, pid 2168) sat at **0 % CPU with zero `rustc` children for 60+ minutes**. `sample 2168`:

```
DrainState::drain_the_queue → _pthread_cond_wait          (main thread)
compiler::prebuild_lock_exclusive → flock                 (every worker thread)
```

Three wasm-dev cargos are in that state at once — **1257** (`semio-s-plugin-trinity`, a peer's),
**1968** (`semio-s-plugin-animate`, a peer's), **2168** (mine). `lsof` on
`.🧬semio/🦑️repo/⚡️cache/cargo/build/wasm32-wasip2/wasm-dev` shows pid **1257 holding ~40 per-unit
`.lock` files** (`semio-framework-plugin`, `semio-framework`, `semio-framework-ui`,
`semio-s-artifact-stdio-semio`, …) while itself blocked in `prebuild_lock_exclusive` — the
fine-grain-locking wasm deadlock, with a peer's process at the head of the chain. Per preamble rules
3 and 15 no peer process was killed and this slice did not wait on it: the **native** build dir is
healthy (an 8-minute `cargo check` completed through it in this same session), so all remaining
native work was moved ahead of the runtime work.

**Consequence, stated honestly:** §S5.3's crate suites are native measurements. The browser re-proof
of `setActiveExample` on playbook/imperative could not be taken while the wasm lane is wedged —
see §S5.5.

## S5.3 📜️imperative — the full bar, all five steps, zero faults

`🗑️generated/b2b-imperative/report.json`, `b2c-imperative-{activate,serve}.txt`. Activated once
(`exit=0`, 05:11) and served detached on 6076; `🐍️b2b-imperative-probe.mjs`:

```
SUMMARY {"ready":"imperative","error":null,"exampleRendered":true,"actionCount":23,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

`addStep` appends ledger entry 3 `create-step step step-1 kind=log.print bodies={ }`, the rendered
surface moves 17 → 28 chars, undo retires it (28 → 17, edits 1 → 0) and redo reapplies it.
**`faultLines` 2 → 0**: §B2c.4's `setActiveExample` recipe is the whole difference — the verb is now
declared, `Migrated`, `HostOnly`, and its `Effect::LoadDocument` completes its archive closure over
both composed `s.stdio.semio` children. `actionCount` 22 → 23. **imperative clears the bar.**

## S5.4 📖️playbook — the recipe was right; the demo ASSET was stale (new root cause)

The first post-recipe run (`🗑️generated/b2b-playbook/report.json`, 04:54) already showed the recipe
working — `action.setActiveExample` is in the pane, `actionCount` 20 → 21, `faultLines` 2 → 1, and the
one surviving line is no longer `undeclared-action` but a real dispatch:

```
setActiveExample refused: dispatch-failed — AppChannelClient.loadDocumentArchive(s.playbook.playbook@1/*#editor):
{"code":"plugin.internal","message":"document archive genesis child projection failed: child restore projection: InvalidReference"}
```

`ChildRestoreProjection::child` (`🧰️framework/…/🏪️store/🦀️.rs:3167-3172`) rejects any child row whose
`child_id != artifact_id`. Decoding playbook's committed demo asset
(`📖️playbook/…/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio`, hex-encoded DSL) shows exactly that:

| slot | `child_id` | target `artifact_id` (before) |
|---|---|---|
| `document` | `playbook-document-4e206d5157ed80e6` | **`playbook-document`** |
| `flow` | `playbook-flow-762b2640e7a53093` | **`playbook-flow`** |

The minting code is correct and has been for a while — `flow_content_child_handle`
(`📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs:161-172`) builds `ArtifactRef { artifact_id: child_id.clone(), … }`
— so the asset is a stale artefact of an older bare-id convention. Nothing on the boot path noticed,
because only a whole-document `Effect::LoadDocument` runs the restore projection; `setActiveExample`
is the first verb playbook ever had that does. Imperative's own asset already carries the matched
form (`imperative-flow-3faf9c3be6d96916!s.stdio.semio@v1/flow`), which is why imperative went to zero
faults without an asset change.

**Fix:** the asset's two target refs now name their own `child_id`. One file, two fields, no code
change:
`✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio`.

### The same defect in twelve more committed assets (census, not repaired here)

A repo-wide scan of every `*.dsl.semio` under `✏️s/🔌️plugins` for child rows whose target
`artifact_id` differs from the row's `child_id` finds **13** (playbook's two were the first two; they
are now fixed). The rest belong to other slices and are **reported, not touched** — each one is a
latent `InvalidReference` the moment its app gains a whole-document load:

| plugin | asset | slot(s) |
|---|---|---|
| 🎬️sequence | 4 `🧫️fixtures/*` | `content` (×4) |
| ✒️writer | `🖼️assets/🎬️demo`, `📚️examples/…/🧪️dag-example` | `document` (×2) |
| 🎞️animate | `🖼️assets/🎬️demo` | `presentation`, `animation` |
| 🌍️gis | `🧫️fixtures/🗺️mutate-gismap-1/…` | `drawing`, `value` |
| 📕️norm | `⚖️en1990/…/🏢️high-consequence-office`, `⚡️din18599/…/🎬️demo` | `qK`, `climate` |
| 🗄️stdio | `🧿️semio/…/🧰️kit/…/🏢️nakagin-capsule-tower` | `properties` |

norm's two are harmless **today** — `NORM_RETAINED_TOOL_IDS` is `setSnapshot`/`evaluate`/
`setSelectedCheckIndex` with no `setActiveExample`, so no norm app ever runs the restore projection;
they become live faults the day norm gets an example picker.

### Runtime proof after the asset fix

Re-activated (`exit=0`, 05:15) and re-served on 6085, `🐍️b2b-playbook-probe.mjs`:

```
SUMMARY {"ready":"playbook","error":null,"exampleRendered":true,"actionCount":21,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

`addStep` appends ledger entry 4 `add-step step { id=step-op-256 … }`, edits 0 → 1, undo → 0,
redo → 1, and **`faultLines` 1 → 0**. **playbook clears the bar.**

## S5.5 📕️norm — re-measured in this session, still the full bar

Re-activated (`b2c-din4108-activate.txt`, `exit=0`, 05:25) and re-served on 6091;
`🐍️b2b-norm-probe.mjs`:

```
SUMMARY {"ready":"din4108","error":null,"exampleRendered":true,"actionCount":12,
         "mutated":true,"undone":true,"redone":true,"faultLines":0,"interactionBar":true}
```

19 evaluated DIN 4108 rows render, `setSnapshot change-t-int-c new-t-int-c="22.5"` lands as ledger
entry 5, undo retires it, redo reapplies it, 0 faults. §B2c.1's preparation-factory fix holds on a
freshly built wasm component. The briefing's "check `factory_type`: a bare bounded factory means
every action is dead" was also checked and is **not** norm's fault: all fifteen norm editors declare a
real factory (`Din4108BoundedCommandJobFactory`, `En1990BoundedCommandJobFactory`, …) in their
`bounded_first_step_tool_proofs!` block, none is bare.

## S5.6 Bar matrix — batch B, every row measured in session 5

| plugin | variant/port | boot | example | mutating action | undo | redo | console | bar |
|---|---|---|---|---|---|---|---|---|
| 🕸️dag | dag / 6017 | — | — | — | — | — | — | not re-run (B2b: PASS) |
| 💡️reasoning | reasoning-wires / 6015 | — | — | — | — | — | — | not re-run (B2b: PASS) |
| 📖️playbook | playbook / 6085 | ✅ | ✅ Demo | ✅ `addStep` (ledger 4, edits 0→1) | ✅ | ✅ | ✅ **0** | **✅ PASS** |
| 📕️norm | din4108 / 6091 | ✅ | ✅ 19 rows | ✅ `setSnapshot` (ledger 5) | ✅ | ✅ | ✅ **0** | **✅ PASS** |
| 📜️imperative | imperative / 6076 | ✅ | ✅ | ✅ `addStep` (ledger 3, 17→28 chars) | ✅ | ✅ | ✅ **0** | **✅ PASS** |

Batch B is **5 of 5 at the bar** — three proven in this session, two carried from B2b §3.1/§3.2 and
explicitly *not* re-measured here.

## S5.7 Crate suites — blocked, stated as blocked

`cargo deadlock at 07:20 and again at 07:55.` The native `debug` build dir wedged twice.
Attempt 1 (`b2c-s5-test1.txt`, `cargo test -p semio-s-artifact-playbook-playbook -p
semio-s-artifact-imperative-procedure -p semio-s-artifact-dag-dag --lib`) sat **76 minutes** on
`Blocking waiting for file lock on artifact directory` with no `rustc` child; `lsof` on
`⚡️cache/cargo/target/debug/.cargo-lock` showed **eleven** cargos queued on it, *none* of them with a
child process, while the 13–28 live `rustc` processes all belonged to the wasm lane. Per rule 23(a)
that cargo was killed by pid and rerun once, narrowed to the single open item
(`b2c-s5-dag-test.txt`, `cargo test -p semio-s-artifact-dag-dag --lib`); it wedged on the same lock
for a further 20 minutes. It is left running detached — if it lands, its output is in that capture —
but **no test result is claimed here.**

What IS proven natively this session: `cargo check -p semio-s-plugin-playbook -p
semio-s-plugin-imperative --lib` finished green in 8m 03s (§S5.1), which type-checks both artifact
crates and both plugin crates including every part of the `setActiveExample` recipe and the new unit
tests' `context` module signatures.

## S5.8 Files changed by session 5

| file | change |
|---|---|
| `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio` | the `document` and `flow` child rows' target `artifact_id` now equal their own `child_id` — the whole playbook fix (§S5.4) |
| `📓️b2b-dormant-plugin-interactions.md` | this `# Session 5` section |

No Rust source was changed in session 5: §B2c.4's recipe was already in the tree, it compiles, and
after the asset repair it runs.

## S5.9 Honest gaps

1. **The three crate suites were not run** (§S5.7). `semio-s-artifact-dag-dag`'s 12 red tests —
   the slice's item (4) — therefore remain **unmeasured by me**; §B2c.6's finding stands unchanged
   (a peer's harness edit is in the tree and every mount site now closes, but nobody has run it).
   The playbook/imperative suites likewise carry B2c.3's honest split (runtime green, unit harness
   partly red) with no new number.
2. **The two new `set_active_example_…` unit tests compile but have never executed.** They are
   type-checked only.
3. **dag and reasoning were not re-measured** in session 5; their PASS rows are B2b's.
4. **Twelve stale child-ref assets are reported, not repaired** (§S5.4) — sequence ×4, writer ×2,
   animate ×2, gis ×2, norm ×2, stdio ×1. Each is a latent `InvalidReference` for whichever slice
   owns it.
5. The extension install census (§B2c.7) was **not** re-run; it was closed in session 4 at 26/26.
6. Servers left running for the next worker (started by this slice, kill by pid if needed):
   playbook 6085, imperative 6076, din4108 6091, all detached with captures in
   `🗑️generated/b2c-*-serve.txt`.


# Session 5b (2026-09-20 06:20→) — the composed-child-ref defect class

Continuation of §S5.4 at the coordinator's direction: close the defect class, not just playbook's
instance of it, because it blocks F1 (writer/animate/sequence) and B3a2 (gis) on `LoadDocument`.

## S5b.1 The law, and where it can and cannot live

The invariant is already declared, once, in the framework: `ChildRestoreProjection::child`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3162-3172`) refuses any child row with
`fields.child_id != fields.artifact_id` → `InvalidReference`.

The DSL producer/consumer pair for those rows is `artifact_child_to_record` /
`artifact_child_from_record` (same file, `:3417` / `:3425`). I did **not** put a normalising law
there, and the reason is a measurement, not caution: **the repo runs two incompatible conventions**.

| convention | `child_id` | target `artifact_id` | admitted by `ChildRestoreProjection` |
|---|---|---|---|
| composed child (playbook, imperative, dag, animate, sequence, stdio-kit, norm) | content-addressed | **same string** | ✅ |
| durable-group member (🌍️gis) | per-parent handle id | **fixed document id** (`gismap-drawing`) | ❌ |

gis's bare ids are not stale: they are declared constants in
`🧰️framework/…/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:2883-2884,3172-3173`, as JSON-schema
`const` in `✏️s/🔌️plugins/🌍️gis/🧬️schema/🔣️.json:372,392`, and in `🌎️hub/🧫️fixtures/…` plus
`🌎️hub/📦️packages/🦀️rust/📜️script.ts:6943,7042`. A rewrite in `artifact_child_to_record` would
silently corrupt that identity on every round-trip, and a refusal in `artifact_child_from_record`
would stop gis's fixtures parsing at all — in both cases from inside a slice that does not own
durable-group. **So the law is enforced as a repo-wide gate with a named, reasoned waiver**, and the
fork itself is reported below rather than decided here.

## S5b.2 `bun nx run workspace:verify -- composed-child-refs` — the new gate

`📜️script.ts`: `VerifyScript.runComposedChildRefs` plus the module-level
`verifyComposedChildRefRows` / `verifyComposedChildRefAssetCount` / `COMPOSED_CHILD_REF_WAIVERS`.
It walks the whole workspace (skipping `.git`, `node_modules`, `dist`, `target`, `⚡️cache`,
`🗑️generated`), decodes every `<slot>=[<hex child_id>,<hex target uri>]` row that
`artifact_child_to_record` writes, and fails on any row whose target uri names an id other than the
row's own `child_id`. Waived rows are still printed, with their reason. `--json` prints the census.

Capture `🗑️generated/b2c-s5-child-ref-gate.txt`:

```
[verify composed-child-refs] assets=214 mismatched=2 waived=2 breaches=0
[verify composed-child-refs] waived …/🗺️liege-with-derived-regions.dsl.semio drawing: gis durable-group member id, …
[verify composed-child-refs] waived …/🗺️liege-with-derived-regions.dsl.semio value:   gis durable-group member id, …
[verify composed-child-refs] passed.
```

Registered so a dev can run it: a `⚖️gate🪆️composed-child-refs` row in **both**
`.vscode/🧩️launch.seed.jsonc` (the source) and `.vscode/launch.json`, in the `4_gate` group at order
411.42, and added to `INTERACTIVITY_ALL_APP_REQUIRED_GATES` (`📜️script.ts:9147-9155`) — the
self-referential launch-registration law — so the row itself cannot be dropped. All seven required
gate rows verified present with their exact commands.

## S5b.3 Nine stale rows canonicalised

Each row's target uri now names its own `child_id`; no `child_id` changed, so no content address,
fixture identity or oracle key moved. Verified beforehand with `git grep` that none of these bare
ids (`sequence-content!`, `animate-presentation-deck-*!`, `kit-props!`, `en1990-qk!`,
`din18599-climate!`) occurs in **any** tracked `.rs`/`.ts`/`.json` — unlike gis's, they were pure
asset residue with no declared constant behind them.

| plugin | asset | slot | was → now |
|---|---|---|---|
| 🎞️animate | `🎬️presentation/…/🖼️assets/🎬️demo` | `presentation` | `animate-presentation-deck-presentation` → `presentation-e7e4559021a08a5d` |
| 🎞️animate | same | `animation` | `animate-presentation-deck-animation` → `animation-007c0e2d54d34f87` |
| 🎬️sequence | `🪜️step/🧫️fixtures/🪜️mutate-sequence-1-step` | `content` | `sequence-content` → `sequence-content-5a292c39ad916e7c` |
| 🎬️sequence | `🔗️dependency/🧫️fixtures/🔗️mutate-sequence-1-dependency` | `content` | idem |
| 🎬️sequence | `✳️any/🧫️fixtures/🔗️mutate-sequence-1-any-dependency` | `content` | idem |
| 🎬️sequence | `✳️any/🧫️fixtures/🪜️mutate-sequence-1-any-step` | `content` | idem |
| 📕️norm | `⚖️en1990/…/🖼️assets/🏢️high-consequence-office` | `qK` | `en1990-qk` → `en1990-qk-e1e5367e104791d5` |
| 📕️norm | `⚡️din18599/…/🖼️assets/🎬️demo` | `climate` | `din18599-climate` → `din18599-climate-9d5801644cfa2e45` |
| 🗄️stdio | `🧿️semio/…/🧰️kit/🧫️fixtures/…/🏢️nakagin-capsule-tower` | `properties` | `kit-props` → `props-01` |

**No producer verb regenerated these.** I looked: there is no nx/`📜️script.ts` target that emits
`🗣️.dsl.semio` example or fixture assets — they are committed authored documents whose only writer
is `store::artifact_child_to_record` at export time from a live app. So all nine were edited in
place, which is the "hand-edit only assets that have no producer" branch; the gate, not a
regeneration verb, is what keeps them honest from here.

**✒️writer's two rows fixed themselves between my 03:00 census and my 06:40 re-scan** — F1 is live in
that plugin and repaired them; not re-done here.

## S5b.4 The gis fork — reported, for durable-group's owner and B3a2

`🌍️gis`'s `gismap-drawing` / `gismap-value` cannot pass `ChildRestoreProjection` as written, so
**gismap can never take a whole-document `Effect::LoadDocument`** — it would fail exactly the way
playbook did. Two ways out, neither of them this slice's to pick: give the durable-group members
content-addressed ids equal to their handles' `child_id` (touching the framework const, the gis
schema `const`, and hub fixtures together), or teach `ChildRestoreProjection` a declared
durable-group shape. Until then the gate's waiver keeps the fact visible instead of silent.

## S5b.5 Crate suites — rule 25's private uplift dir works; here are the numbers

`CARGO_TARGET_DIR=…/⚡️cache/cargo/target-b2c cargo test -p semio-s-artifact-dag-dag -p
semio-s-artifact-playbook-playbook -p semio-s-artifact-imperative-procedure --lib --no-fail-fast`
(capture `🗑️generated/b2c-s5-uplift-test.txt`). The same command without the private uplift dir had
starved for 76 + 20 minutes on `.cargo-lock` earlier in this session; with it, it ran to completion.
**Rule 25 is confirmed on this slice.**

| crate | this session | B2b/B2c baseline |
|---|---|---|
| `semio-s-artifact-dag-dag` | **198 passed / 9 failed** | 179/27 (B2b §5), "12 red" (§B2c.6) |
| `semio-s-artifact-playbook-playbook` | **139 passed / 18 failed** | not measured since B2b |
| `semio-s-artifact-imperative-procedure` | **111 passed / 33 failed** | 83/59 (§B2c.3) |

Every crate improved. **dag's open item (this slice's item 4) is now measured: 9 red, not 12** — the
peer harness edit §B2c.6 found in the tree did land and did help; the three that §B2c.6 expected to
survive did not. The remaining dag 9 are `every_declared_action_is_registered`, two command tests
and six mutation/binary round-trip rows — none of them a publication or dispatch fault.

### My two `set_active_example_…` tests failed, and the test was wrong, not the product

Both panicked on the same line: `the demo example is the document this verb loads, so it must carry
real steps`. That assertion — written by session 4, never run — is false about the data model. A
`🗣️.dsl.semio` parent document carries only its child HANDLES; the step content lives in the composed
`s.stdio.semio` children, which the asset does not contain, so `parse_dsl(demo)` legitimately yields
an empty working scene and `genesis_child_pack` mints the children from exactly that. The runtime is
the proof it is consistent: playbook and imperative both reach **0 faults** with `setActiveExample`
completing its archive closure (§S5.3, §S5.4), and imperative's first `addStep` produces `step-1`,
i.e. the loaded demo really did have zero steps.

Replaced with the invariant that actually gates the load, so each crate now pins §S5b.1's law itself:

```rust
for (slot, child) in [("document", &demo.document), ("flow", &demo.flow)] {
    assert_eq!(child.target.artifact_id, child.child_id, "slot {slot}'s target must name its own child_id or ChildRestoreProjection refuses the whole load with InvalidReference");
}
```

**Not verified.** Two re-runs of `cargo test -p … set_active_example` both died in a peer's in-flight
refactor — `error[E0432]: unresolved imports ui_wgpu::wgpu::SceneMaterialDraw3d,
SceneMaterialKind3d` in `semio-framework-os-infinite`, a crate and a symbol this slice does not
touch. Capture `🗑️generated/b2c-s5-setactive-test.txt`. The edited assertion is therefore
**type-unchecked and unrun**; it must be re-run once `ui_wgpu` settles.

### Honest note on the demo documents

Both shipped `demo` examples are **empty documents**. That is internally consistent and the bar
passes over them, but "Demo" showing an empty playbook/procedure is a content gap worth someone's
slice — it is not a `LoadDocument` fault.

## S5b.6 Files changed by session 5b

| file | change |
|---|---|
| `📜️script.ts` | `verify composed-child-refs` gate: dispatch row, `runComposedChildRefs`, `ComposedChildRefRow`, `COMPOSED_CHILD_REF_WAIVERS`, `COMPOSED_CHILD_REF_SKIPPED_DIRECTORIES`, `verifyComposedChildRefAssets/AssetCount/Rows`; the gate added to `INTERACTIVITY_ALL_APP_REQUIRED_GATES` |
| `.vscode/🧩️launch.seed.jsonc` | `⚖️gate🪆️composed-child-refs` row (order 411.42) |
| `.vscode/launch.json` | the same row (a peer regenerated the file mid-edit and left a duplicate; the duplicate was removed and all seven required rows re-verified) |
| 8 `🗣️.dsl.semio` assets (animate ×1 file/2 rows, sequence ×4, norm ×2, stdio ×1) | nine child rows canonicalised |
| `📜️imperative/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs`, `📖️playbook/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | the false "demo must carry real steps" assertion replaced by the canonical-child-ref law (§S5b.5); **unrun** |

## S5b.7 Open for the next worker

1. Re-run `CARGO_TARGET_DIR=…/target-b2c cargo test -p semio-s-artifact-playbook-playbook -p semio-s-artifact-imperative-procedure --lib set_active_example` once the peer's `ui_wgpu::wgpu::SceneMaterialDraw3d` refactor lands. It is the only unverified edit this slice leaves.
2. The gis durable-group fork (§S5b.4) needs a decision from durable-group's owner + B3a2.
3. dag's remaining 9, playbook's 18, imperative's 33 — all pre-existing, none a dispatch/publication fault; the two biggest imperative clusters are still §B2c.3's `final Dictionary ownership` rows from `neural_engine` and `interactive-job.live-instance` harness debt.
