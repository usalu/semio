# 📓️ Wave I2 — the ◻️2d integration: taking the crate green and coherent

Integrator I2. Paths are relative to `/Users/ueli/Documents/semio`.
`EDITOR2` = `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`,
`ANY2` = `…/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any`. Evidence: `TICKET/🗑️generated/I2/`.

---

## 0. Headline

| gate | before I2 | after I2 |
|---|---|---|
| `cargo check … --all-targets` (native) | **already green** (0 errors) | **green**, 0 errors |
| `cargo check … --target wasm32-wasip2` | not run today | **green**, 0 errors |
| `cargo test … --lib` (full 2d suite) | **603 passed / 268 failed** | **PLACEHOLDER_FINAL** |
| `bun publication-authority-audit` (all 3 owners) | **exit 1** | **exit 0**, `windowOwnershipCases=7` |
| python third-party oracle (shapely/networkx/jsonschema/jsonpatch) | **0 vectors discovered** (silent green) | **90 vectors, 6/6 scenarios, 1729 checks** |
| bun third-party oracle (graphology/jsonschema/fast-json-patch) | **0 vectors discovered** (silent green) | **90 vectors, 4/4 scenarios, 997 checks** |
| renderer `semio-framework-os-renderer-wgpu` | unknown | red at HEAD, **0 errors in the board-sync file** (§6) |

Five root causes account for **230 of the 268 reds**. Every one of them was a real fault, not a
test-expectation tweak.

---

## 1. `setActiveTool`/`setActiveUtility` were never generated tool ids (65 reds)

`EDITOR2/🦀️.rs` aliased `OpBinary::TOOL_JOB_IDS = PUZZLE2D_RETAINED_TOOL_IDS`, and that one list also
keys `Puzzle2dRetainedCommandJobFactory`. `validate_tool_job_rows`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:12837`) forms `expected` as
`TOOL_JOB_IDS ∩ migrated`, so 2C's `Puzzle2dHostConfigurationProofs` block was rejected with
`interactive-job.catalog-authority` and **every registry-backed 2d law died in `app_with_registry()`**.

Adding the two ids to the retained list would have been wrong the other way round: a registered factory
makes the GENERIC proof's `registered.is_none()` false. 3d keeps two separate lists
(`🧊️3d/…/✏️editor/🦀️.rs:2658` vs `:3744`); 2d now does the same without duplicating 54 strings:

- `EDITOR2/🦀️.rs:1984` `PUZZLE2D_HOST_CONFIGURATION_TOOL_IDS` — the two framework-injected verbs.
- `EDITOR2/🦀️.rs:1992` `PUZZLE2D_TOOL_JOB_IDS` — a const-eval concat of both lists.
- `EDITOR2/🦀️.rs:1743` `const TOOL_JOB_IDS = &PUZZLE2D_TOOL_JOB_IDS`.

The retained-jobs fixture's `toolIds` stays the FACTORY list and must not gain the two verbs (it is
checked against `PUZZLE2D_RETAINED_TOOL_IDS`, `◻️2d/🦀️.rs:25`). 5d already carries the identical shape
(`🖐️5d/…/✏️editor/🦀️.rs:4134`), so only 2d was missing it.

## 2. The whole 2d mutation fixture corpus was in the pre-09-01 float spelling (168 reds)

`committed_json_is_canonical` / `committed_diff_is_canonical` / `produces_committed_diff` were red on
**every one of the 33 mutation leaves**, including leaves nobody has touched in weeks. The committed
JSON spells a whole `f64` as `0`; `dsl::json`'s writer has spelled it `0.0` since ticket
`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`
(`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:1082` `write_float`, whose own doc states the rule).

Measured, not assumed — the drift is 2d-only:

| artifact | snapshot fixture files | whole floats with `.0` | bare ints |
|---|---|---|---|
| 🧊️3d | 70 | 680 | 0 |
| 🖐️5d | 98 | 1370 | 0 |
| ◻️2d **before** | 180 | 0 | 13 751 |
| ◻️2d **after** | 180 | 13 751 | 0 |

Re-canonicalised **239 committed leaves** (before/after snapshots, mutation payloads, diffs) by decoding
each through its own typed decoder (`Puzzle2dSnapshot` / `Puzzle2dMutation` / `Puzzle2dDiff`) and
re-encoding with `dsl::json::to_json_string` — so nothing but the number spelling could change — and
refreshed the 239 matching sha256 rows in `ANY2/🧫️fixtures/🧬️mutations/🔣️.json` (450 entries, 0
uncatalogued, 0 missing). The scaffolding regenerator was removed again after the run
(`🗑️generated/I2/recanonicalise.txt`). This also closes 2F §4.4 and the 16 red 2F leaf tests 2H flagged.

## 3. Example documents: 3d data in 2d, and a `#[dsl(table)]` that cannot be absent (9 reds)

