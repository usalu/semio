# Audit — how puzzle3d's fill tool is tested and deployed today

Read-only inventory for verifying a redesigned interactive, streaming fill (visible candidates + progress). No source touched.

## 1. Tests under `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/`

All four are **TypeScript "policy" tests**, not runtime tests: each imports helper functions and file-path constants from the single root `📜️script.ts` router (25,434 lines), reads the *live* Rust/TSX source of the fill implementation as raw text via `policyReadRustPolicySource(repoRoot, path)`, and runs a static-analysis/pattern predicate (`…Failures(sources…) → string[]`) over that text. They never compile or execute the fill algorithm. Each test file also runs a mutation self-test suite first: it string-replaces a known-good snippet in the source text with a broken variant and asserts the predicate now *rejects* it (catches "the check itself is dead/tautological"), then asserts the predicate accepts the real, unmutated source (catches "the check is too strict for real code"). Runner: **`bun`**, invoked from inside the root `verify` command — there is no vitest/jest here despite living in a `🧪️tests` folder.

- `🔬️interactivity-puzzle-fill-p4e/🟦️.ts` → `interactivityPuzzleFillP4eSelfTests()`, calls `interactivityPuzzleFillP4eFailures(precompute, fill, geometry, schema, transport, renderer)` (`📜️script.ts:9535`). ~38 mutation cases. Reads:
  - `INTERACTIVITY_AUDIT_PUZZLE_FILL_ENVELOPE_FILE` = `…/✏️editor/⏳️precompute/🦀️.rs`
  - `INTERACTIVITY_AUDIT_PUZZLE_FILL_STATE_FILE` = `…/⏳️precompute/🪣️fill/🦀️.rs`
  - `INTERACTIVITY_AUDIT_PUZZLE_FILL_GEOMETRY_FILE` = `…/⏳️precompute/📐️geometry/🦀️.rs`
  - `INTERACTIVITY_AUDIT_PUZZLE_FILL_SCHEMA_FILE` = `…/🧬️schema/🦀️.rs`
  - `INTERACTIVITY_AUDIT_PUZZLE_FILL_TRANSPORT_FILE` = `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs`
  - `INTERACTIVITY_AUDIT_PUZZLE_FILL_RENDERER_FILE` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
  Asserts P4E-class invariants: no whole-collection cloning/rebuilding of the fill builder or spatial index, fixed-capacity owners (`FixedOwnerVec`/`FixedOwnerMap`) not silently swapped for `Vec`/`HashMap`/`BTreeMap`, every capacity-preflight branch checked before mutation, fault-before-diagnostic ordering, the renderer overlay gated on a monotonic identity comparison (`identity[4] >= latest[4]`), and that a list of specific fixture test-function names still exist verbatim (guards against a fixture being renamed/weakened to a "smoke" stand-in).

- `🔬️interactivity-puzzle-fill-envelope/🟦️.ts` → `interactivityPuzzleFillEnvelopeSelfTests()`, calls `interactivityPuzzleFillEnvelopeFailures(precompute, fill, geometry, action)` (`📜️script.ts:9460`). ~55 mutation cases covering the same 4 files (envelope/state/geometry) plus `INTERACTIVITY_AUDIT_PUZZLE_FILL_ACTION_FILE` = `…/✏️editor/🎮️commands/🪣️fill-build-tick/🦀️.rs`. Asserts admission/checkout/close-step correctness for the job-envelope pattern: exact byte/item caps, no aliasing across generations, `Drop` handback, terminal reclamation, checked (not wrapping/saturating) counter arithmetic, and presence of ~20 named fixture test functions (`fill_worker_token_reopens_the_exact_retained_owner_and_drives_one_turn`, `fill_worker_cross_generation_restore_rejects_measuring_and_every_live_terminal_phase`, etc.).

