# Round 2 Slice C-1b: Editor, Container, CI And Root Command Canon

Date: 2026-10-08. Ticket `2026/09/23/DASHBOARD-LAUNCH-COCKPIT`. Windows host. Facts below were run unless marked UNVERIFIED.

## 0. Round 2b: `🚚️migration.json` deleted (newest)

Deleted `🚚️migration.json` (no code reads `unmanagedTests`; re-verified by `git grep` over TS/Go/Rust/JSON). Cascade on live-bound files:
- `🧪️test/README.md` step 12 now reads "Delete the replaced legacy test in the same owner change. Never leave two test hierarchies alive."
- `📚️library/🧪️tests/🗺️testing-readme-coordinates/🟦️.ts`: removed the `repository-root` + `🚚️migration.json` assertion.
- `📚️library/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json`: removed the `repository-migration-baseline` coordinate row and the rejected-markdown entry `📇️registry/🚚️migration.json`. This fixture is read by the test only; no hash pins it.
- Tests run: `testing-readme-coordinates` 14 pass, 0 fail (80 expects). No `git grep` hit outside the frozen set below.

Left untouched on purpose (frozen sealed history, needs an owner decision to change): `📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/{📥️reviewed-source/📝️.md:84, 🎯️reviewed-expectations/🔣️.json:73-76,143}`. Why: (1) they are a self-contained copy of an older README (12,511 bytes, sha256 `20fa38e0...`; the live README is already 15,087 bytes) that the manifest calls "newly-authored-copy-of-current-reviewed-bytes"; the live README is not bound to it. (2) Their bytes are hash-pinned in at least six places that must change together: `👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json` (preimage sha/size of both inputs), its parent `🔣️.json` (`fixtureAuthoritySha256`), `📚️library/🔣️taxonomy.json` `currentSourceRevisions` (`currentPreimage`, `expectationsSha256`), `🔖️readme-current-source-revision` and `🟢️readme-current-source-activation` fixtures/schemas, and the `copies` list. (3) No writer or re-seal command exists for them (`git grep` for `expectationsSha256|currentSourceRevisions|reviewed-readme-fixture-inputs-v1` finds only readers/validators); re-sealing would be hand-editing hashes, which the coordinator ruled out. They legitimately describe the README as it was; rewriting history would be an owner decision (either re-author the frozen copy as a new revision, or retire the whole reviewed-fixture mechanism).
Baseline observations (unchanged by me): `readme-current-source-revision` and `readme-current-source-activation` already fail on Windows ("preimage drift": the manifest pins file mode 420, Windows reports 438); `readme-move-source-authority` has one unrelated TypeScript-diagnostic mismatch.

## 1. Root `package.json`

- `scripts` is now exactly `nx`, `setup`, `dashboard`, `dashboard:install` (was 134). `package.json:29-34`.
- Reference sweep (`git grep`, excluding `.🧬semio/` and `.cursor/plans/`, patterns `bun|npm|pnpm|yarn [run] <name>`, quoted names, and every reader of `.scripts`): the only code/config/test references were
  - `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts:4` comment `bun run test:storybook` -> `bun nx run workspace:test-storybook` (edited).
  - `📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🟦️.ts:793-799` asserted `workspacePackage.scripts["test:repo-lib:package-body-policy"]`; that assertion is removed (L-1 had already removed the launch loop). Test `-t 'focused policy command'` passes.
  - Everything else matching (`Dockerfile`, `npm run test` classifier strings in Go/Rust, `bun run dev` in stray-process fixtures, commonmark/ooxml fixtures) is unrelated text.
- Dependency evidence: `dependencyJsParity` uses root scripts only as last-resort evidence. Only `nx` ever matched a script; `nx` is still matched by `setup`/`dashboard`. No dependency loses evidence (script `scriptdeps.ts` in scratch).
- For L-2 (docs, not edited by me): `🎛️dashboard/README.md:90-91` (`bun run dashboard:preferences ...` -> `semio preferences`), `:102` (`bun run dashboard:start|status|stop` -> `semio daemon start|status|stop`), `:9` ("or the Dashboard launch entry"). Root `README.md` only uses `npm run setup` / `bun run dashboard`, both kept. Fixture-only hits (`✏️s/.../commonmark/.../📖️readme.md:716`) are frozen document content, leave.

### Variant scripts versus the playground catalog

