# Lane S5 — `✏️s/**`, `🌎️hub/**`, `♻️mit-bestand/**` (2026-09-11)

Scope: route every cache-location reference in my three trees through the shared cache root
(`.🧬semio/🦑️repo/⚡️cache/`) and the Wave A resolver (`cargoTargetDirectory`/`cargoBuildDirectory`,
`repoCacheDirectory`) — no private `CARGO_TARGET_DIR` overrides, no sccache remnants.

## Task 1 — stdio probes/bridges, sequence/math/fem generators

All of these are standalone `[workspace]` crates (own `Cargo.toml`, no repo-workspace membership) run
via `cargo run`/`cargo build` with `cwd` inside the crate. Since none of them declare a nested
`.cargo/config.toml`, the repo root's `.cargo/config.toml` (`build.target-dir`/`build.build-dir`)
already resolves transitively — confirmed empty `find ✏️s 🌎️hub ♻️mit-bestand -iname config.toml -path
'*.cargo*'`. So the fix everywhere was: **stop overriding `CARGO_TARGET_DIR`**, and where a script
needs to know the binary's landing path (to spawn it after `cargo build`), read it from the resolver
(`cargoTargetDirectory(getWorkspaceRoot())`) instead of a private `.../target` subdirectory.

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🔬️probes/📜️script.ts` —
  removed `cargoTargetDir()` (env-or-`SEMIO_AGENT_CACHE`-or-local-`target` fallback) and the
  `CARGO_TARGET_DIR` env override on the `cargo run` call.
- `…/📐️cad/🔬️probes/📜️script.ts` — same removal.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🏭️bridge/📜️script.ts` —
  same removal; dropped the now-unused `join`/`node:path` import.
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` —
  `runEngine` no longer passes `--target-dir`; the post-build binary path is
  `join(cargoTargetDirectory(getWorkspaceRoot()), "release", "generate")`.
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/🏭️generator/📜️script.ts` —
  same pattern (`TARGET` constant now `cargoTargetDirectory(getWorkspaceRoot())`, no `--target-dir` flag).
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/🏭️generator/📜️script.ts` and the `◻️2d` sibling — the
  `carrier`/`carrier-manifests` JSON-engine build no longer sets `CARGO_TARGET_DIR`; `cargoTargetDir`
  is now `cargoTargetDirectory(getWorkspaceRoot())`.

## Task 2 — `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (17k lines)

- `hubBinaryPath` (~861) and `nativeWgpuExecutable` (~1183) — replaced the
  `CARGO_TARGET_DIR ?? join(repoRoot,"target")` fallback with `cargoTargetDirectory(repoRoot)`.
- Added `import { cargoTargetDirectory } from ".../⚡️caching/🦀️cargo/🟦️.ts"` and
  `import { repoCacheDirectory } from ".../⚡️caching/🟦️.ts"`.
- Removed sccache remnants: `RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1"` from the native-gate `hubEnv`
  (~12973) and the `["--config", 'build.rustc-wrapper=""']` cargo arg on its build call (~12979,
  now just `cargo build --manifest-path Cargo.toml …`).
- Fixed a fence test that had gone **stale and would now fail**: `trusted-stdio-gis-bundle-check`'s
  source-boundary proof (~12854-12876) slices `freshStage(` out of the plugin-describe script and
  asserted it still contains `RUSTC_WRAPPER: ""`. Lane S4 already removed that literal from
  `produceFreshComponentV1`'s `devToolingEnv({ CARGO_TARGET_DIR: targetRoot, CARGO_INCREMENTAL: "0" })`
  call (confirmed by re-reading that file), so the assertion was live-broken; removed the
  `!producer.includes('RUSTC_WRAPPER: ""') ||` clause.
- `process.env.PLAYWRIGHT_BROWSERS_PATH ??= join(repoRoot, "node_modules", ".cache", "ms-playwright")`
  (two call sites, ~13777/13990) → `repoCacheDirectory(repoRoot, "tools", "ms-playwright")`, matching
  the plan's `tools/ms-playwright/` cache-root layout.

### Left untouched — deliberate ticket/freshness isolation, not "private dir for convenience"

