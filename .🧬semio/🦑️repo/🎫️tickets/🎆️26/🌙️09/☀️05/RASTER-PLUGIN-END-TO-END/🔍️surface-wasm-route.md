# Surface WASM Route

The alternate `repo run @semio-tech/framework-surface-rs:wasm` stopped before its producer: the graph host service reported `stat /Volumes/semio-raster-local-9c4a/s/b/d/m/j: no such file or directory` and could not answer. This is the alternate dependency generation store, not a compiler or Surface WASM diagnostic. The log does not prove why that path is absent.

The active root playground has an already supported graph-free path: `serveRootPlaygroundV1` calls `ensureRendererEngines` after boot components, constructs the selected producer with its original issued invocation, and runs it. Surface `WasmScript` uses `buildAdmittedWasmWebV1` and verifies its produced package through the browser suite. A separate direct invocation of the nested script requires the original parent invocation; no independently rooted engine-only graph-free command was found.

`repo dependencies sync --all` is the documented dependency provisioning operation, but no evidence establishes it as a quick repair of this alternate store or a safe change during the active run. No mount, cache, build, or product file was changed for this audit. Letting the active root path reach its own producer preserves publication and avoids this optional graph route.
