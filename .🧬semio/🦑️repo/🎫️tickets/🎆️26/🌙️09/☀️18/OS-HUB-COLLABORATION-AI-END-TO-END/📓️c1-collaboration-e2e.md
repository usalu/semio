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

C1b left two product gaps (§10.4 hub readiness, §12.1 identity). DS1 owns the first. C1c owns the
second, and the brief names its shape exactly: the hub session AU3's live sign-in mints must become
the shell's `verifiedSessionAuthority`, it must survive a reload, and the harness must stand up two
distinct users zero-touch.

**The decision.** There is now exactly ONE identity input to the React shell: the **hub session
capability** a human mints with `POST /auth/sessions`. The launcher-issued `#semio-broker=` proof,
its 15 s TTL, its rolling SHA-256 proof chain and the `LocalBrowserRelay` round trip are gone from
the os shell and its backbone worker. What replaced each part:

| was | is | why |
|---|---|---|
| `#semio-broker=<64 hex>` URL fragment, one-shot, 15 s | `sessionStorage` capability record `{origin, token, userId}` | a fragment cannot survive a reload and a 15 s TTL cannot survive a *page*, which is exactly C1b §12.1 gap 3 |
| rolling proof + `x-semio-browser-broker{,-next,-advanced}` headers | `Authorization: Bearer <session.v1.…>` | the hub already authenticates humans this way; the ratchet authenticated the *browser* to a relay, which is a different question and made the hub see the relay's bootstrap principal, never the human (AU3 §2) |
| `/_semio/hub/*` → `LocalBrowserRelay` → hub | `/_semio/hub/*` → hub, proxied by the dev server | same-origin is kept (no preflight on an authenticated request), the relay is no longer on the shell's identity path |
| `S_USER=<email>` vite env | nothing | it reached no browser at all (C1b §12.2); the two shells are now distinguished by who signed in |

The relay itself is untouched and still serves the hub's own operator launcher: setting
`S_LOCAL_RELAY_URL` still proxies the whole `/_semio` namespace to it and takes precedence. What the
shell no longer does is *depend* on it for identity.

## 15. Identity — one path for the React shell

**Page side.** `🏛️ShellHost/🟦️.tsx`:
- `hubSessionStorageV1()` (`:222`) — `sessionStorage`, chosen deliberately over `localStorage`: the
  connection *book* (which hubs exist) is durable profile identity and carries no secret, but a
  bearer belongs to the browsing context that signed in. It must survive a reload and must not
  outlive the tab or leak into a second one. Every access is `try`-wrapped; a blocked store is "no
  session", never a boot failure.
- `hubSessionCapability` state + `rememberHubSessionCapability` (`:2474-2485`) — seeded from the
  store at mount, replaced by every mint, cleared by sign-out and by the hub answering `401`.
- the identity bootstrap effect (`:3432+`) now takes `[hubEnv, hubSessionCapability]`, opens a fresh
  `MessageChannel` per bootstrap, hands the worker the capability through
  `HubSessionPortClientV1`, and drives the *existing* `startDirectorySessionRefreshV1` off
  `sessionPort.me(signal)`. `verifiedSessionAuthority` is therefore the hub's own
  `GET /auth/sessions/me` answer **for the capability the worker holds** — the same predicate every
  hub-authenticated operation is admitted against.
- `hubEnv` with **no** capability is no longer a fault: the blocking `SessionAuthorityNotice` renders
  only while a capability exists and its authority has not resolved (`:11125`). A shell with a hub
  and no session runs local-first and the badge offers sign-in, which is what AGENTS.md's
  local-first law requires and what the old broker-only assumption broke.
- `hubSessionPresence` (`:9017`) is now **derived** from `verifiedSessionAuthority` instead of
  mirroring the workspace overlay. That closes AU3 gap 7 (a session minted before the overlay was
  opened read `signedOut`) and it is what makes the badge an honest runtime probe: the sign-in
  affordance disappears exactly when the shell holds authority. `HubWorkspace`'s `onSessionChange`
  prop is deleted rather than left dangling.

**Worker side.** `🏪️store/👷️worker/🟦️.ts`:
- `hubSessionCapability` / `hubSessionOwner` / `hubSessionAdmission` / `hubSessionQueue` replace the
  proof state (`:646-656`). The owner/admission/serial-queue machinery is kept verbatim — it is what
  stops a retired owner observing its successor's response, and nothing about it was broker-specific.
- `installHubSessionCapability` (`:733`) is idempotent for the same capability and retires every
  document, directory and inference owner when a *different* one arrives, so one worker can never
  mix two principals' work. That idempotence is what lets a reloaded page re-hand the session it
  restored without tearing anything down.
- `hubSessionFetch` (`:748`) is the single authenticated lane: `${HUB_REQUEST_ROUTE_PREFIX}${path}`
  plus `Authorization: Bearer`. A `401` drops the capability rather than retrying, which is what
  surfaces re-authentication in the shell instead of a silent stall.
- `captureBrowserSessionOperationFence` (`:669`) lost the proof-TTL clause; the session's own
  `expiresAt` is the only deadline there was ever a reason to have.
- `attachHubSessionPort` (`:797`) keeps the private-port RPC shape; `initialize` now carries
  `capability` instead of `proof`, validated against `HUB_SESSION_CAPABILITY_PATTERN_V1`.
- `DirectoryClient`'s three constructions take `requestBaseUrl: ""` and `browserDirectoryRequest`'s
  allowlist matches bare hub paths; the route prefix is applied in one place.

