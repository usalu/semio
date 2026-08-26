# S-Preview Launch Reproducibility

## Outcome

The 14 owned generator previews are now first-class VS Code launch entries in the existing `4_build` sequence. The plugin-registry owner regenerated `.vscode/launch.json`; a permanent owner test proves every taxonomy `previewTarget` appears exactly once with the exact name, command, cwd, group, and order.

`wgpu-frame-worker` generation now fails closed unless the executing Bun version equals the exact root `packageManager` pin. The pin is aligned to the renderer used for the checked bytes (`bun@1.3.14`). Devcontainer and native provisioning derive that version from `package.json`; neither carries a second version literal.

Compose and `temp/compose` were neither read nor modified. Git state was not modified.

## Launch Contract

The entries immediately follow `@semio-tech/plugin-registry:generate` at order `206` and precede taxonomy generation at order `207`:

| Order | Contract | Target |
|---:|---|---|
| 206.01 | `actor-typegen` | `@semio-tech/framework-actor-rs:preview-generated` |
| 206.02 | `assets-build` | `@semio-tech/assets:preview-generated` |
| 206.03 | `async-typegen` | `@semio-tech/framework-async-rs:preview-generated` |
| 206.04 | `framework-manifest` | `@semio-tech/framework-rs:preview-generated` |
| 206.05 | `graph-catalog` | `@semio-tech/framework-graph:preview-generated` |
| 206.06 | `plugin-registry` | `@semio-tech/plugin-registry:preview-generated` |
| 206.07 | `print-latex-tokens` | `@semio-tech/print:preview-generated` |
| 206.08 | `scale-fixture` | `@semio-tech/framework-os-dev:preview-generated` |
| 206.09 | `schema-entity-catalog` | `@semio-tech/framework-schema:preview-generated` |
| 206.10 | `shell-typegen` | `@semio-tech/framework-os-shell-rs:preview-generated` |
| 206.11 | `styling-tokens` | `@semio-tech/ui-styling-tokens:preview-generated` |
| 206.12 | `ui-axes` | `@semio-tech/ui-rs:preview-generated` |
| 206.13 | `ui-contract` | `@semio-tech/ui-contract-rs:preview-generated` |
| 206.14 | `wgpu-frame-worker` | `@semio-tech/framework-renderer-wgpu:preview-generated` |

Every entry is named `📦️preview🤖️<contractId>`, runs `bun nx run <previewTarget>`, uses `${workspaceFolder}`, and belongs to `4_build`.

## Reproducibility Contract

- `assertPinnedBunVersion()` reads the root `packageManager`, requires exact `bun@x.y.z`, compares it to `Bun.version`, and reports both required and actual versions on mismatch.
- Every in-memory WGPU browser bundle calls that guard before `Bun.build`.
- The root pin changed from `bun@1.2.5` to `bun@1.3.14`, matching the Bun runtime that rendered and verified the current bytes.
- `.devcontainer/Dockerfile` no longer installs floating latest Bun during image construction.
- `.devcontainer/post-create.sh` reads the exact pin with `jq`, installs `bun-v<required>` only when missing/mismatched, re-probes, and fails with an actionable diagnostic if it still differs.
- Native `🥾️bootstrap/⌨️script.sh` extracts the same root pin, installs that exact release on macOS/Linux, re-probes, and fails with the required/actual versions if resolution fails.
- The existing submodule loop variable in `post-create.sh` is now task-specific (`submodule_path`).

The owner test renders the worker twice in memory, compares the full artifacts, and cross-checks the output SHA-256 through Node `createHash` and WebCrypto `crypto.subtle`. It also proves the provisioning files contain no duplicate `1.3.14` literal and validates both shell files with `bash -n` on non-Windows hosts.

## Commands and Evidence

### Failing first

After correcting the new test's relative import, before regeneration:

```text
bun nx run @semio-tech/plugin-registry:test --skipNxCache
FAIL 1/1
📦️preview🤖️actor-typegen: expected [] to deeply equal [exact launcher]
```

Importing the WGPU owner before it was made module-safe also failed because its default router executed during the test. The router is now guarded by `import.meta.main`, allowing the exact renderer to be reused by the test without executing a command.

### Owner regeneration

