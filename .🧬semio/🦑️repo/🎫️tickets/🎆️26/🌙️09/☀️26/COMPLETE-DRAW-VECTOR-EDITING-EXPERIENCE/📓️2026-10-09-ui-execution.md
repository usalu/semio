# Draw UI Execution — 2026-10-09

## Implemented Source

- Layer panel exposes all ten admitted creation kinds through existing `addLayer` semantic command and selection effect. Existing first five row ordering remains stable; added ellipse, line, polygon, image and trace. Virtualized logical roster totals and window tests updated.
- Inspector exposes Corner/Smooth/Symmetric node actions and numeric simplification tolerance in document units through `editPath`.
- Dedicated authored rectangle position/dimensions, ellipse/circle centers/radii, line endpoints and image asset key/dimensions use existing `patchLayers` actions.
- Polygon vertices use indexed, labeled X/Y controls and optional `PatchLayers.index`. `PanelTreeBuilder.window_indexed_section` wraps existing framework indexed windows, avoiding a complete polygon list copy during UI projection.
- `UpdateImage` has typed schema-first payload, sparse changed image key/width/height diff, explicit inverse, Rust/TypeScript mutation surfaces, wire fixtures, retained host digest/owned text preparation and apply.
- `SetShapeCoordinate` has typed coordinate field payload, sparse coordinate-list diff, per-coordinate inverse and composition, Rust/TypeScript mutation surfaces, retained host digest and constant-sized coordinate apply. Domain execution supplies validated scalar geometry accessors.
- All new labels are explicit English/German in native/reuse terminology. New controls preserve existing inherited lock disabling, mixed-field intersection and commit-on-blur event bindings.
- Canonical mutation aggregate references, KINDS, Rust module declarations, semantic metadata tags, and subset oracle manifests include both leaves. New tests compare independent Immer output and Ajv admission. Existing permanent Draw test router includes all new source/test entries.

## Schema Limits

The current shape schema has no rectangle corner radii. Text now requires explicit fontFamily from Anta, Kelly Slab, Share Tech Mono or Noto Emoji, alongside content, size and position; the font execution lane owns its schema, localized inspector and portable glyph producer. This execution exposes actual authored facets without inventing unsupported controls. Polygon topology addition/removal remains conversion-to-path plus existing path editing; authored polygon vertices can now be edited directly.

## Validation

Nx `@semio-tech/draw-drawing-rs:test --skip-nx-cache --outputStyle=stream` launched in session 36567, log `🗑️generated/ui-rust-test.log`. At this document revision the native owner command remains live and no Rust result has been reported. No claim of native passing or end-user runtime behavior is made. Parent owns isolated TypeScript suite integration and product runtime acceptance. Initial parent TS suite reported 597 pass / 1 skip / 1 fail due to duplicate missing schema IDs in other newly added asset leaves; both authored-facet schemas have unique IDs, and command execution repaired the asset IDs afterward.

## Shared Sources

Drawing root module tree, aggregate mutation/diff schema Rust/TS/JSON/GraphQL, field-input decoder, PatchLayers optional index, main editor field vocabulary, inspector/layers/terminology, semantic leaf fixtures and tests, subset oracle manifests, retained owned authority and test corpus, framework plugin indexed builder, Draw TypeScript permanent test router.

## Lane Handoff

All authored-facet local TypeScript relative imports were checked against current filesystem; two semantic shape test imports were corrected to six parent levels. All new authored-facet JSON files parsed successfully. This is source validation only. Native session 36567 was repolled and remains live; its owner-command log reports elapsed execution (last observed ~361 seconds), with no terminal native test result. Parent must poll this exact handle and inspect final diagnostics, then rerun isolated TS full test target and runtime product acceptance. Do not restart this native invocation merely due to observation timeouts.


## Encoded Image Import Follow-Up

The native session36567 handle disappeared; the frozen native log stopped at421211ms without a compiler/test terminal result. A process inventory found no corresponding Draw test process. This is inconclusive evidence, never a native pass.

The initial-window utility contract already exists in the domain-neutral framework and Draw declares selectDirect. The runtime gap audit did not establish a missing utility contract, so no speculative utility replacement was added.

A schema-first importImage action now consumes the existing file-open effect with PNG data URLs. AddLayer and DropLayerKind image choices request a supported PNG source; dropped image choices preserve the parent/index through host callback arguments. The action has explicit English/German labels and is hidden from the direct command palette because encoded payloads originate at the host file boundary. A retained admission job stages the encoded source, invokes the existing cooperative decoder, and emits ImportImageAsset plus CreateLayer and replacement selection together after decoding. Layer dimensions match the admitted intrinsic pixel extent. Source, pixel and chunk limits remain explicit. Only PNG is offered because the existing framework decoder rejects other MIME types.

Native decoder result transfer now keeps completed BinarySourceJob/PngDecodeJob/ImageDecodeJob children and their original allocations in the retained workspace until funded close. Cancellation and errors retain private owners. RasterImage/header/decoder types have first-party RetireOwned contracts; the PNG inflater child delegates its actual retained allocation authority. A consumed output refuses repeated publication. Decoder-owned String/Vec/Arc fields retire through the current value retirement framework rather than being replaced wholesale by cancellation. The importing work uses retirement::controlled::ControlledRetirement, matching the actual current source.

