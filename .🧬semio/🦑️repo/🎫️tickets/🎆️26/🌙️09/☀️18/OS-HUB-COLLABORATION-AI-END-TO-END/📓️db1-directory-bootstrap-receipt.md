# DB1 — the directory bootstrap must settle its typed operation, not read the admission

Slice DB1 (session 6, 2026-09-20). Outcome 3, first step: **after sign-in the `s` Home lists the
signed-in user's spaces from the hub directory, and entering a space lists its documents.**

Spec inherited from `📓️c2-two-user-collaboration-on-ready-hub.md` §3 (the named-but-not-fixed item),
`📓️s4-space-studio-and-foreign-kind-open.md` §2.5, `📓️s5-spawned-app-windows-in-s.md`
(`subscribeOperationCompletions` per spawned instance) and `📓️s3-…` (probe conventions).

Status legend: **measured** = this slice ran it and captured the output; **unverified** = read from
source only.

## 0. tl;dr

**Root-fixed in TypeScript only, no guest change and no re-stage.** The directory bootstrap now drives
`applyDirectoryEventPage` to its SETTLED typed-operation receipt through
`subscribeOperationCompletions`, and the runtime's completion now carries the operation's terminal
output at all — it was decoded off the wire and then thrown away. Laws green
(`🗑️generated/db1-bootstrap-laws.txt`): `directory-home-bootstrap-oracle: checks=33 clean`,
**14/14** bootstrap laws, **2/2** PluginRuntime completion-delivery laws.

## 1. The defect, restated from the wire

Three separate things had to be true for Home's space table to stay empty, and all three were:

1. **The receipt was read off the admission.** `applyDirectoryEventPageBootstrapV1` did
   `parseDirectoryProjectionReceiptV1(response.output)` on the FIRST `handleAction` reply. Since S4
   migrated the verb to `InteractiveJobClassification::Migrated` it is job-routed, so that reply is
   the typed-operation handle `{"operationId":"64","generation":"0"}`
   (`🔌️plugin/🦀️.rs:28184`/`:28396`) and never the verb's result. The parse answered `null`, the
   owner was closed with `directory-bootstrap.receipt-mismatch`, and the next page threw
   `app-channel.disposed` — C2 §3 measured exactly this output/expected pair.
2. **The receipt was in the shell already, and was dropped.** The guest emits it as an `AppEvent`
   (`…/🎮️commands/📬️apply-directory-event-page/🦀️.rs:22`) → lane-7 typed-operation result page →
   `consumeTypedOperationEffects` tags it `typed-operation-terminal-output`/`-pending-output`
   (`🔌️PluginRuntime/🟦️.tsx:795`) → the drain parks it in `pendingCompletionEffects`. The completion
   subscriber then **filtered exactly those tags out** and published a `PluginOperationCompletion`
   with `uiScope`/`historyPatch`/`requestedEffects` only. The value the verb exists to produce had no
   carrier on the only lane that delivers it.
3. **A second subscriber on one instance would have seen nothing anyway.** Each
   `subscribeOperationCompletions` call registered its OWN `AppChannelClient.onOperationCompleted`
   listener, and each such callback `delete`s `pendingCompletionEffects` for the instance. The shell
   really does subscribe twice on the visible Home instance — the session completion pass
   (`🏛️ShellHost/🟦️.tsx:6297`) and now this bootstrap, both on `session.instanceId` — so whichever
   ran first would have taken the effects and the receipt and left the other empty.

Everything else on the lane was already correct: the drain (`drainTypedOperations`) advances the
operation, `AppFrame::OperationCompleted.operation` is the same id the admission names
(`TypedOperationCompletion.operation`, `🔌️plugin/🦀️.rs:14223`), and the shell already had
`startedTypedOperationId`/`awaitOperationSettle` for precisely this correlation.

## 2. The fix

Host-side (TypeScript) only. No `🪐️space` guest change, no wasm build, no re-stage — vite picks it up
on the running serves.

