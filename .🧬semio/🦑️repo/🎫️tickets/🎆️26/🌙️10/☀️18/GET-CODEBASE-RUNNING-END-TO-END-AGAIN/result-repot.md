# Result repot: red `🦑️repo` bun suites (law2 `repo-candidates.txt`, 150 non-gated)

Executor repot, foreground, no sub-agents, no cargo, no modifying git command, no `package.json`/`bun.lock` edit, nothing edited under the frozen paths (`⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/**`, `🏗️build/📥️inputs/🔏️revision/**`). Tools: `repot-tools/` (runner `runbun.py`/`runall.sh`, single suite `r.py`/`rb.sh`, edit helper `ed.py`, per-suite verdicts in `repot-tools/decisions.md`).

## Before / after (bun, emulated hoist, runner envelope)

| | suites | green | tests pass / fail |
|---|---|---|---|
| before (law2 harness) | 150 | 4 | 2,284 / 621 |
| after | 148 existing (2 candidates are phantoms, see below) | 40 | 2,488 / 473 |

Counts are noisy by a few suites: deadline-based laws flip under fleet load; the after run is two 4-shard passes plus `workspace-contract` (720/37, 273 s alone, exceeds the runner cap under load). 36 suites turned green by my edits; the other 4 green were already green or load-flaky.

Phantom candidates (files no longer exist, bun exits 1 with 0 tests): `🏗️build/🧪️tests/📥️invocation/🟦️.ts`, `🏗️build/🧪️tests/📥️invocation/🧪️tests/🟦️.ts`.

## Harness improvements (no repo edit, reusable by others)

`repot-tools/envwire.ts` mints the envelope like the launcher: native build caller capabilities from `createNativeBuildCallerCapabilitiesV1`, plus `SEMIO_SCRIPT_PROVIDERS` from `issueRepositoryScriptProvidersV1`; runner adds `SEMIO_WORKSPACE_ROOT`, `SEMIO_BUILD_TOOL=nx` and a short artifact dir inside the repo (`.cache/repot/ta<shard>`, as the launcher). These alone turned bazel `arguments`, native-orchestration `invocation`, `authorizers`, `rust-source-direction/🪵️root`, `🧾️attributes` green and cut `package-boundary-classification` 18 -> 15 fails.

## Production defects fixed

- `🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts` `scriptResourceProcessEnvironmentV1`: the owned-command control path re-minted the child wire from the parent's birth envelope and discarded the caller-supplied (Nx-projected) child wire, so least-authority child data (`projectOwnedNxProcessEnvelopeV1`) never reached the child. Now the supplied envelope (same policy owner) is carried; elapsed ceiling and deadline remain the minimum of original, carried and phase. 33 routing/owned-execution suites: identical 25 green / 8 red with and without the edit.
- `🧰️framework/🔨️modules/🏃️process/🔒️leases/🟦️.ts`: queue/wait loops lost `signal.reason` (node `delay` AbortError); now rethrow the signal reason, so a deadline abort surfaces as the deadline error.
- `⚡️caching/🕸️graph/📤️publication/📦️transaction/🟦️.ts` `removeNext`: a partial unconfirmed staged file (write fault mid-stage, identity still null) made recovery throw, so the primary ENOSPC became an AggregateError; unconfirmed partial files are now removed.
- `⚡️caching/🌐️vite/🟦️.ts` `serveViteCommandV1`: `options.configLoader` was read late inside the compiler-state callback; captured with the other options.
- `🕸️dependencies/🟦️typescript/⚙️configured/🟦️.ts`: `createRequire(...)("observation/🟨️.mjs")` fails since its graph has the intentional top-level await (captured source-inventory implementation); now `await import`.
- `🌎️hub/🎛️commands/🚦️gate-commands/🟦️.ts`: spawned the `bunx` launcher (forbidden cross-platform); now `bun x`.
- Restored files deleted without replacement in bulk commit d4597d0 (law + fixture still consume them): `🧬️schema/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json`, `🧬️schema/🔐️owner-only/🔣️.json`, and four fixture schemas (`🛂️mutation-source-authority/🧭️domains.json`, `🎛️inline-tree-controls`, `🖼️scene-raster-ownership`, `✏️editing`) at their original owners.

## Laws re-pointed or rewritten (evidence in `repot-tools/decisions.md`)

