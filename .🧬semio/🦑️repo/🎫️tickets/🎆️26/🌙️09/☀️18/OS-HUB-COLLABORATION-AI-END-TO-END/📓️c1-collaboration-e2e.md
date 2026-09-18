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
