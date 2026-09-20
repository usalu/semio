# G10 — Goal-Gap Re-Audit (post fleet-3/fleet-4, read-only, 2026-09-19 ~23:50)

Auditor G10 (Sonnet, read-only, no builds/servers/edits outside this file). Read `📓️status.md` in full
(all four coordinator sessions), `📓️g1-goal-gap-audit.md`, `📓️g2-hub-depth-audit.md`,
`📓️g5-ux-completeness-audit.md`, `📓️g7-mcp-agent-and-collaboration-audit.md`, `📓️g8-wgpu-parity-spec.md`,
`📓️g9-s-product-space-audit.md`, then the Honest-gaps/Open/handoff sections of every other `📓️*.md` in
this ticket (~55 reports; the 8 phase-0 `📓️audit-*.md` reports were consumed indirectly through G1/G2,
which already cross-checked them against the tree). Verification below is a mix of (a) direct grep/read
against the current tree, done by this slice, and (b) the most recent report's own claim where a fresh
check was too deep for a static pass — each row says which.

**The tree is moving under this audit.** Three new reports (`📓️r2-reactor-retained-command-owner.md`,
`📓️m5a-mcp-catalog-agent-usability.md`, `📓️g9-s-product-space-audit.md`) and a fresh `📓️status.md` entry
appeared while this slice was reading (fleet-4 is live). The coordinator's own **live MCP smoke test at
~23:40** (`📓️status.md`, "Coordinator live smoke of the semio MCP") is the single freshest, most
authoritative data point in the whole ticket — it post-dates every worker report — and is cited below
wherever it overrides an older claim.

Four outcomes: (1) working os `s` frontend, all plugins/artifacts; (2) working server hub backend (db,
presence, auth); (3) working collaboration between users over the hub; (4) working AI integration over
the user-facing semio MCP (`semio-framework-os-mcp`, not the Go repo MCP).

---

## A. Deduplicated gap ledger

Legend for the status column: **FIXED** = verified fixed on the current tree by this slice's own
grep/read; **FIXED (report)** = the most recent report claims it fixed and this slice did not
re-verify by grep (too deep for a static pass); **PARTIAL** = part of the gap closed, a real remainder
named; **OPEN** = confirmed still open by this slice's own check; **OPEN (report)** = still open per
the most recent report, not independently re-checked; **CANNOT TELL STATICALLY** = needs a build/boot
this slice could not run.

### Outcome 1 — os `s` frontend, all plugins/artifacts

