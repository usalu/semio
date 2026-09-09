# Puzzle Window and Operation Ownership Completion

## Result

Puzzle 2D, Puzzle 3D, and Puzzle 5D now use the framework's concrete window-instance identity as the publication key. Shared generator preferences remain in app configuration; restorable viewport and display values publish through WindowConfig; drafts, popup state, and candidate cursors publish through WindowTransient; fill progress, checkpoint, cancellation identity, and terminal result remain retained operation state.

| Artifact | App configuration | Exact window configuration | Exact window transient | Operation |
| --- | --- | --- | --- | --- |
| Puzzle 2D | node-kind and handle-kind weights | camera, LOD, fill count, grid, suggestion offset | engagement draft and brush candidate state | frozen fill input, identity, checkpoint, progress, cancellation, fault/result |
| Puzzle 3D | fill count, overlap budget, object-kind and vortex-kind weights | camera, sun, LOD, grid, selectable kinds, proximity/chunk/voxel controls, transform/vortex display | suggestion popup, engagement draft, brush candidate cursor | frozen fill input, identity, checkpoint, progress, cancellation, fault/result |
| Puzzle 5D | overlap budget, object-kind and vortex-kind weights | separate Board and World owners for camera and kind-specific display controls | separate Board and World owners for engagement draft and brush candidate cursor | retained command input/identity/progress/result; no fill checkpoint is copied into configuration |

The active utility and live window roster continue to come from framework view/lifecycle state. Puzzle 2D and Puzzle 5D engagement actions now trust the dispatch context's concrete instance and registered window kind. Their SetActiveUtility effects address that exact instance instead of a kind id or the whole fixed window-kind roster.

## Registered-app runtime laws

The native editor tests build real registry-backed VcsArtifactApp<EditorApp<_>> instances, bind receiver instance 1, dispatch through the typed operation route, advance bounded maintenance and publication turns, acknowledge terminal result pages, and drive close until the framework reports terminal-empty ownership.

The framework `#[async_test]` macro pins its complete generated future on the libtest thread. The two new Puzzle 2D and Puzzle 5D laws each kept two large registered apps alive across awaits, so their original unboxed fixture locals enlarged that generated future enough to overflow before its body began. The laws now keep those fixtures in `Box` storage and are being validated at the plugin's existing 8 MiB test boundary. This is test-future storage only; no native release or app-runtime stack setting was added.

- Puzzle 2D: two Overview instances publish different cameras, render independently, leave document/app config unchanged, persist and reload two packs, isolate engagement drafts, clear only the addressed draft on abort, target the exact instance with the utility effect, reset transient state in a reopened app, and close both apps.
- Puzzle 3D: two Main instances publish different options, render independently, leave document/app config unchanged, persist and reload two packs, isolate the suggestion popup, clear only the addressed popup, reset transient state in a reopened app, and close both apps.
- Puzzle 5D: two Board instances publish different cameras, render independently, leave document/app config unchanged, persist and reload two packs, isolate engagement drafts, clear only the addressed draft on abort, target the exact instance with the utility effect, reset transient state in a reopened app, and close both apps.
- Puzzle 3D fill cancellation rejects stale job, operation, and generation identities and cancels only the exact live operation before driving its terminal handle to empty.

## Source ledger

### Cross-artifact contracts

- ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts
- ✏️s/🔌️plugins/🧩️puzzle/🔏️publication-authority/🔣️.json
- ✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts
- 🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts

The TypeScript audit validates seven neutral WindowConfig/WindowTransient schema records with Ajv and a separate exact-key oracle, rejects hostile extra app-config fields, and validates every retained publication contract. The Rust plugin test router supplies an 8 MiB minimum test-thread stack for the generated descriptor-freshness test.

The OS ownership gate's remaining host-opening-context fixture URL now follows the fixture's canonical `🧫️fixtures/🪟️host-opening-context` owner, allowing the Nx project graph to resolve after the concurrent taxonomy move.

### Puzzle 2D

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{⌨️engagement-input,🛑️engagement-abort,📨️engagement-submit}/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️{testkit,unit}/🦀️.rs

### Puzzle 3D

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⏳️precompute
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️{testkit,unit}/🦀️.rs

### Puzzle 5D

- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/{⌨️engagement-input,🛑️engagement-abort,📨️engagement-submit}/🦀️.rs
- ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️{testkit,unit}/🦀️.rs
- Puzzle 5D artifact/schema surfaces carrying the composed kindCatalogsExtra field in Rust, JSON Schema, TypeScript, Protocol Buffers, and GraphQL.

## Validation evidence

| Command | Result | Evidence |
| --- | --- | --- |
| NX_DAEMON=false CARGO_INCREMENTAL=0 bun nx run '@semio-tech/puzzle-js:publication-authority-audit' --output-style=static | GREEN; 3 owners, 7 neutral schemas, Ajv + independent oracle | 🗑️generated/puzzle-publication-authority-audit.log |
| NX_DAEMON=false CARGO_INCREMENTAL=0 bun nx run '@semio-tech/puzzle-5d-rs:check' --output-style=static -- --features component-app-assembly | GREEN; full native dependency and Puzzle 3D/Puzzle 5D library compile | 🗑️generated/p5-window-ownership-check.log |
| Puzzle 5D focused registered-app tests, first attempt | RED before execution; 15 stale compile-time fixture includes were concurrently corrected to existing descriptive fixture paths | 🗑️generated/p5-window-ownership-native-red-1.log |
| Puzzle 5D focused registered-app tests, warm retry | RED after successful compile at test entry; the unboxed two-app async future overflowed the libtest stack before executing its body | 🗑️generated/p5-window-ownership-native-green-2.log |
| Puzzle 5D focused registered-app tests, 128 MiB diagnostic retry | RED before test execution; concurrent DAG migration removed a compile-time demo DSL include | 🗑️generated/p5-window-ownership-native-green-3.log |
| Puzzle 5D boxed-fixture retry at the existing 8 MiB boundary | RED before test execution on the same unrelated missing DAG demo DSL include | 🗑️generated/p5-window-ownership-native-8m-red-4.log |
| Final cross-language authority rerun | RED before target entry; concurrent browser-context migration left a missing `🔣️.json`, and the graph references a currently absent `npm:@asamuzakjp/css-color` project | 🗑️generated/puzzle-publication-authority-audit.log |
| Final cross-language authority rerun after the canonical fixture URL correction | GREEN; 3 owners, 7 neutral schemas, Ajv + independent oracle | 🗑️generated/puzzle-publication-authority-audit-2.log |

All Cargo commands use CARGO_INCREMENTAL=0 and the ticket-owned 🗑️generated/cargo-puzzle-ownership target. No native result is treated as green while its owning process is still live.

## Remaining validation

- Restore the concurrently moving DAG demo include and Nx graph inputs, then obtain terminal native runtime results for the focused Puzzle 5D, Puzzle 2D, and Puzzle 3D registered-app ownership laws.
- Obtain terminal native cancellation result for the Puzzle 3D exact fill identity law.
- Re-run the generated plugin descriptor-freshness test with the 8 MiB test-thread stack.

The ticket and parent goal remain open.
