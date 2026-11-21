# GIS Component and React Staging

The retained GIS component logs reached the known terrain compile errors. The native attempt then rejected an updated hub Cargo unit graph that no longer matched its held plan. No later GIS compiler diagnostic appeared in those runs. The ongoing launch had reached 84 compiled units and 470 reused units while still in component staging at the last check.

The GIS React entry resolved its workspace imports from its own directory. A direct probe of Vite's managed Wasm package resolver found the declared prototype Surface, Editor and Puzzle entry paths. Their generated files were still absent while the component launch was in progress. The browser launch owns their staging. End-to-end startup from those staged artifacts remains to be checked.

## Tile HTTP Delivery

React Vite and the native WGPU asset server both use the same tile proxy middleware. The proxy had returned 404 for every unsuccessful upstream tile response. That meant rate limits and service outages reached the renderer as definitive misses.

The proxy now returns upstream client and server error status codes, with local 502 for unexpected transport status and errors. Live error responses use a local no-store policy. The proxy releases unsuccessful response bodies and sends no upstream response headers. The success path still serves the existing file cache and validators.

The loopback HTTP tests reproduced a rewritten 410 in both delivery paths before the fix and passed afterward. Their language-neutral fixture exercises 404, 410, 408, 425, 429, 502, 503, 504 and successful retry and cache reuse. It checks that upstream cookies, rate-limit details and Retry-After stay upstream. The two HTTP cases and the existing bundled-output case passed together with 137 assertions.

## Concurrent Cold Cache Reads

Two overlapping requests for one uncached tile made two upstream calls before the change. The shared proxy now uses one live download per cache destination, upstream and user agent in a process. Disconnecting the first reader leaves a sibling attached. Disconnecting all readers aborts the upstream request and allows a later request to make a fresh attempt.

The proxy stages complete response bytes beside the cache target, then links the complete file into place. A separate process may make its own upstream call, but its completed file cannot overwrite a winner already published at the final name. The temporary file is removed after publication or cancellation.

Five focused tests passed with 158 assertions across HTTP status delivery, overlapping cold reads, last-reader cancellation, cache reuse and bundled output. Both cancellation cases checked the exact cached and served bytes after a gated upstream response. The focused TypeScript check reported no proxy diagnostics and one existing missing declaration for `fresh` in the surrounding asset suite.

## Bundled Tile Cache

The bundled-output test reproduced a private tile staging directory in the output. The copy now skips staging directory names under numeric zoom and column directories. It still copies complete tiles, similarly named files, and directories outside those coordinates. The five focused asset tests passed with 166 assertions.

