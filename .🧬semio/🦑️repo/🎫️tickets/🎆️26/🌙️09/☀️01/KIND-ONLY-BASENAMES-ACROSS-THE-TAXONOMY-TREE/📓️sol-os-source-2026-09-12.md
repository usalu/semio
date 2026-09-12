# OS and Hub Source Ownership Execution

Date: 2026-09-12

## Scope

This batch closes the 30 authored OS and hub source leaves identified by the OS runtime follow-up packet. It moves semantic implementation out of language package directories or semantic filenames into neutral concern owners with anonymous implementation leaves. It also separates the WGPU Node package from the WGPU Cargo package, closes active consumers and generator metadata, and records portable and native evidence.

The batch does not decompose the large hub server body or the large OS development script. Persistent Flow, JCO and distribution producer work remains in its separate metadata packet. The session producer was changed by the session-fixture isolation executor; this batch only preserved and rebased its source references.

## Ownership decisions

- Executable composition lives under semantic bootstrap or CLI/binary owners; Cargo manifests keep only explicit target wiring.
- The existing local credential transport owner remains unchanged. Executable server composition moved to the distinct bootstrap owner.
- TypeScript package roots retain only package adapters and metadata. The hub package root now re-exports the substantive integration harness owner.
- The descriptor library body moved to descriptor-emission while the Cargo package keeps its native library identity through an explicit `[lib].path`.
- The actor-import guest is source-as-data, but its authored Rust body still follows the same owner/anonymous-leaf rule. Cargo points to the fixture owner's `🦀️.rs`.
- The WGPU TypeScript adapter and Node/Nx tooling form a separate TypeScript package beside the Rust package. Cargo, Trunk, the Rust library/binary adapters and `build.rs` remain in the Rust package.
- Package-root bodies were retained only where they are thin package glue. No compatibility aliases or fallback paths were introduced.

## Source moves and byte identity

The source hashes were captured from the live files before the move. The current owner hashes were captured after required import, doc-path and package-wiring rebases. The topology test recomputes every current owner hash and asserts each old source is absent, except the explicitly declared hub TypeScript package-glue source. Thus the table distinguishes source-byte provenance from the intentionally rebased current owner bytes.