| id | source §            | claim | status on current tree |
|---|---|---|---|
| O1-1 | audit-os-frontend, O1, O2, G9§1.5 | `dev s` (the host variant) has never completed a cold boot to a served page | **OPEN** — verified: no `dist/runtime/react/dev/s/activation` directory exists anywhere in the tree (checked `find . -path "*dist/runtime/react/dev/s/activation*"` → 0 hits); every activation directory present (`sequence`, `shooting`, `architect`, …) is a single-plugin playground, never `s` itself |
| O1-2 | audit-ai-mcp, M1§6.1, S1, G1§1, G9§1.4/§5.7 | `architect`'s duplicate MCP capability id (`architect.s.architect.program@1/*#editor.setAdjacencyKind`) aborts the whole capability-catalog compile → `capabilities_search` returns 0 hits for all 34 plugins | **FIXED** — the coordinator's live smoke test at ~23:40, run from this Claude Code session's own `.mcp.json` `semio` connection, shows `capabilities_search "draw rectangle"` **PASS** with real per-verb BM25 scores (16 draw verbs tied at 6.9096) — the catalog compiles today. No report documents which slice's edit fixed the duplicate id; root-cause location is still unnamed. Re-verify the fix's origin before closing this row for good |
| O1-3 | A1§3, M1§6.1, S1 | ~30 individual descriptor-drift skips (missing `artifactSchema`/`windowKindId`/`executionProtocol`, or no `🔣️.json` at all — was `block`, `stdio`, `playbook`) | **PARTIAL** — `block` now has a committed `🔣️.json` (B3a §3.2, confirmed by directory listing: `…/dist/dev/🔌️plugin-modules/🧱️block/🔣️.json` exists); `playbook` fixed same session per S1. Remaining ~28 skips (imperative/process/sourcing extension families, most window-kind-thin descriptors) unfixed; A3 (queued, cargo-heavy) is the designated fixer, not started |
| O1-4 | O2§0.5/§5.3, G8, G9§5 item5 | wasm32 browser wgpu shell has no artifact-open relay (`handle_open_artifact_relay`/`switch_to_app`/`open_document` still `#[cfg(not(target_arch="wasm32"))]`) so the compiled lazy-install door has no browser caller | **OPEN (report)** — not re-checked by grep this pass; no report claims this landed |
| O1-5 | B1a, F1 | `Effect::LoadDocument` archive-replacement fails closure/authority/retained-publication validation for every batch-A plugin's boot `setActiveExample` (architect/animate/writer identical) | **PARTIAL** — F1 shipped typed refusal diagnostics (§2, done) and the real root fix for the plain-dispatch and refresh-poll lanes not stamping the identity (§5, done, 6/6 native tests green); **F1 §6 live browser proof on architect/animate/writer is explicitly "NOT DONE"** and is fleet-4's current F1 continuation |
| O1-6 | S1, B1a/B1b/B2b/B2c, G9§2 table | 12 previously-dormant plugins (`writer`,`mathematical`,`vcs`,`animate`,`sequence`,`architect`,`reasoning`,`norm`,`playbook`,`imperative`,`dag`,`space`) at the full interaction bar | **PARTIAL, per-plugin** (see breakdown below) |
| O1-6a | ″ | `dag` | **FIXED (report)** — B2b (fleet-3, ~06:30) records "dag ✅ full bar"; 12 crate unit tests remain red (harness debt, not the interaction chain) |
| O1-6b | ″ | `reasoning` | **FIXED (report)** — same B2b entry: "reasoning ✅ full bar" |
| O1-6c | ″ | `norm` | **PARTIAL** — B2c root-caused and fixed the real defect (a hand-copied `NormOneItemPreparationFactory` mis-declared its fold-contract work-item count); `din4108` clears the full bar live in browser (0 fault lines, mutate/undo/redo all proven); the other 14 norm codes' sweep is still "⏳" in B2c's own status table |
| O1-6d | ″ | `imperative` | **PARTIAL, blocked** — migrated all 10 verbs to `Migrated`, mutate/undo/redo proven live in the browser (B2c.3); 2 fault lines remain, both `setActiveExample` refused — the same O1-5/F1 `LoadDocument` blocker, so `interactionBar: false` until F1's live proof lands |
| O1-6e | ″ | `playbook` | **PARTIAL** — 4/5 per B2b; `setActiveExample`'s `LoadDocument` recipe is the missing fifth, same F1 dependency |
| O1-6f | ″ | `writer`,`mathematical`,`vcs`,`animate`,`sequence`,`architect` | **OPEN, blocked on F1** — all six hit the identical archive-replacement fault (O1-5); `writer` additionally has no pane-reachable mutating verb by design (product decision, not a bug); `vcs` has a separate fold-contract violation on `incrementCounter`; `animate` boots with no window open at all (default-layout issue, not dispatch) |
| O1-6g | ″ | `space` | **OPEN** — never booted this ticket; compiles clean including wasm32-wasip2, zero interaction evidence |
| O1-7 | B3a§2.4/§2.5, B3a2§9 | `gis` (gismap) document mutations unreachable from the Actions rail (only `ActionKind::View` verbs staged) | **FIXED at the native/command layer, OPEN at the browser layer** — B3a2 added four fully-staged mutation verbs (`addFeature`/`moveFeature`/`renameFeature`/`deleteFeature`) with undo/redo round-trips proven by 11 new native tests (261/1, the one failure is O1-5's `LoadDocument` blocker); **zero browser verification** — the rail dispatch was never driven in a live browser this pass |
| O1-8 | B3a§3.4, S1 | `block` (block2d/3d/5d) never booted or interaction-probed | **OPEN (report)** — B3a2's per-plugin table (§6) still shows `boots+renders: —` for all three block variants; not reached this ticket |
| O1-9 | O2§5 gap1, G9§1.3/§5 item4 | React DOM host's real artifact-open activation (`createApp`, `🔌️PluginRuntime/🟦️.tsx:3047`) still hardcodes `"manual"` instead of the derived `activationReasonForAppId(appId)` (already exported) | **OPEN** — a one-line, already-typed fix; no report claims it landed |
| O1-10 | O2§5 gap2 | Guest reactor ignores `Event::Activate` entirely (`⚛️reactor/🔄️turn/🦀️.rs:801` matches it to `{}`) | **OPEN (report)** — guest-SDK decision, explicitly out of scope for every slice so far |
| O1-11 | audit-multi-plugin-hub, O1§6/O2§5, G9§1.5 | cold, uncontended `dev s` timing never measured | **OPEN** — same root cause as O1-1 |
| O1-12 | audit-os-frontend P0-2 | wgpu↔React parity gap (chord/camera 17/37, no Actions/Search pane bodies on wgpu) | **OUT OF SCOPE** for this ticket (owned by sibling ticket `WGPU-RENDERER-REACT-PARITY`) but named as the dominant blocker for "a usable wgpu-backed hub" |
| O1-13 | G1§3, G5§1.3 item1 | `MutationKind::label()` — 2690 hard-coded English strings feed the undo/history panel with zero localization (worst: norm 393, architect 268, puzzle 106, block 105) | **OPEN (report)** — a trait-signature change with a huge blast radius; explicitly scoped as its own ticket by both G1 and G5, not started |
| O1-14 | G5§1.3 item2, U1 | `ShellSync`'s `syncStatusLabel` (live/connecting/reconnecting/offline/saved/unsaved/pending) was hard-coded English | **FIXED (report)** — U1 (per `📓️status.md` "ShellSync i18n") localized it; not independently re-grepped this pass |
| O1-15 | G5 P0#1 | `🧵️TaskManager` never mounted as a real window | **FIXED** — verified: `FRAMEWORK_TASK_MANAGER_PANEL_ID` is referenced and dispatched in `🏛️ShellHost/🟦️.tsx` (`SET_PANEL_PATH` at line ~9525); **residual gap (U1's own honest gap 1): the React shell has no `ActivationRegistry` attached**, so the mounted window reads "no actor runtime is attached" — real window, no live data source in the React DOM host |
| O1-16 | G5 P0#2 | `AgentChatPanel` had no cancel affordance for an in-flight tool call/inference | **FIXED (report), with a caveat** — U1 added `AgentCancel`; M7 confirmed the frame/gateway effect are real and tested; **cancellation is cooperative, not preemptive** (U1's own honest gap 2) — a running `inference_run` is not interrupted mid-flight, only flagged and settled `Cancelled` |
| O1-17 | G5 P1#5 / O1§6.4 / O2§3 | plugin-install had no cancel button | **FIXED (report)** — O2 §3 added `ShellChromeFramePhase::PluginInstall` with a cancel hit-target on both React and wgpu (per G8 WG-4, "already closed, closed BEFORE the React work landed") |
| O1-18 | G5 P1#8 | no persistent, always-visible hub-connection indicator (only a per-document `ShellSync` popover) | **FIXED** — verified: `HubConnectionIndicator` is mounted unconditionally in `🏛️ShellHost/🟦️.tsx`'s footer builder (confirmed by G9's own independent re-grep, §3 table, "beside presence and on every device") |
| O1-19 | G5 P1#6 / U2 item6 | no multi-touch pinch-zoom/two-finger orbit in `World3dHost`/`Board2dHost`; `World3dHost` had no wheel-zoom path | **PARTIAL** — pure gesture math (22 tests) landed and is real; U2's own honest-gap #7 **corrects** the audit's "no wheel path" claim (OrbitControls already owns wheel zoom); the actual gap is narrower: the math is not wired into either viewport's pointer handlers yet (WG-8 sub-item (b), explicitly deferred as its own follow-up, not scheduled) |
| O1-20 | G5 P1#7 / U2 item7 | "tablet" is a manual override only, never auto-detected | **FIXED (report), one verification gap** — U2 landed the Rust twin (`mode_dock_device_for_width`) with a byte-identical parity test against the TS constants; U2's own honest gap: `cargo check -p` was never run against it (no borrow/generic/trait risk claimed, but unverified by rustc) |
| O1-21 | G5 P2#9 / U2 item9 | theme editor has no live contrast-ratio warning for custom colors | **OPEN (report)** — WCAG contrast API landed (`🎨️styling/🌓️theme/🟦️.ts`) but wired into no editor UI; U2 flags its own in-source vitest blocks as a pre-existing blind gate (never actually run by any config's `include`) |
| O1-22 | G5 P2#10 / U2 item10 | `🕸️Diagram` (node-graph editor) has zero role/keyboard hits — mouse-only | **PARTIAL** — React-side a11y law + `role="application"` + keyboard handler landed (U2, +9 tests); the wgpu side (`NodeGraph`) has no target at all to attach keyboard nav to (WG-11, foundational math ported, not wired) |
| O1-23 | G8 WG-6, G9§3/§4/§5 item6 | wgpu renderer has **zero** hub UI of any kind — `HubSignIn`/`SpaceBrowser`/`HubConnection`/`HubWorkspace` have no `🎯️targets/🧊️wgpu/` subfolder at all | **OPEN** — verified via G9's independent re-grep (§3); now *unblocked* (AU3 proved the hub's `POST /auth/sessions` route is live, removing G8's stated precondition) but still unscheduled — G9's own queue lists "**WG6**" next after a cold `dev s` boot |
| O1-24 | V1§6 item6, V2 | 23–46 of 65 playground variants missing a conformant React dev launcher | **FIXED (report)** — V2 (fleet-3, ~06:30 per status.md) reports "launchers 46/65 missing → 0/65 for both renderers" |
| O1-25 | V1§6 items 2,7; V2§5 items1,2 | `verify taxonomy report` cannot finish inside a normal window (~1h50m); `plugin-registry check` cannot reach zero (1836 findings, mostly `🗄️stdio`'s 615-file schema family + 892 unreachable-mount family); `verify layering`/`package-purity`/`dependencies*` long-standing backlogs | **PARTIAL, in progress** — V3a (fleet-4) reduced 1836→684 via a new `surface-schema` generator closing 519 lanes' worth of findings (own attribution: "not all mine", §0); V3b (fleet-4) is still an empty stub for its own honest-gaps/files-changed sections — its portion of the 684→? reduction is unmeasured as of this read |
| O1-26 | G1 cross-cutting, Z1 | `.devcontainer/post-create.sh` referenced but absent; zero-touch onboarding unowned | **FIXED (report), unverified end-to-end** — Z1 repointed `postCreateCommand` to `workspace:setup`; Z1's own honest gap: a real fresh-clone/fresh-container run was never executed (host saturated) |
| O1-27 | G1 cross-cutting (retracted by G5) | "no mobile layout system" | **RETRACTED** — G5 §3 found a real, dual-implemented, byte-parity mobile mode in both renderers (767px breakpoint, identical flattened dock behaviour); G1's claim was a shallow-grep false negative |

### Outcome 2 — server hub backend

| id | source § | claim | status on current tree |
|---|---|---|---|
| O2-1 | audit-hub-backend, H1 | `semio-hub` `E0560`×2 compile break; `os-hub:dev` off-by-one import | **FIXED (report)** — H1, not reproduced by any later slice |
| O2-2 | H1§1, P3, B3a | `semio-hub` on **default features** was red — first from a peer's in-flight `stdio-pdf` rewrite (45 errors), then from a gis `⚙️config`→`🎚️config` rename drift (18 errors) once the pdf break cleared | **FIXED (report), not re-confirmed this pass** — P3 closed the pdf half (9/9 pdf tests, verified real by this slice's own read of P3's checks); B3a's files-changed list shows the same gis `owner` paths P3's §5 named as the new blocker, repointed to `🎚️config`. H1b (fleet-4) is tasked to re-measure the whole hub test surface and its own report is still an empty stub for that section |
| O2-3 | G1§4, H1§7 | `cargo test -p semio-hub --lib`/`--bin` not independently re-confirmed green after H1's fixes | **OPEN, fleet-4 (H1b) in progress** — H1b's §9 "Re-measurement of the hub test surface" is unfilled |
| O2-4 | H2, W3a | `semio-framework-server` compiled but was depended on by nothing (dead code) | **PARTIAL/FIXED** — W3b gave hub the dependency and a real `HubInstance: ServerInstance` implementation with durable stores over hub's own `db::Database` (`🌎️hub/🗄️stores/🦀️.rs`, new, 742 lines, all suites green); **not yet load-bearing** — no hub route or `Server::run` uses it yet (W3b's own honest gap: "no `os-hub` process was booted against a `HubInstance`-built server") |
| O2-5 | H2§B.5 item3 | no durable `AuthorityStore`/`ProjectionStore`/`BlobStore`/`SessionStore` over hub's real storage | **FIXED (report)** — W3b, same evidence as O2-4; write-fault reporting is `()`-returned by two of the four stores still (W3d's honest gap, see O2-5b) |
| O2-5b | W3d§7 | `ProjectionStore`/`SessionStore` writes still return `()` — a journal failure cannot reach the caller | **OPEN (report)** — named as a follow-up, framework-wide ripple, not this ticket's scope per W3d |
| O2-6 | H2, W3a | `🔀️dispatch` macro cannot close a `Send`-future port, forcing 4 hand-written `dyn_enum_close!` blocks | **FIXED** — D1 landed the additive branch; verified via D1's own report showing the macro/server test suites green (`73` server tests, `41/41` macro tests) |
| O2-7 | G2§4 P0-1 | no `POST /auth/sessions` (login/session-mint) route on the hub at all | **FIXED** — verified directly: `.route(semio_hub::auth::SESSION_MINT_ROUTE, post(post_auth_session)…)` at `🌎️hub/🏗️bootstrap/🦀️.rs:8375`; AU3's live two-user browser proof exercises it end to end |
| O2-8 | G2§4 P0-2 | rate limiting completely absent (0 hits for `rate.?limit|governor|throttle` in hub) | **FIXED** — verified directly: `HubRateLimiterV1`/`RateLimitClassV1`/`rate_limit_middleware` are real, imported and wired in `🏗️bootstrap/🦀️.rs` (line 67, 6755+), covering `/auth/sessions`, `/auth/credentials`, `/auth/sessions/me` at minimum (AU1: 10/min per identity+address, 10 req/s directory commands, 5/s socket grants — policy numbers are a judgement call, not load-tested, per AU1's own honest gap 8) |
| O2-9 | G2§9 P1-3 | observability absent — 0 `tracing::` call sites, no metrics endpoint, only `println!` | **OPEN, fleet-4 (H1b) queued** — H1b's §12 "Observability minimal bar" section is unfilled; not touched by any other slice |
| O2-10 | G2§4 P1-4 | authorization is 5 hand-written predicate functions, not a reusable policy engine | **OPEN (report)** — explicitly deferred to Wave-3 step 9, not this ticket |
| O2-11 | G2§10 P1-5, K1§9 gap4 | `POST …/checkpoint-publications` route has no HTTP caller anywhere in the os product | **OPEN, confirmed twice** — G2's static grep and K1's independent re-check both agree; unresolved whether dead code or an intended-but-unbuilt feature |
| O2-12 | G2§2/§9 P1-6 | Postgres/Neo4j directory+storage backends' compile claim never reconfirmed by a fresh build | **PARTIAL** — AU1 added credential-audit methods for both backends (`append_credential_audit`/`set_password_credential` in `postgres`/`neo4j` `🦀️.rs`, both compile per AU1's own capture); AU3 confirms both feature builds are green but **neither backend suite was ever executed** — `docker info` failed on the dev machine (AU3 honest gap 5) |
| O2-13 | G2§7 P1-7 | inference is GIS-map-only; `execution_not_wired_error` fires for every other declared inference service | **OPEN (report)** — M6 (queued) is the designated generalizer; `inference_run` (the MCP-side general route, not the hub's `inference_submit/…` quartet) already dispatches non-GIS plugins at the routing layer per M2 — the hub-side job/queue lane itself remains single-service |
| O2-14 | G2§9 P2-8 | no `/healthz` vs `/readyz` split | **OPEN (report)** — K1 added a `/healthz` route but proved it only with an in-process axum test, not against a booted binary over a real socket (K1 honest gap 6) |
| O2-15 | AU1§7 gap1 | the hub-side browser-broker-proof issuer described by the schema was never implemented, and AU1 concludes it is architecturally a **relay** concern, not a hub route | **OPEN, re-scoped** — no relay owner has picked this up; AU3's honest gap 4 confirms the relay still cannot forward a per-user bearer |
| O2-16 | H1§7 item1 | one hub law still fails: `gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` — a contract mismatch between the law's expectation and the current three-store committer | **OPEN (report)** — needs the GIS-map commit owner, explicitly not H1's to fix |
| O2-17 | H1§7 item2 | `os-hub:test` cannot finish inside its own level budget (263 unscoped `fundamental` laws vs a 15s budget) | **OPEN, fleet-4 (H1b) in progress** — H1b's §10 unfilled |
| O2-18 | K1§9 gap1 | `4_gate` launch group missing ~8 families beyond the one row K1 added | **PARTIAL, fixed further** — K2 (fleet-3→4) added 34 more `4_gate` rows, census reports 0 missing in both files (K2 §3, "DONE") |

### Outcome 3 — collaboration over the hub

| id | source § | claim | status on current tree |
|---|---|---|---|
| O3-1 | C1b§8 row8/§10.4 | `semio-s-plugin-stdio`'s fresh descriptor exceeds the 4 MiB descriptor-contract bound, so the trusted stdio+GIS catalog never publishes, so `artifactAuthority` never reports ready, so `DevScript`'s `waitForReadiness` never admits the hub | **OPEN** — verified directly: `FRESH_DESCRIPTOR_MAX_BYTES`/`TRUSTED_DESCRIPTOR_MAX_BYTES`/`DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES` are still all `4 * 1024 * 1024` unchanged in the tree; DS1 (fleet-4, the designated owner) has written only its inherited-state and where-the-bound-lives sections — every measurement/decision/fix/proof section is still literally `(filling)` |
| O3-2 | C1b§12.1 | the two shells have no identity/authentication path at all in the collaboration harness (no relay started, no `#semio-broker=` proof, `SessionAuthorityNotice` blocks every hub-authenticated step) — and AU3's live sign-in exists but is not what the harness or the `s` host uses | **OPEN** — verified: `c1-collaboration-e2e.md` ends at line 615 with no §14 (C1c's own continuation) yet written; C1c is fleet-4's designated owner and has not yet landed a fix |
| O3-3 | G1 item4, C1§9 | the 10-step `collabRunScenario` has never been run end to end with a pass/fail number attached — the last number on record anywhere is 08-17's pre-fix "2 of 8" | **OPEN** — blocked transitively on O3-1 and O3-2; C1b's own per-step table (§9) marks all ten steps "no" (unreachable), not "some passing" |
| O3-4 | C1§3, G1 | presence session-colour wire extension unverified end to end; `cargo test -p semio-framework-ui --lib --features wgpu presence` could not even compile due to an in-flight peer rename | **CANNOT TELL STATICALLY** — the peer rename this blocked on was reported mid-flight in fleet-3; not re-checked this pass |
| O3-5 | C1b§10.1/§10.2/§10.3 | three harness-level defects blocked the scenario from even booting: a build-freshness-gate deadlock, a call to a deleted `dev` CLI verb, and `OS_HUB_DATA` resolving through a symlinked macOS temp root | **FIXED** — all three closed by C1b with new regression tests, confirmed real by reading the diffs (§10.1–10.3) |
| O3-6 | C1b§12.4 | concurrent-edit convergence, per-user undo, and a deliberate short-connection-loss (as opposed to a full hub restart) are not steps of the scenario at all | **OPEN, deliberately deferred** — adding assertions to a scenario that cannot reach step 1 would prove nothing; correctly not attempted until O3-1/O3-2 clear |
| O3-7 | G1§1 Outcome 3 | the wgpu native shell's collaboration path has never been observed running, compiled/unit-tested only | **OPEN (report)** — no slice in this ticket owns it |
| O3-8 | H2§B.7 steps 6-8 | `DocumentAuthority` over `db::ArtifactHandle`, document WS migrated to the gateway, socket grants gated in the gateway, one presence implementation | **NOT STARTED** — W3c is queued, explicitly gated on C1c's baseline (which does not exist yet) |
| O3-9 | AU3§7 gap1 | nothing has ever been observed inside the real `s` (host-mode) hub — every hub-authenticated proof (AU3's two-browser sign-in) ran inside the single-plugin `animate` playground, which shares `ShellHost`/`HubWorkspace` code but cannot exercise the `/hub` host-mode route | **OPEN, confirmed independently** — G9 §3 re-derived and confirmed the same fact by reading `applyShellUri`'s `hostMode` guard directly |

### Outcome 4 — AI integration over the semio MCP

| id | source § | claim | status on current tree |
|---|---|---|---|
| O4-1 | G7§2 headline | `.mcp.json`'s `semio` entry launches `run_stdio`, which hardcodes `bridge: None` and never binds `--folder`/`--hub` — every "live shell" feature (AgentPresence, tool-call frames, ui_focus/reveal, approvals) is unreachable | **FIXED, mostly** — M4 built a rendezvous bridge reachable from stdio mode and repointed `.mcp.json` to `--folder .`; M7 closed the remaining half by making the **React shell actually dial** `/__semio/agent-bridge` (`useDiscoveredAgentBridgeConfig`, tested with 200+ polls opening exactly one socket) and adding a wall-clock elicitation timeout. **wgpu side still has no caller** of `semio_wgpu_set_agent_bridge_config` (M4 gap2, not M7's scope) |
| O4-2 | G7§2 | destructive-capability approval was permanently unresolvable: no elicitation ever sent, no `ApprovalRequested` bridge frame ever published, no tool to resolve an approval handle, no `--auto-approve` CLI flag | **FIXED (report), live round-trip still unproven** — M4 built the `ApprovalCoordinator` chain, an `action_approve`-shaped resolve path, and removed the mock from production; M7 added the wall-clock timeout the blocking `read_line` needed. M4's own honest gap 6b: the runtime approval round trip rests on unit tests alone, blocked at the time on A2's `InstanceOpen` trap — that trap is now fixed (see O4-4), so this is very likely closer to provable live than the reports show, but no report has re-run it since A2 landed |
| O4-3 | audit-ai-mcp, M1§6.1, S1, A1§3 | `capabilities_search` returns 0 hits for every plugin (catalog compile aborts on architect's duplicate id) | **FIXED** — same evidence as O1-2: the coordinator's live ~23:40 smoke test got real, differentiated hits. This is the single most consequential status change in this re-audit relative to every prior report, all of which (including G9, written ~10 minutes before the smoke test) still list this as open |
| O4-4 | A1§5.1, A2 (whole report) | every plugin guest traps on `Event::InstanceOpen` inside the MCP gateway's plugin host (`memory write is out of bounds`) | **FIXED** — verified directly: `.cargo/config.toml:46` carries `-C link-arg=-zstack-size=8388608` under `[target.wasm32-wasip2] rustflags`, exactly A2's fix; A2's own 4/4 host laws and the e2e gate (`draw` now opens for real, past the old trap) are the proof. Root cause was a wasm-ld default 1 MiB stack vs. the repo's declared 8 MiB, compounded by a 654 KiB-costing static arena (also fixed, `UI_PATCH_APPLY_ARENA` made `const`-initialized) |
| O4-5 | A2§6.1, R2 (assigned) | the guest's retained-command ingress owner is stale on the second and later calls (`owner is 1, the retained owner's is 554`), refusing every mutation verb (`action_prepare`/`invoke`→snapshot→undo→redo→rollback→export all blocked) | **OPEN** — this is now the single blocking defect for the whole MCP mutation chain (A2 traced it to `⚛️reactor/🔄️turn/🦀️.rs:943-958`'s retained-owner take-before-current-page-check); R2 is fleet-4's designated owner and its report is entirely `(filling)` — no measurement, fix, or proof section written yet |
| O4-6 | G7§6 P1.4/P1.5, A1§6 | `artifact_export` never executed (always errored); `artifact_create`'s `kind` accepted but not routed to a plugin-specific type | **FIXED, verified to the channel boundary only** — A1 built real `AppCommand::ExportMedia`/plugin-kind routing (`installed_artifact_kinds()`, `PluginArtifactBinding`); A1's own honest gap: both are proven only up to O4-5's retained-owner wall, not against returned guest bytes |
| O4-7 | G7§5 P1.6 | `resources/subscribe`/`unsubscribe` are unconditional no-ops while the server advertises `subscribe: true` | **OPEN (report)** — M5 queued, not started (M5's stub report exists, `📓️m5a-*` only covers the catalog-usability sub-item, not this one) |
| O4-8 | G7§5 P1.7 | no `notifications/progress` support at all; `job_get`/`job_cancel` polling is the only progress/cancel surface | **OPEN (report)** — stated design tradeoff by M2/G7; M5 queued to at least document it, not started |
| O4-9 | G7§5 P1.8, M5a intro | no pagination on `tools/list`/`resources/list` | **OPEN, fleet-4 (M5a) started, unfilled** — M5a's brief explicitly folds this in but every section of its report reads `(filling)` |
| O4-10 | G7§3 P1.9 | `inference_submit/events/cancel/approve` hardcoded to `GIS_MAP_INFERENCE_SERVICE_ID`, unreachable for any other artifact kind | **OPEN (report)** — M6 queued, not started |
| O4-11 | G7§3 P1.10, M4 gap5 | no production caller anywhere spawns `semio-os-mcp --hub` with a delegated credential — "AI agent as a collaborator in a hub space" has no entry point | **OPEN (report)** — M6 queued, not started; independent confirmation from AU3's own read of the same gap |
| O4-12 | G7§5 P2.11 | duplicate `semio://workspace/artifacts` entry in `resources/list` | **OPEN (report)** — cosmetic, unowned |
| O4-13 | G7§6 P2.12 | README's Safety section describes elicitation/shell-approval/`--auto-approve` as already working | **LIKELY FIXED (report)** — M4's files-changed list includes `README.md`; not independently re-read this pass to confirm the specific wording changed |
| O4-14 | G7§3 P2.13 | hub directory has no "AI agent principal" concept distinct from the delegating human — an agent's edit is indistinguishable from the human whose credential it borrowed at the presence layer | **OPEN (report)** — M6 queued |
| O4-15 | M2§7, M3§6 | `os-mcp` crate had 21 pre-existing test failures | **MOSTLY FIXED (report)** — M3 measured 336 passed/1 failed by its own session's end; the 1 remaining failure is the same family as O4-5 (now R2's) |
| O4-16 | status.md ~23:40, M5a (new) | live coordinator smoke test found two fresh agent-usability defects: every capability has `description: ""` (BM25 can't discriminate "rectangle" among 16 tied draw verbs), and raw pointer/engagement input events (`canvasPointerMove`, `canvasEscape`, …) are published as agent capabilities indistinguishable from document verbs | **OPEN, fleet-4 (M5a) just started** — every section of M5a's report is `(filling)`; this is brand-new, found in the last hour of this audit's own window |
| O4-17 | M2§7 | wgpu chat panel still echoed locally rather than reading real bridge frames | **PARTIAL, likely improved (report)** — M3 landed the wgpu panel body reading the live bridge (75+3+21 tests green); G8 WG-2/WG-7 close the remaining cancel/approval controls on the same surface. Not independently re-verified whether the echo path itself is fully retired |
| O4-18 | M1§6.2 | the closed `26/09/06/REPO-RUST-IMPLEMENTATION-AND-TAXONOMY-TREE` ticket's summary actively contradicts the live tree (claims `💻️client` removed, a `SEMIO_REPO_IMPLEMENTATION` switch, 9-tool parity that didn't exist yet) | **OPEN (report)** — M1 added a dated annotation note but did not reopen or correct the ticket itself |

