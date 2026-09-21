# C5 — two humans editing ONE hub document live, in browsers

Slice C5 (session 7, 2026-09-21). Owner: the critical path of **outcome 3** — two signed-in users,
two browser contexts, ONE hub document, live edits crossing in both directions.

## 0. HANDOFF (top of report)

| field | value |
|---|---|
| **hub serving the first jco-1.34 catalog** | **YES, live.** `http://127.0.0.1:7621` — `/readyz` 200, `status:"ready"`, `artifactAuthority.ready:true`, `publicSessionIssuance:true`, `features.openPlan:true`, runId `57728fd47c980fd40c77fc0de4a6a31b` |
| port / data root | **7621** / `.🧬semio/🌐hub/jc1-boot`, generation `8086b61f336e08b5c483ca96525e8e77d6322becbe1569aa6cfedbf3b595ae6b` |
| hub binary | `.🧬semio/🦑️repo/⚡️cache/cargo/target-jc1/debug/os-hub` (1.34 policy) — C5 was the ONE builder in that private `CARGO_TARGET_DIR`; the running hold reuses it, nothing was rebuilt after 05:49 |
| serves | `s` **6190**, `gis2d` **6191**, both bound to `http://127.0.0.1:7621`, both HTTP 200 |
| **two humans + shared document, provisioned** | `user1@semio.dev` `01a0c314-d182-7a22-84e2-d05e7e9a9b7a`, `user2@semio.dev` `01a0c314-d9e0-7d0e-8272-a934da5aacea`; space `01a0c314-e41f-780d-a980-3adda40ca9f7` (`memberCount:2`, BOTH `author`); document `artifact-0954e2d10d8fff9605f101b0dba34f3b`; `open-plan` **200** with `browserActor.importInterfaces=17` |
| **`reactor.poll` lifts in a real browser** | **YES** — JC1's jco 1.34 bump is confirmed end-to-end: the `_liftFlatVariantInner … taskReturn` TypeError that killed every browser actor since C4 is **gone** (§4) |
| browser actor reaches `active` | **NO.** Three further faults sat behind the jco one: the child value validator (**fixed**, §4.1), a stale staged component (**resolved by re-activation**, §4.2) and a flat document-opening deadline (**fixed**, §4.4). What remains is §4.3: the actor's 63 717 043 bytes fetch and verify in **31 s** and the load then **hangs with no refusal and no deadline**, for at least 8 further minutes |
| scenario steps green | **0 of 10 — not run, nothing claimed.** Blocked on §4.3 alone; hub, serves, both humans, space, document, both memberships and the whole attach chain are green |
| binary every measurement ran on | `⚡️cache/cargo/target-jc1/debug/os-hub`, built 03:42 (1.34 policy). PR1's hub-side presence fixes are **not** in it; not swapped, because no step reached presence (§8) |
| containment gate | `browser-actor-child-worker-containment`: **18 laws / 13 wire / 9 fault, chromium, passed** — re-run after every source change here (`🗑️generated/c5-containment-final.txt`) |
| restart lines | §7 |

## 1. Inherited state (measured)

Slice start 03:35, cut by the coordinator outage at ~04:10, resumed 10:27 (preamble rule 29).

| thing | at 03:35 | at 10:27 (resume) |
|---|---|---|
| jco | 1.34.0 in `bun.lock`, JC1's `validateAsyncTaskReturnLift` law landed | unchanged |
| 1.34 policy constant | present: `🌐️browser-actor/🦀️.rs:120`, `🟦️.ts:18`, `🟦️.ts:73`, fixture `🔣️.json:8` (all `semio.os.browser-jco-1.34.0-jspi.v1`) | unchanged |
| catalog with a 1.34 actor | **none published** | `jc1-boot/trusted-catalog/generations/8086b61f…` — **exists**, built 05:33 |
| hub 7621 / serves 6190, 6191 | all down | all down; restarted by C5 |
| wasm mutex | free | free at 10:27, taken by peer `pz1` at 10:32 |

