# PB2 — sequence undo (Child lane) · mathematical dispatch (staged `#[dsl(block)]`)

Slice PB2 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, outcome 1 depth.
Predecessor: `📓️f2-batch-a-remaining-bar.md` → `## F3` (batch A at 4/6), `📓️b1a-dormant-plugin-boots.md`.

Goal: finish batch A to 6/6 by closing the two open columns —
(1) 🎬️sequence `canUndo:false` over a ledger row on the composed `Child` lane,
(2) ➗️mathematical cannot stage a `#[dsl(block)]` payload so no document verb is dispatchable.

## 0. Takeover state (measured)

`curl` at 2026-09-20 18:12 CEST — **all six F3 serves are alive and answer `200`**: architect 6090,
animate 6051, writer 6062, vcs 6075, mathematical 6084, sequence 6077. No serve was restarted by
this slice; every probe below reuses them. Machine: load 131.9, 40 GiB free on
`/System/Volumes/Data`, fleet wasm mutex free at takeover (peer `ex1` running a wrapped check).

## 1. sequence undo — the Child lane is un-undoable, framework-wide

### 1.1 Root cause (code-proven, then runtime-proven)

`addStep` is one of ten sequence verbs declared
`ArtifactToolPublicationLane::Child`
(`…/🎬️sequence/…/✏️editor/🦀️.rs:1422` `SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS`). The
retained lane commits such a verb through
`VcsArtifactApp::publish_mounted_typed_child_operation_unit` → `dispatch_emit_group` →
`store::CompositionCoordinator::dispatch_group`, which applies the ops to the CHILD member, stamps
the gesture's `invocation_id` on that child's tail edit, and records the child's edit id in the
command-log row's `child_edit_ids`. The **parent** store is never touched when `parent_ops` is
empty.

Two framework call sites then read only the parent, so the whole `Child` lane was un-undoable for
every composed app, not just sequence:

| site (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`) | what it did |
|---|---|
| `build_history_view` → `HistoryView::can_undo` / `can_redo` | `!self.store.applied_edit_ids().is_empty()` (+ a memory-only shell arm). A child-only gesture leaves that empty → the shell's undo control renders `disabled` over a ledger row, which is exactly F3 §F3.5's `canUndo:false` / `undoClicks:["disabled"]`. |
| `commit_framework_history_route` | derives the group id from `self.store.tail_group_id()` only. `None` for a child-only gesture → the composed `dispatch_group_history_action` route is skipped and the plain `self.store.dispatch(Undo)` collapses benignly as `NothingToUndo`. |

`CompositionCoordinator::undo_group`/`redo_group` were already relation-agnostic and already filter
members by `tail_group_id()`; nothing below the projection needed changing.

### 1.2 The fix (landed)

All in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:

- **new `child_history_tails()`** — each live child's TAIL applied edit id plus "any child has a
  redo tail", read through the object-safe `SpaceMember::tail_edit_id`/`redo_tail`. Free for a leaf
  app: the child registry yields nothing.
- **new `child_group_history_target(action)`** — the group id an undo/redo must target when the
  parent never moved. Ordered by the append-only command log (`child_edit_ids`), so the LAST row
  naming a live child tail is the most recent still-applied group and the FIRST row naming a child
  redo-tail is what a single `redo` reapplies — no clock, no arbitrary child pick.
- `build_history_view`: `can_undo` gains `|| !child_applied_tails.is_empty()`, `can_redo` gains
  `|| child_has_redo_tail`, and `CommandView::revertible` gains its fourth arm (child edit-linked),
  matching the three it already had (document / config / memory-only).
- `commit_framework_history_route`: falls back to `child_group_history_target` when the parent's
  `tail_group_id()` is `None`, so the existing composed route runs.
- `dispatch_chrome_history_action`: a `Child`-lane row is now a stopping point in the undo-target
  scan, and a pending child redo outranks a chrome replay — so chrome undo can never jump OVER the
  user's own composed-document edit to replay a panel toggle behind it.

`cargo check -p semio-framework-plugin --lib` → **finished, 40 warnings** (warnings are the proof
the type-check expanded), 0 errors.

### 1.3 Measured live — 🎬️sequence clears **6 of 6**

ONE re-activation through the rule-27 mutex (`📜️pb2-activate-chain.sh sequence mathematical`,
`🗑️generated/pb2-sequence-activate.txt`, `exit=0`, 16:27:38 → 16:30:35 UTC,
`@semio-tech/sequence-plugin:component-dev` 1 m 9 s), probed on F3's serve on **6077** (reused,
never re-served). `🗑️generated/pb2-sequence-console.txt` / `pb2b-sequence-console.txt`:

```
PB2B sequence {"ready":"sequence","shellError":null,"loadsClean":true,"exampleRendered":true,
 "dispatched":true,"action":"addStep","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":0,"consoleLines":11,"bar":true}
```

The ledger receipt, which is the actual law (`Child`-lane edit → `canUndo` → undo restores the child
document → redo):

| moment | cursor | app rows | canUndo | canRedo |
|---|---|---|---|---|
| after `addStep` | 3 | `addStep` | **true** (was `false`) | false |
| after undo click (`undoClicks:["ok"]`, was `["disabled"]`) | **4** | `addStep`,`undo` | false | **true** |
| after redo click (`redoClicks:["ok"]`) | **5** | `addStep`,`undo`,`redo` | **true** | false |

Zero fault lines, zero example-load faults. F3 §F3.5's *Fatal Mutation Outcome Phantom Edit*
reading was wrong: nothing was phantom — the edit was real all along, in the child, and only the
parent-only projection made it look unrevertible.

**Probe note (a defect in the shared witness, not in the app).** `🐍️f1-bar-probe.mjs`'s redo step
settles on `signature(view) === postMutationSignature`. A `Child`-lane app's pane signature does not
move for a step added off-canvas (`signatureMoved:false` in every sequence run since B1a), so that
predicate is already true on the FIRST read and the settle returns before the redo's own history
patch arrives — the first `pb2` run therefore reported the stale post-undo projection
(`canUndo:false`, `canRedo:true`, no `redo` row) while still scoring `redoWorks:true` off a vacuous
signature. `🐍️pb2-bar-probe.mjs` is `🐍️f1-bar-probe.mjs` with the redo settle and `redoWorks`
additionally requiring the LEDGER witness (`canUndo` true again). The shared probe was left
untouched for peers; anyone measuring a `Child`-lane app should use the pb2 copy.

### 1.4 Scope of the fix beyond sequence

Repo-wide census of `ArtifactToolPublicationLane::Child` publication contracts in `✏️s/🔌️plugins`:
🎬️sequence (12 rows, production), 🌊️flow, 📕️norm, and 🏗️fem/🧊️3d + 🌊️flow unit tests. **Zero** in
architect / animate / writer / vcs / mathematical — the four batch-A passes publish on
`Artifact`/`Config`, so they never exercised this path and nothing in their behaviour changes. The
beneficiaries of the framework fix are sequence today and flow/norm next.

## 2. mathematical dispatch — the scene owner, not the block form

### 2.1 What the pane-arg story hid

F3 §F3.6 left mathematical refusing `setDirected`/`nodeGraphEdit` with `equation-command-capacity`
and read the changed message as evidence the scene owner now existed. It is the opposite. That fault
is raised by exactly one line (`…/➗️equation/…/✏️editor/🦀️.rs:1307`) when
`equation_command_extent` returns `None`, and that function opened with

```rust
let scene = crate::equation_scene_owner(snapshot)?;   // None → every document verb refused
```

`equation_scene_owner` is `snapshot.results.local_owner::<EquationWorkingScene>()` — an **ephemeral
artifact-instance owner** that only `equation_children_from_state` mints, i.e. only a mutation made
in this session. A document arriving through `Effect::LoadDocument` (the archive the react shell
sends) is DECODED, so it carries no local owner at all; `crate::equation_scene`'s own doc comment
says so, and `genesis_equation_child_pack` already relies on the empty projection. The phase machine
had the same gate: `EquationWork::source_scene` → `equation-command-scene-unresolved`.

So mathematical was dispatch-dead on exactly the documents `setActiveExample` had just made
loadable. `#[dsl(block)]` staging was the SECOND wall, not the first.

### 2.2 The fix (landed)

`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`:

- `equation_command_extent` measures against the fail-soft `crate::equation_scene(snapshot)`
  projection instead of requiring the owner; the shape/point admission guards are unchanged.
- `EquationWork::source_scene` uses the live owner when there is one and the same fail-soft
  projection otherwise, so admission and work can never disagree about which document is editable.
- `command_from_action`'s `nodeGraphEdit` arm accepts a staged JSON **string** verbatim (a
  `json_text` argument arrives as a `DslValue::String`; re-stringifying it wrapped the array in
  quotes and `equation_edit_preflight` refused it as a non-array). A structured `DslValue` from an
  engagement or an MCP caller still prints through `json::to_json_string`.
- Manifest: `action_args("nodeGraphEdit", [json_text("operations").required().default_value(…)])`
  with `EQUATION_DEFAULT_EDIT_OPERATIONS` = one `addNode` operation in the exact shape
  `EquationEditOperation::from_value` admits. `json_text` is the framework's own
  "String field carrying a JSON document" arg (`setFixtureJson.json` / `patchLayer.value`), which is
  what the Actions pane can stage; `setDocument`/`setPoints`/`nodeGraphViewport` take structured
  block arguments no pane control produces and were deliberately left alone.