Moved code: `repository-command` (runRepositoryTestCommand), discovery owners (`readme-current-source-revision`, `source-admission-io`), SDK binary verification read context, `build/tests/🎟️phase` (+producer binding, cargo port detection, diagnosticPhase), `selection/watch` bindings, `rustc` module tree, compiler-state needles, `store/required` (verifyInventoryCohortsV1), `fixture placement` paths (pool-use, deferred-wake, fixture-ownership corpus), kernel config rosters, `cargo-transaction-command-source` consumers, `owner-test-manifests` (provider port, type-only imports), extracted-helper laws (`cargo-discovery-exclusions`, `gitlink-boundary`, `artifact-empty-facet-authority`). Design changes: persisted inference records are brotli, `remainingMilliseconds()` throws on expiry, executors are `repo:script`, taxonomy `repo-process` -> `process`, observation order-insensitive compare, byte-pinned `fixture-law-ownership` rebaselined (3 pure moves + evolved digests).

## Still red (108 existing suites) by cause

1. Poison class (design: any rejected issued operation closes the shared Original, so a law that expects a rejection and then keeps using the Original cannot run in one process): `compiler/⏳️phase`, `cache/artifacts` (2 tests), `store/tests` + `📦️child`-less cascades, `graph/publication` parent, `sdk/📥️source/🔗️derivative`, `selection/watch` lifecycle child, `retention/📥️physical` (31), `build/👁️watch`. Needs one process birth per rejected case (child process per case).
2. Launcher/host gated: dashboard journeys (2, not installed), `transaction-v2` (62, shard runner env), `SEMIO_VITEST_POLICY` (locale-law-ownership test 1), `SEMIO_NX_TOOLING_IDENTITY` (configured, 4 tests), native images/hdiutil (`store/tests`, `graph/operation`), `bazel/dependencies` child, `discovery/runtime` build-producer (`Original Repo build caller`).
3. Cargo-gated (`cargo run`/`--unit-graph`): 12 rows of `package-boundary-classification`, `cargo-provenance`, `cargo/🏭️inventory` (profile tables), `queued-preparation`.
4. Laws target removed APIs (retire/rewrite needs owner ruling): `dependencies/💾️storage/tests` and `👓️reading` (publish/bind effects are read-context only now), compiler-state `recoverCompilerStatePressureV1`, `selection/📥️configuration` (`publishRustUnitDescriptorFilesV3`), `lib-wiring-budget`.
5. Data-heavy inventory laws (consumer/owner censuses stale after the module split): `framework-source-topology`, `framework-root-source-topology`, `manifestless-source-closure`, `wasm-package-wrappers` (full manifest pins), `workspace-publication-source`, `root-artifact-dependency-source`, `owned-script-routes`, `rust-source-direction/🧫️fixture-ownership` (consumers), `mutation-*` x4, `app-verification-source-ownership`, `vitest/tool-configuration-ownership`, `historical-*` x2, `os-source-topology`, `os-dev-composition-ownership`, `collection-catalog`, `readme-move-source-authority`, `transaction-recovery-authority`, `preflight-reference-basis`, `storybook-discovery`, `fixture-placement`, `workspace-contract/*` sub-suites.
6. Possible production regressions for owners: provider validation not shared across project images (`observation`, reads 6 vs 2), gate flags `history.full` (`fault-notices-gate`), defining-source closure 180 > ratchet 128 (`coordinator/builder/execution`), `exhaustive-cache-inputs` crate resolution picks a closer crate, `inference` copied-plugin law (closure of componentized inference incomplete; widening it mounts native images), MAX_PATH budget (`cross-platform-bootstrap`: tracked paths up to 304 UTF-16 units vs 213), `indexed-generated-output` (37 tracked generated files, untrack by owner), `package-boundary-classification` frozen contract `remaining-package-purity-history-v1` absent from `🔣️taxonomy.json`.
7. Dependencies (deps executor, manifests frozen): `npm-run-path` >= 5 exporting `npmRunPathEnv` (`process-budgets`), `smart-buffer` (`transaction-process-ownership`), `@tailwindcss/postcss` (`tool-configuration-ownership`), `@bytecodealliance/jco-transpile` (`discovery/runtime`), Bun TOML `\U` escapes (observation TOML law, Bun 1.3.14).

## Hygiene

Tests that mount native images (`inference` widening) were reverted; mounts under `.cache/repot` detached and removed. `🗑️generated/repot-*` and `.cache/repot` deleted at the end. Edited files: every path named above plus the test/fixture files listed per suite in `repot-tools/decisions.md`.