| # | file | change |
|---|---|---|
| 2.1 | `…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `PluginOperationCompletion` gains `terminalOutput`, read out of the accumulated completion leftover by the new `typedOperationTerminalOutputV1` (both `terminal-output` and `pending-output` spellings — across a multi-turn drain the receipt page and the TERMINAL page ride different turns, and the completion frame IS the terminal witness; >1 is a contract violation, exactly as in `invocationFromFrames`) |
| 2.2 | same file | `subscribeOperationCompletions` now fans out from ONE channel registration per instance (`completionFanouts`), so the leftover is drained once and delivered to every subscriber; the registration is dropped with the instance in `releaseInstanceMaps` |
| 2.3 | `…/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx` | `startedDirectoryOperationIdV1` — the ONLY thing read out of the admission is its operation id |
| 2.4 | same file | `watchDirectoryOperationV1` subscribes BEFORE the dispatch (a completion can win the race against the admitting reply — it is remembered and the wait resolves from that memory), resolves on the matching `operation`, and is disposed in `finally` |
| 2.5 | same file | `applyDirectoryEventPageBootstrapV1` parses the receipt from `await settle.terminalOutputOf(operation)`; an admission that started no operation is the new fault `directory-bootstrap.operation-unstarted` |
| 2.6 | same file | retry is now typed: only a `SemioFaultError` whose `fault.retryable` is set (the wire's own flag, `⚠️diagnostic/🦀️.rs`'s `with_retryable`, `false` by default) or a lost terminal publication re-offers the page; anything else stops the lane with the guest's own code, which `data-directory-bootstrap-code` already surfaces. This is what ends S4's `31× event-page?after=0` storm class for permanent refusals |
| 2.7 | same file | `DIRECTORY_BOOTSTRAP_SETTLE_DEADLINE_MS = 30_000` bounds a completion frame that never arrives. **No polling loop**: the runtime's own continuation drain advances the operation, so this timer only fires when the delivery was lost |

## 3. Laws

`bun ./📜️script.ts directory-home-bootstrap-check` (permanent gate, already registered in
`📋️project.json` and in the root `📜️script.ts`), capture `🗑️generated/db1-bootstrap-laws.txt`:

```
directory-home-bootstrap-oracle: checks=33 clean
Tests  14 passed (14)                 ← 🧪️tests/📇️directory-home-bootstrap
Tests  2 passed | 130 skipped (132)   ← PluginRuntime in-source completion delivery
```

The source oracle used to PIN THE DEFECT — it asserted
`owner.includes("parseDirectoryProjectionReceiptV1(response.output)")`. It now asserts the opposite
(neither `response.output` nor `admission.output` is parsed as a receipt), that the subscription opens
before the dispatch, and that the runtime publishes `terminalOutput`; 29 → 33 checks.

New/rewritten vitest laws:

| law | pins |
|---|---|
| `reads only the typed-operation handle out of an admitting reply` | `startedDirectoryOperationIdV1` accepts `{operationId:"64"}`, refuses a receipt and ten hostile shapes |
| `never ACKs the admitting reply and ACKs only the settled operation's terminal receipt` | after the admission resolves: no post, still `pending`, duplicate page refused; the subscription opened before the dispatch; a completion for ANOTHER operation settles nothing; the matching completion produces the ACK and unsubscribes |
| `settles a completion that wins the race against its own admitting reply` | the remembered-completion path |
| `refreshes the visible projection before publishing the ACK` | `["refresh", "directory-bootstrap-ack"]` ordering survives the settle |
| `re-offers the page only on a typed transient refusal and stops on a permanent one` | **the refused page**: `retryable:true` → `retrying` + reject, owner alive; `retryable:false` → `fault` carrying the guest's own code + close + destroy |
| `re-offers the page when the terminal publication never arrives` | the deadline is transient, the owner survives, the subscription is released |
| `stops when the admitting reply started no typed operation at all` | `directory-bootstrap.operation-unstarted` |
| `closes and destroys on an exact receipt mismatch` | now driven through a published completion |
| `suppresses a late receipt after cancellation` | the settle never outlives its owner (an abort rejects the wait instead of hanging it) |
| `hands one completion its own effects exactly once and never the invocation's` (PluginRuntime) | `terminalOutput` is published, and a SECOND subscriber on the same instance receives the identical completion |

