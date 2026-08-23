# Terra Next Direct Dependency Scout — `@vscode/test-electron` — 2026-08-23

## Decision

**ACCEPTED: retire the direct dev declaration `@vscode/test-electron`.** It is an exhaustively unreachable VS Code extension-host test-harness predecessor, not a replacement candidate. The fresh accepted dependency boundary is **129 = 66 JavaScript + 63 Rust**; removing this one direct JavaScript identity is expected to reach **128 = 65 JavaScript + 63 Rust**.

This is a narrow Phase 10 dependency-ratchet wave, not Phase 10 completion. It has no UI, Storybook, Compose, Dagre, P3/P8, Rust, Cargo, browser-runtime, or compatibility scope.

## Current Ownership And Reachability

The sole declaration is `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json:1001`:

```json
"@vscode/test-electron": "^2.5.2"
```

There is no root, UI, other-workspace, or Compose declaration. The matching workspace tuple is `bun.lock:731-742`, and the only target resolution is `bun.lock:2120` at installed version `2.5.2`.

The exact executable/configuration census excluded `node_modules`, Git/Nx caches, ticket artifacts, `bun.lock`, and `🔒️dependencies.json`. It searched static imports, dynamic imports, CommonJS requires, package/name strings, flat configuration, Nx project files, scripts, and tests for `@vscode/test-electron`, `vscode-test-electron`, and `test-electron`. Its only result was the declaration above. In particular:

- `.vscode-test.mjs:1` imports `defineConfig` from **`@vscode/test-cli`**, not this target.
- `📜️script.ts:19` invokes the `vscode-test` executable only for the long/exhaustive extension-host suite; it does not invoke or resolve `@vscode/test-electron`.
- `📋️project.json:13-43` routes `test`, `test-quick`, `test-long`, and `test-exhaustive` solely through `bun ./📜️script.ts test …`; there is no target binding in any Nx target.
- Installed `@vscode/test-cli@0.0.10` maps `vscode-test` to `out/bin.mjs` and declares its Mocha/configuration stack, not `@vscode/test-electron`.
- `bunx vscode-test --help` succeeded and printed the active VS Code test-runner options. It confirms the surviving CLI command boundary; it is not itself a pre/post test differential.

`bun pm why @vscode/test-electron` returns exactly one path:

```text
@vscode/test-electron@2.5.2
  └─ dev @semio-tech/repo-vscode@workspace (requires ^2.5.2)
```

Thus neither a source import nor a direct/transitive lock retainer connects the active test runner to the target. No shim, facade, externalization, test replacement, or configuration change is justified.

## Reverse Lock And Orphan Expectations

The target resolution declares `http-proxy-agent`, `https-proxy-agent`, `jszip`, `ora`, and `semver`.

- `bun pm why` proves `jszip@3.10.1`, `pako@1.0.11`, and `setimmediate@1.0.5` have only the target chain.
- The target-owned nested `jszip/readable-stream` and `jszip/readable-stream/string_decoder` records are removable with that branch. The unnamespaced `readable-stream@2.3.8` is independently retained through `@bytecodealliance/jco`/`bl`; do not conflate the two lock identities.
- `bun.lock` has 14 `@vscode/test-electron/ora…` namespaced resolution keys. They are target-branch records and must disappear with the target.
- `http-proxy-agent`, `https-proxy-agent`, and `semver` have independent retainers, including active `@vscode/vsce` packaging and other workspace paths. They must remain. Some Ora-subtree resolutions are shared; Bun's reconciled lock, not hand editing, is authoritative for that shared tail.

The executor must remove the VS Code workspace tuple edge, target resolution, target-namespaced Ora keys, and orphaned target-only records through Bun. Exact acceptance is zero target or target-namespaced lock records and no target-only `jszip` branch; an invented overall lock-line count is not evidence.

## Bounded Comparison

I began from `bun ./📜️script.ts verify dependencies list js --format json`, which reports the fresh 66 direct JavaScript identities. For each plausible one-owner/tooling leaf, I resolved its command/config/type boundary rather than treating package-name absence as proof. No smaller or equally safe zero-reachability row surfaced.