Windows NTFS supports hard links between paths on the same volume. The cache creates its stage under the destination directory, so both paths share the volume. A local Bun probe verified complete publication, `EEXIST` on a second attempt and preservation of the published file after removing the stage. Windows behavior was checked against the Win32 documentation. [Microsoft CreateHardLinkW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-createhardlinkw) · [Node file-system documentation](https://nodejs.org/api/fs.html)

## Concurrent B-Rep Source Edit

The GIS WASI log showed the B-Rep analysis crate calling `BoxSectionDraft::convex_edges` against a compiled representation that exposed only the earlier constructors. The live representation source already contains a peer's public, unconditionally compiled constructor with the exact signature used by analysis. The staged-to-working-tree diff confirms that the constructor is the newer edit. No B-Rep files were changed during this check. The active GIS rebuild owns compiler verification.

The immediate retry reported the same pattern for `skew_box_union` and `SectionPolygon::ordered_ring`. Current source includes both public definitions in their already-exported modules, with no target or feature gates. The working-tree diff shows both definitions as peer additions. The retry log shows that Cargo compiled the representation and construction crates before it checked their newer consumers. No source adjustment was needed.

## Early Warm Cargo Retry

The warm single-component driver now uses the existing bounded source watcher before the component links. A stable SHA-256 mismatch against the admitted source can retire the owned Cargo child when a distinct retry remains. The driver drains that child, its streams and the watcher before re-admitting the source and validating the replacement unit graph and held storage. Linked or exited children, cancelled builds, repeated witnesses and the final attempt keep their existing result handling. Multi-component and fresh builds keep their existing behavior.

All 35 focused warm-retry tests passed. The neutral cases check the retirement decision against a SQLite predicate. A gated local process test verifies that a real equal-length file edit produces the digest pair, terminates the owned child and leaves the caller's abort signal open. The full GIS build was left running with the source it loaded when it started.

## Mounted Instance WASI Source Budget

One GIS launch measured the `semio-plugin-instance-app` source closure at 263,166 bytes and refused it against the 262,144-byte ceiling. The live shared sources passed a new measurement with the repository source-closure tool and the pinned nightly compiler's `wasm32-wasip2` cfg. The closure measured 259,426 bytes across 18 files, with 3,079 active lines in production and 3,139 with `artifact-app-testing`. The limits remain 262,144 bytes, 32 files and 4,000 active lines. No source edit was needed during this check.

## Cold React Launch Preflight

The generated catalog resolves `gis2d` to plugin `gis` and app `s.gis.gismap@1/*#editor`, with React on port 6040 and browser WGPU on 6140. The graph-free browser component closure contains only `gis`. Its prototype playground session is already published. At the preflight checkpoint, browser publication for GIS and the React activation receipt were still absent while Cargo was running.

The launcher also prepares browser support and fonts, and stages the managed Wasm engine facet before activation and serve. The managed resolver paths and producer scripts exist. The engine manifests and corresponding JavaScript and Wasm files were absent in the prototype stage at the checkpoint. Vite serve uses the Flow development stub and does not require the distribution Flow browser package. The authored GIS contribution supplies the GIS backbone worker and Puzzle session factory when the catalog selects it.

The GIS app declares `demo` as its default example. The development shell resolves the boot example and waits for its publication before setting the ready marker. The map uses same-origin `/osm/{z}/{x}/{y}.png` and `/vt/{z}/{x}/{y}.pbf` routes served by the configured asset middleware. The probe matches these routes and response media types and reads the canvas renderer marker.

The launcher passes `gis2d` as `VITE_SEMIO_PLUGIN`. With the authored GIS browser entry, `bootFrameworkOsDev` and its boot planner preserve the variant, and the React shell publishes it as `data-shell-ready` through its plugin filter. The device probe now requires that exact value and reports shell error and not-found markers separately. A fixed-port Vite serve can reuse an existing successful HTTP server because the launcher omits its optional expected entry. Port 6040 refused a connection during preflight. The app identity assertion protects the probe against a ready shell from another variant.

The browser probe parses under the repository's Node tool, all seven device profiles are available, and the repository Playwright cache has all three browser families. The queued probe supplies the cache path and Chrome channel. Live startup and actual map rendering remain pending publication and activation.

A loopback server exercised the existing controlled canvas fixture with the three Chromium profiles. Serving it with the GIS2D marker passed the tile and interaction checks. Serving the original `fixture` marker or a GIS2D not-found marker failed immediately for all three profiles, with distinct diagnostics. Controlled runs can serve the same fixture with `data-shell-ready="gis2d"` for the positive case. These checks exercise the probe against synthetic pixels and tile responses.

## Renderer TypeScript Diagnostic Review

Reviewed all 30 diagnostics in the retained React typecheck log against the GIS2D browser entry. The one missing runtime export, `FlowOperationFields`, has its only renderer import and use in a NodeGraph test. The remaining production diagnostics concern layout and data attributes, optional image source metadata with an explicit absence path, Flow node-hover payloads after interaction, and a timestamp type assertion in WGPU frame transport. None identifies a missing runtime import or load path on GIS2D React startup.

The historical MapSession typecheck note predates the current bridge. The live Rust source declares the tile lookup and revision methods and Canvas2D attachment with the JavaScript names used by React. The public React Vite config registers the managed Surface artifact resolver before the Flow and generic missing-package stub resolver. The component and engine publications still need to be exercised by the live browser run. This review used existing logs and current source and made no production changes.

## Release Descriptor Emitter Staging

The graph-free release launch stopped before Cargo could enter the component lane. The driver required a descriptor emitter and found no artifact manifest in the release host publication. The launcher now checks for the manifest and executable for the selected profile and host before it builds missing boot components. It calls the managed emitter producer with the original authorized invocation and compiler admission control. Warm launches with all boot components prepared skip this step.

The descriptor driver still verifies the published artifact, its compiler provenance and its actor preparation capability. Both exact prototype and release host publication directories lacked a manifest at the preflight checkpoint. A present emitter with an outdated capability remains a driver error.

The new neutral fixture covers missing, incomplete and existing artifacts across prototype and release and all three supported host platforms. Its expected paths and binary names were checked against SQLite. The new regression failed before the launcher change and all six launcher tests then passed with 113 assertions. The public release launcher is the remaining end-to-end check.

## Declared Cargo Preparation for Cold Launch

GIS directly depends on the schema-only stdio catalog definition. Its manifest declares the existing `composition prepare-catalog-definition` recipe, which publishes and observes the compressed file consumed by Rust. The graph-free component driver previously went from the selected lock, configuration and registry inputs straight to source admission without running owner preparation. The 28,039-byte bundle appeared during the investigation through separate work.

The graph-free launcher now requests owner preparation for the exact missing components. The driver computes their Cargo plan once, runs the existing repository preparation entry once per selected workspace pass and reuses that plan for compilation. The preparation entry receives the selected mode, target, package names and original invocation control, and recursively handles the declared prerequisites. The warm boot path skips component preparation when its browser publications are already staged.

The neutral fixture checks zero, single and multiple workspace selections, including two packages in one pass, against SQLite. The focused regression failed before the implementation. All eight launcher tests passed with 161 assertions afterward. No generated catalog data was written by this change.

## Independent Browser Engine Staging

All four managed manifests and payloads were absent in both selected-mode product trees. The existing producer commands compile `wasm32-unknown-unknown` with wasm-pack and publish the checked package under the profile-specific products store. They use their own compiler lanes and require CPU capacity. The running GIS component uses `wasm32-wasip2` and cannot supply these packages. The scoped graph run ended in inventory and resident service failure before it built any engine. A direct Editor owner dispatch with an unused command refused the missing original parent before command selection, confirming that the owner script is a managed child.

The 158 rows in the current catalog each require at least one plugin browser module. The prototype store had none of those modules published. Its existing GIS2D, GIS3D and S sessions do not include module publications. No existing empty or ready playground can reach the public graph-free engine loop without building a component first.

The catalog check also found that the standalone GIS2D and GIS3D rows omit `devContribution` in both their authored metadata and the current projections. Their declared GIS development entry is therefore unused on that path. The launcher chooses the core Vite fallback with empty inventory and asset providers, and the fallback entry submits an empty variant. The authored GIS contribution selects the correct GIS shell and the four-engine Play facet. Wiring the standalone contribution and republishing the catalog is required before the device probe can verify this path. No launcher, catalog or engine artifacts were changed during this check.

The standalone GIS catalog regression failed on the missing contribution in the authored GIS2D row. It checks both standalone apps in the authored manifest and published projections, loads the Vite config selected by the actual playground and checks the engine facet and tile middleware routes. The managed publication target is `repo run @semio-tech/s-dev-composition:generate`. Its child publication computes and reconciles the complete declared catalog output. A direct read-only child preview refused the absent original caller. The manifest and the published projections remain pending the active compiler and provenance transaction.

The canonical catalog renderer accepted the two declaration edits in an in-memory overlay and retained both GIS apps and their expected routes. Comparing canonical baseline and overlay outputs isolated the change to `🎠️playgrounds.json`, `🚀️playgrounds.json`, and `🎮️playgrounds/🟦️.ts`. No generated Rust changed between these inputs. A whole-tree publication would also rewrite seven outputs due to separate work already present in the shared source.

Both published playground JSON files also differ from their canonical baseline on 26 unrelated variant rows. The current TypeScript projection exactly matches the canonical TypeScript emitter applied to the current launch JSON. A bounded publication can preserve all unrelated rows by taking only the two canonical GIS rows, verifying each differs from its published row only by the contribution, then re-emitting the existing row sets. An in-memory TypeScript check changed exactly the two GIS row lines. Publication remains subject to the compiler and catalog writer coordination described above.

After the earlier compiler and launcher exited, the two GIS manifest declarations were added and the bounded publication completed. The publisher used exact GIS rows from the canonical renderer, preserved the current rows for all other variants, and rechecked the authored manifest and each destination before replacement. It published the TypeScript projection before the two JSON projections. All nine launcher tests passed with 179 assertions, including the live catalog check for the GIS2D and GIS3D Vite owner, tile providers and engine facet. Live browser startup remains pending the warm component build.

## Cargo Preparation Source Drift

The warm replay stopped during declared preparation when a transitive program source changed after observation. The custody reader proved the difference but threw an ordinary error, and the caller received only the failing child status. Preparation now reports a changed or vanished program source as an existing build-layer concurrent edit. The owned synchronizer sends a bounded typed diagnostic with its source path and reason. The caller validates the failed child outcome and path containment and restores the typed error for the existing fresh-build retry.

Cancellation takes precedence over retry. The same source and artifact custody checks still refuse unproved state, and an output mismatch stays terminal. The regression exercises the real custody reader and the retry boundary with changed and vanished sources, a changed output, cancellation, an unrelated failure message, an unsupported reason, a path outside both roots and a signalled child. The active warm build was left running with its already loaded implementation.

The launcher, custody and diagnostic encoding suites passed together with 17 tests and 293 assertions. The synchronizer entry also imported successfully without launching a compiler.
