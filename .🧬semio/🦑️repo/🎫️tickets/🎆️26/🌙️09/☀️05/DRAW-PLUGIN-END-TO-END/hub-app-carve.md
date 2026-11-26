# Draw Hub Composition Memory

The current component build measured `semio-hub-draw` at 1,617 MiB against the 1,536 MiB library limit. Its actual WASI archive at `.cache/b/r/m/u/l/paefc3c/wasm32-wasip2/prototype/build/semio-hub-draw/02a87b49d4456b7e/out/libsemio_hub_draw-02a87b49d4456b7e.rlib` is 48,906,302 bytes. Its 18 relocatable Wasm objects contain 22,047 named functions and 11,007,245 bytes of function bodies. The largest object contains 2,871,378 bytes of function bodies.

| Symbol family | Functions | Function bytes |
| --- | ---: | ---: |
| Explicit `DrawingPlayApp` specialization | 3,695 | 2,455,336 |
| Explicit `DrawingViewer` specialization | 4,749 | 2,750,221 |
| Explicit builder and declaration markers, also reflected in the role groups where applicable | 41 | 21,425 |

The groups use symbol text and undercount indirect role specializations. They measure emitted code and guide the domain cut. Per-crate compiler peaks still require measurement.

The staged patch gives Draw the same mounted editor and viewer crate boundary as the existing Sourcing composition. Each short package owns the closed dispatch for exactly one concrete `VcsArtifactApp` type and exposes the unchanged payload to the composition. `DrawApps` delegates to those nominal roles and accepts both original typed constructors. Its plugin builder, capability request, activation, mutation rosters, close budget, component export entry, descriptor pair, and surface and guest law sources are unchanged.

The stage generator, unified patch and in-memory law are under the ticket's generated directory. The law used the actual artifact's constructor contract, independently parsed the projected TOML, lockfile and owner registry, validated the neutral role fixture with Ajv, and checked that only the intended nine paths change. The longest projected source or actual-lane object path was 192 UTF-16 units. The full changed Cargo graph path gate still needs its normal managed run.

The live source, hub workspace manifest, lock and owner registry were left untouched while the prior source snapshot finished. Re-run `stage-draw-app-carve.py` against the current shared tree before applying `draw-hub-app-carve-26.patch`. The member decoder carve has independent lock and registry entries. Apply both without replacing either maintainer's unrelated edits.

After the earlier component process has retired, resolve locked offline metadata and use one managed Cargo invocation to measure each new role and the composition, with no parallel Rust compiler. Start with the existing warmed prototype store. Scheduler telemetry should identify one actually compiled unit for each of the three names and the `wasm32-wasip2` target. A restored object or metadata-only unit supplies no peak measurement.

```sh
SEMIO_OFFLOAD=never SEMIO_COMPILER_PARALLELISM=1 \
  repo capacity run --memory=4608 -- repo cargo --store=shared \
  --configuration=prototype --parallelism=1 -- build --locked \
  --manifest-path "🌎️hub/Cargo.toml" --target wasm32-wasip2 --lib \
  -p semio-hub-draw-editor-app -p semio-hub-draw-viewer-app -p semio-hub-draw
```

Compare all three measured peaks with 1,536 MiB. Then run the regular Draw component publication and existing native and browser laws against that publication. The staged source law passed. No Rust build was started.
