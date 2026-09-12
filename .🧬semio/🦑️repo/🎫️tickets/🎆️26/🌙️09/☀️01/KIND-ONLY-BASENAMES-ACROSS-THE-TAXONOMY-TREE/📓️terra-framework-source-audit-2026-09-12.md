# Framework Source Correction Audit

## Scope

This was a bounded, read-only audit of the completed framework source-correction batch described in 📓️framework-source-correction-audit-packet-2026-09-12.md and 📓️sol-framework-source-2026-09-12.md. It examined semantic source ownership, Cargo mounts, the browser/Storybook boundary, 23 render test-owner moves, Metal's test feature, and the completed session-isolation correction. It did not regenerate source or session outputs, alter shared configuration, or execute separately queued framework bodies, mandatory-script work, Puzzle binding work, OS work, or repository work.

## Accepted Evidence

| Check | Current result | Evidence |
| --- | --- | --- |
| Framework portable topology | Pass: 4 tests, 253 expectations | NX target '@semio-tech/repo-lib:test-framework-source-topology'; 🗑️generated/terra-framework-source/topology.log |
| 23 render test-owner physical moves | 23 old paths absent; 23 semantic owner directories contain '🦀️.rs'; all owners omit 'packages-rust' | 🗑️generated/terra-framework-source/render-test-owner-closure.json |
| Native mounts and browser compiler closure | Pass through the topology test: independent @iarna/toml mount parsing and Bun browser compilation | topology test 'binds native crates to owner roots and browser imports to a pure facade' |
| Metal all-features native route | Pass: 10 tests, 0 failures | 'cargo test -p semio-framework-ui-backend-metal --all-features'; 🗑️generated/terra-framework-source/metal-all-features.log |
| Session canonical identity spot-check | Current bytes, SHA-256, and mtimeNs exactly equal the completed isolation record | 🗑️generated/terra-framework-source/session-canonical-identity.json |

The topology test directly validates all declared moved files, package-entry glue limits, six Cargo package root mounts, former-root absence, contextual directory fixture rows, and the two launch authorities. The completed executor report is the authority for the larger source map and its scoped UI Storybook build; this audit independently reran its final portable gate after the batch reached its final 4/4, 253-expectation state.

The current stylesheet browser facade is only two theme re-exports at 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🟦️.ts. The iframe helper at 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌐️iframe/🟦️.ts has no Node or repository imports. Storybook imports Vite/build helpers from the separate semantic builder owner at 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts and defines 'import.meta.vitest' as 'undefined' in its production configuration. The portable gate's Bun browser compilation succeeds on the configured browser-reachable entries, so the production test-import elimination and facade split currently compile.

The Metal feature wiring is coherent: the Metal package's 'backend-testing' feature enables 'ui_render/backend-testing'; Scene::finish is gated by 'cfg(any(test, feature = "backend-testing"))'; and the native Metal unit module requires both test and that feature. The all-features execution passed 10 tests, including headless construction/readback/loss behavior. It emitted one existing 'unnecessary unsafe block' warning at 🍎️backend/🦀️.rs:769. The test permits clean return when no physical Metal device exists, so this verifies compilation and guarded runtime paths, not physical-GPU rendering.

For the session correction, the current canonical file remains 12,139 bytes, SHA-256 '4a785c5f4288f2874e86540a27efb9bde741ed3bfd97ed121d193a643fce0043', and mtimeNs '1789219019005731875', exactly matching 📓️sol-session-fixture-isolation-2026-09-12.md. Current activation owns the shared semantic Vite alias; the Vite config calls 'playgroundSessionViteAlias(sessionRoot, plugin)'; and the current 'ensurePluginRegistry' body generates and synchronizes descriptors without 'writePlaygroundSession'. No session fixture was rerun because its producer intentionally writes private files and the previous completed proof already covers it.

## Actionable Source-Taxonomy Defect

The physical source move is correct, but its semantic directory registration is incomplete.

An independent current 'inventoryTaxonomy' scan of 🧰️framework/🔨️modules/🖱️ui/🖌️render/🧪️tests finds 79 entries and 37 'directory-kind-unresolved' errors. Exactly 23 errors are the moved test-owner directories declared by the framework source topology fixture:

| Target family | Moved owners lacking a directory kind |
| --- | ---: |
| D3D12 | 3 |
| Metal | 4 |
| Vulkan | 6 |
| WebGPU | 10 |
| Total | 23 |

Each has the expected semantic name, an anonymous Rust leaf, and no surviving old 'packages-rust' directory. The failure is therefore not a bad source mount or an implementation-named leaf: the taxonomy has no semantic directory registration for the new test-owner names under the render test parent.

The extracted browser builder directory 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite has the same 'directory-kind-unresolved' error. It is also an explicit framework-source topology owner.

The correction should add precise context-aware directory-kind authority and fixture coverage for the 23 render test owners and the styling builder/Vite owner. It should not special-case these paths in a generic classifier. The portable topology fixture currently verifies physical paths and a smaller contextual-directory list, so it did not fail for these missing registrations.

Exact direct evidence is 🗑️generated/terra-framework-source/direct-owner-taxonomy-findings.json. The render inventory has 14 additional unresolved directories outside the 23-row move map; they are recorded as ambient-to-this-map current findings, with no baseline or causal attribution. Broader spot inventories also currently show styling 12, host 13, and contract 110 violations; they were not assigned to this correction batch.

## Limits

The audit did not rerun the multi-minute Storybook build or the writing session fixture. The executor's completed scoped Storybook result remains the runtime evidence for the full preview graph; this audit validated the live entry split, production sentinel, and native browser compiler closure. The Metal test's hardware-tolerant behavior does not demonstrate a physical device path. No conclusion is made about the separately queued root framework bodies, mandatory scripts, Puzzle binding prerequisite, or concurrent OS/repository source work.