## 4. Measured live

### 4.1 Hub 7611 aborts before a signed-in shell can read anything — 3 reproductions by this slice

The brief's target (serve 6190 bound to hub 7611) could not be driven: **hub 7611 was already dead
when this slice reached it**, and every restart aborted again within seconds of the first sign-in.

Restarted with C2's recipe (`📜️c2-hub-restart.sh`'s exact command line and environment —
`OS_HUB_CREDENTIAL_SIGN_IN=true SEMIO_BUILD_BUDGET_MS=1800000`, `🐍️ds1-hub-hold.ts 7611
.🧬semio/🌐hub/gm1-boot <binary>` — with a DB1 capture path so C2's own log is not overwritten).
Captures `🗑️generated/db1-hub-restart.txt`, `db1-hub-7611.txt`, `db1-journey-{1,2,3}.txt`,
screenshot `db1-aborted.png`.

| run | binary | reached | aborted on |
|---|---|---|---|
| 1 | GM1's `⚡️cache/cargo/target-gm1/debug/os-hub` (C2's) | `status: ready`, runId `a9e8d7fb…` | `semio-pool-worker-5`, right after `server.auth.session.read outcome=ok` |
| 2 | HS1's newer `⚡️cache/hs1/os-hub-gm1` (built 22:03, copied + re-signed to `⚡️cache/db1/os-hub`) | `status: ready`, runId `7292dda8…` | `semio-pool-worker-3`, same place |
| 3 | same | never reached ready | `ArtifactAuthority(DeadlineExceeded)` — a slow boot under fleet load |

```
{"event":"server.auth.session.mint","outcome":"ok","principal":"user:01a0c00d-…","durationUs":3112784}
{"event":"server.auth.session.read","outcome":"ok","principal":"user:01a0c00d-…","durationUs":364}

thread 'semio-pool-worker-3' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

**Two facts this adds to C2 §4, for HS1.** (a) The abort needs **no document socket at all** — it
follows the FIRST `server.auth.session.read` after a mint, on a plain browser sign-in; C2's crash #1
was read as document-socket-specific, and it is not. (b) HS1's newer binary (22:03) **still aborts**,
so whatever is staged there does not cure it. From the shell's side it appears as
`26× GET /_semio/hub/auth/sessions/me → 500` while the direct `GET http://127.0.0.1:7611/auth/
sessions/me` that preceded the abort answered 200.

Two of this slice's own dead holders (2482, 3684, both with no surviving `os-hub` child) were killed
by pid. C2's holder 83161 and every peer serve were left alone. **Hub 7611 is down at hand-off**; one
`zsh 📜️c2-hub-restart.sh` brings it back (and it will abort again on the first sign-in).

### 4.2 The fix's behaviour change, measured on the live shell (serve 6071 → hub 7501)

Serve 6071 (`S_HUB_URL=http://127.0.0.1:7501`, pid 34308) and hub 7501 are S4's own pair — the shell
where §1's defect was named — and both were alive. `🐍️db1-home-space-journey.mjs` (new, permanent,
parameterised) signs in as `user1@semio.dev` and reads the bootstrap notice, the Home table's rows and
the space index. Captures `🗑️generated/db1-journey-{7501,7501b,final-7501}.txt`, screenshots
`db1-home.png`, `db1-space.png`.

