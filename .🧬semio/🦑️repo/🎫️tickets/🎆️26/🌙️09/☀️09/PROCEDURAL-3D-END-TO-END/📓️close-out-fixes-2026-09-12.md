# Close-out fixes — 2026-09-12

Works through §5 of `📓️audit-gates-status-2026-09-12.md`. Every command below was **executed**, in the
foreground, under `NX_DAEMON=false` / `RUST_MIN_STACK=33554432`, shared cargo build dir, no
`CARGO_TARGET_DIR` override. Raw logs: `🗑️generated/close-out/*.txt` (delete when the ticket closes;
this report is the durable artifact). No git-modifying command was run; no ticket state was changed.

---

## 0. Headline

| gate | before (audit, 21:40) | after (this lane) |
|---|---|---|
| generation3d `--lib` | **does not compile** (`E0255` + `E0433`) | **compiles**; `379 passed; 6 failed` — all 6 peer-owned |
| generation3d `--test example-geometry` | 17/17 green | **17/17 green** (re-confirmed) |
| react engine `test long` | `998 passed / 11 failed`, 3 files failed | **`1009 passed (1009)`, `30 passed (30)`, exit 0** — three consecutive runs |
| `@semio-tech/procedural-plugin:test` | 14 passed / 2 failed / 3 never-run | **16 passed / 1 failed** (17/19 run) — the 1 is generation2d, out of scope |
| `verify publication-retirement-authority` | RED, `E0624` ×3 | **GREEN, exit 0** (`2 passed`) — the peer landed their own fix |

---

## 1. generation3d `--lib` compiles again

### 1.1 The collision

Two lanes' test modules were merged into one file without their imports being re-pointed. `git status`
shows the shape: `🧪️tests/🔬️test-support/🦀️.rs` and `🧪️tests/🔬️testkit/🦀️.rs` are **`D`** (deleted) and
their contents now live at the top of `🧪️tests/🔬️unit/🦀️.rs` as `pub(crate) mod context` (line 1) and
`pub(crate) mod serial_execution` (line 346). Both files were last written 12:03:34 and had not moved
for 85 minutes when this lane picked them up, so the rename had settled — it was simply unfinished.

Two errors, both resolved by naming, not by suffixing:

| error | file | fix |
|---|---|---|
| `error[E0255]: the name 'context' is defined multiple times` | `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:362` | the line re-imported the module it already contains: `use crate::editor::generation3d::unit_tests::context::{self, app, …}`. `self` binds `context` a second time in the same type namespace. Now `use self::context::{app, app_with_registry, drain_flow_eval_ticks, drain_flow_eval_ticks_with_view, preview_views};` — the sibling module is reached as a sibling, which also drops the `unused_qualifications` warning the long path carried. Every one of the 17 `context::…` call sites below it still resolves to the same module. |
| `error[E0433]: cannot find 'tests' in 'super'` | `…/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs:155` | the helper's module is declared `pub(crate) mod unit_tests` (`✏️editor/🦀️.rs:2528`), never `tests`. Now `super::unit_tests::node_eval_status(…)`, which is where `pub(crate) fn node_eval_status` actually lives (`🔬️unit/🦀️.rs:1629`). |

No `_2` suffix, no module renamed, both test sets kept whole.

### 1.2 Compile proof

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib --no-run
```
→ `EXIT=0`, `warning: semio-s-artifact-procedural-generation3d (lib test) generated 143 warnings`,
`Finished 'test' profile [unoptimized] target(s) in 2m 46s` (`🗑️generated/close-out/02-lib-compile-after.txt`).
The 143 warnings are the proof the expansion actually completed rather than aborting early.

### 1.3 The run, and every red attributed

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1
```
→ **`test result: FAILED. 379 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 129.36s`**
(`20-lib-test-final.txt`; an earlier identical run at 13:32 gave the same 379/6, `03-lib-test-run1.txt`).

