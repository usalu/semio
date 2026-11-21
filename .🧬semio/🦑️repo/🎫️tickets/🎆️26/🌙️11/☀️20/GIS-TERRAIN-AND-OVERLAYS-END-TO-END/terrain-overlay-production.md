# GIS terrain scene production

The GIS Terrain editor and viewer emit the same terrain scene payload. The demo uses the authored Liège scenery and live Terrarium tiles. A renderable imported map supplies its own positions, routes, regions, and geographic extent. A saved camera is restored within its terrain frame, and a changed geographic frame starts with an overview.

Map positions use `lon`, `lat`, and optional absolute `alt`. Routes use `points`. Regions use an exterior `ring` or `points` and optional `holes`. The projection retains identifiers and names, validates every coordinate in a path against the Web Mercator extent, and emits open polygon rings for implicit closure. The retained map still owns the complete intrinsic feature records.

The language-neutral fixture checks coordinate order, elevation zero, name fallback, a polygon hole, malformed geometry and the imported extent. The native descriptor probe linked the production Rust projection and scene serialization modules against the workspace's Semio value and JSON libraries. It matched the imported-map fixture and independently reconstructed Liège scene. Ajv validated the Rust output against the GIS scene and shared Terrain Overlays 2020-12 schemas, including rejection of latitudes outside the display extent. The preview input contains actual Rust-emitted terrain JSON and a camera computed independently from the authored scenery.

The registered editor smoke test decodes the scene that the actual GIS app publishes and checks all three feature collections on the terrain lane. The first native fallback compiled the production editor and exposed stale helper calls in its test build. The editor tests now use the current original-view admission, command argument, window-config restore, and first-party JSON decoding APIs.

The refreshed editor smoke test initially stopped in `semio-s-artifact-stdio-semio-graph-editor`. The authored tagged operation reader at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/💾️binary/🧬️mutations/🦀️.rs:18` requires `SemioGraphMutation: NativeReceive<(String, DslValue)>`. The compiler reported six errors from that missing bound while the graph root and eighteen mutation leaf receivers were under development. The native producer probe covers the terrain scene wire independently of the registered app host.

After the graph receiver implementations arrived, a six-job repository Cargo run compiled every GIS editor test and ran the registered terrain smoke. The test stopped at the terrain assertion because it decoded only the scene document spine. The test now uses the framework's lane-aware decoder to reassemble the published terrain carrier before asserting the overlay contents.

The retry with the corrected test stopped before GIS on a new `semio-os-dsl` dependency error. `RetainedRecordReceiving::close_work_demand` is missing at `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/📦️pack-value/🦀️.rs:175` and `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs:131`. That build exited 101 before it could run the corrected smoke.

The Terrain Window Config contract passed in Bun with three accepted payloads, four rejected payloads, and two addressed windows. During the Cargo run, repository ingress refreshed the artifact's already stale lockfile against existing workspace manifests.
