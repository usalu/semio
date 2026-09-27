# WP-LA — Landing Window, Framework/Host Core

Slice LA (session 13, 2026-09-26 19:0x). Coordinator = main chat. Ports 8070–8079 / 6570–6579 (none needed so far).
Private cargo target (tests): `.tmp-ticket/wp-la/target`. Captures: `wp-la/generated/` (expendable); durable `.🧬semio/🌐hub/s13-la-*`.
Landing rows: `📓️landing.md` § `# Session 13 Landing Window`. Guest-crate requests: `wp-w3/requests/la.txt`.

## Session 13

**Summary (10:1x 09-27):** all LA sets are in the tree and compile-green. Nx inputs narrowed (re-plan equals R8's simulation: materialize-dev
median 73 153 → 13 432 files, one-plugin edit invalidates median 4 others instead of 59) + new closure law. F1 oracle tsc fix. S15's `waiting` restored.
H10 Q1 (sweep 17/17 identical, ~2.3×), Q2 (+ HMAC/PBKDF2 moved into the hash crate; mint law green). Codec-app resolution (superseded by H12's
no-app codec table). R8 ui retirement (203/203 + 125/125). Open, not LA's: hub trusted-catalog descriptor-identity reds (17, → H11),
framework-os typecheck 64 peer errors (→ R9), plugin lib-test peer reds (→ G11). Guest crates: `wp-w3/requests/la.txt`.

| # | Item | State | Evidence |
|---|---|---|---|
| LA-1 | R8 `nx-narrowed-inputs.py` (Nx `inputs` narrowed; HashPlanInspector re-plan) | **LANDED 19:06, verified**: re-plan on the landed tree = R8 simulation (materialize-dev median 73 153 → 13 432 files; one-plugin edit invalidates median 4 / max 51 others, was 59/59; stdio edits miss exactly their 51 Cargo dependents). Follow-ups: root crate's native lists closed (`workspaceNegationClosed`), new law `⚡️workspace-negation-closure` 4/4 (Nx planner oracle, red on old nx.json), `⚡️production-cache-input-boundary` truths re-derived 1/1 | `s13-la-captures/nx-landed-replan-3.json`, `wp-la/generated/law-negation-closure-2.txt` |
| LA-2 | framework-os tsc error `⌨️text-input-oracle` (F1) | **LANDED 19:31**: root = cast of the JSON fixture to readonly tuples (no overlap) → fixture assigned to the harness type, no cast; file tsc-clean; law 29/29. Full `framework-os:typecheck` still red on peers' in-flight edits (`inlineToolbar` ×13, `🚪️host-io` BodyInit, `🧩️wgpu-extension-store-door` fixture) → re-run in LA-7 | `wp-la/generated/tc-text-input-oracle-1.txt`, `law-text-input-oracle-1.txt`, `s13-la-captures/framework-os-tc-1.txt` |
| LA-3 | S15 S12-3k `"waiting"` in `ArtifactCreationProgressCopyV1` + engine-contract law | **LANDED 19:47**: key restored exactly as S15 specified (+ description); no Rust includes the schema; `schema generate` rerun; engine-contract **673/673** incl. the lifecycle law | `wp-la/generated/law-engine-contract-full-1.txt` |
| LA-4a | H10 Q1 owned-interpreter speedup | **LANDED 20:02 (09-26), laws green 10:06 (09-27)**: plugin-host check green; interpreter laws **15/15**; identity sweep **17/17** guest rows SAME + TRUSTED (+11 stdio SAME); hub `artifact_authority::` 104 pass / 17 red, none from LA (descriptor-identity family, routed to H11) | `s13-la-captures/q1-sweep-{1,2}.txt`, `hub-laws-5.txt` |
| LA-4b | H10 Q2 hardware SHA-256 | **LANDED 20:51, green**: hash **13/13** (hardware == portable, `sha2` oracle, RFC HMAC/PBKDF2 composition oracle); WASI profile reconciled (law 3/3); follow-up: HMAC-SHA256 + PBKDF2 moved into the hash crate (2 compressions/iteration) → hub mint-budget law PASS (was red with Q2 alone), OpenSSL vector laws PASS | `q2-hash-test-4.txt`, `hub-laws-5.txt`, `landing-check-wasm-3.txt` |
| LA-4c | H10 guest codec-app resolution | **LANDED 21:07**, native + wasm32 green; **superseded on top by H12's codec fn table** (no app per codec call, 05:29) — its law `codec_calls_construct_no_app` is H12's (running in the native lane at 10:0x) | `landing-check-native-5.txt`, `landing-check-wasm-2.txt`, H12 row |
| LA-5 | R8 `ui-retirement-item-metered.py` | **LANDED 05:02, green 05:12**: native + wasm32-wasip2 green; ui-contract nextest **203/203** (was 174/197), ui-runtime **125/125** | `s13-la-captures/ui-contract-nextest-1.txt`, `ui-laws-1.txt` |
| LA-6 | other framework/host-core prepared sets | **none left for LA**: the audit's LA-6 (R8's flow_core wasm-pack bindings step) is in W3's chain (`flow-core-bindings`), skipped on the coordinator's word; W2's `VersionReq` narrowing is W3's; Z2 winit is Z3's | coordinator 05:4x |
| LA-7 | suites: plugin-host identity sweep, hash digest oracle, ui-contract, ui-runtime, framework-os typecheck | identity sweep 17/17 guest rows SAME+TRUSTED (+11 stdio SAME); hash 12/12 (NIST + `sha2` oracle + hardware == portable); ui-contract 203/203; ui-runtime 125/125; WASI profile 3/3; **framework-os typecheck rc 2: 64 errors in 22 files, none in LA's sets** (numberStepper `min`/`max`, treeItem `inlineToolbar`, wgpu host-io/accessibility-mirror, stdio kit mutations TS, …: fleet in-flight edits; routed to R9 by the coordinator); F1's `⌨️text-input-oracle` error gone | `s13-la-captures/framework-os-tc-2.txt` |

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
- 20:49 Q1 identity sweep launched (pid 92250); 20:51 Q2 applied; 21:05 hash laws 12/12 (default build-dir, after 20:14); 21:07 codec-app applied;
  21:24 plugin + hash native check green in `build-landing` (rule 27). ~21:30 cut by the usage limit (every process died overnight).