### Infra / cross-cutting

| id | source § | claim | status on current tree |
|---|---|---|---|
| INF-1 | G1§3 | no browser-facing UI for a human to sign in to a *remote* hub | **FIXED for React (report), OPEN for wgpu, OPEN inside `s`** — AU2 built it, AU3 live-proved it with two real browsers; G8/G9 confirm wgpu has no equivalent surface at all, and G9 confirms it has never been exercised inside the actual `s` host (only in the `animate` playground) |
| INF-2 | G1 preamble, this audit's own check | ~439 modified/untracked paths in `git status --short` right now (was 632 at G1's time) — every "fixed" claim in this ledger is "fixed in the working tree," not "fixed and merged" | **STANDING RISK** — verified: `git status --short \| wc -l` → 439 at this audit's time |
| INF-3 | status.md (all sessions) | repo MCP `ticket_reopen` answers `invalid tool params` for every path spelling; bookkeeping for this ticket has been on-disk-only since coordinator session 3 | **OPEN** — a repo-MCP defect, outside this ticket's four outcomes but actively hampering this ticket's own process |
| INF-4 | V1/V2/V3a/V3b | `plugin-registry check`/taxonomy verification backlogs (1836→684 findings, long tail in `verify layering`/`package-purity`/`dependencies*`) | **PARTIAL, in progress** — see O1-25 |
| INF-5 | multiple (H1, C1b, A2, B1a) | build-budget/disk fragility: ENOSPC hit once (~03:57), the shared Cargo build-dir lock repeatedly serialized independent slices, a 20-minute cargo budget note governs expectations | **OPERATIONAL, not a code gap** — actively managed by the fleet (16.7 GiB freed by B1a, 7.4 GiB by A2, coordinator prunes) |
| INF-6 | K2§1/§3/§5/§6 | test-infrastructure defects: `vitest-configuration-ownership`'s "machine-specific cacheDir" diagnosis (refuted, real drift fixed instead), orphaned test-suite census, `documentId`→`artifactId` descriptor rename, three `🔋️energy` `sim::` unit-test failures | **PARTIAL** — items 1,3,4 done (K2 §1/§3/§4, all "DONE"); items 2,5,6 are `_in progress_`/WIP in K2's own table, fleet-4 continuing |
| INF-7 | T4 (whole arc) | os/renderer TypeScript diagnostics (was 596 at start of this ticket) | **IN PROGRESS** — 596→449→318→321 (T4c's own baseline shows it ticked up by 3 net before its own fixes began, from an unrelated rename); T4c has since closed several files to 0 and finished two cross-language half-renames (tutorial-document-track, cold-document-pair) fully, not just in TypeScript |
| INF-8 | G3 (whole report) | 6 mechanical rename-drift fixes needed after the peer `🎚️options`→`☑️options`/`⚙️config`→`🎚️config` rename | items 1 (gismap imports) and 2 (empty-facet-authority golden+enum) **FIXED** (B3a, B3a2, verified via those reports' own test captures); item 4 (6 `owner` literals) **FIXED** (H1's files-changed list); items 3 (sourcing straggler directory rename), 5 (doc-comment cleanup), 6 (taxonomy.json enum narrowing) **OPEN (report)**, unclaimed |
| INF-9 | S1, G3§4 | proven-eight plugins (raster/forms/note/fem/energy/layout/remodel/draw) + `space` re-verification after the rename | **NOT STARTED** — still queued in every coordinator status update since fleet-3 |

---

## B. Fleet-4 / queued / uncovered coverage

**Running fleet-4 slices and what they cover** (per `📓️status.md`'s own table plus this audit's reading of
each report's current state):

| slice | covers (this ledger's ids) | state observed this pass |
|---|---|---|
| DS1 | O3-1 | inherited-state only; every substantive section `(filling)` |
| C1c | O3-2, O3-3 (transitively), O3-9 | not started — `c1-collaboration-e2e.md` has no §14 yet |
| R2 | O4-5, O4-2 (unblocks), O4-6 (unblocks) | not started — every section `(filling)` |
| M7 | O4-1 (closed the React-dial half) | **items 1–2 done**; items 3 (live (a)–(e) transcript), 4 (permanent nx e2e target), 5 (wgpu parity) still `(filling)` |
| H1b | O2-3, O2-9, O2-17 | not started — §9–§13 all `(filling)` |
| F1 | O1-5, and transitively O1-6d/e/f | diagnostics + root fix done; live proof (§6) and B1a leftovers (§7) both "NOT DONE"/"NOT STARTED" |
| B2c | O1-6c (norm sweep), O1-6d (imperative, effectively done pending F1) | norm root-caused/fixed for din4108, 14-app sweep "⏳"; imperative migrated and live; dag/playbook not re-touched this continuation |
| B3b | trinity/wfc/puzzle residuals (not separately IDed above — see B3b's own §5) | trinity's fix is written but unactivated (blocked on a peer's live `semio-framework-plugin` edit at slice time); status unknown this pass |
| B3a2 | O1-7 (gis native mutations, done), O1-8 (block boot, not reached) | gis native half done; block boot/probe still open |
| T4c | INF-7 | actively reducing; not yet at 0 |
| K2 | INF-6 items 2,5,6 | in progress |
| V3a | O1-25 (its slice of it) | done, own report filled |
| V3b | O1-25 (its slice of it) | stub — §6/§7 both `_pending_` |

**Newly launched, not in the original fleet-4 list** (found via the freshest `📓️status.md` entries and
file listing, timestamped after the fleet-4 table was written):
- **G9** (done) — audited outcome 1's `s`/space gap in depth; source of O1-1, O1-9, O1-10, O1-23 confirmations.
- **M5a** (just started) — covers O4-9 (pagination, folded in) and the two brand-new O4-16 catalog-usability defects the coordinator's own live smoke test just found. Every section `(filling)`.

**Queued (per the latest `📓️status.md` line and G9's own priority list), and what each would close**:

| slice | covers |
|---|---|
| A3 | O1-3 (remaining ~28 descriptor skips) |
| W3c | O3-8 (DocumentAuthority/document-WS/socket-grants/presence migration into the gateway) |
| B3c, B3d | remaining middle-tier plugin gaps not itemized individually in this ledger (procedural/flow/process/cad/lowpoly/draw/shooting/sourcing/demonstrator residuals) |
| M5 | O4-7, O4-8 (partially, documentation), O4-9 (M5a already absorbed the pagination half) |
| M6 | O4-10, O4-11, O4-14, O2-13 (hub-side generalization) |
| O3 | wgpu wasm32 artifact-open relay — **O1-4** |
| OB1 | O2-9 (hub observability) |
| WG1 (→ now folded into G9's "**WG6**") | O1-23 (wgpu hub sign-in/spaces/workspace) |
| U3 | O1-13 (`MutationKind::label()` localization — 2690 call sites) |
| R1 | wgpu renderer crate's 30 failing tests + 1 SIGABRT (M3§4.4 — not itemized above; genuinely nobody's, module owners unnamed) |
| **S2 (G9's new proposal)** | O1-1, O1-11 (patient cold `dev s` boot + timing + activation receipt) |
| proven-eight + space re-verify | INF-9 |

**Uncovered by any running, launched, or queued slice** (genuinely nobody's, as of this read):

- O1-4 (wasm32 wgpu artifact-open relay) is *technically* "O3" in the queue list, but O3 has never been
  launched in four coordinator sessions — effectively uncovered.
- O1-9 / O1-10 (React DOM host hardcoded `"manual"`; guest ignores `Event::Activate`) — a one-line fix
  and a guest-SDK decision respectively, named by two separate reports (O2, G9), claimed by nobody.
- O1-19b / O1-22b (wiring the already-ported pinch-gesture and diagram-arrow-nav math into a live scene) —
  explicitly deferred by their own authors (U2, G8) as "needs the scene-handler location identified
  first," never picked up.
- O1-21 (theme contrast live warning) — G8 itself declines to schedule it ("no wgpu theme editor exists
  to attach it to... not actionable as a slice today"); the React-side editor gap is smaller and equally
  unclaimed.
- O2-11 (`checkpoint-publications` dead-or-not decision), O2-15/O2-16 (browser-broker relay, GIS-map
  commit-owner law) — each named twice by independent reports, none assigned.
- O2-18's residual (is `4_gate` now fully populated, i.e. does the launch-row family cover every schema
  that requires one, not just the two named families) — nobody re-audits this after K2.
- INF-8 items 3/5/6 (sourcing directory rename, doc-comment cleanup, taxonomy.json enum narrowing) —
  G3 named all three explicitly as "left for whoever" and nobody has picked them up across three fleets.
- O4-13 verification (did M4's README edit actually fix the overclaiming prose, or just touch the file
  for another reason) — nobody has re-read it to confirm.
- The 26/09/06 ticket's false "closed" status (O4-18) — annotated, never corrected, by design left to
  "a ticket of its own" that nobody has opened.

---

## C. Proposed new execution slices for uncovered gaps

Ordered by how directly each blocks one of the four stated outcomes.

### Slice N1 — "Fix the two one-line MCP/activation loose ends" (blocks Outcome 1 directly, cheap)
- **Scope**: O1-9 (`🔌️PluginRuntime/🟦️.tsx:3047`, `"manual"` → `activationReasonForAppId(appId)`) and,
  if a guest-SDK owner can be found in the same pass, O1-10 (make one guest, e.g. `raster` or `forms`,
  actually branch on `Event::Activate`'s carried kind as a worked example for the others).
- **Main files**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`;
  one plugin's `⚛️reactor` entry point for the second half.
- **Acceptance, observed at runtime**: a unit test on `createApp` asserting the guest-bound turn carries
  `on-artifact-kind:<kind>` instead of `manual` when opening a foreign-kind artifact through the DOM
  host (G9's own proposed acceptance, item 4); for the guest half, a console/log line showing the sample
  plugin took a kind-specific branch on open.

### Slice N2 — "Un-gate the wasm32 wgpu artifact-open relay" (Outcome 1, matches queued "O3", relaunch it)
- **Scope**: O1-4. Un-gate `handle_open_artifact_relay`/`switch_to_app`/`open_document` for
  `target_arch = "wasm32"` in the wgpu shell, pulling in the persistence bindings, hub transport and
  `system_fs` shims the browser target needs.
- **Main files**: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`.
- **Acceptance, observed at runtime**: a browser-wgpu boot of any single-plugin playground can open a
  second, foreign-kind artifact without a full page reload (screenshot or DOM-mirror a11y tree showing
  the second window).

### Slice N3 — "One patient, uncontended cold `dev s` boot" (Outcome 1, G9's own S2 proposal — highest leverage single action left for Outcome 1)
- **Scope**: O1-1, O1-11. No code change — reserve a quiet build window (load average low, no
  concurrent cargo), run `bun ./📜️script.ts dev s` or `activate s react dev` to completion, record
  wall-clock/disk cost, capture the first-ever `dist/runtime/react/dev/s/activation` receipt. Then run
  the foreign-kind-open probe G9 wrote and deliberately did not run (`🐍️g9-s-host-open-foreign-kind-probe.mjs`,
  not yet created).
- **Acceptance, observed at runtime**: activation receipt exists; served page reports
  `PLAYGROUND_SESSION.plugins.length === 60`; a browser opens a non-`space` artifact (e.g. `raster`)
  inside the running `s` session with no reboot, and separately opens a `BatchOnlyPendingRewrite`-refused
  plugin (e.g. `dag`) to observe the refusal surface live inside `s` for the first time.

### Slice N4 — "Reactor retained-command owner lifecycle" (blocks Outcome 4 entirely — resume/finish R2)
- **Scope**: O4-5, unblocking O4-2's live proof and O4-6's full verification. R2 is already assigned and
  has full context (A2's diagnosis is exact, file:line and all) — this is a "resume via SendMessage," not
  a new slice, but it is the single highest-leverage open item for Outcome 4 today.
- **Main files**: `🧰️framework/🛍️products/💻️os/🔨️modules/⚛️reactor/🔄️turn/🦀️.rs:943-958` (the retained-owner
  take-before-current-page-check).
- **Acceptance, observed at runtime**: `bun nx run @semio-tech/framework-os-mcp:client-e2e` shows
  `action_prepare`→`action_invoke`→`artifact_snapshot`→`history_undo`/`redo`→`transaction_rollback`→`artifact_export`
  all `PASS` against a real plugin (`draw` or `note`), not just past the open.

### Slice N5 — "Two-user identity in the collaboration harness and the `s` host" (blocks Outcome 3 entirely — resume/finish C1c, after DS1)
- **Scope**: O3-1 (DS1's own item, a precondition) then O3-2/O3-3/O3-9. C1c's own report already
  specifies the concrete shape needed (a hub entry point booting two `LocalProfile`s + one relay per UI
  origin + the harness navigating each context to its own `#semio-broker=` proof + a reload-reauth
  decision) — this is a resume of an already-scoped slice, not a new one.
- **Main files**: `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (a new dual-profile entry point);
  `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` (navigate to the per-user
  proof URL).
- **Acceptance, observed at runtime**: `collabRunScenario` reports a real pass/fail count per step —
  even a partial number (e.g. "6 of 10") is the single most valuable missing data point in this entire
  ticket.

### Slice N6 — "MCP catalog agent-usability" (finish M5a — blocks Outcome 4's practical usefulness, not its existence)
- **Scope**: O4-16 (empty descriptions defeat BM25; raw input events indistinguishable from document
  verbs) plus O4-9 (pagination). Already scoped and staffed (M5a); flagged here because it was found by
  this session's own live smoke test and is the freshest gap in the whole ledger.
- **Main files**: the descriptor source (schema-first per M5a's own framing) — verb `description`
  fields en+de, an `audience`/intent classification the catalog compiler filters raw input events out
  by.
- **Acceptance, observed at runtime**: `capabilities_search "draw rectangle"` returns a differentiated,
  non-tied top hit; `capabilities_search` results exclude `canvasPointerMove`/`canvasEscape`-class raw
  input actions from the default (non-input-event) result set.

### Slice N7 — "block plugin boot + gis browser verification" (Outcome 1, unblocks N3's own acceptance criteria)
- **Scope**: O1-8 (block2d/3d/5d boot+probe, never attempted) and O1-7's browser half (drive the four
  new gis mutation verbs through a real Actions-rail click, not just the native test suite).
- **Main files**: `✏️s/🔌️plugins/🧱️block/**` (activation + the existing but unverified
  `🐍️b3a-block{2,3,5}d-probe.mjs`); `✏️s/🔌️plugins/🌍️gis/**` (no new source, browser proof only).
- **Acceptance, observed at runtime**: block boots in a browser, renders, and its per-crate tests are
  re-run against a fresh wasm build; `addFeature`/`moveFeature`/`renameFeature`/`deleteFeature` staged
  and dispatched from the live Actions rail move the gismap document and undo/redo round-trips visibly.

### Slice N8 — "Hub observability minimal bar" (Outcome 2, currently unowned beyond H1b's stub)
- **Scope**: O2-9. Add `tracing`/`tracing-subscriber`, instrument at minimum the two WS handlers
  (`document_ws_v1`, `directory_ws_v1`) and `post_directory_commands`.
- **Main files**: `🌎️hub/📦️packages/🦀️rust/Cargo.toml`, `🌎️hub/🏗️bootstrap/🦀️.rs`.
- **Acceptance, observed at runtime**: a booted `os-hub` emits structured trace lines for a document
  WebSocket open/close and a directory command, visible with `RUST_LOG=info` or equivalent.

### Slice N9 — "`checkpoint-publications` dead-code decision" (Outcome 2, cheap, unowned)
- **Scope**: O2-11. Determine whether the route is dead (delete it) or an intended-but-unbuilt client
  feature (wire one caller) before Wave-3's module split inherits the ambiguity.
- **Main files**: `🌎️hub/🏗️bootstrap/🦀️.rs` (the route), whichever os-product store module would call it
  if kept.
- **Acceptance, observed at runtime**: either the route is gone and the hub's route table shrinks by
  one, or one client call site exists and a checkpoint-publication round-trips over HTTP once, observed
  in a network capture.

---

## D. Definition-of-done checklist per outcome

### Outcome 1 — working os `s` frontend, all plugins/artifacts

- [ ] A cold, uncontended `dev s` (or `activate s react dev`) completes and leaves
      `dist/runtime/react/dev/s/activation/🔣️receipt.json` on disk (does not exist today — N3).
- [ ] The served `s` page reports `PLAYGROUND_SESSION.plugins.length === 60` in the browser console.
- [ ] `capabilities_search` returns non-zero, differentiated hits for a representative query
      (already true per the 23:40 live smoke test — recheck after any future descriptor regeneration).
- [ ] At least one proven-interactive plugin (`raster`) and one currently-refused plugin (`dag`/`norm`)
      are opened **inside the running `s` session**, not a single-plugin playground, with a console/network
      capture showing `installPlugin`/`openArtifactWithAppRef` firing (N3's own acceptance).
- [ ] All 12 previously-dormant plugins pass the full interaction bar (boot → mutate → undo → redo,
      0 fault lines) — currently 2 of 12 fully done (`dag`, `reasoning`), 2 partial-and-blocked-on-F1
      (`norm`'s din4108 code, `imperative`), 8 open.
- [ ] `block` boots and renders in a browser at least once (currently: compiles, assembles, has a
      descriptor, never booted — N7).
- [ ] `gis`'s four new mutation verbs are dispatched from a live Actions rail, not just native tests (N7).
- [ ] wgpu renderer has a working hub sign-in/spaces/workspace surface reachable the same way React's is
      (currently zero wgpu hub UI — O1-23, "WG6").
- [ ] The wasm32 browser wgpu shell can lazy-install and open a foreign-kind artifact (N2).
- [ ] `MutationKind::label()`'s 2690 call sites are localized (or explicitly descoped as an accepted,
      documented English-only surface) — currently untouched.

### Outcome 2 — working server hub backend (db, presence, auth)

- [ ] `cargo check -p semio-hub` (default features) is green — was blocked twice this ticket (pdf, then
      gis rename drift); both fixes are in the tree but **not independently re-confirmed since** (H1b).
- [ ] `cargo test -p semio-hub --lib` and `--bin os-hub` are green, with a real number recorded in a
      report (currently unmeasured since H1's fixes — H1b's own stub).
- [ ] `os-hub:test` finishes inside its declared level budget (currently 263 unscoped laws vs. a 15s
      budget — H1b).
- [ ] `bun nx run os-hub:dev` boots to `/readyz` with all subsystem booleans true on a fresh
      `OS_HUB_DATA` (already demonstrated once, H1 §6.2 — needs to stay true after every subsequent fix).
- [ ] `POST /auth/sessions` mints a real session and `DELETE /auth/sessions/me` revokes it — **done**,
      live-proven by AU3's two-browser transcript.
- [ ] A hub request is rate-limited on the auth/directory/socket-grant paths — **done** (AU1).
- [ ] The hub emits structured trace output for at least its two WebSocket handlers and the directory
      command path (N8, currently zero).
- [ ] Postgres and Neo4j directory/storage backends have actually been **run** at least once (compile
      only today — needs Docker on the test host).
- [ ] `HubInstance: ServerInstance`'s durable stores (W3b) are wired to at least one real hub route or
      `Server::run` path, not merely compiled and unit-tested in isolation.

### Outcome 3 — working collaboration between users over the hub

- [ ] `semio-s-plugin-stdio`'s descriptor fits (or the 4 MiB contract bound is deliberately raised) so
      the trusted catalog publishes and `artifactAuthority` reports ready under `DevScript` — **not yet
      true anywhere in the tree** (DS1, all sections still `(filling)`).
- [ ] Two distinct, hub-authenticated user identities exist inside the collaboration harness (and,
      separately, inside a running `s` host) — **not yet true** (C1c not started).
- [ ] `collabRunScenario`'s 10 steps run end to end against a real hub and a real number is recorded in
      a report — **the single most important missing artifact in this entire ticket; has never existed**.
- [ ] A deliberate short connection loss (not a full hub restart) is a scenario step and passes.
- [ ] Per-user undo is a scenario step and passes.
- [ ] Two simultaneous writers converging on one document is a scenario step and passes (today only the
      replication crate's unit tests, not a browser scenario, cover ordering).
- [ ] Presence shows two distinct session colours in a real, rendered browser (wire-level only today).
- [ ] The wgpu native shell's collaboration path is observed running at least once (never has been).

### Outcome 4 — working AI integration over the semio MCP (`semio-framework-os-mcp`)

- [ ] `.mcp.json`'s `semio` server answers `initialize`/`tools/list`/`resources/list` — **done**,
      live-verified by this very session.
- [ ] `capabilities_search` returns real, differentiated hits — **done**, live-verified at ~23:40 tonight.
- [ ] Verb descriptions are non-empty and localized, and raw input events are excluded from the default
      agent-facing capability search — **not yet true** (M5a, just started).
- [ ] The full mutation chain (`action_prepare`→`action_invoke`→`artifact_snapshot`→`history_undo`/`redo`→
      `transaction_rollback`→`artifact_export`) is green end to end against a real plugin through
      `.mcp.json`'s own configuration — **blocked today on the retained-command-owner defect** (R2, N4).
- [ ] A destructive-capability approval can actually be resolved (accept/decline) through a live client
      round trip, not just through unit tests — mechanism built (M4/M7), never proven live.
- [ ] The React shell renders a live agent tool-call transcript sourced from a real MCP session —
      mechanism built and unit-tested (M7 items 1–2); the live (a)–(e) transcript against a running shell
      is still `(filling)` (M7 item 3).
- [ ] An MCP agent can edit inside a real hub space, replicate over the wire, and show up (by actor
      string at minimum) to a human collaborator — the credential-delegation code exists; **nothing in the
      product calls it** (M6, not started).
- [ ] `resources/subscribe` either does something or the server stops advertising it (M5, not started).