Catalog: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🚀️playgrounds.json` (155 variants, gitignored, generated). Registry ids are `playground:<variant>` with `example`/`renderer`/`user-slot` parameters (`🎮️registry/🦀️.rs:932,1196`).

- Direct catalog equivalents (all bare `dev:*` and `dev:fem|block|trinity|procedural|process|puzzle` scripts): `2d/3d/5d` -> `puzzle2d/puzzle3d/puzzle5d` (aliases), `aggregator`, `fem2d|fem3d`, `block2d|3d|5d`, `cad`, `flow`, `layout`, `lowpoly`, `remodel`, `procedural 3d` -> `generation3d`, `process3d`, `sourcing`, `shooting`, `forms`, `playbook`, `raster`, `draw`, `note`, `s`, `vcs`, `writer`, `trinity-jack|rewriting`.
- `dev:puzzle:5d:capsule-dream` had the same body as `dev:puzzle:5d` (never passed `capsule-dream`). The catalog has the example: `playground:puzzle5d` with `example=🌙️capsule-dream`. Nothing lost.
- `fixture <name>` scripts were already dead: the `dev` route resolves only the leading variant segments (`resolveFrameworkOsPlaygroundPlugin`) and has no `fixture` verb (the verb is `example`). Equivalents: `dev:puzzle:3d:concrete-forest` -> `playground:puzzle3d example=🌲️concrete-forest`; `dev:puzzle:5d:concrete-forest` -> `playground:puzzle5d example=🌲️concrete-forest`; `dev:procedural:3d:hexagonal-column` -> `playground:generation3d example=🍄️hexagonal-mushroom-column` (renamed example).
- No catalog equivalent (record only, nothing to port): `dev:wfc` (no variant `wfc`; catalog has `wfc2d`, `wfc3d`, `bitmap`, `grid2d`, `grid3d`), `dev:os:multi` (`script.ts` documents that `dev multi` never resolved; `dev s` is the all-plugins hub), `dev:cad:concrete-forest` (cad examples: demo, demo-session only), `dev:shooting:base-icon` (shooting examples: hexagonal-cut-concrete-forest-left, demo, demo-session), and the five `build:*:fixture` variants (`framework-os-dev:build -- <variant> fixture <name>`; the Nx targets `build-<variant>-react-release` still exist and are dashboard-discoverable, the `fixture` argument forms are dead).
- `wasm:flow-core` (bare `nx`, no `bun`) is gone with the rest; the Nx target `semio-framework-os-flow-core:wasm` stays discoverable.

## 2. Editor settings derivation (one source, one writer, one check)

Canonical: `.vscode/settings.json`, `.vscode/extensions.json` (native hosts). Derived: `.devcontainer/devcontainer.json` `customizations.vscode.settings` and `extensions`.

- Declared overlay (new, plain JSON): `.devcontainer/editor-overlay.json` = omit `terminal.integrated.env.windows|osx` and the Dev Containers extension, set the Linux `python.defaultInterpreterPath`, add `anthropic.claude-code`. `${workspaceFolder}` becomes `${containerWorkspaceFolder}` automatically.
- Module (new owner): `📚️library/⚡️caching/📦️artifacts/📝️derived-config/🟦️.ts` (derive, JSONC in-place property replace that keeps comments, check, write). Tests `🧪️tests/📝️derived-config/🟦️.ts`; language-agnostic fixture `🧫️fixtures/📝️derived-config/🔣️.json` (editor cases, JSONC cases, nextest cases, dependabot cases). Oracles: `fast-json-patch` (RFC 6902) + `lodash` for the editor block, `jsonc-parser` for the JSONC rewrite, `@iarna/toml` for nextest, `yaml` for dependabot. The test also round-trips the writer on a temp copy with a longer tampered devcontainer and a shorter tampered nextest scope, asserts idempotence and byte equality with the checked-in file.
- Command: `bun ./📜️script.ts generate config [--check]` (`📜️script.ts` `GenerateScript.generateConfig`); Nx targets `generate-config`, `generate-config-check` in root `📋️project.json`.
- Gate: `policyDerivedConfigBreaches` (region `PolicyRuleDerivedConfig`) is pushed into the `policy` lint next to `policyMcpConfigBreaches`; test wired into `testCommandInputs` (`⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`).
- Verified: `generate config --check` exits 0; with a bogus dependabot directory appended it prints the drift and exits 1, then 0 after restore; `testDerivedConfig` passes. NOT run: the full `bun ./📜️script.ts policy` (needs the Go repo client binary `💻️client/client.exe`, which is not built here: `ENOENT uv_spawn`); `testCommandInputs` as a whole (its pre-existing lifecycle law fails earlier on Windows paths, see section 7).
- Broken paths fixed in `.vscode/settings.json` (also flow into the derived block): LaTeX Workshop now builds through `bun nx run @semio-tech/mit-bestand-bericht:build` (tool `semio-nx`, autoBuild `never`, `outDir` and `autoClean` removed, root files `♻️mit-bestand/📋️bericht/**/*.tex` and `📓️print/🧾️template/**/*.tex`, `TEXINPUTS` -> `📓️print/🖋️latex//`; LaTeX Workshop itself UNVERIFIED), `python.testing.pytestArgs` -> `["--rootdir=${workspaceFolder}"]` (pyproject `testpaths` do the rest), `eslint.workingDirectories` drops `elements/ui`, `sqltools.*` removed (extension not recommended, db missing), Copilot instructions -> `.agents/skills/commit/SKILL.md`, `python.defaultInterpreterPath` removed from the host file (Python extension auto-selects the workspace `.venv` on every OS; the container overlay sets `.venv/bin/python`), `remote.autoForwardPorts`/`remote.portsAttributes` removed (duplicated the devcontainer port law that L-1 owns). Settings 78 -> 71 keys.
- `SEMIO_GITKRAKEN_WORKSPACE_NAME`: `compose` -> `semio` in `devcontainer.json` and in `🔩️native/🥾️bootstrap/🔵️.ps1:417` (the lifecycle default is already `semio`). BOM of the `.ps1` preserved.

