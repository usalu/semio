# C3 — two users editing ONE shared gis map document, live, over hub 7611

Slice C3 (session 6, 2026-09-20). Outcome 3, the observed form: **two signed-in humans in two browser
contexts edit the SAME hub document at the same time**, step by step, each step with its measured
witness.

Inherited, all re-measured at slice start (`🗑️generated/c3-preflight.txt`):

| what | value |
|---|---|
| hub | `http://127.0.0.1:7611`, pid `os-hub-7611` **48489**, HS1's fixed binary, data root `.🧬semio/🌐hub/gm1-boot` |
| `/readyz` | `status: ready`, `artifactAuthority.ready: true`, `features.openPlan: true`, runId `d8a8fe0d50f30762a4369546fa348a84` |
| `s` serve | 6190 (vite 42307) — Home still lists no spaces (a sibling owns that defect) |
| `gis2d` serve | 6191 (vite 66294) — this slice's shell |
| space / document | `01a0c00f-4f3c-7834-a7e6-2ccf9de925db` / `artifact-2fb248125b8b2b4d56de25933d30ed21` |
| surface | `s.gis.gismap@1/*#editor`, window kind `gis2d-main` |
| humans | `user1@semio.dev` / `gm1-local-dev-pass-1`, `user2@semio.dev` / `gm1-local-dev-pass-2` |

## 0. Headline

**The hub blocker is gone and the shared document opens — but the scenario was never two users: the
second human is not a member of the space.** Measured browser-free against the live hub
(`🗑️generated/c3-authz-probe.txt`), with both humans' own minted session tokens:

```
user1  GET /directory/spaces  → [{"access":"author","space":{… "memberCount":1, "documentCount":1 …}}]
       POST …/open-plan       → 200  semio.hub.document-open-plan/v1, receipt open.v1.ONeYc8j_…

user2  GET /directory/spaces  → []
       POST …/open-plan       → 401  {"schema":"semio.hub.document-open-plan-error/v1","code":"denied"}
```

Both humans mint a session and both pass `GET /auth/sessions/me` → 200 (`user1` =
`01a0c00d-cd33-…`, `user2` = `01a0c00d-d7f0-…`), so authentication was never the fault. GM1
provisioned two **users** but added only one **member**: the space's `memberCount` is `1` and
`user2` cannot see, let alone open, the document. C2's "two humans signed in at once" was real;
"two humans on one document" was never reachable, and the hub stack overflow was hiding the
membership gap behind it.

The hub's own log is the independent witness — across every run of this slice only ever ONE
principal opens a document socket:

```
server.document.socket ok … principal:"user:01a0c00d-cd33-7948-91cb-da23affa54ec"   ← user1, three times
(no server.document.socket for user:01a0c00d-d7f0-… ever)
```

## 1. E2E baseline on the ready hub

The FIRST complete pass, verbatim, before any fix — `🗑️generated/c3-run1.txt`,
`c3-collab-scenario.json`, console `c3-collab-scenario-console.txt`:

| # | step | verdict | measured witness |
|---|---|---|---|
| 1a | two contexts boot the `gis2d` shell on the hub-bound serve 6191 | **PASS** | `user1:ready=gis2d error=none user2:ready=gis2d error=none` |
| 1b | two DIFFERENT humans sign in at once, real form, real hub | **PASS** | both contexts lose `[data-semio-hub-sign-in]`; hub mints `r000000000375` → `user:…cd33…` and `r000000000381` → `user:…d7f0…` |
| 1c | both attached to the SAME document | **FAIL** | `user1` 2 sockets (`…/documents/artifact-2fb2…/socket/v1?surface=s.gis.gismap%401%2F*%23editor` + the directory socket); **`user2` 0 sockets**, `open-plan` → **401**, status `The document target changed. Reopen the document.` |
| 1d | both rosters list both users | **FAIL** | `user1` roster = 1 peer (`peer:hub.v1.2131e219…`, label `U1`); `user2` roster empty |
| 1e | baseline witness tuple on both | **PASS** | `user1` inspector `["Schema gis.map","Positions 152","Routes 149","Regions 0","Layers visible 11/11","Selected 0"]`, syncPill `Persisted`; `user2` inspector `["sync File Folder Remote"]`, syncPill `Remote: detached` |
| 2 | live edit A→B | **FAIL** | A authored a real leaf — `create-position index=152 item { position-1 data={ id="position-1" kind="marker" …` — B **unchanged after 58 148 ms**, canvas unchanged. B is not attached, so nothing could arrive |
| 3 | live edit B→A | **FAIL** | B authored `create-position index=152 …` **into its own local document** (same `index=152`, i.e. a second private copy), A unchanged after 58 125 ms |
| 4 | per-user undo / redo | **PASS** (single-user only) | A `1→0→1` uncommitted edits, ledger `10→11→12`; B's ledger `7→7→7` untouched throughout. Independence is real but trivially so — the two are not sharing a document |
| 5 | presence, distinct colours | **FAIL** | both rosters empty at that point; `user1`'s earlier single row had colour `rgb(137,26,26)` |
| 6–10 | loss/catch-up, convergence, reload, restart, agent | **not run** | every one needs 1c |