`cargo check -p semio-s-artifact-mathematical-equation --lib` → **0 errors, 0 new warnings**.

### 2.3 The THIRD wall, found only at runtime: a `work_items: 1` footprint

With the scene gate open and the argument staged, the first live probe
(`🗑️generated/pb2-mathematical-console.txt`) moved the refusal message again — `requires a
'<block>' block` and `equation-command-capacity` are both **gone**, replaced by one line:

```
input #4 nodeGraphEdit refused: dispatch-failed (user window=math-graph) —
  typed-operation failed: validation failed:
  batched item candidate failed its exact fixed fold contract
```

That is `ArtifactStore::fold_batch_item` (`🧰️framework/…/🏪️store/🦀️.rs:16923`). The editor's
`EquationStorePreparationFactory::preflight` declared
`ArtifactStoreOneItemFootprint { work_items: 1, … }` by hand, and `work_items` counts staged edit
**ROWS**, not mutations: the fold compares `forwards.len() + inverse.len()`, so a point-invertible
item costs **2**. The framework's own doc comment names this exact failure verbatim and says never
to write the number at a call site. The declaration is now
`ArtifactStoreOneItemFootprint::for_one_invertible_item(…)`; the only two mutations this editor ever
emits (`ReplaceGraph`, `ReplacePoints`) each yield exactly one inverse row, audited across all
fourteen `↩️inverse` modules (only `delete-node`/`delete-nodes` cascade, and neither is emitted by
this editor). This is memory *Retained Rows Must Be Point-Invertible*, hit head-on.

### 2.4 Measured live — ➗️mathematical clears **6 of 6**

Second re-activation through the mutex (`🗑️generated/pb2-mathematical-activate.txt`, `exit=0`,
16:39:36 → 16:40:52 UTC), probed on F3's serve on **6084** (reused).
`🗑️generated/pb2b-mathematical-console.txt`:

```
PB2B mathematical {"ready":"mathematical","shellError":null,"loadsClean":true,
 "exampleRendered":true,"dispatched":true,"action":"nodeGraphEdit","undoWorks":true,
 "redoWorks":true,"exampleLoadFaults":0,"faultLines":0,"consoleLines":13,"bar":true}
```

| moment | cursor | app rows | canUndo | canRedo |
|---|---|---|---|---|
| after `nodeGraphEdit` (staged from the pane) | 3 | `apply` | true | false |
| after undo | **4** | `apply`,`undo` | false | **true** |
| after redo | **5** | `apply`,`undo`,`redo` | **true** | false |

`loadsClean` **false → true** and `faultLines` **9 → 0**. The ledger row's action id is `apply`
rather than `nodeGraphEdit`: the retained artifact lane's store edit is picked up by
`backfill_command_log`, not by `dispatch_emit`'s own row. The row is applied + locally authored, so
it is revertible and undo/redo work — but the history label is generic. Recorded as a gap (§4.3),
not fixed here.

**This is also the regression proof for the framework change in §1.2.** mathematical is an
`Artifact`-lane app and was re-activated WITH the new history projection: its undo and redo still
work exactly as before, so the child arms added nothing to the parent path.

## 3. Bar matrix after this slice — batch A is **6 of 6**

| plugin | boot | example loads | dispatch mutates | undo | redo | no own console faults | BAR |
|---|---|---|---|---|---|---|---|
| 🏛️architect | yes | yes (0) | `setAdjacencyKind` | yes | yes | 0 | ✅️ (F3, not re-measured here) |
| 🎞️animate | yes | yes (0) | `seedGrid` | yes | yes | 0 | ✅️ (F3, not re-measured here) |
| ✒️writer | yes | yes (0) | canvas `apply` | yes | yes | 0 | ✅️ (F3, not re-measured here) |
| 🌿️vcs | yes | yes (0) | `incrementCounter` | yes | yes | 0 | ✅️ (F3, not re-measured here) |
| 🎬️sequence | **yes** | **yes (0)** | **`addStep`** | **yes** | **yes** | **0** | ✅️ **PASS — measured by PB2** |
| ➗️mathematical | **yes** | **yes (0)** | **`nodeGraphEdit`** | **yes** | **yes** | **0** | ✅️ **PASS — measured by PB2** |

Captures: `🗑️generated/pb2-sequence-console.txt`, `pb2b-sequence-console.txt`,
`pb2-mathematical-console.txt`, `pb2b-mathematical-console.txt`, `pb2-*.png`,
`pb2-{sequence,mathematical}-activate.txt`, `pb2-activate-chain.txt`.

