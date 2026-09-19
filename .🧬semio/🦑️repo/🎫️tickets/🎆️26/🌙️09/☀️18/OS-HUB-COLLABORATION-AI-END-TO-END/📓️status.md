# Status — OS Hub Collaboration AI End To End

Opened 2026-09-18. Coordinator: Fable 5.1 main session. Workers: Opus 5 (execution slices), Sonnet 5 (audits). Repo MCP down → bookkeeping on disk.

## Phase 0 — audits (done)
- 📓️audit-os-frontend.md — `dev s` boots only the `space` hub plugin; multi-plugin runtime activation unconfirmed (P0).
- 📓️audit-plugins-artifacts.md — 34 real plugins; 12 never booted; 🗺️catalog.json directoryName drift for 26 nested entries; 🗟️artifacts empty stub.
- 📓️audit-hub-backend.md — semio-hub E0560 ×2 compile break; os-hub:dev import off-by-one; semio-framework-server E0277 ×28 dead crate.
- 📓️audit-collaboration.md — transport/wire/reconnect/presence real (274 tests); browser E2E blocked by PluginRuntime retry storm (2/8 steps).
- 📓️audit-ai-mcp.md — repo MCP binary path mismatch + duplicate Go module; os MCP env-guard false positive on CLAUDE_CODE_*; chat panel is an echo mock; inference not-wired except GIS.
- 📓️audit-build-infra.md — 264 crates; ≤3 concurrent cargo; disk tight (pruned 42 GiB of stale incremental sessions 2026-09-18).
- 📓️audit-multi-plugin-hub.md — React hub already lazily installs other plugins' wasm on open (host-mode fan-out to ~60 materialize targets); wgpu lacks the fallback; `dev multi` dead; activation events declared but unused; host-mode tests not in vitest include.

