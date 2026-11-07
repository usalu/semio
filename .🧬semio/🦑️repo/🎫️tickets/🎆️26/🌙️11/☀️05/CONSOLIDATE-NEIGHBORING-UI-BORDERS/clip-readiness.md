# Clip Border Readiness

The architecture clip browser composes the shared window chrome. Before the compositor's first animation frame, a window paints its full silhouette, while clip readiness only checked geometry. The compositor now marks each root pending whenever it schedules paint and clears the marker after frame overlays and React window segments have committed. The browser capture waits for all compositor roots, including overview-only scenes.

The renderer now records every nonempty silhouette path and its state rather than reporting only the first path as a complete outline. A fully shared border can yield no paths; celebrated mask path color is its mask stroke rather than the gradient paint. The browser geometry oracle validates the segment collection. The renderer passes Node syntax validation.

Content camera transforms are applied after the initial readiness check. The renderer now schedules a resize to invalidate every compositor root after the transform, waits for paint again, and records the resulting border segments before capture. The earlier untransformed layout measurements remain available. Node syntax validation and the portable readiness cases pass; the zoomed browser oracle requires the ongoing dependency setup.

The production course overview places cards inside grid cells with double-spacing padding, so adjacent card outlines do not touch. A clip-specific seam oracle should target adjacent navbar/footer frames or another verified touching layout. A local Edge executable exists, but required workspace Playwright, esbuild, and teaching dependencies are not installed; a production screenshot requires the ongoing setup.

The synthetic seam harness uses actual adjacent WindowChrome instances at 1.25× scale, checks a unique winning SVG stroke for active and hover states, and checks raster color with Pillow. It must use the same Chromium provisioning behavior as the production renderer to work on a clean workspace; the tests agent was notified. Browser execution still awaits dependencies.

After the seam harness adopted the production browser provisioning helper, both renderer and seam scripts passed Node syntax checks, and all ten portable readiness cases passed. Cancellation after a browser has launched was identified for correction in the test harness.

The Manim authoring path also draws chrome: window_frame builds one polygon whose title chip and body already share an outline. The inspected internal-gains clip layouts place their separate frames with positive gaps (3.4-wide at 3.9 spacing; 2.15-wide at 3.8 spacing), so their outlines do not form shared seams. This is an inspection of those consumers, not an exhaustive rendered audit of every clip.

A broader source search over energy clip Python for direct zero-gap arrange/next_to expressions found text/diagram alignments and two heating wall-layer cross sections; the adjacent rectangles depict physical material layers rather than UI frames. No additional touching Manim UI seam was identified by that search. This does not cover dynamically computed spacing or substitute for a rendered audit.

The clip browser bundle aliases its window, card, navigation and footer directly to shared UI components and imports only the course chrome-and-home stylesheet. Other standalone course CSS borders occur in educational workbook and graph mockups, which are not imported by this clip renderer. No further demonstrable old touching border was established by this source audit.

Regeneration: the browser bundle records its esbuild inputs and CSS candidates; presentation.source_files includes that closure, and runtime.presentation_state hashes it before deciding whether a published presentation is reusable. The changed compositor and renderer therefore invalidate presentation and handoff identities on the next render. Rendered media is local, generated on demand; the default teaching cache was absent during this audit. The authored render entry supports explicit language, video and audience selection via repo tools exec bun -- <authoring script> render, and requires the ongoing dependency setup plus media rendering. No output was regenerated during this audit.

Example documented route (choose an appropriately short ticket-owned output root):

```text
repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --language de --video cooling --quality l --jobs 4 --output-root <ticket-generated-output> --no-play
```

Invalidation chain: render.mjs esbuild metafile and Tailwind candidate discovery → browser.source_files() stable source closure → presentation.source_files() → runtime.presentation_state() / clip_state() → publish_presentation() reusable_clip check; canonical overview handoffs additionally hash the browser closure via _handoff_identity(). No render was executed in this investigation.

The source audit was extended beyond energy to architecture clip paths. The optimization clip window_frame uses inspected in index and extrema are individual frames around content, and its cached chip frames delegate to the same energy authoring helper. No direct zero-gap arrange/next_to call was found in the optimization clip Python. This is a targeted source search, not proof about all dynamically positioned elements.

Beyond energy and optimization, the BIM concept-panel source uses rounded boxes for UML class diagrams, property tables and class-tree nodes. The inspected class-tree nodes have 0.58 vertical pitch versus 0.36 frame height and separate sibling allocations; these are diagram shapes rather than demonstrated adjoining UI. A direct zero-gap arrange/next_to search of remaining architecture clip Python found no matches. Dynamically positioned objects and rendered clips remain outside this source-only check.

Visual artifact availability check: both teaching output/cache environment overrides were unset, the default video cache was absent, and a bounded architecture tree scan found no mp4/webm files. The presentation contract places the focused window at y=32 below a 28.8 px navbar, leaving 3.2 px rather than an adjoining seam. No existing rendered course clip was available for visual inspection.

A targeted search of the course package and energy clip catalog found no checked-in YouTube, MP4 or WebM clip source suitable for verifying this checkout; the documented published course site is a separate deployed artifact, not evidence that the local compositor changes rendered correctly. No new screenshot was obtained.
