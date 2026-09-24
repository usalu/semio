# G11 — Unowned Gaps After Session 10 (read-only, 2026-09-24 ~12:45)

Auditor G11 (Sonnet, read-only, no builds/servers/edits outside this file and its required copy at
`.tmp-ticket/📓️peer-audit-g10.md`, per `.tmp-ticket-0918/📓️fleet-9-agents.md`'s own row for this slice).
Read in full: `.tmp-ticket-0918/📓️g10-goal-gap-reaudit.md` (§A/§C/§D), `.tmp-ticket-0918/📓️status.md` Session 7
onward through Session 9-bis (12:36), `.tmp-ticket/📓️work-packages.md`, and every `.tmp-ticket/📓️wp-*.md`
(all 62 files — W1/H2‑H5/C7/C8/G4‑G8/R1‑R7/P4‑P7/T1‑T7, plus the earlier collaboration/os-frontend/plugin
packages C1‑C6/O1‑O3/M10b/S14/TC5/H1/H1b summarized in `work-packages.md`). Also read the session-9‑bis WG6/N2
lane: `.tmp-ticket-0918/📓️wg6-wgpu-hub-sign-in-spaces-workspace.md` (today's skeleton), its two predecessors
(`📓️wg6-wgpu-hub-sign-in-and-spaces.md` fleet 4, `📓️wgr-wgpu-hub-and-open-relay.md` fleet 5), and
`📓️fleet-9-agents.md`. Verified a sample of the highest-leverage claims directly against the tree with `grep`/
`find`/`python3 -m json.tool` (results inline, each marked "verified").

**The tree is moving under this audit too**, same as G10. Timestamps checked at 12:43: the most recently
written `wp-*.md` files (`wp-w1.md` 12:36, `wp-h4.md` 12:07, `wp-t4.md` 11:35) were all already captured by
the reads above — no file changed under me during the write-up. Session-10's own fleet is still running
(W1, H4, C8, P5, R6, G7 per the 08:32 wave-1 resume; T1/T3/P6/T4/T5 parked), so anything dated after 12:43
today is **not** reflected here.

---

## A. G10 §D checklist — current status, newest evidence, owner

Legend: **RUNTIME** = observed live (browser, booted hub, real process) this session or later; **TESTS-ONLY**
= proven by an automated law/gate, not driven live; **UNPROVEN** = neither; owner is a running `wp-*` (session
10), a parked `wp-*` (session 10, not currently active), **WG6**/**N2** (session 9-bis, my sibling slices), or
**NOBODY**.

### Outcome 1 — working os `s` frontend, all plugins/artifacts

| G10 item | G10 (09-19/20) status | **Current status** | Evidence | Owner |
|---|---|---|---|---|
| 1. Cold `dev s` completes, leaves `🔣️receipt.json` | OPEN — file did not exist | **FIXED, RUNTIME** — the file exists on disk right now: `🧰️framework/…/🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/s/activation/🔣️receipt.json`, **verified**: `python3 -m json.tool` → `schema: semio.dev.activation/v1`, `variant: s`, `profile: dev`, **`len(plugins) == 60`** | `wp-w1.md` §4.4 12:20 "`activate-s-react-dev` retry 2 → rc=0 (760 s)… dist == staged for 60/60" | **W1** (running) |
| 2. Served page reports `PLAYGROUND_SESSION.plugins.length === 60` in browser console | OPEN | **PARTIAL, UNPROVEN live** — the receipt's plugin count is 60 (item 1), which makes this very likely to pass, but no report this session actually served `dev s` and read the browser console; W1's §4.4 proof is a receipt/staged-dir diff, not a served-page console read | — | **W1** (running) — no report claims the console check |
| 3. `capabilities_search` differentiated hits | already true (23:40 smoke, 09-19) | **STILL TRUE** — unaffected, re-confirmed indirectly: `wp-g4.md` stdio probe 17/17 exercises the same catalog machinery live | `wp-g4.md` §3.1 | n/a (standing) |
| 4. One proven-interactive (`raster`) + one refused (`dag`/`norm`) plugin opened **inside the running `s` session** | OPEN | **STILL OPEN** — no session-10 slice drove this specific probe (N3's own acceptance criterion); W1's work is build/publish, not an interactive open-inside-`s` proof | — | **NOBODY** (see §C, gap G1) |
| 5. All 12 dormant plugins full interaction bar (boot→mutate→undo→redo, 0 faults) | 2/12 done, 2 partial-blocked-on-F1, 8 open | **LIKELY IMPROVED, UNVERIFIED AS A SET** — R1/P4/P5/P6/P7 fixed dozens of native defects across most of these plugins (space, playbook, norm, generation2d/3d, dag via framework pack fixes) and F1's `LoadDocument` blocker's root cause (archive-replacement identity) is structurally addressed by H4's replica-identity fix, but **no session-10 report re-ran the "12 dormant plugins, full bar, inside `s`" browser sweep**; the closest analogue (S10/S11/S12's "sweep" from the 0918 ticket, last measured 30/35 kinds) was not continued in session 10 | `wp-p4.md`, `wp-p6.md`, `wp-h4.md` | **NOBODY** currently owns the re-sweep (see §C, gap G2) |
| 6. `block` boots and renders in a browser at least once | OPEN | **STILL OPEN at the browser layer** — T3 proved block2d/3d/5d import/export natively (222/222, 300/300, 328/328) and P6 fixed a block-5d `#[path]` compile break, but no boot-in-browser probe was run | `wp-t3.md`, `wp-p6.md` §5 | **NOBODY** (see §C, gap G3) |
| 7. `gis`'s four mutation verbs dispatched from a **live Actions rail** | OPEN (browser layer) | **FIXED, RUNTIME** — C7 drove it live: "A authors addFeature → guest-applied, 1 mutation, hub persisted (reload shows Positions 153)"; B's addFeature also applied after the cold-pair fix | `wp-c7.md` §Status table, row "A authors addFeature" | closed (was **C7**, done for this item) |
| 8. wgpu renderer has a working hub sign-in/spaces/workspace surface | OPEN | **PARTIAL, mostly TESTS-ONLY + one RUNTIME leg** — fleet-4/5 (WG6 + WGr) built and unit-tested it (68+7 laws) and **drove it live in a browser up to the sign-in button click** (pill paints, workspace opens, typing reaches ShellState, `POST /auth/sessions` never actually observed leaving the page — WGr's own open finding); session-10's own WG6 report (`📓️wg6-wgpu-hub-sign-in-spaces-workspace.md`) is a fresh skeleton, every section still `(filling)`, **no session-10 progress landed yet** | `wgr-wgpu-hub-and-open-relay.md` §5/§5b (09-20); `wg6-wgpu-hub-sign-in-spaces-workspace.md` (today, empty) | **WG6** (session 9-bis, started, no landed work yet) |
| 9. wasm32 browser wgpu shell can lazy-install and open a foreign-kind artifact | OPEN | **PARTIAL, source landed, TESTS-ONLY** — **verified directly**: `handle_open_artifact_relay`/`switch_to_app`/`open_artifact_relay_target` in `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (lines ~566/577/11373/12202 today) are **not** wasm32-gated — the doc-comment reads "Both targets. The relay has two halves and only the second one is native…"; this matches WGr's fleet-5 finding that O3's 8 seams all landed. **Live browser proof (lazy-install + open a second, foreign-kind artifact) is still not observed** — WGr's own table: "not observed. The install path … no browser run exercised it." No N2 report exists yet | grep verified this pass; `wgr-…md` §2, §5 "Not observed" table | **N2** (session 9-bis, no report file exists — not started) |
| 10. `MutationKind::label()` 2690 sites localized | untouched at G10's time | **LIKELY FIXED, mixed evidence** — this was actually done in the 0918 ticket's fleet 7 (U3/U3b: 2795 call sites, `semio-framework-plugin --all-targets` green, German rows still "unobserved live" at U3b's own writing); S11 (fleet 8) then measured "locale switch re-renders the whole ledger… measured en→de" **live**. Session 10 did not re-touch this | `status.md` "Session 7" U3/U3b entries; S11 in the "23:1x" 09-22 entry | closed before session 10 (U3b + S11); not re-verified this pass |

### Outcome 2 — working server hub backend

| G10 item | G10 status | **Current status** | Evidence | Owner |
|---|---|---|---|---|
| `cargo check -p semio-hub` green | blocked twice, unconfirmed since | **FIXED, RUNTIME** — H2/H3/H5 all show `cargo check -p semio-hub --bin os-hub --tests` EXIT 0 repeatedly on the current tree | `wp-h2.md`, `wp-h3.md` §Evidence, `wp-h5.md` §Evidence | closed (H2→H3→H5 chain, done) |
| `cargo test -p semio-hub --lib`/`--bin` green, real number | unmeasured | **FIXED, RUNTIME** — hub `test quick` **327/327**, `test long` **336/336** on the final H5 binary; re-confirmed again by R6 (327/327) and G6/G7's live-agent-loop runs against the same hub family | `wp-h5.md` §Evidence; `wp-r6.md` §Evidence | closed |
| `os-hub:test` inside its level budget | unmeasured (0918: 301/301 in 8.1 s vs 15 s) | **STILL TRUE**, not re-measured this session but nothing regressed the suite size materially | 0918 `ht16` | standing |
| `bun nx run os-hub:dev` boots to `/readyz` all-true on fresh data | demonstrated once (H1 §6.2) | **FIXED, RUNTIME, repeatedly** — H2/H3/H5 each publish a fresh binary and boot it; H5's graceful-shutdown e2e additionally proves clean restart on sqlite/postgres/neo4j | `wp-h5.md` §"Zero-touch e2e per backend" | closed |
| `POST /auth/sessions` mint + `DELETE …/me` revoke | done (AU3) | still done, unaffected | — | standing |
| Rate-limited auth/directory/socket-grant paths | done (AU1) | still done, unaffected | — | standing |
| Hub emits structured trace for the 2 WS handlers + directory command path | OPEN (N8) | **STILL OPEN** — no session-10 slice touched tracing/observability; the 0918 `OB1r` admin-counters bar is the only precedent and it predates session 10 | — | **NOBODY** (see §C, gap G4) |
| Postgres and Neo4j **actually run** at least once | compile-only, Docker unavailable at G10's time | **FIXED, RUNTIME, thoroughly** — H2: sqlite 2/2, postgres FAIL (no WAL fence) at H2's stage; H3: **root-caused + fixed** the WAL-writer fence for both backends (session advisory lock / lease+fencing token), then sqlite/postgres/neo4j **2/2 each** live two-client e2e with independent `psql`/`cypher-shell` cross-checks; H5 added graceful shutdown + the same 2/2×3 backends again, plus `directory-live-lanes`: postgres 12/12, neo4j 7/7, corpus 3/3; R6 re-ran the whole suite green a third time | `wp-h2.md`, `wp-h3.md` §Evidence (`two-client-postgres-1.txt`, `two-client-neo4j-1.txt`), `wp-h5.md` §Evidence | closed (H2→H3→H5, thoroughly done) |
| `HubInstance: ServerInstance`'s durable stores wired to a **real hub route**, not just unit-tested | not yet load-bearing (W3b's own gap) | **FIXED, RUNTIME** — H2's whole design *is* "one route, one owner: hub" — the document WS is mounted directly on hub's own router against hub's own `db::Database`, and the old framework `HubDocumentAuthority` is **deleted** | `wp-h2.md` §"Design decision" | closed |

**New Outcome-2 findings not in G10's ledger:**
- **O2-5b** (`ProjectionStore`/`SessionStore` writes returning `()`), **O2-10** (authorization as 5 hand functions), **O2-11** (`checkpoint-publications` dead-or-not) — **all still untouched**; `checkpoint-publications` **verified still present** at `🌎️hub/🏗️bootstrap/🦀️.rs:10326` with no caller found this pass either (grep for callers in os product code found none). **NOBODY** (see §C, gap G5).
- A **new** gap H5/R4/R6 found and only *partly* closed: plain in-process `cargo test -p semio-framework-os-kernel-db --lib` (as opposed to `nextest`, which runs one process per test) is **still not green** — R4 got nextest to 765/765 and R6 to 766/766, but the in-process gate hangs/fails on architectural grounds (process-global DB-I/O backend credit, engines dropped without `close_step`). R6/R4 both name this "a separate, crate-wide work package, not a regression." **NOBODY** owns closing it (see §C, gap G6).

### Outcome 3 — collaboration between users over the hub

| G10 item | G10 status | **Current status** | Evidence | Owner |
|---|---|---|---|---|
| stdio descriptor fits the 4 MiB bound, trusted catalog publishes | OPEN, DS1 stub | **SUPERSEDED / PARTIAL** — session 9-bis's own status.md says DS1 → W1; W1's own catalog work hit a **different** blocker (18 packages had no `record_spec()` for pack-schema identity — fixed by P4/P5), not the literal descriptor-byte bound; NB1 (0918 fleet 8) separately shrank norm's *component* (not descriptor) below its 256 MiB bound. **Catalog A (stdio,gis,note,draw,writer,puzzle) publish has not finished** — W1's own log ends "12:35 SIGKILL again… Catalog A now runs as warm… up to 3 retries" | `wp-w1.md` §4.2, §4.4 | **W1** (running, catalog publish in flight/retrying) |
| Two hub-authenticated identities inside the collaboration harness **and** inside a running `s` host | not started (C1c) | **PARTIAL, RUNTIME** — C7's two-headless-Chromium scenario has two real signed-in users, distinct hub colours, both rosters populated; this is the harness-level proof G10 wanted. Inside the actual `s` host specifically: not attempted this pass | `wp-c7.md` §Status | **C7** (in progress, blocked below) |
| `collabRunScenario` 10/10 with a recorded number | never run | **STILL NOT 10/10, but real partial progress with a number**: C7 measured **1a,1b,1c,1d,5,1e PASS; both users author edits; B ingests A's edit** — then blocked on a guest-side panic (`commitCheckpoint`), root-caused and **fixed at the native layer** by C8, pending a guest rebuild from W1 | `wp-c7.md` §Status table; `wp-c8.md` §Item 1 | **C7/C8** (both running, blocked on **W1**'s guest rebuild — see collision note below) |
| Deliberate short connection loss as a scenario step | deferred, correctly | still correctly deferred (scenario can't reach step 1 cleanly yet) | — | blocked, same chain |
| Per-user undo as a scenario step | deferred | still deferred | — | blocked, same chain |
| Two simultaneous writers converging, scenario step | deferred | **PARTIAL, RUNTIME**: C7 observed "B ingests A's edit live (`MergeReport`, canvas changed)" — real convergence, but the **inspector panel stays stale on remote ingest** (fixed natively by C8 item 2, pending the same guest rebuild) | `wp-c7.md`; `wp-c8.md` §Item 2 | same chain |
| Presence shows two distinct session colours, rendered browser | wire-level only | **FIXED, RUNTIME** — C7: "both peers in both rosters, distinct hub colours" | `wp-c7.md` §Status row 1d | closed |
| wgpu native shell's collaboration path observed running | never | **STILL NEVER** — no session-10 or 9-bis slice drove a wgpu collaboration scenario (WG6/N2 are chrome/relay work, not a collaboration run) | — | **NOBODY** (see §C, gap G7) |

**Blocker common to the whole outcome (verified, cross-cutting):** C7 §"Dependency note" and C8 both say the collaboration proof is capped **until W1 rebuilds the gis/draw/writer/puzzle guests carrying (a) the `commitCheckpoint` async-job fix and (b) H4's replica-identity fix (wasi:random entropy for edit/op ids)**. This is one shared dependency for O3-3 (10-step scenario), O3-6 (convergence step), and — see below — Outcome 4's ledger-advance gap.

### Outcome 4 — AI integration over the semio MCP

| G10 item | G10 status | **Current status** | Evidence | Owner |
|---|---|---|---|---|
| `initialize`/`tools/list`/`resources/list` | done | still done | — | standing |
| `capabilities_search` differentiated hits | done | still done | — | standing |
| Verb descriptions non-empty/localized, raw input events excluded from default search | OPEN (M5a just started) | **UNCLEAR — no session-10 slice named** (M5a/M5/M6 were 0918-ticket items; session-10's `work-packages.md` does not list a successor). **Likely still open.** | — | **NOBODY** in session 10 (see §C, gap G8) |
| Full mutation chain `action_prepare→…→artifact_export` green end to end through `.mcp.json` | blocked on retained-owner defect (R2/N4) | **FIXED, RUNTIME** — **verified via the live stdio probe transcript** in `wp-g4.md` §3.1: `action_prepare addBlock → action_invoke SUCCEEDED → artifact_snapshot (spr 223→718) → history_undo → history_redo → transaction begin→commit → begin→rollback` all real, live, over stdio. **`client-e2e` 38/38** confirmed independently by G4, G5, G6, G7 (each re-ran it green on their own tree state) | `wp-g4.md` §0/§3.1; `wp-g5.md`/`wp-g6.md`/`wp-g7.md` §Headline "client-e2e 38/38" | closed — this is the single largest change since G10 |
| Destructive-capability approval resolved through a **live client round trip** | mechanism built, never proven live | **PARTIAL** — cancellation (a close cousin) is now live-proven repeatedly (G4 in-flight cancel 3 ms, G5 10 ms, G6 4–17 ms, G7 1 ms for `artifact_create`); the approval/elicitation *accept* path specifically was not re-driven this session | `wp-g4.md`–`wp-g7.md` cancellation rows | not explicitly re-tested; likely still the same "mechanism built, not live-proven for approval" state |
| React shell renders a live agent tool-call transcript | mechanism built (M7), live (a)–(e) not run | **UNCHANGED** — no session-10 slice drove the React shell against a live MCP session; C7 owns browser collaboration but did not touch the agent-chat surface | — | **NOBODY** in session 10 (0918's M7 item 3 was never picked back up) |
| MCP agent edits inside a real hub space, replicates, shown to a human collaborator | credential-delegation code exists, nothing calls it | **PARTIAL, RUNTIME, with a hard new finding** — G4 **drove it live**: agent signs in via `--hub` delegated session, opens the same note document a human holds open, commits `addBlock` → **SUCCEEDED**, and **the human's live socket receives the agent's `Commands` frame** (real relay, real actor id). **But the hub ledger does not advance** (`head_seq` stays 1, a fresh joiner's catch-up replays only 1 frame after 4+ accepted agent edits) — H4 root-caused this exactly: the guest mints a **constant** edit id per process (no per-session entropy), so the hub's dedup-by-command-id treats every agent edit as an idempotent replay. **Fixed in source** (H4: wasi:random entropy, replica-scoped edit/op ids, hub refuses colliding-content resends) but **not yet live**, because it needs the same W1 guest rebuild the Outcome-3 blocker needs | `wp-g4.md` §4 "Hub Agent Participant Live"; `wp-h4.md` §"Root cause"/§"Fix" | **G4 done measuring; H4 done fixing in source; blocked on W1's rebuild** (same shared dependency as Outcome 3) |
| `resources/subscribe` does something or stops advertising `subscribe: true` | OPEN, M5 not started | **LIKELY FIXED — stale G10 claim, verified in tree**: `🌉️mcp/🧭️protocol/🦀️.rs` now has a real `ResourceSubscriptions` registry (`subscriptions: Arc<crate::notify::ResourceSubscriptions>`), a `publishing_notifications_into` sink binder, and doc comments stating "`'resources':{'subscribe':true}` is a promise this server keeps rather than advertises." **No wp-* report claims this fix by name** — it is not attributable to a session-10 slice I can name (closest candidate: G4/G7's `📣️notify` module rewrites touched the same directory but for cancellation, not subscribe). **Flag: possible stale-claim correction, unverified live (no test drove an actual `resources/updated` notification reaching a client this pass)** | grep-verified this pass, `…/🧭️protocol/🦀️.rs:920-935` | unclear ownership — treat as recently and incidentally fixed |
| `notifications/progress` / `job_get`/`job_cancel` only | stated tradeoff | **IMPROVED** — `artifact_create` and inference binding are now full **jobs** with `notifications/progress` rows per phase (G6 binding phases, G7 `artifact_create` phases: resolving/reading/hashing/loading/opening/reading-document) | `wp-g6.md` §1, `wp-g7.md` §1 | progress surface materially widened; still not a general `notifications/progress` for every tool call |
| `inference_submit/…` hardcoded to gis | OPEN, M6 not started | **UNCHANGED** — wfc now has a real `inference_run` (WI1/GJ1/G4/G5/G6/G7 chain), which is a second *service*, but the hub-side `inference_submit/events/cancel/approve` quartet's GIS-only hardcode was not touched this session | — | **NOBODY** in session 10 |
| No production caller spawns `semio-os-mcp --hub` with a delegated credential | OPEN, M6 not started | **PARTIALLY OVERTAKEN BY EVIDENCE** — G4's own probe script (`wp-g4/g4-agent-edit-human-sees.ts`) *is* exactly this: it spawns the MCP with `--hub` and a delegated credential and edits a real hub document. That is a **ticket-owned proof script**, not a **product** entry point (still no non-test caller in the shipped product) | `wp-g4.md` §4 | test harness exists; product gap stands, **NOBODY** owns making it a first-class product flow |
| Duplicate `semio://workspace/artifacts` entry | cosmetic, unowned | unchanged | — | **NOBODY** |
| Hub directory "AI agent principal" concept | OPEN, M6 not started | **UNCHANGED in concept, but the wire evidence changed**: G4's live run shows the agent's actor id *is* distinguishable (`hub.v1.51fb…`, "not the human's"), so raw distinguishability already exists at the actor-id layer; a first-class *principal* concept (roster kind, UI treatment) was not built | `wp-g4.md` §4 | **NOBODY** in session 10 |
| 26/09/06 ticket's false "closed" status | annotated, not corrected | unchanged | — | **NOBODY** |

---

## B. Stale claims — explicit flags

1. **O1-1/O1-2 (`dev s` cold boot)** — G10 (and every 0918 report before it) says this "has never completed a
   cold boot." **The tree now disagrees**: a genuine 60-plugin activation receipt exists on disk, produced by
   W1 at 12:20 today. This is the single biggest correction to G10's ledger.
2. **O4-5 (retained-command-owner defect)** — G10 calls this "the single blocking defect for the whole MCP
   mutation chain," assigned to R2 whose 0918 report was "entirely `(filling)`." **The tree now disagrees**:
   the session-10 `client-e2e` suite is 38/38 and the live stdio probe runs the full mutation chain end to
   end. I could not find the exact commit/slice that fixed the retained-owner bug specifically (R2 in
   session 10 is a *different* R2, working on kernel edit-text/semio-base — a naming collision with the 0918
   R2). The fix is real; its author is unclear from the reports I read, most plausibly folded into the
   `⚛️reactor/🔄️turn/🦀️.rs` rework that FP8/FP9/FP12/FP13/S12 (fleet-7/8, 0918) or W1/P4/R1 (session 10)
   touched incidentally. **Recommend**: no further action needed on the defect itself, but the ledger should
   record who actually fixed it, for institutional memory.
3. **O4-7 (`resources/subscribe`)** — G10 lists this OPEN, M5 "not started." **The tree now disagrees**: a
   real subscription registry with a bindable notification sink exists. Unclaimed by any report I read.
4. **INF-9 (proven-eight + space re-verification)** — still listed "NOT STARTED" in G10 and I found no
   session-10 or 9-bis slice naming it either. **Still accurate**, not stale.
5. **W1's own §4.2 blocker description ("18 of 34 packages have no `record_spec()`")** was itself resolved
   between two coordinator decisions (P4 fixed 20 of the affected crates; P5 finished `process3d`), so by the
   time you read `wp-w1.md` linearly the blocker paragraph is already historical — W1's own report flags this
   correctly ("§5. Plan after coordinator decisions"), so this is **not** a stale claim, just worth noting the
   read order matters.

---

## C. Genuinely unowned remaining gaps, ordered by leverage

For each: scope, main files (grep-verified against the current tree this pass unless noted), acceptance
(observed-at-runtime, not test-only), cargo-heaviness, collision risk with the live session-10 fleet.

### G1 — Drive N3's own acceptance probe: open a proven-interactive and a refused plugin inside the *running* `s` session
- **Scope**: with W1's activation receipt now real (§A item 1), this is finally executable. G9's own probe
  script name was `🐍️g9-s-host-open-foreign-kind-probe.mjs` and was never created. Serve `dev s`, open
  `raster` (proven-interactive) and `dag` (a `BatchOnlyPendingRewrite`-refused kind) inside the one running
  session, no reboot between them.
- **Main files**: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/**` (the serve/dev script), the served
  `dist/runtime/react/dev/s/` tree (verified present this pass, 60 plugins).
- **Acceptance, observed at runtime**: browser console shows `PLAYGROUND_SESSION.plugins.length === 60`;
  `installPlugin`/`openArtifactWithAppRef` network/console lines fire for both `raster` and `dag`, with
  `dag`'s refusal surfaced visibly (not silently dropped).
- **Cargo-heaviness**: none — this is a served-page + browser probe against W1's already-built artifacts.
- **Collision risk**: low if run read-only against W1's staged output; **do not** touch W1's private target
  dir or the wasm/hub mutex. Coordinate timing with W1 in case it restages mid-probe.

### G2 — Re-run the "12 dormant plugins, full interaction bar, inside `s`" sweep
- **Scope**: the 0918 ticket's S10→S11→S12 sweep (`space`, `playbook`, `norm`, `procedural`, `sequence`, …)
  last measured 30/35 kinds; session 10's plugin-native fixes (P4–P7, R1) fixed dozens of the underlying
  defects (fold-contract violations, close-ladder faults, pack-schema identity) but nobody re-ran the
  browser sweep against the now-much-healthier native layer.
- **Main files**: the sweep probe scripts referenced in 0918 `s10`–`s13` reports (`.tmp-ticket-0918/wp-*` or
  their `🐍️*-sweep*.mjs` equivalents — not present in the session-10 `.tmp-ticket/` tree, would need porting
  or re-authoring against session-10's ports/hub).
- **Acceptance, observed at runtime**: N ≥ 30/35 kinds boot→mutate→undo→redo with 0 fault lines inside a
  live `s` session; German-locale rows rendered live (U3b's own still-open item).
- **Cargo-heaviness**: wasm rebuild for any plugin found newly-broken; otherwise a native+served-page probe.
- **Collision risk**: **high** if it reuses W1's hub/catalog — must queue behind the session-10 wasm/hub
  mutex per `wp-w1.md`'s own coordination rules (`w1-catalog.sh`, `📓️session-10-preamble.md`).

### G3 — Block boot-and-render-in-browser probe
- **Scope**: O1-8 / this session's item 6. Native tests are fully green (T3: 222/222, 300/300, 328/328); no
  boot-in-browser has ever been attempted for `block2d`/`3d`/`5d`.
- **Main files**: `✏️s/🔌️plugins/🧱️block/**`; a serve target once the plugin's descriptor is fresh (W1's own
  §5 tracks `block` as one of the ~20 `descriptor_is_fresh` reds still pending a full describe pass).
- **Acceptance, observed at runtime**: block boots, renders, and one migrated mutation verb round-trips
  visibly through the Actions rail.
- **Cargo-heaviness**: none beyond what W1 already owns (waits on W1's descriptor pass, does not need its
  own build).
- **Collision risk**: low — read-only browser probe once W1's descriptors are fresh.

### G4 — Hub observability minimal bar (N8, still literally untouched)
- **Scope**: O2-9. Add `tracing`/`tracing-subscriber`, instrument at minimum `document_ws_v1`,
  `directory_ws_v1`, and `post_directory_commands`. The 0918 `OB1r` admin-counter bar is a *lesser* substitute
  (polled counters, not structured trace lines) and predates H2's whole route-consolidation redesign (the
  route names OB1r counted may not even be the current ones — H2 deleted `HubDocumentAuthority` and remounted
  the document socket directly on hub's router).
- **Main files**: `🌎️hub/📦️packages/🦀️rust/Cargo.toml`, `🌎️hub/🏗️bootstrap/🦀️.rs` (verified present,
  currently the route-mount site per H2's own files-changed list).
- **Acceptance, observed at runtime**: a booted `os-hub` (any of H2/H3/H5's staged binaries) emits structured
  trace lines for a document-WS open/close and a directory command under `RUST_LOG=info`.
- **Cargo-heaviness**: native hub check + build only (no wasm).
- **Collision risk**: **medium** — touches `🏗️bootstrap/🦀️.rs`, which H2/H3/H4/H5 have all edited this
  session in the same region (route mounting, socket-grant admission). Whoever picks this up should read
  H5's binary first and rebase, not fork from an older one.

### G5 — `checkpoint-publications` dead-or-not decision
- **Scope**: O2-11. **Verified still present, no caller found**: `🌎️hub/🏗️bootstrap/🦀️.rs:10326` mounts
  `POST /spaces/{space_id}/documents/{document_id}/checkpoint-publications`; no os-product client call site
  was found this pass either (same as G2's independent finding in the 0918 ticket).
- **Main files**: `🌎️hub/🏗️bootstrap/🦀️.rs` (route); the os product's document-store module, if kept.
- **Acceptance, observed at runtime**: either the route is deleted and the hub's route table shrinks by one,
  or one client call site exists and round-trips once over HTTP, observed in a capture.
- **Cargo-heaviness**: none-to-native (a route deletion is a one-file hub edit; wiring a caller touches os
  product TS/Rust too).
- **Collision risk**: low, but coordinate with whoever owns `🏗️bootstrap/🦀️.rs` this week (H-slices).

### G6 — Plain in-process `cargo test -p semio-framework-os-kernel-db --lib` gate
- **Scope**: R4 got nextest (process-per-test) to 765/765, R6 to 766/766, but the literal in-process gate
  (what a developer runs with a bare `cargo test`) still fails/hangs. Root cause per both reports: (a)
  process-global DB-I/O backend credit exhausted by tests that abandon engines/catalogs/hello-sessions without
  `close_step`; (b) ~100 laws assert a process-global ledger-witness invariant that inherently races under
  shared-process parallelism. Both R4 and R6 independently call this "a separate, crate-wide work package."
- **Main files**: `🧰️framework/…/🛢️db/⚙️engine/🦀️.rs`, `🛢️db/🗄️storage/🦀️.rs` (retirement hooks R6 just
  built), and every db-crate test file that drops an engine/catalog/hello-session without closing it.
- **Acceptance, observed at runtime**: `cargo test -p semio-framework-os-kernel-db --lib` (no nextest) exits
  0, run twice in a row, in-process.
- **Cargo-heaviness**: **native check/test only, but crate-wide and slow** (R6 measured 437/107/timeout at
  580 s for one in-process attempt).
- **Collision risk**: **high** — R6/R4/R7 have all been mid-edit in exactly this crate today; anyone picking
  this up must resume from R7 (currently running, item 3/4 "IN PROGRESS": "Engines dropped without close
  retire through pre-reserved hook" / "Per-owner ledger witnesses" — this is *literally* the fix for G6,
  already half-landed under R7's name). **This is not actually unowned — R7 owns it, currently running.**
  (Downgrading this from the "unowned" list: see note below.)

### G7 — wgpu native shell's collaboration path, observed running at least once
- **Scope**: O3-7/O3 checklist item "wgpu native shell's collaboration path is observed running." Never
  attempted by any slice, 0918 or session 10. WG6's scope is hub *chrome* (sign-in/spaces), not a
  collaboration scenario; N2's scope is the artifact-open relay, not collaboration either.
- **Main files**: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (the same file WG6/N2 already edit — see collision
  note), plus a two-context wgpu harness analogous to C7's `c7-collab-scenario.mjs` but wgpu-driven.
- **Acceptance, observed at runtime**: two wgpu contexts, signed into the same hub space, exchange at least
  one edit visibly (the wgpu equivalent of C7's "A authors addFeature… B ingests" row).
- **Cargo-heaviness**: wasm32-unknown-unknown build of the wgpu renderer crate (already green per WGr §1/§4);
  no native hub build needed if it reuses a staged binary.
- **Collision risk**: **very high** — same file as WG6/N2 (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, currently
  ~26k+ lines and the site of every wgpu hub/relay edit this ticket has made). This should be sequenced
  *after* WG6 and N2 land, not parallel to them.

### G8 — MCP verb descriptions / raw-input-event exclusion from `capabilities_search` (O4-16/M5a's scope)
- **Scope**: the 0918 coordinator's own live smoke test found every capability's `description: ""` (BM25
  can't discriminate) and raw pointer/engagement events indistinguishable from document verbs. M5a (0918)
  started this and never finished; session 10's `work-packages.md` does not list a successor slice at all.
- **Main files**: the MCP descriptor source (schema-first, per M5a's own framing) — verb `description`
  fields en+de; an audience/intent classification the catalog compiler filters raw input events by.
- **Acceptance, observed at runtime**: `capabilities_search "draw rectangle"` returns a differentiated,
  non-tied top hit; raw input actions (`canvasPointerMove`, `canvasEscape`) excluded from the default result
  set.
- **Cargo-heaviness**: native MCP crate check/test only.
- **Collision risk**: low — touches descriptor/catalog source, not currently under session-10 edit by any
  read report.

### G9 — Duplicate `semio://workspace/artifacts` resource entry
- **Scope**: cosmetic, one-line, still unowned since G1 (0918).
- **Main files**: `🌉️mcp` resource registry (not re-located this pass; G10's original citation, G7§5 P2.11,
  stands).
- **Acceptance, observed at runtime**: `resources/list` returns the entry once.
- **Cargo-heaviness**: none.
- **Collision risk**: negligible.

---

## D. Note on G6 (§C) — actually owned, downgrade

On closer reading, **G6 is not unowned**: R7's own status table (§Status, items 3–4) is "IN PROGRESS" for
exactly "Engines dropped without close retire through pre-reserved hook" and "Per-owner ledger witnesses" —
the two remaining causes R6 named for the in-process gate. I am leaving G6 in the ordered list above (with
this note) rather than silently dropping it, because R7's own report is a stub (only the status table is
filled; every narrative section reads "(Filled in below as items land)") and it is not yet certain R7's
in-flight work actually closes the in-process gate versus only the two specific defects R6 handed it. Re-read
R7's report once it fills in before spinning up a competing slice.

---

## E. The one shared blocker across three outcomes

**W1's guest rebuild carrying (a) C8's `commitCheckpoint` async-job fix and (b) H4's replica-identity fix**
is the single highest-leverage item left in the whole ticket: it is the last blocker for —
- Outcome 1's block/gis-in-browser proofs (indirectly, via W1's describe/catalog chain),
- Outcome 3's `collabRunScenario` progressing past step ~7 (C7/C8),
- Outcome 4's "agent edit becomes visible in the hub ledger" (G4/H4).

This is **not unowned** — W1 is running and has already acknowledged the rebuild requests (`wp-w1/requests/`
from g4, g5, g6, g7, h4, p4, p5, p6, t3, r1 are all listed as accepted or answered in `wp-w1.md`) — but it is
worth flagging to the session-10 coordinator as the critical path, since C7, C8, G4 and H4 are all
independently blocked on the same W1 milestone.
