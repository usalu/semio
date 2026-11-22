# Combined Basemap Scene Order

The shared map scene now paints the screen-pinned vector backdrop first, raster tiles second, and the cached vector features and labels above the raster. Document overlays still paint last. The backdrop remains visible through gaps in the tile coverage, and the vector tile cache continues to hold only world-positioned content. The same style and tile-availability checks choose when to paint the wash.

The language-neutral fixture defines the portable drawing order for Combined, Vector, and Image, including Combined with either tile family absent. It is consumed by the Surface test, which builds real map scenes and checks the draw-list encoding with an independently decoded PNG raster tile. The ticket's small native acceptance script uses the same fixture and compiles the scene assembly with recording stand-ins for layer painting. It failed on the original Combined order of raster, backdrop, features, and passed after the change. The Git whitespace check passed.

The native acceptance script ran without Cargo or worker compilation. The full Surface test and the browser check of the default Combined view require a new Surface build. The earlier browser workflow used a temporary Land override to expose the raster under the original ordering.

## Prototype Surface Publication

The direct Surface owner command refused because it requires an Original parent Script invocation. The repository target is the authorized entry point. The first graph-wide request waited below the planner's 2,048 MB memory floor and was canceled before invoking the owner. The owner-only request sealed its graph, then waited for an 8,190 MB execution reservation. The serial retry uses `SEMIO_BUILD_MODE=prototype repo run @semio-tech/framework-surface-rs:wasm --exclude-dependencies --skip-cache --parallel=1`. Repo reads this flag into host capacity planning before invoking the owner.

Before publication, port 6041 served the previous Surface Wasm at 11,168,583 bytes, with SHA-256 `8f0e2339a91d066aaf3b52eb5a5cab83129158f1088dc364d898bce5c7462857`. This is the comparison baseline for the owner output and the bytes served by Vite.

The single-job attempt received its 3,584 MB execution grant and coalesced with the live workspace dependency sync and generation writer. The Nx step requested a fresh process admission after dependency changes and repo transferred the same grant to its replacement. The replacement successfully compiled the prototype Surface Wasm and generated the package with wasm-pack in 17m 43s. Publication then failed with `SEMIO_LEASE_ORDER`: it held an exclusive source-capture lease and spent 15 seconds waiting for the exclusive destination lease, while the live GIS3D React playground held a shared reader lease for the same Surface package.

The owner removed the temporary `wasm-package-14YGG8` capture and Cargo output through its normal cleanup. No staged Surface package survived for a continuation. The existing playground launcher only builds engines whose publications are physically missing, and its route can also build missing components. The exact Surface project target is the supported refresh route once publication can acquire its writer lease. The live GIS3D reader and the existing Surface artifact were preserved. The default-Land browser check requires Surface bytes compiled from the revised scene source.

## Isolated Browser Acceptance

The ticket's Canvas2D acceptance route provides a governed way to compile Surface for the browser while the shared publication remains in use. The command builds the Surface component with `repo cargo --store=isolated --configuration=prototype --parallelism=1 -- build --locked --lib --manifest-path '<Surface component Cargo.toml>' --target wasm32-unknown-unknown --message-format=json-render-diagnostics`. Cargo's compiler-artifact message identifies the actual linked Wasm. The Surface scene source SHA-256 before the build was `b218c47a2653b8ab66bb5fc75b84c013d61650c152f04094fa6a3a87032f8ea0`.

The repo's wasm-bindgen executable is version `0.2.129`, as specified by the repository and framework lockfiles. The companion JavaScript and processed Wasm for the diagnostic are emitted together in the ticket's generated directory. Existing Surface glue resolves `framework_surface_bg.wasm` relative to its own URL. A browser context can respond with both diagnostic files at the live app's exact Surface URLs while the Vite server continues serving the published package to other clients. An initial Chrome workflow with the earlier isolated Surface package loaded both replacement files and passed pan, wheel, and keyboard with 49 tile responses.

Cargo successfully linked the real Surface component from its owning manifest in 18m 26s. The compiler reported a newly built prototype `cdylib` for `wasm32-unknown-unknown`, and the revised scene source had the same SHA-256 before and after. The compiled component was 13,467,078 bytes with SHA-256 `ef4c448e26368f68f1a8ac447d45815a9a36d5d0b846dfa3740138d29c243dc2`. The earlier shared host target was absent, so the isolated compiler rebuilt its closure under a single admitted job. No shared publication or server restart was involved.

| Paired file | Bytes | SHA-256 |
| --- | ---: | --- |
| `framework_surface.js` | 98,929 | `6e6f87c4c033891d8005a57854abbd1de11c5e0aa036da56cbe09ad792261226` |
| `framework_surface_bg.wasm` | 11,524,569 | `e79ae3841fd78a6f1292739052f14caadf49dd1fbf58a103221d6608054c53b0` |

The pair and its diagnostic provenance manifest were produced under `🗑️generated/surface-current-acceptance/package`. The glue exports `MapSession`, uses a same-directory Wasm URL, and declares no further module imports. The canonical Surface Wasm was still `8f0e2339a91d066aaf3b52eb5a5cab83129158f1088dc364d898bce5c7462857` after this build.

### Desktop Chrome WebGPU

On the live GIS2D React app at port 6041, the diagnostic Playwright context used the JavaScript and Wasm pair with the hashes in the table above. The harness observed the published Colored style and left Land enabled throughout. Vector, Image, and Combined reached the published mode in 3,071 ms, 2,076 ms, and 1,525 ms respectively. Each settled canvas met its mode-specific pixel check, including Combined differing from both Vector and Image. The browser received 48 distinct successful vector tile paths and one raster tile path. Trusted middle pan, wheel zoom, and keyboard interactions also passed on the WebGPU renderer, with no browser console errors. The log is `🗑️generated/current-surface-default-modes-webgpu.log`.

### Desktop Chrome Canvas2D

The same diagnostic package and default style checks passed when Chrome ran with WebGPU disabled. Vector, Image, and Combined reached the published mode in 4,577 ms, 1,921 ms, and 1,538 ms. The settled canvases passed the same pixel checks, and the browser received 48 distinct vector tile paths and one raster tile path. Trusted middle pan, wheel zoom, and keyboard interactions passed on Canvas2D, with no browser console errors. The log is `🗑️generated/current-surface-default-modes-canvas2d.log`.

Canonical publication remains pending the existing GIS3D reader's shared Surface lease. Both acceptance runs used the current-source diagnostic pair in disposable browser contexts while the live servers retained their published package.
