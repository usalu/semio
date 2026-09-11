# Lane S6 — Launch/Devcontainer/CI (2026-09-11)

Scope: `.vscode/**`, `.claude/launch.json`, `.devcontainer/**`, `.github/**`. Goal: stop every private/forked
cache location in these files so all builds share the single `.🧬semio/🦑️repo/⚡️cache/` root that Wave A
(`.cargo/config.toml` `build.target-dir`/`build.build-dir` + fine-grain locking, `nx.json` `cacheDirectory`)
already set up. No legacy/compat entries kept.

## Files changed

| file | insertions | deletions | what |
| --- | ---: | ---: | --- |
| `.vscode/launch.json` | 27 | 196 | removed 103 `CARGO_TARGET_DIR`, 43 `RUSTC_WRAPPER`, 23 `RUSTC_WORKSPACE_WRAPPER` JSON env entries (169 lines) + 1 inline `RUSTC_WRAPPER=''` prefix on a `command` string; fixed 26 now-trailing commas where a removed key was the last property in its `env` object |
| `.vscode/🧩️launch.seed.jsonc` | 27 | 196 | identical fix — this is the schema-owned launch seed that `📜️script.ts` (`INTERACTIVITY_ALL_APP_LAUNCH_SEED_FILE`) and multiple `📚️library/🧪️tests/**` fixtures read alongside `.vscode/launch.json` and expect to stay consistent with it; same 103/43/23 counts, same inline command fix |
| `.claude/launch.json` | 0 | 2 | removed `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-engines` from `mit-bestand-demonstrator-fast` and `mit-bestand-demonstrator-noengine` `env`-style `runtimeArgs` arrays (was also a non-cross-platform absolute host path) |
| `.devcontainer/devcontainer.json` | 3 | 3 | `containerEnv.PLAYWRIGHT_BROWSERS_PATH` and `remoteEnv.PLAYWRIGHT_BROWSERS_PATH` → `${containerWorkspaceFolder}/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`; replaced the standalone `ms-playwright` volume mount (`target=/home/vscode/.cache/ms-playwright`) with one named volume mounted at `${containerWorkspaceFolder}/.🧬semio/🦑️repo/⚡️cache` (overlays that subtree of the bind-mounted workspace so cargo target/build dirs, Nx cache, Playwright, Vite, Go — everything under the cache root — get native volume I/O and survive container rebuilds as one unit, matching the existing `node_modules` volume-overlay pattern) |
| `.devcontainer/Dockerfile` | 1 | 2 | dropped the image-build `mkdir -p … /home/vscode/.cache/ms-playwright` line (that path is no longer used; the directory now comes from the workspace/cache-root mount at container start, not the image layer) |
| `.devcontainer/README.md` | 5 | 4 | updated the `devcontainer.json` doc summary, the "Playwright Browser Cache" section, and the `📌️Requirements` line to describe the shared cache-root path instead of the old `node_modules`-volume story |
| `.github/**` | 0 | 0 | no workflow files exist yet (`.github/workflows/` is empty — `find .github/workflows -type f` returns nothing) and no other `.github/**` file (`dependabot.yml`, `agents/*.md`, `hooks/*.json`) referenced sccache/cache paths — nothing to change |

No other `.vscode/**` file (`settings.json`, `mcp.json`, `extensions.json` — no `tasks.json` exists) contained any
of the target patterns beyond the benign `ms-playwright.playwright` VS Code extension id, which is unrelated to
the cache path and was left untouched.

## Per-var removal counts (`.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`, each)

- `CARGO_TARGET_DIR`: 103 (all were ticket-scoped `🗑️generated/...-target` paths under the now-closed
  `COMPLETE-SEMIO-END-TO-END` ticket, per Wave A these must not exist — one shared build-dir replaces them)
- `RUSTC_WRAPPER`: 43 JSON `"RUSTC_WRAPPER": ""` entries + 1 inline `RUSTC_WRAPPER='' cargo check …` command prefix
- `RUSTC_WORKSPACE_WRAPPER`: 23
- `SCCACHE_*`: 0 occurrences found in any owned file
- `CARGO_INCREMENTAL`: 0 occurrences found in any owned file
- `CARGO_BUILD_JOBS`: left untouched everywhere (not cache-related, per instructions)

