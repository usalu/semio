# Repo Library Fixture Separation

**Date:** 2026-09-09  
**Scope:** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library` plus two VS Code delivery fixtures, excluding the separately owned caching/playground fixture corpus except for the final Nx cache input boundary.

## Result

The relocation classified and moved 199 non-code test inputs into semantic owner-level fixture or schema roots. This count includes 197 library inputs and two VS Code delivery inputs. Every move was checked byte-for-byte with SHA-256. The post-move ledger reported 199 classified sources, 199 targets, zero stale references, and zero failures. The whole-repository layout scan later reported zero physical library findings.

The mutation projection contract now owns one canonical implementation case under `🧬️schema/🧬️mutations/.../🧪️tests` and one separate 12-node data bundle under `🧫️fixtures/🧬️mutations`. The normalizer no longer creates artifact-root projected test trees or combined source/data bundles. Logical scenario IDs and emoji-prefixed physical directory names are validated independently and are independently unique.

Production taxonomy loaders now require runtime authority catalogs and current-revision expectations under `🖼️assets`. Canonical Go discovery treats `🧫️fixtures` as opaque. Nx production and native-source input groups now exclude every workspace fixture, while default/native-test inputs retain fixtures; the two TypeScript WGPU targets route through `production`, and the resident WASM check routes through `nativeSources`.

## Canonical contract files

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-library-fixture-separation-2026-09-09.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/📚️repo-library/📜️script.ts`
- `nx.json`
- `🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📋️project.json`
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚡️production-cache-input-boundary/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖼️runtime-taxonomy-asset-paths/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-case-pair/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-scenario-identities/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚡️production-cache-input-boundary/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖼️runtime-taxonomy-asset-paths/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-case-pair/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-scenario-identities/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/⚡️production-cache-input-boundary/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🐹️canonical-go-discovery/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖼️runtime-taxonomy-asset-paths/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️mutation-case-pair/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️mutation-scenario-identities/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`

The 27 files whose literals were rewritten by the retained relocation program are:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📍️draw-destination-observation/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/↪️rust-divergence-callback/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📍️draw-destination-observation/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`

## Authored Paths

This complete flat manifest includes every created, modified, removed, intermediate, and final path in this lane:

```json
[
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/📓️repo-library-fixture-separation-2026-09-09.md",
  ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/📚️repo-library/📜️script.ts",
  "nx.json",
  "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/📋️project.json",
  "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📋️project.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧫️fixtures/📦️package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧫️fixtures/🧩️host-build/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧪️abstraction-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🧪️abstraction-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🪪️field-parity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️fixtures/🧪️nested-cargo-package-purity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/✂️source-parent-prune-journal/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎫️ticket-document-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎯️scoped-inventory-pathspec/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎲️transaction-disposition-outcomes/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🏭️generator-preview/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📈️taxonomy-cli-progress/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📽️nested-cargo-package-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔀️generator-input-transitions/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔍️discovery-schema-handoff/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🕰️ticket-important-history-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🕸️scoped-incoming-reference-closure/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🗂️mutation-directory-classification/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚏️router-import-boundary/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚶️scoped-admission-walk/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🛤️mutation-path-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🤝️transaction-protocol/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧩️artifact-component-leaf-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🪪️ticket-important-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🫙️artifact-empty-facet-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧫️fixtures/🔬️rust-policy-source-evidence/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧫️fixtures/⚡️inputs/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/⚖️readme-license-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/💉️ticket-important-exact-mutations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🗺️testing-readme-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🚨️transaction-sentinel-cases/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚡️production-cache-input-boundary/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🧬️energy-source-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🍃️artifact-support-leaf-authority/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🎮️command-vectors.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🗺️artifact-location.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🧬️energy-source-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🎯️reviewed-expectations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/📥️reviewed-source/📝️.md",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧫️fixtures/🧪️consumers/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📥️mutation-input-carriers/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔄️transaction-v2/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔍️filesystem/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/📍️projection-scope/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔐️mutation-codec-ownership/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔭️mutation-scope/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖼️runtime-taxonomy-asset-paths/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛑️taxonomy-cli-cancellation/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🧪️fixture/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-case-pair/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-scenario-identities/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-rust-entries/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-rust-entries/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧲️rust-physical-reference-context/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧫️fixture/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🧫️fixtures/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/☑️options.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/☑️options.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⏱️process-budgets/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚙️root-script-compiler/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚡️production-cache-input-boundary/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/✂️source-parent-prune-journal/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/✅️mutation-test-presence/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/✍️rust-writable-path-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-markdown-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-markdown-coordinates/🧬️energy-source-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌐️registry-import-language/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌐️registry-import-language/🧪️imported-data/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌱️mutation-root-discovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌳️workspace-taxonomy/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎟️reference-coverage-selection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎫️ticket-document-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎯️cargo-target-discovery-skip/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎯️scoped-inventory-pathspec/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎲️transaction-disposition-outcomes/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏎️nextest/🎮️command-vectors.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏎️nextest/🗺️artifact-location.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏗️mutation-scaffolding/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️generator-preview/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏷️metadata-source-provider/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏷️metadata-source-provider/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🧬️energy-source-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📥️reviewed-source/📝️.md",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💠️inventory-artifact-shards/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💥️nested-cargo-collision-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📈️reference-coordinate-progress/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📈️taxonomy-cli-progress/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/📸️source-index-capture/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🧪️consumers/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🧾️source-file-facts/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📍️draw-destination-observation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📍️draw-destination-observation/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📡️mutation-reachability/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📡️mutation-reachability/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️typescript-declaration-facts/💥️malformed/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️typescript-declaration-facts/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📥️mutation-input-carriers/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📦️extension-installation-owner/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📦️semantic-package-source-manifest-identity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📽️cargo-provider-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📽️cargo-provider-projection/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📽️nested-cargo-package-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔀️generator-input-transitions/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔍️discovery-schema-handoff/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔍️filesystem/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔎️json-reference-owner-lookup/📍️projection-scope/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔎️json-reference-owner-lookup/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔏️path-emoji-statutes/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔐️mutation-codec-ownership/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔗️markdown-inline-references/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔤️taxonomy-leading-grapheme/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔭️mutation-scope/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕰️historical-json-source-encoding/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕰️ticket-important-history-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕸️scoped-incoming-reference-closure/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🛣️commit-route/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖼️runtime-taxonomy-asset-paths/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🗂️mutation-directory-classification/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🗺️testing-readme-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚏️router-import-boundary/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚚️readme-move-source-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚧️cargo-discovery-exclusions/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚶️scoped-admission-walk/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛟️transaction-recovery-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛤️mutation-path-projection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛤️typescript-path-collection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛫️preflight-reference-basis/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️transaction-protocol/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥒️gherkin-description-inline-code/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦀️exact-cargo-laws/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧩️artifact-component-leaf-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-case-pair/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-scenario-identities/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-type-origin/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-type-origin/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️schema-rust-entries/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️schema-scope-catalog/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧼️clean/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧾️registry-catalog-gitlink-boundary/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪟️windows-checkout-paths/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️cargo-provider-binding/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️cargo-provider-binding/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪪️mutation-metadata/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪪️mutation-metadata/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪪️ticket-important-owner-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/☑️options.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/↪️rust-divergence-callback/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/⏱️process-budgets/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/♻️taxonomy-pattern-compiler-reuse/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/⚡️production-cache-input-boundary/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/✅️mutation-test-presence/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/✍️rust-writable-path-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌐️registry-import-language/🧪️imported-data/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌱️mutation-root-discovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌳️workspace-taxonomy/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🎟️reference-coverage-selection/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏎️nextest/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏗️mutation-scaffolding/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏭️owned-generator-preview-inventory/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🐹️canonical-go-discovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📈️reference-coordinate-progress/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/📸️source-index-capture/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🧾️source-file-facts/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📍️draw-destination-observation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📍️draw-destination-observation/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/💥️malformed/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📦️extension-installation-owner/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📦️semantic-package-source-manifest-identity/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔏️path-emoji-statutes/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔗️markdown-inline-references/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔤️taxonomy-leading-grapheme/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🛣️commit-route/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖼️runtime-taxonomy-asset-paths/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️testing-readme-coordinates/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🚚️readme-move-source-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🥒️gherkin-description-inline-code/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🥤️rust-finite-target-consumption/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🦀️exact-cargo-laws/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️mutation-case-pair/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️mutation-scenario-identities/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️schema-rust-entries/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️schema-scope-catalog/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧼️clean/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪟️windows-checkout-paths/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/☑️options.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🛂️schema/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/💥️generic-stem-collision-resolution/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission/🧪️io/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🚪️source-admission/🧪️io/🔣️.json",
  "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts"
]
```

