# Sol Generator Renderer Script Extraction

## Scope and result

This lane completed the bounded framework graph/schema renderer extraction. Package `📜️script.ts` files now contain only Bun/Nx command registration. Graph input admission, output-catalog validation, Rust/TypeScript projection, exact publication and command composition have distinct semantic owners with anonymous TypeScript leaves. Schema entity-catalog source/provenance, three-language rendering, output planning and command composition likewise have distinct owners.

The handwritten graph enum bridge moved from the domain-named `🌉️generated-value-bridge.rs` leaf to `🛂️manifest/🔄️value-conversion/🦀️.rs`. The schema source catalog moved from `🔣️entity-kinds.json` to `🏷️entity-kinds/🔣️.json`; its genuinely generated Rust projection moved from `🤖️generated.rs` beside the TypeScript projection at `🤖️generated/🏷️entity-kinds/🦀️.rs`. No compatibility leaves remain at the retired paths.

## Exact owned file map

### Graph source, routing and native bridge

- Added `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📥️admission/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/📤️publication/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏃️execution/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🔄️value-conversion/🦀️.rs`; removed `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🌉️generated-value-bridge.rs`.
- Removed the intermediate monolithic extraction leaf `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🏭️generator/🟦️.ts` after splitting its roles.
- Updated `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📜️script.ts` to the 13-line execution-owner router.
- Updated `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/📋️project.json` with exact cached source/data/test inputs.
- Updated `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs` to include the semantic bridge owner.
- Updated `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🧪️tests/🔬️unit/🦀️.rs` with the 21-family structured-source/wire proof.
- Updated `🧰️framework/🔨️modules/🕸️graph/🧪️tests/🧩️suite/🟦️.ts` with owner imports, hostile admission/atomic-plan controls and Nx input closure.

### Schema source, projections and consumers

- Added `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📥️source/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📽️projection/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/📋️plan/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏃️execution/🟦️.ts`.
- Added `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🔣️.json`; removed `🧰️framework/🔨️modules/🧬️schema/🔣️entity-kinds.json`.
- Added generated `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🦀️.rs`; removed generated `🧰️framework/🔨️modules/🧬️schema/🤖️generated.rs`.
- Updated generated `🧰️framework/🔨️modules/🧬️schema/🤖️generated/🏷️entity-kinds/🟦️.ts` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🏷️entity-kinds/🐹️.go` provenance to the neutral source path.
- Removed the intermediate monolithic extraction leaf `🧰️framework/🔨️modules/🧬️schema/🏷️entity-kinds/🏭️generator/🟦️.ts` after splitting its roles.
- Updated `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📜️script.ts` to the 11-line execution-owner router.
- Updated `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust/📋️project.json` with exact cached source/data/runtime inputs.
- Updated `🧰️framework/🔨️modules/🧬️schema/⚛️component/🦀️.rs` code and `include_str!` selectors.
- Updated `🧰️framework/🔨️modules/🧬️schema/🔣️.json`, `🧰️framework/🔨️modules/🧬️schema/🟦️.ts`, `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🏷️entity-kinds/🟦️.ts` and `🧰️framework/🔨️modules/🧬️schema/🧪️tests/🔬️component-unit/🦀️.rs` source/projection references.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/⌨️cli/🧪️tests/🔬️component/🐹️.go` and `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🧩️vscode/README.md` source-as-data/document selectors.

### Shared generator authority and exact contextual consumers

