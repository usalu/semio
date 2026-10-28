# Actual Browser UI Renderer

The production presentation port composes actual semio React components and captures their CSS in repository-managed Chromium. It does not replace the font renderer, silhouette, glass compositor, navbar, footer, popup, or tree.

## Production Interface

Sources: presentation/🌐️browser/🟨️render.mjs, 🟦️.tsx and 🧬️schema.json.

The semio-presentation-browser/v1 protocol carries explicit language, requested output size, an owned work directory and a finite list of controlled frames. One process bundles once, opens one browser/page and captures the batch. Each frame supplies window rectangles and titles, fullscreen/opacity/enabled state, optional authored body images, cursor point and press state, and navigation state. CLI --input and --output support the repository runtime.

The browser tool is loaded through the existing loadPlaywrightV1 interface. Existing esbuild and Tailwind dependencies generate the actual component bundle and stylesheet; no dependency was added. The browser always lays out CSS 1600 by 900 without a scene transform. Output resolution uses deviceScaleFactor = requested width / 1600; DOM metadata and pointer coordinates remain canonical CSS pixels. The Python boundary normalizes a possible one-pixel export-height difference.

## Actual UI Composition

Windows use actual WindowChrome, windowChromeTitleChipClass, windowChromeControlButtonClass, source icon SVGs and the actual active prop. Navbar, Footer and SemioLogo are source components. Domain thumbnail SVGs and authored video stills live inside the real clipped content plane; authored images use the actual base color for letterboxing.

Energy navigation uses controlled Popover, PopoverTrigger asChild, actual PopoverContent and Tree. PopoverContent owns its real portal, positioning, glass, border and padding. The popup is outside navbar DOM ancestry, so navbar hover selectors cannot recolor every menu row. The actual Tree hover path emphasizes the hovered leaf and owner branch while preserving sibling paint. Logical open state remains controlled by the replay model.

The visible capture cursor is a body portal with fixed positioning and maximum stacking order, like the operating-system cursor above popups. After a real mouse event, the renderer reads the actual computed CSS cursor URL and hotspot at the DOM target and displays that source SVG at intrinsic dimensions. It does not guess cursor type from a role or add cursor classes to change source UI behavior.

## Readiness and Time

Every capture waits for bundled fonts, decoded images, visible requested popups and current silhouette geometry. Painted border paths are compared with freshly measured actual WindowSilhouette geometry. Expected content clips round-trip through CSSStyleDeclaration to compare Chromium serialization exactly.

Canonical overview fixed points warm fonts and recreate the actual component tree before capture, removing retained opacity/compositor history. Remounting is restricted to an explicit canonical_overview flag or a settled original overview at the parked cursor, with no press, menu or source image. Consecutive held-press frames retain their actual component and pointer state.

Actual browser animations are paused and sought to transition_elapsed, defaulting to the settled endpoint. Screenshots allow these already-paused animations; disabling animations would incorrectly fast-forward a requested intermediate state.

## Source Identity and Cleanup

A --sources query discovers the input closure before rendering. It exports esbuild inputs, Tailwind dependencies and candidate files, referenced font assets, cursor/icon SVGs, contracts and dependency locks. Both query and render results export scan_sources, the exact scanner roots/patterns/negations needed to detect newly added candidate files. The current closure has 2515 existing files.

An outer try/finally covers writing the compiled private tool adapter and every subsequent load, CSS compile, UI bundle, input validation and browser stage. Browser resources have their own finally cleanup. A pre-browser input rejection after adapter compilation was verified to leave zero new compiled adapter files.

## Validation

- Production 400 by 300 WindowChrome with title 1.1 Luft exactly matches the independent actual-UI oracle: title 41.53125 by 22.390625; control x 427.609375 and size 22.390625; identical content/border paths; font and line height 11.2px; padding 3.2px; identical glass CSS.
- A five-frame overview/intermediate/fullscreen/intermediate/overview batch at DPR 0.245 has byte-identical first/last raw RGB: zero changed channels.
- Actual Popover menu stays at its source background while hovering a leaf. Only that leaf and Energy owner brighten; unhovered siblings retain their original muted foreground. The cursor remains visibly above the popup.
- Actual paused border transitions at 0, 60 and 120 milliseconds yield metadata RGB (123,130,125), (222,221,207) and (247,243,227). Captured border pixels differ accordingly, confirming screenshots preserve the requested time.
- Two consecutive held-press frames with all five original window rectangles retain the source active-hover appearance and have byte-identical raw PNGs. Release returns the control to its actual hover appearance.
- The owned test agent verifies the permanent protocol and popup regressions; the root agent owns the final composed video and ticket lifecycle.

All temporary PNGs, protocol inputs, reports and logs are under the ticket generated directory. This implementation report and production sources remain.

## Zero-Touch Browser Provisioning

The private 🧰️tools.mjs lifecycle helper now launches the pinned browser normally and provisions only when the first launch reports the standard Playwright missing-executable error. Provisioning calls the existing registered playwrightV1 interface with install chromium and retries once. Unrelated launch errors, installer failures and retry failures propagate without changing their identity or starting another install. No system browser or manual setup fallback is used.

Provisioning inherits the existing rendering capacity grant through heavy:false, matching the tool registry inherit policy, and forwards the backend AbortController signal to the registered owned process runner. SIGINT/SIGTERM therefore cancel the installer process tree. Installer output goes to stderr so the protocol keeps one JSON result on stdout; DEBUG progress marks provisioning. The helper is an explicit source-closure input before certification.

The portable lifecycle fixture was demonstrated RED with the missing module, then all eight cases passed after implementation: healthy launch, missing executable followed by recovery, unrelated error, missing-like unrelated error, installer failure, retry failure, cancellation before launch and cancellation during provisioning. The real Playwright cold-cache smoke is owned by the test agent and uses the existing pinned browser for its retry, with no new download.

The real Playwright smoke passed in both modes. An owned isolated empty cache produced the actual standard missing-executable error, the strict detector matched it, and mock provisioning switched the single retry to the existing pinned executable (4.10 seconds). A second smoke exercised the actual registered playwrightV1 install chromium call as a no-op against the already installed pinned cache, including inherited capacity, AbortSignal, stderr routing and running/finished progress, then launched the healthy retry (3.98 seconds). No download or cache deletion was needed. All eight lifecycle fixture cases and spies remain green; source is frozen.