Zero fault lines in either context (`FAULTS 0`) — nothing crashed, nothing trapped, and **the hub
did not abort**: `grep -c "overflowed its stack"` on the 7611 hold log is **0** across the whole
slice. HS1's fix holds under a real browser document session (the log shows sessions of 62 s, 268 s
and 90 s all ending `outcome:"cancelled" detail:"closed"`, i.e. closed by the client).

## 2. Per-step results, after the membership fix

Run `c3r2` (`🗑️generated/c3-run2.txt`, `c3r2-collab-scenario.json`) is the authoritative post-fix
pass; `c3final` (`🗑️generated/c3-final.txt`) adds step 8. Both on the untouched serves 6190/6191 and
hub 7611.

| # | step | verdict | measured witness |
|---|---|---|---|
| 1 | **both attached, two sockets on the hub, both rosters** | **OBSERVED (sockets), PARTIAL (rosters)** | both contexts open `ws://127.0.0.1:7611/spaces/01a0c00f-…/documents/artifact-2fb2…/socket/v1?surface=s.gis.gismap%401%2F*%23editor`. The **hub's own log** is the independent witness: `server.document.socket … principal:"user:…cd33…" durationUs:267757810` and `… principal:"user:…d7f0…" durationUs:212921587`, i.e. two DIFFERENT humans holding a socket on the SAME document for 268 s and 213 s concurrently, both ending `outcome:"cancelled" detail:"closed"` only when the browser closed. Same document proven three ways: identical `ledgerHash aa3598b0f6c4671d`, identical inspector `["Schema gis.map","Positions 152","Routes 149","Regions 0","Layers visible 11/11","Selected 0"]`, identical canvas pixel hash `dea97085c9e00c4f:231382`. Rosters: `user1` listed **both** (`U1` + `U2`), `user2` listed only itself — see §3.3 |
| 2 | live edit A→B | **NOT OBSERVED** | A authored a real leaf (`create-position index=152 item { position-1 data={ id="position-1" kind="marker" …`), B **unchanged after 58 102 ms**, canvas unchanged. Cause in §3.2 |
| 3 | live edit B→A | **NOT OBSERVED** | symmetric; B's leaf also minted `position-1` at `index=152`, i.e. each client wrote into its own copy |
| 4 | per-user undo / redo | **OBSERVED, but single-document only** | `undo=ok redo=ok`, A `1→0→1` uncommitted edits with ledger `10→11→12`; B's ledger `10→10→10` untouched. The independence is real but cannot be claimed as *collaborative* undo while §3.2 stops edits from crossing |
| 5 | presence, distinct colour per peer | **PARTIAL** | in `c3r2`/`c3r3` `user1`'s roster carried BOTH peers with **distinct** colours — `U1` `rgb(137, 26, 26)`, `U2` `rgb(26, 82, 137)` — screenshots `🗑️generated/c3r2-attached-user1.png`, `c3r2-attached-user2.png`, `c3-step5-presence-user1.png`, `c3-step5-presence-user2.png`. No cursor/selection marker is drawn by this product. The roster is **not symmetric and not stable**: §3.3 |
| 6 | 10 s connection loss on B → catch-up | **not run** | meaningless while no edit ever crosses (§3.2); running it would have produced a green tick with no content |
| 7 | two-writer convergence, 10 + 10 edits | **not run** | same reason — both clients already "converge" trivially because neither publishes |
| 8 | reload B → re-attach → same document | **OBSERVED** | `c3final`: after `page.reload()` B boots `ready=gis2d`, re-attaches, opens a **new** document socket, and the inspector reads the same extents `Positions 152 / Routes 149 / Regions 0`. (The harness scored it FAIL on a wrong predicate — it compared the History ledger, which is per-session and correctly restarts at 0.) |
| 9 | mid-edit hub RESTART on an own second hub | **not run** | it proves recovery of a live editing session; there is no live editing session to interrupt |
| 10 | AI agent as third participant (M6b step 6) | **not run** | the hub exposes `POST /auth/agent-delegations` + `/auth/agent-sessions` (`🌎️hub/🏗️bootstrap/🦀️.rs:9723-9725`) and `features.mcpWorkspace` on 7611 reads **`false`**, so the delegated-agent roster entry cannot be reached on this hub as configured |