| Candidate                                                       | Actual boundary                                                              | Disposition             |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------- | ----------------------- |
| `@vscode/test-cli`                                              | `.vscode-test.mjs` imports it; `📜️script.ts` executes its `vscode-test` bin. | Retain.                 |
| `@vscode/vsce`                                                  | `📜️script.ts:39` executes `vsce package`; it retains lock entries above.     | Retain.                 |
| `@types/vscode`                                                 | Extension source imports the VS Code API and needs its declaration.          | Retain.                 |
| Playwright, Storybook, ESLint, Tailwind/PostCSS, Vite/Vitest/Nx | Each has an active config, executor, test, or source binding.                | Retain.                 |
| `binaryen`                                                      | The owned web materializer invokes `node_modules/binaryen/bin/wasm-opt`.     | Retain; real Wasm lane. |
| `dagre`                                                         | Held for the accepted Rust/Wasm/OffscreenCanvas layout lane.                 | Explicitly excluded.    |
| `@nxlv/python` and Compose retainers                            | Active Nx/Compose behavior.                                                  | Explicitly excluded.    |

The remaining runtime identities need an owned behavioral replacement. This candidate is smaller and independent: one private tooling declaration and its lock reachability, with no public/exported type or runtime behavior.

## Executor Packet

1. Record the pre-edit `129 = 66 JavaScript + 63 Rust` list and successful `bunx vscode-test --help` output. Repeat the exact non-ticket/non-lock census and `bun pm why @vscode/test-electron`; abort if a live binding appears.
2. Delete only `@vscode/test-electron` from the VS Code package's `devDependencies`. Do not edit `.vscode-test.mjs`, `📜️script.ts`, `📋️project.json`, extension tests, VS Code test config, Compose manifests, or `🔒️dependencies.json`; add no replacement, stub, facade, externalization, or permanent guard.
3. Reconcile with Bun (`bun install --lockfile-only --ignore-scripts --no-progress --no-summary`, or the established equivalent). Do not hand-edit `bun.lock`. Verify the removal/retention partition above, then use frozen install.
4. Require **128 = 65 JavaScript + 63 Rust** and zero JavaScript undeclared-imports/lock-mismatches. This wave changes no Rust count and runs no Cargo.

## Independent Audit Gates

```text
# Active VS Code extension boundary
bun x nx run @semio-tech/repo-vscode:lint --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:build --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test-quick --skip-nx-cache
bun x nx run @semio-tech/repo-vscode:test-long --skip-nx-cache
bunx vscode-test --help

# Install and ratchet
bun install --frozen-lockfile --ignore-scripts --no-progress --no-summary
bun ./📜️script.ts verify dependencies
bun ./📜️script.ts verify dependencies list js --format json
bun ./📜️script.ts verify dependencies list rust --format json
bun ./📜️script.ts verify dependencies parity js

# Exact target proof (exclude ticket history and static inventory where appropriate)
! rg --hidden -n -F '@vscode/test-electron' --glob 'package.json' --glob '!node_modules/**' --glob '!.🧬semio/**' --glob '!.git/**' .
! rg -n -F '@vscode/test-electron' bun.lock
! rg -n -F '@vscode/test-electron/' bun.lock
! rg --hidden -n -P "(?:from\\s+|import\\s*\\(|require\\s*\\()[\\\"']@vscode/test-electron(?:/[^\\\"']*)?[\\\"']" --glob '*.{js,jsx,ts,tsx,mjs,cjs}' --glob '!node_modules/**' --glob '!.🧬semio/**' --glob '!.git/**' .
bun pm why http-proxy-agent
bun pm why https-proxy-agent
bun pm why semver
bun pm why readable-stream

# Syntax, formatting, and concurrent-tree safety
bun build '🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/📜️script.ts' --target=bun --outfile=/dev/null --external='*'
bunx prettier --check '🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/package.json' bun.lock
git diff --check
git diff --cached --check
git diff HEAD --check
```

`test-long` is the actual extension-host behavioral differential. If the environment prevents its VS Code host from running, that is an audit blocker; the intentionally empty quick suite does not prove host behavior. No UI Storybook build is affected by this VS Code-only declaration deletion.

## Non-Goals

This does not retire `@vscode/test-cli`, `@vscode/vsce`, Vite, VS Code API types, any test, the active VS Code extension host, any Compose dependency, `dagre`, any MDX/Storybook package, any Rust dependency, or a Phase 10 runtime. No production source, manifest, lockfile, Git state, ticket lifecycle, or Cargo input was modified by this read-only scout; this report is the sole new ticket artifact.
