# W0 barrier — baseline, contract, scouting

Coordinator: Opus 5 main chat. Wave ran 5 agents (2 Sonnet lanes + 3 Haiku scouts) concurrently.

## Baseline (see `🧪️w0-baseline-*.txt`, `🧪️w0-gate-baseline.txt`)

| # | Command | Result | Attribution / decision |
|---|---|---|---|
| 1 | `cargo check -p semio-hub --all-features` | **RED** (76 errors in `semio-framework-os-kernel-db`) | **Pre-existing, not ours.** `🛢️db`'s `Cargo.toml` has `sqlite = []`, `postgres = []`, `neo4j = []` with no optional `rusqlite`/`sqlx`/`neo4rs` deps wired, so `--all-features` compiles storage code against crates that are not dependencies. `git log --date=iso` on that Cargo.toml: last touched **2026-08-12**. `🛢️db` is peer-leased → we do not fix it. **All hub lanes verify with default features (sqlite) instead**; `bun nx run os-hub:test*` is unusable because it hardcodes `--all-features`. Frozen as Amendment 2 in `📋️contract-freeze.md`. |
| 2 | `cargo check -p semio-framework-os-kernel -p …-db -p semio-framework-plugin -p semio-framework-os` | **GREEN** (26.5 s, warnings only) | — |
| 2b | `cargo check -p semio-hub` (default = sqlite) | **GREEN** (lane 0-B, `🧪️0-b-hub-check.txt`) | this is our working hub gate |
| 6 | os kernel `cargo test --lib directory` | **GREEN**, 6/6 (lane 0-A) | — |
| 6 | os TS vitest (directory parity) | **GREEN** for the 2 new tests; 320/322 of the rest pass, 2 pre-existing wasm-artifact failures | machine was saturated (410+ concurrent processes for the same nx target from other sessions), so 0-A ran the underlying `vitest run` directly — recorded honestly in `📓️w0-a-report.md` |
| 7 | `bun ./📜️script.ts verify gate` | **RED at step 1** — dependency-cruiser: 828 violations (651 errors), exit 139; no later step ran | Pre-existing, confirmed by the `ARTIFACT-VIEWERS…` ticket's own final summary. Baseline captured in `🧪️w0-gate-baseline.txt`. Our success criterion is **no NEW failing step**, which today means "still fails only at dependency-cruiser". |

**Verdict: START W1.** Items 2, 2b and 6 are green; 1 and 7 are pre-existing peer/legacy breakage with a
documented workaround.

## Lanes

- **0-A (schema + fold) — done.** New `💻️os/🔨️modules/📇️directory/{🧬️schema/*,🦀️component.rs,🟦️component.ts}`,
  golden fixture `💻️os/🧫️fixtures/📇️directory/🧾️events.json` (16 events, dense seq), Rust + TS `fold`
  twins with a shared-fixture parity test both sides. One additive `pub mod os_directory` in the kernel
  glue and one new `🔖️Directory` region in `💻️os/🟦️component.ts`. Found and fixed a real bug on the way
  (`DirectorySpaceRole` needed `Ord`).
  **sharedFileRequest accepted → contract Amendment 1**: the space-kind field is `spaceKind`, not `kind`
  (serde tag collision with the body discriminator). Contract updated; all lanes use `spaceKind`.
- **0-B (ports/env) — done.** `OS_HUB_PORT` 6070 → **8787** in the repo library and in `📦️bin.rs`;
  `🌎️hub/…/📜️script.ts` `DevScript` no longer replaces `process.env` wholesale (the launcher's
  `OS_HUB_PORT`/`OS_HUB_DATA` were being silently dropped). Also found and fixed a **broken import path**
  in that script (5 × `../` where the file sits 3 levels down) which made `bun nx run os-hub:build`/`:dev`
  fail outright. Proven end to end: hub boots, `lsof` shows it listening on 8787, and
  `/tmp/semio-hub-0b/directory.db` proves `OS_HUB_DATA` now reaches the process.
- **0-S1 hub scout — done** → `📓️scout-hub.md` (trait, backends, bin.rs regions/state/routes/test helpers,
  db public API, protocol frames, the axum static-serving pattern to copy, hub deps). Confirms
  `PresencePeer`'s flag byte uses all 8 bits → the `?surface=` out-of-band decision stands.
- **0-S2 client scout — done** → `📓️scout-client.md` (written by the coordinator from the agent's
  findings; see the caveat header in that file).
- **0-S3 gate scout — done** → `📓️scout-gates.md` + `🧪️w0-gate-baseline.txt`.

## Carried into W1

1. Hub lanes: default features only (Amendment 2).
2. `spaceKind` everywhere (Amendment 1).
3. The existing `OpeningPreferences` config facet is the exact template for the new `🪪️identity` facet,
   including its `bindings: []` local-only pattern (`📓️scout-client.md` §7).
4. `data-ui-path` was **not** found in the React `📊️Table` element — lane 2-A/2-B/2-F must add it where
   the wgpu parity join needs it, rather than assuming it exists.
