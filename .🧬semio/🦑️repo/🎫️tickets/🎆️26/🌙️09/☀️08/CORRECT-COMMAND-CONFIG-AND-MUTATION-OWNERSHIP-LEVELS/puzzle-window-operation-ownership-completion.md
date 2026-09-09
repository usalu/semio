# Puzzle Window and Operation Ownership Completion

## Result

Puzzle 2D, Puzzle 3D, and Puzzle 5D now use the framework's concrete window-instance identity as the publication key. Shared generator preferences remain in app configuration; restorable viewport and display values publish through WindowConfig; drafts, popup state, and candidate cursors publish through WindowTransient; fill progress, checkpoint, cancellation identity, and terminal result remain retained operation state.

| Artifact | App configuration | Exact window configuration | Exact window transient | Operation |
| --- | --- | --- | --- | --- |
| Puzzle 2D | node-kind and handle-kind weights | camera, LOD, fill count, grid, suggestion offset | engagement draft and brush candidate state | frozen fill input, identity, checkpoint, progress, cancellation, fault/result |
| Puzzle 3D | fill count, overlap budget, object-kind and vortex-kind weights | camera, sun, LOD, grid, selectable kinds, proximity/chunk/voxel controls, transform/vortex display | suggestion popup, engagement draft, brush candidate cursor | frozen fill input, identity, checkpoint, progress, cancellation, fault/result |
| Puzzle 5D | overlap budget, object-kind and vortex-kind weights | separate Board and World owners for camera and kind-specific display controls | separate Board and World owners for engagement draft and brush candidate cursor | retained command input/identity/progress/result; no fill checkpoint is copied into configuration |

The active utility and live window roster continue to come from framework view/lifecycle state. Puzzle 2D and Puzzle 5D engagement actions now trust the dispatch context's concrete instance and registered window kind. Their SetActiveUtility effects address that exact instance instead of a kind id or the whole fixed window-kind roster.

Each Puzzle transient owner uses the shared `ArtifactEphemeralTransferPreparationFactory` with owner-specific preflight and an infallible direct mutation-to-state move. State and mutation roots use `OwnedValueRetirementFactory`; the previous duplicate app-local lifecycle structs are removed. Puzzle 2D charges every nested `DslValue` allocation and key before queue extension against the exact Store admission ceiling, making both traversal and temporary queue space bounded. Tiny-grant regressions cover nested Puzzle 2D values, Puzzle 3D suggestion/input strings, and Puzzle 5D input strings through terminal-empty retirement.

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

Current Cargo commands use CARGO_INCREMENTAL=0 and the shared ticket-owned 🗑️generated/cargo-trinity target. Note check session 22619 and shell PID 16786 are no longer registered or alive. 🗑️generated/note-window-check-4.log ends on an Nx heartbeat after 1,690,891 ms without a terminal Cargo result, so it is recorded as an orphaned no-result run.

## Capacity admission correction

The direct transient transfers now admit retained heap capacity rather than logical payload length. Puzzle 3D charges all three owned Strings. Puzzle 5D charges its engagement String. Puzzle 2D charges both top-level Strings, the reserved capacity of the top-level candidate Vec, every nested String, every nested array/object Vec, and every object-key String. Checked multiplication covers vector element storage, checked addition covers the cumulative footprint, and each charge rejects once it exceeds the Store one-item maximum before the traversal queue extends.

Native unit regressions construct one-character Strings and empty Vecs whose capacity alone exceeds the Store maximum. They require begin to return the exact mutation allocation by checking value, pointer, and capacity, then retire that returned mutation to terminal emptiness with one-item/one-byte grants. Puzzle 2D covers both top-level Strings, the top-level Vec, nested String/array/object allocations, and object keys. Puzzle 3D covers engagement input plus both suggestion-menu identifiers. The existing accepted multi-kilobyte payload and tiny-grant retirement laws remain.

These source and test changes have not been assigned a native result. The previous Note check was orphaned without a terminal result, and the parent owns the shared Cargo queue for the complete Note and Puzzle runtime gates.

## Remaining validation

- Run the root-owned full Note native registered-app target, then obtain terminal native runtime results for the focused Puzzle 5D, Puzzle 2D, and Puzzle 3D registered-app ownership laws.
- Obtain terminal native cancellation result for the Puzzle 3D exact fill identity law.
- Re-run the generated plugin descriptor-freshness test with the 8 MiB test-thread stack.

The ticket and parent goal remain open.