| # | Original source | Semantic owner | Kind/disposition | Original bytes/lines | Original SHA-256 | Current owner SHA-256 |
| ---: | --- | --- | --- | ---: | --- | --- |
| 1 | 🌎️hub/📦️packages/🦀️rust/🚀️bin.rs | 🌎️hub/🏗️bootstrap/🦀️.rs | rust / removed | 465386 / 8522 | ef9ec7a3c34452ed8c67f23dc2e1b64abff5df27cb034a5ce3034b91a52f4f5f | 4c33d50bcf93594a2f7eb7b278770839089a610743ef5f6fc5f9cfd163e98da4 |
| 2 | 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🚪️index.tsx | 🌎️hub/🔨️modules/🛡️admin/🟦️.tsx | typescript-jsx / removed | 1125 / 28 | 9e04c3dd29a715c7d514a698c451d92b2f5747ddacd40ed8bcff0e91f4c23b37 | 1281624546f8db3326ca8fb8fda750be78efdd6717e50de3a21773e95137bb3c |
| 3 | 🧰️framework/🛍️products/💻️os/⚡️effect-backbone.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone/🟦️.ts | typescript / removed | 32119 / 548 | 598b01321f1a5170510061e74a25f7653945aa52f08a2cea9f1322d2a14d9fdd | e7f128eaef01afa5d167628f044bf98d639456b99e06c2484b1ef345f72ba681 |
| 4 | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🌉️icon-name-value-bridge.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🌉️icon-name-value/🦀️.rs | rust / removed | 30162 / 599 | 9acf0784857c05492b025d8a7b639c918746b069c7f2349d2ae62c5d4a6b7dbd | 9acf0784857c05492b025d8a7b639c918746b069c7f2349d2ae62c5d4a6b7dbd |
| 5 | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔨️bin/📤️dump_guestslim_typst_fonts.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/🦀️.rs | rust / removed | 1198 / 26 | 91f06b069ab14d1a4353969d51e68992d798f8ce58452df2992dac99fa498042 | 91f06b069ab14d1a4353969d51e68992d798f8ce58452df2992dac99fa498042 |
| 6 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/💡️inference-bridge.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference-bridge/🟦️.ts | typescript / removed | 28573 / 405 | d8e769befc825c964e91f96ecbde50a52dce6b426b277fe26df6dbb0bc5c2c25 | 325da505bf03f6948cc9c0aaf4d0a9f91614f3283c3470598733ee14f7676e43 |
| 7 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧬️schema-validation.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema-validation/🟦️.ts | typescript / removed | 1171 / 20 | 432bff29514ca2141c1052b869e1d5b43fee902f753435827fc2c18b1ada1393 | 432bff29514ca2141c1052b869e1d5b43fee902f753435827fc2c18b1ada1393 |
| 8 | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🚀️bin.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs | rust / removed | 9447 / 208 | 2be65a40665654e5507cd6cdf1b34f07c797b84f35efe571c9e1dcdaafb9a940 | 63d163328b76ae79b6069b5899a53d21421b5f85beef1532b8e8900f539a31d4 |
| 9 | 🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/📦️main.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/⌨️cli/💾️binary/🦀️.rs | rust / removed | 281 / 6 | 11a22f8913b309f8eb19c110ff587179d6d26c824a06f1f23ed767498efc3f5b | 11a22f8913b309f8eb19c110ff587179d6d26c824a06f1f23ed767498efc3f5b |
| 10 | 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🚀️bin.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/🏗️bootstrap/🦀️.rs | rust / removed | 21412 / 360 | 31763e3bed568c991a4f78c5fbde83a95b1f7308678868ff243a64e2442a90df | 31763e3bed568c991a4f78c5fbde83a95b1f7308678868ff243a64e2442a90df |
| 11 | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/📦️main.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/⌨️cli/💾️binary/🦀️.rs | rust / removed | 280 / 6 | 7199d59e59194bc016649489ce70575dc28b49212bbcabd9a1cdc5ba2afd2410 | 7199d59e59194bc016649489ce70575dc28b49212bbcabd9a1cdc5ba2afd2410 |
| 12 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🏦️.tsx | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🌱️artifact-creation/🟦️.tsx | typescript-jsx / removed | 9702 / 210 | a8b911ea0a51759c687f2370d1eaa16711e02c9f38eaa58abfb7936a8dfb57f5 | a8b911ea0a51759c687f2370d1eaa16711e02c9f38eaa58abfb7936a8dfb57f5 |
| 13 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/packed-text.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🧳️packed-text/🟦️.ts | typescript / removed | 473 / 15 | 52e87eabcedefe6f0919dcd3234358d779d704d7603e8a377f43256b719000fe | 52e87eabcedefe6f0919dcd3234358d779d704d7603e8a377f43256b719000fe |
| 14 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧵️worker.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts | typescript / removed | 6405 / 85 | c4d40508641667ce47250691f4b97a1d329f0baccfc943beabae0d2581b306f5 | c9571c0b62057f6f3341faf12c950334e5e336a88d1296d4f22d35d7522ce7b3 |
| 15 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️store.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts | typescript / removed | 22857 / 450 | cd57ae8b68bb25dcce795bdbdd586ef64b0ac87dacabfd8d1d71cfd2af564821 | c4f2506fe5831f34188f99a56ac648ba42053da6e9adb9f0be7fc98343ae9c12 |
| 16 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🖥️launch.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts | typescript / removed | 18714 / 266 | bca32d393a929d563621172e7e885b74e987b43fba405909889361591e232aca | 788e8566187826a685d81c0034d453b5e976b02c52d49e9f61bcfb854abf386c |
| 17 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📦️main.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/💾️binary/🦀️.rs | rust / removed | 535 / 9 | b5a9d2045fb64da888e52c884761489058be88caa6ec5203a76d8094abb9020d | b5a9d2045fb64da888e52c884761489058be88caa6ec5203a76d8094abb9020d |
| 18 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📊️bench-web-harness.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️bench-web-harness/🟦️.ts | typescript / removed | 17083 / 261 | f7a0df44d35007c40f534773e3e4f5f59ec7057ea81a4959dea5f3cd7c1ff208 | d2a99963d51230dfcdae774dfe4de1ed9816427a456aee9774442597293c262e |
| 19 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🔌️vite-plugins.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts | typescript / removed | 51546 / 892 | d5966cd8eb2859fff0e4833f2443edae7f5f3eafff50d26b8082f305d6c3eb9a | ffc2132667295e19d94dcc31d8ac057b30b1917c95d8cbef18ab4fb2bee2de9f |
| 20 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/📦️bin.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🏗️bootstrap/🦀️.rs | rust / removed | 162 / 5 | 1df4594a47d729a78c88dd17e8288db7534eae0c96e4eabd4d357efc47c56ed1 | 1df4594a47d729a78c88dd17e8288db7534eae0c96e4eabd4d357efc47c56ed1 |
| 21 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🖥️associations/🪟️windows/🔗️register-semio.ps1 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🖥️associations/🪟️windows/🔵️.ps1 | powershell / removed | 649 / 10 | 64a3c7ad3199d2fc24cd9498ae444589e719c025c3e9012c5681ed388955dcb0 | 64a3c7ad3199d2fc24cd9498ae444589e719c025c3e9012c5681ed388955dcb0 |
| 22 | 🧰️framework/🛍️products/💻️os/🖥️host/🧫️fixtures/🛡️hostile-batch-edge.rs | 🧰️framework/🛍️products/💻️os/🖥️host/🧫️fixtures/🛡️hostile-batch-edge/🦀️.rs | rust / removed | 1069 / 33 | 1e0309f2617125c8894c6916dd408bd827653508baed261b960039aabf4f7746 | 770685502963f4f83a8b2602986710b0ac4f5116e3067ff2c81610380c4231c1 |
| 23 | 🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🖥️host-shim.js | 🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🟨️.js | javascript / removed | 1141 / 26 | 7d8bcedd26a3f7e1ab9c06a04fd49b5312e43e9444448cd97d2cdcda64bd0b06 | 7d8bcedd26a3f7e1ab9c06a04fd49b5312e43e9444448cd97d2cdcda64bd0b06 |
| 24 | 🧰️framework/🛍️products/💻️os/🧵️backbone-worker.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | typescript / removed | 333475 / 6231 | 8e3be6116b6423d004d2c83ccc4655bf25af3930782fc903dd10d4047ce7713a | 41451614dc3e7b5f48d35102a3f4f03f48d466e91d0bffb5024240043cf422ad |
| 25 | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/🟦️typescript/📚️library/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📚️library/🟦️.ts | typescript / removed | 102 / 3 | 6cf64f124834b613b07fbb973504be538c3442093cc6ed5078bfeea28c748c0a | e8c2fe73f1ee90e6aa8aac43db6e652d9176abb0569d6daeaba528c01eff5f15 |
| 26 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧬️session.d.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.d.ts | typescript-declaration / removed | 144 / 4 | 50e1da46fed6f7800eaf361d245c2065af54ee2fd9e2b5719483dcba8c5c1089 | 50e1da46fed6f7800eaf361d245c2065af54ee2fd9e2b5719483dcba8c5c1089 |
| 27 | 🌎️hub/📦️packages/🟦️typescript/🟦️.ts | 🌎️hub/🤝️integration-harness/🟦️.ts | typescript / package-glue | 12437 / 247 | cb751f755848ab9ebe38644561bea8c5b247d8a451c445ca580835926e70705e | f168376029b72459fcc8f6b82f0153ed1e36254308c09aaa74132a7d177829a2 |
| 28 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs | rust / removed | 30813 / 520 | 64d561dc30b9883156a7a3ed942e148991fdb8e3ac4b6737252d15cfb08876f8 | 95055d1e4e7c032bebfe216a9acc4bb15480e60a7ad906ccdf6c5e33b3eb81d9 |
| 29 | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧫️fixtures/🌊️actor-import/👽️guest/📦️packages/🦀️rust/📚️library/🦀️.rs | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧫️fixtures/🌊️actor-import/👽️guest/🦀️.rs | rust / removed | 1153 / 40 | 3276feb9a423a52d2ea5d96faae813af1b1e519a742be5b0fe2bade947e872a2 | 3276feb9a423a52d2ea5d96faae813af1b1e519a742be5b0fe2bade947e872a2 |
| 30 | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/🧹️executable-source/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧹️executable-source/🟦️.ts | typescript / removed | 217 / 4 | 6828ea4e15206d5dd60673dcd2310c2fbc51639fe1be29f781af79c573917717 | 6828ea4e15206d5dd60673dcd2310c2fbc51639fe1be29f781af79c573917717 |

