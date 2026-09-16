# Status: Dev Process React End To End

**Status:** closed 2026-09-16
**Goal:** Get the process plugin working end to end (🛠️dev🪚️process🏙️3d⚛️react, port 6022).
**Bookkeeping:** manual on disk — repo MCP failed to connect (connection closed) for the whole session.
**Report:** [dev-process-react-e2e-2026-09-15.md](./📓️dev-process-react-e2e-2026-09-15.md)

## Outcome
Build → activate → serve → browser is green: the process3d editor boots with its four extension catalogs, every
panel publishes, the workpiece renders the kernel-replayed timber beam (crosscut, lap joint, dowel bore, dowel),
and document mutations (install a machine, toggle a step) round-trip through the op log and re-replay.

## Fixed (see the report for root causes)
1. Wasm component red: `PluginApp::tool_run_trace_delta` named a crate 23 plugins do not depend on → bare `ToolRunTraceCursor` via the prelude.
2. Workshop panel never published: `UiFixedList::try_push` reserved 32 slots per cold push (~240 KiB/binding, ~300 KiB/row action against an 8 MiB surface budget) → page-exact reservation; settle errors now carry the guest's fault text.
3. App trapped at construction: process3d child handles had `child_id ≠ target.artifact_id` → both helpers mint the same id; 44 assets repaired.
4. Workpiece showed the unit-box fallback: process3d posed primitives by corner (documents are centre-posed) → centred; brep boolean gaps for box-through-box / flush / blind cuts and a line-clip window bug → fixed with seven kernel laws.
5. process3d unit suite 246 → 294 / 331 (harness B1 dispatch, roster counts, close-lane disposers, 45 re-minted fixtures).

## Pre-existing red left as-is (verified on HEAD)
- brep kernel: 32 failing mutation-fixture / snapshot round-trip laws, `offset_sphere_matches_closed_form` timeout.
- ui-contract: ~20 retained-copy/assembly/retirement laws; ui-runtime: 2 canonical-document laws.
- process3d: 37 laws (render harness serialisation, 🌉️wasm bridge laws, `brep:out` export kind, a few effect assertions) — listed in the report.
- renderer typecheck: a peer's unstaged `req` in `FaultScope` (`🔌️PluginRuntime/🟦️.tsx:3205`).