- 04:57 resumed (rule 28). Reconciled: all my edits were auto-committed at 22:00 (`40a2736e661`) and are intact; the plugin crate root carries a
  peer's overnight + in-flight edits (tree windows 01:29; composite `child_ops`/`EmitWire` 05:05–05:11), not mine. Q1 sweep had reached 22/43 rows
  (all SAME) → re-ran the remaining guest rows (wfc ×5, writer): all SAME + TRUSTED.
- 05:02 LA-5 applied. 05:0x native check red on a peer's `🌱️value/🔁️codec` (missing closing quote in `format!("missing object key `{key}`)`,
  01:57, 3 h untouched) → one-character fix (rule 14); then red on the peer's in-flight plugin `EmitWire`/`child_ops` edit → split the check.
- 05:10–05:15 ui-contract/ui-runtime/hash native green, wasm32 green; ui laws; hash laws; WASI profile reconciled (W3 relay) + law 3/3.
- 05:20 plugin lib + plugin-host lib/tests green; plugin lib-test red on peers' tests only. 05:27 plugin wasm32 green.
- 05:23 detached: framework-os typecheck (pid 6330), hub `artifact_authority:: auth::` laws (pid 6543, private target, build-landing).
- 05:2x framework-os typecheck (detached): rc 2, 64 errors / 22 files, none in LA's files (16 errors at 19:28 → 64: fleet in-flight work).
- 05:3x H12 asked to replace the codec-app construction with a codec fn table → OK'd (H12 owns that region + the law now). The 05:23 hub-law
  build died on H12's half-landed `AppFactory.codec` (E0063 ×3); files quiet from 05:35. Coordinator: skip LA-6 (W3's chain has
  `flow-core-bindings`); plugin lib-test reds → G11; typecheck → R9. 05:37–05:47 waited for rustc ≤ 14 (stayed 37, five fleet cargos) →
  hub laws launched anyway at 05:47 for the 06:55 deadline (pid 30081; coordinator told).
- 05:47–06:06 hub `artifact_authority:: auth::` run 3: **103 pass / 18 fail**. Classified: (a) `auth::…mint_budget` — **caused by Q2**: with
  hardware compressions the 420 000 bulk compressions cost 116 ms while the hub's own PBKDF2/HMAC loop (hub crate, opt-level 0 in dev/test)
  cost 1.08 s → the law's "a derivation costs what its compressions cost" broke; (b) 2 × exact-pin (`manifest.dependencies…pins an exact
  version`, W3's `VersionReq` narrowing in flight in `🛂️manifest`); (c) 15 × trusted-catalog (`decoded package descriptor identity does not
  exactly match its trust record` ×10 + 5 others) — `validate_descriptor` runs before any guest interpretation, so Q1/Q2 cannot reach it by
  construction; the trusted-catalog code + tests carry peers' uncommitted edits (05:42, H11/H12 area).
- 06:1x (a) root-fixed: HMAC-SHA256 + PBKDF2 are hash primitives → moved into `semio-framework-hash` (the crate the dev profile already runs at
  O3 so "a dev hub's sign-in costs what a release hub's does"): `HmacSha256` keyed once as raw states, `mac32` = exactly two compressions per
  PBKDF2 iteration (fixed final blocks), `hmac_sha256`, `pbkdf2_sha256`; the hub's copies deleted, `password` + its laws call the framework's
  (the hub's OpenSSL vector laws stay the third-party oracle); new hash law `hmac_and_pbkdf2_agree_with_the_rfc_composition_over_the_sha2_oracle`
  → hash **13/13**. Not touched: `🚀️local-bootstrap`'s domain-separated HMAC and the repo coordinator's HMAC (separate framings; follow-up).
- 06:21 hub laws run 4 (pid 52774); hash wasm32 check queued in the wasm mutex behind ld/wg10/n1/lb.
- 09:5x resumed (rule 30, REBUILD START 09:53). Hash wasm32 check had run Fresh/green at 06:36 (a peer's build-landing wasm check compiled the
  hash crate after my 06:12 edit). Hub laws run 4 died at the 06:35 cut → run 5 through the `native` lane (build-fleet-b, nice 15): mint budget
  PASS, HMAC/PBKDF2 vectors PASS, 104/121 `artifact_authority:: auth::`; the 17 reds = descriptor-identity family (not LA; → H11).
  Codec-app law: H12's no-app table replaced my resolution; H12 runs `codec_calls_construct_no_app` itself (its 06:20 run died at the cut).
- Processes: none of mine running (all detached runs exited). Stopped nothing of anyone else.