Investigated and traced every remaining `CARGO_TARGET_DIR`/private-target hit in this file
(~5144-5273 `headlessStdio*`, ~10233/10823 the `--features test-support` native-law helpers,
~12948-12973 the native-gate `hubTarget`, ~6685/produceFreshComponentV1's fresh-component target).
Every one of them builds a **feature-variant or freshness-proof binary** (e.g. `os-hub --features
test-support`, or a component built into a caller-supplied *empty* root that
`requireEmptyFreshRoot`/`proveHeadlessStdioLaunchIsolation` explicitly refuse to let overlap the
ambient shared target) — landing those in the one shared `target-dir` would let two differently
featured builds of the identical binary name race for the same uplifted path, which is exactly the
race these tests exist to catch. This is orthogonal to Wave A's "no private dirs for the ordinary
build/dev/test loop" — I left the isolation requirement and its private nested target dirs in place.
`.vscode/🧩️launch.seed.jsonc` (lane S6's file) still supplies these `CARGO_TARGET_DIR`s at the moment;
if S6 removes them, `headlessStdioCommandRoots()`/`proveHeadlessStdioLaunchIsolation()` will start
throwing — flagging this as a cross-lane follow-up rather than guessing at S6's still-in-flight shape.

## Task 3 — standalone workspaces / stray target dirs

- `✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust` — Cargo.toml has **no** `[workspace]` (it's
  `version.workspace = true`, a real member of the root workspace) and its `📜️script.ts` has zero
  target-dir logic (`runCargoTestBudgeted`/`describePluginComponent`, both library-resolver-backed
  already). The on-disk `target/` there is a stale pre-Wave-A artifact; no code change needed or made
  — left the directory alone per the "never delete target dirs" rule.
- `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/📦️packages/🦀️rust` — standalone `[workspace]` crate, no owning
  script anywhere in my lane sets `CARGO_TARGET_DIR` for it (`rg`-confirmed). Its local `target/` is
  the same kind of stale pre-existing artifact; nothing to change.
- `target-gen3d-opt` (repo root) — `rg -l 'target-gen3d-opt|target-gen3d'` across the whole repo (not
  just my lane) finds it **only** in ticket-note prose (manual `CARGO_TARGET_DIR=target-gen3d …`
  commands a prior session ran by hand) and in the `🎯️cargo-target-discovery-skip` fixture (S7's
  file, lists `target-gen3d` not `-opt`). No committed script creates this directory. It is a stale
  1.9 GB manual artifact from Sep 8; left untouched (never delete target dirs).
- `.claude/launch.json` `target-engines` users inside `♻️mit-bestand`: no `📜️script.ts` under
  `♻️mit-bestand` references `target-engines` (`rg`-confirmed) — the launch file's own entries are
  lane S6's.
- Rust test flagged for a target-dir reference
  (`✏️s/🔌️plugins/🌀️procedural/…/✏️edit/🧪️tests/🕸️flow/🔬️unit/🦀️.rs`) — re-read immediately before
  any edit, twice, at different points in this session (a live peer is actively rewriting
  `🌀️procedural/**`, confirmed by the huge concurrent diff under `🧩️assembly/**`). No target-dir/cache
  string is present in the file as it stands now; either the peer already removed it or the original
  flag was stale. No edit made, nothing reverted.

## Task 4 — full `rg` sweep for the pattern list

Beyond the files named in the brief, the sweep surfaced a **second, larger class of bug**: many
`🏭️generator`/`🏗️generator` scripts under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/**` (and `🖍️draw`,
`🗒️note`) build a sibling `🦀️engine`/`🦀️lopdf-engine` crate and then hardcode
`join(engine, "target", "release", "<binary>")` as the binary's landing path, with **no**
`CARGO_TARGET_DIR` override at all — they always relied on cargo's bare default (`<crate>/target`).
Once `.cargo/config.toml`'s `build.target-dir` became the shared cache path, that hardcoded relative
path silently stopped matching where cargo actually put the binary, i.e. every one of these was about
to fail with "binary not found" on the next run. Fixed all of them to
`join(cargoTargetDirectory(getWorkspaceRoot()), "release", "<binary>")`:

`🖍️draw/🗿️artifacts/🖍️drawing/…/🏭️generator`, `➗️mathematical/…/🔬️probes` (equation probes, distinct
from the generator fixed under Task 1), `🗄️stdio/🗿️artifacts/🧿️semio/…/🧊️brep/🏭️bridge`,
`…/🔺️mesh/🏭️bridge`, `🗒️note/…/🔬️probes`, `🗒️note/…/🏭️generator`, `🗄️stdio/…/📐️cad/🏗️generator`,
`🗄️stdio/…/🗽️obj/…/🏭️generator` (two engines: legacy + tobj reader), `☁️las/…/🏭️generator`,
`🎞️gif/…/7️⃣87a/…/🏭️generator`, `🎞️gif/…/9️⃣89a/…/🏭️generator` (two engines: main + extension reader),
`🖋️dxf/…/r12/…/🏭️generator` + its sibling `🔬️probes`, `🧾️json/…/rfc8259/…/🏭️generator`,
`🖼️tiff/…/6.0/…/🏭️generator`, `🧿️semio/…/📑️document/🏗️generator`, `🧿️semio/…/🖊️drawing/🏗️generator`,
and all 9 of `📖️pdf`'s `4️⃣1.4`/`7️⃣1.7` subset generators (`🗄️a`, `🧱️base`, `🖨️x`, `♿️ua`, `🧾️vt`,
`⚕️h`, `📐️e` — the `7️⃣1.7/🧱️base` one has two engines, asset + mutation).

Also in `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` (`CatalogRootScript`): removed the now
dead `RUSTC_WRAPPER: "", SCCACHE_DISABLE: "1"` from its `devToolingEnv({...})` call and the
`["--config", 'build.rustc-wrapper=""']` cargo args on two `runControlled("cargo", …)` calls (the wasm
component build and the `semio-framework-plugin-describe` build). Its private per-process
`cargoTarget = join(buildRoot, ".stdio-cargo-target-${pid}")` was **left in place** — `requireEmptyFreshRoot`
explicitly refuses a build root that overlaps `resolve(repoRoot, "target")` (the same freshness-proof
pattern as hub's `produceFreshComponentV1`), so this is not an ordinary private-dir convenience.

`♻️mit-bestand`:
- `🧺️demonstrator/⚙️vite.config.ts`: `cacheDir: path.join(repoRoot, "node_modules/.vite-mit-bestand-demonstrator")`
  → `repoCacheDirectory(repoRoot, "vite", "mit-bestand-demonstrator")` (exactly the example named in
  the brief).
- `🧺️demonstrator/🔨️modules/🧪️e2e/📜️script.ts`: `PLAYWRIGHT_BROWSERS_PATH: join(this.repoRoot,
  "node_modules/.cache/ms-playwright")` → `repoCacheDirectory(this.repoRoot, "tools", "ms-playwright")`.

## Unplanned fix found while verifying

Running the sequence generator end-to-end (`bun 📜️script.ts generate`) failed with `can't find bin
'generate' at path '…/src/â¨ï¸generate.rs'` — `✏️s/🔌️plugins/🎬️sequence/…/🏭️generator/🧾️json-engine/Cargo.toml`
had its two `[[bin]] path` strings **double-UTF-8-encoded** on disk (confirmed via raw byte dump), even
though the actual source files (`✨️generate.rs`, `📖️reader.rs`) were named correctly — unrelated to my
change, but it blocked verifying the fix, so I repaired the two path strings (standard
`bytes.decode('utf-8').encode('latin-1').decode('utf-8')` un-corruption) and reran; the generator now
succeeds and the regenerated fixtures matched the committed ones byte-for-byte (no fixture diff).
Repo-wide scan (`✏️s`/`🌎️hub`/`♻️mit-bestand`, mojibake marker heuristic) found no other Cargo.toml
with this corruption.

## Verification

- `bun build --target=bun --no-bundle <file>` — ran on every edited `.ts`/`.tsx` file (40 files);
  all exit 0, no output on stdout beyond the transpiled bundle.
- `bun 📜️script.ts list-mutations s.stdio.step ap214 cc6` (step ap214 cc6 bridge) — real output:
  4 mutations (`set-snapshot`, `set-file-schema`, `set-product-identity`, `set-shape-representation`);
  confirmed the built binary landed at
  `.🧬semio/🦑️repo/⚡️cache/cargo/target/debug/semio-step-ap214-cc6-bridge`.
- `bun 📜️script.ts gate-inputs --out <scratch>` (stdio semio drawing probes) — real output: `status:
  "ok"`, 8 files written (svg/dxf gate corpus).
- `bun 📜️script.ts generate --out <scratch>` (las header generator, an `ENGINE_BIN`-pattern fix) —
  real output: engine compiled fresh, `wrote 495 bytes to …/☁️survey-strip.las`, sha256 printed.
- `bun 📜️script.ts generate` (sequence generator, both csv-engine and json-engine) — real output:
  `[generate] wrote 4 sequence JSON carrier pair(s)`; `git status` after the run shows **no** diff
  under `🧫️fixtures/` (regenerated bytes match committed ones exactly).
- `cargo check -p <crate>` was not needed — every fixed file is TypeScript; no Rust source was edited
  except the Cargo.toml byte-repair (not a `cargo check`-relevant change, and the two `cargo
  build`/`cargo run` verification runs above exercised it directly).
- `bun build` on `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (17k lines) — exit 0 after both the resolver
  edits and the sccache/fence-test cleanup.

## Left undone / follow-ups

- The `.vscode/🧩️launch.seed.jsonc`-fed headless-stdio isolation contract in
  `🌎️hub/📦️packages/🦀️rust/📜️script.ts` (~5144-5273) will break if lane S6 removes the private
  `CARGO_TARGET_DIR`s from those three launch entries without a corresponding change here — flagged
  above, not fixed, since guessing at S6's still-in-flight `.vscode` shape risked a worse
  half-migrated state.
- `target-gen3d-opt` (1.9 GB, root of repo) is stale and uncreated by any current script; left for the
  cache pruner (S7) or a manual `rm` outside this ticket's "never delete target dirs" rule.
