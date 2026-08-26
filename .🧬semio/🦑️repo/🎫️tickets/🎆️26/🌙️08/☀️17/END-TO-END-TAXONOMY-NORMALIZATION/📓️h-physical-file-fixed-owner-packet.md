# Physical File Kind and Fixed Contract Owner Packet

## Result

The historical `19` unknown physical leaves and `32` fixed-scope misses are now reproducibly reconciled against current taxonomy v7 and the current Git-admitted, non-Compose tree.

- The historical 19 are the exact `CACHEDIR.TAG` leaves beneath Cargo target-triple directories in the retained pre-transaction-v2 inventory. All 19 remain present. A twentieth target-triple marker is now admitted, and six other current cache markers also lack the required immediate-parent authority. The exact current cache residual is therefore **26**, while **48** other cache markers resolve to `cargo-cache-tag`.
- All historical 32 fixed-scope misses remain present and have an explicit current decision: **9 exact fixed winners** and **23 exact rejection operations**. The unresolved count for that historical ledger is **0**.
- Removing the historical suffix blanket deliberately exposes a much larger ticket-evidence population. Current strict exported resolvers report 182 present regular files with neither a fixed winner nor a global file kind. Of these, 26 are the cache ledger below, two are exact scratch-Go rejection operations, four are small live-owner decisions, and 150 are retained ticket evidence. The 150 must not receive global extension kinds and are outside this small owner packet.
- The current fixed-looking miss census is 108: 23 are intentional exact rejections and 85 are real current physical decisions. The 85 partition exactly into 26 Cargo cache markers, 19 package/configuration manifests, and 40 nested README/LICENSE documents.

No production, schema, test, script, physical path, or Git state was changed. Actual `compose/**`, `temp/compose/**`, and `temp-compose/**` were excluded lexically from Git enumeration before any filesystem inspection and were not traversed or read.

## Digest-bound observation

Observation: `2026-08-26T19:20:01.191Z`.

| Datum | Value |
| --- | --- |
| Taxonomy schema | v7 |
| `validateTaxonomy()` | 0 problems |
| Taxonomy raw SHA-256 | `db5bc86a4c2c4102e8af93ffa9be4fba3177da15548b97fcf533e32337992a6a` |
| NFC, UTF-8-byte-sorted, NUL-delimited admitted paths | 64,946; `5e50f24a40cecde5f89dbfed0466dc8c82b1e547c5328c767699082ca5a3f4c1` |
| Present regular files | 64,926; `b1aba7bd912f887cc8517807e82ab52da36e28766fd6b090ce5ac3468ed74b30` |
| Historical 19 target-triple cache paths | `fd09bc41ce3e2a144dce4e49eb7d194d81d180ff3163fc5660dcb9bbaa5c038a` |
| Current 26 unresolved cache paths | `7e82a3634ea9a9b2ccbe12cb6430e0b2b047675840389d5fbfb51422beb9bce8` |
| Historical 32 fixed-scope identities | `ccc5c009adced44b42eca8607ba9fabdce6321f87105434ef5d7633c122f3b8a` |
| Current 85 non-rejection fixed-looking misses | `fcd054ddcda97392821e6e30a608db6eb635e98f8283b373225710be96bb08f0` |
| Four small no-kind live-owner paths | `be19e0b9be6ad2e60d05244593754d4bd4096d3ef5aeeb1fdaef4ec246a696a4` |

This report itself was not present in the admitted-path digest. Concurrent ticket reports added between earlier probes explain why this path count is higher than prior ticket reports; no counts are compared without a digest.

## Historical 19 unknown physical leaves

