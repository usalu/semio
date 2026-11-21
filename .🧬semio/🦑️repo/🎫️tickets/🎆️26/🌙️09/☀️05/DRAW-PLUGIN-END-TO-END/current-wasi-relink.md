# Current Draw WASI Relink

The previous `draw-linked-component-snapshot-rebuild.log` emitted two visible compiler failures. The captured Tool Run facade lacked `ToolRunTraceApplyCursor`, and its captured UI contract lacked the `StyleSpec::highlighted` field used by the kernel JSON projection. The current Tool Run Rust facade and trace-store both re-export the cursor. The current presentation contract declares the field with the same type and wire defaults as the kernel JSON projection. Those historical failures did not require edits to the live sources.

The fresh linker runs through the repository Cargo ingress against the Hub Draw component package with the shared prototype store, locked offline dependencies, and the WASI Preview 2 target:

```sh
repo cargo --store=shared --configuration=prototype --lang=en -- build --manifest-path "🌎️hub/🧩️compositions/🖍️draw/🧩️component/📦️packages/🦀️rust/Cargo.toml" -p semio-hub-draw-component --lib --target wasm32-wasip2 --locked --offline --color never --message-format short
```

The independent retention helper monitors the fresh build log. After Cargo reports a successful prototype build, it accepts a newly linked component with the WASI header, checks source stability across a bounded copy, and records the retained size and SHA-256. The output lives in the ticket's generated directory. The existing fresh descriptor verifier can then execute the retained component and validate the JSON and Pack representations against the Draw scenario oracle.

The fresh run compiled the previously failing `semio-plugin-tool-run` integration and reached Draw model, scene, mutations, editor and viewer. Its first compiler attempt stopped at `semio-framework-graph-manifest` with three missing `DslReceiving` derive errors. The live graph manifest already declares its derive crate and both the root and Hub lockfiles list it. The admitted source used the derive while its earlier package graph omitted the dependency. The Cargo ingress detected the changed shared generation and began a fresh snapshot automatically.

After recapturing the moved graph-manifest dependency, Cargo exposed a separate live failure in the Job return guard. The guard is a child of the return owner and must use that owner's Job context source alias for source-row reservation. The source-owned `reserve_original_header_guard_row` preserves the diagnostic `header_guard_row_unavailable` when the guard yields. The reproducing run exited 101 for the unresolved methods. The next WASI build compiled the Job return owner and exited 0 with a completed prototype link. A scoped `git diff --check` passed for the two Rust files.

The retention helper copied 173,320,074 bytes from the successful link, recorded the WebAssembly component header and SHA-256 `2ff4bebd149333639ce2dd34d4668fb063c0f7de8dddf7450bfe426ea1477b7f`, and exited 0. The retained Draw component is `🗑️generated/draw-current-linked.wasm`.

The repository's published macOS release descriptor emitter compiled and executed those retained bytes, then checked the `drawing.document` codec. The descriptor verifier accepted the strict JSON/Pack pair and independent Draw scenario oracle. The fresh descriptor directory is `🗑️generated/current-draw-descriptor` and its semantic SHA-256 is `3304d8163f0d966076755bd2f304b025a4c03c5cc4fabaa73bc38edc470c64b1`. At the time of that comparison, the checked-in descriptor's only semantic drift was in the editor's `interactionHover` arguments. The generated descriptor makes `targets` optional and adds optional `targetId` and `granularityId`. The parent task owns canonical source-pair publication and registry verification.

The native guest fixture runs against the retained component and its freshly emitted JSON under the separate focused Cargo test. Its entry is `wasmtime_runtime_instance_open_settles_against_the_draw_component` in the Hub Draw library.