Four package-body cases called out during coordination are explicitly represented:

1. `🌎️hub/📦️packages/🟦️typescript/🟦️.ts` remains as a thin re-export adapter; the integration harness body is under `🌎️hub/🤝️integration-harness/🟦️.ts`.
2. `.../🔌️plugin/🖨️describe/📦️packages/🦀️rust/🦀️.rs` moved to `.../🛂️descriptor-emission/🦀️.rs`; Cargo retains the library identity using the owner path.
3. The actor-import guest package library moved to the fixture owner `.../👽️guest/🦀️.rs`; its WIT path remains manifest-relative and resolves to the same schema.
4. The development package's executable-source body moved to `.../🧑‍💻dev/🧹️executable-source/🟦️.ts`.

## Portable contract

The language-neutral contract consists of:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🖥️os-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🖥️os-source-topology/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🖥️os-source-topology/🟦️.ts`

The fixture records all 30 source/owner pairs, implementation kinds, dispositions, pre-move and current hashes, semantic anchors and consumers. Its WGPU package boundary records distinct Cargo and Node roots and the retained npm identity.

The test uses Ajv as an independent JSON Schema oracle, Bun's compiler as an independent parser/compiler for every moved TypeScript source, and `@iarna/toml` as an independent Cargo-manifest oracle. It checks anonymous basenames, source removal/package-glue disposition, exact owner hashes, anchors, consumer existence, distinct WGPU package identities, npm export identity, and the existence of every configured Cargo target source.

Registered command closure:

- Repo library `📜️script.ts`: `test os-source-topology`
- Repo library `📋️project.json`: `test-os-source-topology`
- `.vscode/🧩️launch.seed.jsonc` and generated `.vscode/launch.json`: `🧹clean🧩️taxonomy🧪️os-source-topology`

## WGPU package and generator closure

The Node package is now:

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript`

