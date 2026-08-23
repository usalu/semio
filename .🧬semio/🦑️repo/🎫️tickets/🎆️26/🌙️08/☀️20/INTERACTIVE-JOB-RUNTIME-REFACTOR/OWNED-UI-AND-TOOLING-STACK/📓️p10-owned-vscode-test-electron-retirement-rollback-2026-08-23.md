# Owned `@vscode/test-electron` Retirement Rollback — 2026-08-23

## Outcome

The rejected direct `@vscode/test-electron` retirement is fully rolled back. The exact
`"@vscode/test-electron": "^2.5.2"` dev-dependency declaration is restored in its original
`@semio-tech/repo-vscode` position, Bun restored the matching lock and installed closure, and the
live dependency boundary is again `129 = 66 JavaScript + 63 Rust`.

The generated JavaScript and Rust dependency inventories byte-match the current coordinator lists.
The preceding accepted `eslint-plugin-react-hooks` removal remains intact. No runner rewrite,
compatibility code, replacement, externalization, Compose/Dagre/Rust/P3/P8 change, coordinator-list
edit, or ticket-metadata edit was made.

This report records rollback of one rejected wave. It does not claim Phase 10 acceptance.

## Rejection Cause

The independent Terra audit established the missing active edge inside installed
`@vscode/test-cli@0.0.10`:

```js
async importTestElectron() {
  const electronPath = await mustResolve(this.config.dir, "@vscode/test-electron");
  const electron = await import(pathToFileURL(electronPath).toString());
  return electron;
}
```

`PreparedDesktopRun.run` calls `importTestElectron()` before
`@vscode/test-electron.runTests`. The published test-cli package lists
`@vscode/test-electron: ^2.4.1` only as its own development dependency, so Bun does not supply the
shipping runner's consumer-side prerequisite transitively. The consuming VS Code workspace must
retain its direct declaration.

Help, configuration listing, default tests, and quick tests do not traverse this long/exhaustive
runner branch. Their earlier success could not establish extension-host behavior.

## Exact Rollback

The VS Code package declaration was restored with `apply_patch`, then Bun was authoritative for
both lock and installed closure:

```text
bun install --lockfile-only --ignore-scripts --no-progress --no-summary
bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary
```

The lock-only install resolved/extracted 16 packages and saved `bun.lock`. Bun restored:

- the `@semio-tech/repo-vscode` workspace tuple edge;
- `@vscode/test-electron@2.5.2`;
- all 14 `@vscode/test-electron/ora…` namespaced records;
- `jszip@3.10.1`, `pako@1.0.11`, and `setimmediate@1.0.5`;
- `jszip/readable-stream` and `jszip/readable-stream/string_decoder`.

The current lock contains 16 `@vscode/test-electron` mentions: one workspace tuple, one target
resolution, and 14 namespaced Ora records.

The VS Code package manifest now has zero working or staged diff against `HEAD`. A targeted search
of the current `bun.lock` diff finds no target, JSZip, Pako, or SetImmediate rollback residue.
The shared lock remains `MM` because of unrelated accepted/concurrent waves.
`eslint-plugin-react-hooks` remains absent from both the root manifest and `bun.lock`.

No item was restored, moved, deleted, or inspected in Trash during this rollback. Normal Bun
installation reconstructed the required installed packages from the restored lock.

## Installed Runtime Resolution

`bun pm why @vscode/test-electron` now reports:

```text
@vscode/test-electron@2.5.2
  └─ dev @semio-tech/repo-vscode@workspace (requires ^2.5.2)
```

The target-only closure also has the expected reverse paths:

```text
jszip@3.10.1
  └─ @vscode/test-electron@2.5.2

pako@1.0.11
  └─ jszip@3.10.1
     └─ @vscode/test-electron@2.5.2

setimmediate@1.0.5
  └─ jszip@3.10.1
     └─ @vscode/test-electron@2.5.2
```

The installed test-cli resolver was invoked directly against the VS Code package directory without
launching or downloading a host. It resolved and dynamically imported:

```json
{
 "resolved": "/Users/ueli/Documents/semio/node_modules/@vscode/test-electron/out/index.js",
 "runTests": "function",
 "downloadAndUnzipVSCode": "function"
}
```

The active executable remains:

```text
node_modules/.bin/vscode-test -> ../@vscode/test-cli/out/bin.mjs
```

This proves the retained test-cli bin can reach its consumer-supplied test-electron runtime
prerequisite before `runTests`.

## Dependency Boundary And Coordinator Parity