Two faults, both shared with 5d, both written up for I5 in `TICKET/📌️important.md`.

**3a. The example modules had been switched off the DSL.** `document_json()` in both 2d example modules
had been rewritten from the committed DSL projection to `include_str!("…/📄️document.json")`, and the
`📄️document.json` files dropped next to them held the **3d** forest/tower documents. Those files are
untracked and have never been committed (`git log --all -- "*📄️document.json"` is empty), so nothing was
recoverable from git. I5 found the same in 5d. Reverted both modules to the DSL projection, deleted the
two bogus files (copies in `🗑️generated/I2/salvaged-example-json/`) and removed the three tests the same
edit had added — `◻️2d/…/📚️examples/🌲️concrete-forest/{🦀️.rs,🧪️tests/🧩️example/🦀️.rs}`,
`…/🏗️nakagin-capsule-tower/🦀️.rs` and `ANY2/🧬️schema/📸️snapshot/📝️text/🧪️tests/🔬️unit/🦀️.rs` are now
byte-identical to HEAD again.

**3b. `target_regions` is a `#[dsl(table)]`, and a table is never optional.** `FieldKind::VecTable`'s
`from_value` refuses `Absent` (`🗣️dsl/✨️derive/🦀️.rs:1044`), so every shipped example failed with
`expected List, found Absent at 1:1` and poisoned the example `LazyLock` for the whole process.
**The derive is correct and was not changed**: `FieldSpec.optional` exists but the derive sets it only
for `Option<T>`, and `nodes`/`edges` rely on that strictness. 3d has always answered this by spelling
the empty table (`🧊️3d/…/🌲️forest/🗣️.dsl.semio:48`). Both 2d examples now carry

```
target-regions [id:TEXT x:NUM y:NUM width:NUM height:NUM label:TEXT hidden:BOOL locked:BOOL] {
}
```

(one `key:SHAPE` per field in declaration order; parsing is key-driven so it goes at the end of the file).

## 4. `edit history insertion requires its exact mutation retirement factory` (4 reds)

2d had no `puzzle2d_store` constructor: a bare `ArtifactStore::new` carries no member-store retirement
authority, so the very first `ArtifactCommand::Apply` fails closed. 3d has owned the pair since its own
migration. Ported verbatim — `ANY2/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:31` `puzzle2d_store` +
`:54` `close_puzzle2d_store` (`PUZZLE2D_STORE_CLOSE_TURNS`/`_BYTES`) — and switched the four standalone
store tests onto it, each now retiring through `close_puzzle2d_store`:
`ANY2/🧬️schema/📸️snapshot/{💾️binary,📝️text}/🧪️tests/🔬️unit/🦀️.rs`,
`ANY2/🧬️schema/🧬️mutations/💾️binary/🧪️tests/🔬️unit/🦀️.rs`, `EDITOR2/🧪️tests/🔬️unit/🦀️.rs:415`.
All four green.

## 5. `InvocationResult.mutations` is not the carrier for a migrated verb (≈14 reds)

`dispatch_typed` queues the operation; the edits are committed **inside** it and reported as command-log
upserts on `TypedOperationCompletion.history_patch`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29572`). The 2d harness's `settle` dropped that
patch on the floor and every law then asserted `result.mutations`, which is empty by construction — so
**six positive laws were red and nineteen negative ones were passing vacuously** (`setCamera must not
emit document operations` could never fail). 3d never asserts a non-empty `mutations`, which is why only
2d shows it.

- `EDITOR2/🧪️tests/🔬️unit/🦀️.rs` `settle` now adopts and merges `completion.history_patch`.
- New `committed_edits(result)` (`:140`) counts upserts whose `kind == "mutation"` — a `View`/`Shell`
  verb logs a command row too, and counting those would red every "must not mutate" law for the wrong
  reason.
- All 25 `…​.mutations` assertions in `EDITOR2/🧪️tests/🔬️unit/🦀️.rs` and
  `EDITOR2/🧪️tests/🔬️clipboard/🦀️.rs` converted, so the negatives are now real assertions.
- 2H's note on `board_region_events_commit_one_edit_each_through_the_target_region_verbs` (\"the harness
  reports zero mutation rows for `addTargetRegion` too\") is resolved by this: the region differ was
  never at fault (`🧬️mutations/🦀️.rs:269-294` emits all seven region mutations), the harness was.

---

## 6. Coordinator questions, answered

**Descriptor regeneration (brief §4).** `🔣️.json` + `🛂️.descriptor.semio` at the plugin owner root are
emitted by `describePluginComponent`, registered as the `describe` command of the RUST package:

```
cd ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust && bun ./📜️script.ts describe
```

It builds the `wasm32-wasip2` component first, so it is yours, not mine. **It is needed**: the committed
`✏️s/🔌️plugins/🧩️puzzle/🔣️.json` still declares the five pre-rename action ids (`brushOpenSlot`,
`brushCommitSlot`, `brushCancelSlot`, `brushSetCandidateIndex`, `brushCycleCandidate`).

**`semio-framework-os-renderer-wgpu` (coordinator note).** Red at HEAD with **11 errors, none of them in
the board-sync file** (`🗑️generated/I2/check-wgpu.txt`):
- 6 × E0063 in `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `ViewModel` gained `tree_windows` and
  `tree_viewport_rows`.
