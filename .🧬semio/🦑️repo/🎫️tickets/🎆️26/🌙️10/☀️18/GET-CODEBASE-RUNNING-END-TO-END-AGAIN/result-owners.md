# Result: owners (build-owners registry consistency)

## Findings
- `📋️build-owners.json` is an **authored** registry, not generated. Schema: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️owners.json`; validator `readBuildOwnerRegistryV1` in `…/🏗️build/🕸️graph/🚪️admission/🟦️.ts`; readers in `…/🕸️graph/🟦️.ts` and `…/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`. Only the crate-tree carve tools (`🏗️authoring/📦️crate-tree`, `🧬️mutation-tree`) write rows, and only for carved crates. No tool covers renames or project.json-only modules, so the quiz to course rename and the monolith-split projects left it stale. The root cause is "no completeness check"; there is no generator to fix.
- Registry vs tree before the fix (3664 rows): 9 stale rows, 21 projects/crates without a row, 1 package list wrong, 3 Cargo-manifest rows shadowing a sibling `📋️project.json` (its targets were ignored by admission).

## Changes
- `📋️build-owners.json` (3664 to 3675 rows, written by the sync tool, byte-identical JSON formatting, same-position/byte-order inserts):
  - Removed 9 stale rows: `@semio-tech/quiz`, `quiz-react`, `quiz-rs`, `quiz-schema-rs`, `quiz-tests-rs`, `quiz-tests-runs-rs`, `quiz-primitives-tests-rs`, `semio-framework-quiz-tests-support` (roots `🛍️products/❓️quiz/**` no longer exist), and `@semio-tech/animate-js` (no TypeScript package under `✏️s/🔌️plugins/🎞️animate/📦️packages`).
  - Added 20 rows: `@semio-tech/course`, `course-react`, `course-rs`, `semio-framework-course-tests-support` (framework course), `fem-node-rs`, `fem-material-rs`, `semio-s-artifact-fem-2d-results-state`, `energy-model-state`, `framework-record-projection`, `framework-native-tuple-receiving`, `framework-original-slot`, `math-rank-rs`, `math-dense-rs`, `framework-history-read`, `framework-store-native-semantic`, `framework-window-semantic-preview`, `repo-developer-tools`, `repo-nx-output-custody`, `repo-artifact-reader`, `repo-next-inputs`.
  - Converted 3 Cargo rows to their `📋️project.json` owners: `puzzle-2d-rs`, `puzzle-5d-rs`, `framework-plugin-ownership` (old names `semio-s-artifact-puzzle-2d/5d`, `semio-framework-plugin-ownership` are referenced nowhere outside lockfiles).
  - Fixed `packages` of `@semio-tech/framework-io-codec-rs` (its `package.json` declares the name).
- New ticket tools in `C/owners-tools/`: `owners-check.ts` (checker plus `--fix` sync; reuses the discovery exclusions: example collections, generatedDirectories), `owners-check.test.ts` (3 fixture-tree tests).

## Verification
- `bun C/owners-tools/owners-check.ts /Users/ueli/Documents/semio --list`: before 36 findings (first version) / 33 (final rules); after `--fix` 0. Latest re-run: 1 finding, `@semio-tech/trinity-jack-lsp` manifest absent (a peer removed `…/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/📋️project.json` after my run; not mine, row left).
- Independent python cross-check (json + regex over every row): 3675 rows, 0 missing manifests, 0 name mismatches, 0 duplicate names/roots, 0 quiz rows.
- `readBuildOwnerRegistryV1(real file)` (schema + collision): ok, 3675 owners.
- `bun test` in `C/owners-tools/`: 3 pass.
- Existing tests: `…/🕸️graph/🚪️admission/🧪️tests` 1 pass; `…/📥️inference/🧪️tests` 5 pass; `…/🧪️tests/🕸️graph/🟦️.ts` 88 pass, 82 fail, 2 skip, all failures launcher-gated or env (Script process invocation envelope 51, Nx namespace 8, service not running 8, cache reader 4, ...; uses fixture registries, not the real file); `…/🧪️tests/🕸️graph/🔄️candidate` 0/3 and `🔄️settlement` 0/2 (launcher envelope required). Launcher-gated list: those three files.
- Tests that read the real registry: `📡️replication/⚠️error/🧪️tests` 4 pass; `🖱️ui/🧪️tests/📦️crate-boundaries` 59 pass; `🛢️db/…/📦️crate-boundaries` 44 pass, 1 timeout at 5 s; `💻️os/🧪️tests/📦️crate-boundaries` 95 pass, 21 fail (os-crate source-ceiling/`history_read` module drift of the ongoing split, unrelated to rows); `dashboard-control` 7 pass (including the Ajv2020 oracle plus our schema validator over the registry and the native-owner rows), 2 fail (timeout; dashboard route `build` vs `native` for `repo-dashboard-rs:workflow status`, unrelated); `🧠️interpreter/…/🔌️exports` fails on missing module `tree-sitter-wasms` (node_modules).
- `bun repo preflight --scope @teaching/architecture-course`: **not run**. The command exists (`bun repo --help` lists `preflight --scope <owner>`), but `bun repo dependencies lock --all` (pid 69428) was running for more than 8 minutes; I waited ~7 minutes and it did not exit.

## Open items
1. Add a permanent gate: a check (like `owners-check.ts`, moved into the build module with fixture test) in `verify`/preflight so renames and new project.json modules cannot leave rows stale. Needs a repo-build owner; I did not change production code.
2. `@semio-tech/trinity-jack-lsp` row now points to an absent manifest (peer change in flight); re-run the checker with `--fix` once settled.
3. Row `semio-s-artifact-puzzle-2d-graph-manifest` points to a gitignored generated crate (`🤖️generated/…/Cargo.toml`); it exists locally but a fresh clone has no file until the generator runs. Decide whether it belongs in the registry.
4. Run `bun repo preflight --scope @teaching/architecture-course` when no `bun repo dependencies` runs.
5. bun.lock files still mention `@semio-tech/animate-js` (deps owner).
