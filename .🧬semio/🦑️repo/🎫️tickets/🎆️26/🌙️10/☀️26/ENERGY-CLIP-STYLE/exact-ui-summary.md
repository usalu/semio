# Actual Semio UI in Energy Clips

The production clip chrome now renders the actual semio React components and source stylesheet. It no longer estimates the UI through a separate native drawing. WindowChrome, Navbar, Footer, Popover, Tree, source icons, bundled fonts and the actual computed cursor asset all come from their owning UI implementations.

The canvas fills with five docked windows using the real 3.2 px insets and gaps. The nominal chrome is 28.8 px for navbar/footer, 22.4 px for caps and control cells, 16 px for icons, 11.2 px Anta title text with its real line box, and a 1 px inset hairline silhouette. Browser layout quantization is preserved. Title and right control caps share the actual transparent U gap; glyph ink bounds no longer determine font size. Invented root/canvas borders were removed.

The renderer holds a canonical 1600×900 CSS viewport and exports at device scale, preserving the UI's layout and non-scaling SVG stroke rules. A possible one-pixel output-height rounding discrepancy is normalized only after rasterization. Real CSS hover, active-hover and ancestor behavior apply. The Tree uses the actual portaled popup composition, so only the hovered branch emphasizes and siblings keep their normal paint. The cursor image is captured above all popup layers from the actual computed CSS asset and hotspot.

Click events still commit before fullscreen/restore motion or equipment activation. Held presses retain their state. Shared overview fixed points remount the same actual UI and wait for current font/silhouette geometry; no copied boundary pixels or relaxed identity checks hide rendering differences. Native authored content retains its own scientific geometry, using shared canonical metrics and source-font outlines for its UI controls. Fused native controls preserve the silhouette's border ownership.

## Review Deliverable

[exact-ui.mp4](./exact-ui.mp4) is the retained and opened 12.375 second 1600×900 review film at 8 fps. It demonstrates navbar/Tree/footer hover, fullscreen and restore clicks, and a click-activated heater. All 99 frames decode, the first/last overview and tour/lesson handoff pixels are byte-identical, and an independent pixel/event check proves heat starts only after the committed click. See [exact-ui-render.md](./exact-ui-render.md) for final hashes and [exact-ui.png](./exact-ui.png) for the overview.

This is a focused UI review. The complete curriculum has not been regenerated; the previous full physical preview remains separate. The production renderer and all five lesson wrappers use the new actual-UI path.

## Validation

- Nine geometry/browser/cache tests and the separately registered Python lifecycle bridge pass; the bridge executes all eight portable JavaScript lifecycle cases.
- All five encoded lesson regressions preserve their 24 frames, original source-frame order, six-second audio duration, 440 Hz narration surrogate and silent transition padding. All ten decoded overview boundaries are identical.
- Actual rendered pointer, navbar/footer, Tree and stateful popup checks pass. The popup's unhovered siblings retain byte-identical pixels.
- Five native authored-control checks pass, including no duplicate cap border, camera offsets, exact callback order and held paint.
- Independent tests use the real UI in a separately authored browser fixture, plus portable vectors, Pillow, SVG parsing and font/Canvas metrics. Production geometry and sampled pixels match the pinned browser oracle exactly. Native continuous outlines have their documented hinted-raster tolerance.
- Cold-start provisioning is automatic through the existing registered Playwright tool. Missing executables provision once and retry once; unrelated failures do not install. Both real Playwright and real registered-tool smoke checks pass without cache deletion or new downloads.

The new Python and JavaScript suites are registered in the normal authoring test command. See [exact-ui-tests.md](./exact-ui-tests.md),[browser-ui-implementation.md](./browser-ui-implementation.md),[exact-ui-final-audit.md](./exact-ui-final-audit.md) and [exact-ui-port-audit.md](./exact-ui-port-audit.md).

## Infrastructure and Ownership

The browser uses the repository tool interface, with existing dependencies and no system-browser fallback. The compiler's complete source closure is discovered before certification. Known input contents and actual candidate-file membership both invalidate the closure cache, including a new candidate arriving during discovery. Existing source/candidate publication guards remain in place. Owned adapter lifetimes, progress, cancellation and cleanup cover errors before browser launch as well as completed frames.

No framework UI component, AGENTS.md, Git state, worktree or package dependency was changed. Concurrent authored content and audience work was preserved. The maximum four-slot fleet used GPT 6.1 Sol high execution agents and multiple low read-only audits. Reproducible input scripts and reports remain; owned generated diagnostics are removed at closure.

## Closure

Removed the verified owned generated directory: 387 temporary files, 12048870 bytes. The longest generated path was 165 UTF-16 code units; all 52 owned permanent paths are at most 125, below the 256-character limit. All owned test/render processes had exited. Intended MP4/PNG deliverables, reports and reproducible input scripts remain. The shared Python environment and registered pinned browser cache were preserved.