- 5 × E0560/E0609 in `🧑‍🎨engine/🧊️renderer/🦀️.rs` — `JobReplayRoute` / `MountedReplayRouteSeed` lost
  their `document` field.

`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — 2C's/2E's/2H's board sync — is compiled in the same unit and
emits only `unnecessary qualification` warnings (lines 2481-2496, right beside the sync block at
2391-2400) and **zero errors**. 2C's §6.2 "highest-risk item" is therefore clear; the two peers' files
are not mine.

**Target-region mutations reaching the document (coordinator note).** They always did. `addTargetRegion`
→ `puzzle2d_paint_target_region` → `puzzle2d_push_target_region` (`EDITOR2/🦀️.rs:1072`) writes the
fixture, `puzzle2d_snapshot_mutations` emits `create/move/resize/label/hidden/locked-target-region`
(`ANY2/🧬️schema/🧬️mutations/🦀️.rs:269`), and 2H's own board-event law passes on the document. What was
missing was §5's harness carrier, now fixed.

## 7. Both third-party oracle adapters were silent-green — repaired and RUN

2F was right that `vectors()` discovered zero vectors, but the stale constants were two, not one, and
the committed corpus lives in a SIBLING tree from the leaf schemas:

| adapter | was | now |
|---|---|---|
| `ANY2/🧪️tests/🕸️third-party-puzzle-2d-1/🐍️.py:78,82` | `SCENARIOS_DIR = "🧪️tests"`, `LEAF_SCHEMA = "🧬️schema/🔣️.json"` | scenarios are the leaf's own children; `LEAF_SCHEMA_ROOT = ("..","..","🧬️schema","🧬️mutations")` |
| `ANY2/🧪️tests/🌐️third-party-puzzle-2d-1/🟦️.ts:44` | same two | same fix |
| both `MEMBERS` | `(schema, camera, nodes, edges, meta)` | `+ targetRegions` |

2F's fear that repairing discovery "would turn four erroring handlers into four failing ones" did **not**
materialise — the 24 documented payload rejections were an artefact of the schema never being found.

```
.venv/bin/python3 TICKET/🐍️I2-third-party-oracle-probe.py
  discovered 90 committed vectors (15 of them target-region) over 33 leaves
  PASS graph-cascade 130 · kind-compatibility 180 · geometry-transforms 609 ·
       payload-schemas 270 · diff-reproduction 507 · region-containment 33     → 6/6

bun TICKET/🌐️I2-third-party-oracle-probe.ts
  PASS graph-cascade 130 · kind-compatibility 180 · payload-schemas 270 ·
       diff-reproduction 417                                                    → 4/4