| | S4/C2 measured, before | this slice, after |
|---|---|---|
| bootstrap notice | `fault` · `directory-bootstrap.receipt-mismatch` ("Directory update stopped") | **`pending` · "Updating directory through sequence 9"**, no code, owner alive |
| the owner | closed; the next page threw `app-channel.disposed` | never closed — 4 pages offered and applied over 100 s |
| `event-page/v1?after=N` | 31× in 60 s (S4 stage 2), then a dead owner | **4–5×**, one per settle deadline |
| the admission | parsed as a receipt → `null` | read as the handle: `[DB1PROBE] admission output={"generation":"0","operationId":"64"} operation=64` |

So the lane no longer dies on the admission, and the shell now waits for the operation exactly as the
brief asks. **It is still `pending`, not `idle`** — and the reason is §4.3, which is one stage further
down and is NOT the bootstrap.

### 4.3 The next stage, measured: the mounted operation never publishes its terminal

Temporary instrumentation in `drainTypedOperations` and in the bootstrap's completion receiver (added,
measured, **removed** — `grep -c DB1PROBE` is 0 in both files; captures
`🗑️generated/db1-journey-drain{,2,3,4}.txt`):

```
[DB1PROBE] turn instance=2 action=applyDirectoryEventPage status=more-work
[DB1PROBE] admission output={"generation":"0","operationId":"64"} operation=64
[DB1PROBE] drainpoll instance=2 n=1 status=more-work frames=0 leftoverTags=[]
[DB1PROBE] turn instance=2 action=applyDirectoryEventPage status=more-work     ← +30 s, my re-offer
[DB1PROBE] admission output={"generation":"0","operationId":"65"} operation=65
[DB1PROBE] turn instance=2 action=applyDirectoryEventPage status=more-work
[DB1PROBE] admission output={"generation":"0","operationId":"66"} operation=66
```

Read exactly:

- the command turn ends `more-work`, so `drainTypedOperations` IS armed (it is not the "no driver"
  case);
- the drain runs **exactly one poll**, which itself reports `more-work`;
- `drainTypedOperationTurns` then **never returns and never throws** — neither its completion log nor
  its catch log ever fired, across four runs — so it is parked inside its SECOND `await settle()`;
- meanwhile ordinary command turns on the SAME actor keep being served (operations 65, 66), so the
  actor is not wedged and `serializeCommandIngressForActor`'s queue is not blocked by the drain;
- `[DEBUG] typed-operation slots instance=2 live=1/64 → 2 → 3 → 4` and never back to 0: each re-offer
  mounts another operation and none of them ever terminates;
- **no completion was ever delivered** — the receiver's log never fired once.

So `applyDirectoryEventPage`'s mounted typed operation never reaches its terminal publication in the
live `s` shell, for a reason inside `🔌️PluginRuntime`'s typed-operation drain (its second
continuation settle never resolves), not inside the directory bootstrap and not inside `🪐️space`.
The guest source is correct for this route (`HOME_RETAINED_TOOL_IDS`, the `Config` publication
contract, the 1 MiB one-item lane and the `Migrated` classification are all in place, and the
admission handle proves the staged wasm carries them). **Not fixed here**: it is a typed-operation
runtime defect, it is the next owner's, and this report is its first exact measurement. The suspected
mechanism — the drain's `submitTurn(..., { lane: "UserVisible" })` starving behind the shell's
`Interactive` lane — is **unverified**.

### E2E steps reachable before the socket

`collabRunScenario`'s 13 steps are driven through the Home and Space tables (C2 §3), so step 1 needs
§4.3. Measured per step, on the shells that were alive (serve 6071 → hub 7501; serve 6190 → hub 7611
could not be driven at all, §4.1):