## 2. Republish of the trusted catalog with the 1.34 actor — DONE

Launched detached at 03:37 exactly as JC1 §0's resume line says (pid **83628**, `📜️jc1-hub-boot.sh
7621`). It survived the 04:10 coordinator outage and finished on its own: **bootstrap exit 0 at
05:49:34**, hub `/readyz` 200 at 05:49:44. The generation directory
`8086b61f336e08b5c483ca96525e8e77d6322becbe1569aa6cfedbf3b595ae6b` (05:33) is the **first trusted
catalog in this repo built with the fixed jco**. That hold was later SIGTERM-drained by someone
else; C5 restarted it at 10:27 from the same data root and binary — a restart, never a second
bootstrap (§7).

Provisioning on that root (`🗑️generated/c5-provision-run.txt`, `📜️c5-provision.sh`, new and
permanent) is green end to end:

```
credential set user1@semio.dev exit=0      01a0c314-d182-7a22-84e2-d05e7e9a9b7a
credential set user2@semio.dev exit=0      01a0c314-d9e0-7d0e-8272-a934da5aacea
sign-in status=200 tokenShape=true
readyz status=ready artifactAuthority.ready=true features.openPlan=true
create-space status=202 spaceId=01a0c314-e41f-780d-a980-3adda40ca9f7
creation-catalog status=200 generation=8086b61f… kinds=s.gis.gismap
artifact-creation ready documentId=artifact-0954e2d10d8fff9605f101b0dba34f3b schema=gis.map
open-plan status=200  grant={read,write,observe}  browserActor.importInterfaces=17
COMMAND HTTP 202 outcome "accepted" seq 9 member.upserted
user2 spaces [{"access":"author", … "memberCount":2 …}]
```

## 3. Harness repairs before the run

### 3.1 The chromium containment gate could not run at all — THREE defects, all fixed

C4 §8 item 1 asks for one run of `browser-actor-child-worker-containment`. It had **never run**: the
registered nx target fails three times over before reaching chromium. All three are fixed and the
gate now passes (`🗑️generated/c5-containment.txt`):

```
browser-actor-child-worker-containment: ajv=1 typescript=1 chromium=1
{"laws":18,"wireLaws":13,"faultLaws":9,"transfers":2,"forcedLoopTerminations":2,
 "capacity":{"actors":0,"bytes":0}} passed