The retained canonical inventory provides the missing identity ledger. These exact 19 paths all end in a Cargo-owned fixed name but are not direct children of the registered `ticket-cargo-target-evidence` directory kind:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/🧪️target-demonstrator-wasm/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/🧪️target-flow-wasm/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/🧪️target-gis-contract/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/🧪️target-gis-contract/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-0-INTERACTIVITY-OBSERVABILITY-AND-DEPENDENCY-FREEZE/🧪️target-p0-current-unknown/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-0-INTERACTIVITY-OBSERVABILITY-AND-DEPENDENCY-FREEZE/🧪️target-p0-current-wasip2/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-async-wasi/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-async-wasm/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-p1-process-pool/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-p1-process-pool/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-2-JOB-PROTOCOL/🧪️target-p2b-current-unknown/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-2-JOB-PROTOCOL/🧪️target-p2b-current-wasip2/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-2-RESUMABLE-JOB-AND-PROGRESS-PROTOCOL/🧪️target-p2b-current-unknown/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-2-RESUMABLE-JOB-AND-PROGRESS-PROTOCOL/🧪️target-p2b-current-wasip2/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️target-p9-objc2-metal/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/🧪️target-p7-energy-focused/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/🧪️target-p7-energy-focused/wasm32-wasip2/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-p1-finite-drivers/wasm32-unknown-unknown/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/PHASE-1-ONE-POOL-WORKER-RUNTIME/🧪️target-p1-finite-drivers/wasm32-wasip2/CACHEDIR.TAG
```

The new twentieth target-triple path is:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-FEM-JOB-GRAPH/🧪️target-sol-fem-wasip2-final/wasm32-wasip2/CACHEDIR.TAG
```

The other six current cache misses are three unregistered historical target roots and three retained leaked transaction fixtures:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/FIX-PUZZLE-3D-ACTION-PANE-MANIFEST-CRASH/cargo-target/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/OVERHAUL-TEST-SUITES-TO-30S-BUDGET-PER-APP/scratch-fem-3d-target/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/PUZZLE-3D-SELECTION-AND-TOOLS-OVERHAUL/cargo-target/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️s-test-transaction-embedded-v2-lyq6TZ/🧪️tests/🧪️fixture/pkg-a/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️unique-a/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️s-test-transaction-embedded-v2-lyq6TZ/🧪️tests/🧪️fixture/pkg-b/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️unique-b/CACHEDIR.TAG
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️s-test-transaction-embedded-v2-lyq6TZ/🧪️tests/🧪️fixture/pkg-c/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/🧪️unique-c/CACHEDIR.TAG
```

### Classification and minimal closure

| Cohort | Count | Category | Owner decision |
| --- | ---: | --- | --- |
| Cargo target-triple child (`wasm32-unknown-unknown`, `wasm32-wasip2`) | 20 | Unconfigurable exact contract | Cargo owns both external directory names and `CACHEDIR.TAG`; admit only when the triple directory is an exact child of registered `ticket-cargo-target-evidence`. Preserve paths. |
| Unprefixed `cargo-target` / `scratch-fem-3d-target` | 3 | Generated/evidence requiring semantic relocation or removal | No unique registered semantic directory authority. Do not infer an emoji or bless these roots. Ticket owner must retain through an exact evidence manifest or remove them transactionally. |
| Leaked `unique-a/b/c` fixture repositories | 3 | Generated/evidence | Retained execution evidence, not Cargo target-root precedent. Do not expand `ticket-cargo-target-evidence`; keep them blocked from production normalization or relocate the entire exact fixture authority. |

The smallest reusable schema change for the 20 target-triple cases is:

1. Add two fixed-directory contracts for literal `wasm32-unknown-unknown` and `wasm32-wasip2`, each conjunctively scoped to immediate parent directory kind `ticket-cargo-target-evidence` and the governed ticket path grammar.
2. Extend the fixed-filename scope tagged union with `fixed-directory-contract`, or add a second cache filename contract whose parent must resolve to one of those two fixed-directory contracts. Do not authorize by `**/wasm32-*/CACHEDIR.TAG` alone.
3. Keep the existing direct-parent `cargo-cache-tag` unchanged for its 48 winners.
4. Add a counterfeit negative outside a ticket, below an unregistered `🧪️target-*`, and below an unregistered target triple.

There are no moves, reference edits, or collisions for the 20: schema admission preserves their exact paths. The other six have no authorized destination, so collision analysis must remain blocked rather than inventing one.

## Historical 32 fixed-scope misses

### Nine current exact winners

| Current path | Winning contract |
| --- | --- |
| `.cargo/config.toml` | `cargo-workspace-config` |
| `.codex/config.toml` | `codex-workspace-config` |
| `pyproject.toml` | `root-python-tooling` |
| `tsconfig.json` | `root-typescript-config` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/tsconfig.json` | `window-kits-typescript-config` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/go.mod` | `repo-cli-go-module` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/go.mod` | `repo-mcp-go-module` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/go.mod` | `repo-library-go-module` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/go.mod` | `repo-coordinator-go-module` |

