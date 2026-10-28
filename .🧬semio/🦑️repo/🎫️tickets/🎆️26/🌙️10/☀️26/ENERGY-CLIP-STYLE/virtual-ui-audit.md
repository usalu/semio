# Virtual Lesson UI Source Audit

Read-only inspection on 2026-10-26.

## Canonical UI Findings

Navbar and Footer share NavbarBandBody. They are base-level normal-flow horizontal bands; each uses standard padding and gaps, medium-height controls, cream foreground and shell floor fill. The canonical generated CSS gives navbar/footer 28.8 px, medium control 22.4 px and standard padding 3.2 px at its base scale. Do not invent thick theatrical header/footer strips; scale these values together for video legibility. Anta is the canonical sans face.

WindowChrome has title chips at top left, optional controls at top right, a transparent U-gap between them, and a continuous square/miter silhouette around the body. Only chip cells and body paint glass; the entire cap row must not be filled. Footer chips can attach at left, center or right. Body content is clipped to the silhouette. WindowSilhouette normalizes intervals and derives border, body and glass regions from one geometry, so presentation should similarly derive focus rectangles, borders and content clipping from one layout description.

## Existing Video Path

Each of the five lesson wrapper files delegates compose to runtime.compose_videos and main to runtime.video_main. video_main delegates to the canonical render CLI with --video; it does not execute a standalone Manim Scene. Therefore one runtime integration covers direct lesson scripts and the complete curriculum renderer.

Runtime currently admits exactly id/script/intro/chapters/output and inserts intro as a synthetic first chapter twice: segment_plan and compose_videos grouping. Remove intro from the manifest/schema and both grouping paths together; retire the old intro source instead of inserting a transparent legacy segment. Update segment identity/chapter indexes and cache state revisions because the plan changes.

All authored scenes use shared scene_title and beat_subtitle directly or via small local wrappers. Many position subtitles or later content relative to the returned title, and first beats animate titles while later beats add them instantly. A blind removal of calls or a bare empty VGroup breaks authored placement and can alter timing. Centralize the visual role in the fonts helper and runtime scene context: route chapter identity to focused window chrome and retain only a measured positioning anchor where authored content still needs it. Make this an explicit current presentation contract, not a no-op compatibility adapter. Remove title Write animation time from the canonical title helper. The new overview/focus transition belongs at lesson assembly boundaries, never per authored beat.

## Integration Recommendations

Build one deterministic overview with Navbar, canvas containing five fixed lesson windows, and Footer. Use canonical chapter titles as chips, modest body previews and primary active/focus treatment. Treat overview content and window positions as immutable during all five videos; current-lesson metadata, progress, hover, selection glow or clock must not change endpoint pixels.

Create the endpoint overview once at the chosen resolution/fps and reuse its encoded frame for every lesson start/end. A single renderer function is necessary but not sufficient: camera transforms, antialiasing, active state and animation interpolation must also finish identically. Native decoded first/last RGB equality is the appropriate acceptance check. Adjacent lesson endpoints should reference the same overview fingerprint.

Focus transitions should transform the canvas/window framing while Navbar/Footer remain screen anchored. Match the focused body aspect ratio to authored footage, and clip the footage to that body. Avoid scaling full video footage including its old heading area into the window; title suppression must happen in source presentation, not by accidentally cropping scientific diagrams or captions. Narration trace times must remain aligned if title animation duration is removed.

Include new presentation sources and manifest in runtime dependency/provenance state so cached pre-UI lessons cannot pass as current output. Review partial chapter rendering explicitly: it should remain useful authored chapter output or receive an intentional focused shell, without emitting the full lesson intro/outro for every chapter.

## Visual Review

Pending the first native virtual-UI render.

## Integration Follow-up Review

The settled runtime removes intro from the plan and routes complete lessons and selected chapters through publish_presentation. Its post-render guard checks both presentation state and canonical content state before writing successful provenance.

One actionable cache input gap was reported: presentation.source_files initially included the new module/contract/schema/theme but omitted the fonts and visuals implementations used to draw the native pixels, and their font assets. These must enter presentation provenance to prevent a reused stale envelope after typography or silhouette changes.

Transition alpha-zero frames call _canvas(None), independent of selected lesson, and outbound samples reverse the same transition pattern. Lossless H.264 encoding is configured. The structural path supports identical endpoint samples; native decoded RGB tests must verify the final encoded result.

Initial native overview and transparent focused shell reviewed at 960×540. The five lesson windows have clear primary fused title chips, consistent spacing and unclipped labels. Navbar and Footer remain screen anchored with no text overlap. The initial thumbnail opacity reset filled intended wireframe houses; the scene author independently identified and corrected this using relative fading. Regenerated final images and focused video content remain to be reviewed.

The cache input concern is resolved in current source: presentation provenance now includes runtime source dependencies for visuals, fonts and runtime, visual/silhouette contracts, canonical theme tokens, font catalog and bundled font files. Fresh corrected overview reviewed successfully: all five wireframe thumbnails, title chips and shell bands fit without overlap at 960×540.

## Final Native Authored Preview Review

Decoded the completed virtual-ui.mp4 with the independent PyAV library and inspected samples at 2.5, 6.8, 8 and 10.5 seconds. Physical content at focus, outgoing zoom, the common five-window boundary and ventilation content at focus all fit within the primary window silhouette. The diagram, quantity labels, source citation and captions remain visible and clear. Navbar and Footer stay fixed while the focused window grows or shrinks. The outgoing crossfade deliberately reveals the abstract thumbnail while authored content fades; no clipping, overflow or frame/chrome collision was found in these samples.

The preview uses settled authored clause frames for physical power and ventilation volume, rather than a complete lesson rerender. It validates shell geometry and containment; complete-video endpoint equality and audio/timing behavior are covered by the native regression suite. No actionable source or visual concerns remain in this independent audit.