| # | failing test | panic site / fault | owner |
|---|---|---|---|
| 1 | `commands::add_generation::tests::add_generation_records_an_undoable_generation_operation` | `🧰️framework/…/🔌️plugin/🦀️.rs:6871` — `assertion left == right failed: undo did not revert to the expected snapshot` (left 1, right 0) | **peer** |
| 2 | `component::unit_tests::undo_redo_round_trips_flow_graph_edits` | same line, same message (left 8, right 7) | **peer** |
| 3 | `component::unit_tests::two_instances_converge_disjoint_widget_moves` | `🧰️framework/…/🔌️plugin/🦀️.rs:6848` — `attach b: module.vcs … "remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized"` | **peer** |
| 4 | `component::unit_tests::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | `"Generation3d preview operation did not finish"` (30 s retire deadline) | **peer** |
| 5 | `component::unit_tests::refresh_pending_effects_arms_flow_eval_tick_chain` | `flowEvalTick: app.message "registered fixture typed operation did not retire within 30 seconds"` | **peer** |
| 6 | `component::unit_tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `"P3 authority refresh immediately before production maintenance: generation3d-publication.contended"` | **peer** |

**Evidence for the attribution.** Five of the six are byte-for-byte the set
`📓️viewer-status-parity-2026-09-12.md` §4.5 recorded at **380 passed / 5 failed** and already attributed
to peer lanes — same tests, same three panic sites. The sixth, #5, is the *same class* as #4 (the 30 s
retire deadline), and #6's message changed from that report's `"P3 production envelope load did not reach
terminal"` to `"…contended"` — both are the publication guard, not a new code path.

All six land inside one file that a peer is rewriting right now:

```
MM 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs   mtime=2026-09-12 14:22:49   last-commit=521b618cee 10:48
 M 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs      mtime=2026-09-12 14:18:20   last-commit=f39d4b0db3 09-10
```

`git diff` on `🔌️plugin/🦀️.rs` is **347 insertions / 75 deletions, uncommitted**, and it is precisely a
retirement/replacement refactor: it adds `retire_artifact_envelope_load_exact`, six new
`drive_store_replacement_jobs(1, …)` drive points, a `pub(crate) fn test_drive_store_replacement_jobs`
accessor, and a `[DEBUG] recursive replacement diagnostic: publication guard rejected closing=… retirement-admitted=…`
line. That is the same guard that answers `generation3d-publication.contended` in #6 and the same drive
loop whose deadline #4 and #5 time out on. `🏪️store/🦀️.rs` carries an uncommitted snapshot-read-lease
rewrite plus a `🧪️testkit` → `🧫️fixtures` module move.

These are **not** flakes under load — re-running the three deadline-class tests alone still gave
`0 passed; 3 failed` in 90.8 s (`04-lib-retire-class-retry.txt`). They are deterministic against the
peer's in-flight tree, and none of them asserts in a generation3d file. **Left to that lane.**

Net versus the last landed measurement: 380/5 → 379/6, i.e. one test crossed from green to red inside
the peer's retire-deadline class while the tree was mid-refactor. Nothing in this ticket's lanes regressed.

### 1.4 example-geometry

```
RUST_MIN_STACK=33554432 cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry
```
→ **`test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 63.85s`**,
`EXIT=0` (`11-example-geometry.txt`). Matches the landed baseline.

---

## 2. React renderer engine — `1009 passed (1009)`

### 2.1 The target moved

A live peer taxonomy migration relocated the target during this session:
`…/🧑‍🎨engine/📦️packages/🟦️typescript/🎯️targets/⚛️react` → **`…/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript`**
(the old path no longer exists; `git status` shows the whole set as `R` renames). The §5 command in the
audit must be re-spelled for the new layout:

```
cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long
```

### 2.2 Result

| run | log | verbatim |
|---|---|---|
| verification of the fixes | `07-react-engine-long-verify.txt` | `Test Files  30 passed (30)` / `Tests  1009 passed (1009)`, `EXIT=0` |
| final, after the flake fix | `22-react-engine-final-1.txt` | `Test Files  30 passed (30)` / `Tests  1009 passed (1009)`, `EXIT=0` |
| final, repeat | `22-react-engine-final-2.txt` | `Test Files  30 passed (30)` / `Tests  1009 passed (1009)`, `EXIT=0` |

**Remaining reds: none.** The corpus is fully green across three consecutive runs.

### 2.3 What the 11 audit reds became