### Settings policy (Go)

`systemPolicy` no longer flags `.vscode/settings.json`; it requires the devcontainer block to equal the derivation (settings and extensions separately).

- Statutes renamed (greenfield): `BreachSystemDevcontainerVscodeSettingsOutside|ExtensionsOutside` -> `...SettingsDrift|ExtensionsDrift`, ids `system/devcontainer/vscode/settings-drift|extensions-drift`, `Autofixable` false (the autofix used to delete `.vscode/settings.json`; removed), new reason/solution pointing at `generate-config`.
- Edited: `📜️statutes/📦️packages/🐹️go/🐹️.go` (`SystemPolicy`, `DeriveContainerEditor`, `ReadHostEditor|ReadContainerEditor|ReadEditorOverlay`, JSONC comment stripper; the old helpers could not parse the commented `devcontainer.json`, so the extension check was silently dead), `📐️model/.../🐹️.go` (consts, `Info`), `📜️statutes/🧬️schema/🔣️statutes.json` (schema-first catalog read by Go and Rust), `🔗️graphql/.../🐹️.go` (both autofix cases removed), `💻️client/⌨️cli/🧩️component/🐹️.go` (the monolith named in the brief: same change; it does not compile as a Go package because of emoji import paths, only `gofmt -e` parse-checked, and it was already unformatted at HEAD).
- Rust twin: none exists (`git grep` of `vscode/settings.json|devcontainer` over `*.rs` finds nothing; the Rust statutes crate reads `statutes.json`).
- Tests: `🔗️graphql/.../🔬️_test.go` `TestSystemPolicy` rewritten (no breach, settings drift, extensions drift, missing overlay, absent files, shared fixture equality, real checkout equals derivation, registration, meta); monolith `🧪️tests/🔬️component/🐹️.go` same body. `go build ./...` clean for statutes, model, graphql; `go vet ./...` clean for statutes and graphql; `go test -run TestSystemPolicy` passes (9 subtests). `go test ./...` in statutes has one failure that is not mine: `TestPathEmojiStatutesLanguageNeutralFixture` cannot open `📚️library/🧪️tests/🔏️path-emoji-statutes/🔣️.json` (directory holds only `🟦️.ts`).

## 3. Devcontainer lifecycle

`lifecycle/🟦️.ts` `configureRepoHooks` called `target/release/semio configure --repo`: wrong binary, not built. `semio-repo configure` (Go `configureCommand`, Rust `verbs::configure`) only removes blocking git hooks and installs micro-commit hooks, which is exactly what the TS `installMicroCommitGitHooks` does and what `setup git` already runs at create time. It now runs `bun ./📜️script.ts micro-commit install-hooks` in the workspace (no binary, no Go/Rust choice, `SEMIO_REPO_IMPLEMENTATION` gone from code and README); messages "Git hooks installed." / "Git hook installation failed, continuing without blocking attach." Verified natively: `installMicroCommitGitHooks` on a fresh temp git repo wrote the five hooks (`post-checkout, post-commit, post-merge, post-rewrite, prepare-commit-msg`); a recording-host check of `configureRepoHooks` passes for exit 0 and 1 (also added to `testDevcontainerLifecycle`, but that test aborts earlier on Windows at its pre-existing GitKraken path-separator assertion, line 50, so my added lines run only on Linux; I ran them standalone). `.devcontainer/README.md` updated (hook sentence, removed `SEMIO_REPO_IMPLEMENTATION`, added the derivation paragraph). Ports prose and `forwardPorts`/`portsAttributes` untouched.

## 4. `.github/dependabot.yml`

