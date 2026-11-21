# GIS Guest Descriptor Probe

The already transpiled prototype GIS guest has an isolated production-shaped descriptor probe assembly in `🗑️generated/gis-descriptor-proof`. The preparation script at `prepare-gis-descriptor.ts` copies the independent JavaScript and core module into ticket scratch. It calls the production async-result and component-asset rewrites and writes the production `hostShimSource()` under its canonical filename. It copies the actual selected prototype preview2 vendor directory from `.cache/b/p/m/browser/prototype/modules` into scratch and calls production `rewritePreview2ShimImports` against that copy. The same script saves the literal production `PLUGIN_DESCRIPTOR_PROBE_SOURCE` in `probe.eval`.

The next invocation targets the scratch module with the same Node JSPI options as the descriptor materializer and a two-worker limit. The source provides the complete WIT actor export check and invokes the guest's own async `describe()`. The independent build's bytes and production artifacts remain untouched.

The Node run is deferred while the warm GIS build competes for memory. The host reported warning-level pressure and 670–1,453 MiB spare above the capacity floor at the checked points. Assembly success establishes that the canonical rewrites and vendor selection worked. The guest descriptor result remains unverified.

```zsh
scratch="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/🗑️generated/gis-descriptor-proof"
UV_THREADPOOL_SIZE=2 repo capacity run --memory=1536 --cpu=2 -- repo tools exec node --light -- \
  --v8-pool-size=2 --experimental-wasm-jspi --input-type=module --eval "$(<"$scratch/probe.eval")" \
  "$scratch/module/semio_hub_gis_component.js" > "$scratch/semantic.b64" 2> "$scratch/probe.stderr"
```

The capacity governor later admitted one Node v24.15.0 invocation with a 1,536 MiB reservation and one CPU requested. Import and the required actor export checks completed. The guest's own `describe()` aborted before returning a descriptor, with `app s.gis.gisterrain@1/*#editor: fault notice gis-terrain-brush-settings-invalid: syntax` followed by a Wasm `unreachable` trap. The live declaration still carried that code when checked. The canonical validator requires at least three dot-separated kebab segments. The result is a concrete guest startup failure in the retained prototype artifact.

Both rejected notice codes now follow the terrain editor's existing dot-separated convention. All 20 published names passed checks against the host TypeScript grammar and an independent Ajv schema. A rebuilt Wasm descriptor is still required. [Terrain descriptor fix](./📓️terrain-descriptor-fault-notices.md)

The two generated diagnostic output trees were removed after the managed run published the corrected model. The preparation script remains in the ticket. The native descriptor panic log remains in ticket scratch while the active verification runs.
