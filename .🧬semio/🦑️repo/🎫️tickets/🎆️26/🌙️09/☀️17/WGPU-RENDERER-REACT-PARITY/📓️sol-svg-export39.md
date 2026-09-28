# First-Party Icon SVG Export

## Contract

The Icon export batch now selects SVG immediately after the decoded `World3dMeshAsset` is accepted. `IconSvgExport` retains that exact asset owner until its bounded face, edge, and retirement phases finish; only then can `take_svg` publish one byte payload. Cancellation and refusal return through bounded `close_step` ownership retirement, and no PNG raster is embedded into the SVG.

The shared request validator enforces the neutral export-request schema before asset fetch or export: nonempty asset URL, positive finite zoom, finite camera vectors, FOV inside `(0, 180)`, supported shape, integral dimensions no larger than 16384, and the existing pixel-product budget. `IconExportScenePreparation::request_format` exposes the validated format to the Shell batch without a second parser.

## Renderer parity

The serializer projects the accepted mesh with the authored camera, near/far clips faces and edges, applies Three SVGRenderer-compatible Lambert color projection, uses the decoded 1-degree outline stream, preserves source-order ties under descending painter depth, applies 0.5-pixel overdraw, and emits the centered viewBox/background/ellipse clip used by the React implementation.

Painter admission uses a `BTreeMap` keyed by descending `f32::total_cmp` depth plus source sequence. Each face or edge costs `O(log n)` and one bounded step. Candidate path/style bytes are rejected as soon as their accumulated budget would exceed 64 MiB. Body output starts a style group, appends one path fragment per step, and closes that group separately, so it never formats or copies the entire accumulated path in one step. Geometry progress is stored at admission and remains monotonic after the mesh owner has retired.

`new` clones only primitive metadata already bounded by the shared World mesh primitive capacity. It does not scan vertex/index payloads. Face and edge reads occur one source primitive per `advance` step, except the constant maximum of three clipped triangles produced by one source triangle.

## Neutral and third-party evidence

The language-neutral input and expectations live in:

- `IconRenderHost/🧬️schema/📤️svg-export/🔣️.json`
- `IconRenderHost/🧫️fixtures/📤️svg-export/🔣️.json`

The actual installed Three SVGRenderer and Sharp oracle is `IconRenderHost/🧪️tests/📤️svg-export/🟦️.ts`. Its focused React Nx run passed 2/2 tests in 27.14 seconds. It verifies the exact material/light fill, edge stroke, two grouped SVG paths, centered ellipse clip, explicit clipped background, and transparent/outside pixels.

The Rust law `IconRenderHost/🧪️tests/📤️svg-export/🦀️.rs` reads the same fixture geometry, creates the exact mesh/appearance owner, and verifies bounded success, stable progress through retirement, one-shot delivery, expected markup, and bounded cancellation with an empty terminal owner. It emits a `[DEBUG] icon SVG neutral receipt` with steps, bytes, and path count. This law was source-ready for the parent-owned native40 run; no native pass is claimed here yet.

## Known authority limits

Installed Three SVGRenderer ignores PBR metalness/roughness, material textures, emissive intensity, ambient intensity, and shadow maps, so the first-party serializer deliberately does the same. The current decoded carrier exposes one vertex-color lane and primitive-wide edge stream; it cannot represent glTF `COLOR_0` arity distinctions or per-primitive outline ranges beyond the carrier. Those carrier constraints are recorded separately in `📓️sol-svg-export-implementation-audit39.md` and are not hidden by a raster fallback.

## Retained UI receipt

The earlier six-law UI session `67141` compiled successfully, then ran three laws before fail-fast: retained progress and Tree action-row occlusion passed; same-size moved-origin failed at `targets-wgpu-engine-unit/🦀️.rs:1868` because the first-origin candidate had already published. The three prepared-readback laws did not run. The PTY had no saved ticket log, so this is a receipt only and not a passing claim.

## Atomic origin-law correction

The same-size origin-move production fence already includes `RetainedPaintFrame.viewport_origin` and cancels/retires a candidate when the next `frame_into_step` origin differs. The failed law inspected `DrawList.layers.is_empty()`, but `DrawList::default` owns an empty base layer. The law now captures `UiFramePaintCensus::of(target)` before painting and requires that complete visible census to remain exactly unchanged throughout the first-origin candidate and its discard/repaint. It then retains the decisive accepted-origin checks over every published UI rectangle and hit rectangle. The focused six-law rerun is recorded at `🗑️generated/sol-flow34/ui-six-laws-2.log`. Nextest run `b3550868-1be5-40d3-b4dd-5c4df6795faf` passed all 6 selected laws with 720 skipped: moved-origin atomicity, retained progress, Tree action-row occlusion, two neutral readback laws, and the actual production GPU readback/cancellation law.
