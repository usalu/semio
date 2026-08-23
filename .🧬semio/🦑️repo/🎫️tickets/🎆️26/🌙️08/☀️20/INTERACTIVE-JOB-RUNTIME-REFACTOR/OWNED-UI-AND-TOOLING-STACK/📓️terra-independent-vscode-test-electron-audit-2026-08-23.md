# Terra Independent Audit — `@vscode/test-electron` Retirement — 2026-08-23

## Verdict

**REJECT.** The direct declaration removal is mechanically consistent but breaks the live VS Code extension-host test path. This verdict concerns this narrow dependency wave only; it is not a Phase 10 verdict.

## Wave-Local Blocker

The retained `@vscode/test-cli@0.0.10` executable dynamically requires the removed package from the extension package before it can start the extension-host tests:

```text
node_modules/@vscode/test-cli/out/cli/platform/desktop.mjs:60-63
async importTestElectron() {
  const electronPath = await mustResolve(this.config.dir, '@vscode/test-electron');
  const electron = await import(pathToFileURL(electronPath).toString());
}

desktop.mjs:66-74
async run(...) {
  const electron = await this.importTestElectron();
  ...
  return await electron.runTests(...);
}
```

`this.config.dir` is the VS Code package directory. I invoked that same installed `mustResolve` helper against the package directory without launching a VS Code host or downloading anything. It exited 1 with `Can't resolve '@vscode/test-electron'`; its resolver trace confirms that the package-local and upward `node_modules` paths do not contain the target. Both `node_modules/@vscode/test-electron` and the package-local equivalent are absent.

The published test-cli package makes the contract discoverable but easy to miss: its own `package.json:46-53` lists `@vscode/test-electron: ^2.4.1` under `devDependencies`, while its compiled shipping runner still dynamically resolves it from the consuming project. Therefore it is not a Bun transitive dependency; it is a consumer-supplied runtime precondition for the active long/exhaustive test command. This explains why the lock row and `bun pm why` can be absent while the live runner is broken.

The implementation report's pre-cleanup long run reached a later host binary-name error only because an ignored, stale physical copy of the target still existed then. That copy was subsequently moved to Trash. I did not restore, inspect, or remove any Trash item, and I did not repeat the long run because it would launch/download a VS Code host. The historical `Contents/MacOS/Electron` versus `Code` mismatch is a separate pre-existing runner/host problem; it is not evidence that the final post-removal state can reach host execution. The current direct resolver failure happens earlier and is sufficient to reject this wave.

## Exact Scope And Lock Closure

The source manifest diff against `HEAD` is exactly one removal:

```text
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json
- "@vscode/test-electron": "^2.5.2"
```

The current `bun.lock` has no target tuple/resolution/namespaced record and no target-only `jszip`, `pako`, `setimmediate`, `jszip/readable-stream`, or `jszip/readable-stream/string_decoder` record. The combined `HEAD` diff removes exactly the target workspace tuple edge, `@vscode/test-electron@2.5.2`, those three orphan roots, the two JSZip branch records, and all 14 `@vscode/test-electron/ora…` records. `http-proxy-agent`, `https-proxy-agent`, `semver`, and unnamespaced `readable-stream` remain.

This proves lock closure—not behavioral safety. `bun pm why @vscode/test-electron`, `jszip`, `pako`, and `setimmediate` each exit 1 because the records are absent. `bun pm why @vscode/test-cli` retains its one direct VS Code workspace path. The target-specific deletion is currently in the working layer; `git diff --cached` has no target mention. The shared `bun.lock` also contains concurrent staged/working changes from other accepted waves, so they are not attributed here.

The executable-source/configuration scan has no owned source/config import or string binding; its only non-ticket result is the static `🔒️dependencies.json` inventory row. The active binding is instead inside the installed retained runner, as shown above. The changed owned source surface is limited to the VS Code manifest and `bun.lock`; there is no P3/P8, Compose, Dagre, Rust, Cargo, Storybook, or compatibility/facade/externalization edit.