**Contract.** `💻️os/🟦️.ts`: `HubSessionPortRequestV1`/`ResponseV1` +
`parseHubSessionPortRequestV1`/`parseHubSessionPortResponseV1` +
`HUB_SESSION_CAPABILITY_PATTERN_V1` replace the `BrowserBrokerPort*` pair. The module
`📇️directory/🪪️session-refresh/🌐️broker-port/` is renamed `🪪️session-port/` and its class is
`HubSessionPortClientV1`; its fixture/schema pair is renamed with it
(`semio.directory.hub-session-port-client-fixtures.v1`, `clearsCapability`).

**Persistence contract.** `📇️directory/🔐️sign-in/🟦️.ts` gained a `🔖️Capability` region:
`HubSessionCapabilityV1`, `parseHubSessionCapabilityV1`, `readHubSessionCapabilityV1`,
`writeHubSessionCapabilityV1`. The decoder refuses a tampered record, a token of the wrong shape and
— load-bearing — a capability minted by a *different* origin than the one being asked about.

**Port.** `🔗️HubConnection/🟦️.tsx`: `createHubConnectionFetchPortV1` takes `restoredCapability` and
`onCapability`; `mint` now parses through the production `parseHubSessionMintResultV1` (it used to
cast `JSON.parse(...).token`, which would have installed a proxy's error page as a session), `read`
announces `null` on `401`, `end` announces `null`. `useHubConnection` re-bootstraps once at mount
when the port restored a capability — presuming the session live only until the hub's own `me`
confirms it.

## 16. Hub readiness configuration actually used

DS1's blocker is real and unchanged in this tree: `artifactAuthority` never reports ready, so the
`fullyReady` branch of `localHubReadinessAdmitted` is unreachable. The narrowest legitimate
configuration that *does* come up — and the exact one C1c ran against — is the hub's own
`bootstrapSecuritySmoke` readiness boundary:

```
$ bun 🐍️c1c-hub-hold.ts 7501 /private/tmp/c1c-hub-eCgs
provisioned user1@semio.dev=01a0bbc1-126b-758b-a461-e6b9763fdb99
provisioned user2@semio.dev=01a0bbc1-63dc-7354-a4cc-14302245c2ff
[INFO] os-hub ready at http://127.0.0.1:7501
HOLD origin=http://127.0.0.1:7501 status=not-ready artifactAuthority={"ready":false}

$ curl -s http://127.0.0.1:7501/readyz
{"schema":"semio.hub.readiness/v1","status":"not-ready",…,"authentication":{"kind":"local-bootstrap-pipe-v1",
 "bootstrapReady":true,"publicSessionIssuance":true},"directory":{"ready":true},"storage":{"ready":true},
 "artifactCasBarrier":{"ready":true},"artifactPublication":{"ready":true},"artifactAuthority":{"ready":false},
 "adminAssets":{"ready":true},"features":{"openPlan":false,"openPlanExchange":false,"rebootstrap":true,…}}
```

**What that configuration can and cannot serve.** Auth (`/auth/sessions`, `/auth/sessions/me`,
`/auth/credentials`), the directory (`/directory/spaces`, `/directory/commands`, invites, the space
roster) and `/admin` are all live. Artifacts are not: `features.openPlan` is `false`, so a document
open plan — the first hub call any editor makes — is refused. That is the exact boundary between the
scenario's identity/space steps and its document steps, and it is DS1's to move.

**One boot fault found and recorded here because it costs a whole cycle.** Spawning the staged
`os-hub` binary directly with `OS_HUB_MODE=development` and no launcher pipe aborts the process:

```
thread 'tokio-rt-worker' panicked at tokio-1.52.3/src/runtime/io/driver.rs:196:23:
unexpected error when polling the I/O driver: Os { code: 9, … message: "Bad file descriptor" }
worker thread panicking; aborting process
```

A development hub completes an authenticated local-bootstrap handshake over an **inherited pipe on
fd 3**; without it the reactor polls a descriptor that does not exist. `startLocalHub` is the only
supported way to boot one (it opens `stdio: [ignore, …, "pipe"]`), which is what
`🐍️c1c-hub-hold.ts` now does. The message names neither the pipe nor the handshake, so it reads as a
tokio bug rather than a missing launcher contract.

## 17. Per-step observed results

Superseded by `## Session 5` below, which ran the identity path live and states exactly which
collaboration steps were reachable and which were not.

## 18. The three behaviours the brief requires and the scenario lacked

The harness now carries all three as steps 11, 12 and 13 (`COLLAB_E2E_STEP_NAMES`,
`🤝️collaboration/🟦️.ts:111-124`): per-user undo, a short connection loss, and two-writer convergence.
Whether they have been *run* is §S5.3.

## 19. Permanent target wiring

Unchanged from §11 and re-verified on the current tree: script route
`🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts`, nx target `collab-e2e`, launch row `🛠️dev🤝️os-collab-e2e`.
No new target was needed.

## 20. Honest gaps

See §S5.5.

## 21. Files changed by C1c

See §S5.6.

# Session 5 — C1c resumed, 2026-09-20 01:20→02:05

Worker C1c (resume). Everything below is measured; every capture named is on disk under
`🗑️generated/`. Inherited live from the predecessor and reused rather than rebuilt: the hub held by
`🐍️c1c-hub-hold.ts` (pid 4409, `os-hub` pid 5468) on `http://127.0.0.1:7501` with data root
`/private/tmp/c1c-hub-eCgs`, and its `animate` react dev serve on `http://127.0.0.1:7502`
(pid 43068/43140). Both were verified by `curl` before use, not assumed.

### E2E baseline (session 5)

**This is the block W3c gates on.** The 13-step `collabRunScenario` was **not** run, and the honest
reason is a build blocker owned elsewhere, not a scenario result:

| field | value |
|---|---|
| command attempted | `bun nx run @semio-tech/framework-os-dev:activate-s-react-dev` (the harness's own `collabActivateShellRuntime`, `🤝️collaboration/🟦️.ts:339`) |
| capture | `🗑️generated/c1c-activate-s-react.txt` (run 1, 13 min), `c1c-activate-s-react-2.txt` (run 2) |
| outcome | `error: Cargo artifact build failed: ✏️s/🔌️plugins/🧱️block/📦️packages/🦀️rust/Cargo.toml` |
| underlying errors | `error[E0502]: cannot borrow counters as immutable because it is also borrowed as mutable` → `could not compile semio-framework-trace`; `error[E0599]: no method named principal_kind found for struct PresencePeerReader<'a>` → `could not compile semio-framework-replication` |
| owner | peers' in-flight refactors (`principal_kind` is slice M6's agent-presence work). Preamble rule 3/13: not mine to revert. |
| hub mode used | `startLocalHub` + `bootstrapSecuritySmoke` readiness, `OS_HUB_CREDENTIAL_SIGN_IN=1`, `status=not-ready` with `artifactAuthority.ready=false` and `features.openPlan=false` (DS1's blocker, §16) |
| ports | hub `7501`; shells `7502` (`animate`, inherited) and `6108` (`home`, started this session, pid 4842) |
| steps 1-13 | **0 run.** Not "failed" — the two `s` shells the scenario needs cannot be staged while the workspace is red. |

What *was* observed live with two real humans is §S5.1 (identity, 19/19) and §S5.4 (the host-mode
finding that decides what a two-user run will need). No step of the ten/thirteen has yet been
observed passing in a browser, by C1, C1b or C1c.

### E2E run 2 (session 5)

The coordinator's instruction was to stop waiting for the `s` host and prove a shared document in the
cheapest shell that already serves. That was attempted and it produced a definite answer: **host mode
is not what gates a shared document — the hub is, and the gate is DS1's, measured exactly.**

| step | verdict | measured reason |
|---|---|---|
| 1 space created / replicated | not run | needs the Home table, host mode only (§S5.4 finding 2) |
| 2 share + open `/spaces/{id}` | not run | same |
| 3 create artifact, editor opens | **blocked at the hub** | `features.openPlan:false` |
| 4 live edit A→B | **blocked at the hub** | `features.openPlan:false` |
| 5 presence roster, 2 distinct colours | **blocked at the hub** | presence beats ride the document socket, which needs an open plan |
| 6 check-in | not run | needs 3 |
| 7 `/admin/api/connections` | not run | needs a connection, which needs 3 |
| 8 one-round-trip bound | **blocked at the hub** | needs 4 |
| 9 restart persistence / 10 in-flight edit | **blocked at the hub** | needs 3 |
| 11 per-user undo / 12 connection loss / 13 convergence | **blocked at the hub** | needs 4 |

**The gate, exactly.** `🌎️hub/🏗️bootstrap/🦀️.rs:9481`:

```rust
let open_plan_ready = artifact_authority.as_ref().is_some_and(|configured| configured.catalog.open_target_count() > 0);
```

and `:2540` / `:2739` refuse every document-open-plan and plan-exchange request when
`!state.readiness.features.open_plan`. So a document open is admitted only when the **trusted catalog
publishes at least one open target**. Every trusted-catalog directory in this tree is empty —
`.🧬semio/🌐hub/hub-dev/trusted-catalog`, `.🧬semio/🌐hub/c1b-warm/trusted-catalog` (C1b's warm
catalog never materialised) and this slice's own `/private/tmp/c1c-hub-eCgs` — so `openPlan` is
`false` on every hub this repo can boot today, for every shell, host mode or not. Capture
`🗑️generated/c1c-hub-document-admission.txt` (probe `🐍️c1c-hub-document-admission.ts`) shows the
live `features` line and the empty `/directory/spaces` listings from both signed-in humans.

**A real gate on the cheap path, found and removed.** Chasing "can a playground shell join a space
document" turned up the one path that does it without host mode — the sync card's `remote://` attach
— and it was broken at the root. `buildRemoteBackboneUri` encodes three parts
(`remote://<host>/<spaceId>/<documentId>`) and `parseRemoteBackboneUri` (`💻️os/🟦️.ts:159`) decodes
all three, but `attachSyncBackbone` (`🏛️ShellHost/🟦️.tsx:6454`) hand-rolled its own split:

```ts
const spaceId = slash > 0 ? rest.slice(slash + 1) || "default" : "default";   // ← "space-1/doc-a"
const documentId = syncDocumentId(targetSession, panel, hostMode);            // ← ignores the uri
```

`syncDocumentId` is `${pluginId}-${instanceId}` in a playground, so **two browsers attaching to the
same hub document opened two different documents in two mis-named spaces** and never shared a
replication session — which is precisely why joining one shared document looked like it needed host
mode. Fixed to use the canonical decoder: the uri's own `documentId` is used when present, the
`spaceId` is the single segment it actually is. It cannot be *observed* until `openPlan` opens,
and that is said plainly rather than claimed.

### E2E run 3 (session 5b, 06:15→07:0x)

Resumed after the ~03:00 session-limit cut. Housekeeping first: hub 7501 (`os-hub` pid 5468, up
5 h 51 m) and both serves (7502 animate, 6108 home) still answer; C1c's own
`activate-s-react-dev` chain (pid 34630, 4 h 16 m, no output since its generate phase) was **killed by
pid** as instructed — S2 owns the one cold `s` activation from here. No cargo was started by this
slice in session 5b (48 `rustc` processes were running for the rest of the fleet).

**The task: publish a minimal trusted catalog for one proven editor so the 13 steps can run.**
Attempted, and it stops at a specific, nameable place — not at "no artifacts".

What IS on disk and coherent (this was the open question, and the answer is better than expected):

| artifact | path | measured |
|---|---|---|
| component wasm | `✏️s/🔌️plugins/🗒️note/…/dist/component-release/semio_s_plugin_note.wasm` | 14 503 410 B, sha256 `f227eba2c443c965…`, built 09-19 19:56 |
| descriptor pack | `…/🔌️plugin/📦️packages/🟦️typescript/dist/release/🔌️plugin-modules/🗒️note/🛂️.descriptor.semio` | same build |
| descriptor json | same dir, `🔣️.json` | `hashes.wasmSha256 = f227eba2c443c965…` — **matches the component byte-for-byte**; `coreWasmSha256 = 7ea971e4…` matches the staged core |

So a fresh, self-consistent release triple exists for `🗒️note` (and for `🖍️draw` and `✒️writer`), and
it needs **no cargo at all**. `os-hub trusted-catalog publish` is compiled into the staged binary —
fed empty stdin it answers `ArtifactAuthority(Catalog("EOF while parsing a value"))`, i.e. it
dispatches and parses, it is not an unknown verb.

**Where it stops.** `TrustedBundlePackageV1` requires eleven fields
(`🔏️trusted-catalog/🧬️schema/🔣️.json`), and three of them cannot be produced outside the product's
own encoders:

1. `browserActor` is **required for every package** — a closed browser actor artifact with its own
   `sha256`, `policySha256`, `codegenPolicy` and `importInterfaces`, derived from the component by
   `buildClosedBrowserActorArtifactV1` (jco codegen). There is no such artifact on disk for `note`;
   only the gis path builds one today.
2. `component.blake3` — a blake3 digest of the component, alongside the sha256.
3. `profiles[].selectedClosureSha256` and `profiles[].generationId` are canonical digests over
   domain-separated, length-prefixed encodings (`selected_closure_digest`, `🦀️.rs:823-832`, over
   `b"semio/hub/trusted-profile-selected-closure/v1\0"` + a BE `u32` count + `append_document_open_catalog_field`
   per field; `trusted_profile_generation` likewise).

Reimplementing a security-critical canonical encoder inside a ticket probe to make an authority gate
open is the wrong shape of work, and a bundle whose digests I had reverse-engineered would be evidence
of nothing. So this was **not** done, and nothing was published.

**The cheap unblock, located exactly, for whoever owns it (DS1).**
`materializeTrustedStdioGisBundle` (`🌎️hub/📦️packages/🦀️rust/📜️script.ts:9407`) already performs
every one of those steps — component production, descriptor read, closed-actor derivation, all
digests, staging and `validateAndPublishTrustedStdioGisCandidate`. The only reason it cannot produce a
`note`-only catalog today is two hardcoded spots inside it:

```ts
const requests = [
  { pluginId: "stdio", cargoPackage: "semio-s-plugin-stdio", … },
  { pluginId: "gis",   cargoPackage: "semio-s-plugin-gis",   … },
];
…
if (request.pluginId === "gis") derivedActor = await buildClosedBrowserActorArtifactV1(component, …)
```

Parameterising that list (and deriving the actor for whichever package carries the open target) yields
a catalog that **does not contain the oversized `stdio` descriptor at all** — which is the descriptor
that fails `trustedBootstrapReadRegular` at `:9450` and is the whole of DS1's blocker. A `note`-only
or `draw`-only trusted catalog would open `features.openPlan` for outcome 3 while the stdio bound is
fixed separately, and its components are already built.

**Steps 1-13: still 0 observed.** Unchanged from run 2, for the unchanged reason — `openPlan` is
`false` on every hub in this tree. No presence screenshots exist; there is still nothing true to
screenshot.

### E2E run 4 (session 5b) — the generalisation, scoped against the real code, NOT landed

Instruction: generalise `materializeTrustedStdioGisBundle` into a request-list verb, derive the
browser actor for any plugin, publish a note+draw+writer catalog, open `openPlan`, run the 13 steps.
I read the function and its callers end to end before editing, and stopped before editing, for two
findings that change the cost — both measured, not estimated.

**1. The stdio+gis shape is not a parameter, it is the function's type.** Beyond the `requests` array
and the `if (request.pluginId === "gis")` actor line I named in run 3, the same closure is welded in at
six more places:

| place | what is welded | line |
|---|---|---|
| `projectTrustedBootstrapCodecsV1(stdio, gis)` | returns `Record<"gis" \| "stdio", Codec[]>`; both sources are hardcoded file paths (`🗄️stdio/…/native-codec-factories.json`, `🌍️gis/📇️native-codecs/🔣️.json`) | `:8759`, `:8857` |
| exactness law | `codecs.stdio.length !== 26 \|\| codecs.gis.length !== 2` | `:9489` |
| open target | the single `target` literal is gis Map (`s.gis.gismap`, `gis2d-main`, `rendererTarget: "wasm"`) | `:9497` |
| `selectedClosure` / `packageSummary` | two hand-written entries, gis then stdio, with literal `codecCount: 2` / `26` | `:9493`, `:9509` |
| `file(plugin: "gis" \| "stdio", …)` | the package emitter's parameter type, and `openTargets: plugin === "gis" ? [target] : []` | `:9540` |
| rotation reader | `trustedBootstrapReadCurrentBundle` refuses any bundle whose `packages?.length !== 2` | `:9612` |

and **three source-text guard tests assert on the literals** — `processConforms` matches
`"await materializeTrustedStdioGisBundle("` (`:7192`), the retained-GIS proof requires the string
`'"packages/gis/browser/closed-actor.mjs"'` and `gis.browserActor.sourceComponentSha256 !== gis.component.sha256`
(`:8245-8249`), and `:8427-8431` pins the exact `validateAndPublishTrustedStdioGisCandidate(...)` call
text in three callers. Generalising means rewriting those laws too, so they keep proving the same
property for an arbitrary package set.

**2. Even generalised, it cannot reuse the artifacts I found.** `produceFreshComponentV1`
(`🖨️describe/🏭️fresh-component/🟦️.ts:256-259`) runs `cargo rustc … --target wasm32-wasip2 --profile
wasm-release` into a private `CARGO_TARGET_DIR` with `CARGO_INCREMENTAL=0` — a cold build per package,
sharing nothing. That is the point: "fresh" is the provenance property the trusted catalog exists to
have, so feeding it the release wasm already on disk (run 3's table) would defeat exactly the
guarantee being published. note+draw+writer therefore costs **three cold wasm-release component
builds**, on a machine that had 48 `rustc` running and whose cargo queue deadlocked for four hours
this morning.

**Why I stopped rather than landed it.** This is a rewrite of the central function of a file DS1 is
editing right now for descriptor regeneration (preamble rules 3 and 13), its correctness is carried by
source-text laws I would have to rewrite in the same pass, and its payoff is gated behind three cold
cargo builds I have no budget for and was told not to start. A half-landed generalisation in that file
is worse than none: it would collide with DS1's hunks and leave the laws asserting on text that no
longer exists. Recorded here so the next owner starts from the six places above rather than rediscovering
them.

**Steps 1-13: still 0 observed.** `openPlan` is still `false`; no catalog was published; no presence
screenshots exist.

### E2E run 5 (session 5b) — signed-in `s` host, two humans, observed

S2's cold `s` boot (pid 26173, `http://127.0.0.1:6070/`, 60/60 staged — **not touched**) has no hub
env: its serve carries `S_OS_PORT=6070` and no `S_HUB_URL`, and `/_semio/hub/readyz` there answers
`text/html` (the SPA fallback), so no hub lane exists on it. A **second serve of the same staged
output** was started — serve only, no activation — on a free port:

```
S_OS_PORT=6071 S_HUB_URL=http://127.0.0.1:7501 bun 📜️script.ts serve s react dev    # pid 33969
$ curl -sD- http://127.0.0.1:6071/_semio/hub/healthz → 200, content-type: application/json
```

Probe `🐍️c1c-s-host-probe.mjs` (new, permanent), capture `🗑️generated/c1c-s-host.txt`, screenshots
`c1c-s-host-user{1,2}-{before,after}.png` and `c1c-s-host-presence-user{1,2}.png` (10 in all with the
identity and census runs).

**A probe bug worth naming, because every emoji-path probe in this repo can have it.**
`new URL("./🗑️generated/x.png", import.meta.url).pathname` percent-encodes **every** emoji segment,
so playwright silently created a whole stray tree at
`/Users/ueli/Documents/semio/.%F0%9F%A7%ACsemio/%F0%9F%A6%91%EF%B8%8Frepo/…/%F0%9F%97%91%EF%B8%8Fgenerated/`
and wrote all ten screenshots there — no error, and the ticket folder simply had no images. They were
moved into `🗑️generated/`, the stray tree was removed (verified empty first), and all three C1c probes
now use `fileURLToPath(new URL(…))`. Any probe that spells a screenshot or capture path with
`.pathname` under this repo's emoji directories is writing outside the ticket folder.

| # | check | result |
|---|---|---|
| 1 | the `s` host renders its hub badge signed out | **PASS** (`signInOffered: 1`) |
| 1 | Home refuses its surface without a signed-in human | **PASS** — `identityFault: true`, `homeSurface: false` |
| 2 | the human is signed in inside `s` | **PASS** (`signInOffered: 0`) |
| 2 | **Home no longer answers `s.home.session-identity-required`** | **PASS** — `identityFault: false` |
| 2 | the host app surface is published | **FAIL** — `s-home-main` still absent after 120 s |
| 2 | the command palette tab is present | **PASS** |
| 3 | both humans signed in at once, two contexts, one `s` host | **PASS** (`[0, 0]`) |
| 4 | presence rosters | 0 and 0 — expected without `openPlan` (beats ride the document socket) |

**What this settles.** The transition is observed on the SAME page with no reload, for both humans:
`s.home.session-identity-required` is present before the sign-in and gone after it. That is C1c's
ShellHost re-assembly fix (§S5.4 finding 1) working at runtime for the first time, and it is the
blocker S2's foreign-kind probe stops on. The `s` host also accepts two distinct signed-in humans
concurrently — outcome 3's identity half, now inside the real host rather than a playground.

**What it does not settle, precisely.** Clearing the fault is necessary but **not sufficient**:
`refreshUi` talks to the actor that already stopped (`actor space#1 stopped … status=idle`), so the
box disappears while `s-home-main` is never published. The missing half is re-**establishing** the
session, not refreshing it — and the file already has that recovery shape: `establishPrimaryWithShardRetry`
(`🏛️ShellHost/🟦️.tsx:3742`) and the "a LIVE session whose worker was taken down is recoverable
exactly the way a killed boot is" path at `:3759`. The change is to run that recovery when the
**human** component of the session key changes (every surface assembled under `sessionIdentity:
undefined` is invalid anyway), rather than only `refreshUi`. It was **not** made: it is a structural
change in the region S2 edited this session (`:3675+`), each verification cycle against a live `s`
shell is ~12 min, and I had no budget left to verify it — an unverified structural edit there is worse
than none. Whoever takes it can re-run `🐍️c1c-s-host-probe.mjs` unchanged to judge it; check 2's
surface row is the pass/fail.

Document steps (live edit, undo, convergence, restart) remain blocked on DS1's published catalog and
were not attempted.

### S5.1 Two humans, two browsers, one hub — observed, 19/19

`bun 🐍️c1c-identity-probe.mjs http://127.0.0.1:7502 http://127.0.0.1:7501`, capture
`🗑️generated/c1c-identity.txt`, screenshots `c1c-identity-user1.png` / `c1c-identity-user2.png`:

```
PASS 1 the badge is rendered / offers sign-in while signed out / no blocking notice / nothing minted before a human asked
PASS 2 the badge no longer offers sign-in · POST /auth/sessions 200 · GET /auth/sessions/me 200 · capability remembered for this context
PASS 3 the reloaded shell is signed in again · no second session was minted · it re-read the hub's own authority · the user id survived
PASS 4 the second context starts signed out · the second human is signed in · the first human is still signed in
PASS 4 the two browsers hold two DIFFERENT hub principals
PASS 5 the worker reached the hub through its own same-origin lane · every worker hub request carried a session bearer · no retired broker proof
c1c-identity: all checks passed
```

The two principals are the hub's own: `01a0bbc1-126b-758b-a461-e6b9763fdb99` (user1@semio.dev) and
`01a0bbc1-63dc-7354-a4cc-14302245c2ff` (user2@semio.dev), provisioned through
`os-hub credential set` and signed in through the shell's own form — no launcher, no relay, no
`#semio-broker=` fragment on any request (check 5 asserts that). This is C1b §12.1 closed: **the
React shell now has a two-user identity path and it was observed at runtime.**

Note the two humans share ONE dev server origin. That is a consequence of §15's rework, not an
accident: identity is now a `sessionStorage` capability per browsing context, so two contexts on one
origin are two humans. The harness's per-user `S_OS_PORT` pair is no longer required for identity.

### S5.2 Root fix — one transient `me` failure permanently signed a shell out

**Found live, not by reading.** On the first two-user run, check 4 "the first human is still signed
in" FAILED: user1's shell, signed in and refreshing happily for minutes, was signed out the moment
user2's context signed in on the same dev server. Isolated with `🐍️c1c-session-lifetime.mjs`
(capture `🗑️generated/c1c-session-lifetime.txt`): one shell alone held its authority for the whole
90 s window, refreshing `GET /auth/sessions/me` every 5 s, 45/45 samples `signInAffordance=0`. So it
was not a lifetime or a shared-worker defect — it was the second page's boot (≈900 module
transforms) queueing the first shell's proxied `me` past its deadline.

`startDirectorySessionRefreshV1` treated **every** non-200 as a refusal:

```ts
if (response.status !== 200) throw new Error("directory.session-authority.unavailable");
…
} catch { close(); options.onUnavailable(); }      // ← closed the loop for ever, on one hiccup
```

`onUnavailable` nulls `verifiedSessionAuthority`, and the loop was closed, so nothing ever retried.
A shell was signed out for the rest of the page's life by a queued request — and the brief's "short
connection loss with catch-up" could never have passed for the same reason.

Fixed at the root, `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️session-refresh/🟦️.ts`:

- `DIRECTORY_SESSION_AUTHORITY_REFUSAL_STATUSES = [401, 403]` (`:11`) — the only answers *about the
  session*. An expired parsed authority is the other one.
- everything else (transport throw, gateway status, a proxy's HTML error page) is transient: retried
  on `directorySessionRefreshRetryDelayMsV1` (`:38`), 500 ms doubling to a 5 s ceiling, **keeping the
  authority already verified**, bounded by that authority's own `expiresAt` — once it has passed
  there is nothing left to ride out and the next failure retires it.
- new optional `onDegraded(boolean)` reports the transient window's two edges, wired in
  `🏛️ShellHost/🟦️.tsx:3537` to `setIdentityOffline` only — the session is *not* torn down.

Laws added to the existing corpus (`🪪️session-refresh/🔣️.json` + `🧬️.schema.json` gained
`transientFailures` and `retryDelaysMs`; `refusals` narrowed to `http-401`/`http-403`/`expired`), in
`🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts`: each of five transient shapes keeps the authority,
reads on exactly the fixture's backoff, reports `[true]` then `[true,false]` degraded edges and never
calls `onUnavailable`; a loss outliving the session's own deadline does retire it; the delay function
matches the fixture and refuses attempt 0.

```
$ bun ./📜️script.ts test long "👷️worker" -t "refreshes Shell session authority"
Test Files  1 passed (1)     Tests  1 passed | 111 skipped (112)
```
capture `🗑️generated/c1c-session-refresh-test.txt`. With it in place the two-user probe went
18/19 → **19/19** (§S5.1) against the same hub and the same shells.

### S5.3 Harness fix — the sign-in click was ambiguous

`collabSignIn` (`🤝️collaboration/🟦️.ts:265`) clicked `button[type="submit"][aria-busy]`, which
resolves to **two** elements in the live workspace overlay ("Sign in" and a disabled "Join space") —
playwright strict-mode violation, so every harness run would have died in sign-in before step 1. The
same bug was in `🐍️c1c-identity-probe.mjs`. Both now click
`button[type="submit"][aria-label="Sign in"]`. Found by running it, not by reading it.

### S5.4 Why the collaboration steps still cannot run — and exactly what it will take

With the identity path live, the obvious next move was to skip the 60-crate `s` activation and drive
the space Home from the `home` playground variant, whose activation receipt is fresh on disk
(`dist/runtime/react/dev/home/activation/🔣️receipt.json`, `space` @ `ad15ab32…`, 01:08 today). A
`home` react dev serve was started against the live hub (port 6108, pid 4842,
`🗑️generated/c1c-home-serve.txt`) and censused signed-in (`🐍️c1c-home-census.mjs`, capture
`🗑️generated/c1c-home-census.txt`, screenshots `c1c-home-census*.png`). The shell boots, signs in and
renders `#s-presence-peers` — and the Home app refuses its own surface:

```
PluginRuntime: actor space#1 stopped without publishing requested UI surfaces
  (missing=["1:s-home-main"], status=idle,
   faults=["plugin.internal: s.home.session-identity-required: current host session identity is required"])
tableHosts: 0   rows: []
```

Two distinct findings came out of chasing that, both verified in the source after being seen live:

1. **The shell never re-assembled a plugin surface when a human signed in.** `resolvedTargetViewState`
   (`🏛️ShellHost/🟦️.tsx:4595`) stamps `sessionIdentity` from `identityRef.current` at plugin-call
   time, but the effect that re-renders the session was keyed on
   `pluginId:app:instanceId` only (`:5376`). A shell that boots local-first — the ordinary state:
   hub configured, nobody signed in yet — assembled Home identity-less, and nothing re-ran it after
   sign-in. **Fixed**: the key now carries the signed-in human
   (`…:${identity?.userId ?? ""}:${identity?.displayName ?? ""}`), so signing in, signing out and
   switching humans each re-assemble the active session's surfaces.
2. **The `home` variant can never show the spaces table, by design.** `resolvePluginHostConfig`
   (`🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:3186`) returns `undefined` for any playground row that
   names an `app`, and `🪐️space`'s `home` row names `s.space.home@1/*#editor`. No host config ⇒
   `hostMode === false` ⇒ no `hostPlugin`/`landingApp` ⇒ the directory bootstrap effect
   (`🏛️ShellHost/🟦️.tsx:3563`, the one that opens the Home owner *with* `identity`) is guarded off
   entirely. Only the `s` variant is the host. So fix 1 is landed but **not yet observable**: proving
   it needs the same `s` shell the scenario needs.

That is the whole remaining chain for outcome 3: `semio-framework-trace` + `semio-framework-replication`
green ⇒ `🧱️block` builds ⇒ `activate-s-react-dev` completes ⇒ two `s` shells ⇒ steps 1, 2, 5, 6, 7
are reachable on this hub today, and steps 3, 4, 8, 10-13 additionally need DS1's
`features.openPlan` (§16) because every one of them opens a document.

### S5.5 Honest gaps

- **The 13-step scenario has never been run.** Not by C1, C1b or C1c. §"E2E baseline (session 5)"
  states the exact blocker and capture.
- **Presence colours, cursors and rosters with two users: not observed** (coordinator addendum 2).
  `#s-presence-peers` exists in the React shell and reads "No one else is here" for a lone signed-in
  human (`c1c-home-census.txt`); the roster/colour/`online` assertions need two shells in ONE space,
  which needs the host variant. `collabPresenceColors` (`🤝️collaboration/🟦️.ts:466`) already reads
  *computed* border colours, so the distinctness assertion is not vacuous when it does run. No
  `c1c-presence-*.png` exists, deliberately: there is nothing true to screenshot yet.
- **Fix 1 of §S5.4 is landed but unobserved** for the reason in that section.
- **Six worker tests were red from the predecessor's broker→capability rework; five are fixed, one is
  left.** They all failed with `Error: hub session rebootstrap required` from `hubSessionFetch`
  (`👷️worker/🟦️.ts:768`) because the `gis map inference port` harness still installed a retired
  64-hex *proof* where the worker now demands a `session.v1.…` capability
  (`HUB_SESSION_CAPABILITY_PATTERN_V1`). Fixed by finishing that migration in the corpus, not by
  loosening the pattern: `🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json`'s `successorProof` is now
  `successorCapability` with a capability value, `🧬️schema/🔣️.json` gained
  `GisMapInferencePortSessionCapability`, the harness installs `WORKER_LAW_CAPABILITY`, the three
  header assertions compare `Bearer <capability>` (they compared a bare proof), and the retirement
  law asserts the session port's own statuses — `401` when the hub said the capability is gone, `503`
  otherwise (`👷️worker/🟦️.ts:838`); the broker-era `428` it asserted has no producer left in the
  worker at all.

  ```
  $ bun ./📜️script.ts test long "👷️worker" -t "session"
  before: Tests  6 failed | 11 passed | 95 skipped (112)
  after:  Tests  1 failed | 16 passed | 95 skipped (112)
  ```
  capture `🗑️generated/c1c-worker-session-tests.txt`.

  The last one was resolved by taking the contract decision rather than fitting the assertion:
  **retirement follows the hub's authority over the session, not the shape of one answer.** `401`
  (the hub saying the capability is gone, `👷️worker/🟦️.ts:779-782` → `clearHubSessionCapability`)
  and a capability replaced mid-flight (the human's own switch) retire every owner, bump
  `directorySessionEpoch` and drop the open artifacts. A `201` or an unparsable body is the link
  failing to *confirm* the session and says nothing about it: `acceptBrowserSessionAuthority` refuses
  the read — the caller sees `503` and retries — and the authority already accepted is kept. That is
  the same law §S5.2 established one layer up in the shell's revalidation loop; before this slice both
  layers signed a shell out for ever on one hiccup, and the old assertion encoded exactly that
  broker-era behaviour. Final: **17 passed, 0 failed** on `-t session`.

- **Eight `backbone-worker offline resilience` laws plus `DirectoryEventPageBootstrapV1` are red and
  are NOT this slice's doing** — verified, not assumed: run in isolation with `-t "offline resilience"`
  (so none of the tests C1c touched execute at all) they still fail 8/8
  (`🗑️generated/c1c-worker-offline-isolated.txt`); the full file is 103 passed / 9 failed
  (`c1c-worker-suite.txt`). They fail on `browser directory operation denied` and missing
  `open-plan`/`manifest` routes — the same proof→capability rework debt, in a different test family,
  and no product file the worker uses was edited by this session.
- **A mid-edit hub restart** (steps 9/10) was not attempted: restarting the hub with no scenario able
  to reach step 1 proves nothing.

### S5.6 Files changed by C1c in session 5

| File | Change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️session-refresh/🟦️.ts` | transient-vs-refusal split, bounded retry backoff, `onDegraded`, `directorySessionRefreshRetryDelayMsV1`, `DIRECTORY_SESSION_AUTHORITY_REFUSAL_STATUSES` |
| `…/🪪️session-refresh/🔣️.json`, `…/🧬️.schema.json` | `transientFailures` + `retryDelaysMs` corpus; `refusals` narrowed to the three authoritative shapes |
| `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts` | new laws for the transient window, the degraded edges, the expiry bound and the delay function; the `gis map inference port` harness finished its proof→capability migration (5 of 6 red laws green) |
| `🧰️framework/🛍️products/💻️os/🧫️fixtures/💡️gis-map-inference-port-v1/🔣️.json`, `🧰️framework/🛍️products/💻️os/🧬️schema/🔣️.json` | `successorProof` → `successorCapability` with a `session.v1.…` value and its own `$def` pattern |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `onDegraded` → `setIdentityOffline` without retiring the session (`:3537`); the session-refresh key carries the signed-in human (`:5383`); `attachSyncBackbone` decodes `remote://` with `parseRemoteBackboneUri` so the uri's own space and document ids survive (`:6454-6475`) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` | `collabSignIn` clicks the unambiguous `aria-label="Sign in"` submit |

Ticket folder (not product code): `🐍️c1c-identity-probe.mjs` (selector fix),
`🐍️c1c-session-lifetime.mjs` (new), `🐍️c1c-home-census.mjs` (new), captures
`🗑️generated/c1c-identity.txt`, `c1c-session-lifetime.txt`, `c1c-session-refresh-test.txt`,
`c1c-home-census.txt`, `c1c-home-serve.txt`, `c1c-activate-s-react{,-2}.txt`, screenshots
`c1c-identity-user{1,2}.png`, `c1c-home-census{,-reloaded}.png`.

Processes started by this session, by pid: `home` react dev serve **4842** (port 6108),
`activate-s-react-dev` **77973** (run 1, exited on the `🧱️block` cargo failure after 13 min) and
**34630** (run 2, still in its generate phase at hand-off, capture
`🗑️generated/c1c-activate-s-react-2.txt` — check it before starting a third). The inherited hub
(4409/5468, port 7501, credential sign-in on, both humans provisioned) and the animate serve (43068,
port 7502) were left running: **the successor can re-run `🐍️c1c-identity-probe.mjs` against them in
under two minutes** rather than rebuilding any of it.

Next worker's shortest path, in dependency order — and the order matters, because run 2 proved the
hub is the binding constraint, not the host:

1. **DS1's `openPlan` first.** Until `artifact_authority.catalog.open_target_count() > 0`
   (`🌎️hub/🏗️bootstrap/🦀️.rs:9481`) no browser can open a shared document on any hub in this tree,
   so steps 3-5 and 8-13 are unreachable no matter which shell is served. This is the single
   highest-value unblock for outcome 3.
2. **`s` activation** for steps 1, 2, 6 and 7 (the Home/Space tables, which need host mode). Watch
   `c1c-activate-s-react-2.txt`; if it dies on `semio-framework-trace` /
   `semio-framework-replication` again, that is the peers' tree — wait, do not patch. S2's healthy-set
   rule (`📓️s2-cold-s-boot-and-foreign-kind-open.md` §2.1) now lets an activation exclude a broken
   component instead of aborting, which is why run 2 got past where run 1 stopped. S2 had no `s`
   receipt of its own at 01:46, so there was nothing to share; check again before starting a third
   activation.
3. With both: `serve s react dev` (one port is enough — see §S5.1), sign the two provisioned humans
   in through the badge, and the whole 13-step scenario is reachable. Verify the §S5.4 re-assembly fix
   there — Home must stop answering `s.home.session-identity-required` once a human signs in — and the
   run-2 `remote://` fix by attaching two contexts to one `remote://127.0.0.1:7501/<space>/<doc>`.
