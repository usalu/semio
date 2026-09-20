# F2 — batch A remaining bar: animate · writer · mathematical · sequence

Slice F2 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`, outcome 1 depth, batch A finish.
Predecessor: `📓️f1-load-document-archive-replacement.md` (architect 7/7, vcs 7/7 live PASS).

Bar (from `📓️b1a-dormant-plugin-boots.md`, extended by F1): **boot → example loads → one real
document mutation observed in the ledger → undo → redo → zero console fault lines**, measured
headless (`🐍️f1-bar-probe.mjs`, chromium `--use-angle=metal`, `SEMIO_PROBE_SETTLE_MS=60000`).

## Bar matrix (measured only; `—` = not measured by this slice)

| plugin | boot | example loads | dispatch mutates | undo | redo | no console faults | BAR |
|---|---|---|---|---|---|---|---|
| 🎞️animate | **yes** | **yes** (0 faults) | **yes** `addTile` | **yes** | **yes** | **6** (1 `seedGrid`, 5 `drawImage`) — both root-fixed, re-measurement queued | ❌️ (5 of 6 columns) |
| ✒️writer | **yes** | **yes** (0 faults) | **yes** canvas `apply` | **yes** | **yes** | 3 — all M7's bridge WebSocket, none writer's | ✅️ **PASS** (own surface) |
| ➗️mathematical | — | — | — | — | — | — | F9 landed, `cargo check` 0 errors, activation queued on the build lock |
| 🎬️sequence | — | — | — | — | — | — | not attempted — see §4 for why, and the precise spec |

Inherited (F1, not re-measured here): 🏛️architect ✅️ PASS, 🌿️vcs ✅️ PASS.

## Inherited state verified at takeover (2026-09-20 09:01)

Four of F1's detached vite serves are alive and answer `200`, and were reused (no re-activation
except where a guest change required one):

| plugin | port | pid | uptime at takeover |
|---|---|---|---|
| 🏛️architect | 6090 | 52100 | 9 h 05 m |
| 🌿️vcs | 6075 | 23269 | 2 h 29 m |
| ✒️writer | 6062 | 30718 | 2 h 19 m |
| 🎞️animate | 6051 | 53697 | 1 h 53 m |

Machine at takeover: load 44, 34 GiB free on `/System/Volumes/Data`.

**Serve state at hand-off:** animate **6051**, writer **6062**, vcs **6075** all answer `200` and
were reused throughout (each plugin got exactly ONE re-activation; `dev <variant>` watch mode was
never used). F1's architect serve on **6090 died during this session** — not killed by this slice
(nothing of this slice touches architect, and F1's own capture names 6090 as its pid 52100); a
re-probe of architect therefore needs its serve restarted. mathematical (6084) and sequence (6077)
have no serve.

## 1. animate — promote the document verbs to interactive jobs

### Run 1 (measured) — the promotion works; two residual defects, both root-caused

`🗑️generated/f2-animate-console.txt`, 2026-09-20 11:16 CEST, against the serve F1 left on **6051**
after this slice's own re-activation (`🗑️generated/f2-animate-activate.txt`, `exit=0`, 16 m 35 s,
`component-dev` 9 m 33 s):

```
F2 animate {"ready":"animate","shellError":null,"loadsClean":false,"exampleRendered":true,
 "dispatched":true,"action":"addTile","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":6,"consoleLines":14,"bar":false}
