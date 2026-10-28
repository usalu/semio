# Exact UI Final Audit

## Source-Verified Requirements

The actual WindowSilhouette inset is applied to the outer bounds and cap baseline only. Internal title/control interval boundaries remain measured raw x coordinates. An inset-one path therefore runs at outer x/y 1 and gap y cap-depth+1. A centered one-pixel stroke paints outermost at half a pixel from the host edge. Content clipping uses inset zero; body fill begins at cap depth. These must not be collapsed into individually inset chip rectangles.

WindowChrome SVG uses miter joins and non-scaling-stroke. Normal stroke width is one CSS pixel. Its U gap remains transparent; glass is restricted to chip regions and the body. No whole-cap or whole-host translucent fill can match this.

The checked native geometry implementation preserves raw internal interval boundaries and nominal compact dimensions. Browser quantization of 22.4 to 22.390625 is expected layout rounding; 0.04-pixel geometry tolerance is appropriate. This is not a reason to change nominal metrics.

## Required Final Checks

- Verify geometry source/schema and actual owning UI sources participate in presentation cache identity. The presentation source_files implementation inspected before the root update did not yet include geometry.source_files.
- Inspect actual native title baseline/ink against browser screenshot, including accented labels, and inspect maximum-width truncation rather than accepting advance-only comparison. The original height-scaled text implementation is explicitly insufficient.
- Inspect native stroke pixels at canonical scale and during focus interpolation; preserve miter corners and avoid double-scaling stroke widths.
- Compare the final full overview and focus shell to current browser oracle, with transparent gap visibly showing the base floor.

## Status

Source audit complete. Final renderer edits and media were still in progress at this checkpoint; no final visual parity sign-off is claimed.

## Canvas and Shell Hover Ownership

Canvas component renders only floor fill, compact padding and a loading/waiting status border; idle status maps to an empty class. Mode and mode-body similarly own no ordinary border. The actual UI stylesheet contains no Canvas hover outline selector. Root and Canvas rectangle outlines invented by the renderer must be removed, while ancestor hit/hover semantics may remain. Navbar owns only its bottom horizontal one-pixel pseudoedge and footer only its top horizontal one-pixel pseudoedge. Their owner hover switches normal to emphasized color with a 120ms transition (ui.css6559-6582); neither owns a full rectangle.

## Native Image Review

Reviewed generated/exact-native overview, click and focus against generated/exact-tests overview. Cap heights, chip widths, tiled bounds, clear gaps and title logical size closely align. Magnified crops are retained temporarily under generated/exact-final-audit.

Actionable mismatches observed before final corrections:

1. The initial visual interpretation of focus y0 was incorrect. Independent Pillow sampling confirms navbar divider28, dark floor through31, focused stroke begins32/33 and ends867. Focus geometry matches the source-derived shell bounds.
2. Native icon is diagonal maximize2 arrows; browser oracle icon is four bracket corners. Actual source uses Maximize2; native is correct and browser oracle must use the same source paths.
3. Native logo is a white block with knockout mark, rather than the actual multicolor emblem. Root is correcting SVG fill inheritance.
4. Thresholded label ink above110 begins y40 native versus y39 browser, but full row-luminance profiles show both labels first diverge from background at y39. This demonstrates antialias/font-raster weight differences, not a proven baseline translation. Logical extents and advance agree; do not shift native text by1 based on threshold alone.
5. A body-fill pixel200,100 reads native RGB7,24,29 versus browser6,25,31. Gap pixel200,40 matches0,17,23 exactly. Body glass blend differs by up to2RGB and should be reconciled with canonical color-mix compositing.

No final exact-pixel sign-off was issued for these images.

## Actual Browser Production Port Review

Production chrome now moves to the actual React components/CSS/fonts because native font rasterization and glass color mixing could not provide exact UI parity. Reuse WindowChrome, Navbar, Footer, SemioLogo, CSS entry and canonical icon assets without a manual CSS fork or native fallback. Native geometry/fonts remain content helpers.