| audit red | disposition |
|---|---|
| 6 × `extension invocation completion ownership > classifies/dispatches …` | the `signal` field is now asserted: `…toHaveBeenCalledExactlyOnceWith(capability, requestJson, { originInstanceId: fixture.instanceId, signal: expect.any(AbortSignal) })` at `🔬️engine-contract/🟦️.ts:1287` and `:1304`, matching `🏛️ShellHost/🟦️.tsx:1719`'s `invoke.call(handle, capability, requestJson, { originInstanceId: instanceId, signal: cancellation.signal })`. `originInstanceId` is still pinned exactly; nothing was relaxed to `expect.anything()`. |
| `buildNoteShellCommandAction … targeting the given controller` | **test was stale, implementation right.** The law now expects `inverseCommandId` (always) and `inverseArgs` (only with `detail`). `🛠️ShellHelpers/🟦️.tsx:337` is committed at `46c3cb9de0` and untouched; `🔌️plugin/🦀️.rs:23521` reads `inverseCommandId` when pushing the reserved note onto the undo stack, and `🔌️plugin/🧫️fixtures/reserved-undo-browser-note.json` already pins it. |
| `binds two instances of one body to distinct surfaces …` | **test was stale.** `windowHostContextBindings` appends the leftover default `window` alias (`🔌️PluginRuntime/🟦️.tsx:1615,1657`, committed `46c3cb9de0`); the law predated it. The fourth expected pair `[DEFAULT_LEFTOVER_WINDOW_SURFACE, "canvas-body"]` and its packed projection row were added, the "panel carries no windowId" check moved to `views[3]`, and the pair is asserted against the exported constant rather than the literal `"window"` so the law tracks the contract. |
| `readAppDocumentPack() … null when the reply carries no document frame` | **test was stale, and was under-asserting.** The law now pins `ops` as well, with a **non-empty** value (`"(extrude (polygon 6))"`) so it proves the text crosses. `ops` is load-bearing: `🏛️ShellHost/🟦️.tsx:4560-4568` feeds it to `documentSourcesFromPack` → `resolveDocumentOperatorKinds` and treats a missing one as `{ status: "unresolved", reason: "document-ops-missing" }`. |
| `window fault discriminators > carries a distinct English and German label …` | **peer-caused, fixed anyway — flagged.** The peer's migration sweep rewrote this `?raw` import once but landed it on what is now a 182-byte re-export shim; the real 516 KB bundle moved to the target root. A `?raw` read of a three-line shim can never satisfy the law. One-token correction: `🖱️ui/🎯️targets/⚛️react/🟦️.tsx?raw`. This is a completed-migration straggler sitting inside this ticket's own corpus, not contested work. |
| `footer credits render the funding/partner logos` (`cn is not a function`) | no longer present in this corpus. |

### 2.4 One further red, not in the audit — this ticket's own flake, fixed

`node-graph surface attachment in a hidden tab > replaces a canvas that can no longer give a 2D context
and re-attaches its surface to the successor` (`🔬️engine-contract/🟦️.ts:10452`) failed intermittently
with `expected [300, 150] to equal [966, 836]` — the jsdom default canvas size. It passed in isolation
and in most full runs. This is this ticket's `node-graph-paint-restart` lane
(`📓️node-graph-paint-restart-2026-09-12.md` §2.3), so it is ours to fix: the assertion read
`successor.width/height` in the same tick as the `waitFor(attaches ≥ 2)` that preceded it, before the
successor's own sizing effect had necessarily run. Folded into a `waitFor`, with a comment saying why.
The assertion itself is unchanged — still exactly `[966, 836]`.

---

## 3. Close-ladder law — the ratio replaced

### 3.1 Why the 8× ratio had to go, and why an additive spread could not replace it

`generation3d_close_cost_is_independent_of_the_retained_session`
(`✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs`) asserted `high ≤ low × growth` with
`growth = 8`. The close-ladder lane's own report is explicit about why that cannot hold
(`📓️close-ladder-budget-2026-09-12.md` §2.3): *the turn count is not a step count — it is how many empty
reactor polls fit while the close pump grinds on the maintenance worker lane and loses
`cell.instance.try_lock()` to the polling thread. It is a latency proxy, and it is noisy.*

