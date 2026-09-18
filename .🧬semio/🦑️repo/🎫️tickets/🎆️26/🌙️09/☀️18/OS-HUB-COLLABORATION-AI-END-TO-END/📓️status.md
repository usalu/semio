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
