# R19 Plugin App Fleet B

## Scope

This packet owns the concrete runtime app boundary for VCS, playbook, the playbook procedural extension, remodel, layout, trinity, block, forms, note, sourcing, and GIS. FEM, imperative, opening configuration/process transport, and ticket metadata are excluded.

## Source changes

- Added 11 closed `PluginApp` enums, one per export: `VcsApps`, `PlaybookApps`, `ProceduralModuleApps`, `RemodelApps`, `LayoutApps`, `TrinityApps`, `BlockApps`, `FormsApps`, `NoteApps`, `SourcingApps`, and `GisApps`. Every plugin builder/result is now `Plugin<ConcreteApps>`.
- Passed the concrete app type to every `plugin_exports!` invocation and re-exported each enum from package glue so nested artifact declarations name the same closed runtime.
- Added direct `semio-framework-dispatch-macros` dependencies to each owning Rust package manifest.
- Made the pure plugin assembly entry points synchronous; the playbook procedural extension's separate extension bundle remains asynchronous.
- Typed the existing schema-first VCS, playbook, trinity, block, forms, note, and sourcing declaration graphs as `ArtifactDeclaration<Apps>` → `StandardDeclaration<Apps>` → `SubsetDeclaration<Apps>`, including typed editor/viewer factories. Their deterministic declaration helpers are synchronous.
- Removed the decorative async boundary from pure old-channel artifact definition/declaration helpers still used by remodel, layout, and GIS while retaining their concrete typed editor/viewer runtime registrations.
- Made pure descriptor hash normalization synchronous in the shared plugin runtime; genuine plugin manifest/description suspension remains explicitly resolved at the async descriptor test boundary.

## Structural inventory

- Typed `plugin_exports!(bundle, Apps)` invocations: 11.
- Closed `PluginApp` enums: 11, containing 29 concrete variants.
- Direct `semio-framework-dispatch-macros` dependencies: 11.
- Untyped one-argument `plugin_exports!` invocations in Fleet B: 0.
- `NoPluginApp` uses in Fleet B: 0.
- Typed schema-first declaration occurrences: 10 `ArtifactDeclaration`, 10 `StandardDeclaration`, and 10 `SubsetDeclaration` occurrences. These cover VCS, playbook, two trinity artifacts, three block artifacts, forms, note, and sourcing.
- Remodel, layout, and GIS retain their existing old declaration channel; their exported runtime storage and editor/viewer variants are nevertheless closed and concrete. No erased adapter or fallback was introduced.

## Validation

- Structural census: pass at the counts above.
- `cargo fmt --all -- --check`: Fleet B sources are not reported. The workspace check is currently nonzero solely because `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🤖️generated.rs` has one trailing blank-line diff; that root-owned UI typegen file is explicitly excluded from this packet.
- Focused native boundary, `cargo check -p semio-s-plugin-vcs`: reaches the VCS crate and fails on 180 rustc errors / 169 rendered primary diagnostics. Primary family counts are E0308=92, E0053=63, E0277=8, E0599=4, E0271=2. This is down from 188 before the pure declaration-helper closure. The remaining diagnostics are the product's broad sync-trait/UI/IO migration wall, not typed export or declaration-generic errors.
- Focused wasm boundary, `cargo check -p semio-s-plugin-vcs --target wasm32-wasip2`: reaches the VCS crate and fails on 96 rustc errors. It exposes four shared component-export hygiene/path diagnostics at the typed `plugin_exports!` invocation plus the product's sync-trait/UI migration wall. No wasm pass is claimed.
- The prior parallel native snapshot reached seven Fleet B products before stopping: VCS 188, sourcing 248, GIS 570, layout 698, note 773, block 1,599, and remodel 3,406 rustc errors. Those snapshot counts predate the final pure-helper repairs and are inventory only, not current pass claims. Playbook, procedural, forms, and trinity did not all reach terminal summaries in that parallel run; the focused trinity boundary separately recorded 1,245 errors before the final helper repairs.

## Evidence

- `📝️r19-fleet-b-native-1.txt`: initial boundary; untyped trinity/sourcing exports were the first errors.
- `📝️r19-fleet-b-native-2.txt`: superseded cold target run, intentionally interrupted after switching to the warm isolated target.
- `📝️r19-fleet-b-native-3.txt`: shared manifest stale-await blocker, subsequently repaired by its owner.
- `📝️r19-fleet-b-native-4.txt`: parallel product reachability snapshot and terminal counts.
- `📝️r19-fleet-b-trinity-warm-1.txt`: focused trinity compiler boundary before the final helper repair.
- `📝️r19-fleet-b-vcs-native-2.txt`: authoritative focused native VCS boundary after helper repair.
- `📝️r19-fleet-b-vcs-wasm-1.txt`: authoritative focused wasm VCS boundary after helper repair.
- `📝️r19-fleet-b-fmt-check.txt`: workspace formatting boundary identifying only the excluded UI typegen diff.