These are complete. They are unconfigurable exact contracts, preserve their current paths, require no reference edit, and create no collision.

### Four exact normalize rejections

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️20/INVESTIGATE-SLOW-INTEGRATION-TESTS-IN-REPO-CLI-GO/scratch/gitignore-check/go.mod
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️20/INVESTIGATE-SLOW-INTEGRATION-TESTS-IN-REPO-CLI-GO/scratch/gitignore-check/go.sum
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️01/TOTAL-JSON-PURGE/progress.md
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️03/HANDCRAFTED-GRAMMAR-FOR-EVERY-ARTIFACT/progress.md
```

The first pair is retained evidence, not an active Go module; it must normalize as evidence rather than acquire Go authority. The progress documents are physical Markdown that require semantic relocation, but no unique `progress` directory emoji precedent exists. Their rejection identities are correct; their destinations remain an explicit owner decision.

### Nineteen exact relocate rejections

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️12/OS-APP-PLAYGROUND-UI-REFACTOR/OS-REFACTOR-FOUNDATIONS-ENFORCEMENT-INFRA/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/GENERALIZE-APPS-ONTO-FRAMEWORK-PRIMITIVES/EXTRACT-SHARED-PLUGIN-SDK-PRIMITIVES-AND-DE-APP-FRAMEWORK-PLUGIN/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/GENERALIZE-APPS-ONTO-FRAMEWORK-PRIMITIVES/GENERALIZE-VCS-HISTORY-PROTOCOL-LIST-SURFACES-TO-GRAPH-TIMELINE-BLOCK-LIST/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/FRAMEWORK-HOSTS-WASM-STORYBOOK-STORIES/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/FRAMEWORK-OS-PLUGINS-AND-WGPU-STORYBOOK-STORIES/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FEATURE-COMPLETE-COMPOSABLE-STORYBOOK-COVERING-THE-MONOREPO/STYLING-SCOPE-STORYBOOK-STORIES/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FULL-PHOTOGRAMMETRY-AND-VIDEOGRAMMETRY-STACK-FOR-REMODEL/REWRITE-REMODEL-DOCUMENT-SCHEMA-FOR-THE-FULL-PHOTOGRAMMETRY-PIPELINE/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/FULL-PHOTOGRAMMETRY-AND-VIDEOGRAMMETRY-STACK-FOR-REMODEL/REWRITE-REMODEL-PLUGIN-TO-WIRE-THE-FULL-PHOTOGRAMMETRY-STACK-INTO-THE-APP-UI/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️19/NETWORK-X-PARITY-FOR-MATHEMATICAL-GRAPH-CRATES/MATHEMATICAL-ALGEBRA-SPARSE-MATRICES-AND-EIGENSOLVERS-FOR-NETWORK-X-PARITY/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/CONSTITUTIONAL-SPLIT-ANIMATE-PRESENT-APP/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️29/MOVE-ALL-APPS-INTO-THE-S-PRODUCT-TREE-WITH-CONSTITUTIONAL-CRATES-EMOJI-LAYOUT/FIX-DSL-KERNEL-LIST-OF-RECORDS-NODE-COUNT-OVERFLOW/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/OWNED-UI-AND-TOOLING-STACK/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-0-INTERACTIVITY-OBSERVABILITY-AND-DEPENDENCY-FREEZE/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-ONE-POOL-WORKER-RUNTIME/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-2-RESUMABLE-JOB-AND-PROGRESS-PROTOCOL/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-3-UI-THREAD-ISOLATION/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-5-RESUMABLE-FRAME-TRANSACTION-AND-RENDERER/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-FEM-JOB-GRAPH/🎫️ticket.json
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/RESUMABLE-WFC-PUZZLE2D-AND-ENERGY-JOBS/🎫️ticket.json
```

