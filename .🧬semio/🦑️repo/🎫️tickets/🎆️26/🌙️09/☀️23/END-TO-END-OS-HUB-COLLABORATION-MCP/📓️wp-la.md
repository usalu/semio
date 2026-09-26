# WP-LA — Landing Window, Framework/Host Core

Slice LA (session 13, 2026-09-26 19:0x). Coordinator = main chat. Ports 8070–8079 / 6570–6579 (none needed so far).
Private cargo target (tests): `.tmp-ticket/wp-la/target`. Captures: `wp-la/generated/` (expendable); durable `.🧬semio/🌐hub/s13-la-*`.
Landing rows: `📓️landing.md` § `# Session 13 Landing Window`. Guest-crate requests: `wp-w3/requests/la.txt`.

## Session 13

| # | Item | State | Evidence |
|---|---|---|---|
| LA-1 | R8 `nx-narrowed-inputs.py` (Nx `inputs` narrowed; HashPlanInspector re-plan) | **LANDED 19:06, verified**: re-plan on the landed tree = R8 simulation (materialize-dev median 73 153 → 13 432 files; one-plugin edit invalidates median 4 / max 51 others, was 59/59; stdio edits miss exactly their 51 Cargo dependents). Follow-ups: root crate's native lists closed (`workspaceNegationClosed`), new law `⚡️workspace-negation-closure` 4/4 (Nx planner oracle, red on old nx.json), `⚡️production-cache-input-boundary` truths re-derived 1/1 | `s13-la-captures/nx-landed-replan-3.json`, `wp-la/generated/law-negation-closure-2.txt` |
| LA-2 | framework-os tsc error `⌨️text-input-oracle` (F1) | **LANDED 19:31**: root = cast of the JSON fixture to readonly tuples (no overlap) → fixture assigned to the harness type, no cast; file tsc-clean; law 29/29. Full `framework-os:typecheck` still red on peers' in-flight edits (`inlineToolbar` ×13, `🚪️host-io` BodyInit, `🧩️wgpu-extension-store-door` fixture) → re-run in LA-7 | `wp-la/generated/tc-text-input-oracle-1.txt`, `law-text-input-oracle-1.txt`, `s13-la-captures/framework-os-tc-1.txt` |
| LA-3 | S15 S12-3k `"waiting"` in `ArtifactCreationProgressCopyV1` + engine-contract law | **LANDED 19:47**: key restored exactly as S15 specified (+ description); no Rust includes the schema; `schema generate` rerun; engine-contract **673/673** incl. the lifecycle law | `wp-la/generated/law-engine-contract-full-1.txt` |
| LA-4a | H10 Q1 owned-interpreter speedup (`q1-land.sh`) | pending | — |
| LA-4b | H10 Q2 hardware SHA-256 (`q2-sha256-hardware.py`) | pending | — |
| LA-4c | H10 guest codec-app resolution (`codec-app-resolution.py`) | pending | — |
| LA-5 | R8 `ui-retirement-item-metered.py` | pending | — |
| LA-6 | other framework/host-core prepared sets | pending | — |
| LA-7 | suites: plugin-host identity sweep, hash digest oracle, ui-contract, ui-runtime, framework-os typecheck | pending | — |

### Session 13 log

- 19:05 read AGENTS.md, preambles 13/12, handovers `📓️wp-r8.md`, `📓️wp-f1.md`, `📓️wp-s15.md` (S12-3k), `📓️wp-h10.md`. Load 21, 113 GiB free,
  1 rustc, wasm mutex free, no W3 "REBUILD START" yet (`wp-w3/` absent).
- 19:06 LA-1 applied (dry run clean). Re-plans 1/2 died on Nx's 10-min plugin timeout (`nx/js/dependencies-and-lockfile`, load 80–97);
  run 3 with `NX_PLUGIN_NO_TIMEOUTS=true` (pid 48811, detached) finished 20:16: equal to R8's simulation (table above).
- 19:17–19:39 LA-2: framework-os typecheck (683 s at load 80) → the oracle's TS2352 is the only error of mine; fixed; isolated tsc of the file clean;
  law 29/29 (Chromium oracle).
- 19:47–19:57 LA-3: `waiting` restored; `schema generate --check` said stale (peers' schemas + this hash) → regenerated; engine-contract 673/673.
- 20:02 LA-4a Q1 applied (dry run 11 hunks clean; before-copy `s13-la-captures/q1-interpreter-before.rs`). Checks 1–2 red on the kernel
  (`📡️spr` imports LD's `mutation_envelopes_from_edit_since`): the replication rmeta in the shared build-dir was a stale-fresh unit (fingerprint
  pointing into a P8 scratch clone, coordinator rule 24) → `touch` of `📡️replication/🔗️causal/🦀️.rs` (content unchanged). Check 3: **plugin-host
  `--lib --tests` rc 0** (15 m 25 s, 78 warning lines). Interpreter laws building (pid 76023, private target).
- 20:2x–20:45 LA-1 laws: the boundary law was red before my set (runner-tree samples stale since the runner moved to command sources) and would
  turn red on the nx.json assertion → truths re-derived with owner-tree samples; Nx's planner (throwaway workspace) shows references are
  flattened (a workspace positive through a reference closes the list) → closure predicate expands references; the workspace-root crate's
  `nativeSources`/`nativeTestSources` were the only open lists repo-wide → `workspaceNegationClosed` for native lists; law 4/4 (long level incl.
  all 438 projects' derived inputs).