This lane's first attempt was the additive form the audit suggested — `high − low ≤ K`, `K = 192`. It
**passed serially and then failed under the gate's own nextest parallelism**, which is the measurement
that matters:

| run | costs `[cold, warm, long]` | ratio | additive spread | dilution |
|---|---|---|---|---|
| at landing (09-12) | `[88, 152, 265]` | 3.0× | 177 | 3.06 |
| idle, `--test-threads=1` | `[2, 39, 84]` | 42× | 82 | 2.48 |
| nextest `fundamental` | `[38, 112, 233]` | 6.1× | **195 — broke K=192** | 2.56 |
| idle, second | `[102, 165, 220]` | 2.2× | 118 | 4.00 |
| nextest, second | `[15, 87, 238]` | 15.9× | 223 | **1.95** |
| nextest, third | `[10, 61, 143]` | 14.3× | 133 | 2.27 |
| nextest, fourth | `[29, 86, 164]` | 5.7× | 135 | 2.80 |
| idle, third | `[0, 63, 158]` | **∞ — cold reached `Retired` in 0 turns** | 158 | 2.13 |
| idle, loaded | `[108, 266, 442]` | 4.1× | 334 | 3.21 |
| nextest, fifth | `[14, 71, 153]` | 10.9× | 139 | 2.47 |

Both the ratio (2.2× – ∞) and the spread (82–223) swing by an order of magnitude across runs of the
**same tree** — they measure machine load, because load inflates all three sessions together and `cold`
is nothing but the noise floor. The last row is the clincher: `cold` closed in **zero** turns, which no
ratio can divide by. The old law only survived that at all because of a `.max(1)` on the denominator —
a fudge whose only job was to keep a division from exploding, which is a tell that the quantity being
divided was never the law. Every one of these ten runs is under the 768-turn ceiling; the dearest sample seen, `long = 442`, still leaves 1.74x of headroom.

### 3.2 The law the lane meant, restated

The fixture (`✏️s/🔌️plugins/🌀️procedural/🧫️fixtures/🚪️close-ladder/🔣️.json`, still pure language-agnostic
JSON) now declares two things instead of `maximumCloseTurnGrowth`:

1. **`maximumCloseTurns: 768`** — unchanged, and now the primary statement. Every session, cold and
   eight-document alike, must fit the **same** fixed budget; that budget is the browser's own
   (`plugin-ui.lifecycle-close-budget-exhausted`), and it is what the pre-fix 2 052-turn constant blew.
2. **`minimumRetainedWorkDilutionPercent: 140`** — the load-invariant half. With
   `retainedWork = documents × rendersPerDocument`, the dearest session's turns-per-retained-unit must be
   at least 1.40× cheaper than the cheapest *non-cold* session's. `cold` (work 0) is excluded, which is
   exactly what kills the ratio.

Dilution is load-invariant because both sides inflate together: across all ten runs above it stayed in
**1.95 – 4.00** for a 5.33× growth in retained work, while the regression this law exists to catch — a
ladder that pages one retained item per turn — scores a flat **1.00**. `140` sits between the two, 1.39×
below the worst observation and 1.40× above the failure mode. The assertion is integer-only
(`lean_cost × full_work × 100 ≥ dilution_percent × full_cost × lean_work`), so no float enters a law.

The docstring and the fixture's `lawNote` both carry the ten measurements and the reasoning, so the
next lane that sees an odd absolute number does not re-derive it.

### 3.3 Gate result

```
bun nx run @semio-tech/procedural-plugin:test
```
→ `EXIT=1`, **`Summary [3.890s] 17/19 tests run: 16 passed, 1 failed, 0 skipped`**
(`18-procedural-plugin-test-final.txt`). Was 14 passed / 2 failed / 3 never-run.

The single remaining failure is **`surface_tests::generation2d_viewer_never_mutates`** — panics at
`🧰️framework/🔨️modules/📡️replication/…/🌱️value/🗂️ordered/🦀️.rs:81` (`"ordered-map root must be explicitly
retired before drop"`). The panicking library file is clean and last touched 2026-09-08, before this
ticket opened; the bug is in **generation2d's** use of it, the sibling artifact this ticket explicitly
excludes. Nextest's fail-fast is why 2 of 19 did not run — so the gate's own line is not by itself proof
that the close-ladder law ran.