- Updated only the graph/schema generator-contract rows in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` with the semantic owners, exact external inputs, current graph schema path and generated Rust output path.
- Updated `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧫️fixtures/🏭️generated-source-topology/🔣️.json`, its schema `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏭️generated-source-topology/🔣️.json`, and verifier `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🏭️generated-source-topology/🟦️.ts` from 29 to 30 exact moves.
- Updated the graph source-as-data AST consumer in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔏️path-emoji-statutes/🟦️.ts`.
- Updated only stale bridge/projection links in `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs`, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️theme/🦀️.rs`, and `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🌉️icon-name-value/🦀️.rs`.

No launch command changed: all existing graph/schema generate, preview and check targets retain their command identities, and the root generation aggregate still depends on both producer targets.

## First reds and corrections

1. The first hostile graph admission control failed because a linked directory was followed and the same manifest appeared through both real and linked paths. The final walker uses component-wise `lstatSync`, rejects a linked root, linked intermediate ancestor, linked directory and linked manifest leaf, distinguishes `ENOENT` from inspection errors, and wraps directory/file read failures with the admitted relative path.
2. The first generated-topology run rejected the new generated Rust case because the portable schema fixed both `minItems` and `maxItems` at 29. The fixture/schema/verifier now agree on exactly 30 cases.
3. The first semantic split exposed one excess relative segment in the new graph discovery imports and one excess parent in the schema output-plan root. Both were corrected before producer verification.

## Portable and producer evidence

- Graph direct suite after the final split: **7 passed, 0 failed, 98 assertions**. It covers the Ajv output catalog oracle; duplicate/unsafe/mismatched identities; exact nested output membership; output symlink refusal; linked input root/ancestor/directory/leaf refusal; a real mode-`000` directory control on hosts that enforce that mode; injected file-read failure; malformed JSON, non-graph document and catalog mismatch with no output; actual generated registry loading; and exact Nx input closure.
- Terra independently reproduced current linked-area and mode-`000` failures under uid 501. Both errors named the exact boundary and both render calls left output absent.
- Graph preview: contract `graph-catalog`, **33 nodes**, **0 stale removals**. Direct check: **9 manifests fresh**. Final ordinary generate wrote the same exact 9-manifest plan.
- Registered isolated `bun nx run @semio-tech/framework-graph:check-generated --skip-nx-cache`: **exit 0**, **7/98**, **9 fresh**, Nx run duration **25.1s**, cache skipped. `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false`, and ticket-private workspace/cache/tmp directories were used. Bootstrap/project-graph latency preceded the reported target duration.
- Schema entity suite after the final split: **7 passed, 0 failed, 60 assertions** with Ajv and the owned parser. It retains **58** ordered kinds, **56** first-wins emoji entries and exactly **2** shadows (`todo`, `interaction-started`).
- Schema preview: contract `schema-entity-catalog`, exactly **3 nodes**, **0 stale removals**. Final ordinary generate and check preserved source SHA `f237cdd640bd1ac2b4546726f78f33c90aaf0c2ed11e1f6115294c8e9c3be242` across TypeScript, Rust and Go.
- Registered isolated `bun nx run @semio-tech/framework-schema:check --skip-nx-cache`: **exit 0**, 58-kind catalog fresh, Nx run duration **3.4s**, cache skipped, with the same private Nx environment.
- Installed TypeScript parsed all **11** new owners/routers with zero syntax diagnostics and resolved the generated TypeScript `../../🟦️` import to the real schema type owner.
- Generated-source topology after the final taxonomy update: **2 passed, 0 failed, 224 assertions**, exactly 30 generated moves.
- Focused graph semantic-filename AST control: **1 passed, 0 failed, 12 assertions**.
- All 12 introduced/moved anonymous source/data leaves returned no implementation-basename finding. The retired graph bridge/generator and schema source/generator/Rust-output paths are absent.

## Native evidence and limits

- Root's independent current Go consumer ran the selected `TestAllEntityEmojisProjectsTheFrameworkCatalog` through the actual repo-client package route: **exit 0**, selected root package **0.483s**, 9 fixture glob vectors. It read the moved 58-row JSON and asserted ordered IDs/emojis, normalized `AllEntityEmojis`, and FIRST-WINS. Root retained `📓️coordinator-entity-catalog-native-go-control-2026-09-13.md`.
- The authored Rust bridge law enumerates all **21 generated enum families**. For each family it compares generated `ALL/as_str` order with IDs from the parsed structured manifest, proves exact `ToValue` wire strings, `FromValue` round trips and one-to-one membership, and rejects an unknown string plus a non-string value.
- Two initial selected attempts against the shared artifact coordinate reached only `Blocking waiting for file lock on artifact directory`. They were bounded and cancelled after approximately **2m24s** and **3m03s**; neither is counted as native evidence.
- The final selected command used ticket-private `CARGO_TARGET_DIR=…/sol-generator-renderer-script-extraction/cargo-target` and `CARGO_BUILD_JOBS=2`: `cargo test -p semio-framework-graph generated_manifest_enum_value_mappings_are_exact_and_total -- --nocapture`. It completed **exit 0**: test profile **1m38s**, exact law **1 passed, 0 failed, 184 filtered out**, law runtime **0.01s**. Cargo metadata resolved the private target coordinate; the repository compiler cache wrapper reported the final content-addressed test binary under `.🧬semio/🦑️repo/⚡️cache/cargo/build`. It emitted one unrelated existing `AtomicU64::fetch_update` deprecation warning from the trace crate.
- A broad direct kind-only suite passed its first six cases then its semantic-generator case reached that test's explicit 30-second deadline. This is timeout evidence, not an assertion failure or a graph/schema acceptance result; root owns the separately recorded broader wrapper issue.
- A full path-statute suite was not accepted as a lane result because concurrent mutation fixture moves made seven unrelated cases red. Its graph AST case was rerun alone and passed as recorded above.

## Scratch and process ownership

All own Nx and Cargo processes are stopped. Ticket scratch `🗑️generated/sol-generator-renderer-script-extraction` was removed after retaining the exact results in this report. No live output outside the ordinary tracked generator destinations was published, and no Git, lifecycle or `AGENTS.md` action was taken.

## UI axes and actor typegen remainder

### Result and semantic owners

The UI generator now reads a neutral source at `🖱️ui/🎚️axes/🔣️.json`; its source admission, Rust/TypeScript projections, exact two-output plan, publication and command composition live under distinct semantic concerns with anonymous TypeScript leaves. The package `📜️script.ts` retains only Bun/Nx routing plus native UI test/wasm tasks. The ordered two-locale (`en`, `de`) and two-terminology (`native`, `reuse`) authority remains unchanged, and both generated outputs were regenerated through the ordinary route with the new source provenance.

Actor typegen now separates its exact output/protocol plan, publish-before-prune behavior and bounded native execution. The native exporter writes into staging first; publication writes the verified mirror before deleting exact siblings. Preview uses an isolated Cargo target, accepts the shared cancellation-file protocol, checks cancellation before and during the child, and applies a 600,000ms default deadline when no narrower build budget is supplied. Output ancestry is inspected without following links from the admitted repository boundary. The actor package route retains only task registration, native package tests and wasm orchestration.

### Exact UI file map

- Added `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📥️source/🟦️.ts` for strict neutral JSON validation and reading.
- Added `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📽️projection/🟦️.ts` for Rust and TypeScript rendering.
- Added `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📋️plan/🟦️.ts` for the exact two-file plan and preview protocol.
- Added `🧰️framework/🔨️modules/🖱️ui/🎚️axes/📤️publication/🟦️.ts` for freshness and publication.
- Added `🧰️framework/🔨️modules/🖱️ui/🎚️axes/🏃️execution/🟦️.ts` for preview/generate/check command composition.
- Added `🧰️framework/🔨️modules/🖱️ui/🎚️axes/🔣️.json`; removed `🧰️framework/🔨️modules/🖱️ui/🔣️ui-axes.json`.
- Added `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎚️axes/🟦️.ts`.
- Updated `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📜️script.ts` to the semantic-owner router and retained native task composition.
- Updated `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/📋️project.json` with exact source/projection/plan/publication/execution/test, native source, TypeScript consumers and external command/cache-writer inputs.
- Regenerated `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🤖️generated/🦀️.rs` and `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts` from the relocated source.
- Updated source-authority references in `🧰️framework/🔨️modules/🛂️manifest/🟦️.ts`, `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🌐️locale-terminology/🧾️value/🦀️.rs`, and `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`.

### Exact actor file map

- Added `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/📋️plan/🟦️.ts` for the generated coordinate, canonical protocol nodes and exact stale sibling inventory.
- Added `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/📤️publication/🟦️.ts` for no-follow ancestry admission and publish-before-prune.
- Added `🧰️framework/🔨️modules/🎭️actor/🧬️typegen/🏃️execution/🟦️.ts` for isolated native export, deadline/cancellation enforcement and command composition.
- Added `🧰️framework/🔨️modules/🎭️actor/🧪️tests/🧬️typegen/🟦️.ts`.
- Updated `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📜️script.ts` to the semantic-owner router and retained package test/wasm tasks.
- Updated `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📋️project.json` with exact semantic owners, portable test, native source, generated consumers and external command/cache-writer inputs.
- The ordinary registered route rewrote `🧰️framework/🔨️modules/🎭️actor/🤖️generated/🎭️actor/🟦️.ts` byte-identically after the private preview proved its content.

### Shared authority and retained native control

- Updated only the `actor-typegen` and `ui-axes` rows in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Executable `ownerPath` values remain the real Nx/Cargo package projects; sorted `inputPatterns` name every semantic owner, test and external runtime source. Actor declares the 600,000ms/16MiB preview limits.
- Existing `.vscode/launch.json` entries `📦️preview🤖️actor-typegen` and `📦️preview🤖️ui-axes` still select the same registered targets, so no launch identity changed.
- Added retained ticket-native input `📋️ui-axes-native-control/Cargo.toml`, its locked `Cargo.lock`, and anonymous `🦀️.rs`. It includes the actual generated Rust leaf directly and reads the actual neutral JSON; it is an oracle harness, not product/runtime code.

### First reds and corrections

1. The authored portable tests first ran before the semantic owners existed and failed at both new owner imports: **0 passed, 2 failed, 2 module-resolution errors**. This is the extraction first red.
2. The first ownership extension exposed two precise test defects: a UI local input list was not bound for its taxonomy assertion, and inspecting ancestry from filesystem root rejected macOS `/var` because it is a system link. The list is now shared within the case; actor publication admits a caller-owned boundary and inspects only descendants from that boundary, so system topology is outside the decision while links inside the owned repository/output route still reject.
3. The first registered actor invocation used the shared Cargo coordinate and reached only `Blocking waiting for file lock on artifact directory`. It was cancelled after about 48 seconds, and the orphaned owned Nx PID/tree was terminated explicitly. The accepted retry used ticket-private Cargo/Nx coordinates.

### Portable, producer and registered evidence

- Final focused direct command over both suites: **9 passed, 0 failed, 58 assertions**. UI proves the strict 2×2 source, unique ids/variants, identical ordered Rust/TypeScript values, installed-TypeScript syntax parsing, exact two-node preview, publication/freshness, direct Rust/TypeScript consumers and Nx/taxonomy input closure. Actor proves exact protocol/stale inventory, publish-before-prune, linked-root refusal without external mutation, deadline/cancellation child termination, native exporter source binding, direct TypeScript consumers and Nx/taxonomy closure.
- Workspace-aware `loadTaxonomy()` completed with **zero problems** after the two generator input lists were deduplicated and JavaScript-lexically sorted.
- Direct UI `preview-generated` emitted contract `ui-axes`, exactly **2 file nodes**, **0 stale removals**; both decoded node bodies equal the committed Rust and TypeScript outputs.
- Registered isolated `bun nx run @semio-tech/ui-rs:check --skip-nx-cache`: **exit 0**, **4/36**, two locales/two terminologies fresh, Nx **935ms**, cache skipped. It used `NX_DAEMON=false`, `NX_ISOLATE_PLUGINS=false` and ticket-private Nx/cache/tmp roots.
- Direct actor `preview-generated` ran the real native exporter: Rust `component::tests::exports_typescript_bindings` **1 passed, 0 failed, 126 filtered**, build **14.31s**, law **0.01s**. The emitted protocol has exactly **2 nodes**, **0 stale removals**, one **7,934-byte** file, and its decoded body equals the committed mirror.
- Registered isolated `bun nx run @semio-tech/framework-actor-rs:typegen --skip-nx-cache` with ticket-private Cargo/Nx/cache/tmp coordinates: **exit 0**; native exporter **1 passed, 0 failed, 126 filtered**, build **2.88s**, law **0.03s**; Nx **5.2s**, cache skipped. It published the same mirror and found no sibling to prune.

### Native evidence and limits

- The retained locked/offline UI harness used installed `serde 1.0.228` and `serde_json 1.0.149`, directly included the actual generated Rust leaf, and parsed the actual neutral JSON. `neutral_axes_and_native_projection_preserve_the_complete_two_by_two_matrix` passed **1/1** after a **29.13s** compile, law **0.01s**. It proves ordered ids/variants, both `ALL` arrays, both `COUNT` values, complete 2×2 membership, `parse`, unknown rejection and serde round trips. Dead-code warnings arise only because the isolated oracle includes a production module with APIs beyond this selected law.
- The full UI package selected `value_round_trip` Cargo attempt did not compile the UI crate: the current OS pack dependency failed first with five errors in `🎒️pack/🌱️value/🦀️.rs` (one `usize`/`u64` mismatch, obsolete `close_step` arity/result use, and missing `pop`/`is_empty` on `ManuallyDrop<RetainedPackSymbolTable>`). This is unrelated live dependency compiler evidence, not a UI-axis extraction failure or native UI runtime pass. It was not retried.
- The schema native delta requested after the graph/schema report used the selected command `cargo test -p semio-framework-schema entity_kind_ -- --nocapture` with ticket-private Cargo metadata and two jobs. It completed **exit 0**, build **13.95s**, laws **0.02s**, **3 passed, 0 failed, 25 filtered**. The exact laws were `entity_kind_catalog_data_validates_through_the_owned_validator_and_matches_the_rust_projection`, `entity_kind_emoji_index_is_first_wins_in_the_rust_projection`, and `entity_kind_catalog_rejects_every_declared_violation`.
- The recurring `AtomicU64::fetch_update` warning came from the separate trace crate. It is compiler provenance only and was not changed in this lane.

All UI/actor processes are stopped. Disposable `🗑️generated/sol-generator-renderer-script-extraction` outputs were removed after the evidence above was retained; the report and three authored locked native-control inputs remain.