It owns `package.json`, `📋️project.json`, `vitest.config.ts`, `📜️script.ts`, and the anonymous adapter `📚️library/🟦️.ts`. The sibling Rust package retains Cargo/Trunk/build metadata and anonymous Rust target adapters.

The renderer package catalog now carries an explicit `nodePackageRelativePath`. The renderer schema, discovery parser, projection validation, root generator and package generator all consume that distinction. Root `package.json`, `bun.lock`, repository package lookup, browser-cache fixture, Nx cwd values and the package export were rebased to the Node root. Cargo operations continue to use the Rust root.

The package frame-worker generator now delegates rendering to the root `renderWgpuPackageArtifacts` authority and selects the canonical frame-worker artifact. This removed the two independently rendered bundle variants. The browser carrier check still rejects credential, query, session and document-cookie carriers; it permits the existing diagnostic-toggle `localStorage` use because that value carries no credentials.

Current catalog digest:

`5a4c510292d2c1f56aad20fe33adfde7b4313dc118e30ae5eb3087919c43b7da`

## Schema membership and scoped inventory

The shared taxonomy adds only exact, parent-scoped semantic directory members:

| Kind |
| --- |
| os-hub-integration-harness |
| os-module-effect-backbone |
| os-canvas-icon-name-value |
| os-mcp-inference-bridge |
| os-mcp-schema-validation |
| os-renderer-artifact-creation |
| os-renderer-packed-text |
| os-plugin-installation |
| os-registry-launch |
| os-descriptor-emission |
| os-dev-bench-web-harness |
| os-dev-vite-plugins |
| os-dev-executable-source |
| os-hostile-batch-edge-fixture |

`loadTaxonomy()` and `validateTaxonomy()` return `[]`. Scoped `inventoryTaxonomy` checks returned zero violations for each new owner context: hub integration; MCP inference and schema validation; plugin effect, installation, registry launch and descriptor emission; development bench, Vite plugins and executable source; hostile-batch fixture; infinite-canvas icon bridge; renderer artifact creation and packed text.

## Consumer closure

The portable fixture retains the following exact consumers. Each exists in the final tree, and imports, Cargo target paths, include paths, project roots, docs and producer dependencies were rebased where the moved leaf changed their referent:

- ♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts
- 🌎️hub/📦️packages/🟦️typescript/🟦️.ts
- 🌎️hub/📦️packages/🦀️rust/Cargo.toml
- 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🌐️.html
- 🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts
- 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧫️fixtures/🌊️actor-import/👽️guest/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📋️project.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/vitest.config.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/⚙️vite.config.ts
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json
- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts
- 🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/Cargo.toml
- 🧰️framework/🛍️products/💻️os/🟦️.ts
- 🧰️framework/🛍️products/💻️os/🧫️fixtures/🧩️jcoprobe/🌐️browser-host/🌐️.html

Additional WGPU and command authority files changed by this batch:

- `package.json`
- `bun.lock`
- `📜️script.ts`
- `.vscode/🧩️launch.seed.jsonc`
- `.vscode/launch.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪪️package-catalog.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts`
- WGPU TypeScript package: `package.json`, `vitest.config.ts`, `📋️project.json`, `📜️script.ts`, `📚️library/🟦️.ts`
- Removed from the WGPU Rust package: `package.json`, `vitest.config.ts`, `📋️project.json`, `📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/⚙️vite.config.ts`

A final 47-row local module-specifier check verified that every moved owner exists, contains its recorded replacement specifier, and resolves every relative referent to an existing file. The active demonstrator Vite config was found by the final old-path scan and rebased to the development Vite-plugin and installation-store owners.

### Rebased local referents

| Owner | Original specifier | Final specifier | Final referent |
| --- | --- | --- | --- |
| 🌎️hub/🔨️modules/🛡️admin/🟦️.tsx | ../../🧱️elements/📚️I18n/🟦️.tsx | ./🧱️elements/📚️I18n/🟦️.tsx | 🌎️hub/🔨️modules/🛡️admin/🧱️elements/📚️I18n/🟦️.tsx |
| 🌎️hub/🔨️modules/🛡️admin/🟦️.tsx | ../../🧱️elements/🔑️AdminSession/🟦️.tsx | ./🧱️elements/🔑️AdminSession/🟦️.tsx | 🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx |
| 🌎️hub/🔨️modules/🛡️admin/🟦️.tsx | ../../🧱️elements/🛡️AdminApp/🟦️.tsx | ./🧱️elements/🛡️AdminApp/🟦️.tsx | 🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️.tsx |
| 🌎️hub/🔨️modules/🛡️admin/🟦️.tsx | ./🎨️.css | ./📦️packages/🟦️typescript/🎨️.css | 🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🎨️.css |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone/🟦️.ts | ../../🔨️modules/🎭️actor/📬️mailbox/🟦️.ts | ../../../../../🔨️modules/🎭️actor/📬️mailbox/🟦️.ts | 🧰️framework/🔨️modules/🎭️actor/📬️mailbox/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone/🟦️.ts | ./🟦️.ts | ../../../🟦️.ts | 🧰️framework/🛍️products/💻️os/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone/🟦️.ts | ./🟦️.ts | ../../../🟦️.ts | 🧰️framework/🛍️products/💻️os/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚡️effect-backbone/🟦️.ts | ./🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts | ../../../🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts | 🧰️framework/🛍️products/💻️os/🧪️tests/🧪️effectbackbone-capability-gating/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference-bridge/🟦️.ts | ../../🧬️schema/🟦️.ts | ../🧬️schema/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/👷️worker/🟦️.ts | ./🧬️schema/🟦️.ts | ../🧬️schema/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts | ./🟦️.ts | ../🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts | ../../🧩️extension/🟦️.ts | ../../../🧩️extension/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🧩️extension/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts | ../📇️registry/📦️deployment/🟦️.ts | ../../📇️registry/📦️deployment/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts | ../📦️packages/🟦️typescript/🟦️.ts | ../../📦️packages/🟦️typescript/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts | ./🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts | ../🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️tests/🧪️authored-extension-installation-identity/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🚀️launch/🟦️.ts | ./📜️script.ts | ../📜️script.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️bench-web-harness/🟦️.ts | ../../../../../../../🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts | ../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts | 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️bench-web-harness/🟦️.ts | ../../../../../../../🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️.ts | ../../../../../🔨️modules/🌱️value/💾️resident/🟦️.ts | 🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts | ../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts | ../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts | ../../♻️activation/🟦️.ts | ../♻️activation/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts | ../../../🔌️plugin/🏪️store/📥️store.ts | ../../🔌️plugin/🏪️store/📥️installation/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/📥️installation/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🟦️.ts | ../../../../../../🔨️modules/🔏️hash/🟦️.ts | ../../../../../🔨️modules/🔏️hash/🟦️.ts | 🧰️framework/🔨️modules/🔏️hash/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts | ../../📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🪪️session-authority-v1/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🟦️ | ../../../🟦️ | 🧰️framework/🛍️products/💻️os/🟦️ |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🟦️ | ../../../🟦️ | 🧰️framework/🛍️products/💻️os/🟦️ |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🟦️ | ../../../🟦️ | 🧰️framework/🛍️products/💻️os/🟦️ |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ../../../🌎️hub/💡️inference/🧬️schema/🟦️.ts | ../../../../../../🌎️hub/💡️inference/🧬️schema/🟦️.ts | 🌎️hub/💡️inference/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts | ../../📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🌱️space-artifact-creation-v1/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧾️describe/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🧵️child/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ../../🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts | ../../../../../🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts | 🧰️framework/🔨️modules/🎭️actor/📥️cold-pair/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts | ../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts | 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts | ../../../../../🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts | 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts | ../../../../../🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts | 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts | ../../🔌️plugin/📡️backbone/🔗️binding/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🟦️.ts | ../../🔌️plugin/🌐️browser-bundle/🪟️view-context/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🪟️view-context/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ../../🔨️modules/🛂️manifest/🟦️.ts | ../../../../../🔨️modules/🛂️manifest/🟦️.ts | 🧰️framework/🔨️modules/🛂️manifest/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/📇️directory/🧬️schema/🟦️.ts | ../../📇️directory/🧬️schema/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/📇️directory/🧬️schema/🟦️.ts | ../../📇️directory/🧬️schema/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🔨️modules/📇️directory/🧬️schema/🟦️.ts | ../../📇️directory/🧬️schema/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️ | ../../../🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️ | 🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🪪️sign-in/🟦️ |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts | ./🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts | ../../../🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts | 🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📚️library/🟦️.ts | ../../../../🎬️renderer-boot/🟦️.ts | ../../../🎬️renderer-boot/🟦️.ts | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🎬️renderer-boot/🟦️.ts |