## Safe Checks Performed

| Check                                                                       | Result           | Meaning                                                                                                                                                                                    |
| --------------------------------------------------------------------------- | ---------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `bunx vscode-test --help`                                                   | PASS             | The retained test-cli bin loads without host execution.                                                                                                                                    |
| `bunx vscode-test --list-configuration`                                     | PASS             | Loads `.vscode-test.mjs`, the `out/test/**/*.test.js` glob, workspace folder, extension path, and `@vscode/test-cli/out/runner.cjs`; it does not call `run()`.                             |
| `nx …:test --skip-nx-cache`                                                 | PASS             | Intentional “no fundamental-level suite” branch; no host execution.                                                                                                                        |
| `nx …:test-quick --skip-nx-cache`                                           | PASS             | Intentional “no quick-level suite” branch; no host execution.                                                                                                                              |
| Installed `mustResolve(..., '@vscode/test-electron')`                       | **FAIL, exit 1** | Reproduces the active long/exhaustive prerequisite failure without launch/download.                                                                                                        |
| `bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary` | PASS             | Exit 0 with no lifecycle scripts or observed download output. It preserves the broken final closure because the package is absent from the lock.                                           |
| Dependency verifier                                                         | PASS             | Current `128 = 65 JavaScript + 63 Rust`; ratchet reports no new dependency.                                                                                                                |
| `verify dependencies parity js`                                             | PASS             | `manifests=83`, `undeclared-imports=0`, `lock-mismatches=0`, `lock-workspaces=44`, `lock-fixtures=5`. This verifier cannot establish an installed package's dynamic consumer prerequisite. |
| Test-script syntax (`bun build … --external='*'`)                           | PASS             | One module bundled successfully.                                                                                                                                                           |
| VS Code manifest Prettier check                                             | PASS             | Applicable JSON input is formatted.                                                                                                                                                        |
| `bun.lock` Prettier inference                                               | NOT APPLICABLE   | `prettier --check bun.lock` exits 2 because no parser is inferred for Bun's generated lockfile; no JSON-parser rewrite is a validity gate.                                                 |
| Working/staged/HEAD `git diff --check`                                      | PASS             | All three exit 0 despite concurrent changes.                                                                                                                                               |

The active CLI ownership that still passes is exact: `node_modules/.bin/vscode-test` points to `../@vscode/test-cli/out/bin.mjs`, `.vscode-test.mjs:1` imports `defineConfig` from `@vscode/test-cli`, and that package exposes the `vscode-test` bin. Help, configuration listing, default, and quick do **not** traverse `PreparedDesktopRun.run`, so none compensates for the long/exhaustive blocker.

## Non-Green Context Not Replayed

No broad root lint, Cargo command, host launch, VS Code download, or Trash operation was performed.

The existing implementation artifact records package lint failure because `📜️script.ts:32` selects missing `🟦️eslint.config.ts`, and build failure because `📜️script.ts:25` invokes default Vite build while the package has no `index.html` or Vite config input. Current filesystem checks confirm those inputs remain absent. They are unchanged structural package defects, not caused by this declaration deletion; I did not rerun commands that could write build artifacts. `nx show project @semio-tech/repo-vscode --json` lists only `build`, `build-vsix`, `dev`, `lint`, `test`, `test-exhaustive`, `test-long`, and `test-quick`: no `type` or `typecheck` target exists.

The historical long-test host-name mismatch is likewise not replayed under the audit restrictions. Its evidence does not discharge the more immediate current resolver failure.

## Required Disposition

Do not accept this retirement. A correct follow-up must first preserve a functional extension-host runner path and prove it without relying on ignored stale `node_modules` content. That is a new scoped decision: this audit neither restores the dependency nor proposes a replacement, stub, facade, externalization, or configuration workaround.

No source, manifest, lockfile, coordinator file, ticket lifecycle, Git state, Cargo input, network download, VS Code host, or Trash content was modified by this audit. This Markdown file is the only audit artifact created.