Without fail-fast the whole profile is measurable, and the law's own `[DEBUG]` line proves the recalibrated
threshold is the one that executed:

```
cargo nextest run -p semio-s-plugin-procedural --profile fundamental --no-fail-fast --success-output immediate
```
→ **`Summary 19 tests run: 18 passed, 1 failed, 0 skipped`**, twice
(`17-nextest-repeat-1.txt`: `[DEBUG] close-cost fixture costs=[("cold", 10, 0), ("warm", 61, 6), ("long", 143, 32)] ceiling=768 dilution-percent=140`;
`17-nextest-repeat-2.txt`: `costs=[("cold", 29, 0), ("warm", 86, 6), ("long", 164, 32)] … dilution-percent=140`).
The one failure in both is the same generation2d test. Confirmed a third time after the final docstring
pass (`25-nextest-final.txt`: `costs=[("cold", 14, 0), ("warm", 71, 6), ("long", 153, 32)] … dilution-percent=140`,
`19 tests run: 18 passed, 1 failed`). And the close-ladder target alone:

```
cargo test -p semio-s-plugin-procedural --test close_ladder -- --test-threads=1 --nocapture
```
→ **`test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.43s`** (`24-close-ladder-final.txt`) — all three close-ladder laws,
both editors, green.

### 3.4 One transient seen and dismissed

The first gate run after the close-ladder edit showed a second failure:
`close_ladder::generation3d_instance_close_reaches_retired` **SIGABRT**, root cause
`plugin.internal "instance busy or poisoned: 7"` on document load, then
`"runtime lifetimes require terminal exact ACK before teardown"` panicking inside a destructor
(`08-procedural-plugin-test.txt`). It did **not** reproduce in five subsequent runs of the same profile,
and the whole target passes serially. `"instance busy"` is the documented
`cell.instance.try_lock()` contention (§2.3 of the close-ladder-budget report) losing to the polling
thread under nextest's parallelism on a loaded machine. Recorded here rather than acted on.

---

## 4. `verify publication-retirement-authority` — the peer's file, now green

`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition/🦀️.rs` is **not this ticket's**:

- last commit `f39d4b0db3` (2026-09-10 13:54), a framework lane, not any of this ticket's;
- `git status: MM`, mtime **13:35:01** — live peer edit, minutes old when inspected;
- it is one half of the peer's own uncommitted `🔌️plugin/🦀️.rs` retirement refactor (§1.3). That diff
  adds `pub(crate) fn test_drive_store_replacement_jobs(&mut self, maximum_items, maximum_bytes)`
  wrapping the private `drive_store_replacement_jobs(…, false)`, and the composition test's three call
  sites (lines 654, 811, 892) now call **that accessor**.

So the peer had already chosen exactly the clean fix the brief describes — a `pub(crate)` accessor, not a
test-only hack — and was mid-flight when the audit snapshotted the crate. Nothing for this lane to do,
and touching it would have collided.

```
bun ./📜️script.ts verify publication-retirement-authority
```
→ **`EXIT=0`**, `test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 681 filtered out; finished in 0.02s`
(`19-publication-retirement.txt`). The gate is **green**.

---

## 5. Final counts, and every remaining red with its owner

