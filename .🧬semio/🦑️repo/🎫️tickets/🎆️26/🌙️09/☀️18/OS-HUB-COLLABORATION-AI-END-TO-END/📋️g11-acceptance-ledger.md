# G11 — Acceptance ledger against G10 §D's definition-of-done

Auditor G11 (Sonnet, read-only, no builds/servers/edits outside this file). Read in full:
`📓️worker-preamble.md`, `📓️status.md` (all coordinator sessions), `📓️g10-goal-gap-reaudit.md`,
`📓️fleet-4-agents.md`, `📓️fleet-5-agents.md`, and every report a row below depends on — at minimum
ds1, c1 (incl. C1c §14-21), r2, s2, m5a, m7, m4, a1, a2, au1, au3, h1 (incl. H1b), w3b, w3d, f1, b1a,
b1b, b2b (incl. B2c), b3a (incl. B3a2), b3b, g9, s1, v2, z1, t4, k2, plus the fleet-5 stubs (m5b, ob1,
wg6, o3, k3, m6).

**Method.** Each row cites the report section it is derived from and, where a capture is named, whether
that file still exists under `🗑️generated/` (checked by `ls`, not assumed). Status legend:

- **OBSERVED-AT-RUNTIME** — a live run (browser, booted server, real stdio client) produced the claimed
  behaviour, captured.
- **TEST-ONLY** — a unit/integration test proves it; no live/browser/process run has.
- **COMPILED-ONLY** — the code type-checks or `cargo check`s; no test and no run has exercised it.
- **CLAIMED-UNVERIFIED** — a report asserts it in prose with no named run, capture, or test.
- **OPEN** — confirmed not done, or explicitly `(filling)`/not started.

**Adversarial notes up front, because two reports disagree with each other or with G10:**

1. **O1-7 (gis mutations reaching the browser)** — G10 (written ~23:50 on 09-19) says "zero browser
   verification" for gis's four new mutation verbs. `📓️b3a-block-gis.md` §10 (B3a2, timestamped after
   G10) reports a live browser run: `addFeature` dispatched from the real Actions rail, ledger entry
   `create-position … index=152`, `undo`→entry 4, `redo`→entry 5, `{"mutated":true,"undone":true,
   "redone":true}`. **G10 is stale on this row; B3a2 §10 is the newer, dated, capture-backed claim** —
   scored OBSERVED-AT-RUNTIME below, not OPEN.
2. **C1c identity fix — landed but not proven by the scenario it was built for.** `📓️c1-collaboration-e2e.md`
   §14-16 describes and cites file:line for a real rewrite of the shell's identity path (hub session
   capability replacing the broker proof) and even a hand probe (`🐍️c1c-hub-hold.ts`) showing a hub
   holding at `not-ready` with two users provisioned. But §17 "Per-step observed results", §18 "the
   three behaviours", §19 "permanent wiring", §20 "Honest gaps" and §21 "Files changed" are **all
   `(filling)`, literally**. No report anywhere states a pass/fail count for `collabRunScenario`'s 10
   steps against this new identity path. Scored PARTIAL (code landed = TEST-ONLY-at-best for the
   plumbing; the actual scenario claim stays OPEN) — do not read C1c as "collaboration works."
3. **F1's session-5 bar matrix is empty.** `📓️f1-load-document-archive-replacement.md`'s "Bar matrix"
   table (6 plugins × 7 columns) is `(filling)` for every plugin except what §6.1 measured before the
   table existed (architect, full bar, PASS). vcs/writer/animate's root fixes (F1-a/b/c) are **code
   changes with zero post-fix measurement** — do not read "root fix landed" as "plugin now passes."
