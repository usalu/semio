# Typed PNG bridge and component build

Cargo's inverse WASM dependency tree traced `semio-s-artifact-stdio-semio` solely through Remodel I/O. The PNG bridge was calling the umbrella registry, which registers nineteen subsets before dispatching one image conversion. This was also the crate in which the earlier full WASM build received signal 9.

Both directions now use the existing typed `SemioImageFromPng` and `SemioImageToPng` implementations, matching Remodel's typed mesh export. This removes the umbrella dependency and the unused drawing and mesh import registrations, along with intermediate pack encoding and decoding. The locked WASM runtime tree resolves 803 distinct package names and still contains both image converters and the mesh exporter. The locked native I/O dependency tree also excludes the umbrella. Cargo pruned 198 package entries from Remodel's artifact lock compared with the index, 189 of them internal Stdio crates. No registry packages were added.

The fixture-sweep lock contains the same four targeted edge removals. A broader refresh attempted to resolve 571 pre-existing workspace changes and failed because `rand 0.8.6` was absent from the offline cache. The broad refresh was reverted, preserving the four Remodel changes and the staged changes already present in that lock.

FFmpeg independently decoded the existing two-by-two fixture to opaque red, green, blue, and white pixels in that order. The native I/O test verified those pixels and the typed bridge's round trip.

The PNG document importer also published the frame's asset handle without its durable image chunks. Reconstruction could find the frame but could not read its image. The importer now uses the same bounded durable asset constructor as media mutations and writes the content under the handle's child ID. A second law reads the original fixture bytes back through the production reconstruction chunk source.

The locked narrowed graph linked successfully. A same-target relink with symbols stripped let the owned interpreter execute the component and emit matching descriptor forms. After earlier runs were canceled to protect component disk headroom, the final warmed native PNG run compiled and passed both focused tests. The second test verified that the importer publishes the exact original PNG bytes under the frame content handle.

Remodel I/O now takes `OsMediaExportResult` from its existing owner `semio-os-host-workflow`. This preserves the PNG envelope and removes the OS host, Wasmtime, and native host sync stack from the resolved Remodel I/O, editor, and artifact dependency graphs. Locked offline Cargo metadata and the native inverse tree both passed. The artifact lock lost 289 obsolete package entries and added no package versions. Fixture sweep still has other OS host consumers and needed only the Remodel dependency edge changed.

The native graph still reaches the Stdio Binary mutation aggregate through LAS in Remodel I/O and through JPEG in the editor. The typed PNG leaf already uses `remodeling_png_asset` directly. The OS envelope helper's only in-tree caller is its Rust test.