```text
bun nx run @semio-tech/plugin-registry:generate --skipNxCache
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 38 framework packages)
.vscode/launch.json regenerated
NX Successfully ran target generate for project @semio-tech/plugin-registry
```

The registry generated-root digest remained unchanged across regeneration; only the launch output changed as intended:

```text
before launch  b5e6d52ff17a3ceacbe87d3c42392c6d0caf814f0f27ff6e36b4127bf812b723
after launch   703850ccbbf0998dabc8752da56d685a3b9175a8f39f65acdaa0ec6c258ba5ce
generated root 5a2372581b78e55a3726413930827cfb6c10a570c26eb73b882fa70137db235d (8 files, before and after)
```

### Focused permanent tests

```text
bun nx run @semio-tech/plugin-registry:test --skipNxCache
Test Files 1 passed (1)
Tests 1 passed (1)
Duration 450ms; tests 8ms

bun nx run @semio-tech/framework-renderer-wgpu:test-preview-generated --skipNxCache
Test Files 1 passed (1)
Tests 7 passed (7)
Duration 750ms; tests 35ms
```

Nx labeled the registry test flaky because its earlier failing-first execution and later green execution shared the task identity; the final uncached execution passed.

Targeted `git diff --check` exited `0`; root/project JSON and launch seed/output JSONC all parsed successfully.

### Preview immutability

Plugin registry:

```json
{"schemaVersion":1,"contractId":"plugin-registry","nodes":10,"staleRemovals":0,"canonical":true,"sha256":"e61d3e0dc2b1309c1c2ce0f1b4ba04d9b3be3c83367c87f8bbcf8a4110f55916"}
```

WGPU frame worker:

```json
{"schemaVersion":1,"contractId":"wgpu-frame-worker","nodes":1,"staleRemovals":0,"canonical":true,"decodedSha256":"e9aacb469938553608d11e6083843e7ddd185e568bc17de1a0b501451b608f8b","manifestSha256":"10a901a86bb6a5b1b4017b95c72915a80f676225d9920a0066b72f0afb45c0c4"}
```

The physical `🟨️frame-worker.js` hash was `e9aacb469938553608d11e6083843e7ddd185e568bc17de1a0b501451b608f8b` both before and after preview. The final plugin-registry hashes after preview and check remained:

```text
.vscode/launch.json 703850ccbbf0998dabc8752da56d685a3b9175a8f39f65acdaa0ec6c258ba5ce
generated root      5a2372581b78e55a3726413930827cfb6c10a570c26eb73b882fa70137db235d (8 files)
```

### Check result and independent blocker

```text
bun nx run @semio-tech/plugin-registry:check --skipNxCache
exit 1
plugin taxonomy tree violations (area(s) "✏️s/🔌️plugins" is "clean")
```

The check passed its generated catalog and `.vscode/launch.json` byte-freshness block, then failed at the subsequent repository-wide taxonomy-tree gate. The physical plugin tree is not yet normalized although the area is declared `clean`; representative findings are legacy `🦀️component.rs` leaves and missing required artifact/surface directories. This lane did not weaken the gate or modify plugin trees. The failure is independent of launch generation and is expected to clear only after the transactional taxonomy apply.

## Touched Paths

- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json` (owner-generated)
- `package.json`
- `.devcontainer/Dockerfile`
- `.devcontainer/post-create.sh`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/⌨️script.sh`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️launch.test.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️vitest.config.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧪️index.test.ts`

## Acceptance Checks

- [x] All 14 owned preview targets have exact launch entries in stable build order.
- [x] `.vscode/launch.json` was regenerated only through the plugin-registry owner.
- [x] Exact launcher contract has a permanent focused Nx test.
- [x] WGPU preview/generation rejects an unpinned or mismatched Bun runtime.
- [x] Root package-manager pin matches the verified renderer runtime.
- [x] Devcontainer and native provisioning derive one exact version without a duplicate literal.
- [x] WGPU bytes are deterministic across repeated in-memory rendering with independent digest parity.
- [x] Registry and WGPU previews are canonical and leave output roots unchanged.
- [ ] Full plugin-registry check passes; blocked by the separately owned physical taxonomy migration after freshness succeeds.