These are real repo-MCP manifests in invalid nested owner positions, not generic JSON leaves. The current `nested-ticket-manifests` rejection contract is the correct fail-closed classification. Physical destinations, reference edits, and collisions belong to the separate embedded-ticket transaction authority and must not be guessed by a fixed-filename adapter.

## Current additional owner-packet decisions

### Configurable repository entry

Exact path:

```text
.🧬semio/🦑️repo/compose-micro-commit-bun
```

It contains one Bun executable path plus newline. This is internally configurable repository metadata, not an external fixed filename and not a new global no-extension file kind.

Current owner references:

- repo library TypeScript `📦️index.ts`, around line 4601: `MICRO_COMMIT_BUN_PIN = "compose-micro-commit-bun"`;
- repo library TypeScript test, around line 751: exact installation-path assertion;
- repo CLI Go `🐹️component.go`, around line 38642: currently reads the different filename `🐹️compose-micro-commit-bun`.

The TypeScript writer/current file and Go reader already disagree. The owner must freeze one canonical semantic destination and update both implementations plus the test atomically. No unique semantic directory precedent currently authorizes a destination, so this packet does not invent one. The clean schema mechanism is a repository-metadata configurable-entry contract tied to the `semio-governance` owner, a plain-text physical leaf, and exact TypeScript/Go configuration references. Collision checking is blocked until that destination is registered.

### Unconfigurable external filenames currently prefixed

| Source | Required external basename | Current exact references | Collision |
| --- | --- | --- | --- |
| `♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🌐️public/🌐️CNAME` | `CNAME` | Static-deploy writer and documentation in `vite-elements-assets.ts` use `🌐️CNAME`; manifest API documentation repeats it. Update writer/tests/docs to `CNAME`. | Destination absent |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🌐️Caddyfile` | `Caddyfile` | No live exact path-token consumer found outside retained evidence; validate the Caddy launch/config owner. | Destination absent |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/🐳️Dockerfile` | `Dockerfile` | Two self-documentation strings in the file and the repo CLI file-kind test around line 2280 use the prefixed name. | Destination absent |

The existing `github-pages-cname`, `caddyfile`, and `dockerfile` contracts already supply the authority. Move to their exact basenames; do not add emoji-prefixed fixed aliases. All three destinations are shorter and collision-free.

### Nineteen package/configuration manifests

These exact current fixed-looking misses require package-owner authority rather than physical file-kind emojis:

```text
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️15/BOARD-REACT-RECONCILER/_tmp/package.json
♻️mit-bestand/🧺️demonstrator/package.json
✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🦀️rust/package.json
✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/package.json
🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/package.json
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json
🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/package.json
🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/package.json
🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/package.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/package.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/package.json
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/package.json
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/package.json
🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/package.json
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/tsconfig.json
🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml
🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/Cargo.toml
🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest/Cargo.lock
```

Classification:

- The ticket `_tmp/package.json` is retained generated/evidence and has no live Nx owner; normalize/remove it through ticket evidence authority.
- The other 13 `package.json` files each have an adjacent `📋️project.json` and a real Node/Nx package identity. They are unconfigurable exact manifests. The smallest reusable scope is an exact sibling-owner scope requiring the adjacent winning `nx-project-manifest`; it must outrank but not overlap the ordinary TypeScript package-root manifest contract.
- The two nested React `tsconfig.json` files are exact TypeScript configuration owned by those same adjacent Nx/Node project roots. Use the same conjunctive owner scope; do not broaden `**/tsconfig.json`.
- The three Cargo files require semantic package relocation rather than an in-place exception. Their exact destinations, 191 WGPU live reference tokens, three JCO live edits, collision/path-budget proof, and generator owners are already frozen in `📓️h-nested-cargo-package-authority.md`. Do not duplicate or weaken that projection with a broad Cargo contract.

The 16 Node/TypeScript manifests preserve their paths and therefore add no move collision. Any fixed scope must require adjacent owner identity, not merely `package.json`/`tsconfig.json` basename. Cargo uses the previously designed atomic package moves.

### Forty nested README/LICENSE documents

Current exact count: 32 `README.md` and 8 `LICENSE.md`. They are intentionally outside the repository-root-only fixed contracts. They split into ordinary owner documentation, package-publication-sensitive documentation, third-party attribution, and ticket scratch. No single broad fixed rule is legitimate.

Exact README paths:

```text
.devcontainer/README.md
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️14/NEO4J-DEVCONTAINER-SETUP/README.md
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️04/move-ui-framework-folders/README.md
.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️23/END-TO-END-TESTING-REFACTOR/w7-md-commonmark-scratch/README.md
✏️s/🔌️plugins/✒️writer/README.md
✏️s/🔌️plugins/➗️mathematical/README.md
✏️s/🔌️plugins/🌀️procedural/README.md
✏️s/🔌️plugins/🌊️flow/README.md
✏️s/🔌️plugins/🌍️gis/README.md
✏️s/🔌️plugins/🎥️shooting/README.md
✏️s/🔌️plugins/🏗️fem/README.md
✏️s/🔌️plugins/💡️reasoning/README.md
✏️s/🔌️plugins/📐️cad/README.md
✏️s/🔌️plugins/🔱️trinity/README.md
✏️s/🔌️plugins/🧩️puzzle/README.md
✏️s/🔨️modules/README.md
🧰️framework/🔨️modules/🖱️ui/README.md
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/README.md
🧰️framework/🔨️modules/🖱️ui/🖼️assets/README.md
🧰️framework/🔨️modules/🖼️assets/README.md
🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/README.md
🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/README.md
🧰️framework/🛍️products/💻️os/🧫️fixtures/README.md
🧰️framework/🛍️products/🦑️repo/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🔌️mcp/🔗️graphql/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🪶️sqlite/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🐹️go/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🧬️schema/🐘️postgres/README.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/README.md
```

Exact LICENSE paths:

```text
🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/LICENSE.md
🧰️framework/🔨️modules/🖱️ui/🖼️assets/LICENSE.md
🧰️framework/🔨️modules/🖼️assets/LICENSE.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/LICENSE.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🖼️assets/LICENSE.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/LICENSE.md
🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/LICENSE.md
🧰️framework/🛍️products/🦑️repo/🖼️assets/LICENSE.md
```

Default disposition is semantic relocation to a registered owner-specific documentation/license directory plus physical `📝️.md`. No unique repository-wide emoji precedent currently authorizes those directory identities, so the schema owner must split the cohort using actual publication/attribution manifests rather than inventing one. Package-root README/LICENSE cases may receive exact owner-local fixed contracts only where the package publisher or VS Code packager demonstrably requires the basename. Ticket scratch remains generated/evidence. Reference edits and collision projections are blocked until those owner identities/destinations are frozen.

## Current accounting by requested category

| Category | Exact current population in this packet | Count |
| --- | --- | ---: |
| Configurable entry | Micro-commit Bun pin | 1 |
| Unconfigurable exact contract, already resolved | Historical fixed winners | 9 |
| Unconfigurable exact contract, needs owner scope or external basename move | Target-triple cache 20; prefixed external files 3; Node/TypeScript manifests 15 | 38 |
| Generated/evidence | Unprefixed/leaked cache 6; ticket `_tmp/package.json` 1; scratch Go rejection 2; ticket progress 2 | 11 |
| Semantic relocation | Nested ticket manifests 19; Cargo package files 3; nested README/LICENSE 40; progress documents are already counted as evidence pending owner destination | 62 plus the two pending progress destinations |