- `🔬️interactivity-puzzle-fill-preview-json/🟦️.ts` → `interactivityPuzzleFillPreviewJsonSelfTests()`, calls `interactivityPuzzleFillPreviewJsonFailures(...10 sources)` (`📜️script.ts:9729`, exported). Reads 10 files including both puzzle3d and **puzzle5d** fill precompute/window files, both plugins' `🗣️terminology/🦀️.rs`, the fixture JSON, and a renderer contract test file (`🧰️framework/…/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`). Asserts: `FILL_PREVIEW_JSON_MAX_BYTES = 4096` cap, deadline/fuel budgeting for the incremental JSON writer (no lost fuel decrement, no backward clock, checked deadline arithmetic), the puzzle3d/puzzle5d consumer call sites are wired, the TS-side shape checks (`candidatePage.length !== 8`, `Array.isArray(candidateGhost)`, allowed-own-key census on root/diagnostic/ghost objects), locale defaults, and that the language-neutral fixture's `maximumBytes: 4096` hasn't drifted.

- `🔬️tool-job-puzzle-reserved-routes/🟦️.ts` → `toolJobPuzzleReservedRoutesSelfTests()`, calls `toolJobPuzzleReservedRoutesExact(source, host)` (`📜️script.ts:3596`). This one is **puzzle5d** copy/cut/paste/import reserved-route plumbing, not fill directly — included in your ticket's inventory list but not itself fill logic. 13 mutation cases against an inline synthetic source string (not a real file read) checking reserved-tool-job wiring shape (factory registration, fixed-page ingress, cancellation, commit-output parity, host freshness/exact-ACK/terminal-close checks).

### Invocation (Nx targets / project.json)

There is **no `📋️project.json` at `✏️s/🔌️plugins/🧩️puzzle/`** — no per-plugin test target. These four files are imported directly by the **root** `📜️script.ts` (import lines 17, 28–30) and run as part of:

```
bun ./📜️script.ts verify interactivity
```
wired to the Nx target in root `📋️project.json`:
```json
"verify-interactivity": {
  "executor": "nx:run-commands",
  "options": { "command": "bun ./📜️script.ts verify interactivity" }
}
```
→ `bun nx run workspace:verify-interactivity`. `VerifyScript.run()` (`📜️script.ts` ~line 8037, `runInteractivityAudit`) calls the site at line 9384–9401 that threads `interactivityPuzzleFillEnvelopeSelfTests` → `interactivityPuzzleFillP4eSelfTests` → `interactivityPuzzleFillPreviewJsonSelfTests` in that order, folding their `…Failures(...)` results into a single `report.findings` array under category `"blocking-bridge"`. `INTERACTIVITY_AUDIT_SEVERITY` gates whether findings are fatal (`"deny"`, default) or reporting-only (`"warn"`).

There is also `bun ./📜️script.ts verify interactivity apps` (separate `verify-interactivity`-adjacent path, not wired to these fill tests) and the plain `nx test` root target (`workspace:test`, `dependsOn: [{target:"test", projects:["*","!workspace",...]}]`) which does **not** touch these four TS files — they only run under `verify interactivity`.

**To run just the fill policy tests today** there is no narrower Nx target; the fastest path is `bun nx run workspace:verify-interactivity` (runs the whole interactivity audit, several seconds, pure string processing, no build) or invoking the private self-test functions directly with `bun -e` against `📜️script.ts`'s exports (all four self-test functions are exported).

## 2. Language-agnostic test / third-party-oracle pattern

The canonical, already-working example for fill is:

- **Fixture (language-neutral, consumed by ≥2 implementations):** `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json` — schema `"semio.puzzle3d.fill-preview-json.v1"`. Declares `limits` (byte/item caps), `documentCapacities`, and `boundaryLaws` (max/max+1 admitted/refused pairs for `sourceVortexIndex`, `color`, `statusLabel`, `fullWire`) each tagged with `"admitted": true/false` and, for the wire-level law, `"oracle": "serde_json"`.
- **Rust consumer + independent oracle:** `…/⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` (lines ~90–240). The test `retained_preview_json_matches_language_neutral_fixture_and_test_only_serde_oracle` (line 239) does `include_str!("../../🧫️fixtures/🔣️.json")`, parses it as `schema-first` (`serde_json::from_value` into the same `FillBuildPreview` struct the production code uses), then compares production's hand-written fixed-page byte writer (`fill_preview_json_page`) byte-for-byte against `oracle_json(...)` — a **separately hand-written** `OracleRoot` struct serialized through `serde_json::to_string` (the "third-party library" — CLAUDE.md's rule is satisfied by using `serde_json`'s real `Serialize` derive as the independent implementation, not a second in-house serializer). `oracle_json_admits`/`oracle_json_scalar_admits` re-derive the same byte/field caps independently from the schema types rather than importing the production constants, so a production regression in the caps can't silently agree with a broken oracle.
- There is **no dedicated `🔮️oracles/` folder** under `🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/` for puzzle3d (that convention exists elsewhere in the repo, e.g. `✏️s/🔌️plugins/🔋️energy/🔮️oracles`, `✏️s/🔌️plugins/📕️norm/🔮️oracles`) — puzzle3d instead keeps the oracle function inline in the same `🧪️tests/🔬️unit/🦀️.rs` file, next to the fixture it validates.

