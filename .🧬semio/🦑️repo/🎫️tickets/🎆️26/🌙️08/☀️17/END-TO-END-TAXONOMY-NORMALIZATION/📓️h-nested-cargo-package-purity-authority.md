# Nested Cargo Package Purity Authority

## Decision

The 36 mappings in the nested Cargo structural authority are co-plannable with package purity without moving any production file in this packet.

- Structural population: 32 WGPU leaves plus 4 JCO leaves.
- Purity population: 24 WGPU implementation leaves, 2 WGPU structurally unresolved leaves with exact external authority, and 1 JCO implementation leaf.
- Structural-only remainder: 9 leaves.
- Derived package adapters: 5.
- Open ambiguity: 0.
- Purity destination maximum: 194 UTF-8 bytes.
- Full composed maximum including derived adapters: 196 UTF-8 bytes.
- Budget: 240 UTF-8 bytes.

The language-neutral authority is 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️nested-cargo-package-purity/🔣️.json. It pins the structural golden by SHA-256, pins every purity source by SHA-256, and declares fail-closed rejection for source drift, role drift, missing registration, missing boundary contract, destination collision, Unicode/casefold collision, path overflow, generator-owner mismatch, implementation left under a package, or opaque-root access.

No production, schema, normalization, transaction, Git, or Compose content was changed or traversed by this authority packet.

## Exact Purity Destinations

W is 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🎯️targets/🧊️wgpu.

P is W/📦️packages/🦀️rust.

| Source relative to current WGPU boundary | Source role | Exact projected destination | Ownership |
| --- | --- | --- | --- |
| build.rs | implementation | W/🏗️builder/🦀️.rs | authored |
| 🎠️runtime.rs | implementation | W/🎠️runtime/🦀️.rs | authored |
| 📦️bin.rs | implementation | W/⌨️native-entrypoint/🦀️.rs | authored |
| 📦️glue.rs | implementation | W/🧊️renderer/🦀️.rs | authored |
| 📦️index.ts | implementation | W/🧊️renderer-boot/🟦️.ts | authored |
| 🟦️typescript/🐚️plugin-bridge.ts | implementation | W/🐚️plugin-bridge/🟦️.ts | authored |
| 🟦️typescript/🟦️boot.ts | implementation | W/🧵️browser-boot/🟦️.ts | authored |
| 🟦️typescript/🟨️frame-worker.js | implementation | W/🧵️frame-worker/🤖️generated/🟨️.js | generated; regenerate |
| 🟦️typescript/🧵️browser-frame-transport.ts | implementation | W/🧵️browser-frame-transport/🟦️.ts | authored |
| 🟦️typescript/🧵️browser-interactive-job-port.ts | implementation | W/🧵️browser-interactive-job-port/🟦️.ts | authored |
| 🟦️typescript/🧵️frame-worker.ts | implementation | W/🧵️frame-worker/🟦️.ts | authored |
| 🟦️typescript/🧵️interactive-job-registry.ts | implementation | W/🧵️interactive-job-registry/🟦️.ts | authored |
| 🦀️browser_worker.rs | implementation | W/🧵️browser-worker/🦀️.rs | authored |
| 🦀️deadlines.rs | implementation | W/⏰️deadlines/🦀️.rs | authored |
| 🦀️frame_job.rs | implementation | W/🧵️frame-job/🦀️.rs | authored |
| 🦀️kernel_seam.rs | implementation | W/🪢️kernel-seam/🦀️.rs | authored |
| 🦀️lib.rs | unresolved package leaf | P/📚️library/🦀️.rs | Cargo-authoritative thin adapter |
| 🦀️os_host.rs | implementation | W/🏠️os-host/🦀️.rs | authored |
| 🦀️render_snapshot.rs | implementation | W/📸️render-snapshot/🦀️.rs | authored |
| 🦀️runtime_mailbox_core.rs | implementation | W/📮️runtime-mailbox-core/🦀️.rs | authored |
| 🦀️surface_lane.rs | implementation | W/📐️surface-lane/🦀️.rs | authored |
| 🦀️winit_app.rs | implementation | W/🪟️winit-app/🦀️.rs | authored |
| 🧪️browser-frame-transport.test.ts | implementation | W/🧪️tests/🧪️browser-frame-transport/🟦️.ts | authored |
| 🧪️browser-interactive-job-port.test.ts | implementation | W/🧪️tests/🧪️browser-interactive-job-port/🟦️.ts | authored |
| 🧪️index.test.ts | implementation | W/🧪️tests/🧪️package-integration/🟦️.ts | authored |
| 🧪️vitest.config.ts | unresolved package leaf | P/🟦️typescript/🧪️tests/🟦️.ts | explicit Vitest tool metadata |

