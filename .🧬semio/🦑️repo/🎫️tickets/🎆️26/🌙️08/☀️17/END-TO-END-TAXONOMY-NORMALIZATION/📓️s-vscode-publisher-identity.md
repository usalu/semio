# VS Code Publisher Identity Closure

## Result

The complete VS Code package gate is closed. The extension manifest uses the valid unscoped package name `repo-vscode`, while `nx.name` and `📋️project.json` preserve the workspace project identity `@semio-tech/repo-vscode`. Bun's generated workspace identity in `bun.lock` was refreshed to `repo-vscode`.

The package router now owns a config-free programmatic Vite build: `🟦️extension.ts` emits CommonJS `out/extension.js`, the extension-host test emits `out/test/extension.test.js`, workspace dependencies are bundled, and only VS Code plus Node host modules remain external. The manifest's absent legacy icon was replaced with the package-owned `🖼️icon/🔣️.svg`; test harnesses and test output are excluded from the VSIX.

The package-root `LICENSE.md` remains byte-for-byte and path-for-path fixed. No README or LICENSE projection was applied.

## Test-first authority

- Language-neutral authority: `🧪️vscode-extension-package-identity/🔣️.json`
- Portable Bun test: `🧪️vscode-extension-package-identity/🟦️.ts`
- Authority SHA-256: `30e40d789e17fe1bc9b702ef11ed0779fbd521d2c3dc5f7d25fb143a6a12050f`
- Fixed LICENSE SHA-256: `40edde47d31a21b2995b888a0b259f55b80d7a872a9b228820e96e09b6dc5829`

The test was exercised red-to-green at each boundary: it first rejected the scoped VSCE name and failed VSCE listing; it then reproduced the `index.html` build failure; finally it rejected the obsolete live installer targets. The fixes made each boundary green without weakening the fixed LICENSE assertion.

## Publisher verification

```text
bun pm pack --dry-run --ignore-scripts
```

Passed. The output is `repo-vscode-0.0.1.tgz` and includes `LICENSE.md`.

```text
bunx vsce ls --no-dependencies
```

Passed after the real build. Exact selected files, independently cross-checked against `.vscodeignore` through the third-party `ignore` package:

```text
LICENSE.md
out/extension.js
package.json
🖼️icon/🔣️.svg
```

```text
bun install --lockfile-only --frozen-lockfile
```

Passed with 1,556 packages and the generated `repo-vscode` workspace key.

```text
bun nx show projects | rg '^@semio-tech/repo-vscode$'
```

Passed and returned the preserved scoped Nx project identity.

```text
bun nx run '@semio-tech/repo-vscode:build'
bun nx run '@semio-tech/repo-vscode:build-vsix'
```

Both passed. The second target emitted `🧩️repo.vsix` with exactly the fixed LICENSE, manifest, CommonJS entry, and semantic SVG icon as extension content.

```text
bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️vscode-extension-package-identity/🟦️.ts' \
  './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️readme-license-owner-authority.test.ts'
```

Passed: ten tests, zero failures, 652 expectations in 14.40 seconds.

The root coordinator independently reran the exact combined command after handoff: ten tests, zero failures, 652 expectations in 5.72 seconds. It also reran `bun nx run '@semio-tech/repo-vscode:build'` successfully; the cached target replayed both Vite builds and Nx reported success. That verification recreated only the two known ignored `out` leaves, which the coordinator removed exactly with `apply_patch` before removing their now-empty directories. No package-root VSIX was present.

The retained ticket verification artifact is `🧩️repo-vscode.vsix`: 163,393 bytes, SHA-256 `822455ee96a07d400799964ee1558f3804c8f2f3d9554f636de2bcb1437009a3`. `unzip -t` reported no archive errors, the embedded identity is `usalu.repo-vscode`, and the embedded LICENSE SHA-256 is the fixed preimage `40edde47d31a21b2995b888a0b259f55b80d7a872a9b228820e96e09b6dc5829`.

## Build diagnosis and stale consumer deletion

The missing entry was caused by invoking Vite without an entry or configuration, which selected the browser `index.html` default. The router now supplies the extension/test entries and exact outputs directly, so no extra permanent configuration script exists.

Once the test bundle became real, Rollup exposed one stale package-local suite importing three functions already removed with Compose. Only that deleted-Compose-dependent import and suite were removed; no Compose tree was restored, enumerated, traversed, or read. The remaining extension-host tests compile into their declared harness output.

The live devcontainer and Windows bootstrap consumers also still invoked nonexistent `repo:build` targets. Both now use `@semio-tech/repo-vscode:{build,build-vsix}` and the current package root; the obsolete historical VSIX ignore path was removed because the repository already ignores `*.vsix` globally.

## Generated-artifact cleanup

Before cleanup, the exact package-root targets resolved beneath the VS Code package root and were independently proven ignored: `out/extension.js` and `out/test/extension.test.js` by the root `out` rule, and `🧩️repo.vsix` by the root `*.vsix` rule. Their sizes were 613,133 bytes, 648,948 bytes, and 164,821 bytes respectively. After retaining the verified ticket VSIX above, only those exact ignored package-root build artifacts were removed.

## Additional diagnostics

`bash -n .devcontainer/post-attach.sh` and `xmllint --noout` on the semantic SVG icon passed. PowerShell Core is unavailable on this host, so the Windows bootstrap received no local PowerShell parser claim. An exploratory `bunx tsc --noEmit -p <package>/tsconfig.json` is not a configured Nx package gate and remains red on pre-existing shared `.ts`-extension/import-meta configuration plus malformed generated GraphQL overload and optional ephemeral-box typing in `🟦️extension.ts`; those diagnostics were not caused or hidden by the Vite/VSCE repair. The bounded Nx build and VSIX publisher gates above are green.

## Scope

Changed the extension manifest, package router, package-local test reference/stale deleted-Compose suite, `.vscodeignore`, semantic SVG icon, Bun lock entries, two exact installation consumers, one redundant stale `.gitignore` entry, README/LICENSE authority metadata, and ticket-local evidence. `📋️project.json` and the fixed `LICENSE.md` were not edited. Actual `compose/**`, `temp/compose/**`, `temp-compose/**`, normalization, transaction files, Git index/state, and physical README/LICENSE paths were not touched.
