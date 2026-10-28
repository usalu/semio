# Current Chapter Test Contracts

Root updated existing geometry tests to select a canonical chapter window and all four card grids. The native media tests now isolate the finite chapter encoder from separately tested source/publication certificates, use context-sensitive opening/closing durations, and expect different outer frames for a chapter whose neighboring chapters share different ancestor overviews. Existing atomic publication, current-input, locale, crop, geometry, audio, resampling and edited-source decode coverage remains.

Twelve contract/media tests pass in73.717s. The actual native49-frame audible chapter test also passes complete body pixels, camera seams and audio padding. The larger10-case run retained one concurrent-discovery failure (fixed subsequently and independently rerun) and one camera test stopped by the correct changing-source handoff guard; only that affected current camera case needs rerun. No native production frame-count test was relaxed: the failed historical4089 publication remains rejected; current full4079body+41transition oracle decodes4120frames.

The camera rerun reached all trajectory/body/press/release assertions, then exposed an obsolete right_cap_rect assertion. It now checks control containment inside the title chip and zero right-cap controls. A Windows mapped-file write was rejected; the unchanged test run was stopped and the narrow edit applied atomically before the final rerun. No production file changed.

Final affected native camera test is GREEN in144.713s after the test-only title-chip containment correction. It passed all camera progress/scale/translation checks, exact full-body pixels, real pointer press/release/click sequence and fullscreen-control membership inside the name chip. No production change was needed.