`bun ./📜️script.ts verify dependencies` reports:

```text
baseline: 238
current: 129
removed: 109
clean — no new third-party dependencies
```

The ecosystem lists report exactly:

| Ecosystem  | Count |
| ---------- | ----: |
| JavaScript |    66 |
| Rust       |    63 |
| Total      |   129 |

Byte comparisons completed with exit 0:

```text
cmp <(bun ./📜️script.ts verify dependencies list js --format json) \
  📝️coordinator-current-js-dependencies.txt
cmp <(bun ./📜️script.ts verify dependencies list rust --format json) \
  📝️coordinator-current-rust-dependencies.txt
```

JavaScript parity is clean:

```text
manifests=83
external-rows=245
evidenced=103
unowned=142
undeclared-imports=0
lock-workspaces=44
lock-mismatches=0
lock-fixtures=5
```

## Safe Harness Checks

| Gate                                     | Result                                                                                                                    |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `bunx vscode-test --help`                | PASS; retained CLI loads                                                                                                  |
| `bunx vscode-test --list-configuration`  | PASS; loads `.vscode-test.mjs`, `out/test/**/*.test.js`, workspace/extension paths, and `@vscode/test-cli/out/runner.cjs` |
| Default Nx package test                  | PASS; intentional no fundamental-level suite                                                                              |
| Quick Nx package test                    | PASS; intentional no quick-level suite                                                                                    |
| Installed `mustResolve` + dynamic import | PASS; target resolves and exposes `runTests`                                                                              |
| Host/long/exhaustive test                | NOT RUN; prohibited for this rollback                                                                                     |
| Package `.vscode-test` cache             | ABSENT after all safe checks                                                                                              |

The historical downloaded-host `Contents/MacOS/Electron` versus `Contents/MacOS/Code` mismatch
remains separate, non-green context. This rollback restores the prerequisite resolver path but does
not claim an end-to-end extension-host pass.

## Remaining Verification

| Gate                              | Result                                               |
| --------------------------------- | ---------------------------------------------------- |
| Lock-only Bun reconciliation      | PASS                                                 |
| Frozen Bun install                | PASS; no scripts                                     |
| Dependency verifier               | PASS; 129                                            |
| JavaScript list                   | PASS; 66 and byte-identical to coordinator list      |
| Rust list                         | PASS; 63 and byte-identical to coordinator list      |
| JavaScript parity                 | PASS; zero undeclared imports and lock mismatches    |
| Target reverse dependency         | PASS; direct VS Code workspace owner                 |
| Target-only closure reverse paths | PASS                                                 |
| Installed active resolution       | PASS                                                 |
| Test script syntax                | PASS; one-module Bun build with imports externalized |
| VS Code manifest formatting       | PASS; Prettier                                       |
| `bun.lock` Prettier inference     | NOT APPLICABLE; no parser can be inferred, exit 2    |
| Working diff check                | PASS                                                 |
| Staged diff check                 | PASS                                                 |
| HEAD diff check                   | PASS                                                 |

The rejected wave's earlier package lint and build failures were not replayed: lint selects the
unchanged missing `🟦️eslint.config.ts`, and the first unchanged Vite build cannot resolve an
`index.html`. They are outside this rollback. No type/typecheck target exists for the VS Code
package.

## Commands Run

```text
bun install --lockfile-only --ignore-scripts --no-progress --no-summary
bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js
cmp <(bun ./📜️script.ts verify dependencies list js --format json) <coordinator-js-list>
cmp <(bun ./📜️script.ts verify dependencies list rust --format json) <coordinator-rust-list>
bun pm why @vscode/test-electron
bun pm why jszip
bun pm why pako
bun pm why setimmediate
bun -e '<mustResolve and dynamic-import proof>'
bunx vscode-test --help
bunx vscode-test --list-configuration
bun x nx run @semio-tech/repo-vscode:test --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test-quick --skip-nx-cache
bun build '<vscode-package>/📜️script.ts' --target=bun --outfile=/dev/null --external='*'
bunx prettier --check '<vscode-package>/package.json'
bunx prettier --check bun.lock
git diff --check
git diff --cached --check
git diff HEAD --check
```

No host download, VS Code launch, long/exhaustive test, Cargo command, broad root lint, Git-modifying
command, coordinator/report-list edit, ticket lifecycle operation, or excluded-file edit occurred.
The repository MCP and `repo://goals` resource were unavailable in this executor session, so
ticket lifecycle and goal state were left unchanged.