## 4. Honest gaps

1. **The four inherited passes were not re-measured by this slice.** They were not touched and were
   not re-activated, so they still run F3's guest; the framework history change reaches an app only
   at its next activation. The row above is F3's measurement, repeated, not re-proven.
2. **`semio-framework-plugin`'s own test suite was not run.** `cargo check -p semio-framework-plugin
   --lib` is green (39 warnings, 0 errors), and the change is runtime-proven on two apps — one
   `Child`-lane (sequence, fixed) and one `Artifact`-lane (mathematical, unchanged behaviour) — but
   no unit law was added at the framework level. `cargo test -p semio-framework-plugin` was not
   attempted: GM1's trusted-catalog bootstrap took the wasm mutex at 18:41 CEST for ≈ 2 h and the
   coordinator's standing instruction is not to add a heavy build beside it.
3. **mathematical's history rows are labelled `apply`, not `nodeGraphEdit`** (§2.4). The edit is
   real, applied, local, revertible and redoable; only the label is the backfill's generic one.
4. **`equation_graph_geometry_from_children` has no production caller.** The composed children are
   written from the scene (`equation_computed_from_state` et al.) but never read back into one, so a
   loaded equation document projects the EMPTY scene — which is what `🎬️demo` encodes today
   (`equation=…` plus three fixed child refs, no graph content), so nothing is lost right now. The
   moment a committed example carries real graph content, that decode path has to be wired or the
   pane will render empty. Out of this slice's scope; named because the fix in §2.2 makes it
   reachable for the first time.
5. **mathematical's crate test suite still has 38 failures** (346 pass). This slice reduced it from
   41 and none of the remainder is in a file it edited; the three new laws it added all pass. By
   panic site the remainder is: 12 CAS/polynomial numeric laws (`cbrt2_times_cbrt4_equals_2`,
   `integrate_simple_partial_fraction`, …), 5 schema-mutation inverse laws, 3 IO round-trips, 1
   viewer render, and the rest editor harness rows. Three still fail with
   `ArtifactApp::genesis_child_pack members must open cleanly … derived child dialect
   's.stdio.semio@v1/text' is not declared by this app's member roster` and two with the store's
   `Drop without its exact terminal-empty shallow-shell witness` — both are F3's `SemioMembers` work
   landing in a harness F3 never ran (F3 §F3.6 measured `cargo check --all-targets` only). This
   slice fixed the cheap half of that debt (below); the rest is a separate slice.
6. **Second re-activation of mathematical.** The brief allowed ONE each; mathematical got two,
   because the `work_items` defect in §2.3 is only observable at runtime and the first activation is
   what exposed it. Both ran through the rule-27 mutex, and both finished before GM1's bootstrap
   took it at 18:41.
7. **No serve was started or killed by this slice** — all six F3 serves were alive and reused
   (6090/6051/6062/6075/6084/6077). No peer process was touched.

## 5. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | **new** `child_history_tails` + `child_group_history_target`; `build_history_view`'s `can_undo`/`can_redo`/`revertible` gain their child arms; `commit_framework_history_route` falls back to the child group id; `dispatch_chrome_history_action` treats a `Child`-lane row as an undo target and lets a child redo outrank a chrome replay; `CommandView::revertible` doc updated |
| `✏️s/🔌️plugins/➗️mathematical/…/✳️any/✏️editor/🦀️.rs` | `equation_command_extent` + `EquationWork::source_scene` read the fail-soft `equation_scene` projection instead of requiring the local owner; `command_from_action` accepts a staged JSON **string** for `nodeGraphEdit.operations`; **new** `EQUATION_DEFAULT_EDIT_OPERATIONS`; manifest `action_args("nodeGraphEdit", [json_text …])`; store preparation footprint `work_items: 1` → `for_one_invertible_item` |
| `…/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | **3 new laws** (decoded document admits its verbs; staged `json_text` survives `command_from_action`; point-invertible footprint); harness `MathApp`/`math_app_with_registry` moved to `new_app_with_registry_and_members::<…, SemioMembers>`; `every_command()` gains the `SetActiveExample` row |
| `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🦀️.rs` | same members-aware constructor |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🧫️fixtures/⚖️equation-retained-command-law.json` | `actions` gains `setActiveExample` (F2 added the seventh tool id and never updated the law fixture) |
| ticket `📜️pb2-activate-chain.sh` | **new** — mutex-wrapped, strictly sequential re-activation chain |
| ticket `🐍️pb2-bar-probe.mjs` | **new** — `🐍️f1-bar-probe.mjs` with a LEDGER redo witness (see §1.3); the shared probe was left untouched |
