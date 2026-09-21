# G14 — Acceptance ledger, re-score over G13's 35 rows plus sessions 5d/5e/6/7 (2026-09-21, ~04:30)

Auditor G14 (Sonnet, read-only, no builds/servers/edits outside this file, no sub-agents). Read in
full: `📋️g13-acceptance-ledger-evening.md`, `📓️status.md` from "### Session 5d progress (19:33
actual)" to the relaunch header at the end, and every report those entries cite that postdates G13:
S5, S6, S7, S8, S9, GM1, C2, C3, C4, JC1, HS1, DB1, HT6–HT12, TC2, TC2b, A3b, CE1, PB3, FP3, plus
fleet 7's in-flight skeletons (C5, PR1, M8, S10 — all exist but read `(filling)` at the time of this
audit; PZ1/U3/G15/G16 have not written a report file yet) and `📓️fleet-7-agents.md`.

**Method note — the 02:30 wipe.** Per instruction: the user cleared `🗑️generated/` at ≈02:30
2026-09-21 (session 5e→6 boundary; `📓️status.md`'s own line, "every capture of this ticket is gone
— reports remain"). Verified directly: `ls 🗑️generated` now returns **10 files**, all from fleet 7
(launched ~03:40) — `c2-serve-{s,gis2d}-*.txt`, `c5-containment.txt`,
`coordinator-hub-{build-and-nextest,nextest-full-s7a}.txt`, `jc1-hub-dev.txt`, `pr1-hub-7611.txt`,
`s10-{home-a,hub-7641}.txt`. **Every capture cited by S5–S9, GM1, C2, C3, C4, JC1, HS1, DB1, HT6–HT12,
TC2, TC2b, A3b, CE1 and PB3 is gone.** Per the task's own instruction this is scored
**OBSERVED-per-report**, not downgraded to CLAIMED-UNVERIFIED — the narrative, line-numbers and
exact printed output in those reports are taken as read (this is consistent with G11/G13's own
method, which never required the auditor to re-run anything), but I could not independently `ls` a
single one of those captures myself, and neither can anyone else from this point forward. Only the
10 fleet-7 files above, plus `📓️fleet-7-agents.md`'s existence, were verified by my own `ls`.

Status legend (unchanged from G11/G13): OBSERVED-AT-RUNTIME / TEST-ONLY / COMPILED-ONLY / PARTIAL /
CLAIMED-UNVERIFIED / OPEN. Where two reports contradict, both are cited.

Fleet at time of writing: **C5, PR1, M8, S10** running (opus, session 7, all reports still 100%
`(filling)` — zero measured progress in any of the four as of this read); **PZ1, U3, G15, G16**
launched but have written no report file yet; a coordinator hub rebuild+rerun (`s7a`) is
**mid-compile** (`coordinator-hub-nextest-full-s7a.txt` ends mid-`Compiling`, no test result line
yet) — **HT13 has not launched** because its trigger (the rerun's Summary) has not printed.

## Outcome 1 — `dev s` frontend hosting every plugin/artifact

| item | morning → evening → NOW | evidence | what is still missing | owner now |
|---|---|---|---|---|
| Cold, uncontended `dev s` completes; receipt exists | OPEN → OBSERVED-AT-RUNTIME → **unchanged, OBSERVED-AT-RUNTIME** | S2 (G13); every session-6/7 slice (S5–S10, C2, C5) built directly on the same staged `s-react-dev` output with no re-activation needed, which is itself a reconfirmation | nothing — closed | none |
| Served `s` page reports `plugins.length===60` | OPEN → OBSERVED-AT-RUNTIME → **unchanged** | same | nothing — closed | none |
| `capabilities_search` differentiated hits | OBSERVED → OBSERVED-AT-RUNTIME, deeper → **OBSERVED-AT-RUNTIME, materially deeper, 2 diagnostics left** | A3b closed the wasm-mutex-queued rebuild G13 flagged UNOWNED: catalog diagnostics **22→2**, decodable descriptors **38→58**, descriptions **155→219**; CE1 finished the sweep: descriptors **58→58 (of 60)**, descriptions **219→410**, destructive **44→82**, undeclared gesture routes **26→5** (`📓️a3b-descriptor-sweep.md` §5, `📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §6) | the 2 survivors (`puzzle` over the 4 MiB bound — a wire-shape change, CE1 §4 measured why it is not a one-line fix; `stdio` — GM1's, deliberately untouched) | **PZ1** (assigned, no report file yet) |
| A proven plugin + a refused plugin opened **inside the running `s` session** | OPEN → PARTIAL → **PARTIAL, the local half now far exceeds the bar; the hub-catalog half still blocked mid-flight** | Local foreign-kind spawn+mutate+undo+redo inside `s`: S5 (draw/note/puzzle full round trip, 9 programs) → S6 (32/35 spawn) → **S7 (35/35 spawn, 34/35 dispatch, 22/35 full round trip)** — a total reversal of S6's "8 kinds never moved the document", which S7 showed was a reading bug, not a plugin defect (§2, see contradiction note below). Hub-catalog-driven open: S6 §4 got sign-in + the hub space INDEX rendering inside `s`, but the document row itself was 0-count when hub 7611 died mid-run; S9 root-fixed the deeper cause (`sessionIdentity` never stamped into the render/dispatch view state, §2.3) but the sweep was killed mid-run by the 02:30 clean before a full re-measure | S10's sweep re-run (≥30/35 target, still 22/35 as of S9's last honest number) and the hub-document-inside-`s` open, now that GM1's catalog is live and 7611 is more stable | **S10** (running, `(filling)`) |
| All 12 previously-dormant plugins pass the full bar | PARTIAL 2.5–4/12 → PARTIAL ≈7–8/12 → **OBSERVED-AT-RUNTIME, 12/12** | **PB2 DONE** (`📓️status.md` line 429): `sequence`'s undo was a FRAMEWORK hole (`child_history_tails`/`child_group_history_target`, undo/redo read only the parent store) and `mathematical` had three walls (extent, staging, a fail-closed footprint) — both measured live `bar:true`, 0 faults, "43 artifacts at the full bar" | nothing on the named 12 — closed | none |
| `block` boots and renders at least once | OPEN → OBSERVED-AT-RUNTIME, exceeds the bar → **unchanged** | b3a (G13), unchallenged since | nothing — closed | none |
| `gis`'s four mutation verbs dispatched from a live Actions rail | OBSERVED → OBSERVED-AT-RUNTIME, extended → **unchanged, and now the load-bearing plugin of the whole hub story** | b3a (G13); GM1's whole publish chain is built ON the gis cold-load law; C2/C3/C4/JC1's entire collaboration attempt is a gis map document | nothing — closed | none |
| wgpu renderer has a working hub sign-in/spaces/workspace surface | OPEN, skeleton → PARTIAL → **unchanged, PARTIAL, UNOWNED** | WGr (G13 §5b/§5) — nothing in sessions 5d/5e/6/7 touches wgpu at all; confirmed absent from `📓️fleet-7-agents.md` | rerun the 7 written laws; resolve the sign-in ambiguity; observe end to end | **UNOWNED — no fleet-7 slice covers wgpu** |
| wasm32 browser wgpu shell lazy-installs and opens a foreign-kind artifact | OPEN, skeleton → PARTIAL → **unchanged, PARTIAL, UNOWNED** | same | a live lazy-install + open run | **UNOWNED**, same reason |
| `MutationKind::label()`'s 2690 call sites localized/descoped | OPEN → OPEN, unchanged → **OPEN, now assigned but not started** | fleet 7 names **U3** for this (`📓️fleet-7-agents.md`); no `📓️u3-*.md` exists yet | not started | **U3** (assigned, unstarted) |

## Outcome 2 — hub backend with db/presence/auth

| item | morning → evening → NOW | evidence | what is still missing | owner now |
|---|---|---|---|---|
| `cargo check -p semio-hub` (default features) green | OPEN (regressed) → COMPILED-ONLY, closed → **unchanged, reconfirmed repeatedly** | TC2 §8/§8b (`Finished, 0 errors`), TC2b (`0 errors, 316 warnings`), JC1 §3b (`0 errors in any file JC1 changed`) — three more reconfirmations across a genesis-equality removal, a regression repair and a codegen-policy version bump | JC1's version bump means the coordinator must rebuild `os-hub` before the suite counts below mean anything again — that rebuild is **mid-compile right now** (`coordinator-hub-nextest-full-s7a.txt`, no result line yet) | coordinator / **HT13** (not yet launched — waiting on the rebuild's Summary) |
| `cargo test -p semio-hub --lib`/`--bin` green, recorded number | OBSERVED 247/32/1 → OBSERVED-AT-RUNTIME, 321—312/9 → **OBSERVED-AT-RUNTIME, best-of-day 319/2, current rerun in flight** | Chain since G13: HT6 (312/9→317/4) → HT7/HT8 (→315/6, new timeouts named) → TC2's genesis fix caused a **297/24 regression** (own §8b) → TC2b repaired it → HT9 (→315/6→311/10, HS1's in-flight edit was the noise) → **HS1 DONE**, root-fixed the pool-worker stack overflow (§0: six frames reserving 8.5 MiB on a 2 MiB stack) → suite **318/3** → HT10/HT11 → **319/2 at 00:47, no aborts, 39 s** (`📓️status.md` line 506) → HT12 phase-ordering fix → 318/3 at 02:45, one more blocker named. Session 7 (JC1's jco 1.34 bump) requires a **fresh** hub binary before this number is meaningful again; that rerun is in progress | HT12's last-named blocker (a phase alias holder in the durable-group ADOPT path) needs the coordinator rerun to re-verify; the rerun itself needs to finish compiling | coordinator, then **HT13** |
| `os-hub:test` finishes inside its level budget | FIXED, projected → unchanged, still only projected → **unchanged, still unowned** | no report since G13 stopwatch-measures it | an actual timed run | none — nobody's explicit scope |
| `bun nx run os-hub:dev` boots to `/readyz` all-true, fresh `OS_HUB_DATA` | OBSERVED once, narrower → PARTIAL, still not all-true → **OBSERVED-AT-RUNTIME, closed** | **GM1 §4c** (20:20:22): `/readyz` `status:ready`, `directory/storage/artifactCasBarrier/artifactPublication/artifactAuthority/adminAssets` ALL `ready:true`, `features.openPlan:true` — first time this gate has ever been reached, 44 min end to end | nothing on the readiness gate itself — closed. (A *different* hub, C2's copy of it, later aborted on first sign-in — that is the HS1 stack-overflow row above, not a readiness regression) | none |
| `POST /auth/sessions` mints; `DELETE /auth/sessions/me` revokes | OBSERVED → OBSERVED-AT-RUNTIME, reconfirmed → **unchanged** | M6b (G13); reconfirmed transitively by every sign-in in C2/C3/C4/GM1/S6 | nothing — closed | none |
| A hub request is rate-limited | OBSERVED → unchanged, closed → **unchanged** | no new evidence either way | policy numbers never load-tested | none, gap carried |
| Hub emits structured trace for WS handlers + directory command path | OPEN, skeleton → PARTIAL → **unchanged, PARTIAL** | no slice touched this narrowly-scoped ask since OB1r | the literal ask (`tracing::` on WS handlers/directory path) vs. what shipped (an admin counters route) is still an open scope question | **UNOWNED** |
| Postgres and Neo4j backends actually run once | COMPILED-ONLY, nobody → unchanged → **unchanged** | status.md's recorded open decision, restated verbatim at every session boundary through session 7 | a Docker-capable host or an explicit descope | **nobody — recorded open USER decision** |
| `HubInstance` durable stores wired to a real route | TEST-ONLY → OBSERVED-AT-RUNTIME → **unchanged, reconfirmed live many more times** | every sign-in/session-mint in C2/C3/C4/GM1/M6b rides this path; sagas still register no deciders (unchanged small gap) | a live saga actually draining | none for the wiring claim |

## Outcome 3 — collaboration between users over the hub

| item | morning → evening → NOW | evidence | what is still missing | owner now |
|---|---|---|---|---|
| `semio-s-plugin-stdio` descriptor fits; trusted catalog publishes; `artifactAuthority` ready | OPEN → PARTIAL, descriptor closed, publish not achieved → **OBSERVED-AT-RUNTIME, closed** | **GM1**: gis cold-load law fixed (§3, a retained-receiver rewrite), catalog **PUBLISHED**, hub 7611 `/readyz` `artifactAuthority.ready:true`, a signed-in human creates a space + a gis Map document and is issued a 200 open-plan with a document-socket receipt (§4e). This is the single biggest closure in this ledger — it was the standing blocker of outcomes 1, 2 and 3 for three consecutive audits | nothing on the publish claim itself — closed. `OS_HUB_CREDENTIAL_SIGN_IN=true` is still set only by GM1's own hold script (§4d) — every OTHER hub launcher in this ticket still cannot be signed into by a browser | none for this row |
| Two distinct hub-authenticated identities in harness + running `s` host | PARTIAL → OBSERVED-AT-RUNTIME → **unchanged, reconfirmed** | C1c (G13); C2/C3/C4/S6 all sign in two distinct humans against a live hub | nothing — closed | none |
| `collabRunScenario`'s 10 steps run end to end, real number recorded | OPEN ("single most important missing artifact") → STILL OPEN → **PARTIAL — run for the first time, with real per-step numbers** | **C2/C3/C4** built and ran a 10-step probe against the published catalog (not the original `collabRunScenario` harness by name, but the fleet's now-explicit substitute, C2 §3): step 1 (two sockets on one document) **OBSERVED**, step 4 (per-user undo) **OBSERVED but weak** (independent, not crossing), step 5 (presence colours) **PARTIAL** (observed once, asymmetric), step 8 (reload/re-attach) **OBSERVED**; steps 2/3 (live edit both ways) **NOT OBSERVED** — blocked on a named jco bug (C4 §3), now fixed upstream and proven in Bun (JC1 §3) but **not yet re-observed in a browser**; steps 6/7 (loss/convergence) vacuous until 2/3 close; step 9 (mid-edit restart) not attempted; step 10 (agent as third participant) blocked on `features.mcpWorkspace:false` | C5's republish-and-observe (running, `(filling)`) closes 2/3/6/7/9; M8 closes 10 | **C5** (running, no measured progress yet), **M8** for step 10 |
| A deliberate short connection loss is a scenario step and passes | OPEN → unchanged → **unchanged, transitively blocked on C5** | C3 §2 step 6, explicitly not run (would be vacuous while nothing crosses) | needs steps 2/3 first | C5 (in scope, unstarted) |
| Per-user undo is a scenario step and passes | OPEN → unchanged → **PARTIAL** (upgraded) | C3 §2 step 4: `undo=ok redo=ok` on each of two live sockets on the SAME document, but each client's edit lives only in its own copy — C3 itself flags this as "real but trivially so" | a crossing edit, so undo can be tested against a peer's write | C5 |
| Two simultaneous writers converge on one document, as a scenario step | OPEN → unchanged → **unchanged, transitively blocked** | C3 §2 step 7, not run | same as connection loss | C5 |
| Presence shows two distinct session colours in a real, rendered browser | CANNOT TELL/TEST-ONLY, nobody → unchanged, nobody → **PARTIAL — observed once, then found unstable** | **C3 §2 step 5**: `user1`'s roster listed BOTH peers with **distinct** colours (`rgb(137,26,26)`/`rgb(26,82,137)`, screenshots) — first time ever. But the roster is **asymmetric and decays**: `user2`'s roster stayed empty in the same run, and the asymmetry REVERSED in a later run (`c3final`) | root cause (§3.4, not found by C3); symmetric, stable presence | **PR1** (running, no measured progress yet) |
| The wgpu native shell's collaboration path is observed running at least once | OPEN, nobody → unchanged → **unchanged, UNOWNED** | WGr (G13); nothing since, absent from fleet 7 | a second wgpu session in the same space, observed | **UNOWNED** |

## Outcome 4 — AI integration over the semio MCP

| item | morning → evening → NOW | evidence | what is still missing | owner now |
|---|---|---|---|---|
| `.mcp.json`'s `semio` server answers `initialize`/`tools/list`/`resources/list` | OBSERVED, closed → unchanged → **unchanged** | reconfirmed transitively by every gate this session (S7's agent gate, CE1's client-e2e) | nothing | none |
| `capabilities_search` differentiated hits | OBSERVED → OBSERVED-AT-RUNTIME, deeper → **same row as Outcome 1**, see above | — | — | PZ1 |
| Verb descriptions non-empty+localized; raw input events excluded from default agent search | PARTIAL → PARTIAL, materially improved → **PARTIAL, nearly closed** | A3b + CE1 (see Outcome 1 row 3): descriptions 155→219→**410**, destructive 25→44→**82**, undeclared gesture routes 35→26→**5** (`puzzle`×3, `block`×1, `writer`×1) | the 5 remaining undeclared routes are per-plugin declaration work, not descriptor staleness | PZ1 (partially in scope — puzzle only; `block`/`writer` UNOWNED) |
| Full mutation chain (`action_prepare→invoke→snapshot→undo/redo→rollback→export`) green e2e | OPEN → PARTIAL, one infra blocker → **OBSERVED-AT-RUNTIME, closed** | **CE1**: `client-e2e` **13/17 → 34/36** (32 s, warm components) — the two remaining reds are the catalog's puzzle+stdio diagnostics and `wfc`'s own `job.explicit-state-machine-required` refusal, neither a mutation-chain gap; a product defect the pin exposed (`RevisionStamp.head_edit_id` unchanged after undo) was found and fixed (§2b). **S7**: the live-agent-loop-check gate against a real spawned `note` editor inside the real `s` host is **21/21, exit 0** (§6.4) — superseding WR4's 19/20 and S6's 11/21 | the wfc state-machine declaration and the puzzle/stdio catalog rows (tracked above, not blocking this claim) | PZ1 (catalog rows only) |
| A destructive-capability approval resolved through a live client round trip | OPEN → OBSERVED-AT-RUNTIME → **unchanged** | AP1/WR4 (G13); no contradicting evidence since | nothing | none |
| React shell renders a live agent tool-call transcript from a real MCP session | OBSERVED mostly → OBSERVED-AT-RUNTIME, superseded → **unchanged, superseded again by S7's 21/21** | see above | nothing | none |
| An MCP agent can edit inside a real hub space, replicate, and show up to a human collaborator | OPEN → PARTIAL, identity/delegation closed, editing half open, UNOWNED → **PARTIAL, unchanged in substance, now OWNED but unstarted** | M6b (G13) unchanged; **M8** is now explicitly scoped to exactly this ("picks up where M6b §M6b.4 step 6 stopped… turns `features.mcpWorkspace` from a hard-coded `false` into a derived readiness") but its report is 100% `(filling)` | the actual agent-edits-a-hub-document step, plus the presence-roster `agent` kind | **M8** (running, no measured progress yet) |
| `resources/subscribe` either does something or stops being advertised | OPEN, skeleton → OBSERVED-AT-RUNTIME → **unchanged** | M5br (G13); no contradicting evidence | crate's own unit-test lane still not obtained (minor, unaddressed) | none |

## NEW rows — gaps or facts that appeared after G13

| item | status NOW | evidence | owner now |
|---|---|---|---|
| Live agent gate **inside `s`**, driving a **spawned** editor (distinct from Outcome 4's headless mutation-chain row — this is inside the real host with a human-visible window) | **OBSERVED-AT-RUNTIME, 21/21** | S6 got it running for the first time (7/21, then 11/21 after a root fix to `agentBridgeInstances`, §5); **S7 closed it to 21/21, exit 0** (§6.4) — every `(f)` step drives instance **3**, the spawned `note` editor a human would see on the canvas, not the landing app. `data-semio-artifact-id` in the live DOM names the focused program | none — closed |
| Agent edit **visible to a human** inside the real `s` host (a narrower, already-met form of Outcome 4's still-open hub-space claim) | **OBSERVED-AT-RUNTIME** | S7 §6.4, step `(f5)`: "the live shell shows the same artifact… carried by `[data-semio-artifact-id]` in the live DOM" — the agent's `action_invoke` on a spawned `note` editor is reflected in the same DOM a human collaborator would be looking at. This is NOT the same claim as "inside a real HUB space" (M8's scope, still open) — it is local-`s`-only | none for the local form; M8 owns the hub-space form |
| Hub pool-worker stack overflow (killed the FIRST document socket ever opened) | **found and fixed within this window** | C2 discovered it (§0/§4: two aborts, two different pool workers, right after the first `server.document.socket`/`server.auth.session.read`); **HS1 root-fixed it** (six stack frames totalling 8.5 MiB on a 2 MiB stack; `WorkerPool` now states 64 MiB); **C3 independently re-verified 0 overflows across 42 socket events**, including two concurrent 268 s/213 s sessions | closed (HS1); DB1 found the abort is NOT document-socket-specific (see contradiction note below) — also covered by the same fix |
| jco 1.27.0 async `task.return` indirect-lift bug (the actual reason no edit has ever crossed the hub wire) | **root-caused, fixed upstream, proven in Bun, NOT yet proven in a browser** | C4 named the exact line (`closed-actor.mjs:3673`, `caseMetas[caseIdx]` undefined) and corroborated it four ways (§3.3); **JC1** found the fix is a published version bump (jco 1.27.0→1.34.0, no patch needed), landed a **build-time refusal law** (`validateAsyncTaskReturnLift`) so no such actor can ever be published again, and proved the fixed actor lifts a full 344-byte `TurnResult` in Bun (§3) — but the republish + live-browser observation is C5's unstarted work | **C5** |
| Home lists the user's studios (`collabRunScenario` step 1) | **root-caused twice over, fixed, not yet confirmed end-to-end** | DB1 found and fixed the "receipt read off the wrong reply" defect; **S8 disproved DB1's OWN follow-on diagnosis** (measured the drain is not parked at all — see contradiction note); S8 found and fixed the real typed-operation stall (a guest owner demanding 1 MiB against a 4 KiB grant); **S9 found and fixed a THIRD, deeper root** (`sessionIdentity` never stamped into two of three view-state projections, so `🪐️space`'s Home saw `None` identity and legitimately rendered its empty case) — the fix landed but the 02:30 clean killed the sweep before Home's table could be re-measured against it | **S10** |
| Two-dispatch spawned-program repaint lag inside `s` | **root-caused and fixed** | S5 found it, S6 narrowed it to "two dispatches behind" and ruled out two of three causes; **S7 found the real root** (ONE history projection shared by the whole window, so a spawned document's patches were admitted against a DIFFERENT document's cursor and silently discarded) and fixed it — sweep result went from 3/35 full round trip to **22/35** on the same probe | closed (verify held under S10's re-sweep) |
| `client-e2e` gate, exact number | **34/36** | CE1 §0/§1 — up from A3b's 13/17 and WR4's 15/17; the growth in denominator (17→36) is real: 18 new steps run for the first time (subscribe, resource-updated push, unsubscribe, snapshot, live head, undo, redo, transaction begin/rollback, inference, job rows) | PZ1 (the remaining 2 reds) |
| Hub suite, exact number | **318/3 (last coordinator-confirmed), 319/2 best-of-day; current rerun (session 7, post-JC1 codegen bump) in flight, no result yet** | see Outcome 2 row 2 | coordinator / HT13 |
| `wfc`'s inference refuses with `job.explicit-state-machine-required` | **named, not fixed** | CE1 §0/§3b — surfaced only after `wfc` was re-described on today's SDK (the prior refusal was a stale-component ABI break masking the real one) | **PZ1** (named in its scope) |
| `🧩️puzzle` under the 4 MiB catalog bound | **NOT met; shown to require a WIT-projected wire-shape change, not a one-line fix** | A3b measured 2.5× more fuel in the same 1800 s budget, still unfinished; **CE1 §4** read the whole consumer chain (`describe`, the kernel's scope contributions, catalog verification, the live shell) and found the deferred body must still cross the component ABI as part of `PluginManifest` — a tree-wide regeneration, not a framework edit | **PZ1** |
| KD1 corrected: **gis is the outlier, not stdio** — 7 (then 8, 9, 10) product sites fixed; `vcs` found to be a second, different outlier one string away from being creatable | **TC2/TC2b landed the id-space fix; the N-plugin generalisation (note, vcs) is still NOT landed** | TC2 §2 (the census), §4.2 (vcs's one-string gap), §9 (the catalog-carried-genesis design, not implemented); TC2b repaired TC2's own regression (315/6→297/24→ back to 318/3) and found an EIGHTH, NINTH and TENTH copy of the same equality | **UNOWNED** — no fleet-7 slice targets note/vcs genesis generalisation |
| Hub production posture | **real for the first time, not re-audited since G13** | G13 §D.10 already noted P4's release tarball; nothing regressed it, nothing extended it this window | **G15** (assigned, no report yet) |
| Store-reader retention defect on flow's no-durable-edit paths (child-lane redo, a refused rename) | **named, not fixed** | PB3 §3.3: "document store close awaits a retained reader or owner" — 2 of flow's remaining 3 native reds | **UNOWNED** |
| fem3d's `live_visual` reconcile spin (the guest's only signal while undo silently no-ops) | **named, not fixed; PB1's own diagnosis of this row is disproven** | see contradiction note below (PB1 vs PB3) | **UNOWNED** |

## Row counts by status, per outcome (morning → evening → NOW)

| outcome | OBSERVED-AT-RUNTIME | TEST-ONLY | COMPILED-ONLY | PARTIAL | CLAIMED-UNVERIFIED | OPEN |
|---|---|---|---|---|---|---|
| 1 — os `s` frontend | 2 → 5 → **6** | 0 → 0 → 0 | 0 → 0 → 0 | 1 → 4 → **3** | 0 → 0 → 0 | 7 → 1 → **1** |
| 2 — hub backend | 3 → 4 → **5** | 1 → 0 → 0 | 1 → 3 → **3** (pending rerun) | 0 → 2 → **1** | 0 → 0 → 0 | 4 → 0 → **0** |
| 3 — collaboration | 0 → 1 → **2** | 0 → 0 → 0 | 0 → 0 → 0 | 2 → 1 → **4** | 0 → 0 → 0 | 6 → 6 → **2** |
| 4 — semio MCP | 3 → 5 → **6** | 0 → 0 → 0 | 0 → 0 → 0 | 1 → 3 → **2** | 0 → 0 → 0 | 4 → 0 → **0** |
| **total (35 rows)** | **8 → 15 → 19** | **1 → 0 → 0** | **1 → 3 → 3** | **4 → 10 → 10** | **0 → 0 → 0** | **21 → 7 → 3** |

Outcome 3 is where nearly all of tonight's movement happened: from 6 OPEN rows this morning's-evening
to **2 OPEN now** — the trusted-catalog-publish blocker that gated everything is gone (GM1), and the
scenario has been RUN for the first time with real per-step numbers, even though most of those
numbers are not yet green. The 3 remaining PARTIAL-not-OBSERVED rows in outcome 3 (undo, presence,
the scenario itself) and the 2 still-OPEN (connection loss, two-writer convergence) are all
transitively behind ONE fact: the jco bug is fixed but not yet re-observed in a browser (C5).

**New-rows table, status only (not counted above — these did not exist as ledger rows this morning):**

| status | count |
|---|---|
| OBSERVED-AT-RUNTIME | 2 (live agent gate inside `s` 21/21; agent edit visible to a human, local form) |
| fixed-in-this-window, awaiting re-observation | 3 (hub pool-worker crash; jco task-return bug; two-dispatch repaint lag) |
| root-caused-and-fixed-pending-confirmation | 1 (Home lists studios) |
| named-not-fixed | 4 (wfc state machine; puzzle bound; flow store-retention debt; fem3d reconcile spin) |
| exact-number tracking (not a pass/fail row) | 2 (client-e2e 34/36; hub suite 318/3, rerun in flight) |
| landed-partially, generalisation not done | 1 (KD1 / N-plugin catalog) |
| unchanged | 1 (production posture) |

## A. Shortest remaining path to "observed at runtime end to end", per outcome

**The headline blocker G13 named (the wedged wasm mutex) is gone** — GM1's bootstrap finished,
published, and the mutex has been free since ≈19:33. **The new single highest-leverage fact tonight
is the reverse of G13's: everything outcome 3 still needs is now unblocked in principle (GM1's
catalog is live, JC1's jco fix is proven in Bun) and needs exactly one thing — C5 finishing its
republish and observing a live edit cross the wire in a real browser.** Every other open row in
outcome 3 is transitively behind that one observation.

### Outcome 1
1. **S10** finishes its sweep re-run (target ≥30/35, S9 fixed the identity root that was suppressing
   Home's rows) and the hub-document-inside-`s` open against GM1's now-live catalog.
2. **PZ1** closes the last 2 catalog diagnostics (puzzle, stdio) and the wfc state-machine
   declaration.
3. **U3** — `MutationKind::label()`'s 2690 call sites — assigned, zero lines of progress so far;
   this is the same "never actually started" gap G13 flagged, just now with a name attached.
4. **WGr's whole scope (rows 8 and 9) is UNOWNED by fleet 7.** Nobody relaunched it. If the
   coordinator wants wgpu parity to move again this session, this needs an explicit new slice —
   nothing in the current fleet touches wgpu at all.

### Outcome 2
1. The coordinator's hub rebuild+rerun (`s7a`) needs to finish compiling before HT13 can even
   launch — this is a hard, mechanical wait, not a design gap.
2. Once it finishes: confirm HT12's last-named blocker (the tail-undo alias in the durable-group
   ADOPT path) is actually gone, and chase whatever the fresh 1.34-binary rerun surfaces.
3. Postgres/Neo4j and the `os-hub:test` wall-clock budget remain **UNOWNED**, unchanged since this
   morning — the same standing gaps G11/G13 both named, with no fleet-7 slice picking either up.
4. The structured-trace scope question (OB1r's admin route vs. the literal WS-handler ask) is
   **UNOWNED** and has been since G13; nobody has made the call either way.

### Outcome 3 — now genuinely a single chain, not the multi-blocker chain G13 described
1. **C5** republishes the trusted catalog with the 1.34 jco actor on hub 7621 and observes a live
   edit cross the wire in a real browser for the first time. This single observation closes or
   unblocks: live edit A→B/B→A, a MEANINGFUL per-user undo (crossing, not independent), connection
   loss + catch-up, two-writer convergence, and gives the presence roster something to be tested
   against under real traffic.
2. **PR1** root-causes and fixes the presence-roster asymmetry/decay (§3.4) — independent of C5,
   can run in parallel.
3. **M8** needs `features.mcpWorkspace` to stop being a hard-coded `false` before step 10 (agent as
   third participant) is even reachable — this is M8's own first item, in progress.
4. **The wgpu native collaboration observation (row 8) is UNOWNED**, same as outcome 1's rows 8/9.

### Outcome 4
1. **PZ1** closes the last 2 client-e2e reds (catalog diagnostics, wfc's state-machine refusal) —
   the mutation chain itself is already OBSERVED-AT-RUNTIME and does not block on this.
2. **M8** is the only thing standing between "an MCP agent authenticates, delegates, is revoked" (all
   proven) and "an MCP agent edits a document inside a hub space and a human sees it" (never
   attempted by any slice, on any hub, all day) — this has now been the single largest outcome-4 gap
   across four consecutive audits (G1, G10, G11/G13, now G14), and for the first time it has an
   explicit owner with a stated plan (§2–§7 of its skeleton report), just zero measured progress yet.
3. **The `block`/`writer` undeclared gesture routes (2 of the remaining 5) are UNOWNED** — PZ1's
   scope is puzzle+stdio+wfc only.

## B. Items no running or freshly-assigned slice owns (carried + new, explicit for the coordinator)

1. **wgpu renderer, both outcome-1 rows and the outcome-3 native-collaboration row.** Nothing in
   fleet 7 mentions wgpu. If this needs to keep moving, it needs a new slice, full stop.
2. **Postgres/Neo4j.** Unchanged recorded USER decision since 01:32 the previous morning — not a
   fleet task by anyone's assignment, ever.
3. **`os-hub:test` wall-clock budget** — never anyone's explicit scope, across five consecutive
   audits now.
4. **Hub structured-trace scope question** (OB1r's admin route vs. literal WS-handler/directory-path
   `tracing::`) — an unresolved DECISION, not a build task; nobody has made the call.
5. **`semio-framework-plugin --lib`'s remaining ~65 reds** (FP3 closed 81→65; §7/§8 name the next
   root — a UI admission arena not reset between tests, "the single highest-value item left"). No
   fleet-7 slice targets it.
6. **115 orphan test suites** (K2, carried unchanged since G11).
7. **`ContributionSet` duplication** and **`stdio`'s 376 MB wasm over the 256 MiB DESCRIBE bound**
   (DS1 §9 items 5/7) — both still named, still not fixed, still not in any slice's scope. PZ1's
   "stdio skip" tolerates the symptom (excludes it from the diagnostics count) without fixing the
   underlying size.
8. **TC2's N-plugin catalog generalisation** (note, vcs) — three consecutive slices (TC1, TC2, and
   now nobody in fleet 7) have stopped at exactly this function. `vcs` is one string away from being
   the second creatable kind (§4.2) and nobody owns landing it.
9. **flow's store-retention debt** (PB3 §3.3, 2 reds: "document store close awaits a retained reader
   or owner" after a no-durable-edit operation) and **fem3d's `live_visual` reconcile spin** (PB3
   §2.4) — both need a rebuild this session's mutex budget did not reach, and neither is in any
   fleet-7 slice's stated scope.
10. **Docker image / hub data-root upgrade story** (G12/G15's territory) — G15 is assigned to
    RE-AUDIT this, not necessarily to fix it; the underlying gaps are unchanged since G12.
11. **`browser-actor-child-worker-containment`**, the chromium containment suite C4 and JC1 both
    named as "the first thing to run after the republish" — not explicitly named in any fleet-7
    slice's scope (C5's is the closest fit but does not say so by name). Worth confirming C5 actually
    runs it rather than assuming.

## Contradictions between reports, both cited

1. **PB1 vs PB3 on fem3d's undo.** PB1 (session 5) concluded fem3d's undo failure was a **probe
   artefact**: "B3f's probe pressed `addSupport` with an empty `node_id`; 3 laws green… fix the
   probe" (`📓️status.md` line 433, summarising `📓️pb1-raster-redo-fem3d-undo-flow-addwidget.md` §2).
   **PB3 disproved this at runtime**: `addSupport` cannot carry a node id at all by construction
   (§2.1), so PB3 re-ran with `addNode` instead — a verb that demonstrably moves the document
   (`edits 0→1`, the revert affordance renders) — and undo **still does nothing on all three
   routes**, with zero refusals anywhere (§2.2/§2.3). PB3 states this explicitly: "this settles the
   open question between PB1 §2 and B3f §B3f.6 in B3f's favour: fem3d's undo IS a live defect, and
   it is not an artefact of the no-op verb." Both are cited in this ledger's NEW-rows table.
2. **DB1 vs S8 on the "drain stall".** DB1 (session 6) diagnosed Home's empty table as the
   typed-operation drain parking "inside its second `await settle()`" (`📓️db1-directory-bootstrap-receipt.md`
   §4.3), and separately suspected a `lane: "UserVisible"` starvation. **S8 measured the opposite**
   with an in-page hook: "the drain is not parked… neither scheduler is starved… DB1's
   `parks in its second await settle()` and its `UserVisible`-starvation suspicion are both
   disproved" (`📓️s8-typed-operation-drain-and-undo-gaps.md` §2.1). S8 found the real root one layer
   down (a guest owner demanding a 1 MiB grant against a 4 KiB ladder step).
3. **C2 vs DB1 on what triggers the hub abort.** C2 read its crash as document-socket-specific:
   "Seconds later the hub aborted… immediately after its own `server.document.socket outcome=ok`
   line" (`📓️c2-two-user-collaboration-on-ready-hub.md` §0). **DB1 reproduced the identical abort
   with NO document socket involved at all** — "the abort needs no document socket at all; it
   follows the FIRST `server.auth.session.read` after a mint, on a plain browser sign-in… C2's crash
   #1 was read as document-socket-specific, and it is not" (`📓️db1-directory-bootstrap-receipt.md`
   §4.1). HS1's later frame-level root cause (a generic ~8.5 MB-on-2 MB-stack overflow on the
   artifact-engine open path) confirms DB1's broader reading: the trigger is opening ANY artifact
   engine turn, not the document socket specifically.
4. **S8 §3's own candidates vs S9's finding.** S8 named two candidates for "Home lists no studios"
   and left them for the next owner (an instance mismatch; `home_space_rows`' membership filter,
   §3). **S9 read the source for both and found neither exists**: `home_space_rows` has no user
   filter at all, and the bootstrap owner is provably the visible session's own instance
   (`📓️s9-home-studios-and-sweep-forms.md` §2.1). S9 found the real root by driving the guest
   directly rather than reasoning from source: a session-identity-required refusal that named the
   exact defect (§2.2/§2.3).
