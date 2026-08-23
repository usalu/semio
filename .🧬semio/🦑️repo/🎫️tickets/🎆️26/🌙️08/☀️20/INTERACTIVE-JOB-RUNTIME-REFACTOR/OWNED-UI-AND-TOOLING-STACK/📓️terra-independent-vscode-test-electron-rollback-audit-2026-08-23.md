# Terra Independent Rollback Audit — `@vscode/test-electron` — 2026-08-23

## Verdict

**ACCEPT.** The rejected `@vscode/test-electron` retirement is fully restored as a narrow rollback: the direct declaration, Bun lock closure, physical package closure, and active test-cli dynamic resolver agree at **129 = 66 JavaScript + 63 Rust**. This accepts restoration only, not Phase 10.

## Restored Declaration And Lock Shape

The VS Code workspace manifest again contains its original direct development declaration at `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json:1001`:

```json
"@vscode/test-electron": "^2.5.2"
```

The current lock has exactly 16 target mentions: the VS Code workspace tuple edge (`bun.lock:738`), `@vscode/test-electron@2.5.2` (`:2120`), and all 14 `@vscode/test-electron/ora…` namespaced records. It also restores the required target branch:

- `jszip@3.10.1`, `pako@1.0.11`, and `setimmediate@1.0.5`;
- `jszip/readable-stream` and `jszip/readable-stream/string_decoder`.

`bun pm why` confirms the complete owner chain: target → `@semio-tech/repo-vscode`; JSZip → target; Pako/SetImmediate → JSZip → target. The prior accepted `eslint-plugin-react-hooks` removal remains intact: it is absent from both root `package.json` and `bun.lock`.

The VS Code manifest has zero working, staged, and `HEAD` diff. Zero-context working/staged/`HEAD` lock-and-manifest diff scans contain no `@vscode/test-electron` addition, deletion, or replacement. The shared `bun.lock` may contain concurrent changes, but there is no target-specific residue. No P3/P8, Compose, Dagre, Rust, Cargo, Storybook, source, runner, configuration, replacement, compatibility, or externalization edit overlaps this rollback.

## Active Runner Restoration

The retained executable is still `node_modules/.bin/vscode-test -> ../@vscode/test-cli/out/bin.mjs`; `.vscode-test.mjs` remains the active `@vscode/test-cli` configuration input.

After the required frozen install, the physical target and closure (`@vscode/test-electron`, `jszip`, `pako`, and `setimmediate`) are present. I invoked the same installed test-cli `mustResolve` helper with the absolute configuration directory emitted by `vscode-test --list-configuration`, then dynamically imported its result without launching a host:

```json
{
 "resolved": "/Users/ueli/Documents/semio/node_modules/@vscode/test-electron/out/index.js",
 "runTests": "function",
 "downloadAndUnzipVSCode": "function"
}
```

This directly restores the prerequisite which caused the rejected wave: `@vscode/test-cli`'s desktop runner resolves and imports `@vscode/test-electron` before it calls `runTests`.

An earlier audit probe used a relative context path and therefore failed enhanced-resolve before it could walk to the workspace root. That is not the runner's actual context: `--list-configuration` reports an absolute config path. The absolute-context probe above is the controlling result.

## Independent Gates

| Gate                                                                        | Result                                                                                                                                                           |
| --------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `bunx vscode-test --help`                                                   | PASS; no host launch.                                                                                                                                            |
| `bunx vscode-test --list-configuration`                                     | PASS; reports the absolute `.vscode-test.mjs` config, `out/test/**/*.test.js`, workspace/extension paths, and `@vscode/test-cli/out/runner.cjs`.                 |
| `nx …:test --skip-nx-cache`                                                 | PASS; intentional no fundamental-level suite.                                                                                                                    |
| `nx …:test-quick --skip-nx-cache`                                           | PASS; intentional no quick-level suite.                                                                                                                          |
| `.vscode-test` cache after safe commands                                    | ABSENT.                                                                                                                                                          |
| `bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS; exit 0.                                                                                                                                                    |
| Dependency verifier                                                         | PASS; baseline 238, current 129, no new identity.                                                                                                                |
| Exact lists                                                                 | PASS; JavaScript 66, Rust 63.                                                                                                                                    |
| Coordinator-list byte comparison                                            | PASS; both `cmp` exit 0.                                                                                                                                         |
| JavaScript parity                                                           | PASS; `manifests=83`, `external-rows=245`, `evidenced=103`, `unowned=142`, `undeclared-imports=0`, `lock-workspaces=44`, `lock-mismatches=0`, `lock-fixtures=5`. |
| Test-script syntax                                                          | PASS; externalized one-module Bun build.                                                                                                                         |
| VS Code manifest Prettier                                                   | PASS.                                                                                                                                                            |
| `bun.lock` Prettier inference                                               | NOT APPLICABLE; `prettier --check bun.lock` exits 2 because Bun's generated lock has no inferred parser.                                                         |
| Working/staged/`HEAD` diff checks                                           | PASS; all `git diff --check` variants exit 0.                                                                                                                    |

The safe default/quick paths do not claim extension-host execution. Long/exhaustive tests remain deliberately unrun: they would launch/download VS Code, and the historical downloaded-host binary-name mismatch is separate non-green context. This rollback instead proves the pre-launch resolver/import condition that the rejected removal broke.

## Commands Independently Run

```text
bun pm why @vscode/test-electron
bun pm why jszip
bun pm why pako
bun pm why setimmediate
bunx vscode-test --help
bunx vscode-test --list-configuration
bun x nx run @semio-tech/repo-vscode:test --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test-quick --skip-nx-cache
bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json | cmp -s - <coordinator-js-list>
bun ./📜️script.ts verify dependencies list rust --format json | cmp -s - <coordinator-rust-list>
bun ./📜️script.ts verify dependencies parity js
bun build '<vscode-package>/📜️script.ts' --target=bun --outfile=/dev/null --external='*'
bunx prettier --check '<vscode-package>/package.json'
git diff --check
git diff --cached --check
git diff HEAD --check
```

No VS Code host launch/download, long/exhaustive test, Trash access, Cargo command, broad root lint, Git mutation, source/manifest/lock/coordinator/ticket-metadata edit, or lifecycle operation was performed. This audit Markdown is the only new artifact.