The JCO implementation decision is:

| Source relative to 🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️jcoprobe/👽️guest | Source role | Exact projected destination | Ownership |
| --- | --- | --- | --- |
| 🦀️component.rs | implementation | 🧩️component/🦀️.rs | authored |

Every implementation destination stores semantics in its directory and uses a physical-kind leaf: 🦀️.rs, 🟦️.ts, or 🟨️.js. The generated WGPU JavaScript is regenerated rather than byte-edited.

## Package Contents After Composition

The WGPU package retains Cargo.toml, Trunk.toml, package.json, 🌐️index.html, 📋️project.json, and 📜️script.ts. It also owns:

- 📚️library/🦀️.rs — canonical Cargo library adapter.
- 🏗️builder/🦀️.rs — canonical Cargo build adapter.
- 💾️binary/🦀️.rs — canonical Cargo binary adapter.
- 🟦️typescript/📚️library/🟦️.ts — canonical Node export adapter.
- 🟦️typescript/🧪️tests/🟦️.ts — explicit Vitest configuration.

The JCO Cargo package retains Cargo.lock, Cargo.toml, and 🧬️schema/📜️world.wit, and adds 📚️library/🦀️.rs as its Cargo library adapter.

The resulting 15 unique package-contained leaves are all manifest, configuration, declaration, registration, tool metadata, or thin glue. No implementation remains below either package boundary.

## Exact Adapters And Manifest Authority

The five adapter byte templates are pinned in the golden. Their relative operands resolve as follows:

| Adapter | Target |
| --- | --- |
| P/🏗️builder/🦀️.rs | ../../../🏗️builder/🦀️.rs |
| P/📚️library/🦀️.rs on non-WASI | ../../../🧊️renderer/🦀️.rs |
| P/📚️library/🦀️.rs on WASI | ../../../📮️runtime-mailbox-core/🦀️.rs |
| P/💾️binary/🦀️.rs | ../../../⌨️native-entrypoint/🦀️.rs |
| P/🟦️typescript/📚️library/🟦️.ts | ../../../../🧊️renderer-boot/🟦️.ts |
| JCO 📦️packages/🦀️rust/📚️library/🦀️.rs | ../../../🧩️component/🦀️.rs |

The WGPU library adapter preserves the current target split. The current extern-crate/macro registration and non-WASI renderer body move to W/🧊️renderer/🦀️.rs; the WASI adapter exposes only the target-neutral mailbox core.

Exact manifest edits:

- WGPU Cargo package.build becomes 🏗️builder/🦀️.rs.
- WGPU Cargo lib.path becomes 📚️library/🦀️.rs.
- WGPU Cargo semio-wgpu-native bin.path becomes 💾️binary/🦀️.rs.
- WGPU Node exports[.] becomes ./🟦️typescript/📚️library/🟦️.ts.
- JCO Cargo lib.path becomes 📚️library/🦀️.rs.

The repository classifier reports each derived adapter as declaration. 🦀️lib.rs remains fail-closed unless Cargo lib.path selects the adapter. 🧪️vitest.config.ts has declaration-shaped TypeScript content, but its current physical name has no package configuration contract; it remains structurally unresolved until the exact 📜️script.ts runVitest config operand is registered.

## Schema Prerequisites

- Extend members-of-wgpu-target with the exact 21 owner members pinned in the golden. No wildcard is authorized.
- Register 🧩️component for the exact JCO guest owner/member context.
- Add a configurable rust-build-entry selected by Cargo.toml package.build.
- Add a configurable vitest-config-entry selected by the package script's explicit Vitest config operand.
- Permit binary, builder, and tests in the Rust package-boundary directory kinds. Existing library and typescript-language authority is reused.

These are prerequisites, not ambiguity. Absence of any one is a hard rejection.

## Reference Owners And Exact Rewrite Rules

The structural golden remains the exact authority for its 194 live path-token occurrences:

- WGPU: 191 occurrences — 106 authored and 85 generated.
- JCO: 3 occurrences — 2 authored and 1 generated.

Purity changes do not reclassify those owners. Exact-path implementation references override structural root-prefix rewriting; manifest and project-root references continue to use the projected package root.