| § 5 command | result | exit |
|---|---|---|
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=1` | `379 passed; 6 failed` | 101 |
| `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --test example-geometry` | `17 passed; 0 failed` | 0 |
| `cd …/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && SEMIO_TEST_LEVEL=long bun ./📜️script.ts test long` | `Test Files 30 passed (30)` / `Tests 1009 passed (1009)` | 0 |
| `bun nx run @semio-tech/procedural-plugin:test` | `17/19 tests run: 16 passed, 1 failed` | 1 |
| `bun ./📜️script.ts verify publication-retirement-authority` | `2 passed; 0 failed` | 0 |

**Remaining reds — 7 in total, all seven owned outside this ticket's lanes:**

| red | owner | evidence |
|---|---|---|
| `add_generation_records_an_undoable_generation_operation` | peer | asserts at `🔌️plugin/🦀️.rs:6871`, file `MM`, mtime 14:22:49, +347/−75 uncommitted retirement refactor |
| `undo_redo_round_trips_flow_graph_edits` | peer | same line, same message |
| `two_instances_converge_disjoint_widget_moves` | peer | `🔌️plugin/🦀️.rs:6848`; `module.vcs` fail-closed gate in `🏪️store/🦀️.rs` (` M`, mtime 14:18:20) |
| `generation_preview_is_one_app_transient_shared_by_two_generation_windows` | peer | 30 s retire deadline, the peer's `drive_store_replacement_jobs` drive loop |
| `refresh_pending_effects_arms_flow_eval_tick_chain` | peer | same class — `"registered fixture typed operation did not retire within 30 seconds"` |
| `vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | peer | `generation3d-publication.contended` — the publication guard the peer's diff rewrites |
| `surface_tests::generation2d_viewer_never_mutates` | generation2d, out of ticket scope | panics in `🌱️value/🗂️ordered/🦀️.rs:81`, a clean file last touched 2026-09-08; the defect is generation2d's usage |

Five of the six lib reds are byte-identical to the peer-attributed set already recorded in
`📓️viewer-status-parity-2026-09-12.md` §4.5; the sixth is the same retire-deadline class.

**Still open from §5 of the audit, not attempted here:** item 4, `verify taxonomy enforce` — it did not
finish in 20 minutes on the audit run and this lane's budget went to items 1-3 and 5. Item 6's repo-wide
gates remain out of scope, except `verify publication-retirement-authority`, which is now green.

---

## 6. Live peer churn encountered (for the next lane's benefit)

Three separate peers were writing shared files throughout this session; two of them broke commands
mid-run and both cleared on retry:

- **`semio-framework-os-kernel` stopped compiling** at 13:42 —
  `error[E0063]: missing fields 'expires_at_us', 'generation' and 'operation' in initializer of 'InitialMemberStoreOpen'`
  at `🏪️store/🧩️composition/🚪️open/🏭️operation/🦀️.rs:160`, a file written 16 seconds earlier. Green on the
  next poll.
- **`🔣️taxonomy.json` was rewritten repeatedly**, breaking `loadTaxonomy` and with it any `nx` target that
  depends on `framework-graph:generate`. Two distinct shapes seen:
  `generatorContracts["styling-tokens"].nativeConsumers has no Cargo project at "…/📦️packages/🐍️python"`
  (13:55) and `packageSourceDispositions is missing source-format contract "storybook-main"` (14:26).
  Both cleared within one retry. Any close-time gate run should retry past this rather than record it.
- **the renderer react target was relocated** mid-session (§2.1) — the `📦️packages/🟦️typescript` and
  `🎯️targets/⚛️react` path segments swapped order. The audit's §5 command line is stale as written.

---

## 7. Files changed

Seven, `576 insertions / 180 deletions` — the two large diffs are the settled test-module merge this lane
inherited, not new code.

| file | change |
|---|---|
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | `use self::context::{…}` — drop the self-reimport that collided with the file's own `mod context` (E0255) |
| `✏️s/…/🧊️generation3d/…/✏️editor/🧪️tests/🔬️tick-addressing/🦀️.rs` | `super::tests::` → `super::unit_tests::node_eval_status` (E0433) |
| `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🚪️close-ladder/🦀️.rs` | ratio assertion replaced by the absolute ceiling + retained-work dilution law; docstring rewritten to say why a ratio cannot express it |
| `✏️s/🔌️plugins/🌀️procedural/🧫️fixtures/🚪️close-ladder/🔣️.json` | `maximumCloseTurnGrowth: 8` → `minimumRetainedWorkDilutionPercent: 140`, plus a `lawNote` carrying the ten measurements |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` | six `invoke` doubles now assert `signal: expect.any(AbortSignal)`; `buildNoteShellCommandAction` law expects `inverseCommandId`/`inverseArgs`; the node-graph successor-size read folded into a `waitFor` |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | leftover default window alias added to the surface-binding law; `readAppDocumentPack` law pins a non-empty `ops` |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🩺️window-fault/🟦️.ts` | `?raw` import repointed from the 182-byte re-export shim to the real bundle at the target root |

No production file was edited: every change is a test, a fixture, or a docstring. No file that a peer was
actively writing was touched.
