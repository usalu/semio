# STDIO Converter Dependencies

The two JSON↔TXT converters are mounted by the GeoJSON schema package, semio-s-artifact-stdio-json-schema. Their only TXT import is TxtSnapshot. The full TXT facade re-exports that same type from the UTF-8 schema package, and that schema package already publicly exports it. Both converter bodies keep the exact earlier implementations and use the actual schema owner for the import.

The GeoJSON schema now depends on semio-s-artifact-stdio-txt-schema. The other six JSON subpackages no longer directly depend on TXT. Their Rust sources have no TXT crate references. The existing test companion, owner records, snapshot type, public converter signatures and serialization behavior are still used.

The coordinated scope also moves three valid TXT snapshot imports in XML and DXF to the same schema, removes thirteen unused TXT edges in those families, and removes the unused Deflate facade from Draw's write owner. The editor agent owns those files. The lock reconciliation made those 24 exact edge changes across 85 live workspace locks. It removed the four now-unreferenced TXT facade, tests, mutation and inference records in 30 of those locks. Existing genuine TXT facade dependencies stayed in place where their owners still use them. Archived ticket workspaces were excluded. Each changed file was re-read immediately before replacement.

The prior actual Draw prototype WASI Cargo unit graph contained 923 reachable units. Projection of these exact changes into that graph yielded 919. The removed units were the full TXT facade, tests, inferences and mutations. The TXT schema remains reachable. That projection measures the dependency boundary change. A new link supplies current compiler measurements.

The neutral manifest and lock law was red against the old facade dependency. The third-party smol-toml parser then verified both JSON converter imports, all seven JSON package boundaries, all 24 coordinated edge rules in all 85 locks, the existing build-owner mapping, and the projected Draw graph. The managed repository Cargo metadata command for JSON passed locked and offline in the WASI profile. Its actual dependency record identifies the TXT schema as the selected owner. The managed Draw standalone WASI metadata also passed locked and offline. It resolves TXT schema and omits the four removed TXT packages, and its Draw write declaration has no Deflate edge. More representative workspace receipts are tracked below.

## Managed Cargo Check

Substitute the selected workspace manifest. This invokes metadata without starting a compiler build.

```sh
repo cargo --store=shared --configuration=prototype --parallelism=1 --lang=en -- \
  metadata --locked --offline --format-version=1 --filter-platform wasm32-wasip2 \
  --manifest-path "$MANIFEST"
```

- JSON workspace passed.
- Draw standalone workspace passed.
- Hub passed the independent manifest/lock graph law. Its managed command was stopped after five minutes in the repository workspace synchronization phase, before Cargo emitted metadata. Its process tree and capacity grant retired.
- XML, DXF and glTF managed invocations were deferred so the pinned component can use the available capacity. Their reconciled locks passed the independent graph law.

## Owned Files

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🌍️geojson/🧬️schema/🧪️tests/🔬️unit/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🔢️number/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/Cargo.toml`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/📤️export/🧵️serializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🔤️txt/🔖️utf-8/✳️any/🦀️.rs`

### Reconciled Workspace Locks

- `✏️s/🔌️plugins/✒️writer/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer/Cargo.lock`
- `✏️s/🔌️plugins/➗️mathematical/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/Cargo.lock`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/Cargo.lock`
- `✏️s/🔌️plugins/🌍️gis/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/Cargo.lock`
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/Cargo.lock`
- `✏️s/🔌️plugins/🌿️vcs/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🌿️vcs/🗿️artifacts/🌿️vcs/Cargo.lock`
- `✏️s/🔌️plugins/🎞️animate/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️presentation/Cargo.lock`
- `✏️s/🔌️plugins/🎥️shooting/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/Cargo.lock`
- `✏️s/🔌️plugins/🎪️demonstrator/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground/Cargo.lock`
- `✏️s/🔌️plugins/🎬️sequence/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/Cargo.lock`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🏙️bim/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/Cargo.lock`
- `✏️s/🔌️plugins/🏛️architect/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/Cargo.lock`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/Cargo.lock`
- `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/Cargo.lock`
- `✏️s/🔌️plugins/📋️forms/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/Cargo.lock`
- `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.lock`
- `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/Cargo.lock`
- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.lock`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/Cargo.lock`
- `✏️s/🔌️plugins/🔱️trinity/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/Cargo.lock`
- `✏️s/🔌️plugins/🕸️dag/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/Cargo.lock`
- `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/Cargo.lock`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/Cargo.lock`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/Cargo.lock`
- `✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/Cargo.lock`
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🧱️block/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d/Cargo.lock`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/Cargo.lock`
- `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/Cargo.lock`
- `✏️s/🔌️plugins/🪐️space/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🏭️bridge/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.lock`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.lock`
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/Cargo.lock`
- `✏️s/🧑‍💻dev/🧩️composition/Cargo.lock`
- `✏️s/🧑‍💻dev/🧹️fixture-sweep/Cargo.lock`
- `🌎️hub/Cargo.lock`

The durable ticket inputs are `json-txt-graph-law-26.json`, `json-txt-graph-law-26.ts` and this report.
