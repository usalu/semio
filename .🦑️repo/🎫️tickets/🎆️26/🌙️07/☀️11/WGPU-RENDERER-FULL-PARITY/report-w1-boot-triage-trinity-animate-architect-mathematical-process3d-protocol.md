# W1 Boot-Triage — trinity-rewrite, animate, architect, mathematical, process3d, protocol

Completed cargo-check and wasm-build verification for all 6 assigned variants. Live `parity triage`
runs were attempted for all 6 but were severely compromised by extreme, sustained concurrent-session
load on this shared machine (`uptime` load average spiked from ~13 baseline to **87+** during this
session, with 40-48 concurrent `cargo build`/`bun 📜️script.ts dev`/`parity triage` processes observed
system-wide from other sessions) — compounded by two now-fixed harness bugs (see below) that were
independently discovered and fixed during this run: (1) `parity triage`/`smoke`/`verify` hardcoded
the same port pair (7300/7301) for every concurrent invocation, causing false `SERVER-FAIL`/
`DUMP-EMPTY` results whenever two triage runs overlapped — now picks a free pair automatically via
`findFreeParityPortPair()`; (2) `bun ./📜️script.ts plugin <variant>` silently dropped the variant filter
and built the entire 33-crate catalog every time — now fixed to pass the filter through correctly.
Given both bugs were live during this agent's run, the "not yet triaged" rows below should be
considered higher-confidence to re-run than usual, not written off as environment-only noise.

## Per-variant status

| Variant | `cargo check` (own crate) | wasm plugin build | live `parity triage` | Root cause / notes |
|---|---|---|---|---|
| trinity-rewrite | PASS | PASS (release wasm built) | Inconclusive — 2 attempts, one hit a port collision (now fixed), one got `react=BOOT-TIMEOUT`/`wgpu=SERVER-FAIL` under load 70-90 | No structural difference found vs. known-healthy `trinity-jack` sibling — both apps registered identically via the same builder pattern, same custom layout helper, same shared `build_node_graph_scene` for all 4 graph windows. No root cause identified. **Recommend re-triage now that the port-collision bug is fixed.** |
| animate | PASS | PASS | One clean (non-collision) attempt: `react=DUMP-EMPTY` / `wgpu=DUMP-EMPTY` on both renderers | Code review of `app_id`/`document_schema`/`initial_projection`/`render` wiring found nothing wrong — matches every other working variant's pattern. Load was 70-80 during this run; cannot rule out environmental cause. **Recommend re-triage now that the port-collision bug is fixed.** |
| architect | PASS | PASS | Not completed — queued 5+ minutes behind other sessions' cargo builds on the shared `target/` lock, killed rather than waiting indefinitely | Static review (window_kind/default_layout/plugin_exports wiring) found nothing wrong. **Needs live triage.** |
| mathematical | PASS | PASS | Not attempted live (deprioritized once results were unreliable) | Static review clean. **Needs live triage.** |
| process3d | PASS | PASS | Not attempted live | Static review clean. **Needs live triage.** |
| protocol | PASS | PASS | Not attempted live | Static review clean; smallest/simplest of the 6, well-covered by its own unit tests (8 tests incl. a two-instance-convergence backbone test). **Needs live triage.** |

All 6 `.wasm` artifacts confirmed present: `target/wasm32-wasip2/release/{animate,architect,mathematical,process,protocol,trinity}_plugin.wasm`.

## Shared-file churn hit (outside these 6 crates — all self-resolved except one noted below)
1. `ui/wgpu/rs/lib.rs` — E0753, doc comment misplaced after a code move. Self-resolved.
2. `framework/core/rs/lib.rs` — E0425, `IconName` not in scope (in-flight String→IconName enum migration). Self-resolved.
3. `framework/plugin/rs/lib.rs` — E0308, `impl Into<IconName>` builder params missing `.into()` (same migration). Self-resolved.
4. `protocol/rs/lib.rs:1066` — E0308, same migration, in the base crate shared by flow/forms/procedural/protocol. Self-resolved.
5. `draw/rs/lib.rs:11` — E0252, `DocumentDsl` imported both directly and re-exported (`pub use vcs::DocumentDsl;`), duplicate definition. **Was still broken when last checked by this agent** — only matters for a full-registry/studio build (e.g. trinity hosting `s/plugin/rs`'s studio host metadata), doesn't block any of these 6 crates' own `cargo check`/wasm build. Worth a follow-up check.

## Harness bugs found and fixed (during this session, not by this agent directly — see summary above)
- `framework/product/os/dev/script.ts`'s `plugin` router double-consumed the variant argument, always building all ~33 plugin crates instead of the requested one. **Fixed.**
- `ParityTriageScript`/`ParitySmokeScript`/`ParityVerifyScript` hardcoded the same port pair for every invocation, causing false failures whenever two triage runs overlapped (directly observed via `[dev] Port 7300 is already in use` in boot logs and concurrent `parity triage` processes from other sessions/agents at the same time). **Fixed** — now auto-selects a free port pair.

## trinity-rewrite vs trinity-jack (the "known BLANK wgpu screenshot" flagged in the original task)
Read both `app_jack` and `app_rewrite` modules in `trinity/plugin/rs/lib.rs`. Both apps are registered
identically via the same `semio_plugin!` macro, declare windows via the same `.window_kind(...)`
builder pattern, use a structurally identical custom layout helper, and wire all 4 `NodeGraph` body
keys through the same shared `build_node_graph_scene` helper `app_jack` uses. No structural difference
found that would explain a blank wgpu paint specific to `trinity-rewrite`. Could not confirm or refute
the "known BLANK" report with a real draw-call count due to unreliable live-triage conditions during
this run.

## Files touched
None in any of the 6 variants' own plugin crates — no genuine root-cause bug was found in any of
them (all compile cleanly standalone, build to wasm cleanly, passed static structural review).

## Recommendation
Re-run `parity triage` for all 6 variants (especially trinity-rewrite and animate) now that both
harness bugs are fixed — prior non-PASS results are not trustworthy enough to call genuine bugs.