## Verification

Focused tests ran through the ticket-owned Nx fixture workspace with `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, ticket-local cache/TMP paths, and `SEMIO_FIXTURE_REPO_ROOT` pointing at the repository.

| Proof | Result |
| --- | --- |
| Relocation verification | 199 classified, 199 targets, 0 stale references, 0 failures |
| Canonical Go discovery plus Go toolchain oracle | 1 test, 9 assertions, passed |
| Runtime taxonomy asset paths plus picomatch/Ajv oracle | 1 test, 11 assertions, passed; live taxonomy validation returned no problems |
| Mutation scenario logical/physical identities plus Ajv oracle | 1 test, 19 assertions, passed |
| Extracted normalization pair contract | 1 test, 27 assertions, passed |
| Extracted mutation registry pair contract | 1 test, 8 assertions, passed |
| Actual VCS mutation inventory normalization | 1 test, 14 assertions, passed |
| Actual Energy mutation inventory normalization (552 scenarios) | 1 test, 14 assertions, passed |
| Production cache boundary through Nx getTargetInputs/filterUsingGlobPatterns plus minimatch/Ajv | 1 test, 21 assertions, passed |
| Full dynamic project projection (667 config inputs) | WGPU generate/check use production; MCP build and shell check use nativeSources; resident check-wasm uses nativeSources; all reject owner and shared-runner fixtures |

The representative command shape was:

```sh
NX_WORKSPACE_ROOT_PATH="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️05/☀️30/ASSETS-FIXTURES-SEPARATION/🧑‍💻coordination/🧫️nx-fixture" NX_DAEMON=false NX_ISOLATE_PLUGINS=false SEMIO_FIXTURE_REPO_ROOT="$PWD" bun nx exec --projects=fixture-probe -- bun test "$PWD/<canonical-case>/🟦️.ts"
```

The plugin-wide URI resolver independently reported 9,057 references with zero missing targets. The full layout census and cross-scope production catalog follow-ups are recorded in the coordinator reports; they were not repeated here.

## Byte-preserving relocation ledger

Five intermediate fixture targets were subsequently reclassified as runtime `🖼️assets`; the final path is shown below. The old mutation projection vector was retired once the canonical pair contract replaced it.

| Source | Final disposition | SHA-256 at relocation |
| --- | --- | --- |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/📦️package/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧫️fixtures/📦️package/🔣️.json` | `168e575595ea91bccbfb0db96237b7b1f02fdce9113b0d1e3463e82b948292b2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/📦️packages/🟦️typescript/⚙️build/🧪️tests/🧩️host-build/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/🧫️fixtures/🧩️host-build/🔣️.json` | `b6368b02382e9adab0eab87c4c5d764b04a2d696de9e901d1d04b641b908406a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🧪️abstraction-ownership/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🧪️abstraction-ownership/🔣️.json` | `21b07d15c2fd9ef41f1c1ed2d20b57357d104c121767686da8804ef8d1c7483a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧪️tests/🪪️field-parity/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📏️ownership/🧫️fixtures/🪪️field-parity/🔣️.json` | `82662d5656d4b927ac3d93418bd5566d6cc46c299770582c80dc6df70fb13ff1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/⚖️readme-license-owner-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/⚖️readme-license-owner-authority/🔣️.json` | `2dcb31628412a9e95b4adfc8756d8fbe9dfa1b397742b3f8a47c603ad15ccc79` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/✂️source-parent-prune-journal/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/✂️source-parent-prune-journal/🔣️.json` | `06d372c376fbd024c0ec2702de8b52189452e4e18d69125c3ab609dbbc935e1b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-coordinate-evidence/🔣️.json` | `c5e1c1b8186b3343f85dad0d08677f7517f858f16e8eafb92f5d0932ac14c414` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🍃️artifact-support-leaf-authority/🔣️.json` | `de6a1a8fd2da11a31aedf3e393478cfea86c1afba4e7f6277450b8b62f176d34` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎫️ticket-document-owner-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎫️ticket-document-owner-authority/🔣️.json` | `d2b444b299980e2a7c9ea73adc698c714305dc0684b5c25955a35748519496fc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎯️scoped-inventory-pathspec/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎯️scoped-inventory-pathspec/🔣️.json` | `cd2a40fb3d1929386e1208dc4ed3ccc862abf316817a42acf7f81b8c8b886a42` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🎲️transaction-disposition-outcomes/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎲️transaction-disposition-outcomes/🔣️.json` | `e449ef9167cdaa0d779a9754a4ee653194ad83bc54afcc93e1c96f2323b943a3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🏭️generator-preview/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️generator-preview/🔣️.json` | `ebf63d6740e579549d5d50ef6f401cff676418a610493a2d89858c83935a9971` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/💉️ticket-important-exact-mutations/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/💉️ticket-important-exact-mutations/🔣️.json` | `a5012fa365145d27001fdd21d87e072f2b7a0d7992d017b7753390febf24a195` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💎️nested-cargo-package-purity/🔣️.json` | `8138a5271d1b4f7d21a8b544ef65ebab630e0d7763b258ce3196cab500205ecd` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📈️taxonomy-cli-progress/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📈️taxonomy-cli-progress/🔣️.json` | `aa954962b353978ea3e1c6d82ad3a55771483e0b72041355bd3c4deb4b3dc091` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📐️cad-draw-path-projection/🔣️.json` | `eb4d78938e21f11619e75f915d4228ac79eaf9f01e4a445ac8d10c16d9a42786` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json` | `223b5722e248826648a49ced41178dbd40d5b3f1406a500ddc06339564fc7cad` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📦️extension-installation-owner/🔣️.json` | `a1d37688e5a5fe299becfe28e0ff5955cd9892b70ded3fb98388e1558b4ea131` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📦️extension-installation-owner/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📦️extension-installation-owner/🔣️.json` | `96f5e324912e9f10cd9d9f6d198d122bbe18fcd6f2793e7f53360ab015018436` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/📽️nested-cargo-package-projection/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/📽️nested-cargo-package-projection/🔣️.json` | `46a9750c0e31aa2d778ce851d3c8a472cd0433bde7d0e4ea687668c4b0c323a7` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔀️generator-input-transitions/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔀️generator-input-transitions/🔣️.json` | `17c2c3a9c2e180f83e18f1e4bd53dc5cc80dbcb57b443cf554b86d6c1f74f5f3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🔍️discovery-schema-handoff/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔍️discovery-schema-handoff/🔣️.json` | `7c0b5c22d47d387f285878d96bf21fae60af019b09880ae04e5a0bce3c53e8fb` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🕰️ticket-important-history-owner-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕰️ticket-important-history-owner-authority/🔣️.json` | `45ea308ec4b36a2988025981b08b35deaaa404f0729086a4bd9b16db891445cb` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🕸️scoped-incoming-reference-closure/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕸️scoped-incoming-reference-closure/🔣️.json` | `63e66803512719e7a887a9ee770de73f33281730d5ce6151111fa24604e24cfa` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🗂️mutation-directory-classification/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🗂️mutation-directory-classification/🔣️.json` | `24babbc55a925f41764dd3e99fa063772dacde830f4b7fccb6b56c9ba66d2e64` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚏️router-import-boundary/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚏️router-import-boundary/🔣️.json` | `b29fa5554d2956e72a5b3f492bfdd610d82b5e6c46b8ee9478c4962358bfdf6d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚨️transaction-sentinel-cases/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🚨️transaction-sentinel-cases/🔣️.json` | `c67b436abca4b9005efce6023f74bd900de5e88187ee9eb3c4328c2397bd081b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🚶️scoped-admission-walk/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚶️scoped-admission-walk/🔣️.json` | `fea49d7dda66e6d541d49d7a0f998d833973fbb602af453292128c133d2da207` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛑️taxonomy-cli-cancellation/🔣️.json` | `ef69a880db79511c57675d510b2ffdc36b97315db46e30c091b6badecb0d6def` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🛤️mutation-path-projection/🔣️.json` | `retired after the canonical pair contract replaced projected storage` | `e87a99e0868f9eb4dadc2303c9d6eb8478d92e19ee76d188f3f829702bf003aa` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🤝️transaction-protocol/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️transaction-protocol/🔣️.json` | `0e4695233061eb2e5db38c6731e9f1c460e694f358c1f6802b5d3fa9ba161802` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦀️nested-cargo-package-authority/🔣️.json` | `0ab0599756f0331a4f2dbe42d1b383a5f99a5a0f92c1dcba51b71bc66b7b9d11` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧩️artifact-component-leaf-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧩️artifact-component-leaf-authority/🔣️.json` | `ab520e4b72cf7dca98e09eef0e284acb194731bc71126ff56887d9f0ed4da291` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔣️.json` | `56ecb27e82572200936e1724666f44468071fea459b7d88b5cbca7ea44f2f64c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧲️rust-physical-reference-context/🔮️oracle/⚙️.toml` | `028b6d8e393f26bef506d164406d9f0dc294c28726bd4597b52b47d1819f74cf` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧲️rust-physical-reference-context/🧬️join-provenance/🔣️.json` | `c63d847ce0ae1e7b733bd31e9b9b95112f2ae9098f7f3ad9d51009a73036aa2d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧼️remaining-package-purity-authority/🔣️.json` | `1eb151498e9a135ac516a80fff94f8283e848f06c3d7e840e87fc29bafcfcfea` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️transaction-harness-retention/🔣️.json` | `62aa8ff5baeb92069ae0773e2ebed24218f2a38c93f190a6bee28e58bf66da0d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🪪️ticket-important-owner-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪪️ticket-important-owner-authority/🔣️.json` | `b9aa1215f3ddfc7681f33d2adc5badbdaeec924966909b97bc7264708242c033` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🫙️artifact-empty-facet-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/🔣️.json` | `52f0b5b8a516e7f894ea42e8d55f30dcfbd7780e80f320914849fa495502969a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🔬️rust-policy-source-evidence/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧫️fixtures/🔬️rust-policy-source-evidence/🔣️.json` | `3d99cb1e192e053c5aaa72707b3e1c049dcea34e8d044ba02d0ae6fdc006cc8a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧪️tests/⚡️inputs/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🟦️typescript/🧫️fixtures/⚡️inputs/🔣️.json` | `24119a4ec4951e711c1dbe27e387691ee08c96aa85f429394c1cf5a51af3841e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/↪️rust-divergence-callback/🔣️.json` | `91efb284b36d29c83469de0bcb8005c9d296f170f0153995c6eaf5301fcf215e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/↪️rust-divergence-callback/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/↪️rust-divergence-callback/🔣️.json` | `ce44d2ea3fa60e6f28ceaf6ef25e700761bcf7d4a3a93103f95e93a352a92dc0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⏱️process-budgets/🔣️.json` | `b744e13c5cfb378acc9f62637f4eb470917015e24b8140c1acab6ff8e2447be0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⏱️process-budgets/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/⏱️process-budgets/🔣️.json` | `2a4db0f76adf930ee267dd998674e8dea3409c7307f1a0231134836aa37bfb10` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🔣️.json` | `dd12182123e1a260aa2097b7b9b45189a3330d8d985f007a31b587452f4e35a2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/♻️taxonomy-pattern-compiler-reuse/🔣️.json` | `9505110a61f4f7bd42915c5100815a278f96ac9ea6fcac70aa186411a97d0284` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json` | `c410cef702db87762078a6646ff7b71860fdea5757681c3f34b839cb83f01cb5` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/♻️taxonomy-pattern-compiler-reuse/🧪️registration/🔣️.json` | `3caea4ad27034dc1cc51e86ce1451592177c9064da48683fdbdd6d93b4efb52c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚙️root-script-compiler/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/⚙️root-script-compiler/🔣️.json` | `f7585a05dad104e07bbe932a5d1a766c5deae669f6f96f9bfb48c42b625cf019` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/✅️mutation-test-presence/🔣️.json` | `bd88847f6b6caa505d5e44a342a1bac901528c4b54938e6d343c22e8d6cc80df` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✅️mutation-test-presence/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/✅️mutation-test-presence/🔣️.json` | `dea1ce6d2dfb2b6f0cae72fe48e8b9309f1d498902052dd7400a7397158dea73` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/✍️rust-writable-path-authority/🔣️.json` | `231a9e82497252033a8b1ffb2b58aac10759ae6847b17eb46a0ba71e5d6cdf69` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/✍️rust-writable-path-authority/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/✍️rust-writable-path-authority/🔣️.json` | `dabe70739d0d5bab3c1b152dd640860def5254ab26dd273abf0ec144372bf3f4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-markdown-coordinates/🔣️.json` | `1f72c6c843b5d8665e987239c71df2c011a9aafc2cb3d7b235120be9a6cbe78c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/❄️frozen-markdown-coordinates/🧬️energy-source-coordinates/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/❄️frozen-markdown-coordinates/🧬️energy-source-coordinates/🔣️.json` | `909f751608fa14ebe00a61aa0fea5bcc94b4544edaf40dc9922c189f4aeb62ab` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌐️registry-import-language/🔣️.json` | `da4ea142d6136b8f731529d2475e473e392fdfb1ad613d90ed9c5a0fe18c87e3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌐️registry-import-language/🧪️imported-data/🔣️.json` | `e3544f2a66b8587711a48ee7115bef905e784408c60c7c2c72d77743c5997868` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌐️registry-import-language/🧪️imported-data/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌐️registry-import-language/🧪️imported-data/🔣️.json` | `b8a839d0c4bdddceeaaf615ebd817e14f9c10435abcd9f809bd2e9664a571444` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌱️mutation-root-discovery/🔣️.json` | `f47b28fe1093ca1556309c4c899634c499b5c9c749ac283972e5e024c00e6fcd` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌱️mutation-root-discovery/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌱️mutation-root-discovery/🔣️.json` | `5b3c138f02c6ed7b48e437f66cad87fbe08d1ec7b1c13350bdfa2f311868eab4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🌳️workspace-taxonomy/🔣️.json` | `b51d30afe2093b41e0b445cfff89e37a737192a946b468c61fa0a1bbe5e3f19b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🌳️workspace-taxonomy/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🌳️workspace-taxonomy/🔣️.json` | `020651bb10641d163fbc1277574a43de7ac4984ecc6e2c3794e79a6be4677cae` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎟️reference-coverage-selection/🔣️.json` | `9a92cb84a01be198f00aaccdb2ec4ab2511d23c199e1a1a14dff7cb4858b97ac` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎟️reference-coverage-selection/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🎟️reference-coverage-selection/🔣️.json` | `3b3c5e89e691ed0a6ae3214f8c7a4a055710cb12ded00ba6b4ce66261daeeb2f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎯️cargo-target-discovery-skip/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🎯️cargo-target-discovery-skip/🔣️.json` | `241fc76f14c16732633e11811ba255d0077d0c1524797891764884a4aaaec61b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🎮️command-vectors.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏎️nextest/🎮️command-vectors.json` | `7a612daf5df4b6ff9acd4f28fda7d57138951b214d4983d84a998dad6ba170bb` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🗺️artifact-location.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏎️nextest/🗺️artifact-location.json` | `c7124475fccb65d70e2324a96b46c0b83bbbcf5ded2fabea72461c2782728320` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏎️nextest/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏎️nextest/🔣️.json` | `6111d26f95f0bd6076227b004afcf711440d126ab452fd9f347f96a532688316` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏗️mutation-scaffolding/🔣️.json` | `63f1dc5b024f2e34151b668eda1f4512fed85d9a60a87960f51f74ae36c7efb4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏗️mutation-scaffolding/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏗️mutation-scaffolding/🔣️.json` | `20245cef86d009ed8cdb77ae178bfccc76c326677637e5eacd26a7209f0d5651` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️owned-generator-preview-inventory/🔣️.json` | `2d88bde0685c9ea94239692876d4d26eaf8c525d8324fa621a4c56a6b3979d83` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️owned-generator-preview-inventory/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏭️owned-generator-preview-inventory/🔣️.json` | `6933e6624f948009e76ccae37f5fd412c2314141be6865f1ce3313cf962c24d4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏷️metadata-source-provider/🔣️.json` | `ebf41cf547289305f6b1c89fee1a6e478fd99fa5fe0f5ad436bd0171cb1e10b0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏷️metadata-source-provider/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏷️metadata-source-provider/🛂️schema/🔣️.json` | `e0bb878e68ccdab3019148eee5222068114047c4c75c4dcfe432f7f1fc0d6f3d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🔣️.json` | `95b0cfc60c4a74faf5d7e9ab1633f3e18042997f14de484fe3e35c30da90b483` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏺️historical-package-owner-identity/🧬️energy-source-coordinates/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏺️historical-package-owner-identity/🧬️energy-source-coordinates/🔣️.json` | `eff61e07885ad98caea74f2ecc8a37f6ad62b4adfdecdd3e41316a7e35f37953` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🐹️canonical-go-discovery/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🐹️canonical-go-discovery/🔣️.json` | `4d32b220bba5d746b600355d01b63bd9aba2c4e8fdc06f01d948f6f95f13cc1b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🔣️.json` | `d4601ef5b10c9003a3c3c49173df8848b88923b1fddf923485a39a9f3b999717` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/🔣️.json` | `65d4dd5431111d2d5a2a5009b4d34e202c9e486369cb6c3dff72a24128689182` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🎯️reviewed-expectations/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/🎯️reviewed-expectations/🔣️.json` | `e0a59e016877f99058215c4d5aa152b352f89865a6674179f0e29c3e32d2ee70` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/📥️reviewed-source/📝️.md` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📥️reviewed-source/📝️.md` | `20fa38e056c09959c00b652f0ed2985b65cf47b0115df0a6ca9418258dab85fc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json` | `19dad9567700afb557c84d2973bcc5e70ab03e5af9cc74a888636436d58e3b1e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/👀️readme-reviewed-fixture-inputs/🧫️fixtures/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/👀️readme-reviewed-fixture-inputs/📋️manifest/🔣️.json` | `0c24404661ba20591f948ff944cec5c18a1f5745bfbb65f35d5a372a37f6307c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💠️inventory-artifact-shards/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💠️inventory-artifact-shards/🔣️.json` | `f8498b5ae3b0ab00a211dce9928b91d0b20119dcbe5d7c707aee472d4668357c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/💥️nested-cargo-collision-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/💥️nested-cargo-collision-authority/🔣️.json` | `b715ae9589e0aef922a29a865df33fc9ef86afa95183c45e0b9610c823697b50` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📈️reference-coordinate-progress/🔣️.json` | `b9c1344849d198a3b3521d65e75f0056571285119fdab4d780d4cd8b3c6a829c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📈️reference-coordinate-progress/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📈️reference-coordinate-progress/🔣️.json` | `a2f1c7a9d9d9ac76c9bcf7d79a81c646e4d1f48320ee47876c6e33359702c027` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json` | `7ffb8158c3d358df4aac73f9e98e4a1b5a8c216552dd49a37d37bd12853d20e2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎫️ticket-role-routing/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🎫️ticket-role-routing/🔣️.json` | `bb50dce668f680d7cc8eb30885cdef1794d6f5a33f0c3f30c238b30690f3f6a0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json` | `110f594509f9286cbb2eae905b1cdb09e6b473c3fd2f9a6cf989ab4fa4e85d87` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🎭️source-roster-roles/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🎭️source-roster-roles/🔣️.json` | `e59cec0d7f1fc079b1aa62145726451eee73be258137e005cb181fcfc990a9af` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/📸️source-index-capture/🔣️.json` | `334a8f2bc820af1cacdc5b56252c8796df4120b89f0b9350d7e588553bb3545f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/📸️source-index-capture/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/📸️source-index-capture/🔣️.json` | `b1ba1d30ccbc675deec350c9880d6f8320fa3678934540594919db91c2cb1722` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🔣️.json` | `52af0f5ce3de04befb7e22a8d4da840c5f0719b4193c92759d9df3d274e12874` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🔣️.json` | `b8523cf50f8897e169dd44c40cb78186c9cd77d128dc564dd82c9707832cc37d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧫️fixtures/🧪️consumers/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🧪️consumers/🔣️.json` | `1247b098f47163f83ab99f5774c2726ae16e4857cb2633498bc4a36e7623d98c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📋️mutation-inventory/🧾️source-file-facts/🔣️.json` | `a27830505bf1a7b55a2b8f21fab684fd14f23a288a373e2a0b6c7d8004d18d73` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📋️mutation-inventory/🧾️source-file-facts/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📋️mutation-inventory/🧾️source-file-facts/🔣️.json` | `7c35594804d05e0defc7759f203bdf5aacccd70e148b0baebed5c4518ffb1b3a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📍️draw-destination-observation/🔣️.json` | `53b157af899a80d20fb050667f0620d0850a2ff60a5e528348cca8db8e90c735` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📍️draw-destination-observation/🔣️.json` | `fa3b3425ef069f794b99ee1c02dc3689f18f66c9e548d8d464da48168435028e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📍️draw-destination-observation/🧪️registration/🔣️.json` | `480ff8735d82c3ed78e8a5ef906baa20ab14603c8dcb2ed883b980173d11ffa4` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📍️draw-destination-observation/🧪️registration/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📍️draw-destination-observation/🧪️registration/🔣️.json` | `c3fb941146f2c3365a4f79fcc2c444e14fe00acc02d4f2fcb83a0c69b5cc6a10` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📡️mutation-reachability/🛂️schema/🔣️.json` | `8c7d464d45c07e8e7284518b333fc7a764c6962ed85106c5d45d99a6ca360884` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📡️mutation-reachability/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📡️mutation-reachability/🔣️.json` | `0e258612b566070295426002435da55cda1800f63d28f8553b28c85ac8d22533` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️typescript-declaration-facts/💥️malformed/🔣️.json` | `d36ab85591265aecbc692e8fde7702ed9324f9fcff3e6aafa9da21ce41391462` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/💥️malformed/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/💥️malformed/🔣️.json` | `2366950a975d26b438721f02961aeead77b77e1f973d3a6a26b641097bff05dc` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️typescript-declaration-facts/🔣️.json` | `d2e0f14760acaced4fc376b69b7ff2a82d77e0cf7b4850fddb85d8857a72fe48` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json` | `a530c6260921657b35cec95c3bd0be818e049173c101aefd1d0cfebe5d243188` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🚫️unsupported/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/🚫️unsupported/🔣️.json` | `fa36e55eaa1441897fb167dd8eb603958cfe054899e6db02e6ed824e58456acf` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📣️typescript-declaration-facts/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📣️typescript-declaration-facts/🔣️.json` | `22bf492d7445532cae0d86f2735224d1f0e20dd167a2b02ecb7dbbe4c63b3774` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📥️mutation-input-carriers/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📥️mutation-input-carriers/🔣️.json` | `fd725b2943941b9a16987d1e423eb7910b3ad6bc9f10344fe8e81ac34d5db801` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📦️semantic-package-source-manifest-identity/🔣️.json` | `7ed0a25bfaadaf3ebd1d4372671adc19dd5bcfc78a909ed4314e33ee82e98660` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📦️semantic-package-source-manifest-identity/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/📦️semantic-package-source-manifest-identity/🔣️.json` | `7cd5bd8695c9cb211c47f0981efcf918e382fd1773230a45d8dbb1f678f499ea` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📽️cargo-provider-projection/🔣️.json` | `09ba224a3c48afa1dc25b1050947348e18c45687e90d1b6001f6cb2500d04858` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/📽️cargo-provider-projection/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/📽️cargo-provider-projection/🛂️schema/🔣️.json` | `d99ba6eeefa2a5dfae90fb501168f0954d52f80ef0ddee2419b49cd2717ed194` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔍️filesystem/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔍️filesystem/🔣️.json` | `dcf5a89d8fb604a26292fe1dcf98f9c592102428987e263382b0b462e734f79c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/📍️projection-scope/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔎️json-reference-owner-lookup/📍️projection-scope/🔣️.json` | `b82766a06a84d38fbf5b3268ab4ffb811419d54943c83f5c9334ee44d19d1207` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔎️json-reference-owner-lookup/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔎️json-reference-owner-lookup/🔣️.json` | `dc7392a7c0d977be5a01760d197da6f250309f5fa80e922c8840775a71e4f694` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔏️path-emoji-statutes/🔣️.json` | `1e83832b973af31d0e7322375d75e8c66c26d3e622021a8be699d2f4acd90941` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔏️path-emoji-statutes/🔣️.json` | `054858efecc1df0012ff946211f1d18b1c7a750bfc6ee6497a44c7f4c398f5ad` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔐️mutation-codec-ownership/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔐️mutation-codec-ownership/🔣️.json` | `563b9d8d7f02c74a075032e94fa8f44baa6a4fb7af1871c57270cd71eecd291b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔖️readme-current-source-revision/🔣️.json` | `ffb8c4ac2eec18bbb9f8e5c6bc354cb25d18fb5c93f75be4000cc2d7965f3077` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔖️readme-current-source-revision/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔖️readme-current-source-revision/🔣️.json` | `74a11b1acec0dfaedf9ede0c3aacda0a065e5e3988b99bba32dca2e9e0a38cc6` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔗️markdown-inline-references/🔣️.json` | `5aaac86de980500a854fc82ad0bafd24af5d8b78c7c211e9482e06aab9ec955f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔗️markdown-inline-references/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔗️markdown-inline-references/🔣️.json` | `db2628f8e852a5be1373ee9d2a4efa3970476a729164146b1a89c92a37552c7f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔤️taxonomy-leading-grapheme/🔣️.json` | `a6efad3292b69743af16b62651d77ac45b0ef87bcb0f310d09be1724ea4e35cf` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔤️taxonomy-leading-grapheme/🔣️.json` | `b9e89a207befc0097687f0daf47dcd0738542c23717b6c02a3277f6b1bad9290` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json` | `c927089d7c846a17714bf0ab9c7b30dfbce202aa9c37d6c2a045a2dfbbf75851` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔤️taxonomy-leading-grapheme/🧪️registration/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🔤️taxonomy-leading-grapheme/🧪️registration/🔣️.json` | `aa849e781be036259cde3cde8cb2187e34f59edf7f60d4d60cbe5bc2fb308ee0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔭️mutation-scope/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🔭️mutation-scope/🔣️.json` | `ba6cfa694520ef7e8a1825cb44bd4102b6e0ca829ba1a24775704ed0cf48b611` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕰️historical-json-source-encoding/🔣️.json` | `c3703720d4add142f426284f31d2bb16340b09f7fafe7593b1ee6f500083146d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json` | `1ee0b877806e56d89615ae0111e7f15a80670e0a283ca17f73210c77004b3577` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json` | `722fbe62cfb378f302fd3b961199046365dcc486f97e290c5fef53556aa8e3bd` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/📨️submitted-proof/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/📨️submitted-proof/🔣️.json` | `2f6c44c37ff9e44930a0aa713f0b4a37ebab97603be91c89952c13d3b6155056` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🔣️.json` | `1080844b8f61bf3e81060458c9e3e5563c299a08ee12316a4533ef8b05290c08` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json` | `2525d0c0edd8d79d83c7bb1585ae3575e1f5fadaa7304c5a95daa6e63555cd4b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🙈️residue-ignore/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🙈️residue-ignore/🔣️.json` | `4c03dbe39c183696dbbfa5b519e487c168ca52a9053fd733b273fd72949a91e9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🔣️.json` | `d60311d8c650616bef510feab69f596e0b937400af70f783e094459910086203` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json` | `8fa19dd60838ae58f41e1922e7e2b37fe90fd91b8e8be6de597edb492248e362` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛟️residue-recovery/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🛟️residue-recovery/🔣️.json` | `0886c6373e29a9ca9c76ea77fe3561991ce51a34d1688580468b07e56f847c1f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🛣️commit-route/🔣️.json` | `1625938a389d914ea7aaeb28d140fc87659c8938c240970dd2b7eb48f0b41419` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛣️commit-route/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🛣️commit-route/🔣️.json` | `44fb835368af5ba0d415b6c2ec8a86a5f8e56e6b887b0610f5976c5910f6f29a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json` | `e5d91f7e55fa7402ea6aa3c360d2e1e755640ad7ecc41c6b81de8cb195ca5bad` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖍️draw-source-scenario/🛫️preflight-context/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖍️draw-source-scenario/🛫️preflight-context/🔣️.json` | `35cceb3194adc7d52682b6aa1716450fb401f1ccb45a5b9ac4a49fd706cf8ade` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖼️assets/🗺️testing-readme-coordinates/🔣️.json` | `f6fc7fc2be5fae77247880672a300a3bd9e682d34c157f04c8a444122cffc9ab` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🗺️testing-readme-coordinates/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🗺️testing-readme-coordinates/🔣️.json` | `829f8e2ccbc230643f695543c883660662bb34a5e0883928a278555edefe6221` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚚️readme-move-source-authority/🔣️.json` | `c4bf57b186e9d2f7afe38a9544a551359121bf2acd209f1bf841dddc6d5713a1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚚️readme-move-source-authority/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🚚️readme-move-source-authority/🔣️.json` | `a6d644c96479a488d54ec96dcb1f5b9ead24963018418ab96e07097927287559` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🚧️cargo-discovery-exclusions/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🚧️cargo-discovery-exclusions/🔣️.json` | `0a622422c4164e5c0740f2f3135af79a2584232995e7d4853702f3fa5d9b7ff2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛟️transaction-recovery-authority/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛟️transaction-recovery-authority/🔣️.json` | `6df797889862f5cef95454b72f987a1ace04f14451050063cfad75c75bfad6c0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛤️typescript-path-collection/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛤️typescript-path-collection/🔣️.json` | `b031f9bb3fbd859143eb0ab1ea8917e04d5cb6fbbd14abe0d6d211c75ecada8d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🛫️preflight-reference-basis/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🛫️preflight-reference-basis/🔣️.json` | `37760fbedf095cf467dc4e331d535c44e8f8b4dfbbf00cc75c06fc732d4caf9a` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🟢️readme-current-source-activation/🔣️.json` | `a1fecb7faeab7daa55bc2cc90db9ef349b951cd0c23f90fabb4afbb8b1e209cd` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🟢️readme-current-source-activation/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🟢️readme-current-source-activation/🔣️.json` | `d73a795b0aa9210848f6f5a7e880973e5b6b25ca7d747bb2d61111f9e43d560d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json` | `1d0feb5a78eeef2a08c62348072fe0e4fa9fc38668e323870e5c4f7cab48b2ee` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/💾️resident-package/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/💾️resident-package/🔣️.json` | `bf98144ef0aea90b541e0ef7337b256b843c1f36e7722009769387a865a6045d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/🔣️.json` | `cd65c79dfd036b8d3abc4586a0d6ef1cef0f66b912d601ed1ad659ff84260a07` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json` | `f7cf6fa7575620cacdeef74a0543db1065a4f1762f64c821a8525e987cb9dcf7` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json` | `887b17b65052ca6a1ad89eaf2e385fca490307b24f2f55bf3c4c57be3f7f24d8` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🤝️package-language-kind-handoff/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🤝️package-language-kind-handoff/🔣️.json` | `6b96e680de90488e1de57125ac603ec0ddf228358cd2a97d6a5431959e377038` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥒️gherkin-description-inline-code/🔣️.json` | `18221e8df384c6bc753bb409c3bc337515742a27f8a11ff7998feb2d1f5270ca` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥒️gherkin-description-inline-code/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🥒️gherkin-description-inline-code/🔣️.json` | `76911bb507747cec4dc5930d0aa70b2ccbb033946733e108819dc5ef86efe197` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🥤️rust-finite-target-consumption/🔣️.json` | `06d35fa876489819c61a01919166e705223c7b76fee908a0681aca335583f86d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🥤️rust-finite-target-consumption/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🥤️rust-finite-target-consumption/🔣️.json` | `3953f9df6efd9a40a2708fd19483edf1c37cdf17b7f33bceb3b75a8413d0ec7b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🦀️exact-cargo-laws/🔣️.json` | `8aeaf280b649ce7f3654f1ce0dad39a3790de0604fbe190522e95ea0c09cd647` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🦀️exact-cargo-laws/🧪️fixture/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🦀️exact-cargo-laws/🔣️.json` | `5475d3dc9fb1a7de4f86cf78e4da6a0fe9f7ff98440f3853592c8994e904173f` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-type-origin/🔣️.json` | `57d4cfed0458fb6872dce19a0948f46f88f8f78a4d2947cbc335711b512abb1d` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️mutation-type-origin/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️mutation-type-origin/🛂️schema/🔣️.json` | `58ecc86a6dd4ac2f5ac0d899fdc5b046ea9ee632931bd304336977a37d7ebaff` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-rust-entries/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️schema-rust-entries/🔣️.json` | `f6f2ec51f4f969a617794f1f5697b2551a0ce7ef0c60e62a2e8e495834efcc54` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-rust-entries/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️schema-rust-entries/🔣️.json` | `3dc27ab7ab498c377a82218c43117c239919b8dfa709e2031ae9c11ab4d502a9` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧬️schema-scope-catalog/🔣️.json` | `1a522bb1cf61aa3485861362029d3b24dfe2f55110a13a817af23beaad76b8a0` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧬️schema-scope-catalog/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧬️schema-scope-catalog/🔣️.json` | `e2de108bf6713c2f93fe9a7905e671066af3ae1948647dcb52acb121d6435a37` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧼️clean/🔣️.json` | `91e579933d5a3156fc570711f40040157b9d8a378a6cdcf0abfc759438446782` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧼️clean/🧫️fixture/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧼️clean/🔣️.json` | `3dcaf303e788d178c45b4a236d526acaf5a68edf3e2ec5cbbf45543ae1149fa1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧾️registry-catalog-gitlink-boundary/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🧾️registry-catalog-gitlink-boundary/🔣️.json` | `de82b42884d46965ea0c6b58c1ffc308077d4b140788445369af40a6ae238a53` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪟️windows-checkout-paths/🔣️.json` | `6ae6a14d3002ba3baca872f158c58a1c5aee404f76fb5a27df5fbc7f152c8298` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪟️windows-checkout-paths/🧫️fixture/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪟️windows-checkout-paths/🔣️.json` | `2201fc2d34a9fa014554461d960c0c0ff9015f583c0de20dfab47733e48df7ae` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️cargo-provider-binding/🔣️.json` | `47b985f8138b478c3fbf2d93c37e6f9e74b7d569caa821f2bcae35f9e4d8d709` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪢️cargo-provider-binding/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪢️cargo-provider-binding/🛂️schema/🔣️.json` | `c5d61a242e5c90320a374c307c671a53d329da433aeb769e9a2b89124423f8af` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪪️mutation-metadata/🛂️schema/🔣️.json` | `f5bcb853cf86f25418ebe4b915d369fdcff3f6627a1f394ebbe58c463e0bc6f2` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪪️mutation-metadata/🧫️fixtures/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪪️mutation-metadata/🔣️.json` | `601a17c130dcc031ab52acb097e19c00543ba252e2bddc5082e62c7db5bb9535` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json` | `ccc39b164290a3526e137f7a888f7dfdc9a869a03ac172979936e622cc18d12e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📋️registration/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/📋️registration/🔣️.json` | `bcbcfbc7bc9b20d1309123350afbeb6dee43e62e5da49a389c7697baa71165a1` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json` | `8560918486a6b647bb5f8279beac3b6f1d59a34b39af132d42aabbcf4892895c` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/📨️request/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/📨️request/🔣️.json` | `11d0556dc1f054c028d7fcee0fc975daf7c82335e4d10b1062c3099677246cdf` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🪶️artifact-empty-facet-authoring/🔣️.json` | `1fb63fbca082e55fd8f4e0142fb93839aa71cb4b6fcdad5a8049a4c107582ec3` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🪶️artifact-empty-facet-authoring/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🪶️artifact-empty-facet-authoring/🔣️.json` | `3f484220d6a92c7455a5be0d0c0546ee65966f46e62860c1f953654265c61763` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/☑️options.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/☑️options.json` | `86c1776f2d3aece83e8ad4fc8d76cdfe5f246321d44ca94ae7c038468f53d89b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/☑️options.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/☑️options.json` | `2aae97fed6d395383d92db47b7022fe596e6328347c19a34605181bafe59c55e` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/🔣️.json` | `f93e4a4ac980f2b8f7851dc15337809528d1727c12eb66ea8c7a607e72b887e6` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json` | `da200400462a77566008728bafb912b72242e44b65af5c48253ee8d17e83cba6` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🫙️artifact-empty-facet-authority/🧪️registration/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🫙️artifact-empty-facet-authority/🧪️registration/🔣️.json` | `e8be548b6f392299ed519e629767c1fcade545e1906bdeff4b71f4c9d4f0fdff` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/💥️generic-stem-collision-resolution/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/💥️generic-stem-collision-resolution/🔣️.json` | `2fba6e003cc62738fa70b65c94b9685c499d4badad891bdfe7c243e4454d6320` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/📦️package-boundary-classification/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/📦️package-boundary-classification/🔣️.json` | `080d4fcccc337b29e264edf1d9d06dae86381a71a2070ba32808a19b41825b7b` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission/🔣️.json` | `d21812ddff8c02c182807df584b7ac145978d7c8541c9b365bd996cc3504ebdf` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧫️fixtures/🚪️source-admission/🧪️io/🔣️.json` | `799b3c4c82de5fc4851f2c8cf251becdfc491a6c499cfb80908e6909121af045` |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧪️tests/🚪️source-admission/🧪️io/🛂️schema/🔣️.json` | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🧬️schema/🚪️source-admission/🧪️io/🔣️.json` | `0f54f3fc40a8b5c103aa65528c06ff77e8b6fe771c712032cc925f2af5bd1161` |
