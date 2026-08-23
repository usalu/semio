# Owned `@vscode/test-electron` Retirement — 2026-08-23

## Outcome

The narrow direct `@vscode/test-electron` retirement is implemented. The only manifest declaration
is gone, Bun removed its lock reachability and target-only dependency branch, and the dependency
boundary changed from `129 = 66 JavaScript + 63 Rust` to
`128 = 65 JavaScript + 63 Rust`.

The active extension-host harness remains `@vscode/test-cli@0.0.10`. No VS Code test
configuration, runner script, Nx target, extension source/test, Compose input, Dagre input, Rust
input, Cargo input, Phase 3/8 file, coordinator file, or ticket metadata/checklist was edited.

This report covers only this dependency wave. It does not claim Phase 10 acceptance or completion.

## Pre-Edit Reachability And Ownership

The pre-edit dependency lists reported 66 direct JavaScript identities and 63 direct Rust
identities. The exact non-ticket/non-lock census found only:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json:1001:
    "@vscode/test-electron": "^2.5.2"
```

The static import, dynamic import, and CommonJS `require` census found no source binding.
`bun pm why @vscode/test-electron` reported the single direct path:

```text
@vscode/test-electron@2.5.2
  └─ dev @semio-tech/repo-vscode@workspace (requires ^2.5.2)
```

The pre-edit `bunx vscode-test --help` invocation succeeded. The active ownership inspection found:

- `.vscode-test.mjs` imports `defineConfig` from `@vscode/test-cli`.
- `📜️script.ts` invokes `vscode-test` only at the `long` and `exhaustive` levels.
- `📋️project.json` routes every test target through `bun ./📜️script.ts test …`.
- The package has no type/typecheck target.
- `@vscode/test-cli` owns the `vscode-test` bin; the target dependency owns no active bin,
  configuration, source import, or Nx executor edge.

The three ownership files above were unchanged before and after the retirement; their scoped Git
diff is empty.

## Exact Change

The source change deletes only the direct dev-dependency row from:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json
```

Bun reconciled `bun.lock` with:

```text
bun install --lockfile-only --ignore-scripts --no-progress --no-summary
```

For this dependency, Bun removed:

- the `@semio-tech/repo-vscode` workspace tuple edge;
- the `@vscode/test-electron@2.5.2` resolution;
- all 14 `@vscode/test-electron/ora…` namespaced records;
- target-only `jszip@3.10.1`, `pako@1.0.11`, and `setimmediate@1.0.5`;
- target-only `jszip/readable-stream` and
  `jszip/readable-stream/string_decoder` records.

`http-proxy-agent`, `https-proxy-agent`, `semver`, and unnamespaced
`readable-stream` remain with independent retainers. The retained proxy agents still resolve
through `@vscode/vsce`; `semver` retains several workspace/tooling paths; and
`readable-stream@2.3.8` remains through the `@bytecodealliance/jco` and
`@nxlv/python` branches.

The shared worktree already had staged and working `bun.lock` changes. Before this wave the lock
status was `MM`, with a working diff of 0 additions/9 deletions and a staged diff of
11 additions/5 deletions. Bun also reconciled a concurrent root
`eslint-plugin-react-hooks` manifest deletion that was present before this edit. That concurrent
branch is visible in the total lock diff but is not attributed to this wave and was not reverted.

## Active Harness Proof After Removal

The post-edit executable and dependency ownership are exact:

```text
node_modules/.bin/vscode-test -> ../@vscode/test-cli/out/bin.mjs

@vscode/test-cli@0.0.10
  └─ dev @semio-tech/repo-vscode@workspace (requires ^0.0.10)
```

Post-edit `bunx vscode-test --help` again succeeded. Running
`bunx vscode-test --list-configuration` from the package loaded:

- config path `.vscode-test.mjs`;
- file glob `out/test/**/*.test.js`;
- the VS Code workspace folder;
- extension development path at the VS Code TypeScript package;
- extension test runner `node_modules/@vscode/test-cli/out/runner.cjs`.

The default and quick Nx test targets both completed successfully and printed their intentional
no-suite messages. The quick target does not prove extension-host behavior.

## Extension-Host Differential

`bun x nx run @semio-tech/repo-vscode:test-long --skip-nx-cache` reached the live
`@vscode/test-cli` host path. It validated and downloaded VS Code `1.134.0`, then failed before
extension tests because the runner expected the old executable name `Electron`, while the
downloaded application contains `Contents/MacOS/Code`.

The exact terminal failure was:

```text
✔ Downloaded VS Code into .../.vscode-test/vscode-darwin-arm64-1.134.0
Test error: Error: ENOENT: no such file or directory, posix_spawn
  '.../Visual Studio Code.app/Contents/MacOS/Electron'
Exit code:   -2
NX   Running target test-long for project @semio-tech/repo-vscode failed
```

Therefore the live extension-host behavioral differential is blocked by the current
runner/downloaded-host binary-name mismatch. It is not reported as passing, and no extension test
result was produced. The exhaustive target reaches the same script branch and was not re-run after
this exact blocker because doing so would repeat the 302.34 MB download without adding evidence.

The runner generated a 921 MB `.vscode-test` cache during this audit. It was removed from the
workspace and moved recoverably to:

