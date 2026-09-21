# C6 — the browser actor LOADS and ACTIVATES; where the document open still stops

Slice C6 (session 7, 2026-09-21). Inherited C5 §4.3: *"the actor's 63 717 043 bytes fetch and verify
in 31 s, then the child LOAD hangs silently — no `rejected`, no child deadline, live region stuck on
'Verifying…', proven a hang with a 900 s deadline."*

## 0. HANDOFF

| field | value |
|---|---|
| **C5 §4.3 "the load hangs"** | **WRONG diagnosis, and now disproved by measurement.** The load never hung. It was *silent*: nothing between the last fetched byte and a mounted component reported anything, and the one stage that *was* instrumented (the host's fetch) was the last thing anyone saw. §2 |
| **browser actor loads** | **YES, measured.** decode → compile → instantiate → activate, in **7.5 s** after the bytes land, on the live hub document. §2 |
| **browser actor reaches `active`** | **YES, measured.** `describe` answers, the descriptor verifies against the staged one, `openGuest` returns, the host view refreshes: stage `actor-ready 1/1` at **33.9 s** from page load. §2 |
| what still stops | The **artifact bootstrap / cold pair** never reaches `installColdPair`, so `publishMountedIfReady` never fires, `browser-actor-ui-mounted` never reaches the shell, the opening attempt never resolves and the reservation self-retires ~21 s later. §3.4 |
| scenario steps green | **0 of 10 — not run, nothing claimed.** §4 |
| gate wired | **No** — gated on ≥ 8 green steps (C5 §8), zero are green. |
| infra | hub **7621** pid 48044 (untouched, C5's); serves `s` **6190** pid 48224, `gis2d` **6191** **pid 47392** (restarted by C6 at 11:02 after the TS edits — `SEMIO_VITE_HMR=0`, so a restart is required for source to take effect) |
| containment gate | `browser-actor-child-worker-containment`: **18 laws / 13 wire / 9 fault, chromium, passed** after every source change here (`🗑️generated/` capture via the nx target's own env) |
| tsc | `tsc -p 🧰️framework/🛍️products/💻️os/tsconfig.json --noEmit`: 63 errors, **none in any file this slice changed** (all pre-existing, peer-owned; the two in `🏛️ShellHost/🟦️.tsx` shift by exactly the 5 lines this slice inserted) |

## 1. Method

The load phase was silent, so the first move was to find out whether the *bundle* could load at all,
independently of the product. `🐍️c6-actor-bundle-probe.mjs` (new, permanent) reproduces the product's
exact topology — page → worker → **nested** worker → blob-URL module import → `activate` — against
the live catalog's own `closed-actor.mjs`, with no hub, no shell and no sign-in, and stamps a
monotonic clock on every stage.

It also showed the thing the product was missing: **the generated bundle already emits its own
progress**. `closed-actor.mjs` calls `control.onProgress?.({ phase, core, completed, total })` for
every decode and compile of every embedded core module and for the instantiate — and nothing in the
product ever passed an `onProgress`.

### 1.1 The bundle in isolation — 5.3 s, no hang (`🗑️generated/c6-actor-bundle-probe.txt`)

| ms | stage | detail |
|---:|---|---|
| 162 | fetched | 63 717 043 B |
| 209 | verified | SHA-256 |
| 216 | blob | object URL created |
| 372 | imported | `activate` is a function (63 MB module parsed in **156 ms**) |
| 379–5107 | decode | `browser-actor.core.wasm` 47 416 521 B, ~960 base64 chunks, 4.7 s |
| 5112 | compile | `WebAssembly.compile(47 416 521)` |
| 5150 | **active** | actor handle returned (compile = **38 ms**) |
| 5327 | described | `invoke(["describe","describe"])` → 477 654 B in **177 ms** |

So none of C5's candidate causes hold: not `WebAssembly.compile`, not the nested worker, not the
blob import, not a memory limit (heap limit 3.76 GB, 10 MB used), not JSPI, not `base64Cutoff: 0`
(the cores are inlined, there is no external `.core.wasm` fetch). The hang was never in the bundle.

## 2. Making the load speak — product change, then the live measurement

Every stage now reports, from the child's own thread, through the store worker, onto the DOM.

| layer | file | what was added |
|---|---|---|
| vocabulary | `🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts` | `BROWSER_ACTOR_CHILD_LOAD_STAGES` (received, verified, importing, imported, decode, compile, instantiate, activating, active), `BrowserActorChildLoadProgressV1`, `isChildLoadProgress`, **`childLoadStageDeadlineMs(stage, bytes)`**; `childLoadDeadlineMs` is now exactly the sum of the stage budgets so the owner's backstop can never be shorter than the stages inside it |
| child | `🧵️child/👷️worker/🟦️.ts` | `announce()` sends a typed `progress` frame per stage (throttled to one per 4 MiB of the same stage, always on a stage change); `bundleProgress()` relays the generated bundle's own `onProgress`, which is now passed into `module.activate(...)` |
| owner | `🧵️child/🟦️.ts` | `load(bytes, onProgress?)`; `receive()` admits the `progress` frame; **every frame re-arms `childLoadStageDeadlineMs(stage)`**, and the close reason now names the stage (`deadline loading stage compile …`) instead of only the phase |
| store worker | `🏪️store/👷️worker/🟦️.ts` | the sink is wired into `child.load`, plus host-side stages `actor-transfer`, `actor-describe` (issued/answered), `actor-verify`, `actor-open`, `actor-cold`, `actor-view`, `actor-ready` |
| schema | `📇️directory/🧬️schema/🟦️.ts` | `DocumentExecutionTargetProgressStageV1` — the closed stage vocabulary, still carrying no origin, path, receipt or digest |
| shell | `🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx` | `data-semio-execution-target-stage` on the status region |

### 2.1 The live timeline (`🗑️generated/c6e-actor-probe-user1.txt`, user1, hub 7621, doc `artifact-0954e2…`)

| ms from page load | stage | note |
|---:|---|---|
| 26 296 | — | document socket open (`ws://…/socket/v1?surface=s.gis.gismap@1/*#editor`) |
| 26 336 | — | **child worker created** |
| 26 444 | `browser-actor` 58 720 256 / 63 717 043 | host still fetching the actor |
| ~26.5 s | `actor-transfer` → `actor-received` → `actor-verified` → `actor-importing` → `actor-imported` | |
| 27 948 – 30 954 | `actor-decode` 8.6 M → 21.2 M → 33.9 M / 47 416 521 | the 47 MB core decoding, ~11 MB/s |
| ~32 s | `actor-compile`, `actor-instantiate`, `actor-activating`, `actor-active`, `actor-describe 0/1 → 1/1` | |
| 32 457 | `actor-verify 0/1` | descriptor verified against the staged one |
| 33 963 | **`actor-ready 1/1`** | `openGuest` returned, cold transfer attempted, host view refreshed |

**The browser actor loads and activates on the live hub document in 7.5 s after its bytes land.**
That is the thing outcome 3 had never shown.

## 3. Defects found and fixed

| # | defect | file | state |
|---|---|---|---|
| 1 | the generated actor bundle emits decode/compile/instantiate progress through `control.onProgress` and **nothing ever passed one**, so the only part of a load that takes real time reported nothing | `🧵️child/👷️worker/🟦️.ts` | **fixed** — `onProgress: bundleProgress` |
| 2 | the child's load was bounded by ONE flat budget, so a stall had no name and the reason could not say which stage stopped | `🧵️child/🧬️schema/🟦️.ts`, `🧵️child/🟦️.ts` | **fixed** — `childLoadStageDeadlineMs`, re-armed per frame, stage named in the close reason |
| 3 | the whole activation after the load — describe, descriptor verification, guest open, cold transfer, host view — reported nothing at all | `🏪️store/👷️worker/🟦️.ts` | **fixed** — seven host stages |
| 4 | **a successfully mounted document kept announcing "Verifying document component…" forever**: `execution-target-cleared` was dispatched only on detach/close, never on `browser-actor-ui-mounted` | `🏛️ShellHost/🟦️.tsx` (mounted handler) | **fixed** — cleared on mount |
| 5 | the document-open failure path suppresses its own diagnostic: `activateDocumentBrowserActorAfterSession`'s catch emits a status only when `current`, and the reservation's self-retire (`grant.retireAtMs`) emits nothing at all, so a retired document leaves the last progress row on screen forever | `🏪️store/👷️worker/🟦️.ts` | **found, NOT fixed** — §3.4 |
| 6 | the artifact bootstrap / cold pair never reaches `installColdPair` on this document, so the mount never happens | store worker bootstrap path | **found, NOT fixed** — §3.4 |

### 3.4 What still stops the open — the mount, located to two lines

`publishMountedIfReady` (`🏪️store/👷️worker/🟦️.ts:1776`) requires a rendered UI patch, a document
binding, `documentBackboneReady` **and** `coldApplied`. All four come from `installColdPair`, which
is called from exactly one place — `installArtifactBootstrap` (`:3969`), i.e. the hub document
bootstrap. `refreshHostView`'s turn also returns early on `if (!child || !this.coldApplied) return`.
So with no cold pair: no render, no mount, no `browser-actor-ui-mounted`, `entry.resolveReady()`
never runs, and the document-opening attempt burns its whole budget (`pageerror: document opening
deadline exceeded`, observed at 244 777 ms with the derived 220 840 ms budget).

Measured consequences, all three recorded:
- the child worker is **closed at 55 217 ms**, ~21 s after `actor-ready`, by the reservation's own
  `setTimeout(this.retire, grant.retireAtMs - Date.now())` — the socket grant expiring;
- **no status is emitted** on that path (`DocumentBrowserActorReservation.close()` is silent), so the
  live region keeps the last stage row;
- the store worker is **alive and its timers fire** throughout (`worker.evaluate` answered, a
  `setTimeout` inside it resolved `timer-fired` at 62 s) — so C5's "no deadline fired" was never a
  wedged event loop.

Narrowed by elimination, each step measured:

| candidate | ruled out by |
|---|---|
| `refreshHostView` threw | its catch emits `renderer-unavailable` and closes; the live region still reads `verifying / actor-ready` at 70 s and the probe's `sawRendererUnavailable` is false |
| `refreshHostView` never entered | it is called synchronously at the end of `activate()`, and `actor-ready` — emitted **after** it — is on screen |
| `this.viewRefresh` already in flight | the only other caller is `installColdPair`, which returns at its first guard while `this.lifetime` is still null |
| the descriptor verification is slow | `decodePackValue` + `encodePackValue` + `verifyBrowserActorDescribeV1` on the real 477 852-byte descriptor: **5.6 / 19.4 / 52.3 ms** (measured in bun against the catalog's own `descriptor.semio`) |

What is left is `refreshHostView`'s inner guard `if (!child || !this.coldApplied) return` — i.e.
`state.verifiedColdPair` was still null when `activate()` reached its cold step, and the later
`installColdPair` that would have mounted it never arrives. The bootstrap live region
(`data-semio-bootstrap-status`) is polled every 1.5 s through the whole run and **never renders once**,
which is what the `Welcome.bootstrap === "None"` branch (`🏪️store/👷️worker/🟦️.ts:4049`) does: it
aborts the assembler, sets remote live and returns, emitting no UI state and creating no cold pair.

Honest caveat: a genesis bootstrap of two CAS chunks could also complete between two polls and
render nothing, so "the hub sent `None`" is the best-supported reading, **not** a hub-side
measurement. The hub's data root does hold artifact CAS chunks and manifests, and the open plan's
checkpoint carries the genesis frontier (`headEditOrdinal: 0`, empty `headEditId`, zero chain hash)
that `publishMountedIfReady` compares against — so the design clearly expects a cold pair for this
document. Deciding it needs one hub-side read, which this slice did not take because `-p semio-hub`
builds belong to the coordinator (preamble rule 26).

**The exact next step**: capture the `Welcome` frame on
`ws://127.0.0.1:7621/spaces/…/documents/…/socket/v1` (the probe already logs the socket; a CDP
`Network.webSocketFrameReceived` listener on it is a five-line addition) and read its `bootstrap`
variant. If it is `None`, the document C5 provisioned over HTTP has no canonical artifact pair on
the hub and the provisioning must seed one (or the document must be created through the shell's own
creation flow, which carries a `creationMount`); if it is `ArtifactBootstrap`, the fault is in
`installArtifactBootstrap`/`installColdPair`'s race with the 8-second actor activation — note that
`installColdPair` (`:2192`) returns **silently** whenever `this.lifetime` is still null, which it
always is while the actor is loading.

## 4. The ten-step two-user scenario — NOT RUN

Not one step is claimed. Blocked on §3.4: no client holds a mounted document, so every step from
"both attached" onwards would be a tick with no content. Recording a step without its capture is
what preamble rule 6 forbids.

## 5. Files changed

Product source:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🪪️host-bootstrap/🟦️.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx`

Ticket folder:
- `🐍️c6-actor-bundle-probe.mjs` — new, permanent: the bundle's own stage timeline in the product's topology, with no hub
- `🐍️c4-actor-reason-probe.mjs` — reads `data-semio-execution-target-stage`, prints the stage timeline, traces worker and websocket lifecycle, and interrogates every page worker during a stall (thread alive? timers firing?)
- `📓️c6-browser-actor-load-and-scenario.md` — this report

## 6. Honest gaps

- **No scenario step was run and none is claimed** — no screenshots of two attached humans, no live
  edit crossing, no undo, no connection loss, no convergence, no hub restart. The harness (C5 §3) is
  ready and parameterised; the run line is C5 §5.
- **The gate was not wired** (C5 §8's `LiveCollaborationScript` + `os-hub:live-collaboration-check` +
  the `⚖️gate🤝️hub-collaboration👥️two-users` launch row at `presentation.order` 411.107585). It is
  gated on ≥ 8 green steps and zero are green; adding it now would pin a red path as a gate.
- **The mount blocker is located but not fixed**, and its last hop is an inference from a live
  region that never rendered, not a read of the `Welcome` frame (§3.4). Nothing here claims which
  side owns it.
- **Defect 5 (the silent retirement path) is found, not fixed.** Three separate silences feed it:
  `DocumentBrowserActorReservation.close()` emits no status, `activateDocumentBrowserActorAfterSession`'s
  catch emits one only when `current`, and both `publishMountedIfReady` and `refreshHostView` return
  early with no report. Fixing them is a coherent next slice and would have saved C5 and C6 together
  several hours.
- **Defect 4's fix is verified only negatively**: the clear-on-mount edit is in the tree and
  typechecks, but no document has mounted yet on this hub, so the cleared notice has not been
  observed. It is a one-line dispatch on the message that already resolves the document open.
- **`childLoadDeadlineMs` changed shape** — it is now the sum of the nine stage budgets
  (`45 000 + 4·ceil(bytes/2048)`, i.e. 169 448 ms for this 63.7 MB actor) instead of a flat
  `5 000 + ceil(bytes/2048)`. `documentOpeningDeadlineMs()` is derived from it and is therefore now
  220 840 ms. That is the admitted worst case, not the expected one: every stage is individually
  bounded at 5 000 ms (handoffs) or 36 112 ms (byte-priced), so a real stall is named and cut within
  seconds to a stage budget and the total is only a backstop.
- **No cargo was run by this slice** and no hub binary was built, swapped or restarted. Hub 7621
  (pid 48044) and the `s` serve on 6190 are exactly as C5 left them.
- **The `gis2d` serve on 6191 was restarted by C6** (old pid 48111 → **new pid 47392**) because
  `SEMIO_VITE_HMR=0` means source edits need a restart. Restart line:
  `nohup zsh "$T/📜️c2-serve.sh" gis2d 6191 http://127.0.0.1:7621 > /dev/null 2>&1 & disown`.
- **The four probe runs are on the 03:42 `target-jc1` hub binary**, as C5's were; PR1's hub-side
  presence fixes are still not in it, and no step reached presence.

## 7. Captures

`🗑️generated/`: `c6-actor-bundle-probe.{txt,json}` (§1.1), `c6-actor-probe-user1.txt`,
`c6{b,c,d,e,f,g}-actor-reason-user1.json`, `c6{,b,c,d,e,f,g}-actor-reason-user1-console.txt`,
`c6{,b,c,d,e,f,g}-actor-reason-user1.png`, `c6-serve-gis2d-pid.txt`.