4. **R2's fixes are entirely unverified at runtime, by R2's own admission** (§8 gap 1: "nothing in §3–§4
   has been run"). This is the single blocker for the whole Outcome-4 mutation chain and it is
   currently in a state where even its own author does not know if it compiles.
5. **M7 found that NO capability in the entire repo is authored `destructive`**, so
   `ApprovalMode::WhenDestructive` never fires anywhere — this makes G10's O4-2 "FIXED (report), live
   round-trip still unproven" too generous: the approval gate cannot fire at all today, on any plugin,
   so there is nothing "unproven" to prove yet. Scored OPEN below (see Outcome 4, approval row), and
   flagged as an uncovered gap in §B.

---

## Outcome 1 — `dev s` frontend hosting every plugin/artifact

| item (§D) | status | evidence | owning slice now | what is still missing |
|---|---|---|---|---|
| Cold, uncontended `dev s` completes; `dist/runtime/react/dev/s/activation/🔣️receipt.json` exists | **OPEN** | `📓️g9-s-product-space-audit.md` §1.5 (two attempts, both stopped: b1b at 19/132 Nx tasks, au3 dies on missing staged `🧱️block`); `📓️s2-cold-s-boot-and-foreign-kind-open.md` §1 shows activation launched (pid 35711, 00:55) but every section past the launch line is `(filling)` — no receipt confirmed | S2 (`a195d470a598493be`, session-5 relaunch by [ce2eb8]) | a full, uncontended run to completion; S2's own report never reached its §1 table past the launch event |
| Served `s` page reports `PLAYGROUND_SESSION.plugins.length === 60` | **OPEN** | same as above — S2 §2 "Per-plugin staged/missing table" is `(filling)` | S2 | the boot itself |
| `capabilities_search` returns non-zero, differentiated hits for a representative query | **OBSERVED-AT-RUNTIME** | `📓️status.md` "Coordinator live smoke of the semio MCP" (~23:40 09-19), re-confirmed by `📓️m5a-mcp-catalog-agent-usability.md` §1 (`🗑️generated/m5a-before.txt`, exists) | M5a (`a8c34f491156e1326`) improving quality further | draw's descriptions are live (§6, `(filling)` — the "after" capture `m5a-after.txt` was never confirmed complete); note/raster/layout/forms/cad's descriptions compile but their committed `🔣️.json` predates them (A3's regeneration sweep, not started) |
| A proven-interactive plugin (`raster`) and a currently-refused plugin (`dag`/`norm`) are opened **inside the running `s` session** (not a single-plugin playground) | **OPEN** | `📓️g9-s-product-space-audit.md` §0/§1.5; S2 (the designated owner) never reached this — its §3 "Live foreign-kind open probe inside `s`" is `(filling)` | S2 | the cold boot above is the precondition |
| All 12 previously-dormant plugins pass the full interaction bar (boot→mutate→undo→redo, 0 fault lines) | **PARTIAL, 2.5–4 of 12** | see per-plugin table in §C below; confirmed full-bar: `dag`, `reasoning` (`📓️b2b-dormant-plugin-interactions.md` §1 scoreboard), `architect` (`📓️f1-load-document-archive-replacement.md` §6.1), `norm`'s `din4108` code only (`📓️b2b…md` §B2c.1 "runtime proof — din4108 clears the whole bar") | F1 (`af19e417f97ac5f08`), B2c (`ac066a8150d9104fa`), B3b (`a58899af51f4d13b8`) | `playbook` 4/5 (setActiveExample recipe written, session-5 says it "was never compiled" — §B2c session-5 note); `imperative` mutate/undo/redo proven live but 2 fault lines block full bar (same F1 `LoadDocument` dependency); `vcs`/`writer`/`animate` root-fixed but **unmeasured since the fix** (note 3 above); `mathematical`/`sequence` untouched since S1 (09-18), still refuse every document verb; `space` never booted |
| `block` boots and renders in a browser at least once | **OPEN** | `📓️b3a-block-gis.md` §13.3 "why block is still not on the bar — honest": descriptor exists, all 3 apps' dispatch chains verified complete by reading the source, but **not activated, not probed** — a machine-time blocker (cargo lock saturation), not a code defect. Session-5 continuation §15-16 ("the staged module exists"/"the full interaction bar") are both `(filling)` | B3a2 (`a074b8a1ff9328baf`) | one `activate-block2d-react-dev` + serve + probe cycle, ~ready to run per §13.3's exact recipe |
| `gis`'s four new mutation verbs dispatched from a live Actions rail | **OBSERVED-AT-RUNTIME** | `📓️b3a-block-gis.md` §10 (`🗑️generated/b3a2-gis2d-addfeature.txt`, exists) + §12 "all four gis verbs live, chained, in ONE shell session — zero faults"; see adversarial note 1 above | B3a2 | second artifact / tiled-map lane (§17, `(filling)`); wgpu side untouched (§14.1 item 4, `SceneDoc::merge_lane` has no production Rust caller) |
| wgpu renderer has a working hub sign-in/spaces/workspace surface | **OPEN, in progress** | `📓️wg6-wgpu-hub-sign-in-and-spaces.md` §1 — starting-state table only measured (no wgpu target exists for `HubSignIn`/`SpaceBrowser`/`HubConnection`; the footer pill exists from a peer but is not U1's fold and is not clickable); every design/implementation section past §1 is absent from the file entirely | WG6 (`ae6c4962e57dbd54e`, fleet-5 [56a1f0]) | the whole surface — nothing built yet |
| wasm32 browser wgpu shell can lazy-install and open a foreign-kind artifact | **OPEN, in progress** | `📓️o3-wasm32-wgpu-artifact-open-relay.md` — every section (`§0 starting state`, `§1 design`, `§2 fixes`) is `(filling)` | O3 (`a9993357c4914b931`, fleet-5) | everything — the report is a skeleton |
| `MutationKind::label()`'s 2690 call sites localized (or explicitly descoped) | **OPEN** | `📓️g1-goal-gap-audit.md` §3, `📓️g5-ux-completeness-audit.md` §1.3 item1 — untouched since first named | U3 (queued, no agent id yet — fleet-5 queue, "launch as load allows") | not started; trait-signature change, large blast radius |

## Outcome 2 — hub backend with db/presence/auth

| item (§D) | status | evidence | owning slice now | what is still missing |
|---|---|---|---|---|
| `cargo check -p semio-hub` (default features) green | **OPEN (regressed since H1)** | `📓️h1-hub-build-and-boot.md` §13 item 6 (H1b): `semio-s-artifact-stdio-semio` broken again by a live peer refactor (11× `E0308`), blocks every default-feature hub check right now (`🗑️generated/h1b-check-1.txt`). The `--no-default-features --features sqlite` lane is green (§13 item 7, `h1b-check-sqlite.txt`) | H1b (`a02e6312d906c73ac`) | this is a peer-dependency break outside H1b's own files, per preamble rule 3 not touched; needs the peer's stdio-semio rewrite to land, then a re-check |
| `cargo test -p semio-hub --lib`/`--bin os-hub` green with a recorded number | **OBSERVED-AT-RUNTIME, with named reds** | `📓️h1-hub-build-and-boot.md` §9.1: **247 passed / 32 failed / 1 timed out** of 280 (nextest, `🗑️generated/h1b-nextest-default.txt`), all 32 classified into 8 root causes (§9.3), none of them in `stores::`/`auth::`/presence lanes | H1b | R3/R4 (stale test fixtures predating a genesis-parent rule), R6 (discarded three-store failure reason), R7 (8 deadline laws, load vs. regression undetermined), R8 (5 directory/open-plan laws) — all open per §13 |
| `os-hub:test` finishes inside its level budget | **FIXED, measured** | `📓️h1-hub-build-and-boot.md` §10: 29 laws split into `quick`/`long`/`exhaustive` submodules by measured cost; `os-hub:test` projected ≈8.3 s vs 15 s budget (§10.2 table) | H1b (closed) | §10.2's own honest limit: projection is arithmetic over per-law times, not a stopwatch on the actual nx target — end-to-end timing blocked on the peer break above |
| `bun nx run os-hub:dev` boots to `/readyz` with all subsystem booleans true on a fresh `OS_HUB_DATA` | **OBSERVED-AT-RUNTIME, once, at a narrower config** | `📓️h1-hub-build-and-boot.md` §6.2 (original H1, full readiness); `📓️c1-collaboration-e2e.md` §16 (C1c) shows the **same hub, same tree, today**, holds at `artifactAuthority: {"ready":false}` under `bootstrapSecuritySmoke` — auth/directory/admin live, artifacts refused (`features.openPlan:false`) | DS1 (`ab48cda96c1824eb6`) owns the artifactAuthority gate; H1b owns re-confirming full readiness once DS1 lands | DS1's descriptor-bound fix (§ Outcome 3 row 1) is the blocker for `artifactAuthority` ever reporting ready |
| `POST /auth/sessions` mints; `DELETE /auth/sessions/me` revokes | **OBSERVED-AT-RUNTIME** | `📓️au3-live-sign-in-integration.md` §5 (38 checks/0 failed, `🗑️generated/au3-live-transcript.txt`) + §5.3 two real browsers (`au3-browser.txt`) | closed | none for the route itself; §7 gap 6 notes no os UI calls `POST /auth/credentials` yet |
| A hub request is rate-limited on auth/directory/socket-grant paths | **OBSERVED-AT-RUNTIME** | `📓️au1-hub-auth-sessions-and-rate-limit.md` §5.1 (16 unit + 7 integration, `au1-bin-test-1/2.txt`); AU3 §5 exercises the lockout live (`retry-after`) | closed | AU1 §7 gap 8: policy numbers (10/min, 10 req/s, 5/s) are a judgement call, never load-tested |
| Hub emits structured trace output for both WS handlers and the directory command path | **OPEN, in progress** | `📓️h1-hub-build-and-boot.md` §11: **0 `tracing::` call sites** under `🌎️hub` still; §12 raised the floor to `/readyz` naming closed gates + `[WARN]`/`[INFO]` startup lines (real, tested: `a_not_ready_hub_names_every_closed_gate…`), which is progress but not the tracing bar itself; `📓️ob1-hub-observability-and-store-wiring.md` — every section `(filling)` | OB1 (`a990d5f6c9172bb7e`, fleet-5) | the whole tracing instrumentation; OB1's report is a skeleton |
| Postgres and Neo4j directory/storage backends actually **run** once | **COMPILED-ONLY** | `📓️au3-live-sign-in-integration.md` §6/§7 gap 5: both features and test builds green (`au3-hub-postgres-check.txt`, `au3-hub-neo4j-check.txt`, both exist), but `docker info` fails on the dev machine — neither backend suite has ever executed | **nobody** | a Docker-capable host; see §B |
| `HubInstance: ServerInstance`'s durable stores wired to a real hub route or `Server::run` | **TEST-ONLY** | `📓️w3b-hub-instance-and-durable-stores.md` §6 (18/18 new store tests, 151/151 whole-suite; conformance suite run 3× per law from one body) + §7 "No `os-hub` process was booted against a `HubInstance`-built server, `Server::run` is still unexercised — hub's binary still constructs its own axum app" | OB1 item 3 (per its own scope line 3) | wiring a real route to the durable stores; not started (OB1 report empty) |

## Outcome 3 — collaboration between users over the hub

| item (§D) | status | evidence | owning slice now | what is still missing |
|---|---|---|---|---|
| `semio-s-plugin-stdio`'s descriptor fits (or the bound is raised); trusted catalog publishes; `artifactAuthority` ready under `DevScript` | **OPEN, in progress** | `📓️ds1-stdio-descriptor-bound.md` §9: root cause measured (32 descriptors, 31.9% duplicate bytes, puzzle over 4 MiB), fix designed and partially landed (framework compiles, §6 claim 3), **but** §9 items 1-2 (run the new laws; regenerate the schema twin) never got the build-dir lock, and item 4 ("trusted catalog publishes → hub `/readyz`") explicitly "NOT RUN … nothing in this report claims the hub booted." §10's continuation table is 5 rows, all "(running)" with no result filled in | DS1 (`ab48cda96c1824eb6`) | the two cheapest steps (run the laws, regenerate the twin) are blocked only by cargo-lock contention, per DS1's own words |
| Two distinct, hub-authenticated user identities exist in the harness and in a running `s` host | **PARTIAL — code landed, scenario unproven** | `📓️c1-collaboration-e2e.md` §14-16: real identity rewrite (hub session capability replaces the broker proof, file:line cited throughout, `hubSessionFetch`/`installHubSessionCapability`/`hubSessionPresence` all real code); `🐍️c1c-hub-hold.ts`'s output is inlined in §16 (a hub holding at `not-ready` with two users provisioned); the matching captures (`🗑️generated/c1c-hub.txt`, `c1c-hub-data.txt`, `c1c-identity.txt`, all exist) back it. §17 "Per-step observed results" is `(filling)` | C1c (`acc74c66688882845`) | running `collabRunScenario` against the new identity path and recording a number — see adversarial note 2 |
| `collabRunScenario`'s 10 steps run end to end against a real hub, real number recorded | **OPEN** | same as above; `📓️c1-collaboration-e2e.md` §9 (C1b) recorded all 10 steps "no" (unreachable); C1c's continuation never re-ran it (§17 empty) | C1c | the single most important missing artifact in this whole ticket — unchanged since G10 |
| A deliberate short connection loss is a scenario step and passes | **OPEN** | `📓️c1-collaboration-e2e.md` §12.4 (C1b): not a step at all, deliberately deferred until the scenario can reach step 1 | C1c (transitively, once the scenario runs) | scenario authoring + a pass, both blocked on the row above |
| Per-user undo is a scenario step and passes | **OPEN** | same, §12.4 | C1c (transitively) | same |
| Two simultaneous writers converge on one document, as a scenario step | **OPEN** | same, §12.4; only the replication crate's unit tests cover ordering today (G10 O3-6) | C1c (transitively) | same |
| Presence shows two distinct session colours in a real, rendered browser | **CANNOT TELL / TEST-ONLY at best** | `📓️c1-collaboration-e2e.md` §3 (wire extension, presence-color unit-tested); `📓️au3-live-sign-in-integration.md` §7 gap 3: "presence `online` is never `true` in any run I made" — a related but distinct claim (no document open in a space during AU3's proof) | **nobody explicitly** | a live two-browser run with one shared document open, both colours visible on screen |
| The wgpu native shell's collaboration path is observed running at least once | **OPEN** | no report in this ticket claims it; G10 O3-7 named it, unowned then and unowned now | **nobody** | see §B |

## Outcome 4 — AI integration over the semio MCP

| item (§D) | status | evidence | owning slice now | what is still missing |
|---|---|---|---|---|
| `.mcp.json`'s `semio` server answers `initialize`/`tools/list`/`resources/list` | **OBSERVED-AT-RUNTIME** | `📓️a1-mcp-end-to-end.md` §5 transcript (`🗑️generated/a1-client-e2e-final.txt`, `PASS os: initialize`, `PASS os: tools/list — 27 tools`) | closed | none |
| `capabilities_search` returns real, differentiated hits | **OBSERVED-AT-RUNTIME** | `📓️status.md` live smoke (~23:40 09-19); `📓️m5a-mcp-catalog-agent-usability.md` §1 before-capture | closed (M5a improving depth) | see Outcome 1 row on the same claim |
| Verb descriptions non-empty + localized; raw input events excluded from default agent-facing search | **PARTIAL** | `📓️m5a-mcp-catalog-agent-usability.md` §2-§5: audience taxonomy landed (`CapabilityAudience::{Agent,Input,Chrome}`), 6 new laws written (§4), draw fully annotated (13 verbs, EN+DE, real typed args) and **live-verified is `(filling)`** (§6: "Status: (filling) — the machine is running a 20+-cargo fleet"); note/raster/layout/forms/cad written but not regenerated into their committed descriptors | M5a (`a8c34f491156e1326`), A3 (queued, after DS1) for the regeneration sweep | §6's own live "after" capture was never completed; A3 hasn't started; `use_when` phrases are English-only (§7 gap 3) |
| Full mutation chain (`action_prepare→invoke→snapshot→undo/redo→rollback→export`) green end to end | **OPEN, fixes unverified** | `📓️r2-reactor-retained-command-owner.md` §5/§8: root cause diagnosed exactly (file:line), fix written, **"nothing in §3-§4 has been run"** — not even confirmed to compile; the single native law that would prove it (`an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving`) never got the build-dir lock in over an hour | R2 (`af983e34a31bc5012`) | run the law, then the wasm rebuilds (note/draw), then the crate test, then the nx e2e gate — R2 §6's own 4-step ladder, none of it started |
| A destructive-capability approval can be resolved through a live client round trip | **OPEN — the gate cannot fire at all today** | `📓️m7-live-agent-bridge-loop.md` §5.4: measured directly against the live catalog — `.destructive()` (the builder call that would mark a verb destructive) has **0 call sites in the whole repo**; `ApprovalMode::WhenDestructive` (56 of note's 121 capabilities) never fires. `📓️m4-mcp-bridge-approval-binding.md` §4c: 18 unit tests pass for the chain itself, but every live capability tested traps before reaching the gate (pre-A2 fix) or (post-A2) hits R2's cursor mismatch first | **nobody** — see §B and adversarial note 5 | one plugin author call (`ActionDefinition::destructive()`) + a descriptor regeneration; genuinely nobody's slice |
| React shell renders a live agent tool-call transcript from a real MCP session | **OBSERVED-AT-RUNTIME (mostly)** | `📓️m7-live-agent-bridge-loop.md` §5.2: `bun nx run @semio-tech/framework-os-mcp-rs:live-agent-loop-check` → **8 passed / 0 failed / 3 skipped of 11** (`🗑️generated/m7-live-agent-loop-gate.txt`) — rendezvous, presence, `ui_reveal`/`ui_focus` moving the real dock, tool-call rows, cancel all PASS live; the 3 SKIPs are exactly the approval steps (row above) | done (M7, not relaunched in session 5 — finished before the restart per `📓️status.md` "Done: M7 ~01:05") | wgpu parity compiles but does not link (§7, 4 unrelated peer errors) — its 3 new tests never ran; single-variant only (note, one shell, one gateway) |
| An MCP agent can edit inside a real hub space, replicate over the wire, and show up to a human collaborator | **OPEN** | `📓️m6-agent-principal-in-hub-space.md` — every section from §0 to §10 is `(filling)`; nothing landed | M6 (`ac972fc195bfb5f4e`) | everything; report is a skeleton |
| `resources/subscribe` either does something or stops being advertised | **OPEN** | `📓️m5b-mcp-protocol-conformance.md` — every section `(filling)` except the scope header | M5b (`ac65f79e11ce3348b`, fleet-5 [56a1f0]) | everything; report is a skeleton |

---

## A. Ten items whose closure moves the most rows to OBSERVED-AT-RUNTIME

Ordered by how many currently-OPEN/PARTIAL rows above each one unblocks, cheapest-looking first among ties.

1. **Run R2's native law + wasm rebuilds + the client-e2e gate** (`cargo test -p semio-framework-plugin
   --lib an_abandoned_ingress_owner`, then rebuild `note`/`draw` wasm, then
   `bun nx run @semio-tech/framework-os-mcp:client-e2e`). Unblocks: the whole Outcome-4 mutation chain
   row, the "approval round trip" claim's remaining half (M4 §4c), A1's `artifact_export`/`artifact_create`
   verification-to-guest-bytes gap, and M7's "mutation verbs remain red" honest gap. **Highest leverage
   single action in the ticket for Outcome 4.**
2. **DS1 §9 items 1-2**: run the two already-written manifest laws, then `bun ./📜️script.ts generate`.
   Both are stated as blocked only by cargo-lock contention, not by unsolved design. Unblocks: DS1 item 4
   (descriptor regeneration → trusted catalog → `artifactAuthority` ready), which in turn unblocks
   Outcome 2's `/readyz`-full-green row, Outcome 3's whole critical path, and A3's descriptor sweep.
3. **One uncontended `activate s react dev` → `serve` → `S2`'s own probe script.** Unblocks 4 Outcome-1
   rows in one run: the receipt, the `plugins.length===60` check, the foreign-kind-open-inside-`s` proof,
   and (partially) the React-activation-reason claim S2 also owns.
4. **One `activate-block2d-react-dev` + serve + probe cycle** (recipe fully specified,
   `📓️b3a-block-gis.md` §13.3). Cheapest remaining Outcome-1 plugin gap — pure machine time.
5. **Author `ActionDefinition::destructive()` on ~5-10 genuinely destructive verbs** (e.g. note's
   `deleteSelection`, the three `remodel.clear*`, `lowpoly.clearSeam`) + regenerate their descriptors.
   Unblocks the entire approval-gate row for Outcome 4 — today it is unreachable on **every** plugin.
6. **Finish C1c §17-21** (run `collabRunScenario` against the new hub-session identity path, even
   partially). This is the single most-cited "most valuable missing data point" across three prior
   audits (G1, G10, this one) and the identity plumbing it needs is already built.
7. **A3's descriptor regeneration for note/raster/layout/forms/cad** (M5a already wrote the
   descriptions; A3 has zero remaining work per M5a §5's "nothing for A3 to do" analysis — it is a batch
   `describe` re-run, ~6 min/plugin). Unblocks the "verb descriptions" row fully, plus 5 of A1 §3's 30
   catalog skips.
8. **F1's post-fix re-measurement of vcs/writer/animate** (the root fixes are landed; the bar matrix
   is empty). Three plugins from OPEN/CLAIMED-UNVERIFIED to a real number with ~one activate+probe cycle
   each, reusing F1's own `🐍️f1-bar-probe.mjs`.
9. **Playbook's `setActiveExample` recipe** — written (§B2c.4) but "never compiled" per B2c's own
   session-5 note. A single `cargo check` + probe closes the 5th-of-5 gap on a plugin already at 4/5.
10. **H1b's R3 fixture fix** (the stale `publish_checkpoint_for_test` helper) — closes 1 of 8 root causes
    behind the hub's 32 test failures and, per §9.4, also removes the one 300 s test timeout that pads
    every future hub test run.

## B. §D items no running or held slice owns — new slices

1. **Postgres/Neo4j backends have never actually run** (Outcome 2). Both compile; `docker info` fails on
   the dev machine (AU3 §7 gap 5). Needs a Docker-capable host or a documented, deliberate descope —
   nobody in fleet 4 or 5 owns making that call.
2. **The wgpu native shell's collaboration path has never been observed running** (Outcome 3, last row).
   Named by G1 and G10, still unowned in the fleet-5 queue.
3. **Two real browsers rendering two distinct presence colours on one shared document** (Outcome 3).
   Adjacent to but distinct from AU3's "online never true" gap and C1's wire-level test; nobody's slice
   currently includes "open a document with two live sessions and screenshot the colours."
4. **Authoring `ActionDefinition::destructive()` on any real verb** (Outcome 4). M7 measured the gap
   precisely and explicitly declined it as "plugin authoring, outside this slice." No plugin-owning
   slice in fleet 4/5 has this in scope. This is the one blocking item for the entire approval-affordance
   surface across every plugin.
5. **`ContributionSet` duplication** (DS1 §9 item 6 — `plugin_contributions()` copying
   `manifest.topic_contributions` verbatim, gis pays 196 363 B twice). DS1 named it, did not fix it, and
   it is not in DS1's own §10 continuation table.
6. **Inlined example-document bloat** (DS1 §9 item 5 — puzzle's 3.5 MB example, stdio's 8.8 MB gif
   fixture, both counted against the same 4 MiB descriptor bound). DS1 names the correct shape
   (`AssetDeclaration`, body travels out-of-band) but "not attempted here," and nobody has picked it up.
7. **`semio_s_plugin_stdio_component.core.wasm` at 376 957 060 B**, over the 256 MiB
   `DESCRIBE_ARTIFACT_MAX_BYTES` bound (DS1 §9 item 7) — flagged as possibly stale, not investigated by
   anyone.

## C. Per-plugin proven level (all plugins from the S1 matrix, best level observed to date)

Levels, weakest to strongest: **staged** (compiles/wasm built, never booted) · **boots** (renders, no
mutating action reachable or none tried) · **example loads** (default document renders) · **mutation**
(one real document mutation dispatched from the UI) · **undo-redo** (mutation + undo, redo not
necessarily proven) · **full bar** (mutation + undo + redo + 0 console fault lines, live).

Note: S1's matrix is dated 2026-09-18; rows below are updated wherever a later report measured the same
plugin. "S1" in the source column means no later report touched it.

| plugin | best level | source | note |
|---|---|---|---|
| 🕸️dag | **full bar** | B2b §3.1 | `addNode`, undo, redo, 0 faults |
| 💡️reasoning | **full bar** | B2b §3.2 | same |
| 📕️norm (`din4108`) | **full bar** | B2c.1 "runtime proof" | only 1 of 15 codes swept; other 14 unmeasured |
| 🏛️architect | **full bar** | F1 §6.1 | `setAdjacencyKind`, undo, redo, 0 faults (was 1) |
| 🔱️trinity jack | **full bar** | B3b §1 | `patchNodes`, undo, 0 faults |
| 🀄️wfc bitmap/grid2d/wfc2d/grid3d/wfc3d (5) | **full bar** | B3b §1 | all 5 PASS, `change-seed`, undo, 0 faults |
| 🧩️puzzle 2d/3d/5d (3) | **full bar** | B3b §1 + S4.3 | all 3 PASS; 5d closed this session |
| 🌍️gis (`gismap`/`gis2d`) | **full bar** | B3a2 §10, §12 | 4 verbs live, chained, mutate/undo/redo, 0 real faults (4 env noise lines filtered); see adversarial note 1 |
| 📖️playbook | **undo-redo, 4/5** | B2b §1 scoreboard | `addStep` mutate/undo/redo proven; `setActiveExample` recipe written but uncompiled (B2c session-5 note) |
| 📜️imperative | **undo-redo** | B2c.3 | 10 verbs migrated, mutate/undo/redo live; 2 fault lines block full bar (F1's `LoadDocument` dependency) |
| 🔱️trinity rewriting | **mutation** | B3b §1 + S4.2 | `setParameter` mutate/undo proven; 1 boot fault, host-side fix built, guest wasm rebuild pending |
| 🖍️draw | **mutation** | S1 (09-18) + M5a §3 | boot + `setActiveExample` fixed same day; 3 deeper gesture-decode bugs found+fixed; native suite 191/54 (documented harness debt); M5a additionally gave it real typed args + EN/DE descriptions |
| 🖨️raster | **full bar (per S1)** | S1 | most thoroughly proven plugin in the 09-18 matrix: add/undo/hover all zero-fault; not re-verified since |
| 📋️forms | **full bar (per S1)** | S1 | zero-fault interaction probe on record |
| 🗒️note | **full bar (per S1)** | S1 | native + `interactionSelect→deleteSelection` chain proven |
| 🏗️fem (2d/3d) | **full bar (per S1)** | S1 | `addNode`, `setCamera`, `setResultDisplay` all proven |
| 🔋️energy | **full bar (per S1)** | S1 | 9-step interaction probe series on record |
| 📏️layout | **mutation (per S1)** | S1 | tree-row select dispatches; native gesture tests pass |
| 📸️remodel | **full bar (per S1)** | S1 | React+wgpu boot, dispatch confirmed per its own ticket |
| ➗️mathematical | **example loads** | S1 (unchanged since) | `setDocument`/`nodeGraphViewport` refuse; F1 did not touch this plugin |
| 🎬️sequence | **example loads** | S1 (unchanged since) | `reorganize`/`nodeGraphEdit` dispatch-failed; F1 did not touch this plugin |
| 🌿️vcs | **example loads, fix landed unmeasured** | F1 §7 (F1-a) | fold-contract fix for `incrementCounter` landed; bar matrix row is `(filling)` |
| ✒️writer | **example loads, fix landed unmeasured** | F1 §7 (F1-b) | fold-contract fix landed; also has no pane-reachable mutating verb by design (S1) |
| 🎞️animate | **boots (0 windows), fix landed unmeasured** | F1 §7 (F1-c) | framework layout defect (`"stack"` axis/discriminator confusion) fixed; bar matrix row `(filling)` |
| 🧱️block (2d/3d/5d) | **staged** | B3a2 §13 | descriptor + all 5 dispatch places verified complete by reading source; never activated — machine-time blocker only |
| 📐️cad (root + 4 ext) | **mutation (root, per S1)** | S1 | root baseline proven 09-16; 4 extensions unverified |
| 💠️lowpoly | **boots (per S1)** | S1 | last dedicated proof 08-29, not re-verified despite a same-day wasm rebuild |
| 🎥️shooting | **boots (per S1)** | S1 | proven 09-16/17, no fresh dump this ticket |
| 🌀️procedural (2d/3d) | **boots, unstable** | S1 | historically interactive; recurring vite wedge under load |
| 🌊️flow | **staged** | S1 | not probed standalone |
| 🏭️process (root + 4 ext) | **boots, unstable (root)** | S1 | root structurally OK; known vite wedge; 4 extensions each missing a descriptor |
| 🎪️demonstrator | **boots, in-progress** | S1 | active fault-fixing cluster, no dated pass/fail |
| 🪵️sourcing (root + 3 mod) | **boots (per S1)** | S1 | moderate confidence, no fresh proof this ticket |
| 🪐️space | **staged, never booted** | G9 §1.5, B1b | compiles clean incl. wasm32-wasip2; zero boot evidence this ticket; S2 is the designated (not yet successful) attempt |
| 🗄️stdio | **N/A — not a bootable plugin** | S1 | I/O codec library; own defect is descriptor generation for MCP purposes, not boot |

---

## Row counts by status, per outcome

| outcome | OBSERVED-AT-RUNTIME | TEST-ONLY | COMPILED-ONLY | PARTIAL | CLAIMED-UNVERIFIED | OPEN |
|---|---|---|---|---|---|---|
| 1 — os `s` frontend | 2 | 0 | 0 | 1 | 0 | 7 |
| 2 — hub backend | 3 | 1 | 1 | 0 | 0 | 4 |
| 3 — collaboration | 0 | 0 | 0 | 2 | 0 | 6 |
| 4 — semio MCP | 3 | 0 | 0 | 1 | 0 | 4 |
| **total** | **8** | **1** | **1** | **4** | **0** | **21** |

35 rows total (10 + 9 + 8 + 8, matching G10 §D's own checklists). No row was scored
CLAIMED-UNVERIFIED outright — every claim in G10 §D was traceable to either a real capture/test or an
explicit `(filling)`/not-started marker; the adversarial notes above instead demote several rows a
casual read of the source reports would over-credit (C1c, gis's G10-vs-B3a2 conflict, F1's bar matrix,
M7's destructive-verb finding).