```

Both probes are left in the ticket folder (not the scratchpad) and print the vector count each scenario
saw, so a green that checked nothing is impossible from here on.

## 8. `publication-authority-audit` → exit 0

`ownerOracle`'s 2d branch (`🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts:289`) still pinned
`(addressed <= PUZZLE2D_SELECTION_BATCH_LIMIT).then_some(addressed.max(1))`, which 2D replaced with an
early return plus the proximity budget. Re-pinned to the new shape — both the refusal
(`if addressed > PUZZLE2D_SELECTION_BATCH_LIMIT { return None }`) and
`Some(addressed.saturating_add(connects).max(1))`, so the audit still holds the extent honest.
`bun ./📜️script.ts publication-authority-audit` → **exit 0** for `Puzzle2dPlayApp`, `Puzzle3dPlayApp`
and `Puzzle5dPlayApp`; `validateWindowOwnershipSchemas` passes all 7 cases
(`🗑️generated/I2/publication-audit.txt`).

## 9. Other product/law fixes

| what | file:line |
|---|---|
| `setProximityRadius` had a verb, a window-config field, a lane and labels but **no stepper** — 2C handed it to 2D and 2D handed it back. Landed at `puzzle2d-play-settings.proximity-radius` (the id 2G's probe looks for). | `EDITOR2/📌️panels/⚙️settings/🦀️.rs:76` |
| Label stamping was missing at the **paste** creation site (2C's hand-off). The other two mint paths (`add_node_to_host_snapshot`, `apply_brush_place_payload`) already stamped; `proximityConnect` mints edges only. | `EDITOR2/🦀️.rs` `puzzle2d_paste_operations_on` |
| `FillRunReason::OutsideTargetRegion` existed in Rust but not in its schema twin, so `fill_tool_declares_the_schema_tool_run_definition` was red (12 vs 11). | `ANY2/🧬️schema/🔣️.json` `$defs.Puzzle2dFillRun.x-semio-toolRun.reasons` |
| The retained-jobs fixture's `toolIds` held the right 54 ids in the WRONG ORDER (first divergence at index 36); the owned oracle compares ordered lists. | `ANY2/🧫️fixtures/🗄️retained-jobs/🔣️.json` |
| `utility_registry_declares_utilities` never learned 2F's `areaBrush`. | `EDITOR2/🧪️tests/🔬️unit/🦀️.rs:652,655` |
| `dispatch_registers_semantic_descriptors` still pinned 26 mutation kinds (now 33). | `ANY2/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs:141` |
| The neutral-config member count laws still pinned 3 (2C added `contactTolerance` + `brushPlacementOverlapBudget`). | `EDITOR2/🎚️config/🧪️tests/🔬️unit/🦀️.rs:8`, `EDITOR2/🪟️window/🧪️tests/🔬️unit/🦀️.rs:21` |
| `a_node_display_label_…` asserted the raw NODE id for a kind the catalogue does not name; the ported 3d contract (`puzzle3d_object_display_label`) answers the KIND id, and only a kindless node falls back to its own id. Law corrected and widened with the kindless case. | `EDITOR2/🧪️tests/🔬️unit/🦀️.rs:1152` |

## 10. Harness fixes that unmasked real behaviour

1. **`pump_fill_session` treated back-pressure as a fault.** `WorkerPool::try_submit` is a *try*: it
   answers `Contended` whenever the live worker thread holds its own lane queue, and the framework's own
   `submit_retained_timer_job` reschedules on exactly `Contended | Saturated`. The 2d helper panicked on
   the first refusal, so **all ten mounted fill laws were unrunnable** (`mounted fill pump fault`, with
   the fault variant swallowed by `Err(_)`). Now it retries under the existing
   `FILL_TEST_PUMP_LIMIT` and names the exact variant for everything that is a real fault.
   `EDITOR2/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:66`.
2. **A failing fill law aborted the whole process.** `panic!`ing while holding a `StepOutcome` trips
   `RetainedJobPayload`'s non-unwinding `Drop` (`🧵️job/🦀️.rs:662`) and takes the suite with it (`SIGABRT`),
   which is why the 724-test suite could not be run at all. The three terminal arms now drain the outcome
   to terminal-empty before panicking, so a red is an ordinary red.
   `EDITOR2/⚙️engine/🖌️brush/🧪️tests/🔬️unit/🦀️.rs:198,983`.

## 11. Conformance sweep (AGENTS.md)

- **Legacy verb ids.** `brushOpenSlot` / `brushCommitSlot` / `brushCancelSlot` /
  `brushSetCandidateIndex` / `brushCycleCandidate`: **zero** as app verb ids in Rust/TS/JSON/fixtures.
  The remaining hits are two other namespaces and are correct: the wasm **session methods**
  (`EDITOR2/🌉️wasm/🦀️.rs:395-416` + their `Board2dWasmSession` declarations and `Board2dHost` callers —
  2B's local engine-mirroring channel, never an action id), and the **stale generated descriptor**
  (`✏️s/🔌️plugins/🧩️puzzle/🔣️.json`, §6).
- **`[DEBUG]` leftovers.** One was 2A's, in the new ledger-ceiling law — removed (the refusal string now
  carries the index). One is far worse and **not a 2d file**: a peer left
  `eprintln!("[DEBUG] maintenance idle probe …")` in the framework's `maintenance_step` hot path
  (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29249`, introduced today 12:02), which spams
  every app's test output and would ship. **I removed it** — it is a bare `eprintln!` with no semantics —
  and flag it here for its owner. The two remaining 2d `[DEBUG]` lines are witness prints from 09-09,
  outside this ticket.
