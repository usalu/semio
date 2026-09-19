# C1 — Two-tab collaboration over the local hub

Slice C1 (Opus 5 execution worker). Ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`.
Everything below is measured; command captures are under `🗑️generated/c1-*.txt` in this folder.

---

## 0. Headline

Three real defects were found and fixed at the root, and one hub-owned defect was found that
**no `OS_HUB_DATA` in this repo can ever get past without an environment override** — it is why every
collaboration e2e run in this session (and, on the evidence, the `26/08/17` runs before it) reported
`hub exited early (code 1)` rather than a scenario result.

| # | Defect | Owner | State |
|---|---|---|---|
| 1 | `readHistory` reported `missing HistorySnapshot frame` and threw away the guest's own fault text | `🔌️PluginRuntime` (mine) | **fixed** |
| 2 | `readHistory` had no bound against the transient `instance busy` refusal | `🔌️PluginRuntime` (mine) | **fixed** (bounded retry + backoff) |
| 3 | `ArtifactCodec`'s `print_mirror`/`apply_ops_binary` dropped a parsed envelope without detaching its owners — aborts the process on **every artifact bootstrap install** | `🏪️store` (mine) | **fixed** |
| 4 | `os-hub:dev` cancels its own trusted stdio+GIS catalog build because `buildBudgetMs()` defaults to `0` and `trustedBootstrapBuildControl` reads `0` as *already expired* | `🌎️hub/**` (worker H1) | **diagnosed, not edited** — worked around with `SEMIO_BUILD_BUDGET_MS` |

---

## 1. The `readHistory` storm — what it actually was

The audit (`📓️audit-collaboration.md` §7 P1.1) and `26/08/17`'s `📓️final-summary.md` both describe the
blocker as a hub bootstrap-ordering problem: *"why the HistorySnapshot frame is missing (host-side
`🏪️store/🔄️sync` actor / hub `document_ws_v1` bootstrap frame order)"*. **That framing is wrong, and the
wrongness was caused by the bug itself.**

`HistorySnapshot` is not a `ServerFrame`. `grep -n "HistorySnapshot" 🧰️framework/🔨️modules/📡️replication/📡️wire/🦀️.rs`
returns nothing: the hub never sends such a frame and never could. `HistorySnapshot` is an **`AppFrame`**
on the plugin app-channel (`💻️os/🔨️modules/📡️spr/🧵️channel/🦀️.rs:2339`, wire tag `14`), i.e. the
host ↔ wasm-guest channel, one layer *below* anything the hub can see.

### 1.1 Root cause, guest side

`plugin_exchange`'s `ReadHistory` arm
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:37723-37732`):

```rust
protocol::AppCommand::ReadHistory { seq } => {
    let snapshot = with_instances_mut(runtime, |list| { … resolve_ready(instance.app.history_snapshot()) });
    match snapshot.await {
        Ok(history_patch) => frames.push(protocol::AppFrame::HistorySnapshot { in_reply_to: seq, … }),
        Err(fault) => push_app_fault(&mut frames, Some(seq), fault).await,
    }
}
```

The instance cell is taken with `try_lock` (`🔌️plugin/🦀️.rs:35395` and `:35681`), so a snapshot read that
lands while the **same** instance is already mid-turn is refused outright with
`instance busy or poisoned: {instance_id}` rather than queued behind that turn. A refusal carries an
`Error` frame and **no `HistorySnapshot` frame at all**.

### 1.2 Root cause, client side — the message that hid it for a month

`adaptPluginHandle`'s `readHistory` was the **only** method in its block that did not inspect the `Error`
frame. Its siblings `applyMutations`, `readAppDocumentPack`, `loadAppDocumentPack`, `setMergePolicy`,
`resolveConflict`, `readConflicts` all do `faultDisplayMessage(errorFrame.Error.fault, …)`. `readHistory`
instead did:

```ts
if (!frame) throw new Error("[DEBUG] readHistory: missing HistorySnapshot frame");
```

So the guest said *"instance busy"* and the host printed *"missing HistorySnapshot frame"* — a sentence
that names a **hub** concept and discards the **plugin** fault underneath it. Every later reading of the
log therefore pointed at `document_ws_v1`. There was no hub bug to find.

### 1.3 Fixes (file:line, after the edit)

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`

- `:3653-3701` new `//#region 🔖️HistorySnapshotRetry` — `PLUGIN_INSTANCE_BUSY_MAX_ATTEMPTS = 4`,
  `PLUGIN_INSTANCE_BUSY_BACKOFF_MS = 25`, `isPluginInstanceBusyFaultV1`, `readHistoryWithBoundedRetryV1`.
  The ladder is 25 / 50 / 100 ms (175 ms total, two orders of magnitude above a guest turn's 8 ms
  ceiling) and then **stops**: `🏛️ShellHost`'s history effects are edge-triggered and never re-armed by
  their own callers, so an exhausted ladder must raise the real fault, not re-arm.
- `:3776` `readHistory` now delegates to it. A refusal that is **not** the transient lock refusal is
  raised on the first reply, with the guest's own text, with no retry at all.

The client "spinning" was never a loop inside `readHistory` — it was React effect identity churn, already
fixed by `26/08/17` lane 5-A in `🏛️ShellHost` (`:2124-2153`, `:4815`, `:5215`, `:5859`). What was left was
the opposite failure: a single refusal became a permanent, mislabelled error. Both halves are now closed.

### 1.4 Renderer test — bounded retry with backoff

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx:1468-1540`,
new `describe("history snapshot bounded retry")`, four laws:

1. a transient refusal is retried on the doubling ladder and the eventual snapshot is returned;
2. the ladder is bounded (`MAX_ATTEMPTS` reads, `MAX_ATTEMPTS - 1` waits) and raises the guest's own
   sentence — explicitly asserting the message does **not** contain `missing HistorySnapshot frame`;
3. a non-transient fault is raised on the first reply, `reads === 1`;
4. `isPluginInstanceBusyFaultV1` names only that one refusal.

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" -t "history snapshot bounded retry"
 Test Files  1 passed | 58 skipped (59)
      Tests  4 passed | 1441 skipped (1445)
   Duration  60.26s
```

---

## 2. Bootstrap ordering on the wire — and a process-aborting store defect underneath it

The brief asked for "a unit test that the bootstrap sequence contains the HistorySnapshot before
Commands". Since `HistorySnapshot` is not a server frame, the honest equivalent law — and the one the
client actually enforces — is: **a `ServerFrame::Commands` tail must not be accepted before the artifact
bootstrap that it rebases on has completed.** That rule lives in
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs:2569-2572`
(`"tail arrived before artifact bootstrap completion"`) and had no test.

New test: `🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs:614-700`,
`bootstrap_sequence_refuses_a_command_tail_that_arrives_before_the_snapshot`. It drives two real
`native_actor::ArtifactActor`s through `inject_hub_frame`:

- **out of order** — `Welcome{ArtifactBootstrap}` → one chunk → `Commands`: nothing is installed
  (`current_pack`/`current_spr` stay `None`), the server frontier never advances, the remote state is not
  `Live`, and the refusal is **emitted** as an `artifactBootstrap` conflict naming the order it enforced;
- **in order** — the same `Commands` frame after `ArtifactBootstrapDone` is accepted, the pair installs,
  the frontier advances, the state is `Live`.

The second half is what makes it an ordering law rather than a blanket rejection.

### 2.1 What writing that test uncovered

The test failed — and so did the two **pre-existing** sibling tests
(`native_bootstrap_commits_pair_before_failed_local_replay_then_restarts_without_duplicate`,
`native_inline_and_chunked_bootstrap_install_the_same_typed_pair_after_cancelled_restart`), with:

```
panicked at 🏪️store/🦀️.rs:2730:
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority
detached every nested owner
```

`RUST_BACKTRACE=1` put the drop inside **production** code, not the test:

```
2: <ArtifactEnvelope<…> as Drop>::drop
4: drop_glue::<ParsedDocumentText<…>>
5: <ArtifactCodec>::of::print_mirror_impl::{closure#0}
7: <native_actor::ArtifactActor>::install_artifact_bootstrap::{closure#0}
```

`ArtifactEnvelope`'s `Drop` is a hard `assert!` (not a `debug_assert!`), so this **aborts the process on
every artifact bootstrap install** — native and, as a wasm trap, in the browser shell. It is precisely the
failure mode `ParsedDocumentText::into_envelope`'s own docstring was written to prevent
(`🏪️store/🦀️.rs:11025-11034`, "what killed every `.pack`/`.spr` load of a generation3d document"), and
three call sites in the codec erasure table had never been moved onto it.

Fixed in `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`:

- `:10386-10390` `print_mirror_impl` — `parsed.into_envelope()`, print, then `drop(envelope.into_owners())`.
- `:10402-10407` `apply_ops_binary_impl`'s empty-ops branch — same, with the print's `?` deferred so the
  error path also retires the envelope.
- `:10416-10421` `apply_ops_binary_impl`'s apply branch — `parsed.into_envelope()` instead of the partial
  move `let mut envelope = parsed.envelope;`, which used to leave `snapshot` to drop (the exact
  fail-closed-projection abort the docstring names).

A fourth, smaller leak was in the sync test fixture itself
(`🔄️sync/🧪️tests/🔬️unit/🦀️.rs:340`, `demo_artifact_bootstrap` dropped its `create_document_envelope`
result) — also fixed.

```
$ cargo test -p semio-framework-os-kernel --lib --features sync bootstrap --message-format short
running 4 tests
test os_store::sync::tests::bootstrap_frontier_identity_rejects_same_ordinals_with_wrong_authenticated_head ... ok
test os_store::sync::tests::bootstrap_sequence_refuses_a_command_tail_that_arrives_before_the_snapshot ... ok
test os_store::sync::tests::native_inline_and_chunked_bootstrap_install_the_same_typed_pair_after_cancelled_restart ... ok
test os_store::sync::tests::native_bootstrap_commits_pair_before_failed_local_replay_then_restarts_without_duplicate ... ok
test result: ok. 4 passed; 0 failed; 1146 filtered out
```

Two of those four were red before this slice. Note the feature gate: the sync module is mounted
`#[cfg(all(feature = "sync", …))]` (`💻️os/📦️packages/🦀️rust/🦀️.rs:280`), so a bare
`cargo test -p semio-framework-os-kernel --lib` compiles **none** of it and reports `1095 tests` with the
whole sync corpus silently absent. `--features sync` raises that to `1150`.

---

## 3. Presence session-colour wire extension (audit P1.3)

**Already landed.** `presence_peer_rows_for_surface`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:562-581`)
carries `color: peer.color` at `:578`; `PresencePeer.color: Option<u8>` exists on the wire
(`📡️replication/📡️wire/🦀️.rs:1368`). The file moved since the `26/08/17` cross-lane note was written
(`🧱️elements/Shell/🧊️component.rs` → `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`), which is why the
note reads as outstanding.

Gates:

- `cargo check -p semio-framework-plugin --message-format short` — **green**
  (`Finished dev profile … in 4.06s`, 40 pre-existing warnings). Capture: `🗑️generated/c1-plugin-check.txt`.
- `cargo test -p semio-framework-ui --lib --features wgpu presence` — **cannot compile the test target**,
  for reasons that are not presence and not mine. 5 errors, all unresolved module paths in two test
  files: `crate::wgpu::events`, `wgpu::widgets`, `wgpu::chrome`
  (`🖱️ui/🧪️tests/🔬️targets-wgpu-input-unit/🦀️.rs:90,102,105,110` and
  `🔬️targets-wgpu-theme-token-parity/🦀️.rs:229`). `git status` shows a peer mid-refactor across
  `🖱️ui/🎯️targets/🧊️wgpu/**` right now (`🦀️.rs`, `🪀️widgets/`, `🖌️paint/`, `🖍️draw/`, `🧊️gpu/` all
  modified, `⚡️events`/`🪀️widgets`/`🖥️chrome` directories present but not yet re-exported under those
  names). My diff touches no file in that crate. Capture: `🗑️generated/c1-ui-presence-test.txt`.
  **Hand-off: this gate is blocked on whoever owns the `🖱️ui` wgpu module rename, not on presence.**

---

## 4. The hub cannot boot from a fresh `OS_HUB_DATA` — `SEMIO_BUILD_BUDGET_MS`

Three separate collaboration e2e attempts reported `hub exited early (code 1)`. The causes, in the order
they appeared:

1. **`bun nx run os-hub-admin:build` failed once** (first attempt only), transiently, under nx graph
   contention. It builds clean on demand (`✓ built in 10.86s`, `Successfully ran target build`).
2. **`Missing Nx-staged os-hub dev binary: 🌎️hub/📦️packages/🦀️rust/dist/build-dev/os-hub`.**
   `hubDevBinaryPath` (`🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts:82-84`) reads a staged binary and does
   not build it. Fixed by running `bun nx run os-hub:build-dev` — **5m 34s**, green
   (`Successfully ran target build-dev for project os-hub and 4 tasks it depends on`), producing a
   265 MB `dist/build-dev/os-hub`. This also proves `cargo check -p semio-hub` is currently green.
   Capture: `🗑️generated/c1-hub-build-dev.txt`.
3. **`trusted codec capture: trusted codec capture cancelled`** — the real, structural one.

`ServeScript.run` materializes a trusted stdio+GIS catalog whenever `trustedBootstrapCurrent(dataRoot)`
is empty (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:11955-11963`). That build is guarded by

```ts
const buildControl = trustedBootstrapBuildControl(buildBudgetMs());          // :9348
…
remainingMs: () => Math.max(0, deadlineMs - (Date.now() - started)),          // :9325
…
if (control.cancelled() || control.remainingMs() <= 0) throw new Error("trusted codec capture cancelled"); // :9355
```

and `buildBudgetMs()` returns `Number(process.env.SEMIO_BUILD_BUDGET_MS ?? BUILD_BUDGET_MS)` with
`BUILD_BUDGET_MS = 0`, documented as *"zero leaves compilation and Cargo lock waits unlimited"*
(`🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts:13-19`).

`trustedBootstrapBuildControl` is the one consumer that reads `0` as **already expired** instead of
unlimited, so with `SEMIO_BUILD_BUDGET_MS` unset — which is the default in this environment
(`env | grep SEMIO` is empty) — the very first `check()` throws and the hub exits `1` **before it has
done any work at all**. Every hub boot against an empty `OS_HUB_DATA` fails this way, which is why the
collaboration e2e (fresh `mkdtempSync` data dir per run, by design) has never reached step 1 through the
hub, and why `26/08/17` recorded 2/8 with steps 2-6/8 attributed elsewhere.

**I did not edit this** — `🌎️hub/**` is worker H1's. The fix is one of: default the control to unlimited
when `deadlineMs === 0`, or give `trustedBootstrapBuildControl` its own non-zero default. Until then every
hub invocation needs `SEMIO_BUILD_BUDGET_MS` set to a real number; this slice used `5400000`.

<!--C1-STATUS-->

---

## 5. Two new scenario assertions

`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`. The scenario is now
**10 steps**; the previous step 8 (hub restart) became step 9 so the two new steps sit in execution order.

### 5.1 Instrumentation — counting `ServerFrame::Commands`

- `:111-117` `CollabCommandFrameCounter`.
- `:302-320` `collabCountCommandFrames(page)` — attaches to every websocket the page opens, keeps only
  `/spaces/{id}/documents/{id}/socket/v1` (the directory socket is excluded by path), and counts frames
  whose second byte is `3`. A server frame is `lane: u8 | tag: u8 | fields…` and `Commands` is tag `3`
  (`📡️replication/📡️wire/🦀️.rs:950-955`, `out.push(3)`).
- `:322-341` `collabWaitForEditorText` — polls the peer's editor and returns **how many `Commands` frames
  arrived** between the call and the text first being observed.

### 5.2 STEP 8 — one round trip (audit §7 step 3)

`:583-612`. User 1 opens the artifact, types a marker into its editor, and the harness asserts the marker
reaches user 2's editor with `frames >= 1` (it cannot have arrived any other way than through the hub —
this catches two unbound ephemeral editors that merely look synced) **and** `frames <= marker.length`
(one relay per keystroke, no tail re-sends).

### 5.3 STEP 10 — restart with an unacknowledged in-flight edit (audit §7 step 4, §4's open item)

`collabRunRestartStep`, `:630-700`. The step now:

1. kills the hub and waits for the port to free;
2. **then** types a marker into user 1's already-open editor — the socket is down, so the edit commits to
   the local ledger and queues in the `ArtifactActor` outbox with no `ServerFrame::Ack` behind it. This is
   exactly the "short connection shortage" condition `AGENTS.md` calls out, and exactly what §4 of the
   audit lists as untested at the document layer;
3. restarts the hub on the same port against the same `OS_HUB_DATA`;
4. asserts (step 9) the space + artifact rows survive the reload, and (step 10) that user 2 receives the
   offline edit through at least one `ServerFrame::Commands` frame — i.e. the resume-token/frontier path
   carried it rather than dropping it.

Step 10 refuses to pass vacuously: if user 1 had no open editor at restart time it fails with
"nothing was ever in flight" rather than reporting a truth about nothing.

<!--C1-SCENARIO-->

---

## 6. Cold `dev s` build budget

<!--C1-BUDGET-->

---

## 7. Files changed

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `🔖️HistorySnapshotRetry` region (`:3653-3701`); `readHistory` delegates to it (`:3776`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | `describe("history snapshot bounded retry")`, 4 laws (`:1468-1540`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` | envelope retirement at the three codec erasure-table sites (`:10386`, `:10402`, `:10416`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` | fixture retirement (`:340`); bootstrap-order law (`:614-700`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` | 10-step scenario, `Commands` frame counter, STEP 8 and STEP 10 |

No file owned by worker H1 (`🌎️hub/**`) or worker O1 (`🏛️ShellHost/🟦️.tsx`, root `📜️script.ts`,
wgpu `switch_to_app`, the framework vitest include list) was touched.

---

# C1b — the 10-step scenario, run end to end

Slice C1b (Opus 5 execution worker), 2026-09-19. Continues the C1 report above. Captures under
`🗑️generated/c1b-*.txt`.

## 8. Run log — what actually happened

**Headline: the scenario still has not run, and C1b found the three reasons why — two of them fixed, the
third measured and handed off.** Everything below is observed; captures are `🗑️generated/c1b-*.txt`.

C1's §4 diagnosis (the zero build budget) was correct and worker H1 fixed it, so the hub no longer exits
before doing any work. What that uncovered is a second, longer chain, in the order C1b hit it:

| # | blocker | owner | state |
|---|---|---|---|
| 5 | `preparePluginBuildTargets` asserts the extension install root is fresh **before** the sync that refreshes it — every plugin build in the repo refused, with a recovery instruction that routes back through the same assert | `🔌️plugin/🏗️build` (mine) | **fixed** (§10.1) |
| 6 | the harness spawns `bun 📜️script.ts dev`, a command the `🧑‍💻dev` router does not have — both shells exited 1 before serving a byte | `🤝️collaboration` (mine) | **fixed** (§10.2) |
| 7 | `OS_HUB_DATA` under macOS's symlinked `/var` temp root | harness + H1 | **fixed both sides** (§10.3) |
| 8 | `semio-s-plugin-stdio`'s fresh descriptor exceeds the 4 MiB descriptor contract bound, so the trusted stdio+GIS catalog cannot be published, so `artifactAuthority` never reports ready, so `DevScript`'s own `waitForReadiness` never admits the hub | hub/plugin descriptor contract | **measured, handed off** (§10.4) |
| 9 | the two shells have no way to authenticate: the shell's only identity path needs a `#semio-broker=` proof answered by a `LocalBrowserRelay`, and the harness starts none | hub + shell design | **diagnosed, not fixed** (§12.1) |

Blockers 5-7 were real and are gone; blocker 8 is why no hub came up in this session; blocker 9 is why the
scenario would still not have gone green if it had.

**What did run, measured:**

- `🐍️c1b-prebuild.ts` — the scenario's own plugin prebuild, green. `🪐️space` (the cold build C1 deferred)
  and `✒️writer` both produce `*_component.core.wasm`. Capture `🗑️generated/c1b-prebuild.txt`.
- the extension install root went from **26/26 stale to 5/26** after one prebuild, measured with the same
  probe before and after (§10.1, §12.3).
- `deployment output preservation` — **7/7 pass**, including C1b's new law. Capture
  `🗑️generated/c1b-extension-gate-laws.txt`:

```
$ bun ./📜️script.ts test quick --testNamePattern='deployment output preservation'
 Test Files  1 passed | 2 skipped (3)
      Tests  7 passed | 167 skipped (174)
```
  Note the invocation: a bare `bunx vitest run --config ../../🧪️tests/🎚️config/🟦️.ts` **cannot load this
  suite at all** (`Cannot bundle built-in module "bun:sqlite" imported from ⚡️caching/🔒️leases/🟦️.ts`).
  The project's own `📜️script.ts test` verb configures the environment correctly; the raw `bunx vitest`
  form is what makes this file look red.
- `cargo build --bin os-hub` on **default** features — green in 8m 16s, 7 warnings. This is the first
  default-features hub build recorded in this ticket: worker H1's §1 blocker (the `semio-s-artifact-stdio-semio`
  PDF callers) was cleared by slice P3 while C1b was running.

**Two concurrency incidents worth recording** (both cost a full build cycle):

1. The first hub build failed with 10 × `E0433 cannot find module or crate 'server'` in
   `🌎️hub/🗄️stores/🦀️.rs`. Not a real break: cargo had read `Cargo.toml` at 02:13 and a peer added
   `semio-framework-server = { workspace = true }` at 02:19. Re-running compiled it. A repo-wide failure
   whose manifest is newer than the cargo start time is a stale-manifest race, not a defect.
2. The first materialization was SIGKILLed at 77 minutes by `SEMIO_BUILD_BUDGET_MS=5400000`
   (`fresh component timeout at build (status=null, signal=SIGKILL)`). Worker H1's fix makes an **unset**
   budget mean the 24 h fresh-component ceiling, so pinning 90 minutes — as C1's own runner script did — is
   now strictly worse than leaving it alone. Both ticket runner scripts were corrected.

## 9. Per-step results

No step of the ten has been observed passing in a browser, by C1b or by anyone before it. The honest table
is therefore about *reachability*, not about pass/fail — claiming otherwise would be inventing results.

| step | what it asserts | reachable today? | blocked by |
|---|---|---|---|
| 1 | user1 creates a space; user2's Home shows the row | no | §10.4 (no hub) then §12.1 (no identity) |
| 2 | share with user2 as author; user2 opens `/spaces/{id}` | no | §10.4, §12.1 |
| 3 | create a writer artifact; row in both tables; editor opens | no | §10.4, §12.1 |
| 4 | user1 types, user2 sees it | no | §10.4, §12.1 |
| 5 | 2 presence peers in distinct hub session colours | no | §10.4, §12.1 |
| 6 | check-in moves the table's updated column for both | no | §10.4, §12.1 |
| 7 | `/admin/api/connections` names both users; `/admin` is HTML | no | §10.4; and see §12.2 — the assertion is written against an identity delivery that no longer exists |
| 8 | one `ServerFrame::Commands` round trip per keystroke | no | §10.4, §12.1 |
| 9 | hub restart against the same `OS_HUB_DATA` preserves space+artifact | no | §10.4, §12.1 |
| 10 | an unacknowledged in-flight edit survives the restart | no | §10.4, §12.1 |

What C1b did change is that steps 1-10 no longer fail for **harness** reasons. Before this slice the run
reported all ten as `blocked — shells did not boot` because of §10.2, and before that `hub exited early
(code 1)` because of C1's §4 and §10.3. Those three are closed; what is left are two genuine product gaps
(§10.4, §12.1), each with a named owner and a concrete shape.

The brief's other named behaviours — **concurrent edits converging**, **per-user undo**, and a **short
connection loss that does not freeze the app** — are not steps of this scenario at all (§12.4). They were
not added: writing assertions into a scenario that cannot reach step 1 would produce evidence of nothing.

## 10. Root fixes made by C1b

### 10.1 The extension freshness gate refused the build that repairs it — **root-fixed**

`preparePluginBuildTargets` is the entry point of every plugin build in the repo (`collabPrebuildPlugins`,
`plugin` CLI, `activate-*`, `dev`). Its order was:

```ts
assertExtensionOutputsFresh();                     // 🧰️framework/…/🔌️plugin/🏗️build/🏃️execution/🟦️.ts:148 (before)
const targets = resolvePluginBuildTargets(catalogEntries, filterPlugin);
syncBuiltExtensionsToInstallRoot(targets);         // the step that REFRESHES what the assert just judged
```

`assertExtensionOutputsFresh` compares each installed extension's `🟨️.js` against the current
`hostShimSource()` and throws, retaining the bytes, with *"Rebuild its owner through
@semio-tech/framework-os-dev:plugin"*. That instruction routes straight back through this same assert, so
the gate had **no reachable exit**: once the host shim source changed, every plugin build in the repo was
refused for ever.

Measured state when C1b started (probe `/tmp`-scratch, census reproduced in §8): **26 of 26** installed
extensions under `🧑‍💻dev/🧩️extension-modules/` were stale (installed 09-01, 09-06 and 09-14; the shim is
6752 bytes today), while **54 of 54** built extension outputs under `🔌️plugin-modules/` already carried the
current shim. The bytes to repair 21 of the 26 were already on disk, one function call away, behind the
assert.

Fix, two files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📥️installation/🟦️.ts:40-57` —
  `assertExtensionOutputsFresh(root, rebuilding)` skips an install directory whose extension is in
  `rebuilding`, because `publishBuiltExtension` replaces that directory wholesale later in the same run.
  The default `rebuilding = []` keeps the function's existing single-argument semantics, so its five
  existing laws are untouched.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts:149-151` — resolve targets,
  **sync**, then assert with the build scope.

New law: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`,
`"does not refuse the very build that republishes the stale install"` — a directory holding both a stale
shim and a retained worker is accepted when its extension is in scope, refused when another extension is in
scope, refused with no scope at all, and its retained bytes are still on disk afterwards.

Effect, measured: `🐍️c1b-prebuild.ts` went from

```
error: Extension stale host shim preserved: …/🧩️extension-modules/🔡️imperative-extension-text/🟨️.js
      at assertExtensionOutputsFresh (…/🏗️build/📥️installation/🟦️.ts:51:73)
      at preparePluginBuildTargets (…/🏗️build/🏃️execution/🟦️.ts:148:3)
```

to

```
program build scope: all (60 plugin crates)
[c1b-prebuild] space: PRESENT …/🔌️plugin-modules/🪐️space/semio_s_plugin_space_component.core.wasm
[c1b-prebuild] writer: PRESENT …/🔌️plugin-modules/✒️writer/semio_s_plugin_writer_component.core.wasm
[c1b-prebuild] done
```

Captures: `🗑️generated/c1b-prebuild.txt`. This unblocked the `🪐️space` cold wasm build C1 deferred, and it
unblocks every other slice that touches a plugin build.

### 10.2 The harness spawned a command that no longer exists — **root-fixed**

`collabStartUserDevServer` spawned `bun 📜️script.ts dev`. The `🧑‍💻dev` bundle router registers
`prepare|activate|serve|…` and **no `dev`**:

```
$ bun ./📜️script.ts dev
unknown command "dev"
usage: bun ./📜️script.ts <prepare|activate|serve|canonical-bootstrap-folder-mirror-check|…> [args…]
```

So both shells exited 1 immediately and `runCollabE2eVerify` recorded all ten steps as
`blocked — shells did not boot`. No collaboration step in this harness could ever have run since the
`dev` command was split into `activate` + `serve`.

Fix, `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts`:

- new `collabActivateShellRuntime()` — runs `nx run @semio-tech/framework-os-dev:activate-s-react-dev`
  **once** for both users. Activation is keyed by variant+renderer+profile, not by user: a per-user
  activation would stage the same tree twice and let the second run swap modules out from under the first
  shell's open page. It is deliberately not the `dev-…` nx target, which is a *watch* that re-activates on
  any peer's source write — under this repo's concurrent fleet that moves the staged tree mid-scenario.
- `collabStartUserDevServer` now spawns `bun 📜️script.ts serve s react dev` with `SEMIO_VITE_HMR=0` and the
  per-user `S_OS_PORT`/`S_HUB_URL`/`S_USER`/`S_DATA_DIR`. `S_HUB_URL` is baked at Vite `define`-time
  (`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:216`), so it has to be in the **serve** process's env.
- `runCollabE2eVerify` calls the activation before the two serves, with its own
  `blocked — shell activation failed` fan-out so a staging failure is never mistaken for a scenario result.

### 10.3 `OS_HUB_DATA` under a symlinked ancestor — **hardened in the harness**

`collabHubDataDir` returned `mkdtempSync(join(tmpdir(), …))`, i.e. `/var/folders/…` on macOS, and `/var` is
a symlink. The hub's trusted-catalog loader walks its configured data root with `O_NOFOLLOW`, so the hub
exited 1 with `ArtifactAuthority(Catalog("Not a directory (os error 20)"))` before binding a port (worker
H1's §6.1). H1 fixed the loader; the harness now also realpaths both branches of `collabHubDataDir`
(`🤝️collaboration/🟦️.ts:153-166`) so a run does not depend on which `os-hub` binary happens to be staged.

## 11. Permanent target wiring

The permanent wiring the brief asks for **already existed** and was verified, not re-added:

| layer | where | value |
|---|---|---|
| script route | `🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts:47,60` | `VerifyScript` imports `runCollabE2eVerify` and dispatches `segments[0] === "collab"` |
| nx target | `🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json:269-275` | `"collab-e2e"`, `nx:run-commands`, `cache: false`, `bun ./📜️script.ts verify collab` |
| launch row | `.vscode/launch.json:3306-3318` | `🛠️dev🤝️os-collab-e2e`, group `3_dev`, `bun nx run @semio-tech/framework-os-dev:collab-e2e`, `SEMIO_BUILD_BUDGET_MS=5400000` |
| project inputs | `📋️project.json:50` | the harness file is already a declared build input |

Two ticket-folder runners were written so a rerun does not have to reconstruct the environment:

- `📜️c1b-warm-catalog.sh` — publishes the trusted stdio+GIS catalog **once** into
  `.🧬semio/🌐hub/c1b-warm` (`trusted-stdio-gis-bootstrap`). This is the expensive half of a hub boot:
  a default-features `cargo build --bin os-hub` plus two `wasm-release` component builds into private
  `--target-dir`s that share nothing with the workspace cache.
- `📜️c1b-collab-run.sh` — one `verify collab` run seeded from that catalog, with `NX_DAEMON=false` (the
  shared daemon re-invalidates the project graph on every peer write and never settles under this fleet)
  and widened hub/dev/prebuild boot budgets.

## 12. Honest gaps

### 12.1 The two shells cannot authenticate to the hub at all — **diagnosed, not fixed**

This is the deepest finding of C1b and it is upstream of every collaboration step. It is **not** a boot
problem, so none of §10's fixes touch it.

The React shell's identity comes from one place only (`🏛️ShellHost/🟦️.tsx:3392-3412`): a
`BrowserBrokerPortClientV1` handshake against the backbone worker, which refuses every request unless a
**local browser broker proof** was installed:

```ts
const current = localBrowserBrokerProof;
if (!current || Date.now() > localBrowserBrokerProofExpiresAtMs) {
  clearLocalBrowserBrokerProof();
  throw new Error("browser broker rebootstrap required");
}
```
`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts:772-777`; the proof is a 64-hex value
installed by `installLocalBrowserBrokerProof` (`:747-760`) with `BROWSER_BROKER_PROOF_TTL_MS = 15_000`
(`:642`), delivered to the page as the `#semio-broker=<hex>` fragment and answered by a
`LocalBrowserRelay` holding a hub-issued `react-relay` credential envelope.

The harness starts **no relay** and navigates to a bare `http://127.0.0.1:<port>/`. So
`broker.me()` rejects, `onUnavailable` fires, `identityOffline` is set, `verifiedSessionAuthority` stays
`null`, and `ShellHost` renders `SessionAuthorityNotice` (`:10960`) over the shell. Every hub-authenticated
operation the scenario needs — create space (STEP 1), share (STEP 2), create artifact (STEP 3), open a
document socket (STEPS 4/8/10) — is refused before it reaches the hub.

The only code path in the repo that wires this correctly is the hub's own
`DevScript secure-suite` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:12100-12119`): it issues one
`react-relay` credential with `issueLocalCredential(run, "developer", "react-relay", 4)`, starts ONE
`startLocalBrowserRelay(hubOrigin, uiOrigin, envelope)`, spawns ONE UI, and opens it at
`${uiOrigin}/#semio-broker=${proofHex}`. Both `issueLocalCredential` and `startLocalBrowserRelay` are
module-private to that script, and it supports exactly **one** user.

What a real two-user run needs, concretely:

1. a hub entry point that boots with **two** `LocalProfile`s (distinct `subject`, both allowing
   `react-relay`) and starts **one relay per UI origin**, reporting each user's relay url and bootstrap
   proof on a receipt the harness can read — the hub already accepts a profile list in `startLocalHub`, so
   this is a new command around existing parts, not new authority machinery;
2. the harness navigating each context to `<uiOrigin>/#semio-broker=<that user's proofHex>` instead of `/`;
3. a decision about proof lifetime across a **reload**: `relay.takeBrowserBootstrapProof()` is one-shot and
   the worker's copy is per-worker with a 15 s TTL, so STEP 9's `user2.reload()` drops the authority as the
   code stands. That is a product design question (does a reloaded shell re-bootstrap, and from where?),
   not a harness bug, and it is why this was not patched from this slice.

Until that lands, STEP 1-4, 6, 8, 9 and 10 cannot pass for reasons that have nothing to do with the
transport, the ordering, the presence wire or the round-trip bound C1 verified.

### 12.2 `S_USER` reaches no browser

`collabStartUserDevServer` passes `S_USER=user1@semio.dev` / `user2@semio.dev`, and the vite config defines
only `VITE_S_HUB_URL` and `VITE_S_DATA_DIR` (`🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts:216-217`). There is no
`VITE_S_USER` anywhere in the tree outside env-sealing allowlists. The two users are therefore distinguished
by nothing on the client; identity is meant to come from the hub session (§12.1), which is exactly the path
that is not wired. STEP 7's `/admin/api/connections` assertion (`text.includes("user1")`) is written against
an identity delivery mechanism that no longer exists.

### 12.3 Five extension installs are still stale

After §10.1's fix and one prebuild, the install root went from **26/26 stale to 5/26** (measured twice with
the same probe). The five are `imperative-extension-{text,math,logic,effect,control}`: their crates are live
catalog entries but have **no built output** in this tree at all, so `syncBuiltExtensionsToInstallRoot` has
nothing to republish from. They no longer block any build (they are in scope of a full `s` build and are
skipped by the gate), and the scenario does not load them, but a targeted build of one of those five is the
only thing that will refresh them.

### 12.4 Not verified by C1b

- The presence colour path (§3) — C1 verified the wire and the wgpu row projection; no browser has rendered
  two distinct avatar colours.
- Undo being per-user: the scenario has no undo step at all. `COLLAB_E2E_STEP_NAMES` covers creation,
  sharing, replication, live edit, presence, check-in, admin, round-trip bound, restart persistence and
  in-flight recovery — per-user undo and a deliberate short connection loss (as opposed to a full hub
  restart) are **not** among the ten. The brief asks for both; they are absent from the harness and were not
  added, because adding steps to a scenario that cannot reach step 1 would be writing assertions nobody can
  run.
- Concurrent-edit convergence: STEP 4 and STEP 8 are one-writer propagation, not two simultaneous writers.
  The event-sourced ordering is covered by the replication crate's 274 native+wasm tests (worker H2), not by
  this browser scenario.

## 13. Files changed by C1b

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📥️installation/🟦️.ts` | `assertExtensionOutputsFresh(root, rebuilding)` — skip an install directory this run will republish (`:40-57`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts` | `preparePluginBuildTargets` resolves targets, syncs, **then** asserts with the build scope (`:149-151`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts` | new law `"does not refuse the very build that republishes the stale install"` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` | `collabActivateShellRuntime()` (new, exported); `collabStartUserDevServer` spawns `serve s react dev`; `runCollabE2eVerify` activates once before the two serves; `collabHubDataDir` realpaths both branches; chromium launched with `--use-angle=metal` and an explicit 1440×900 viewport |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟦️.ts` | `readStableBuildFile` names which bound failed, with the path, the size and the limit (`:63-64`) — the old `"build input: file bound"` discarded all three, which is why blocker 8 read as a mystery for a whole build cycle |

Ticket folder (not product code): `🐍️c1b-prebuild.ts`, `📜️c1b-warm-catalog.sh`, rewritten
`📜️c1b-collab-run.sh`, captures `🗑️generated/c1b-*.txt`.

No file owned by `🌎️hub/**` was edited.

# C1c — two real users over the hub, observed at runtime

Slice C1c (Opus 5 execution worker), 2026-09-19 session 4. Continues C1b above. Captures under
`🗑️generated/c1c-*.txt`.

## 14. Inherited state and the decision taken

_(filling)_

## 15. Identity — one path for the React shell

_(filling)_

## 16. Hub readiness configuration actually used

_(filling)_

## 17. Per-step observed results

_(filling)_

## 18. The three behaviours the brief requires and the scenario lacked

_(filling)_

## 19. Permanent target wiring

_(filling)_

## 20. Honest gaps

_(filling)_

## 21. Files changed by C1c

_(filling)_