Rewritten from the real manifests: `bun` (root, `♻️mit-bestand`, `✏️s`, `🌎️hub`, `🎓️teaching`, `🏢️semio-tech`), `cargo` (root, `✏️s`, `🌎️hub`, `🎓️teaching`, the glob `✏️s/🔌️plugins/*/🏭️bridge`, the nested stdio-semio bridge, `💻️os/🎚️config/🏭️bridge`), `gomod` (all 28 `go.work` modules), `uv` (root + 2 python packages), `nuget` (the 2 built `.csproj` projects), `github-actions` (`/`). The check (`dependabotProblems`): every directory/glob match exists and holds its ecosystem manifest, every `go.work` module has a `gomod` entry. While deriving it found a tracked orphan `✏️s/🔌️plugins/🗄️stdio/🏭️bridge/Cargo.lock` with no `Cargo.toml`; deleted (working tree).

## 5. `.config/nextest.toml` copies

Read per physical workspace (`library/🟦️.ts:1413` `repositoryCargoTestPolicyV1` joins `<workspace>/.config/nextest.toml`; test `⏱️process-budgets` "process native profiles belong to each physical Cargo workspace"), so they stay. Derived: scope file = root file + its own `[[profile.quick.overrides]]` tables (`deriveNextest`, writer, check, wired as above). `🌎️hub` differed only by one trailing blank line (rewritten); `✏️s`, `🎓️teaching` already conformed. Existing test still pins periods and override counts.

## 6. Deletions and non-deletions

- `.🧬semio/🦑️repo/compose-micro-commit-bun`: NOT dead. Read by the five git hook scripts (`🪝️hooks/*`), `🪝️hooks` Go/Rust (`bun_candidates`), `library/🟦️.ts` (`MICRO_COMMIT_BUN_PIN`, written by `installMicroCommitGitHooks`) and its test. The tracked content was a macOS path (`/Users/ueli/.bun/bin/bun`), invalid everywhere else. Working-tree file deleted and `.gitignore` now ignores it (it is regenerated per host by `setup git`; hooks fall back to `command -v bun`). The owner still needs to commit the deletion.
- `🚚️migration.json`: deleted in round 2b, see section 0.
- `.gitignore`: dead `!.vscode/tasks.json` line removed (separate line); the `launch.json` exception line was not present at the time (L-1 area, untouched).

## 7. Not done / UNVERIFIED / pre-existing failures

- Full `policy` lint run (needs unbuilt Go client binary) and the whole `testCommandInputs` suite: not run.
- `testDevcontainerLifecycle` fails on Windows before my addition (`ws create ... /metabolism` vs `\metabolism`, test line 50): pre-existing.
- `tool-configuration-ownership` (3 failures: vite export `playgroundAssetVitePlugins`, playwright "Total: 7 tests", activation config token) and `vitest-configuration-ownership` (1 failure) fail on unrelated tracked content; the `.vscode/settings.json` tokens/keys they read (`eslint.options.overrideConfigFile`, `tailwindCSS.experimental.configFile`, `vitest.configSearchPatternInclude`) are unchanged.
- Dependabot ecosystems `bun` and emoji directory names are accepted by GitHub: UNVERIFIED (cannot run Dependabot here).
- Bun on Windows: `writeFileSync` over a longer previous version left stale tail bytes once while writing `devcontainer.json`; the writer therefore uses `Bun.write` (verified, idempotent, truncating). `Bun.file().text()` strips a BOM (matters for the `.ps1`; handled).

## 8. Files touched

Edited: `package.json`, `.vscode/settings.json`, `.devcontainer/devcontainer.json` (derived block + env name), `.devcontainer/README.md`, `.github/dependabot.yml`, `.gitignore`, `🌎️hub/.config/nextest.toml`, `📜️script.ts` (import, `GenerateScript`, `PolicyRuleDerivedConfig`, policy push), `📋️project.json` (2 targets), `.storybook/🧪️tests/🧪️browser-runner/🟦️.ts` (comment), package-boundary-classification test, `lifecycle/🟦️.ts` and its test, `cache-contracts/🟦️.ts`, `🔩️native/🥾️bootstrap/🔵️.ps1`, Go: statutes, model, graphql (+tests), statutes.json, client monolith (+tests).
Created: `.devcontainer/editor-overlay.json`, `📝️derived-config/{🟦️.ts, 🧪️tests/📝️derived-config/🟦️.ts, 🧫️fixtures/📝️derived-config/🔣️.json}`, this report.
Deleted (working tree): `.🧬semio/🦑️repo/compose-micro-commit-bun`, `✏️s/🔌️plugins/🗄️stdio/🏭️bridge/Cargo.lock`.