The final exact old-path scan found no active source consumer. It found only two retained authority datasets: one row in the frozen remaining-package-purity authority fixture and historical launch-path rows in the generated nested-Cargo package projection. Those datasets are owned by the separately coordinated authority/projection lane and were not rewritten here.

## Native package target identity

`cargo metadata --no-deps --format-version 1` exited zero for all nine manifests checked. The target paths relevant to this batch were:

| Manifest | Retained targets |
| --- | --- |
| `🌎️hub/📦️packages/🦀️rust/Cargo.toml` | lib `🦀️.rs`; bin `os-hub=../../🏗️bootstrap/🦀️.rs` |
| OS kernel Cargo | lib `🦀️.rs`; bins `pack`, `semio`, and `spr` at their new semantic owners |
| Infinite Cargo | lib `🦀️.rs`; bin `dump-guestslim-typst-fonts=../../🖼️canvas/🔤️fonts/🦀️.rs` |
| MCP Cargo | lib `./🦀️.rs`; bin `semio-os-mcp=../../🏗️bootstrap/🦀️.rs` |
| Run Cargo | lib `./🦀️.rs`; bin `semio-framework-os-run=../../🏗️bootstrap/🦀️.rs` |
| Describe Cargo | lib `../../🛂️descriptor-emission/🦀️.rs`; bin `semio-framework-plugin-describe=💾️binary/🦀️.rs` |
| OS host Cargo | retained lib `🦀️.rs` |
| Actor-import guest Cargo | cdylib at `../../🦀️.rs` |
| WGPU Cargo | cdylib/rlib `📚️library/🦀️.rs`; bin `💾️binary/🦀️.rs`; custom build `build.rs` |

The actor-import WIT declaration remains `🧬️schema/📜️world.wit` relative to its Cargo manifest, so the referent identity is unchanged.

## Validation results