## Method

`.vscode/launch.json` (28,676 lines) and `.vscode/🧩️launch.seed.jsonc` (15,642 lines) are hand-editing-hostile at
this size, so a small Node script (`🧹️lane-s6-strip-launch-cache-env.mjs`, kept in this ticket folder) did the
removal:

1. Line-match `^\s*"(CARGO_TARGET_DIR|RUSTC_WRAPPER|RUSTC_WORKSPACE_WRAPPER)":\s*"[^"]*"\s*,?\s*$` and drop the line.
2. For every dropped line that had **no** trailing comma (i.e. it was the last property in its `env` object),
   strip the trailing comma from the nearest surviving preceding line so the object stays valid JSON.
3. Everything else (formatting, key order, comments, unrelated entries) is untouched byte-for-byte.

The one inline `"command": "RUSTC_WRAPPER='' cargo check …"` occurrence in each file (not a JSON `env` key) was
fixed with a targeted `Edit`. `.claude/launch.json`'s two `env`-array (`env NAME=value bun nx run …`) entries were
also fixed with targeted `Edit`s since they are a different shape (array of `"KEY=value"` strings, not a JSON
object).

## Verification (all commands run from `/Users/ueli/Documents/semio`, read-only except the two `node -e` JSON
checks which only parse)

1. JSON validity (JSONC comments stripped first, since these files use `//`-line comments):
   ```
   node -e '...JSON.parse(stripped)...' .vscode/launch.json            → VALID
   node -e '...JSON.parse(stripped)...' .vscode/🧩️launch.seed.jsonc    → VALID
   node -e '...JSON.parse(stripped)...' .devcontainer/devcontainer.json → VALID
   node -e 'JSON.parse(...)' .claude/launch.json                        → VALID
   ```
2. No empty `"env": {}` blocks were produced by the comma-fix pass (checked programmatically against both
   `.vscode` files before applying — none found).
3. Final `rg` sweep for `sccache|SCCACHE|RUSTC_WRAPPER|CARGO_TARGET_DIR|target-engines|target-demonstrator|/target\b|\.nx/cache|ms-playwright|\.vite`
   over `.vscode/`, `.claude/launch.json`, `.devcontainer/`, `.github/` — the only remaining hits are the new,
   correct cache-root paths (`devcontainer.json` `PLAYWRIGHT_BROWSERS_PATH` ×2, `README.md` prose ×2) and the
   unrelated `ms-playwright.playwright` VS Code extension id (×2, in `extensions.json` and
   `devcontainer.json`'s extensions list).
4. `git diff --numstat` (read-only) sanity:
   ```
   0  2  .claude/launch.json
   1  2  .devcontainer/Dockerfile
   5  4  .devcontainer/README.md
   3  3  .devcontainer/devcontainer.json
   27 196 .vscode/launch.json
   27 196 .vscode/🧩️launch.seed.jsonc
   ```
   `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` have identical insertion/deletion counts — expected,
   since the seed file mirrors the same ticket-scoped launch identities and had the exact same 103/43/23 hits
   before this change.
5. Spot-checked diff output by eye (`diff` on before/after copies before touching the real files, then
   `git diff` after) to confirm every multi-key `env` block that lost its last property had its new-last
   property's trailing comma correctly stripped, and no unrelated line moved.

## Left undone / out of scope for this lane

- `.github/workflows/` is currently empty — there is no CI workflow to add an `actions/cache` step to. Nothing
  was fabricated; when a workflow is added later it should cache the `nx`, `cargo/build`, `cargo/target`
  subdirectories of `.🧬semio/🦑️repo/⚡️cache/` keyed on lockfiles + toolchain, per the plan.
- Non-owned files that still reference the old `node_modules/.cache/ms-playwright` / per-agent target-dir
  patterns (`🌎️hub/📦️packages/🦀️rust/📜️script.ts`, `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📜️script.ts`,
  `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1`) are owned by other lanes (S2/S5) and
  were left untouched.
- Did not run `bun nx run …` or boot a dev server to confirm runtime behavior — this lane's scope is static
  config/launch-definition correctness, not runtime verification; JSON validity + `rg` sweep + diff inspection
  is the applicable evidence for JSON/JSONC config files with no build step of their own.
