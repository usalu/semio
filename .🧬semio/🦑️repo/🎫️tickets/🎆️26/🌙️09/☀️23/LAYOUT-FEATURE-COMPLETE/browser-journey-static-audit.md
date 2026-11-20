# Browser Journey Static Audit

The default example ID `demo` and optional `linked-asset` exist in the Layout example registry. The stock story text, page row key pattern, linked asset catalogue key pattern, addFrame and exportSvg action names agree with authored Layout source. The WGPU browser host publishes the canvas ID and accessibility mirror attribute used by the journey; the action surface and pane IDs also appear in the current renderer.

The snapshot tab filter `/^dock\.tab\.[^.]*\.[^.]+$/` rejects any window ID containing a dot. This is fragile: the authored Blueprint window kind is `layout-blueprint`, while its canvas surface ID is `layout.play.blueprint`; those are distinct. Confirm actual tab IDs before concluding this prevents boot. To support arbitrary dotted window IDs, allow a nonempty remainder and retain the current split-and-join extraction.

No browser was run; dynamic placement, downloads, archive import, exact IDs, and current server port remain to be verified by the live journey.
