# Process3d B-Rep Pack import

Demonstrator wP6kzMf compiled new process geometry code against the declared semio-framework-pack package. The actual dependency in its Wasm unit graph is named pack, as specified by the package library target. The B-Rep converter and the Process3d editor smoke fixture imported the package name as a crate instead. Rust reported that the implemented encode and decode trait methods were inaccessible.

Both callers now use pack::snapshot, matching the sibling flow and scene converter. A typecheck for wasm32-wasip2 against the queue's actual pack and SemioBrepSnapshot compiler metadata resolved both the exact encode_pack_with and decode_pack calls. The complete Demonstrator retry is wSeD9dw. The previous run also reported mixed procedural preview and semantic UI graph dependencies. Their source and six lockfiles have been reconciled.

The same stale crate name appeared in five further test source files across Drawing I/O, EPW export, and Semio natural and text snapshots. Their manifests directly depend on the same pack library. All five now use its declared name.

The wSeD9dw warm retry revealed a separate error in the Process3d workpiece window. Its window declaration correctly uses the declared World3d kind, while the scene builder accepts the presentation contract's World3d kind. Calling the builder with the declared enum reproduced E0308 in an isolated Wasm32 typecheck. The corrected call in the real renderer and the matching typecheck use semio_plugin::app::SurfaceKind and pass type resolution. The procedural hover field reported alongside it is present in the current scene facade and passed its independent compiler check. The next Demonstrator build follows the fresh Procedural build in lane A.
