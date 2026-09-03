# WGPU Path Authority Repair

## Outcome

The authoritative WGPU package root is now:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`

Target-level authored Rust, browser entries, runtime bridges, and tests remain under the WGPU target above that package boundary. The active repaired scope contains zero references to the executable legacy package root.

## Implementation

- Moved only the tracked legacy `🟦️typescript/🐚️plugin-bridge.ts` into the canonical package. A byte comparison against the legacy `HEAD` blob passed.
- Removed only the tracked obsolete legacy Vitest config.
- Repointed `📋️project.json` atomically: one `sourceRoot` plus all sixteen target `cwd` values now name the canonical package.
- Split browser inputs from package outputs in the package router:
  - authored input: `../../🧵️browser-boot/🟦️.ts`;
  - authored input: `../../🧵️frame-worker/🟦️.ts`;
  - generated output: `🟦️typescript/🟨️boot.js`;
  - generated output: `🟦️typescript/🟨️frame-worker.js`.
- Routed all three Vitest invocations to the canonical package config and corrected browser/preview selectors to the three real target-level tests.
- Kept Vitest's runtime root at `../..`. Verification showed that Vitest resolves this setting from the package working directory: `../../../..` selected `…/engine`, while `../..` selects the intended `…/targets/🧊️wgpu`.
- Repaired target test imports and their source-inspection paths.
- Repaired renderer boot and frame-worker imports without adding compatibility barrels:
  - plugin catalog → plugin registry;
  - worker scheduler/descriptors → interactive-job registry;
  - plugin bridge → canonical package;
  - diagram worker codec → UI Diagram owner;
  - interactive port authority → UI Ports owner.
- Corrected the package export, repository directory, lint source, and Trunk watches.
- Semantically mapped every active legacy renderer path in root `📜️script.ts`, including the two native launch-command expectations.
- Regenerated `🔒️dependencies.json` through `bun ./📜️script.ts verify dependencies write-baseline`.
- Regenerated and verified launch output through the plugin-registry Nx generator. No launch seed or generated launch output was hand-edited.

## TDD and Verification

### Required red

`bun nx run @semio-tech/framework-renderer-wgpu:test-quick`

- exit: 1;
- diagnostic: `Module not found "./📜️script.ts"`;
- cause: Nx used the physical but non-executable legacy working directory.

### Passing final WGPU/package checks

| Check | Result |
| --- | --- |
| `generate-frame-worker` | pass |
| `check-browser-worker` | pass |
| `check-frame-worker` | pass |
| `test-browser-worker` | 2 files, 32 tests passed |
| `test-preview-generated` | 1 file, 9 tests passed |
| `preview-generated` | pass, 16,109,697-byte protocol capture |
| dependency baseline write | pass, 175 third-party dependencies |
| dependency verification | pass, current 175 equals baseline 175 |
| plugin-registry launch generation | pass, 59 plugin crates, 60 playgrounds, 45 framework packages |
| plugin-registry generated/launch freshness | pass |
| scoped `git diff --check` | pass |
| canonical Nx root count | 17 of 17 |
| active old-root references in root script plus WGPU target | 0 |
| moved bridge byte comparison | pass |

The WGPU-owned browser result is 41 passed and 0 failed tests across three files.

### External or bounded blocks

1. The repaired canonical quick target reaches Cargo, but Cargo stops before Vitest with four external `E0277` diagnostics in `semio-framework`: `dsl::SelectionSpec` lacks `serde::Serialize` and `serde::Deserialize` (one serialize and three deserialize diagnostics). Result: quick target blocked, 0 tests executed.
2. `bun install --lockfile-only` stops before lock generation because root `package.json:21` names the missing workspace `🧰️framework/🔨️modules/◻2d/📦️packages/🟦️typescript`; the physical workspace uses `◻️2d`. The two legacy WGPU records therefore remain in `bun.lock`.
3. `verify interactivity apps` ran 25 self-tests and found launch coverage for all eight declared apps, but failed on 761 broader discovery/capacity findings, beginning with 555 generated launch configurations exceeding capacity 512 and 12,055 descriptors exceeding capacity 256.
4. The wasm/Trunk target reached canonical worker generation and compiled the canonical WGPU crate and dependencies after waiting on shared Cargo locks. It was deliberately bounded and interrupted with exit 130 before the workspace build completed. Native smoke was not started because it exercises the same occupied/external Cargo lane.

## Preserved Boundaries

- The legacy tree was not recursively deleted.
- The pre-existing ignored legacy `🟨️boot.js`, local caches, historical tickets, and unrelated generated material were not moved or removed.
- Taxonomy `sourceManifestPath`, its legacy source-module projection records, and nested-Cargo fixtures remain unchanged by this packet. The canonical `destinationManifestPath` remains intact.
- The task-owned `🗑️generated/sol-wgpu-preview.json` capture was removed. Concurrent generated directories in the umbrella ticket were preserved.
- No downstream serde implementation, missing `◻2d` workspace metadata, or unrelated interactivity descriptors were edited.
