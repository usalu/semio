# Layout Component Descriptor Staging

The registered browser facet’s describe producer is `repo run workspace:components:prototype -- describe layout --profile prototype --facet browser`. The scoped Script entry and current guest evidence are in [Layout WASI Guest and Descriptor](wasi-guest-2026-11-20.md).

The standalone emitter consumes the linked `wasm32-wasip2` Layout component. Copy the linked guest from the build’s reported target path into this ticket’s generated directory and record its SHA-256. The target key can change between builds, and the managed cache can prune older files. The published native emitter is available under the prototype browser product’s `tools/descriptor-emitter` directory. The managed Cargo invocation below is the fallback when that tool has yet to be published.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 -- run \
  --manifest-path '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/Cargo.toml' \
  --bin semio-plugin-describe -- describe "$layout_guest_wasm" --out "$layout_descriptor_stage"

repo tools exec --light bun -- "$layout_ticket/linked-asset-descriptor-check.ts" \
  "$layout_descriptor_stage/🔣️.json" "$layout_descriptor_stage/🛂️.descriptor.semio"
```

The verifier reads both staged representations through the registry's semantic descriptor API. It requires the Layout package and plugin identities, the Demo registration, one Linked Asset registration, and the exact authored Linked Asset document. Publish the verified emitted pair together to `🌎️hub/🧩️compositions/📏️layout/🧩️component` and rerun the verifier on the destination. The currently checked-in pair differs at both application I/O labels and the app channel version. It lists only Demo.
