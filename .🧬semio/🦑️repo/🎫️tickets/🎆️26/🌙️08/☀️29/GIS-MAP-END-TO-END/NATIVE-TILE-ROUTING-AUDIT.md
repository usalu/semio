# Native GIS2D Tile Routing

## Launch and route selection

The GIS2D Cargo playground selects the GIS map editor. Playground discovery filters app-scoped asset rows and retains their `app` field in the copied JSON session. It selects `/osm` with the OpenStreetMap PNG template and `/vt` with the OpenFreeMap Planet TileJSON endpoint. The GIS3D `/dem` row is excluded. The asset dispatcher requires the provider's exact tile schema, so the native launcher now removes the already-used `app` selector before creating the server.

React and browser WGPU consume the generated TypeScript playground catalog. `emitAssetSpecTypeScript` already emits only delivery fields, so both browser configurations receive the tile schema accepted by the provider.

The native launcher starts an IPv4 server on `127.0.0.1` using an OS-allocated port, waits for the listening event, and passes the address to the executable as `SEMIO_ASSET_BASE_URL`. It retains the server until the child settles. The controlled HTTP integration exercised the path where the listener was initially pending.

## Native request transport

The interactive Winit boot loads staged component-model Wasm via `load_wasm_plugins`. The kernel pool installs `GuestRuntimes::Owned`. Its `wasi:cli/environment` response is an empty list, so the map's environment lookup leaves the scene templates `/osm/{z}/{x}/{y}.png` and `/vt/{z}/{x}/{y}.pbf` relative. The map's engine canvas passes substituted paths to the renderer as `WorldAssetRequestKind::MapTile`.

`RendererAssetFetchOwner::kind()` returns the request's `WorldAssetRequestKind` from either owner variant. The mailbox and asset-authority crates use that same type from their `infinite_canvas` dependency aliased to `infinite_world`. The native-only stream resolves relative map tile paths at the launcher’s validated IPv4 HTTP origin. The existing retry loop keeps the same request owner and resolved URL. Other request kinds and already-absolute URLs follow the applicable existing transport path. No environment values are added to the guest.

The plain HTTP mailbox pool uses `SocketHttpTransport`. Its parser extracts the numeric port and path before connecting through `std::net::TcpStream::connect((host, port))`. The literal IPv4 loopback target avoids hostname resolution and proxy configuration. This is a source-level check of the Linux and Windows paths.

## Fetch concurrency and timeouts

The map can queue several visible tile requests, but the native pump takes one owner after acquiring its `native_asset_fetching` flag. It publishes the handoff before releasing the flag and drains that handoff before the next admission. HTTP and HTTPS each have a separate pool capped at one outstanding request per actor. Native requests use actor 0. Package identity keys the byte budget. A normal queue of tiles does not contend for the HTTP credit, and increasing the pool limit alone would leave fetches serial. A slow request can delay all following tiles, including requests for the other layer in combined mode. The effect on visible map latency has not been measured. [Map tile reservation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🗺️tiled-map/🦀️.rs:63) · [Native pump](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🌐️native-asset-pumps/🦀️.rs:280) · [Pool configuration](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🏗️mailbox-construction-and-authorities/🦀️.rs:23) · [HTTP admission](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🪣️http-pool/🦀️.rs:238) · [Native actor and package](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🦀️.rs:112)

The 15-second renderer deadline can expire while the proxy is still waiting on an upstream tile under its 30-second deadline. The blocking socket worker can outlive the response-head future. On this path the outstanding credit stays with the physical worker and the native guard releases its slot without interrupting the socket. A subsequent tile can then receive `OutstandingCapReached` despite the serial pump. The existing HTTP service law checks refusal while an abandoned head worker is still live. The native renderer classifies only transport errors as retryable, and the final shared map rejection records a miss until the visible-range revision changes. [Renderer deadline](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🦀️.rs:103) · [Proxy deadline](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖼️assets/🗺️tile-proxy/🧬️schema/🔣️.json:73) · [Blocking I/O deadline](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧰️compute-pool/🦀️.rs:223) · [Physical credit retention](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🪣️http-pool/🦀️.rs:101) · [Native guard drop](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🦀️.rs:572) · [Abandoned head law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🌐️http/🦀️.rs:115) · [Final shared rejection](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🖼️asset-decode-pump-and-reference-images/🦀️.rs:411)

The socket cancellation handle already interrupts a bound TCP socket and latches an interrupt requested before binding. The socket transport exposes no I/O timeout knob. A targeted recovery can interrupt the failed response-head operation and observe that worker's exact terminal before starting another tile. A scheduler-side alternative would treat cap contention as deferred work, retire response credits, and return the open owner to the queue with bounded backoff. The existing retry flag cannot do that alone. It adds up to two in-place retries, after which shared map rejection still records a miss. These recovery options were assessed in source and were not implemented or tested during this check. [Socket cancellation](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🔌️http-socket/🦀️.rs:86) · [Retry delays](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🦀️.rs:66) · [Response credit reset](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/📥️renderer-asset-steps/🦀️.rs:50) · [Open owner handback](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/📡️asset-io/🦀️.rs:395)

## Verification

- The retained native launch integration reproduced the strict schema failure, then passed with the real asset dispatcher and tile provider. An independent TOML parser identified the exact authored GIS2D routes and upstreams. The extracted native runner launched both routes with controlled local upstreams, published the bound origin before the child callback, served one cold raster response and one cold vector response through TileJSON, and closed the server. The callback substituted for the native executable.
- The standalone Rust resolver law failed with a missing implementation and passed across all 18 fixture rows after the change. The independent `whatwg-url` package agreed with all fixture expectations. Pinned rustc parsed the changed native transport file. The full native renderer was not compiled or launched in this audit, and no interactive GIS2D result is established for macOS, Windows or Linux.
- The scoped Rust transport diff passed the whitespace check. The shared launcher diff contains a previously present blank line at EOF from an unrelated change.

## Path lengths

Absolute paths were counted in UTF-16 code units. The longest changed or added path was 205 of the allowed 256 units.

| File | UTF-16 units |
| --- | ---: |
| [Native launcher](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🟦️.ts) | 139 |
| [Native asset stream](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🦀️.rs) | 177 |
| [Tile URL resolver](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🧭️tile-proxy-url/🦀️.rs) | 195 |
| [Rust URL law](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🧭️tile-proxy-url/🧪️tests/🦀️.rs) | 204 |
| [URL fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🌐️native-asset-stream/🧭️tile-proxy-url/🧪️tests/🔣️.tsv) | 205 |
| [Native launch integration](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/🧪️native-tile-launch.cjs) | 117 |
| [Native launch fixture](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/🧪️native-tile-launch.json) | 118 |
| This audit | 120 |

## Pinned Node Reproduction

The pinned Node launcher integration was rerun and passed. It intentionally checks that the Node API returns a server while the listen event is pending, then verifies the child callback sees it listening. Bun starts the equivalent listener synchronously on this host, so the fixture's explicit pending-listener assertion requires Node. The production launcher handles either listener state.

```zsh
ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END'
repo tools exec node --light -- --experimental-strip-types "$ticket/🧪️native-tile-launch.cjs"
```