```

| # | defect | file:line | fix |
|---|---|---|---|
| 1 | the gate's own TypeScript program is built with `types: []` and DOM/webworker libs only, so the moment C4 gave `🧵️child/🧬️schema/🟦️.ts` the repo's standard in-source `import.meta.vitest` registration the gate died with `TS2339: Property 'vitest' does not exist on type 'ImportMeta'` + `TS2307: Cannot find module 'node:url'` — a gate that refuses the repo's own test-registration convention | `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5520` | `types: ["node", "vitest/importMeta"]` — the two ambient sets that convention needs (`node_modules/vitest/importMeta.d.ts` declares `ImportMeta.vitest`); 44 modules under the os product use this pattern |
| 2 | the gate throws `child Worker gate requires SEMIO_TEST_ARTIFACT_DIR`, and its **own nx target never sets it** — so `nx run …:browser-actor-child-worker-containment-check` could not succeed for any operator | `🌎️hub/📦️packages/🦀️rust/📋️project.json:654` | added `options.env.SEMIO_TEST_ARTIFACT_DIR` (repo `⚡️cache` path), the `env` form used by the root `📋️project.json` dev targets |
| 3 | the `neutral limits` law (`📜️script.ts:5593`, an exact `JSON.stringify` equality against `BROWSER_ACTOR_CHILD_LIMITS`) failed: C4 §2 added `loadBytesPerMs: 2048` to the constant but not to the fixture or its json-schema, so the containment corpus no longer described the constant it guards | `…/🧵️child/🧫️fixtures/🔣️.json` limits, `…/🧵️child/🧬️schema/🔣️.json` `limits.required` + `limits.properties` | `loadBytesPerMs` added in the constant's key order (the law compares stringified key order) and pinned as `"const": 2048` in the schema |

Defect 3 is the substantive one: it means C4's load-deadline fix (§2 of that report) shipped without
its corpus, and the gate that would have caught it was itself unrunnable because of 1 and 2.

### 3.2 The step-8 predicate compared the wrong thing (C3 §5) — FIXED

C3 scored its own reload/re-attach step FAIL on a predicate that compares the **History ledger**
across a reload. That ledger is per SESSION and correctly restarts empty, so the predicate could
never pass on a working re-attach. Replaced with the document's own state — the app's inspector
extents, with the canvas hash recorded alongside — plus the new socket:

`🐍️c3-collab-scenario.mjs` step 8: `before` is now taken WITH the inspector, and the verdict is
`newSocket && sameDocument` where `sameDocument` compares inspector extents. The ledger counts are
still printed, labelled `(per-session, not a witness)`.

### 3.3 Presence symmetry is now scored, and step 9 exists

- Step 5 previously passed when **one** roster listed both peers (`some`). C3 §3.4 records that the
  rosters are asymmetric, so the harness was measuring less than the claim. It now computes
  `symmetric` (`every`) and scores on it; `bothListedInOneRoster` is still reported so the
  asymmetric case is distinguishable from an empty one.
- **Step 9 (mid-edit hub restart) did not exist in the harness** — C3 left it unwritten. Added:
  baseline both, author a mid-flight edit on A, run `C3_HUB_RESTART` (`📜️c5-hub-restart.sh`, which
  stops the hold BY PID, waits for the port, starts a second hub process from the same data root and
  binary and blocks on `/readyz` 200 — the catalog is never republished), re-attach both contexts
  through the product's own sync card, then require BOTH clients to author successfully again.

### 3.4 Probes parameterised for this slice's ports

Both probes already took `[shellUrl] [hubHostPort] [spaceId] [documentId]` on argv; they are run
against **6191 / 127.0.0.1:7621** with this slice's own space and document. One real gap was closed
in `🐍️c4-actor-reason-probe.mjs`: it only ever waited for a *diagnostic*, so a SUCCESSFUL activation
looked identical to a timeout. Success has no status code at all — `reduceExecutionTargetUiState`
DELETES the row on `execution-target-cleared`, and `verifying` is the only non-alert code
(`📇️directory/🧬️schema/🟦️.ts:1693`). The probe now scores `ACTIVE` = a `verifying` notice was seen,
the notice is gone, and no diagnostic and no alert was ever rendered.

## 4. Browser actor on the republished catalog — the jco blocker is GONE, two more behind it

### 4.0 `reactor.poll` lifts in a real browser — JC1's bump is confirmed end to end

First run of `🐍️c4-actor-reason-probe.mjs` against the 1.34 catalog
(`🗑️generated/c5-actor-probe-user1.txt`, `c5-actor-reason-user1.json`), user1 signed in, sync card
found, attach pressed:

```
WHO user1 card=true attach=ok
DIAGNOSTIC browser actor child: invocation rejected: invoke reactor/poll:
           Error: browser actor child: value type/alias
           at visit@38:71<visit@63:7<measureChildValue@66:3<receive@111:24
```

Compare C4's diagnostic on the 1.27 catalog:

```
browser actor child: invocation rejected: invoke reactor/poll:
  TypeError: undefined is not iterable (cannot read property Symbol(Symbol.iterator))
  at _liftFlatVariantInner@3673:11 < _liftFlatResultInner@3856:14 < taskReturn@2751:33
```

**The jco frames are gone.** `taskReturn` no longer throws; the guest's `reactor/poll` result is
lifted and handed to the host, and the refusal now comes from **semio's own child-side validator,
after the lift**. That is the direct runtime confirmation of JC1's version bump, on a real browser
actor, in a real document session — the thing no slice had yet shown.

### 4.1 Defect — the child value validator refuses `undefined` and conflates three causes — FIXED

`measureChildValue` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts:38`)
guards the child→parent message with, in one line:

```ts
if (!item || typeof item !== "object" || objects.has(item)) throw new Error("browser actor child: value type/alias");
```

Two defects in that line.

1. **`undefined` is refused.** Every earlier branch returns for `null`, boolean, number, bigint and
   string, so `!item` at this point can only be `undefined` — and `undefined` is exactly how an
   **absent `option<T>` lifts across the component ABI**. So any turn result carrying an unset
   optional is undeliverable: the guest computes it, jco lifts it correctly, and the host's own
   bound-checker throws it away. `structuredClone` carries `undefined` in records and arrays without
   complaint, so nothing downstream needed it refused.
2. **Three different faults share one reason.** An unsupported type and a repeated object reference
   are distinct bugs with distinct fixes, and `value type/alias` names neither — the same
   anonymous-failure disease C3 §3.3 and C4 §1 spent slices on, one layer deeper.

Fixed at that line: `undefined` joins `null`/boolean in the return branch, and the refusal is split
into `unsupported value type <typeof>` and `value alias`. The docstring above the function now
states why `undefined` is a value here. No fixture or law changed shape (the containment corpus has
no `undefined` row; its `result-alias` wire row still exercises the alias branch, and the gate
passes — §3.1).

### 4.2 Blocker — the staged gis component predates a peer's kernel wire change (found, NOT fixed here)