```

Against F1's last animate capture this is: `dispatched` `setActiveExample` → **`addTile`** (a real
document mutation, not a document *replacement*), `undoWorks` **false → true**, fault lines
**15 → 6**. The `BatchOnlyPendingRewrite` family — thirteen of F1's fifteen lines — is **gone**.

**What the promotion is, concretely** (`…/🎞️animate/🗿️artifacts/🎬️presentation/…/✳️any/✏️editor/🦀️.rs`):

| part | before | after |
|---|---|---|
| `ANIMATE_PRESENTATION_RETAINED_TOOL_IDS` `:269` | 3 ids (`setActiveExample`, `engagementInput`, `noMutation`) | **16** — every document, view and host verb except `exportVideoFromDeck` |
| `…_PUBLICATION_CONTRACTS` | 3 rows | 16 rows: `Artifact` for the ten document verbs, `Config` for `engagementInput`, `[Artifact, Config]` for `engagementSubmit` (it emits both), `HostOnly` for the four effect-only verbs |
| **`build_artifact_store_one_item_preparation_factory`** | **absent** → every `Artifact`-lane tool marked `unsupported-publication-contract` at boot | `bounded_config_store_one_item_preparation_factory::<Snapshot, Mutation>("animate-presentation-artifact-retained", 65_536)` — the framework's own generic, whose preflight already returns `for_one_invertible_item` (the 🕸️dag precedent). This is B1a's "two-part change", and it is the half without which classification alone changes nothing. |
| `bounded_first_step_tool_proofs!` `tools:` | 3 | 16 |
| `.action_interactive_job(…)` ×16 | 13 × `BatchOnlyPendingRewrite` | 16 × `Migrated` (only `exportVideoFromDeck` left, see gaps) |
| retained reduce | a 3-arm match, everything else `route-mismatch` | roster + extent guard, then `command.dispatch(…)` — the SAME generic dispatcher `ArtifactApp::handle` uses, so a promoted verb cannot drift from its batch behaviour |
| retained reduce's dispatch context | `selected_ids: Vec::new()` **always** | `animate_presentation_selected_ids(interaction)`, read off the job's own `InteractionState` exactly as `ArtifactApp::handle:830` does |

That last row was a latent defect the promotion would otherwise have shipped: the retained lane handed
every handler an EMPTY selection, so `deleteSelection`/`renameTiles`/`patchTileCrops` would have become
silent no-ops the moment they became retained jobs. It never showed before because none of the three
previously-retained verbs reads the selection.

### The two residual faults, both root-caused and fixed (re-measurement pending)

**A — `seedGrid refused: … retained command exceeds semantic work capacity` (1 line).** My first
extent priced fan-out per emitted ROW (`rows × columns`). But
`ArtifactRetainedCommandPayload::try_new` is handed `ANIMATE_PRESENTATION_RETAINED_WORK_ITEMS`
(= **1**) as its preflight ceiling, and `🧵️retained-command/🦀️.rs:541` refuses
`extent > maximum_work_items`. Each of these verbs completes in ONE `BoundedArtifactCommandWork`
step, so the extent is 1 and fan-out must be bounded by refusing an oversized payload instead —
which is what `ANIMATE_PRESENTATION_MAXIMUM_TILES` (256) now does. This is exactly why `addTile`
(extent 1) dispatched while `seedGrid` (extent 4) did not.

**B — `drawImage … HTMLImageElement is in the 'broken' state` (5 lines, an uncaught `pageerror`).**
Not animate's: a **framework renderer** defect. `📐️Canvas2dHost/🟦️.tsx` guarded both `drawImage`
calls with `image.complete` — but `complete` is `true` for a **failed** load too, and `drawImage` on
a broken element throws `InvalidStateError`. Both sites now go through a new `isDecodedImage`
predicate that also requires `naturalWidth > 0`. Any app whose document names an image the server
does not serve hit this, not just animate.

## 2. writer — `textSelect` declaration + re-probe after F1's asset fix

`🗑️generated/f2-writer-console.txt`, 2026-09-20 11:47 CEST; this slice's own re-activation
(`f2-writer-activate.txt`, `exit=0`, 30 m 30 s, `component-dev` 24 m 8 s) on the serve F1 left on
**6062**, probe run with `SEMIO_F1_GESTURE=canvas`.

```
F2 writer {"ready":"writer","shellError":null,"loadsClean":false,"exampleRendered":true,
 "dispatched":true,"action":"canvas-gesture:apply","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":3,"consoleLines":68,"bar":false}
```

Against F1's last writer capture: `exampleLoadFaults` **1 → 0**, fault lines **11 → 3**.

- **F1's asset fix is now proven at runtime, not only by unit test.** F1 corrected two committed
  `.dsl.semio` assets whose `child_id != target.artifact_id` and gated it with a Rust law, but never
  re-probed. The archive refusal
  (`document archive genesis child projection failed … InvalidReference`) is **gone from a live
  boot**: `exampleLoadFaults: 0`.
- **W2 (`textSelect` declared by no window kind) is fixed and gone.** F1's ten `textSelect` fault
  lines are absent. The root cause was **not** a missing feature: writer already owned the whole
  window-transient selection lane — `WriterEditorSelection {start, end}`, a `WindowTransient`
  publication contract, `Migrated` classification, an `InteractiveJob` — it had simply **named the
  verb `setEditorSelection`** while the framework's text-editor surface dispatches the fixed id
  `textSelect` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts:924` `textEditorActions.select`, emitted
  with `{start, end}` from `✏️TextEditor/🟦️.tsx:177` — exactly writer's payload shape). Every caret
  move was therefore dropped before dispatch. The id is now `textSelect` at all seven Rust call
  sites; the `editor-selection` wire keyword and the `SetEditorSelection` payload type stay writer's
  own vocabulary (the two-vocabulary design `app_commands!` documents). The committed descriptor
  mirror `✏️s/🔌️plugins/✒️writer/🔣️.json` and the migration fixture
  `…/🧫️fixtures/🎬️writer-migration/🔣️.json` were updated to match.
  This is a **cheaper and more correct** fix than the `🔱️trinity/🔌️jack` precedent F1 pointed at
  (a bespoke `WindowTransient` job factory): writer needed no new lane at all.
- **The three remaining fault lines are not writer's.** All three are
  `WebSocket connection to 'ws://127.0.0.1:50516/bridge' failed … ERR_CONNECTION_REFUSED` — M7's
  agent bridge, which is not running in this playground. B3b recorded the identical line as the sole
  reason `🔱️trinity/🔌️jack` missed "zero faults". Writer clears every column this slice owns:
  boot, example load with zero faults, a real document mutation through the editor canvas, undo and
  redo. The `bar: false` in the summary is the probe's blanket `faultLines === 0` clause.