| # | step | verdict | measured |
|---|---|---|---|
| 0a | the shell boots on a hub-bound serve | **PASS** | `s-home-main` published before and after sign-in |
| 0b | one human signs in, real form, real hub | **PASS** | `POST /auth/sessions → 200`, the sign-in badge clears, `25× /_semio/hub/auth/sessions/me → 200` |
| 0c | the directory frontier is delivered and parsed | **PASS** | `4× GET /directory/event-page/v1?after=N → 200`, notice reads "through sequence 9" |
| 0d | the page is dispatched into Home and ADMITTED | **PASS** | turn `more-work`, `{"operationId":"64","generation":"0"}` |
| 1 | Home's space table lists the user's spaces | **FAIL** | 0 rows, "No studios yet." — the operation never settles (§4.3). The lane is `pending`, no longer `receipt-mismatch` |
| 2 | enter a space | **NOT REACHED** | no row to enter; the hub-workspace `Open …` fallback offers nothing on this hub for this principal |
| 3 | the space's document list shows the gis map | **NOT REACHED** | the gis map lives in the gm1 catalog on hub 7611 (§4.1) |
| 4–13 | second human, shared document, live edit, undo, catch-up, convergence, presence, restart | **NOT REACHED** | every one of them is behind step 1, and 3b onward is additionally behind the hub abort |

Nothing above is claimed from reading source: every verdict is a line in
`🗑️generated/db1-journey-*.txt`.

## 5. Honest gaps

- **Outcome 3's first step is not reached.** Home still lists no spaces. What this slice fixed is the
  step that used to KILL the lane (`receipt-mismatch` on the admission); what stops it now is §4.3,
  measured and handed on at file granularity, not guessed.
- **Nothing was measured on the brief's own pair (6190 + hub 7611).** The hub aborts on the first
  signed-in session read, three times, on two different binaries. Screenshot `db1-aborted.png` is the
  honest record of that; `db1-home.png`/`db1-space.png` are from 6071 + hub 7501.
- **The gis-map document was never listed**, because it lives in the gm1 catalog that only hub 7611
  serves.
- **The fanout of §2.2 is verified by law, and live only indirectly** (the bootstrap's own subscriber
  never fired live, because no completion was ever published — §4.3). A live proof of two subscribers
  sharing one completion needs an operation that completes.
- **`🧪️tests/🔌️plugin-runtime/🟦️.tsx` is not in any vitest `include` list** — it runs only as
  PluginRuntime's in-source suite (`longInSourceSuites`), which is how the new completion law is
  gated here. Registering that suite is somebody's cheap follow-up, not this slice's.
- No wasm was rebuilt and no component re-staged: every change is TypeScript on the serves' own vite
  graph, proved by the behaviour change on the unchanged serve pid 34308.

## 6. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `PluginOperationCompletion.terminalOutput` + `typedOperationTerminalOutputV1`; `subscribeOperationCompletions` fans out from one per-instance channel registration (`completionFanouts`), released in `releaseInstanceMaps` (§2.1, §2.2) |
| `…/🧱️elements/🏛️ShellHost/📇️directory-bootstrap/🟦️.tsx` | `startedDirectoryOperationIdV1`, `watchDirectoryOperationV1`, `DIRECTORY_BOOTSTRAP_SETTLE_DEADLINE_MS`, the typed-transient retry rule, and `applyDirectoryEventPageBootstrapV1` settling the operation instead of reading the admission (§2.3–§2.7) |
| `…/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts` | `directoryHomeBootstrapOracle` pins the settled-receipt law instead of the defect (29 → 33 checks); the second vitest run also gates the completion-delivery law |
| `…/🧪️tests/📇️directory-home-bootstrap/🟦️.tsx` | the fake Home handle now admits a typed operation and publishes its receipt through the completion subscription; four new laws, four rewritten (§3) |
| `…/🧪️tests/🔌️plugin-runtime/🟦️.tsx` | the completion-delivery law asserts `terminalOutput` and a second subscriber on one instance |

Ticket folder (not product code): `🐍️db1-home-space-journey.mjs`; captures
`🗑️generated/db1-bootstrap-laws.txt`, `db1-journey-*.txt`, `db1-hub-restart.txt`, `db1-hub-7611.txt`;
screenshots `db1-home.png`, `db1-space.png`, `db1-aborted.png`. Binary copy
`⚡️cache/db1/os-hub` (HS1's build, copied and re-signed; nothing of HS1's was touched).