After 4.1 the probe stopped reaching the sync card at all: `card=false`, no canvas, the shell dead
before attach. Cause, read from the live page with the full stack
(`🐍️c5-shell-boot-diagnose.mjs` and the stack capture added to the C4 probe's `pageerror` handler):

```
TypeError: Cannot read properties of undefined (reading 'en')
    at historyEntryLabelText (🧰️framework/🔨️modules/🎠️kernel/🟦️.ts:884)
    at …/🏛️ShellHost/🟦️.tsx  (entries.map over the History ledger)
    at updateMemo / useMemo
  → An error occurred in the <FrameworkOsShellInner> component
```

`LocalizedLabel` is `Record<ShellTerminology, Record<ShellLocale, string>>`
(`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts:15`), so `label[terminology]` is
`undefined` only when the producer does not carry the terminology axis. Timeline, measured from
mtimes and `git diff`:

| artifact | when | carries the terminology axis |
|---|---|---|
| `🎠️kernel/🦀️.rs` `pub label: dsl::LocalizedLabel` (peer, uncommitted) | 2026-09-21 **04:01** | — (the change itself) |
| hub catalog generation `8086b61f…` (C5's republish) | 2026-09-21 **05:33** | **yes** (built after) |
| `🎠️kernel/🟦️.ts` `historyEntryLabelText` + `🏛️ShellHost/🟦️.tsx` reader (peer, uncommitted) | 2026-09-21 **10:33–10:39** | — (the reader) |
| `✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/dist/component-dev/semio_s_plugin_gis.wasm` | 2026-09-**20** 16:46 | **NO** |

So the shell's **locally staged** gis component still emits history labels without the axis, while
the host reader landed hours later and refuses them by design ("deliberately NO English fallback",
the peer's own docstring). The shell crashes on the first History render after sign-in — before any
attach — so outcome 3 cannot be reached with that component, on any hub.

This is a stale-artifact mismatch, not a code defect: nothing here should be shimmed, the component
must be rebuilt. `📜️c5-activate-gis2d.sh` (new) re-activates the `gis2d` variant through the fleet
wasm mutex (rule 27). It was launched at **11:00:28** and is **queued behind peer `pz1`**, which has
held the mutex since 10:32 (holder pid 52089, alive and compiling — checked, not stale, not killed).
At 11:31 the hold was 59 minutes old and C5's activation had still not acquired it, so **no step of
the scenario is claimed**. The activation is detached (`nohup … & disown`, pid 69596) and proceeds on
its own the moment the mutex frees — the same shape that carried C5's republish through the 04:10
outage. Its progress is `🗑️generated/c5-activate-gis2d.txt`; when it exits 0, restart the 6191 serve
so vite picks the new component up, then run the two lines in §5.

**Resolved at 11:56:48**: `activate gis2d react dev` exited 0 — *"Activated gis2d react dev: 1
completed components (changed)"*. The 6191 serve was restarted from the fresh tree (old vite pid
48099 killed by pid, new pid 12801) and the shell then signs in, finds the sync card and attaches
cleanly. §4.3 is what stands behind it.

Two earlier probe runs (`c5c`, `c5d`) showed a *different* transient — the peer saving mid-run
triggered a full page reload into a half-edited module graph. `🐍️c5-shell-boot-diagnose.mjs` booting
to `READY gis2d` in 5.6 s with no sign-in proves the shell itself is sound; the crash is reached
only once history rows exist.

### 4.3 Blocker — the browser-actor load hangs silently after the bundle is delivered (found, NOT fixed)

With the fresh component the whole chain is green up to the last hop. Measured
(`🗑️generated/c5-actor-probe-user1-{g,h,i,j,k,l}.txt`, `c5{g,h,i,j,k,l}-actor-reason-user1.json`):

```
WHO user1 card=true attach=ok           ← sync card found, Attach pressed, no fault
ws-open ws://127.0.0.1:7621/spaces/…/documents/artifact-0954e2…/socket/v1?surface=s.gis.gismap@1/*#editor
ws-open ws://127.0.0.1:7621/directory/spaces/…/documents/…/socket/v1?since=0
29390 ms  Verifying document component…  progress 2/3                  ← sha256 + blake3 verified
30900 ms  Verifying document component…  progress 63717043/63717043    ← browser actor fully fetched
   … nothing, ever again …
```

**The actor's 63 717 043 bytes are fetched and verified in 31 s, and then nothing happens.** No
`rejected` frame, no `integrity-failed`, no child deadline, no status change — the live region keeps
saying *"Verifying document component…"* indefinitely. Proved to be a **hang, not slowness**: with
the outer deadline temporarily raised to 900 s (a measurement, reverted immediately — the tree
carries the derived value, `grep 900_000` is empty) the probe polled for a further **8 minutes** and
the status never moved off that row.

The child's own load budget is `childLoadDeadlineMs(63717043)` = `5000 + ceil(63717043/2048)` =
**36 112 ms**, so on a healthy path the child would have refused with a `deadline` reason 36 s after
receiving the bytes. It never does — so on this path either the child is never handed the bundle or
its deadline is never armed. That is the next owner's first measurement, and it needs the child's
own frames, which C3 §3.3.2 showed cannot be read from the page (nested worker): the reason must
travel in a frame, not a log line.

### 4.4 Defect — the outer document-opening deadline was flat and shorter than the budgets it waits on — FIXED

Found on the way to §4.3. `runDocumentOpeningAttemptV1` races `socket()` then `attach()` against one
budget (`🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts:122`), and the call site passed a
literal **`deadlineMs: 60_000`** (`🏛️ShellHost/🟦️.tsx:6716`). But `attach()` contains the host's
fetch-and-verify of the actor *and* the child's own load, boot and first invocation — whose admitted
worst case is already `childLoadDeadlineMs(actorBytes) + bootMs + invokeMs` = **49 768 ms**, before
the host has fetched a single byte. So the outer deadline was **shorter than the sum of the inner
deadlines it waits on**: a large component is cut off mid-load and the child is still inside its own
admitted budget, which is why the failure arrived as a bare `pageerror: document opening deadline
exceeded` with no refusal behind it.

This is C4 §2's defect one layer up — that slice made the *child's* load deadline byte-priced and
left the *document's* flat. Fixed by deriving it from the same limits, next to `childLoadDeadlineMs`:

- `🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` — new `documentOpeningDeadlineMs()` =
  `childLoadDeadlineMs(actorBytes) + ceil(actorBytes / loadBytesPerMs) + bootMs + invokeMs` =
  **82 536 ms**, i.e. the child's whole admitted load plus the host's fetch-and-verify of the same
  bytes charged at the same admitted throughput floor, with the docstring stating the invariant.
- `🏛️ShellHost/🟦️.tsx:6717` — the literal replaced by that call.

It does not cure §4.3 (a hang has no budget large enough), but it removes a real ceiling: with the
old value any actor over roughly 30 MB was capped before its child could legally finish.

## 5. The ten-step two-user scenario — NOT RUN

Not one step is claimed. The harness is repaired, parameterised and ready (§3), the hub, both
serves, both humans, the shared space, the shared document and both memberships are live and
measured (§0, §2), §4.2 is resolved, and the run is now blocked on **§4.3** alone: the document's
browser actor never finishes loading, so no client ever holds the document and every step from
"both attached" onwards would have been a tick with no content. Recording a step
without its capture is exactly what preamble rule 6 forbids, so this section stays empty until the
re-activation lands.

The exact line to run once `📜️c5-activate-gis2d.sh` finishes (and the serve on 6191 has been
restarted so it picks the new component up):

```sh
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️18/OS-HUB-COLLABORATION-AI-END-TO-END"
C3_TAG=c5 C3_HUB_RESTART="$T/📜️c5-hub-restart.sh" \
  bun "$T/🐍️c3-collab-scenario.mjs" http://127.0.0.1:6191 127.0.0.1:7621 \
      01a0c314-e41f-780d-a980-3adda40ca9f7 artifact-0954e2d10d8fff9605f101b0dba34f3b
# env: C3_ONLY=2,3,4,6,7,8,9 to select steps (1a-1e and 5 always run)
```

and, first, the single-context actor check that gates it:

```sh
C4_TAG=c5 bun "$T/🐍️c4-actor-reason-probe.mjs" user1 http://127.0.0.1:6191 127.0.0.1:7621 \
      01a0c314-e41f-780d-a980-3adda40ca9f7 artifact-0954e2d10d8fff9605f101b0dba34f3b
# PASS is the line `ACTIVE true`; any `DIAGNOSTIC`/`ALERT` line is the failure, named.
```

## 6. Product defects found and fixed

| # | defect | file:line | state |
|---|---|---|---|
| 1 | the browser-actor containment gate's own TS program refuses the repo's `import.meta.vitest` convention (`types: []`) | `🌎️hub/📦️packages/🦀️rust/📜️script.ts:5520` | **fixed** — gate passes |
| 2 | that gate's nx target never sets the `SEMIO_TEST_ARTIFACT_DIR` the script requires, so it could not run for any operator | `🌎️hub/📦️packages/🦀️rust/📋️project.json:654` | **fixed** — `options.env` added |
| 3 | the containment corpus lost `loadBytesPerMs` when C4 added it to `BROWSER_ACTOR_CHILD_LIMITS`, failing the `neutral limits` law | `🌐️browser-bundle/🧵️child/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json` | **fixed** |
| 4 | `measureChildValue` refuses `undefined`, i.e. every absent `option<T>` the component ABI lifts — the live blocker behind the jco fix | `🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts:38` | **fixed** |
| 5 | the same line conflated "unsupported type" and "value alias" into one unactionable reason | `🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts:38` | **fixed** — split |
| 6 | the reload/re-attach step compared the per-session History ledger, so a working re-attach could never pass | `🐍️c3-collab-scenario.mjs` step 8 | **fixed** — document extents |
| 7 | the presence step passed when ONE roster listed both peers, while the claim is a symmetric roster | `🐍️c3-collab-scenario.mjs` step 5 | **fixed** — `every` |
| 8 | the actor probe could not tell success from timeout: it only ever waited for a diagnostic, and success has no status code | `🐍️c4-actor-reason-probe.mjs` | **fixed** — `ACTIVE` verdict |
| 9 | the staged gis dev component predated the kernel's `LocalizedLabel` history wire, crashing the shell on first History render | `✏️s/🔌️plugins/🌍️gis/…/dist/component-dev/` (artifact, not source) | **resolved** — re-activated, exit 0 at 11:56:48, §4.2 |
| 10 | the document-opening deadline was a flat `60_000` literal, shorter than the inner child budgets it waits on, capping any actor over ~30 MB | `🏛️ShellHost/🟦️.tsx:6716` + new `documentOpeningDeadlineMs()` in `🧵️child/🧬️schema/🟦️.ts` | **fixed**, §4.4 |
| 11 | after the actor bundle is fetched and verified, the load hangs with no refusal, no child deadline and a live region stuck on "Verifying…" forever | browser-actor child load path | **found, NOT fixed**, §4.3 |

Defects 1–3 and 6–8 are gates and harnesses that were scoring the wrong thing or could not run at
all; 4 and 5 are in the product's live browser-actor path.

## 7. Restart lines, pids

Everything below was started by C5 and is alive at the end of this slice. Nothing of a peer's was
started, stopped or rebuilt. **Hub 7621 is restarted by C5 only** (preamble rule 29); a peer needing
a 1.34 hub reads `curl -s http://127.0.0.1:7621/readyz`.

| what | pid | capture | restart line |
|---|---|---|---|
| hub **7621**, data root `jc1-boot`, 1.34 binary | **48042** (`🗑️generated/c5-hub-pid.txt`) | `c5-hub-7621.txt` | `cd /Users/ueli/Documents/semio && OS_HUB_CREDENTIAL_SIGN_IN=true nohup bun "$T/🐍️ds1-hub-hold.ts" 7621 "$PWD/.🧬semio/🌐hub/jc1-boot" "$PWD/.🧬semio/🦑️repo/⚡️cache/cargo/target-jc1/debug/os-hub" > "$T/🗑️generated/c5-hub-7621.txt" 2>&1 & disown` |
| serve `s` **6190** | **48057** | `c2-serve-s-6190.txt` | `nohup zsh "$T/📜️c2-serve.sh" s 6190 http://127.0.0.1:7621 > /dev/null 2>&1 & disown` |
| serve `gis2d` **6191** | **48099** | `c2-serve-gis2d-6191.txt` | `nohup zsh "$T/📜️c2-serve.sh" gis2d 6191 http://127.0.0.1:7621 > /dev/null 2>&1 & disown` |
| `gis2d` re-activation (queued on the mutex since 11:00:28) | **69596** (`c5-activate-pid.txt`) | `c5-activate-gis2d.txt` | `nohup zsh "$T/📜️c5-activate-gis2d.sh" > /dev/null 2>&1 & disown` |

**Never re-run `📜️jc1-hub-boot.sh`**: the catalog is published, and that script re-runs the ~2 h
bootstrap. Restart the hold with the line above instead.

The step-9 hub restart is `📜️c5-hub-restart.sh` — it stops the hold **by the pid in
`c5-hub-pid.txt`**, waits for 7621 to be released, starts a second hub process on the same data root
and binary, and blocks until `/readyz` 200. It never republishes.

## 8. Honest gaps

- **Outcome 3's live edit is still NOT observed, and nothing in this report claims it.** What is new
  and measured is that its long-standing blocker moved: `reactor.poll` lifts in a real browser on a
  real hub document (§4.0), which had never happened before.
- **No scenario step was run** (§5). No screenshots of two attached humans, no convergence, no undo,
  no connection loss, no hub restart. The harness for all of them is ready and the infrastructure is
  live; only §4.2 is in the way.
- **§4.1 is fixed but its fix is verified only against the child containment gate and the live
  refusal disappearing from the diagnostic path** — not yet against a turn result that actually
  crosses, because §4.2 stops the shell before that point.
- **§4.2 is resolved** (re-activation exit 0 at 11:56:48) and is no longer the blocker. **§4.3 is
  the blocker and is diagnosed, not fixed**: the actor's bytes arrive and verify in 31 s and the
  load then hangs silently for at least 8 further minutes. Its last hop needs the child's own
  frames, which cannot be read from the page.
- **The hub-side presence fixes (PR1) are NOT in 7621's binary** (built 03:42 from the 1.34 policy
  tree); PR1's TS half is live in the serves. The coordinator's 12:11 binary
  (`⚡️cache/cargo/target-coordinator-hub/debug/os-hub`, 327/327) carries the hub half. It was **not**
  swapped in, because no step reached presence — swapping it would have changed a variable with
  nothing to measure. Every measurement in this report ran on the **03:42 `target-jc1` binary**.
- **The permanent wiring (C3 §4, brief step 7) was NOT added.** It is gated on ≥ 8 green steps and
  zero are green. Adding an nx target and a launch row now would pin a red path as a gate, for the
  same reason C3 and C4 both declined. The exact shape it should take when the steps go green: a
  `LiveCollaborationScript` beside `HubLiveSignInScript`
  (`🌎️hub/🔐️auth/🧪️tests/🤝️live-sign-in/🏃️execution/🟦️.ts`) registered as
  `os-hub:live-collaboration-check` in `🌎️hub/📦️packages/🦀️rust/📋️project.json`, and a launch row
  `⚖️gate🤝️hub-collaboration👥️two-users` at `presentation.order` 411.107585, between
  `⚖️gate🌉️os-mcp🤖️live-agent-loop` (411.10758) and the next row.
- **No cargo was run by this slice after the resume.** The hub binary is the one C5's own bootstrap
  built at 03:42/05:33; the re-activation is a wasm build, queued, not a `-p semio-hub` build.
- **The presence asymmetry (C3 §3.4) is now scored but never measured**, since step 5 did not run.

## 9. Files changed

Product source:

- `🌎️hub/📦️packages/🦀️rust/📜️script.ts` — containment gate TS program `types` (defect 1)
- `🌎️hub/📦️packages/🦀️rust/📋️project.json` — containment target env (defect 2)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧫️fixtures/🔣️.json` — `loadBytesPerMs` (defect 3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🔣️.json` — `loadBytesPerMs` required + const (defect 3)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` — `measureChildValue` admits `undefined`, split refusals, docstring (defects 4, 5)

Ticket folder (all new or repaired by C5, all permanent and parameterised):

- `📜️c5-provision.sh` — two humans, space, gis document, both memberships, on any hub data root
- `📜️c5-hub-restart.sh` — step 9's by-pid restart onto the same published catalog
- `📜️c5-activate-gis2d.sh` — the §4.2 re-activation, inside the fleet wasm mutex
- `🐍️c5-shell-boot-diagnose.mjs` — boot crash with the FULL stack (`pageerror` alone stringifies to the message)
- `🐍️c3-collab-scenario.mjs` — step 8 predicate, step 5 symmetry, new step 9 (defects 6, 7)
- `🐍️c4-actor-reason-probe.mjs` — `ACTIVE` verdict, stack capture (defect 8)
- `📓️c5-live-collaboration-over-hub.md` — this report

Captures in `🗑️generated/`: `c5-containment.txt`, `c5-provision-run.txt`, `c5-open-plan.txt`,
`c5-add-member.txt`, `c5-provision.txt`, `c5-actor-probe-user1{,-b,-c,-d,-e}.txt`,
`c5-actor-reason-user1.json`, `c5{,b,c,d,e,f}-actor-reason-user1-console.txt`,
`c5-shell-boot-diagnose.txt`, `c5-activate-gis2d.txt`, `c5-hub-7621.txt`, `c5-hub-pid.txt`,
`c5-serve-{s,gis2d}-pid.txt`, `c5-activate-pid.txt`, `c2-serve-{s-6190,gis2d-6191}.txt`.
