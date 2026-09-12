# 🌳️ Current Implementation-Neutral Taxonomy Inventory

Date: 2026-09-12. Scope is the working tree at observation time. The inventory includes `🧰️framework`, `🌎️hub`, `✏️s`, and first-party `♻️mit-bestand`; it excludes the three schema-declared opaque subtrees (`compose/`, `temp/compose/`, and `♻️mit-bestand/🔎️recherche/`) and ticket history. Concurrent edits mean this is an execution packet, not a frozen baseline.

## Statute and the present enforcement gap

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` is explicit:

* `physicalLeafRendering.filename` is `file-kind-emoji-and-extension-chain`.
* Its tree-purity comment says semantic concerns belong in emoji-plus-slug directories.
* The registry has 86 file kinds, 390 semantic directory kinds, 553 fixed external filename contracts, and six configurable entry contracts.

The live canonicalizer does not enforce that rule for implementation leaves. In `📚️library/🧹️normalization/🟦️.ts`, `canonicalFile` preserves every leading-emoji stem before deciding whether it is a semantic filename. It also returns a kind-only target without a violation for `GENERIC_SEMANTIC_STEMS`, including `component`, `implementation`, `index`, `test`, `cases`, and `vectors`. Thus `🦀️component.rs` and `🔁️reconcile.rs` can be reported cleanly even though neither is a kind-only file leaf.

`enforcement` is `null` in the statute. `verify taxonomy report` plans currently existing rules but cannot be accepted as proof of the user goal until the two bypasses are removed or constrained to explicitly structural asset/member contexts.

## Bounded live candidates

The strongest source packet is the stated violation shape, still present under the UI Rust target:

`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/`

It contains 31 semantic Rust leaves, including `🔁️reconcile.rs`, `🧩️component.rs`, `⚙️engine.rs`, `🧵️mounted_layout.rs`, `🔣️icon_name_value.rs`, `📋️draw_types.rs`, and `🌐️locale_terminology_value.rs`. This is a module collection, not an asset collection. Each needs a registered semantic directory (for example `🔁️reconcile/🦀️.rs`) or a deliberate same-domain merge; the Rust module declarations and all path consumers must move with it.

The same issue is broader inside the UI renderer packages: `🖌️render/📦️packages/🦀️rust` has 13 named Rust leaves, and its Vulkan, Metal, D3D12, and WebGPU target packages together expose more named source leaves. These are a separate, large UI execution lane and should not be mixed with the small non-UI cleanup.

Outside UI, two direct source candidates remain in first-party framework product code:

* `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🦀️component.rs`

Both are source leaves and should become `🦀️.rs` after collision and reference checks. They are independent of the UI module-tree move.

First-party `♻️mit-bestand` adds a distinct authored-source packet. The demonstrator has `🎨️globals.css`, `🪧️brand.ts`, and `⚛️footer.tsx`; the `33.projektetage` presentation has 27 named authored TypeScript slide leaves plus `🎨️globals.css`, `📦️index.ts`, and `🌐️index.html`. Its `🌐️public/🧰️zukunft-bau-entwerfen-mit-bestand_files/` contains 12 bundled CSS/JS artifacts plus the rendered HTML document. Treat the 12-bundle population as generated/third-party evidence, not as a rename packet; the 33 authored leaves need their own topology and reference audit.

## Configuration, policy, and schema classes

Do not run the leaf migration over every semantic `.json` filename. The live tree still contains a large data population: 31 `🔣️typology.json` and 9 `🔣️modelDefinition.json` files, chiefly CAD record collections. Their identity must first be classified as either structural collection membership or a schema record that needs a semantic directory. They are not implementation source merely because JSON is executable-adjacent.

The same separation applies to `🔣️taxonomy.json` and `⚡️caching/🔣️policy.json`: they are authority/configuration records. Fixed tool names (`Cargo.toml`, `package.json`, `project.json`, `tsconfig.json`, `vitest.config.ts`, and comparable declared contracts) require no migration. Cargo `build.rs` also remains externally imposed: prior ticket evidence verified Cargo derives a valid crate identifier from this filename and rejects an emoji basename.

At repository root, `📜️script.ts` is explicitly mandated by `AGENTS.md`; it is not a domain source violation. Root manifests and editor configuration are covered by fixed-contract/tooling classifications. `.storybook` stories and editor plans are tooling/historical-document populations, not a substitute for a domain implementation tree.

## Package-language descendants: separate audit universe

Across `🧰️framework`, `🌎️hub`, and `✏️s`, the scan found **16** `📦️packages/<language>` roots with files more than one descendant segment below the language directory, covering **92** files at observation time. These are not automatically violations: `packageBoundaryRules` explicitly permits certain descendants (`targets`, fixtures, apps; Rust also library/binary/builder). They must nevertheless be checked recursively for kind-only leaves.

| Bucket | Files | Package roots | Disposition |
| --- | ---: | ---: | --- |
| UI Rust WGPU target | 32 | 1 | Leaf violation packet: 31 named module leaves plus one bare Rust leaf. |
| UI TypeScript React target | 14 | 1 | Three named source leaves (`runtime`, `render`, `build-tooling`) and target-local tool configuration. |
| Repo server Next application | 13 | 1 | `app/api/.../route.ts`, `page.tsx`, and `layout.tsx`; framework-imposed paths need explicit fixed-contract coverage, not a blind rename. |
| Renderer React target | 6 | 1 | Package/target assembly; kind-only implementation leaf is present. Audit only after the UI lane settles. |
| Puzzle React target | 5 | 1 | Package/target assembly; kind-only implementation leaf is present. |
| WGPU nested Cargo package | 7 | 1 | `library`, `binary`, `builder`, and generated TS bridge structure. Most leaves are already anonymous; preserve the Cargo-required `build.rs`. |
| Remaining fixture, oracle, VS Code, dev, plugin, actor, and stdio descendants | 15 | 10 | Small packets. Classify fixture/schema identity before moving; do not treat test artifacts as source. |

The boundary rule is a useful admission constraint, but it does not enforce the user’s rule inside an admitted package descendant. The policy must inspect source-role leaves recursively after package admission, while continuing to exempt external fixed-name contracts.

## Independent execution lanes

1. **Gate and statute lane.** Make source-role leaves reject semantic basenames regardless of an emoji prefix; remove the generic-stem escape for source roles. Retain only an explicit, tested structural-member exemption for asset/fixture collections. Add report-mode fixture cases for `🦀️component.rs`, `🔁️reconcile.rs`, a named icon asset, `route.ts`, and Cargo `build.rs`; then wire enforce mode into the repository gate.
2. **Core non-UI source lane.** Move the two `🦀️component.rs` leaves above to `🦀️.rs`, update Rust module/path consumers, and run their owning package tests. This is small and independent.
3. **UI source topology lane.** Move semantic Rust leaves in WGPU, renderer core, and backend target collections into registered semantic directories with anonymous `🦀️.rs` leaves. Split by target family to avoid overlapping module declarations.
4. **Package-boundary classification lane.** Add an inventory/policy that emits `package-semantic-source-leaf` for a named source descendant below every admitted language package. Classify the 92-file universe into external framework conventions, Cargo entry/build requirements, generated artifacts, fixtures, and real source moves.
5. **Mit-Bestand authored-source lane.** Separate authored demonstrator/presentation sources from the emitted public bundle, relocate the 33 authored semantic leaves using registered directories, and update the presentation build references. Keep `🔎️recherche/` opaque.
6. **Schema-record lane.** Decide whether CAD typology/model-definition JSON names are collection identities or location errors; encode that decision in `semanticDirectoryMemberKinds` and fixtures before moving any record.

## Verification limits

This audit used the live taxonomy registry, its canonicalizer and package-boundary rules, and tracked/unignored `rg --files` inventories. A direct `verifyTaxonomy({ scope: "🌎️hub" })` probe did not complete within the 60-second audit budget, so no clean result is claimed here. The coordinator’s full report is the required quantitative baseline after the enforcement fix; this report identifies why that baseline alone cannot prove kind-only implementation leaves today.