Neutral import fixtures cover supported PNG variants, independent work grants, refusals and cancellation. Rust tests additionally specify reversible atomic semantic import and exact child close grants. The TypeScript import job matched independent pngjs sample output: isolated Nx actual owner16 tests passed,0 failed,1445 assertions, followed by strict source checking and exit0. The raw child evidence is owner-test-image-import.log. Initial Nx plugin-worker failure occurred before owner execution; retry used NX_DAEMON=false and NX_ISOLATE_PLUGINS=false.

Native pixels production source compiled. The first independent native attempt exposed one incorrect ControlledRetirement test import, which was corrected. The broader pixels owner then stopped at an existing DEFLATE test because Bun1.3.14 ftruncate on piped stdout returned EINVAL on Windows; this is not recorded as a full native pass. A focused native image_sources test is in progress at report time. The full Draw native editor remains blocked by shared OS kernel compiler errors documented separately by the coordinator; no current end-user import workflow is claimed verified.

The ticket isolated Nx input gained a pixels-test target delegating the existing Pixels owner with an explicit cargo-test policy. Executable selector/launch entries were added for image-import and image owners. Generated logs and compiler products remain inside the ticket generated folder. No runtime external dependencies were added; the binary-source/pixels retirement dependency is the existing first-party value module.


## Final Import Lane Verification

The actual focused native image_sources selector completed5 tests with0 failures.47 encoded PNG transport vectors matched the independent native base64/percent-encoding/png oracle;35 source/result/retirement witnesses closed every private owner using exact next-item, copy, capacity, release and depth demands. The first test run deliberately retained an external source Arc and consequently blocked custodial retirement; the test now checks the source before releasing its external leases, then verifies terminal close. The earlier zero-test selector exit is inconclusive and is not counted as evidence. The final child log owner-pixels-test-image-sources.log ends with DEBUG exit0 and real5-test output.

The final image-import TypeScript selector reran after parent/index schema and source changes:16 passed,0 failed,1445 assertions, strict source exit0. The final test output is owner-test-image-import.log. Native image import DTO/work/publication tests remain authored but unexecuted because the full Draw artifact cannot compile through the concurrently changing OS kernel. Browser/native end-user import behavior has not been verified. A lazy retained workspace prevents an invalid unopened import route from abandoning a live ControlledRetirement owner on preflight rejection.

A path-length inventory of generated native decoder artifacts found no paths over256 characters. The shared ticket runner preserves source-scoped verification, managed Bun1.3.14 identity, fresh log truncation and the coordinator's ui-test native selector. Focused image-source arguments are selected in the input router instead of relying on Nx to forward arbitrary compiler flags. Generated artifacts stay under the ticket generated folder for coordinator cleanup at ticket completion.

## Current Integration Prerequisite Evidence

The historical OS Kernel blockage above describes the earlier lane snapshots. The canonical JPEG factory publisher subsequently reached actual native compilation and passed its original carrier law, publishing a protocol receipt matching current source. MP3 and PDF17 authentic publisher receipts remain sequential prerequisites for the standard managed Draw serve. The new gesture original issuer source has three passing tests and 37 assertions plus strict TypeScript; its fresh exact native replay remains pending after coordinated Plugin fixture compiler repairs. Current browser end-user creation, layers, path painting, history, export pane and EN/DE journeys have still not been run against the freshly materialized mounted app. There is no current Draw end-user acceptance claim.

## Canonical mounted acceptance entry

The existing launch entry at386.31 runs `@semio-tech/framework-os-dev:serve-draw-react-dev`. Its actual Dev script lazy-loads activation/serve, validates the requested variant/profile activation receipt, ensures the declared local hub, and starts the declared Vite configuration with fixed catalog port and explicit Draw/React environment. The repository cache-contract source expects each `serve-<variant>-react-<profile>` target to delegate that same permanent script and depend on the original plugin-registry session. This was read-only source inspection, not a serve or browser pass. No alternate Vite process, stale runtime, private target or codec protocol bypass was introduced.

Authentic JPEG publication now matches current source. Read-only inspection still finds current protocol mismatches only for MP3 and PDF17. The original MP3 publisher is live; PDF17 follows after its terminal receipt. Mounted acceptance remains pending. Required real journeys include localized creation choices and sparse properties, image file import and selection, layers and inherited locks, node modes and simplification, no-op-safe history, cancellation/progress, and PNG/SVG/PDF export through the declared owner infrastructure.

## Current browser surface observation

After reading the provided computer-use skill and its guidance/confirmation policy, this lane used the purpose-built CUA browser inventory only, following the skill preference for browser automation. The actual current `cua.getState()` response was `apps:[]` and `browsers:[]`. No app or tab was created, no managed server was started, and no UI input was sent. Earlier parent observations of two browsers are historical or belong to its own enabled surface; this observation must not be treated as a current browser acceptance pass. The canonical managed Draw server and its emitted URL remain the required entry when codec receipts are stable. Native automation is disabled in this lane's CUA tools.

## Current raster import format authority

Fresh read-only source inspection confirms the existing dedicated image action remains explicitly PNG: its file-open effect accepts `image/png,.png`, both native/TypeScript validation require the PNG base64 prefix, the localized label says Import PNG Image/PNG-Bild importieren, and the shared neutral corpus deliberately rejects JPEG input. The native framework ImageDecodeJob itself rejects every MIME except image/png before creating its yielded PNG decoder. Expanding only the picker or label would therefore create a nonfunctional route; additional raster formats require a real decoding/conversion producer and shared oracle fixtures. This checkpoint is distinct from the parent's current editable SVG ingress work and from authentic JPEG codec publication, which proves its Stdio document carrier rather than Draw raster admission. No raster import or decoder source was changed by this inspection.