The categories overlap only where a retained-evidence source still needs a semantic destination; operation accounting must choose one final disposition per source. The fixed-scope ledger itself remains disjoint: `32 = 9 fixed + 23 rejected`.

## Minimal TDD packet

Schema/discovery tests:

1. Resolve all 48 direct `cargo-cache-tag` paths and all 20 target-triple paths through distinct exact parent authorities.
2. Reject the three unprefixed roots, three `unique-*` fixtures, a production lookalike, a target triple under a non-ticket root, and a marker with no parent authority.
3. Reassert the historical ledger exactly: 32 present, 9 unique winners, 23 exact rejections, zero overlap.
4. Resolve each of the 13 project-root `package.json` and two project-root `tsconfig.json` files only with the adjacent winning `📋️project.json`; reject an adjacent counterfeit filename and a similar directory without an Nx project identity.
5. Preserve repository-root README/LICENSE as the only global fixed winners; all 40 nested documents remain unresolved until an owner-local contract/projection is added.
6. Prove prefixed `🌐️CNAME`, `🌐️Caddyfile`, and `🐳️Dockerfile` are not aliases; only the exact external basename resolves.
7. Third-party parity: use `fast-glob` only to verify the candidate path patterns. Exact scope/owner identity remains the in-repo resolver's authority.

Owner tests:

1. Static deploy writes `CNAME`, never `🌐️CNAME`, and a GitHub Pages fixture accepts the exact output.
2. Docker/Caddy launch checks consume the unprefixed destination names.
3. TypeScript and Go micro-commit implementations read/write the same registered path; missing/stale pin content fails with one actionable diagnostic.
4. Cargo package relocation uses the existing golden/reference matrix from `📓️h-nested-cargo-package-authority.md` rather than an in-place exception.
5. A second normalization inventory/plan is empty for every implemented exact cohort.

## Blockers

1. The six unprefixed/leaked cache paths have no legitimate registered target-root owner. They must not inherit the 20-path Cargo target-triple authority.
2. The micro-commit Bun pin has conflicting TypeScript and Go filenames and no registered semantic destination.
3. The nested README/LICENSE set mixes package publication, third-party attribution, ordinary owner documentation, and ticket scratch. One global fixed or semantic contract would be false authority.
4. The three Cargo paths require package relocation with 194 live reference edits/regenerations already documented elsewhere; an exact fixed contract in place would preserve the wrong owner hierarchy.
5. The 150 other current unknown ticket-evidence files were intentionally exposed when `scopedFileKinds` became empty. They require an exact retention-manifest projection, not global registration of `.pid`, backup, fragment, template, profile, or intermediate-build suffixes.
6. Final transaction-v2 inventory has not yet replaced the retained pre-v2 evidence. Acceptance must rerun against the final source/taxonomy digests.

## Evidence commands

```text
git ls-files -z --cached --others --exclude-standard -- . \
  :(exclude)compose :(exclude)compose/** \
  :(exclude)temp/compose :(exclude)temp/compose/** \
  :(exclude)temp-compose :(exclude)temp-compose/**

bun -e '<loadTaxonomy + validateTaxonomy + exported fileKind/fixed/rejection resolver census>'

jq -r '.entries[] | select(.sourcePath | endswith("/CACHEDIR.TAG")) | .sourcePath' \
  📊️taxonomy-inventory/🔣️.json

rg -n -F '<exact live path token>' \
  --glob '!compose/**' --glob '!temp/compose/**' --glob '!temp-compose/**'
```

Every `lstat` or content read occurred only after the admitted path ledger passed the lexical opaque-prefix assertion.