| Owner class | Exact owner | Required action |
| --- | --- | --- |
| Cargo | WGPU P/Cargo.toml | Apply the three target-path edits; dependency paths continue to resolve from P. |
| Cargo | JCO 📦️packages/🦀️rust/Cargo.toml | Select the JCO library adapter. |
| Nx | WGPU P/📋️project.json | Keep sourceRoot and every target cwd at P; add the W semantic owner glob to namedInputs.default; outputs follow generated owner paths. |
| authored generator | WGPU P/📜️script.ts | Resolve W as ../..; point boot, worker, tests, lint source, and preview output at exact semantic leaves; keep Vitest config package-owned. |
| authored config | WGPU P/Trunk.toml | Watch and copy exact semantic/generated leaves. Preserve 🟨️boot.js only as a browser distribution URL. |
| authored Rust | W/🧊️renderer/🦀️.rs | Recompute every path, include_str, and include_bytes operand from the renderer directory. |
| authored Rust | JCO 🧩️component/🦀️.rs | Change WIT operand and prose marker to ../📦️packages/🦀️rust/🧬️schema/📜️world.wit. |
| authored taxonomy | repo 🔣️taxonomy.json | Change wgpu-frame-worker owner to P, input to W/🧵️frame-worker/🟦️.ts, tracked output to W/🧵️frame-worker/🤖️generated/🟨️.js, and add ignored boot-output authority. |
| generated | .vscode/launch.json, bun.lock, 🔒️dependencies.json | Regenerate from existing authored owners after source references change. |
| generated | W/🧵️frame-worker/🤖️generated/🟨️.js | Regenerate from P/📜️script.ts; never move or edit bytes directly. |

Local Rust module operands from W/🧊️renderer/🦀️.rs become:

- ../⏰️deadlines/🦀️.rs
- ../🪢️kernel-seam/🦀️.rs
- ../🏠️os-host/🦀️.rs
- ../📸️render-snapshot/🦀️.rs
- ../📮️runtime-mailbox-core/🦀️.rs
- ../🧵️frame-job/🦀️.rs
- ../📐️surface-lane/🦀️.rs
- ../🧵️browser-worker/🦀️.rs
- ../🪟️winit-app/🦀️.rs
- ../🎠️runtime/🦀️.rs

Renderer self/source assertions become 🦀️.rs, ../⌨️native-entrypoint/🦀️.rs, and ../📦️packages/🦀️rust/Cargo.toml. Element references lose one ascent; plugin registry references lose one ascent; the UI WGPU and font include operands likewise lose one ascent.

TypeScript owner-local rules:

- Renderer boot imports ../🐚️plugin-bridge/🟦️.ts.
- Browser boot imports ../🧵️browser-frame-transport/🟦️.ts and addresses ../🧵️frame-worker/🤖️generated/🟨️.js.
- Browser frame transport imports ../🧵️browser-interactive-job-port/🟦️.ts.
- Frame worker imports browser frame transport, interactive job registry, and plugin bridge as sibling semantic members.
- Interactive job registry imports browser interactive job port as a sibling semantic member.
- Tests import implementations through ../../../ plus the semantic member and physical leaf. Package integration imports P/📜️script.ts by computed relative path and passes P, not its test directory, to renderFrameWorker.

The ignored boot artifact becomes W/🧵️browser-boot/🤖️generated/🟨️.js. It is a generator output, not a 37th admitted structural move.

## Collision And Path Proof

- The 27 purity destinations are unique.
- The 5 adapter paths add 4 unique paths because the rewritten WGPU library mapping and WGPU library adapter are the same decision.
- The composed purity/adapter set therefore contains 31 unique paths.
- NFC, locale-independent lowercase, and variation-selector-insensitive canonicalization remains unique.
- No planned destination currently exists.
- Maximum mapping destination: 194 bytes at P/🟦️typescript/🧪️tests/🟦️.ts.
- Maximum including derived adapters: 196 bytes at P/🟦️typescript/📚️library/🟦️.ts.
- Headroom to the 240-byte budget: 44 bytes.

## Verification

The ticket-local Bun test is 🧪️nested-cargo-package-purity-authority.test.ts. It uses repository source classifiers, fast-glob independent enumeration, @iarna/toml independent Cargo parsing, SHA-256 drift checks, structural/purity composition, adapter target resolution, semantic registration, collision, path-budget, and authored/generated ownership assertions.

    bun test './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️nested-cargo-package-purity-authority.test.ts'
    5 pass, 0 fail, 227 assertions

    bun test --timeout 30000 './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️nested-cargo-package-authority.test.ts'
    4 pass, 0 fail, 313 assertions

The existing structural suite's default five-second run timed out only in its live-reference census. With an explicit 30-second timeout, that census completed in about nine seconds and passed.

## Root Independent Rerun

After the authority handoff, the root coordinator ran both frozen worktree suites together without changing source, golden, or Git state:

```text
bun test --timeout 30000 './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️nested-cargo-package-authority.test.ts' './.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/END-TO-END-TAXONOMY-NORMALIZATION/🧪️nested-cargo-package-purity-authority.test.ts'
9 pass, 0 fail, 540 expect() calls, 14.76 s
```

This independently confirms the 36 structural mappings and 27 purity decisions against the authoritative worktree goldens. It does not claim that either production move has been applied.
