# Hub Headless Stdio Catalog Dependency Frontier

## Result

The selected `os-hub` execution-target acceptance compiles the full Stdio catalog even though its two laws never execute a Stdio codec. This is not an `--all-features` effect: the selected command asks only for `sqlite`. The cause is unconditional Hub package edges to Stdio full and GIS, plus startup that always constructs the native codec provider.

The linked catalog is genuine native execution authority today, not a descriptor re-export. The split must retain a separate full native-provider target; it must not make component/descriptor bytes look executable without matching native codecs.

No build was run for this audit.

## Current causal graph

```text
execution-target native law (--features sqlite)
  -> semio-hub direct Stdio full-artifact-catalog
  -> Stdio 26 native codec factories (includes glTF)

execution-target native law (--features sqlite)
  -> semio-hub direct GIS
  -> GIS direct Stdio full-artifact-catalog
  -> same complete Stdio closure

os-hub startup
  -> NativeCodecProviderSetV1::linked()
  -> TrustedCatalogLoader::load(... providers)
  -> instantiate ArtifactCodec receipts
  -> Store preflight and atomic codec registration
```

Hub has the unconditional direct edges at [Cargo.toml:39](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:39) and [Cargo.toml:40](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/Cargo.toml:40). Removing only the former is insufficient: GIS itself requests Stdio `full-artifact-catalog` at [GIS Cargo.toml:77](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/📦️packages/🦀️rust/Cargo.toml:77).

The slow selected group is [Hub script:2700](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2700): it runs `semio-hub --bin os-hub --features sqlite` and only the two execution-target laws. Broad `--all-features` groups remain separate amplification, but do not explain this immediate closure.

Stdio's full switch is a genuine high-cardinality compiler boundary: [Stdio Cargo.toml:31](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:31), the 26-factory table at [registry:982](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:982), and the glTF factory at [registry:954](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:954). Its new `home-io` feature is separate and cannot reduce a Hub still reaching either full edge.

## Execution-required, not re-export-only

`NativeCodecProviderSetV1` invokes GIS and Stdio receipt factories at [native-openable-provider:56](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:56) and [native-openable-provider:85](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs:85). Stdio's receipt contract constructs functional codecs, not just metadata ([registry:872](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📇️registry/🦀️.rs:872)). The verified catalog retains an `ArtifactCodec` ([trusted-catalog:238](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:238)), then Store preflights and atomically registers it ([trusted-catalog:555](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:555)). Production startup always links providers first ([os-hub:6595](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6595)).

GIS is another functional edge: Hub derives its verified Map binding from the catalog at [os-hub:6630](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6630). Any direct Hub feature split must therefore cover provider, GIS binding, and native inference—not merely make Stdio optional.

The two targeted route laws instead inject a synthetic `DocumentOpenCatalogAuthorityV1` with bounded component/descriptor bytes ([os-hub:7910](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:7910), installed at [os-hub:8241](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:8241)). They test receipt, authorization, revalidation, and body policy—not codec execution. The full provider law already exists at [os-hub:6979](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/🚀️bin.rs:6979) and is grouped with Stdio provider acceptance at [Hub script:3347](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:3347).

## Clean boundary and transition

Promote the existing private `NativeCodecProviderSourceV1` port ([trusted-catalog:383](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:383)) into Hub artifact-authority core, and change `TrustedCatalogLoader::load` from concrete `NativeCodecProviderSetV1` ([trusted-catalog:399](/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs:399)) to that domain-neutral provider port. `NativeCodecBinding` remains core; the port imports no product plugin.

Then extract `NativeCodecProviderSetV1`, the Stdio/GIS receipt adapters, and native GIS inference/binding glue to a host-only package such as `semio-hub-native-artifact-runtime`. That package becomes the only direct owner of Stdio full and GIS. The production launcher explicitly supplies it; headless Hub supplies no provider and fails closed when a trusted-catalog bundle/profile is configured. With neither setting, it retains the current unconfigured-authority state. Do not add a byte-only loader: that would bypass the existing native-codec registration invariant.

The smallest immediate transition is an optional Hub feature:

```toml
[features]
native-artifact-execution = [
  "dep:semio-s-plugin-stdio",
  "semio-s-plugin-stdio/full-artifact-catalog",
  "dep:semio-s-plugin-gis",
]
```

Make both direct dependencies optional and gate the provider construction, full-provider tests, GIS binding, and native inference under it. Headless uses `--no-default-features --features sqlite`. This unblocks the selected law, but is not the final boundary: `cargo --all-features -p semio-hub` would still enable it. The host-only package is what makes Hub's own all-feature/headless checks independent of the frontend artifact catalog.

## Required executable gates

1. Change [Hub script:2700](/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📜️script.ts:2700) to `--no-default-features --features sqlite`, retaining both exact execution-target laws. Add a source fixture forbidding direct Stdio/GIS Hub dependencies after extraction, or requiring them to be optional and all uses feature-gated during transition.
2. Keep `native_openable_stdio_provider_is_the_only_atomic_readiness_transition` plus the Stdio receipt laws in a distinct target that deliberately selects the adapter / `native-artifact-execution`. It must retain the 26 Stdio bindings and GIS closure; `home-io` is not a valid substitute.
3. Update Stdio's consumer-topology source check at [Stdio script:692](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📜️script.ts:692), which currently assumes every non-Space consumer is full. Make its allowed topology exact: GIS and the native Hub adapter are full consumers; headless Hub is not a Stdio consumer.

## Decision

Full Stdio/glTF is required by the current configured native-catalog execution boundary, but not by current execution-target byte-route acceptance. Preserve the former in an explicit native artifact runtime target; remove it from headless Hub rather than weakening catalog trust semantics.