**The hub never aborted.** `grep -c "overflowed its stack"` on `🗑️generated/hs1-hub-7611.txt` is **0**
across every run of this slice, over 42 `server.document.socket` events including two concurrent
multi-minute sessions. HS1's fix holds under real browser document sessions from two principals at
once — that is this slice's independent re-verification of it.

## 3. Defects found and fixed

### 3.1 The space has one member, so outcome 3 had no second participant

Not a code defect: a **provisioning gap** in the GM1 fixture. `POST /admin/api/intents` with
`{"kind":"upsert-space-member", spaceId, email, role}` is the product's own route for it
(`🌎️hub/🏗️bootstrap/🦀️.rs:9747`, client `🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx:157`).

The route that does **not** need an admin provider is the product's own directory command:
`authorize_directory_command` (`🌎️hub/🏗️bootstrap/🦀️.rs:5901`) admits `UpsertMember` from any AUTHOR
of the named space, and user1 is the author. `POST /admin/api/intents` answers **401** without an
admin session, so that path was not used.

`🐍️c3-add-member.ts` (new, permanent, parameterised) mints the author's session, seals the request
with the product's own `sealDirectoryCommandRequestV1` + `directoryCommandRequestJson` — the exact
canonical JSON `DirectoryCommandRequestV1::parse_canonical_json` accepts — and posts it. Measured
(`🗑️generated/c3-add-member.txt`):

```
REQUEST {"schema":"semio.directory.command-request.v1","requestId":"413d4c57…","command":
         {"kind":"upsert-member","spaceId":"01a0c00f-…","email":"user2@semio.dev","role":"author"}}
COMMAND HTTP 202  outcome "accepted", event seq 9 kind "member.upserted" userId 01a0c00d-d7f0-…
user1 spaces  … "memberCount":2 …
user2 spaces  [{"access":"author","space":{… "memberCount":2 …}}]     ← was []
```

and immediately after, browser-free, `user2` `POST …/open-plan` → **200**
`semio.hub.document-open-plan/v1`, receipt `open.v1.1MhbA0V0fJSKAtfjxzRuIZbCjB5ub1UcQtS43cipJ04` —
**was 401 `denied`**. One gotcha for the next caller: `requestId` must be exactly 32 lowercase hex
digits and not all zeros (`DIRECTORY_COMMAND_REQUEST_ID_LEN`), or the seal throws
`directory-command.invalid-request-id` before anything is sent.

This is a change to the GM1 hub's **data**, not to any source file: the space
`01a0c00f-4f3c-7834-a7e6-2ccf9de925db` now has two author members and is a genuinely shared space.

### 3.2 The document's browser-actor child rejects its activation invocation — THE live-edit blocker

Both humans hold a document socket, the whole HTTP chain is 200 for both
(`open-plan` → `execution-target/{manifest,component,descriptor}` → `socket-grants` →
`execution-target/browser-actor`, all 200, `🗑️generated/c3r2-collab-scenario-console.txt`), the hub
sends `Session`, and then the shell shows **`The document component could not be verified. Reopen the
document.`** — `emitExecutionTargetStatus(…, "integrity-failed")` from
`activateDocumentBrowserActorAfterSession` (`🏪️store/👷️worker/🟦️.ts:2609`).

Named exactly, by temporary instrumentation in that catch (added, measured, **removed** — the tree is
clean, `grep -rn C3PROBE 🧰️framework/` is empty and `git diff --stat` on both files is empty):

