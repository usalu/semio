# Graph DSL TypeScript Dead-Package Central Acceptance

## Disposition

The authored TypeScript graph DSL had zero production consumers and its Math TypeScript package became empty after deletion. The seven authored/package files are deleted. The package's `node_modules` cache directory remains untouched and out of scope. The live Rust graph DSL and Math implementation remain unchanged.

## Baseline

- HEAD: `0727b80aa6a802cac1760f90fb7a148f74035413`.
- Deleted source/package baseline hashes are recorded in `📓️terra-graph-dsl-typescript-dead-package-dissolution-acceptance.md`.
- Root package baseline SHA-256: `6216197e8dab4a76939c4456b3fa3b5796bfdf616803ce48b51cc758e6801c85`.
- OS TypeScript package baseline SHA-256: `418abb9ed16f25fd014ac48fb28faafffc404551ae5028eb7ad89a206fbbcbc7`.
- Bun lock baseline SHA-256: `6dafbd22ee4765b5bc54d94c4d413933e1d96cf7ecc1f42890acf29e46c7c976`.
- All three coordinator-owned paths were clean before editing.

## Central Closure

- Removed the dead Math TypeScript workspace path from root `package.json`.
- Removed `@semio-tech/framework-math-js` from the OS TypeScript package dependencies.
- Removed exactly the corresponding Bun lock workspace package block, OS dependency entry, and workspace resolution entry.
- `.vscode/launch.json` had no Math TypeScript project registration and was not edited.

`bun install --lockfile-only` was attempted before and after removal of the root workspace entry. The first attempt exposed that required root entry. The second attempt reached an unrelated pre-existing repository blocker: missing `patches/@electron-forge%2Fcore-utils@7.11.2.patch`. The lock was therefore not regenerated. Only the three deterministic dead-package records above were removed manually; no unrelated lock resolution changed.

## Final Hashes

- Root `package.json`: `5d29caab1f4209c12bacc9da7a0028f0b5369afde466a41ed3c53afba21ff551`.
- OS TypeScript `package.json`: `921ae542437933dff6f207c5e88901768aebf152bf31d8c5331e173d30d22b02`.
- `bun.lock`: `5aba4612e6a593cf53ae133e5e9f3e8ca54d90b082e8d2bd05352958414f487e`.

## Verification

- Active source/config search for the deleted package name and workspace path: zero hits outside tickets, history, and dependency caches.
- Search of all 19 deleted graph DSL exports: zero external hits, as recorded by Terra.
- Root and OS package JSON parse with Bun: pass.
- Ordinary and cached scoped `git diff --check`: pass.
- Coordinator central diff: root package deletes one workspace line; OS package deletes one dependency line; Bun lock deletes only the five-line workspace block, one dependency line, and two-line resolution record.
- `bun nx show projects`: exit 0; `@semio-tech/framework-math-js` is absent and the workspace project graph remains discoverable.
- `bun nx run @semio-tech/framework-os:test-quick --skip-nx-cache`: reaches 294 tests, with 292 passing and two duplicate-run failures for the same missing generated WASM file `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/pkg/semio_framework_os.js`. This is an external generated-artifact prerequisite and not a Math TypeScript reference or TS-02 failure.

## Inventory

Updated:

- `package.json`
- `bun.lock`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/package.json`

Deleted:

- `🧰️framework/🔨️modules/🕸️graph/🗣️dsl/🟦️component.ts`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/tsconfig.json`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/📦️index.ts`
- `🧰️framework/🔨️modules/🧮️math/📦️packages/🟦️typescript/🧪️vitest.config.ts`

Created:

- `📓️terra-graph-dsl-typescript-dead-package-dissolution-acceptance.md`
- `📓️sol-graph-dsl-typescript-dead-package-central-acceptance.md`