## Wave 1 — P0 unblockers (in flight)
| slice | worker | scope | report |
|---|---|---|---|
| H1 | Opus | hub compile fix, os-hub:dev boot to /readyz, hub tests, test targets w/o --all-features | 📓️h1-hub-build-and-boot.md |
| H2 | Opus | DONE: FrontierSummary serde derive; server crate 73/73, replication 274/274 native+wasm; memo recommends executing Wave 3 (hub as server instance, ~13–16k lines, 14 steps) after H1 baseline | 📓️h2-server-crate-and-wave3-memo.md |
| P1 | Opus | DONE: catalog directoryName is a flat install basename (audit Q2.2 retracted, 60/60 verified by 🐍️p1-catalog-verify.ts); 6 stale registry test counts fixed (21/21); three 🗟️ garbage dirs removed; reasoning crate renamed; energy probe deleted. Handed back: verify taxonomy unrunnable, plugin-registry check 2299 violations, stale fixture-sweep fixture | 📓️p1-catalog-and-registry-hygiene.md |
| O2 | Opus | activation reason to guest via WIT; wasm32 wgpu lazy install; plugin_install chrome; hub foreign-kind probe | 📓️o2-activation-follow-ups.md |
| V1 | Opus | verify taxonomy digest fix; plugin-registry check violations to zero; fixture-sweep fixture; cheap gates green | 📓️v1-verification-gates.md |
| P2 | Opus | DONE: mathematical-equation import fixed; playbook-procedural ArtifactCompositionFields impl added; 41/46 dormant crates green, 5 blocked only by peer's live stdio-pdf edit; space/dag/imperative wasm32-wasip2 green | 📓️p2-dormant-plugin-compile.md |
| B1a | Opus | boot recipes + headless probes: writer, mathematical, vcs, animate, sequence, architect | 📓️b1a-dormant-plugin-boots.md |
| B1b | Opus | DONE: reasoning/norm/playbook/imperative/dag boot React (space deferred to C1's cold `s` build); 6 root defects fixed (playbook unclassified verbs + bundle dep, reasoning retained-tool migration + command_from_action + example ids + store init); none yet passes the interaction bar; 8 launch.json rows | 📓️b1b-dormant-plugin-boots.md |
| B2b | Opus | finish interaction chain for reasoning/norm/playbook/imperative/dag + per-crate tests | 📓️b2b-dormant-plugin-interactions.md |
| M1 | Opus | DONE: os MCP env seal made exact (S_/VITE_S_ carriers only); stale duplicate Go module deleted, canonical 💻️client/🔌️mcp in go.work with build/test; `mcp` verb moved ahead of catalog resolve (25 s → 2 s spawn); initialize strict-decode regression fixed; goal_open/close/reopen added (9 tools, 8 resources); go tests green; handshake probe passes both servers. Open: capability catalog fails to compile (duplicate architect id + ~20 descriptor-drift skips) | 📓️m1-mcp-servers-start.md |
| M3 | Opus | os-mcp crate 21 pre-existing test failures to green; wgpu agent panel on the live bridge | 📓️m3-mcp-tests-and-wgpu-agent-panel.md |
| T1 | Opus | regenerate emoji-corrupted generated .d.ts; root tsconfig source-only scope; bun types | 📓️t1-codegen-corruption.md |
| T2 | Opus | hub + repo library TS to zero errors, hub tsconfig scoped | 📓️t2-typescript-library-hub.md |
| T3 | Opus | framework modules/packages TS to zero; framework typecheck target | 📓️t3-typescript-framework.md |
| T4 | Opus | os product scoped tsconfig + typecheck target; os/s/build-script TS to zero | 📓️t4-typescript-os-renderer.md |
| A1 | Opus | capability catalog zero skips; MCP e2e client script (open → invoke → snapshot → undo → inference_run) as permanent nx test | 📓️a1-mcp-end-to-end.md |
| M2 | Opus | DONE: AppCommand::Infer + inference_run MCP tool (progress/cancel via job registry), AgentToolCall/AgentToolResult/AgentMessage bridge frames, live AgentChatPanel, BasicChatPanel echo mock deleted; 10 new tests green, 23 bridge fixtures round-trip both codecs. Open: 21 pre-existing os-mcp crate test failures (classified §4.7), wgpu chat echo parity debt, renderer TS 865 pre-existing errors | 📓️m2-agent-surface-and-inference.md |
| C1 | Opus | PluginRuntime retry storm root fix, presence color wire, collabRunScenario 8/8 + live-edit + mid-edit restart assertions | 📓️c1-collaboration-e2e.md |
| O1 | Opus | DONE: `dev multi` deleted; artifact-kind activation owner generated (both kind spellings) and used by React ShellHost + wgpu switch_to_app/open-relay with lazy install + cancel; framework vitest gate was running 0 tests (fixed, 141/141); 16 new tests green. Open: guest never receives activation reason, wasm32 wgpu lazy install, plugin_install chrome | 📓️o1-multi-plugin-hub.md |

## Audits wave 2
- 📓️audit-typescript-debt.md — 2608 deduped TS diagnostics; 68% are emoji-spliced identifiers in generated .d.ts (codemod incident, regenerate); ~833 real, clustered.

## Wave 2 — planned (depends on Wave 1 + multi-plugin audit)
- O1 multi-plugin hub: `dev s` hosts every registered plugin's wasm, lazy activation by artifact kind.
- C1 collaboration E2E: PluginRuntime retry storm fix, collabRunScenario 8/8, live edit propagation, mid-edit hub restart.
- S1 hub as generic-server instance (or retire generic server) per H2 memo.
- B1 boot recipes + probes for the 12 dormant plugins.
- A1 MCP end-to-end slice from Claude Code: tools/list → artifact_open → action_invoke → snapshot → undo.

## Relaunch 2026-09-18 ~23:50 (coordinator session 2)
Session 1 ended with H1, V1, B1a, B2b, M3, T1, T2, T3, T4, A1 unreported (workers died with the parent turn) and C1 with three unfilled placeholders. Shared rules now live in `📓️worker-preamble.md`. Relaunched as one fleet, each told to inherit partial tree edits first:
| slice | worker | report |
|---|---|---|
| H1 | Opus | 📓️h1-hub-build-and-boot.md (adds C1's §4 zero-budget hub fix) |
| C1b | Opus | 📓️c1-collaboration-e2e.md §8 + placeholders (10-step scenario, O2 handoff line) |
| V1, B1a, B2b, M3, T1, T2, T3, T4, A1 | Opus | as in Wave 1 table |
| S1 | Sonnet audit | 📓️s1-plugin-coverage-matrix.md (all 34 plugins: recipe/probe/interaction/tests/catalog) |
| W3a | Opus | 📓️w3a-server-instance-seams.md (H2 memo §B.7 step 3 only: `ServerInstance` trait, de-close the ten sets, TestInstance profile) |

## Wave B3 — middle-tier plugins (launched 2026-09-19 ~00:05 from S1 matrix)
| slice | plugins | report |
|---|---|---|
| B3a | 🧱️block (E0053 + missing 🔣️.json descriptor), 🌍️gis (Xcode license → CLT fallback in script) | 📓️b3a-block-gis.md |
| B3b | 🔱️trinity, 🀄️wfc ×5, 🧩️puzzle ×3 | 📓️b3b-trinity-wfc-puzzle.md |
| B3c | 🌀️procedural ×2, 🌊️flow +9 ext, 🏭️process +4 ext | 📓️b3c-procedural-flow-process.md |
| B3d | 📐️cad +4 ext, 💠️lowpoly, 🖍️draw, 🎥️shooting, 🪵️sourcing +3 mod, 🎪️demonstrator | 📓️b3d-cad-lowpoly-draw-shooting-sourcing-demonstrator.md |
Not yet assigned: 🪐️space (C1b's cold `dev s`), the proven eight (raster/forms/note/fem/energy/layout/remodel/draw re-verify), 🗄️stdio-pdf 45-error peer edit (PDF-ARTIFACT-SPEC-COMPLETE ticket).

## Wave 3 + unblockers (launched 2026-09-19 ~01:10)
| slice | scope | report |
|---|---|---|
| P3 | port `semio-s-artifact-stdio-semio` pdf import/export callers to the peer's new `PdfSnapshot` (28 fields) — unblocks hub default features, build-dev, writer/animate/vcs | 📓️p3-stdio-semio-pdf-callers.md |
| W3b | H2 §B.7 steps 4–5: `HubInstance: ServerInstance`, `StorageProfile` real variants, durable hub stores + ported conformance tests | 📓️w3b-hub-instance-and-durable-stores.md |
| D1 | `🔀️dispatch` macro: close `impl Future + Send` ports; restore `#[dyn_enum]` on the eight server ports | 📓️d1-dispatch-send-future-ports.md |
Done: S1 audit, T1 (report written), W3a (step 3 done: trait + de-close + TestInstance). H1 report written but §5–8 still placeholders (worker still running).

## Relaunch 2026-09-19 ~02:10 (coordinator session 3)
Session 2 ended with no worker alive; `🗑️generated` empty. Reports missing: V1, M3, T2, T3, A1, B3a–d, P3, W3b, D1; B2b is a 600-byte stub; B1a has four `*_PLACEHOLDER` tokens; H1 §5 thin; T4 owns 596 open diagnostics. Relaunched as fleet 3 (Opus workers, Sonnet audits), same report names as the tables above, plus:
| slice | scope | report |
|---|---|---|
| G1 (Sonnet) | goal-level gap audit: what still separates the tree from the four outcomes after waves 1–3 | 📓️g1-goal-gap-audit.md |
| G2 (Sonnet) | hub backend depth audit: db/persistence, presence, auth, directory, artifact-authority, inference — real vs stub | 📓️g2-hub-depth-audit.md |
| G3 (Sonnet) | proven-eight + space re-verification matrix (static): recipes, probes, launch rows still valid after peer renames (`🎚️options`→`☑️options`, `⚙️config`→`🎚️config`) | 📓️g3-rename-drift-audit.md |

### Fleet 3 additions (~02:30, from G1/G2 audits — both done)
| slice | scope | report |
|---|---|---|
| AU1 | hub sign-in/session-mint route, revocation/expiry as events, token-bucket rate limiter, integration tests | 📓️au1-hub-auth-sessions-and-rate-limit.md |
| AU2 | os sign-in flow, end-user spaces surface (list/switch/create/members/invite/redeem), en+de, a11y, phone width | 📓️au2-os-sign-in-and-spaces-ui.md |
| K1 | hygiene: missing launch row, stray [DEBUG] ignores, mcp README drift, dead ui.chat keys, dead hub routes, /healthz, postgres/neo4j feature check | 📓️k1-hygiene.md |
Queued (cargo slots): B3a–d middle-tier plugins; O3 wgpu wasm32 artifact-open relay; OB1 hub observability on the framework `⏱️trace` module; proven-eight + 🪐️space re-verify; devcontainer zero-touch; i18n/mobile audit.

### Fleet 3 additions (~02:40, from G3/G7 audits — done; G4 zero-touch, G5 UX audits running)
| slice | scope | report |
|---|---|---|
| M4 | G7 P0 1–3: stdio-mode bridge to the live shell, resolvable approval chain (elicitation / shell approval / `--auto-approve`), no silent `MockArtifactChannel`, `.mcp.json` workspace binding | 📓️m4-mcp-bridge-approval-binding.md |
A1 scope extended by message: G7 P1 4 (`artifact_export` executes) and 5 (`artifact_create` kind routing).
Queued behind M3/A1/M4 (same crate): M5 protocol conformance (G7 P1 6–8, P2 11), M6 collaboration×AI (G7 P1 9–10, P2 13: `--hub` delegation caller, agent principal + presence, inference job quartet generalisation).
G3 result: rename 99 % done; only load-bearing break is gis `🗺️map/🟦️.ts:23-27` imports (goes to B3a) + one leftover `🪵️sourcing/…/🪟️windows/🎚️options` dir (goes to B3d).

### Fleet 3 additions (~03:15)
Done: P3 (pdf callers ported, 9/9 pdf tests; hub now red only on gismap rename drift — 18 errors), G5 UX audit (📓️g5-ux-completeness-audit.md), G7.
| slice | scope | report |
|---|---|---|
| B3a | URGENT gismap `MutationLeaf source authority` rename drift (unblocks hub), then gis + block boot/interaction | 📓️b3a-block-gis.md |
| U1 | G5 items 1,2,4,5,8: TaskManager window, agent cancel, ShellSync i18n, install cancel, persistent hub-connection indicator | 📓️u1-progress-cancel-and-connection-status.md |
| U2 | G5 items 6,7,9,10: multi-touch viewports, tablet breakpoint, theme contrast check, Diagram a11y | 📓️u2-touch-tablet-contrast-diagram-a11y.md |
Queued: U3 `MutationKind::label()` localisation (2690 call sites, trait-signature change — needs a calm cargo window), B3b–d, M5, M6, O3, OB1.

### Fleet 3 progress (~04:00)
Done this session: G1 G2 G3 G4 G5 G7 audits; P3; W3b (HubInstance + durable journals, server 78/78, hub sqlite lane 151/151, conformance ×3 profiles); T3 (420→65); T4 (599→449, os vitest 292→348 pass); T2 (repo 507→232, hub 99→72); A1 (e2e gate 13/16, export + kind routing implemented; ROOT BLOCKER: every guest traps on `InstanceOpen` under `OwnedRuntime`; 31 descriptor skips need regeneration).
Launched: Z1 (zero-touch + React launch rows forced to wgpu — G4 P0), T3b, T4b, T2b (finish TS to zero), A2 (InstanceOpen trap + settle pumping, e2e to green).
Still running from the first batch: H1, D1, C1b, M3, V1, B1a, B2b, AU1, AU2, K1, M4, B3a, U1, U2.
Queued: A3 descriptor regeneration for 31 plugins (≈6 min each, cargo-heavy), W3c (memo steps 6–8: DocumentAuthority over db, document WS → gateway, socket grants, one presence — gated on C1b's E2E baseline), W3d (Result-returning projection/session writes, sagas in ServerState), U3, B3b–d, M5, M6, O3, OB1.

### Fleet 3 progress (~04:40)
Done: D1 (dispatch closes Send-future ports, 8 ports re-closed, macros 41/41, server 83/83); V1 (taxonomy runnable but >1h50m; plugin-registry 2299→1836, 58 % is 🗄️stdio; layering 213 / deps 236 / purity 434 red; 23/65 variants lack a React launcher); M3 (os-mcp 336/1 — the 1 is the InstanceOpen trap → A2; wgpu agent panel on live bridge 75+3+21 green, wasm32 check clean; wgpu renderer crate 1012 pass / 30 fail + one SIGABRT test).
Launched: W3d (Result-returning store writes, sagas in ServerState, server gate, TS twin, math tests), V2 (23 launchers, registry data drift, taxonomy perf).
Queued additionally: R1 wgpu renderer crate 30 failing tests + SIGABRT.

### Fleet 3 progress (~05:20)
Done: K1 (launch row, 28 [DEBUG] leftovers, README, dead keys, /healthz, postgres+neo4j check clean; three "dead" routes refuted); AU1 (POST /auth/sessions, PBKDF2 on own Sha256, token-bucket limiter, 16/16 unit + 7/7 integration; hub lib 141/33 red elsewhere); AU2 (HubSignIn + SpaceBrowser + HubWorkspace at /hub, 55/55, 18 stories, 0 TS diagnostics; no live sign-in yet); U1 (TaskManager window, AgentCancel frame, ShellSync i18n, hub-connection indicator; 31 vitest + 48 Rust); U2 (pinch/two-finger, tablet breakpoint w/ wgpu parity gate, contrast check, Diagram a11y; 48 new tests).
Launched: AU3 (relay admits auth routes, /me, password change, first-user bootstrap, pg/neo4j credentials, LIVE two-user sign-in→invite→presence e2e), K2 (vitest ownership hash, orphaned suites census, 4_gate launch families, foundation-source fixture, documentId→artifactId rename, energy sim tests).
Queued: WG1 wgpu parity for sign-in/spaces/task-manager/indicator/agent-cancel; A3; W3c; U3; B3b–d; M5; M6; O3; OB1; R1.

### Fleet 3 progress (~06:30)
Done: B2b (dag ✅ reasoning ✅ full bar; playbook 4/5; norm + imperative root-caused, not fixed); T3b (framework TS 65→0; typecheck target runs e2e; vitest 163); T2b (hub TS 72→0; repo product 232→156; `verify interactivity apps` was fatal — fixed to 25); V2 (launchers 46/65 missing → 0/65 for both renderers, 134 ports no collisions; registry vitest 48/48; `verify taxonomy --scope` crash → 1 m 45 s; plugin-registry still 1836: 892 unreachable Rust mounts, 615 schema lanes, 194 directory lanes, 45 owner decisions).
Launched: B2c (norm factory, imperative Actions pane, playbook LoadDocument, dag 12 tests), B3b (trinity/wfc/puzzle), T2c (repo product 156→0), V3a (`surface-schema-projection` generator for the 615 schema lanes), V3b (892 unreachable mounts, directory lanes, interactivity gates, frozen-contract retirement, layering/deps/purity triage).
Still running: H1, C1b, B1a, M4, B3a, Z1, T4b, A2, W3d, AU3, K2.

### Fleet 3 progress (~07:45)
Disk hit ENOSPC ~03:57; B1a freed 16.7 GiB (superseded incremental sessions), coordinator pruned >18 h build units + >12 h nx cache; 71 GiB free after.
Done: Z1 (draw rows booted space — fixed; 11 duplicate launch names → 0; 3 compounds fixed; devcontainer → `workspace:setup`, 27-task graph; G4 P0#1 refuted); M4 (rendezvous bridge in stdio mode, ApprovalCoordinator chain, mock removed from prod, `.mcp.json --folder .`; 39+5 tests; gap: React shell never dials, approval round trip unproven live); B3a (hub check GREEN; gis2d boots + tiled-map lane; block assembles + descriptor generated; gis has no real mutations reachable); T4b (os vitest 361/361; owned 449→318; puzzle diff parsers written, 39 tests); B1a (6/6 boot, 2/6 dispatch+undo, 0/6 full bar; FRAMEWORK BLOCKER: `Effect::LoadDocument` archive replacement validation fails for every batch-A plugin); W3d (Result-returning store writes + fault laws ×3 profiles, sagas wired, server 90/90, TS twin 1176 lines w/ 30 shared wire vectors, math 191/191).
Launched: F1 (LoadDocument blocker + writer/animate/vcs leftovers), M7 (React shell dials bridge, elicitation timeout, live (a)–(e) agent loop e2e), T4c (318→0 + two half-renames through Rust/codegen), B3a2 (gis real mutations, block bar, ui-scene tests, facet-authority fixture).
Still running: H1, C1b, A2, AU3, K2, B2c, B3b, T2c, V3a, V3b.
Queued: W3c (after C1b baseline), WG1, A3 descriptors, U3, B3c, B3d, M5, M6, O3, OB1, R1, proven-eight + space re-verify.

### Outage 2026-09-19 ~08:15 → 11:00 (account session limit, reset 07:00 was already past when noticed)
Cut mid-flight (14): H1, C1b, A2, AU3, K2, B2c (had only just started), B3b, T2c, T4c, V3a, V3b, F1, M7, B3a2. No cargo/vite/hub process survived; disk 79 GiB free. Resumed 11:00 via SendMessage with context intact (preamble rules 17–18 added).

### Fleet 3 progress (~12:00)
Done: T2c (repo product 156→0, hub 0; VS Code extension load crash + dead CLI router found); AU3 (LIVE: hub gate `os-hub:live-sign-in-check` 38/0, browser probe 19/0 — two users sign in → space → invite → redeem → sign out; `POST /auth/credentials`, `os-hub credential set` operator verb, pg/neo4j credential paths, `os.openHub` verb, real roster; not yet observed inside the `s` host).
Launched (Sonnet, read-only): G8 wgpu parity spec, G9 `s` product / 🪐️space audit.
Running (resumed): H1, C1b, A2, K2, B2c, B3b, B3a2, T4c, V3a, V3b, F1, M7. Load ≈ 50 — no new cargo slices until some land.