| Check | Result |
| --- | --- |
| Strict taxonomy load and validation | Pass, `[]` |
| Scoped inventories for every introduced owner | Pass, zero violations |
| `@semio-tech/repo-lib:test-os-source-topology` | Pass, 5 tests, 246 expectations, 149 ms test / 429 ms Nx |
| Current owner/source/specifier identity | Pass, 30 owner rows and 47 relative referents |
| Cargo metadata | Pass, nine manifests |
| `@semio-tech/framework-os-kernel:check` | Pass |
| `semio-framework-os-infinite:check` | Pass |
| `@semio-tech/framework-os-host-rs:check` | Pass |
| `@semio-tech/framework-os:generate-wgpu` | Pass, six exact artifacts, two updated during the generation run |
| `@semio-tech/framework-os:check-wgpu` | Pass, six exact artifacts current |
| `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | Pass |
| `@semio-tech/framework-renderer-wgpu:check-frame-worker` | Pass, canonical frame-worker current |
| `@semio-tech/plugin-registry:generate` | Pass, two tasks, 17.8 s; catalog and launch regenerated |
| `@semio-tech/plugin-registry:check-generated` | Pass, two tasks, 17.7 s; catalog and launch bytes fresh |
| Focused OS effect/Rust parity filter | Pass, all three moved-source referent tests |
| Isolated session fixture, direct route | Pass, 2/2, recorded in `📓️sol-session-fixture-isolation-2026-09-12.md` |
| Isolated session fixture, registered Nx route | Pass, 2/2, three tasks, 25.2 s, recorded in the session report |
| WGPU browser boot cache oracle | Pass; Bun and esbuild positive controls each intercepted the canonical input once, both WGPU variant builds intercepted zero, parseable bundles identical at 58,152 and 63,603 bytes |

The first full `@semio-tech/framework-os:test-quick` run exposed three effect-backbone parity tests whose Rust source URL still used the pre-move relative depth; the route then reached its registered 30-second budget. The exact Rust referent was corrected to `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`, and a registered focused filter for all three parity cases passed. The entire quick suite was not repeated because the unchanged registered route exceeds its 30-second budget.

The combined native check reached the current MCP crate and reported two feature-level compilation defects:

- `DirectorySessionAuthorityV1` has no `expires_at_ms` field at remote `🦀️.rs` lines 315 and 338; the current field is `expires_at`.
- `ProbeSnapshot` does not implement `ArtifactCompositionFields`, blocking undo/redo calls in workspace `🦀️.rs` lines 1481 and 1493.

The kernel, infinite and host packages passed in the same bounded check. The MCP diagnostics arise after source and Cargo target resolution and do not name any moved source, manifest path or rebased import; this batch records the observed limit without assigning baseline provenance.

## Remaining limits

- The generated nested-Cargo authority projection and the frozen remaining-package-purity fixture still retain historical path rows. Their dedicated projection/body-policy lanes own regeneration or authority updates.
- The current MCP feature errors prevent a green combined four-package native check.
- The registered full OS quick route has a 30-second budget lower than its current runtime; only the three move-sensitive parity cases were rerun after their path repair.
- Ambient taxonomy findings outside the exact introduced owner contexts remain outside this batch.
- Large hub and development script decomposition and the separately identified missing-manifest OS/plugin package body remain follow-up work.

No modifying Git command, AGENTS edit, migration script, compatibility alias or runtime dependency was added.

## Topology invariant correction

The portable OS topology schema, fixture and test no longer make captured implementation hashes a permanent repository invariant. Exact pre-move and owner hashes remain above as audit evidence. The lasting gate checks the portable schema, source disposition, anonymous owner basenames, authored anchors, consumer existence, package identities, compiler acceptance and configured Cargo target existence. The registered `@semio-tech/repo-lib:test-os-source-topology` route passed all 5 tests after this correction.

## Topology invariant correction

The portable OS topology schema, fixture and test no longer make captured implementation hashes a permanent repository invariant. Exact pre-move and owner hashes remain above as audit evidence. The lasting gate checks its portable schema, source disposition, anonymous owner basenames, authored anchors, consumer existence, package identities, compiler acceptance and configured Cargo target existence. The registered `@semio-tech/repo-lib:test-os-source-topology` route passed all 5 tests after this correction.