- **Comments inside definitions.** One, 2F's, inside the `Puzzle2dSnapshot` struct body — moved into the
  type docstring (`ANY2/🧬️schema/📸️snapshot/🦀️.rs`).
- **EN+DE.** Every label this integration touched already had both (`labels.proximity_radius`,
  `fill_reason_outside_target_region`); the terminology table is compile-checked.
- **Docstrings** on everything added here start with an emoji.

---

## 12. Commands run — verdicts

| # | command | verdict | log |
|---|---|---|---|
| 1 | `CARGO_INCREMENTAL=0 cargo check -p semio-s-artifact-puzzle-2d --features component-app-assembly --all-targets --message-format=short` | **exit 0**, `Finished dev profile … in 46.87s`, **0 errors**, 5 lib + 26 lib-test warnings | `check-native-alltargets-1.txt` |
| 2 | the same with `--target wasm32-wasip2` | **PLACEHOLDER_WASM** | `check-wasm.txt` |
| 3 | `cargo test … --lib -- --skip ingest_operations_is_idempotent` (baseline) | 603 passed / 268 failed | `test-baseline-1.txt` |
| 4 | the same, final | **PLACEHOLDER_FINAL** | `test-8.txt` |
| 5 | `cargo check -p semio-framework-os-renderer-wgpu --message-format=short` | 11 errors, **0 in the board-sync file** (§6) | `check-wgpu.txt` |
| 6 | `bun ./📜️script.ts publication-authority-audit` (all owners) | **exit 0**, `windowOwnershipCases=7` | `publication-audit.txt` |
| 7 | `.venv/bin/python3 TICKET/🐍️I2-third-party-oracle-probe.py` | **exit 0**, 90 vectors, 6/6 scenarios | `oracle-python.txt` |
| 8 | `bun TICKET/🌐️I2-third-party-oracle-probe.ts` | **exit 0**, 90 vectors, 4/4 scenarios | `oracle-bun.txt` |
| 9 | renderer react `bun ./📜️script.ts test long -t "interactionHover lane\|LAST hover row\|vortex-domain granularity\|handle-suggestions popup\|refuses politely\|rotate commit\|transformPreview\|parseBoard2dTransformFlags\|board2dStatusJson\|areaBrush"` | **7 passed**, 0 failed | — |
| 10 | the same with `-t "board2d\|Board2d\|board-2d\|puzzle2d"` | **5 passed**, 0 failed, 3 files | — |

Not run by me, by the rules: `nx`, activate/serve, the Playwright battery, `describe` (§6).

---

## PLACEHOLDER_REDS

---

## 14. Hand-offs

- **Coordinator** — run `bun ./📜️script.ts describe` in `🧩️puzzle/📦️packages/🦀️rust` (§6); the committed
  descriptor still names five renamed verbs.
- **Coordinator** — the six puzzle example `📄️document.json` files are `include_str!`ed and **untracked**;
  a fresh clone cannot build the puzzle crates. Worth its own ticket (also in `📌️important.md`).
- **Coordinator / framework** — the `[DEBUG] maintenance idle probe` eprintln I removed from
  `🔌️plugin/🦀️.rs:29249` was a peer's; if they still need it, it belongs behind their own switch, not in
  `maintenance_step`.
- **I5 (5d)** — `📌️important.md` carries the 2d half of both shared faults with the exact remediation, and
  the `setActiveTool` finding (5d already has the right shape).
- **2G / probe** — `puzzle2d-play-settings.proximity-radius` now exists; `areaBrush` is the third utility
  in the overview bar; the example documents are back to real 2d boards.
- **Latent hazard, verified correct today, not changed:** `BoardHost::set_selection_options` in
  `♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs:4352` takes `(…, nodes, edges, handles)` while
  `🎲️board/🦀️.rs:1106` takes `(…, nodes, handles, edges)`. I checked all five live call sites
  (`EDITOR2/🦀️.rs:1528`, `EDITOR2/🎮️commands/☑️set-selectable-kind/🦀️.rs:20`, `EDITOR2/🌉️wasm/🦀️.rs:303`,
  `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:2391-2393`, and the engine test at `:844`) — **every one matches
  the normal port's order**. Aligning the two signatures means flipping five all-`bool` argument lists
  the compiler cannot check, in a file slice 2H is still editing, for no behavioural gain today; it
  belongs in a quiet tree.