**Recommended pattern for a new "fill-stream" test** (matching what's already there): add a `boundaryLaws`-style block (or a new fixture file) to `⏳️precompute/🪣️fill/🧫️fixtures/🔣️.json` describing the new streamed-candidate wire shape (byte caps, admitted/refused boundary pairs, an `"oracle"` tag), then add one Rust `#[test]` in `⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` that `include_str!`s it, round-trips through the real `FillBuildPreview`/successor type, and independently re-serializes via `serde_json` (or another already-linked crate) to catch byte-shape drift — plus, if the redesign changes invariants the P4E/envelope/preview-json policy tests already assert (fixed-capacity owners, checked arithmetic, admission/close-step shape, renderer identity gating), extend the corresponding `…Failures()` predicate in root `📜️script.ts` and its mutation self-test in the matching `🧪️tests/🔬️interactivity-puzzle-fill-*` file — do not add a 5th test dir without a reason; these three already jointly cover envelope/P4E/preview-json layers of fill.

## 3. Cargo crate `semio-s-artifact-puzzle-3d`

Path: `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml`, lib at `../../🦀️.rs` (i.e. the crate root is the plugin-shared `✏️s/🔌️plugins/🧩️puzzle/🦀️.rs`/`🗿️artifacts/🧊️3d/…` tree).

- Feature `component-app-assembly = ["semio-framework-plugin/component-guest", "dep:semio-framework-3d", "dep:semio-framework-async", "dep:semio-framework-ui-contract", "dep:semio-framework-ui-scene", "dep:wasm-bindgen"]`, **default = []**. The whole `editor` module tree (including all fill code) is `#[cfg(feature = "component-app-assembly")]`-gated, so a default-feature build compiles ~286/577 tests and **skips every `editor::puzzle3d` app test, fill included**.
- No `[[test]]` entries in `Cargo.toml` — all tests are inline `#[cfg(test)] mod tests` inside the `🦀️.rs` source files (ASCII-named-file concern from memory doesn't apply here).
- Local router `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/📜️script.ts` wraps the shared `runArtifactRustPackageMain(dir, "semio-s-artifact-puzzle-3d", { testFeatures: ["component-app-assembly"] })` and force-sets `RUST_MIN_STACK ??= "134217728"` (128 MiB) — needed because the app-driving tests overflow libtest's default 2 MiB thread stack.
- Exact commands (from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/📜️script.ts`):
  - **check** (does NOT auto-add the feature — pass it explicitly to reach fill code):
    ```
    cd ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust
    bun ./📜️script.ts check --features component-app-assembly
    # → cargo check --locked --manifest-path <…>/Cargo.toml --features component-app-assembly
    ```
    (equivalently `cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly` from repo root — confirmed fast-path in memory `project-puzzle3d-crate-fast-seeded-check`)
  - **test** (auto-adds `component-app-assembly` via `testFeatures`, runs through `cargo nextest` when available, budgeted, profile keyed by test level `fundamental|quick|long|exhaustive`):
    ```
    cd ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust
    bun ./📜️script.ts test
    # → RUST_MIN_STACK=134217728 cargo nextest run -p semio-s-artifact-puzzle-3d --features component-app-assembly --profile fundamental …
    ```
    Or via Nx: `bun nx run @semio-tech/puzzle-3d-rs:check -- --features component-app-assembly` / `bun nx run @semio-tech/puzzle-3d-rs:test`.
  - Fill-only test filter: append nextest string filters after `--`, e.g. `bun ./📜️script.ts test -- -E 'test(fill)'` (nextest expression syntax) or plain substring via cargo's own filter if not using nextest: `... test -- fill`.

- `.cargo/config.toml`: shared build dir under `.🧬semio/🦑️repo/⚡️cache/cargo/build`, shared target dir under `.../⚡️cache/cargo/target` (`build-dir`/`target-dir`), `fine-grain-locking = true` + `checksum-freshness = true` (per-compilation-unit locking, mtime-independent freshness — see memory `project-shared-cargo-build-dir-fine-grain-locking`). Also sets `RUST_MIN_STACK=67108864` (64 MiB) globally in `[env]` for native test threads (distinct, larger override from the puzzle-3d package script above — the package's own 128 MiB wins since `??=` only applies if unset, but config.toml's `[env]` **is** the environment at process start, so the package script's `??=` is a no-op here; effective value is whichever a caller sets first — check `env | grep RUST_MIN_STACK` if debugging stack-overflow test aborts).

## 4. Deploy chain to `http://127.0.0.1:6013/?plugin=puzzle3d`

Targets are **inferred**, not authored in any `📋️project.json` — synthesized by the Nx plugin `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs` (`nx.json` registers it against `**/📋️project.json`, `**/Cargo.toml`, …). Three inferring functions matter:

| function | targets |
|---|---|
| `componentTargets` (~line 837) | `component-dev`/`component-release`, `materialize-dev`/`materialize-release` on the puzzle Cargo package (`package.metadata.component.package`, `semio.role ∈ {plugin, extension}`) |
| `playgroundSessionTargets` (~line 866) | `session-<variant>` on `@semio-tech/plugin-registry`, one per Cargo-declared `[package.metadata.semio.playground]` variant (`puzzle3d` here) |
| `playgroundPreparationTargets` (~line 886) | `prepare\|activate\|serve\|dev-<variant>-react-<profile>` and `build-<variant>-react-release` on `@semio-tech/framework-os-dev` |

Full chain (dev profile):
```
@semio-tech/puzzle-plugin:component-dev        cargo rustc --target wasm32-wasip2 --profile wasm-dev
    ↓ dist/component-dev/semio_s_plugin_puzzle.wasm (+ .nx-artifact.json)
@semio-tech/puzzle-plugin:materialize-dev      jco transpile + descriptor probe + bridge
  ⊕ @semio-tech/framework-plugin-web:support-dev   preview2 shim vendor + shard worker
    ↓ 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/{🔣️.json,🛂️.descriptor.semio,🌉️bridge.js,…}
@semio-tech/plugin-registry:session-puzzle3d   dist/sessions/puzzle3d/🟦️session.ts
semio-framework-os-infinite:fonts              dist/fonts/🔤️guestslim-typst-fonts.bin
{surface,editor,flow-core,puzzle-plugin}:wasm  engine bindings/pkg
    ↓
@semio-tech/framework-os-dev:prepare-puzzle3d-react-dev    (verifies every input above)
    ↓
@semio-tech/framework-os-dev:activate-puzzle3d-react-dev   dist/runtime/dev/puzzle3d/activation/🔣️receipt.json
    ↓
@semio-tech/framework-os-dev:serve|dev-puzzle3d-react-dev  bunx vite, S_OS_PORT=6013
```

`.claude/launch.json` entries:
- `puzzle3d-react` → `bun nx run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev`, port 6013, env `SEMIO_RENDERER=react`, `S_OS_PORT=6013`.
- `puzzle3d-react-attach` → attaches to already-running `http://localhost:6013` (no command; expects the above already up).

**Rebuild + reserve without restarting vite** (host TS stays vite-live; only the wasm needs re-materializing), per ticket note `project-puzzle3d-battery-method` and `📓️2026-09-09-wave-R-react-boot-pipeline.md`:
```
bun nx run @semio-tech/puzzle-plugin:component-release --skip-nx-cache   # 4–7 min warm cargo build
bun nx run @semio-tech/framework-plugin-web:support-release              # (or -dev, matching profile)
bun nx run @semio-tech/puzzle-plugin:materialize-release --manifest ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml
bun nx run @semio-tech/framework-os-dev:prepare-puzzle3d-react-release
bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-react-release
```
(swap `-release`→`-dev` throughout for a faster, unoptimized cycle — dev profile is what `puzzle3d-react`/port 6013 in `.claude/launch.json` actually runs). "Compile-only first" = run just `component-<profile>` to catch build errors before paying for materialize/activate; only materialize when no probe lane is mid-measurement (activation swaps the served files out from under a live page).

**Verify served wasm == disk:** compare sha256 of the served core wasm (fetched via the dev server's `/@fs/<absolute path>` passthrough, e.g. `curl -s http://127.0.0.1:6013/@fs/<abs path to core.wasm> | shasum -a 256`) against `shasum -a 256` on the on-disk artifact under `…/🔌️plugin/📦️packages/🟦️typescript/dist/dev/🔌️plugin-modules/🧩️puzzle/…` (or `dist/release/…`). A mismatch means the browser has a stale in-memory/vite-cached module and needs a hard reload, not a rebuild.

## 5. Browser probe — `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/PUZZLE-3D-END-TO-END/🔍️browser-probe.ts` (4,351 lines, Playwright, headless)

CLI: `bun 🔍️browser-probe.ts [--battery] [--only=step,step] [--port=<n>] [--reload-between-groups] [--example=<name>] [--settle=<seconds>] [--paint-limit=<n>]`, plus per-section flags in `STEP_FLAGS` (`--fill` among them). `--only=a,b` is the sole gate when present; otherwise a step's own flag OR `--battery` decides (`add(name, section, group, gate, run)`, line ~309).

Fill-related registered steps (all gated by `process.argv.includes("--fill") || battery`, group `"mutate"`, section `"§12"`), listed in `STEP_TRAILS_ITS_GROUP` (line 329) so they always run **last** within their group — arming Fill spawns a build job that adds ~160 `puzzle3d.brush.*` objects to the document, which would pollute every later verdict in the same battery run:
- `tool-category` — opens the Tool category footer.
- `fill-tab` — clicks `button[id="tool.fill"]`, waits for arm (`panel.text` containing `"Cancel fill"` or `treeItems >= 4`).
- `fill-abort-engagement` — Escape while armed; asserts no residual fill state.
- `fill-wait-ready` — polls `fillCountFromState(...)` until the build job produces a nonzero count.
- `fill-apply-max` — drives the count slider to its rail max via keyboard `End` (or `--paint-limit=<n>` to stop short — measured: `End`→1000 leaves 160 placed objects on the Concrete Forest fixture, `--paint-limit=12`→127).
- `fill-history` — asserts an undo-history entry exists after apply (`hist.entryCount > 0`).

Reads state via `fillState()` (queries `[data-slot="toggle-group-item"]`, sliders, and the fill panel's text/tree), `fillConsoleCensus()` (regexes the buffered console for `/fill_build_tick|fillBuildTick/` tick counts and `[DEBUG] setActiveTool|reconcileToolTab|fillBuildTick gate` arm lines), and `dumpInstances()`/`data-instances-json` (the world-surface DOM attribute the host publishes per instance, plus `data-plugin-recovery` for guest-alive checks at group boundaries).

**Adding a step for "fill streams candidates with danger/highlight and locks objects incrementally":** register a new step (or steps) inside the fill block, e.g. `fill-stream-candidates`, gated the same way (`process.argv.includes("--fill") || battery`) and appended to `STEP_TRAILS_ITS_GROUP` if it also needs a document with the fill build already run. Read the new candidate/danger/highlight/lock state the same way existing steps do — either a new `data-*` attribute on the world surface host (cheapest, matches `data-instances-json`'s existing convention) or new console markers matched by a `fillConsoleCensus()`-style regex if the signal is only observable as a debug log during the redesign's early implementation. Assert progress via polling (`fillState`/a new state reader) with a `waitedMs` budget, matching every other mutate-group step's pattern, and call `verdict(name, ok, note)` (line 350) to emit the pass/fail into `probe-<stamp>.ndjson`.

## 6. Live processes at audit time (2026-09-13, snapshot only — nothing killed)

- **puzzle3d dev server chain, up since 12:16 PM:**
  - PID 8070 — `node nx.js watch --all … -- bun nx run @semio-tech/framework-os-dev:activate-puzzle3d-react-dev --output-style=stream`
  - PID 8072 — `node nx.js run @semio-tech/framework-os-dev:dev-puzzle3d-react-dev`
  - PID 13620 — `bun ./📜️script.ts serve puzzle3d react dev`
  - PID 13631 — `bun`, listening on `127.0.0.1:6013` (LISTEN, confirmed via `lsof`)
- **A fresh activation in flight since 1:22 PM** (watch-triggered re-activate, likely from a source change under puzzle3d):
  - PID 60189 — `node nx.js run @semio-tech/framework-os-dev:activate-puzzle3d-react-dev --output-style=stream`
  - PID 60184 / 60181 — the `bun`/bootstrap wrapper processes under it
- **Unrelated peer activation also running** (generation3d, different ticket — `26/09/09/PROCEDURAL-3D-END-TO-END`): PID 36455/36457/36458/36459, `activate-generation3d-react-dev`, writing to that ticket's own `🗑️generated/fix-forward-set-contributions-hang/restage-1.txt`.
- **Cargo test/build activity** (none are `browser-probe`; no probe currently running on :6013):
  - PID 49979, 61046 — `cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- --test-threads=…`
  - PID 58658 — `cargo test -p semio-s-plugin-stdio --message-format short`
  - PID 59759 — `cargo test -p semio-framework-ui --features testkit --lib -- control_commit`
  - PID 61051 — `rustc --crate-name semio_framework_plugin …` (a shared dependency of puzzle3d's `component-app-assembly` feature — an implementation wave touching puzzle3d fill will likely rebuild against whatever this lands as)
  - PID 61134/61137/61138 — wrapper shells around another `cargo test -p semio-s-artifact-procedural-generation3d …` run
- No `browser-probe` process is currently running, so port 6013 is free for a probe run right now (per the port-gate convention, confirm with `pgrep -f 'bun .*browser-pro[b]e'` immediately before starting one).

## Key takeaways for the implementation wave

1. The four "tests" under `🧪️tests/` are **static source-policy checks**, not functional tests of fill's runtime behavior — a redesigned streaming fill will need its invariants (fixed-capacity owners, checked arithmetic, admission shape, byte caps) re-encoded into the matching `…Failures()` predicate in root `📜️script.ts`, or the checks will simply not see the new code path.
2. Functional/behavioral verification of fill lives in `⏳️precompute/🪣️fill/🧪️tests/🔬️unit/🦀️.rs` (Rust `#[test]`, real `cargo nextest`, `--features component-app-assembly` required) and in the Playwright battery (`🔍️browser-probe.ts --fill`).
3. `cargo check`/`cargo test` against this crate silently skip all fill code unless `--features component-app-assembly` is passed explicitly (check) or is picked up automatically (test, via the package's own `📜️script.ts`).
4. The dev server on :6013 is live right now and mid-reactivation (watch-triggered) — a wasm rebuild does not require restarting vite, only re-running `component-<profile>` → `support-<profile>` → `materialize-<profile>` → `prepare` → `activate`; verify with a served-vs-disk sha256 diff before trusting the browser.