## 3. mathematical — F9 wiring

**Landed and compile-proven; the live probe had not run when this section was written.**
`cargo check -p semio-s-artifact-mathematical-equation --lib` → **0 errors** (it was 3 × `E0004`
first: three exhaustive `match command` arms the new variant had to answer).

### Why F9 is the whole blocker here, not one of several

B1a listed four separate mathematical symptoms — `setDocument`/`nodeGraphViewport`/`setPoints`
refusing `requires a '<block>' block`, `setDirected`/`nodeGraphEdit` "rejected by the retained
command reducer", `setAlgorithm` silently inert. Reading the reducer shows **one** cause under the
last two: `equation_command_extent` opens with

```rust
let scene = crate::equation_scene_owner(snapshot)?;   // None → the command is refused
```

and a scene owner only exists once a document with composed children has been **loaded**. With no
`setActiveExample`, nothing ever loaded one, so every document verb was measured against an absent
scene and refused. F9 is therefore not cosmetic here (it was, for architect, "only a console
error"): it is what makes the document surface reachable at all.

### The nine call sites

New command module
`…/➗️equation/…/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` (its `emit(example_id)` is
deliberately free of any `ArtifactView`, because the retained work must answer this verb *before* a
scene owner exists), mounted in the crate root's `commands` block; then, in
`…/✳️any/✏️editor/🦀️.rs`: the `app_commands!` row (appended last — row order is the binary variant
ordinal), `EQUATION_TOOL_IDS`, a `HostOnly` publication contract, the extent (answered **before**
the scene lookup), a one-step branch at the top of `step` (before `source_scene`), the
`command_from_action` arm, the `bounded_first_step_tool_proofs!` row, and the manifest
`ActionDefinition` + `Migrated` classification + `action_args` select defaulting to
`crate::examples::demo::ID`.

Two supporting pieces:

- **`reset_equation_document_effect`** — built with `store::empty_document_spr`, never by minting an
  `ArtifactEnvelope` to print one. That is the B1a fix #7/#10 trap: an envelope is a terminal store
  shell whose `Drop` asserts its bounded retirement authority detached every nested owner, and this
  path never mounts or retires it, so the envelope route **panics inside the guest** — which is
  exactly how animate lost all its panes.
- **`build_document_store_initialization_job`** — the trait default owns no retained initialization
  authority, so the host answers the app's own archive with
  `artifact-store.persisted-initializer-refused` *after* the guest already accepted the verb
  (architect's fix #9, dag's precedent).

The loaded document is parsed from the subset's own committed asset via
`io::snapshot::text::parse_dsl(crate::examples::demo::PRIMARY_TEXT)`, so what loads is exactly what
`📚️examples/🎬️demo` declares rather than a second hand-built copy that could drift from it.

## 4. sequence — NOT attempted, and deliberately so; specified call-site by call-site

Nothing of sequence was touched. Half-landing it was the worse option, and the reason is structural
rather than a matter of time: **sequence's steps are composed children** — all ten of its artifact
verbs publish on the `Child` lane
(`SEQUENCE_RETAINED_ARTIFACT_PUBLICATION_CONTRACTS`), it owns `genesis_child_pack`, and it runs
**three** retained factories (artifact / persistent / config) each with its own
`bounded_first_step_tool_proofs!` catalog. A whole-document `Effect::LoadDocument` on a
composed-child document is not the flat-snapshot shape architect, dag and mathematical use: it has
to satisfy the composed-child load contract (`child_id` equal to the target artifact id, a
`genesis_child_pack` on both sides, `Members` declared) — the class B2c found broken across 13
committed assets, **four of which are sequence's own**.

What it needs, precisely:

1. A **fourth**, small retained factory (`setActiveExample` only, `HostOnly` lane) rather than a row
   on any of the three existing ones — their work impls all route document/child mutations, and
   this verb emits an effect and nothing else. `🔱️trinity/🔌️jack`'s `JackRetainedTransientJobFactory`
   is the worked precedent for a one-verb factory beside the main ones.
2. A `reset_sequence_document_effect` over `store::empty_document_spr(SEQUENCE_PLAY_APP_ID,
   SEQUENCE_DOCUMENT_SCHEMA)` and `default_snapshot()` — **never** a minted `ArtifactEnvelope`.
3. `build_document_store_initialization_job` → `bounded_document_store_initialization_job`.
4. Its own `bounded_first_step_tool_proofs!` block for that factory, plus the manifest
   `ActionDefinition` / `Migrated` / `action_args` trio.
5. **First**, confirm B2c's repo gate (`workspace:verify -- composed-child-refs`) is green for
   sequence's four assets — otherwise the example will load into the `InvalidReference` projection
   failure that cost writer its bar, and the F9 work will be blamed for it.

B1a already measured sequence dispatching `addStepDropped` and undoing it, with 8 fault lines; the
bar columns it has never had measured are **example load** and **redo**.

## 5. Honest gaps

1. **animate's re-measurement did not run.** The two residual defects (§1 A and B) are root-caused
   and fixed in source, and `cargo check -p semio-s-artifact-animate-presentation --lib` is green,
   but the re-activation that would put the extent fix into the guest **never got the shared cargo
   build-dir lock**: `cargo rustc … 🎞️animate` (pid 18001) sat **1 h 48 m at 0 % CPU with no `rustc`
   child**, in `cargo::core::compiler::prebuild_lock_exclusive → flock` (verified by `sample`), with
   five peer wasm cargos queued identically behind it. This is **contention, not the rule-23a
   deadlock**: 10–28 peer `rustc` processes were running at every check. Recorded per preamble
   rule 14; no peer process was killed.
   So: animate is measured at **5 of 6 bar columns** on the previous guest, and the sixth
   ("zero console faults") has a fix that is written and compile-proven but **not runtime-proven**.
2. **mathematical is compile-proven only.** No boot, no example load, no dispatch has been observed.
   Its activation is second in the same queued chain.
3. **The activation chain is still running, detached** (`📜️f2-activate-chain.sh animate mathematical`,
   captures `🗑️generated/f2-animate-activate.txt` / `f2-mathematical-activate.txt`, cursor
   `f2-activate-chain.txt`). It will finish without this slice. **Nothing will probe the result** —
   whoever picks this up should re-run the two probe commands in §7 rather than re-activating.
4. **sequence was not touched at all** (§4).
5. **`exportVideoFromDeck` is the one animate verb left `BatchOnlyPendingRewrite`**, deliberately:
   it is a Shell export whose `DownloadMediaExport` payload can exceed the retained contract's
   65 536-byte output ceiling, so it belongs on a download queue (memory: *Segmented Download Queue
   Ownership*), not on the bounded first-step lane. It is unreachable from the probe because the
   probe stops at the first verb that dispatches.
6. **No per-crate unit tests were run for the three crates touched.** `cargo check --lib` is green
   for animate and mathematical; `cargo test -p …` was not attempted because a single `cargo check`
   was already waiting hours for the lock, and preamble rule 25's private `CARGO_TARGET_DIR` cures
   uplift starvation, not the prebuild lock.
7. **writer's descriptor mirror `🔣️.json` was hand-edited**, not regenerated by its `describe` verb.
   The actions array may now be out of sort order for `textSelect` (it was `setEditorSelection`);
   V3b's `plugin-registry check` is the gate that would say so, and it was not re-run.

## 6. Files changed

| file | change |
|---|---|
| `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | 16-verb retained roster + publication contracts + proofs; **new** `build_artifact_store_one_item_preparation_factory`; extent rewritten (1 work item, explicit tile ceiling); reduce routed through `command.dispatch`; selection read from `InteractionState`; 13 classifications `BatchOnlyPendingRewrite` → `Migrated`; new `ANIMATE_PRESENTATION_MAXIMUM_TILES` / `…_ARTIFACT_MUTATION_MAXIMUM_BYTES` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📐️Canvas2dHost/🟦️.tsx` | **new** `isDecodedImage` (requires `naturalWidth > 0`, not just `complete`); both `drawImage` guards use it — fixes an uncaught `InvalidStateError` page error for ANY app naming an unserved image |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | verb id `setEditorSelection` → **`textSelect`** at 7 call sites (command row, `TOOL_JOB_IDS`, publication contract, proof roster, `command_from_action`, `ActionDefinition`, classification) + label/icon |
| `✏️s/🔌️plugins/✒️writer/🔣️.json` | descriptor mirror row for the renamed verb |
| `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/…/✏️editor/🧫️fixtures/🎬️writer-migration/🔣️.json` | fixture action id |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎬️set-active-example/🦀️.rs` | **new** — the F9 command payload + `emit()` |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs` | mounts that command module |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs` | F9's nine call sites + `reset_equation_document_effect` + `build_document_store_initialization_job` + 3 exhaustive-match arms |
| `.🧬semio/…/26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END/📜️f2-activate-chain.sh` | **new** — strictly sequential re-activation chain (never `dev <variant>`) |

Captures: `🗑️generated/f2-animate-console.txt`, `f2-writer-console.txt`, `f2-animate.png`,
`f2-writer.png`, `f2-animate-activate.txt`, `f2-writer-activate.txt`, `f2-activate-chain.txt`.

## 7. The two commands that finish this

Both against the serves already running (6051 animate, 6084 mathematical once its activation lands —
mathematical has **no serve yet**; start one with `📜️b1a-serve.sh mathematical`). Do **not**
re-activate animate: the queued chain does that.

```
cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END
SEMIO_F1_PLUGIN=animate SEMIO_F1_PORT=6051 SEMIO_F1_PREFIX=f2b SEMIO_F1_SECONDS=150 \
  SEMIO_PROBE_SETTLE_MS=60000 bun 🐍️f1-bar-probe.mjs
SEMIO_F1_PLUGIN=mathematical SEMIO_F1_PORT=6084 SEMIO_F1_PREFIX=f2 SEMIO_F1_SECONDS=150 \
  SEMIO_PROBE_SETTLE_MS=60000 bun 🐍️f1-bar-probe.mjs
```


---

## F3 — finish batch A (re-activation, live bar, sequence)

> **Bottom line: 4 of 6 batch-A plugins clear the whole five-clause bar live — 🏛️architect,
> 🎞️animate, ✒️writer, 🌿️vcs, each with ZERO console fault lines. 🎬️sequence clears 4 of 6 (its
> example now loads clean, which is the column it never had) and ➗️mathematical 3 of 6 (its archive
> closure is fixed; it still cannot stage a document verb's arguments).** Every row below was
> measured by this slice on a live serve; nothing is inherited without re-measurement.

Took over 2026-09-20 13:58 CEST. Machine at takeover: load 43.5, 72 GiB free on
`/System/Volumes/Data`, five peer wasm cargos running, fleet wasm mutex held by slice `s3`.

Serve state found by `curl` at takeover: **animate 6051 `200`**, **writer 6062 `200`**,
**vcs 6075 `200`**, architect 6090 **dead**, mathematical 6084 none, sequence 6077 none.
F2's detached chain was confirmed dead (its two cargos were killed by the coordinator at 13:57).

### F3 bar matrix (measured only; `—` = not measured by this slice)

| plugin | boot | example loads | dispatch mutates | undo | redo | no own console faults | BAR |
|---|---|---|---|---|---|---|---|
| 🏛️architect | **yes** | **yes** (0 faults) | **yes** `setAdjacencyKind` | **yes** | **yes** | **0** | ✅️ **PASS** (re-proven live) |
| 🎞️animate | **yes** | **yes** (0 faults) | **yes** `seedGrid` | **yes** | **yes** | **0** | ✅️ **PASS** |
| ✒️writer | **yes** | **yes** (0 faults) | **yes** canvas `apply` | **yes** | **yes** | **0** | ✅️ **PASS** (re-proven live) |
| 🌿️vcs | **yes** | **yes** (0 faults) | **yes** `incrementCounter` | **yes** | **yes** | **0** | ✅️ **PASS** (re-proven live) |
| ➗️mathematical | **yes** | **yes** (0 faults, was 2) | no — pane stages no `block` payload | — | — | 9 (all pane-arg staging) | ❌️ (3 of 6) |
| 🎬️sequence | **yes** | **yes** (0 faults) | **yes** `addStep` | no — shell reports `canUndo:false` | — | **0** on a staged verb (14 on the full sweep) | ❌️ (4 of 6) |

M7's `ws://127.0.0.1:*/bridge … ERR_CONNECTION_REFUSED` lines are the agent bridge, which is not
running in this playground; they are **not** counted against any plugin row.

### F3.1 architect — re-proven live, 6/6 (14:05 CEST)

F1's serve on 6090 was dead; this slice restarted **only the serve** (`📜️f3-serve.sh architect 6090`,
no activation — nothing of batch A touched architect's guest) and re-ran the bar probe.
`🗑️generated/f3-architect-console.txt`:

```
F3 architect {"ready":"architect","shellError":null,"loadsClean":true,"exampleRendered":true,
 "dispatched":true,"action":"setAdjacencyKind","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":0,"consoleLines":10,"bar":true}
```

Zero fault lines of any kind — the only batch-A row that clears the probe's own blanket
`faultLines === 0` clause.

### F3.2 sequence — F9 landed (the composed-child shape), tests green

F2 §4 specified this call site by call site; all five items are landed.

| item | where |
|---|---|
| **new** command module (`SetActiveExample` + `emit`, free of any `ArtifactView`) | `…/🎬️sequence/…/✳️any/✏️editor/🎮️commands/📚️example/🦀️.rs` |
| module mount | `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs` `commands::example` |
| `reset_sequence_document_effect` (`store::empty_document_spr`, **never** a minted `ArtifactEnvelope`) | `…/✏️editor/🦀️.rs` 🔖️Constants |
| `app_commands!` row, appended LAST (row order is the binary variant ordinal) | 🔖️Commands |
| **fourth** retained factory `SequenceRetainedExampleJobFactory` + `SequenceRetainedExampleWork`, `HostOnly` lane, 2 work units | new region 🧵️RetainedExampleRoutes |
| `SequenceExampleProofs` `bounded_first_step_tool_proofs!` + chained into `bounded_first_step_tool_proofs()` | 🧾️ProofCatalogs |
| `build_document_store_initialization_job` → `bounded_document_store_initialization_job` | `ArtifactEditor` impl |
| `example_route` in `build_tool_job` (admission, work, `try_new` work-item ceiling) | `build_tool_job` |
| `command_from_action` arm (`exampleId`/`example_id`/`id`/`value`, default `demo`) | `command_from_action` |
| manifest `ActionDefinition` + `Migrated` classification + `action_args` select | 🔖️Manifest |

Why a fourth factory and not a row on one of the three: sequence's existing factories route
document, **child** and window-config mutations through their work impls; this verb publishes on no
store at all (`HostOnly`) and emits a host-applied effect — the `🔱️trinity/🔌️jack` one-verb-factory
shape. Its extent is answered **without** touching the snapshot or the working scene: loading the
example is what makes a scene exist, so measuring it against one would refuse it at exactly the boot
moment the navbar dispatches it (F2 §3's mathematical lesson, and §1-A's extent-vs-`maximum_work_items`
lesson — the extent is 2 units and `try_new` is handed `SEQUENCE_RETAINED_EXAMPLE_MAXIMUM_UNITS`).

**B2c's gate re-run first, as F2 §4.5 required** — `bun ./📜️script.ts verify composed-child-refs`
(nx wrapper bypassed, preamble rule 16): `assets=214 mismatched=2 waived=2 breaches=0 … passed.`
Both waived rows are gis's declared durable-group constants; sequence's four assets are canonical,
so the example cannot land in the `InvalidReference` projection failure that cost writer its bar.

**Measured, not assumed:** `cargo check -p semio-s-artifact-sequence-sequence --lib` → finished, 5
pre-existing warnings (warnings are the proof the type-check actually expanded). Then
`cargo test -p semio-s-artifact-sequence-sequence --lib -- example` with a private
`CARGO_TARGET_DIR` (rule 25) → **9 passed, 0 failed**, four of them new:

- `committed_example_carries_the_genesis_content_child` — the composed-child load contract as a law:
  `genesis_sequence_child_pack` answers the `content` slot ONLY for the child id `default_snapshot()`
  mints, and the react shell sends the archive member-less, so a committed example whose content
  child id drifted from it would load as `InvalidReference`. Asserts the parsed asset's
  `content.child_id` equals the genesis one AND equals its own `target.artifact_id`, and that
  `genesis_sequence_child_pack` actually answers it.
- `set_active_example_emits_one_load_document_effect`, `a_foreign_example_id_is_an_empty_emit`,
  `set_active_example_is_declared_and_bridged`.

`--features component-app-assembly` does not exist on this crate (B1a measured the same for all six
batch-A crates); plain `-p` is the right gate here.

(runtime bar rows fill as the activations land)

### F3.3 the wasm activation queue (measured, not an excuse)

Every wasm32 activation of this slice runs through preamble rule 27's fleet mutex
(`📜️f3-activate-chain.sh` wraps EACH `activate-<variant>-react-dev` in
`📜️wasm-build-mutex.sh f3 -- …`; `📜️f3-activate-chain-sequence.sh` is the same for sequence).

| time | lock owner | waiters | this slice's chains |
|---|---|---|---|
| 13:58 | `s3` | — | animate chain queued 13:59 |
| 14:17 | `a3` | — | sequence chain queued 14:11 |
| 14:42 | `a3` (25 min) | **9** mutex waiters | both still queued (43 / 31 min) |

No peer process was killed and no cargo of this slice was started outside the mutex. The two
activation chains are detached (`nohup … & disown`) and write
`🗑️generated/f3-{animate,mathematical,sequence}-activate.txt`.

### F3.4 animate — F2's two source fixes are now proven at runtime

Re-activation `🗑️generated/f3-animate-activate.txt` (`exit=0`, 14:51:49 → 14:54:31, `component-dev`
1 m 22 s — the build dir was warm from F2's killed run), probed on F1's serve on **6051** (reused,
never re-served). `🗑️generated/f3-animate-console.txt`:

```
F3 animate {"ready":"animate","shellError":null,"loadsClean":true,"exampleRendered":true,
 "dispatched":true,"action":"seedGrid","undoWorks":true,"redoWorks":true,
 "exampleLoadFaults":0,"faultLines":0,"consoleLines":8,"bar":true}
```

Both of F2's §1 residual defects are closed **at runtime**, and the capture names them:

- the dispatched verb is now **`seedGrid`** — the exact verb F2 measured being refused with
  `retained command exceeds semantic work capacity`. The work-item extent fix (extent 1 per
  `BoundedArtifactCommandWork` step, fan-out bounded by `ANIMATE_PRESENTATION_MAXIMUM_TILES`) is
  what makes it dispatch, and the probe stops at the first verb that dispatches, so `seedGrid` is
  now first past the post instead of `addTile`.
- `faultLines` **6 → 0**: the five `drawImage … HTMLImageElement is in the 'broken' state`
  `pageerror`s are gone, which proves the framework-level `isDecodedImage` guard in
  `📐️Canvas2dHost/🟦️.tsx` (`naturalWidth > 0`, not just `complete`).

`loadsClean` also flipped **false → true**. animate clears all six columns.

### F3.5 sequence — measured live: boot + example load are CLEAN, one column open

Serve started by this slice on **6077** (`📜️f3-serve.sh sequence 6077`) after the activation
(`🗑️generated/f3-sequence-activate.txt`, `exit=0`, `@semio-tech/sequence-plugin:component-dev`
2 m 6 s). Three probe runs, all on the same serve:

| run | `SEMIO_F1_ACTIONS` | loadsClean | exampleLoadFaults | dispatched | undo | redo | faultLines |
|---|---|---|---|---|---|---|---|
| `f3` | (all rows) | false | **0** | **`addStep`** | false | true | 14 |
| `f3b` | `addStepDropped` | **true** | **0** | false | — | — | **0** |
| `f3c` | `addStep` | **true** | **0** | **`addStep`** | false | true | **0** |

**What that proves.** The F9 work is done and works: `setActiveExample` is no longer dropped as
undeclared, the composed-child whole-document load succeeds, and the boot+load path produces
**zero** console faults (`f3b`/`f3c`). This was one of the two bar columns sequence had NEVER had
measured (F2 §4's closing line), and it is now green. B2c's `composed-child-refs` gate plus the new
`committed_example_carries_the_genesis_content_child` law are why: the loaded example's content
child id is the id `genesis_sequence_child_pack` answers.

**The 14 fault lines of run `f3` are the probe sweeping the Actions pane with unstaged arguments**,
not a defect of this work: `setViewport requires a camera`, and six verbs
(`addStepToSlot`/`removeStep`/`moveStep`/`connectSteps`/`disconnectSteps`/`setStepParams`) refused
with `Sequence command does not match its exact retained route or payload envelope` — each one's
`sequence_retained_artifact_command_admitted` row demands a non-empty id the pane never staged.
B1a recorded this same class ("`setViewport`/`addStepToSlot` need args the Actions pane does not
stage"). Restricted to a verb the pane CAN stage, the run has zero fault lines.

**The one open column: undo.** `addStep` dispatches and appends an app row to the shell ledger
(`entriesBefore: 0 → entriesAfter: 1`, `actionIds: […, "addStep"]`, no fault), but in the SAME
receipt `signatureMoved: false` and the shell's undo control is **disabled** (`undoClicks:
["disabled"]`, `canUndo: false`, cursor `3 → 3`). A ledger row over an unchanged document with undo
unavailable is the *Fatal Mutation Outcome Phantom Edit* signature (memory), not a probe artefact —
it reproduces identically in the isolated `f3c` run.

Honest reading: this is **not proven** to be caused by the load path, and it is **not proven** to
pre-date it either. B1a measured `addStepDropped` mutating and undoing on the pre-F9 guest with a
DIFFERENT probe and a different verb, so there is no like-for-like baseline. The next diagnostic is
cheap and specific: dispatch `addStep` twice — once on a boot where `setActiveExample` never ran
(the pre-load store) and once after it — and read the journal's mutation outcome; if only the
post-load one is fatal, the child handle restored by `Effect::LoadDocument` is reaching
`diff_replace_content` without its local owner.

### F3.6 mathematical — the archive closure was `Incomplete`; root-caused and fixed

F2's F9 wiring was correct and reached dispatch, but the **first live probe** (`f3`,
`🗑️generated/f3-mathematical-console.txt`) showed the verb failing at the host:

```
input #1 setActiveExample refused: dispatch-failed — AppChannelClient.loadDocumentArchive(
  s.mathematical.equation@1/*#editor): {"code":"plugin.internal.document-archive-replacement
  .closure-rejected","message":"document archive replacement failed its closure leg: recursive
  ownership closure validation rejected the candidate (Incomplete)"}
```

`Incomplete` is requirement 2 of the composed-child contract (memory
*Composed Child Load Requirements*), and mathematical met **none** of it: `EquationSnapshot` declares
**three** `#[child(kind = "s.stdio.semio")]` slots (`notation`/`results`/`computed`) but the plugin
had no `genesis_child_pack` at all and no `type Members` on either surface, while the react shell's
`loadDocumentPair` sends `members: []`. F2's `bounded_document_store_initialization_job`
(requirement 3) was in place — it is simply the *next* gate after the closure one.

Landed (four files):

| file | change |
|---|---|
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs` | **new** `genesis_equation_child_pack` — the three slots through the crate's OWN converters (`equation_notation_from_graph` / `equation_results_from_graph` / `equation_computed_from_state`), guarded on `child_id == snapshot.<slot>.child_id`; plus `SemioMembers` on both `EquationApplication` trait bounds |
| `…/✳️any/✏️editor/🦀️.rs` | `type Members = SemioMembers` + `genesis_child_pack` |
| `…/✳️any/👁️viewer/🦀️.rs` | same, on the viewer (the contract is declared on BOTH surfaces) |
| `✏️s/🔌️plugins/➗️mathematical/🦀️.rs` | `dyn_enum_close!` variants carry the roster: `VcsArtifactApp<EditorApp<…>, SemioMembers>` |

A decoded snapshot carries no local owner, so `equation_scene` answers the empty scene for a loaded
archive — which is exactly the document `EquationSnapshot::default()` and the committed `🎬️demo`
asset describe (the asset's DSL holds only `equation=…` plus the three fixed child refs
`equation-text`/`equation-table`/`equation-value`), so genesis is a total function of what is on
disk.

Measured: `cargo check -p semio-s-plugin-mathematical --lib` green (it first failed with two
`E0277 From<VcsArtifactApp<…, SemioMembers>>` bounds — the roster must be named in the artifact
crate's `EquationApplication` trait too), and `cargo check -p semio-s-artifact-mathematical-equation
--all-targets` green (no test constructor needed `new_app_with_registry_and_members`).

Re-activated through the mutex and re-probed (`🗑️generated/f3b-mathematical-console.txt`):

```
F3B mathematical {…,"loadsClean":false,"exampleRendered":true,"dispatched":false,
 "exampleLoadFaults":0,"faultLines":9,"consoleLines":18,"bar":false}
```

**`exampleLoadFaults` 2 → 0 and both `setActiveExample` lines are gone**: the composed-child
whole-document load now succeeds live. What remains is a different, pre-existing family — the
Actions pane stages no `#[dsl(block)]` payload, so `setDocument`/`nodeGraphViewport`/`setPoints`
refuse with `requires a '<block>' block` (6 lines, B1a's own finding), and `setDirected`/
`nodeGraphEdit` now refuse with `equation-command-capacity` instead of
`equation-work-extent-overflow` — the change of message is itself evidence the scene owner now
exists, because that is the branch *after* the scene lookup. mathematical therefore clears boot,
example render and example load, and cannot clear dispatch/undo/redo until a verb is reachable with
staged arguments.

### F3.7 honest gaps

1. **sequence's undo is unproven, and its cause is not proven either.** The measured facts are in
   §F3.5: a ledger row with `canUndo: false` and an unmoved render signature. The one structural
   observation worth passing on: sequence is the ONLY batch-A app publishing on the
   `ArtifactToolPublicationLane::Child` lane (all ten of its document verbs do), and it is the only
   one whose edit produced a non-undoable row — the four that pass publish on `Artifact`/`Config`.
   That matches the memory *Retained Rows Must Be Point-Invertible*, but it is a hypothesis, not a
   measurement: no Child-lane app was measured undoing.
2. **mathematical cannot dispatch a document verb from the Actions pane at all** — every one of its
   six document verbs takes a `#[dsl(block)]` payload the pane stages as nothing. This is B1a's
   original finding, untouched by F2 or F3; it needs `action_args` forms (or a pane-side block
   editor), not another retained-lane change.
3. **`equation-command-capacity` on `setDirected`/`nodeGraphEdit` is new and unexplained.** It
   replaced `equation-work-extent-overflow` once the scene existed; the extent branch after the
   scene lookup now refuses. Not diagnosed.
4. **No descriptor mirror was hand-edited by this slice** (F2's gap 7 was `✒️writer`'s). Sequence's
   `🔣️.json` was regenerated by the activation's own `describe`/`session-sequence` step, not by hand.
5. **The five serves this slice used are still running** (architect 6090, animate 6051, writer 6062,
   vcs 6075 reused; mathematical 6084 and sequence 6077 started by this slice). No peer process was
   killed; every wasm32 activation went through the rule-27 mutex, and this slice queued at most one
   ticket at a time per variant.
6. `--features component-app-assembly` does not exist on any batch-A crate; `cargo check -p <crate>`
   (and `--all-targets` for the test build) is the gate that applies, with rule 25's private
   `CARGO_TARGET_DIR=…/target-f3` on every test/all-targets run.

### F3.8 files changed by F3

| file | change |
|---|---|
| `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📚️example/🦀️.rs` | **new** — `SetActiveExample` payload + `emit()` |
| `…/🎮️commands/📚️example/🧪️tests/🔬️unit/🦀️.rs` | **new** — 4 laws incl. the genesis-child-id law |
| `…/🎬️sequence/🗿️artifacts/🎬️sequence/🦀️.rs` | mounts `commands::example` |
| `…/🎬️sequence/…/✳️any/✏️editor/🦀️.rs` | `reset_sequence_document_effect`; `app_commands!` row; new 🧵️RetainedExampleRoutes region (work + factory + admission); `SequenceExampleProofs`; `build_document_store_initialization_job`; factory registration; `example_route` in `build_tool_job`; `command_from_action` arm; manifest action + classification + args |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🦀️.rs` | **new** `genesis_equation_child_pack`; `SemioMembers` in the `EquationApplication` bounds |
| `…/➗️equation/…/✳️any/✏️editor/🦀️.rs` · `…/👁️viewer/🦀️.rs` | `type Members` + `genesis_child_pack` |
| `✏️s/🔌️plugins/➗️mathematical/🦀️.rs` | `dyn_enum_close!` variants carry `SemioMembers` |
| ticket `📜️f3-activate-chain.sh` · `📜️f3-activate-chain2.sh` · `📜️f3-activate-chain-sequence.sh` · `📜️f3-serve.sh` | **new** — mutex-wrapped activation chains and the detached serve |

Captures: `🗑️generated/f3-{architect,animate,writer,vcs,sequence,mathematical}-console.txt`,
`f3b-{sequence,mathematical}-console.txt`, `f3c-sequence-console.txt`,
`f3-{animate,mathematical,sequence}-activate.txt`, `f3-activate-chain*.txt`, `f3-*-serve.txt`.