```text
/Users/ueli/.Trash/semio-vscode-test-electron-audit-20260823
```

No `.vscode-test` cache remains in the package.

## Verification

| Gate                                          | Result                                                                                                                                     |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ |
| Lock-only Bun reconciliation                  | PASS; exit 0 and saved lockfile                                                                                                            |
| Frozen install                                | PASS; `--frozen-lockfile --ignore-scripts`, exit 0                                                                                         |
| Dependency ratchet                            | PASS; baseline 238, current 128, 110 removed, zero new                                                                                     |
| JavaScript dependency list                    | PASS; 65 identities; target absent; `@vscode/test-cli` retained                                                                            |
| Rust dependency list                          | PASS; 63 identities; unchanged                                                                                                             |
| JavaScript parity                             | PASS; 83 manifests, 244 external rows, 103 evidenced, 141 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 fixtures |
| Target manifest census                        | PASS; zero non-ticket `package.json` declarations                                                                                          |
| Target module-edge census                     | PASS; zero static/dynamic/CommonJS edges                                                                                                   |
| Target and target-namespace lock census       | PASS; zero records                                                                                                                         |
| Target-only orphan lock census                | PASS; zero `jszip`, `pako`, `setimmediate`, or `jszip/…` records                                                                           |
| `bun pm why` target/orphans                   | PASS-as-absence; each returns the expected “No packages matching … found in lockfile” with exit 1                                          |
| Installed target/orphan absence               | PASS; target, `jszip`, `pako`, and `setimmediate` are absent after the final frozen install                                                |
| Shared proxy/semver/readable-stream retention | PASS; records and independent reverse paths remain                                                                                         |
| Active CLI/config/runner ownership            | PASS; all resolve to `@vscode/test-cli`                                                                                                    |
| Script syntax                                 | PASS; one-module Bun build with imports externalized                                                                                       |
| Default package test                          | PASS; intentional no fundamental-level suite                                                                                               |
| Quick package test                            | PASS; intentional no quick-level suite                                                                                                     |
| Long package test                             | BLOCKED; downloaded host lacks expected `Contents/MacOS/Electron`                                                                          |
| Package lint                                  | FAIL; ESLint 10.8.0 cannot stat the script-selected missing `🟦️eslint.config.ts`                                                           |
| Package build                                 | FAIL; first default Vite invocation transforms 0 modules and cannot resolve `index.html`                                                   |
| Type target                                   | NOT PRESENT; Nx exposes no type/typecheck target for this package                                                                          |
| Manifest formatting                           | PASS; package `package.json` matches Prettier                                                                                              |
| Combined manifest/lock Prettier command       | NOT APPLICABLE to lock by inference; Prettier cannot infer a parser for `bun.lock`                                                         |
| Explicit JSON-parser lock check               | FAIL; Prettier reports Bun-generated lock style differs from its JSON style                                                                |
| Working diff check                            | PASS; `git diff --check`                                                                                                                   |
| Staged diff check                             | PASS; `git diff --cached --check`                                                                                                          |
| HEAD diff check                               | PASS; `git diff HEAD --check`                                                                                                              |

The lint and build failures are structural and independent of the removed declaration: the
unchanged package script points lint to a file that does not exist and invokes the first Vite build
without resolving the emoji-named configuration. They were recorded, not repaired, because those
configuration/build changes are outside this wave.

The first frozen install did not prune the pre-existing ignored physical directories
`node_modules/@vscode/test-electron` (572 KB), `node_modules/jszip` (1.0 MB),
`node_modules/pako` (836 KB), and `node_modules/setimmediate` (16 KB), all timestamped
2026-08-07. They had no manifest, lock, reverse-dependency, bin, or active runner ownership after
the retirement. They were moved recoverably under
`/Users/ueli/.Trash/semio-vscode-test-electron-audit-20260823/stale-node-modules`. A second frozen
install passed and did not recreate them. The active bin continues to resolve to
`@vscode/test-cli`.

## Commands Run

```text
bun pm why @vscode/test-electron
bunx vscode-test --help
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun install --lockfile-only --ignore-scripts --no-progress --no-summary
bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary
bun x nx show project @semio-tech/repo-vscode --json
bun x nx run @semio-tech/repo-vscode:lint --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:build --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test-quick --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test-long --skip-nx-cache
bunx vscode-test --help
bunx vscode-test --list-configuration
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
bun pm why @vscode/test-cli
bun pm why http-proxy-agent
bun pm why https-proxy-agent
bun pm why semver
bun pm why readable-stream
bun pm why jszip
bun pm why pako
bun pm why setimmediate
bun build '<vscode-package>/📜️script.ts' --target=bun --outfile=/dev/null --external='*'
bunx prettier --check '<vscode-package>/package.json' bun.lock
bunx prettier --check '<vscode-package>/package.json'
bunx prettier --check bun.lock --parser json
git diff --check
git diff --cached --check
git diff HEAD --check
```

No Cargo command, Git-modifying command, broad root lint, ticket lifecycle operation, coordinator
edit, or excluded-file edit was made. The repository MCP and `repo://goals` resource were not
available in this executor session, so ticket lifecycle and goal state were left unchanged.
