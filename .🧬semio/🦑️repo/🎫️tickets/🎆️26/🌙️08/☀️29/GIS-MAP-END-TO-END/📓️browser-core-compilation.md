# GIS Core Streaming Compile

The independent native transpiler produced the exact main GIS core at `🗑️generated/native-transpile-proof/semio_hub_gis_component.core.wasm`. The input was 138,445,673 bytes (132.0 MiB). The [diagnostic script](./probe-gis-core.mjs) served the file through a loopback HTTP stream with `Content-Type: application/wasm`, a fixed content length and `Cache-Control: no-store`. It started a fresh Playwright browser and timed one `WebAssembly.compileStreaming(fetch(...))` call. The time includes the local transfer and compilation overlap. The browser had no compiled-module cache entry. The host filesystem cache was left as-is.

| Browser | Result | Fetch through compiled | Transfer |
| --- | --- | ---: | ---: |
| Installed Chrome channel 158.0.8127.36 | Compiled, 73 imports and 104 exports | 1,321 ms | 1,305 ms |
| Pinned Firefox 157.0 | Compiled, 73 imports and 104 exports | 3,466 ms | 893 ms |
| Pinned WebKit 27.2 | Compiled, 73 imports and 104 exports | 1,001 ms | 967 ms |

Chrome received headers in 5 ms and Firefox in 10 ms. This measures main core compilation eligibility and leaves instantiation, Jco glue, async imports, rendering, and the full GIS shell to their own checks.

Each browser ran under a separate 1,536 MiB capacity grant and closed after one successful retrieval. Firefox used the ticket-local browser app-data settings needed on this macOS host. After an initial capacity delay, WebKit completed with 8 ms to response headers and 993 ms from headers through compilation.