The existing public repository interface @semio-tech/process/tools exports loadPlaywrightV1(root), implemented at framework/modules/process/tools/playwright. It installs the repository shared PLAYWRIGHT_BROWSERS_PATH environment before loading the pinned package. Production must use this interface rather than direct playwright plus a win32 msedge channel. The existing tool-setup chromium contract pins the browsers.json revision and provisions with the dependency bootstrap sync/browsers preparation. Setup must provision this browser consistently on Windows, macOS, Linux and devcontainer; a system-browser fallback would invalidate reproducible screenshots.

Cache/source closure must include esbuild metafile inputs, Tailwind compile onDependency paths, scanned UI class candidates, canonical SVG/font assets, token/CSS entry and package lock/version input. Source-scanned new classes can affect CSS even if no imported component changes.

Batch protocol requirements communicated to implementing agents: explicit locale; fixed canonical CSS viewport and DSF1; single output scaling; finite geometry/output validation; font ready and newly measured silhouette ready after each frame, not an old selector; bounded jobs; machine-readable stdout; progress/errors separately; cancellation closes browser descendants and cleans partial output. Screenshot only after pointer/hover and transient time are intentionally set. Readiness must await ResizeObserver/animation frames after geometry changes.

No final browser production screenshot was available at this audit checkpoint.

## Final Production Browser Preview

Reviewed actual production overview, menu probe and sampled tour frames. The overview now uses the canonical multicolor mark, precise cap and clear U gap, canonical text rasterization and actual Tree. Independent Pillow dimensions are1600x900. Body/chip RGB5,25,30; clear gap0,17,23; navbar/footer divider123,130,125 all exactly match the independent current browser oracle at sampled pixels. Earlier native glass and font raster residuals are resolved by browser rendering.

An actual composition issue remains in reviewed tour frame0012: hovering a Tree row fills the entire menu body grey and brightens every label. The production Navigation is nested in the Navbar item DOM, so the navbar descendant data-level hover selector matches the menu itself. Actual PopoverContent uses createPortal (Popover component404, menu level413 and SurfaceScope434); ContextMenu likewise portals to document.body. Correct the popup boundary by moving Navigation outside Navbar as an anchored scene sibling or using actual portal composition. Do not override CSS to hide this ancestry mismatch. This finding was sent to root before final sign-off.

## Final Resolution

Reviewed retained exact-ui.png, exact-ui-menu.png, exact-ui-click.png and exact-ui-device.png after the actual Popover portal correction. The earlier menu ancestry issue is resolved: only the hovered Tree row receives fill, sibling labels remain muted, the parent branch remains legible and the source cursor is visible. The menu no longer fills its entire body with navbar hover color.

Overview, click and focused device frames preserve compact caps, transparent U gaps, current canonical type and multicolor logo, compact tiled insets, and source-owned navbar/footer edges. Independent Pillow sampling confirms all four exports are1600x900. Overview navbar divider is row28 and footer divider row871. Root and Canvas outer samples x0,y100 and x0,y870 remain floor, confirming no invented rectangular borders. Decoded video stills have expected YUV color conversion differences from the lossless overview PNG; their geometry remains aligned.

The nested authored System and Heizgerät windows show aligned title caps, body outlines and the visible red An control. The cursor remains visible over the control, and heater plus1500W annotation remain unobscured. No residual visual blocker was found in the four retained final images.

Root independently reports the final video verifier passed99frames,12.375seconds,1600x900,8fps; first/last and tour/lesson boundary images are byte-identical, heater is off before click and active after click, release equals click, and full FFmpeg decode succeeds. This audit independently reviewed images and pixel placement; it does not claim to have independently rerun that video verifier.

### Final Device Refresh

Re-inspected the refreshed retained exact-ui-device.png after the final authored-body correction. System now shows its actual fullscreen icon inside the right cap. The Heizgerät An control remains visibly integrated with its silhouette, with no extra rectangular border apparent; its cursor, heater and1500W annotation remain clear. Outer actual UI chrome and spacing remain aligned. No visual blocker found in this final refreshed image.