```
C3PROBE browser-actor-activation-failed doc=artifact-2fb248… error=browser actor child: deadline
        sameSocket=true hubActorReady=true actor=hub.v1.b14a… grantActor=hub.v1.b14a…
        kind=closed-browser-actor owner=true
C3PROBE browser-actor-activation-failed doc=artifact-2fb248… error=browser actor child: invocation rejected
        sameSocket=true hubActorReady=true actor=hub.v1.659d… grantActor=hub.v1.659d…
        kind=closed-browser-actor owner=true
```

The Session handshake is sound (`hubActorReady=true`, `actor` equals `grantActor`, `sameSocket=true`)
— the failure is the **browser-actor child**, the nested worker that hosts the document component in
the browser.

Two things were ruled out by measurement, not by reading:

1. **It is not the child's deadlines.** `BROWSER_ACTOR_CHILD_LIMITS` is
   `bootMs 10 000 / loadMs 5 000 / invokeMs 2 000` against GM1's **47 390 785 B** gis component, which
   made a timeout the obvious suspect. Raising them to `120 000 / 120 000 / 60 000` and re-running
   changed the symptom from `deadline` to `invocation rejected` **on both clients** and fixed nothing
   (`c3r7`). The constant was restored to its original values.
2. **It is not a load failure.** `invocation rejected` is only reachable from
   `🧵️child/🟦️.ts:152`, which requires the child to be in phase `active` — i.e. the 47 MB bundle was
   transferred, its sha256 verified and `module.activate` returned. The child then answers
   `{kind:"rejected"}` to the activation invocation, from
   `🧵️child/👷️worker/🟦️.ts:73`.

So: **the guest's own `actor.invoke` throws during activation.** That is the single thing between
this slice's state and steps 2, 3, 6, 7, 9.

Consequence, measured: local edits never reach `relayMutationsToHub`'s ready path. Instrumentation on
the relay predicate showed the ONLY artifact state in the store worker is `os.config.identity`
(`allStates=["local:v1:18:os.config.identity"]`) — the gis document's mutations live in the
browser-actor child that never activated, so nothing is ever queued for the hub. The hub log
confirms it from the other side: **not one `Commands` frame** in 42 socket events.

### 3.3 Two diagnosability defects that made §3.2 expensive to find (found, NOT fixed)

1. **The guest's activation error is swallowed whole.** `🧵️child/👷️worker/🟦️.ts:73` is
   `catch { if (phase === "active") send({ kind: "rejected", sequence }); }` — no reason is captured
   and the `rejected` frame carries none, so the parent can only ever say "invocation rejected". Every
   browser-actor failure in this product is therefore anonymous by construction.
2. **The child is a NESTED worker, so its console never reaches the page.** A `console.warn` added at
   that exact catch produced **nothing** in Playwright's `page.on("console")` (`c3r8`), while the same
   probe in the store worker one level up printed fine. Any future owner instrumenting this path will
   lose the same hour: the reason must travel in the `rejected` frame, not in a log line.

   Fixing 1 means widening the child protocol's `rejected` frame with a bounded reason and its
   `exact("sequence")` validator with it — a contract change with fixtures and laws attached, which is
   not a thing to half-land at the end of a slice. It is the next owner's first step.

### 3.4 The presence roster is asymmetric and decays

`user1`'s roster listed both peers with distinct colours; `user2`'s listed only itself, in every run
— and in `c3final` the asymmetry reversed (`user2` populated, `user1` empty, `user1` syncPill
`Remote: detached`). Reading the roster at the END of a run showed it **empty in both contexts**
(`c3-run1.txt`, `c3-run2.txt` step 5), which is why this slice moved the presence witness to attach
time. Presence beats ride the document socket, and the socket stays open the whole time, so an empty
roster on a live socket is a real defect — not investigated here, and it may well be downstream of
§3.2 (a document whose browser actor never activated may never publish a beat).

## 4. Permanent wiring

**Not added, deliberately.** The brief says to wire the scenario as an nx target + launch row "once it
is mostly green". It is not: three of the ten steps are observed, one is partial, and the live-edit
half is blocked on §3.2. An nx target today would pin a red path as a gate and a launch row would
invite a peer to run a scenario whose central claim fails. The harness itself is permanent and
parameterised, so wiring it is a five-line change the moment §3.2 lands:

```sh
bun "$T/🐍️c3-collab-scenario.mjs" http://127.0.0.1:6191 127.0.0.1:7611 \
    01a0c00f-4f3c-7834-a7e6-2ccf9de925db artifact-2fb248125b8b2b4d56de25933d30ed21
# env: C3_TAG=<capture prefix>  C3_ONLY=2,3,4,8  (steps 1a-1e and 5 always run)
```

## 5. Honest gaps

- **Outcome 3 is HALF observed.** Two humans, two contexts, two concurrent sockets on one hub
  document, one shared document state, distinct presence colours: measured. Two humans *editing* it:
  **not measured, and not claimed anywhere in this report.**
- **Steps 6, 7, 9 and 10 were not run at all.** Steps 6/7 would have returned green ticks with no
  content (nothing crosses, so "catch-up" and "convergence" are vacuous) and this slice will not
  record a tick it cannot defend. Step 9's second hub was not built: restarting a hub proves recovery
  of a live editing session, and there is none. Step 10 needs `features.mcpWorkspace`, which reads
  **`false`** on 7611.
- **§3.2 is diagnosed, not fixed**, and its last hop — the guest's own exception text — was never
  read, because of §3.3.2. Everything said about it above is from frames that were printed by a live
  process; nothing is inferred from source alone.
- **Step 4's undo independence is real but weak evidence.** Two clients that never exchange an edit
  will always show independent undo. It is recorded as OBSERVED for what it is and no more.
- **The presence roster asymmetry (§3.4) was not root-caused.**
- **The step-8 harness predicate is wrong** (it compares a per-session History ledger across a
  reload). The underlying behaviour is observed; the tick is scored FAIL. Left honest rather than
  loosened to make a run look green.
- **No release measurement, no second hub, no agent** — see above.
- **Nothing was rebuilt.** Every experiment here is TypeScript picked up by the running vite serves
  (pids 42307 / 66294, untouched), or plain HTTP against the running hub.

## 6. Files changed

**No product source file is modified by this slice.** Three files were instrumented temporarily and
all three are byte-identical to how they were found — verified with `git diff --stat` (empty) and
`grep -rn "C3PROBE" 🧰️framework/` (empty):

| file | what it carried, and why it is gone |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts` | relay-readiness and browser-actor-activation probes (§3.2) — removed |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts` | the swallowed-guest-error probe (§3.3.1) — removed |
| `…/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` | `BROWSER_ACTOR_CHILD_LIMITS` raised to test the deadline hypothesis (§3.2.1) — **restored** to `bootMs 10000 / loadMs 5000 / invokeMs 2000` |

Hub data changed (intended, §3.1): the GM1 space now has `user2@semio.dev` as a second **author**
member (`memberCount 1 → 2`).

Ticket folder, new and permanent: `🐍️c3-collab-scenario.mjs` (the ten-step harness),
`🐍️c3-attach-diagnose.mjs` (single-context attach chain with every hub request, its credential header
and its response body), `🐍️c3-add-member.ts` (§3.1). Captures `🗑️generated/c3-*.txt`,
`c3*-collab-scenario.json`, `c3*-collab-scenario-console.txt`; screenshots `c3-attached-*.png`,
`c3r2-attached-*.png`, `c3-step5-presence-*.png`, `c3-step2-b-sees-a-user2.png`,
`c3-step3-a-sees-b-user1.png`, `c3final-attached-*.png`.

## 7. State at hand-off

| what | pid | note |
|---|---|---|
| hub 7611, HS1's fixed binary, data root `gm1-boot` | `os-hub-7611` **48489** | untouched and healthy: `status: ready`, `openPlan: true`, **0** stack overflows across 42 document-socket events. The space now has two author members |
| `s` react dev serve 6190 | wrapper 42162 / vite 42307 | untouched, still bound to 7611 |
| `gis2d` react dev serve 6191 | wrapper 66282 / vite 66294 | untouched, still bound to 7611 |

Nothing was started and nothing was killed by this slice — no server, no hub, no cargo. No peer's
files, captures or `🗑️generated` entries were touched.

**For the next owner, in order:** (1) make the browser-actor child's `rejected` frame carry a bounded
reason (§3.3.1) — without it §3.2 cannot be read; (2) fix whatever that reason names; (3) re-run
`🐍️c3-collab-scenario.mjs` with `C3_ONLY=2,3,4,6,7,8` — steps 2-8 need no rebuild, no activation and
no new hub, only the serves already running.
