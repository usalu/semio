# Content Camera Audit

The presentation uses the shared LayeredOverview, WindowChrome, Navbar, Footer and real Playwright pointer events. Keep native controls and screenshot acquisition; move only the visual camera after the real video-open click settles. Reverse the camera before the real restore click.

## Exact Mapping

Canonical screenshot: 1600 × 900. Focus content viewport: x 6.4, y 57.6, width 1587.2, height 807.2. Its aspect ratio differs from 16:9. VideoPane uses object-fit contain, so an authored 16:9 frame occupies x 82.4888889, y 57.6, width 1435.0222222, height 807.2.

A uniform camera zoom targeting that image ink rectangle reaches a full 1600 × 900 image without stretching or retained bars. Use scale 900 / 807.2, transform origin 0 0, and translation negative source x/y times scale. A camera targeting the entire viewport retains its horizontal contain bars; independent x/y scale distorts the source. Compute from measured DOM image ink bounds where practical.

## Continuity

Retain the first decoded source image throughout focus UI and inward camera transition; land on its fullframe equivalent. Then play the original authored source directly at output size. Retain its final decoded frame throughout outward camera transition; land on the same open-window state before native restore and parent navigation. Do not fit the original source through the focus viewport again during the content segment.

Current composition scales/pads source into viewport and overlays shell. Remove that middle-shell stage for fullframe authored playback. FocusHold currently lasts only 0.2 seconds after video-open release; explicitly allow the native open transition to settle before the new camera begins. The browser renderer advances actual animations before screenshots, and pointer events must remain in canonical source coordinates.

The source pixel interval and boundary still extraction must agree. Avoid a temporal jump caused by sampling an arbitrary last-second seek frame; use the exact final decoded frame corresponding to the body interval. Rounding should occur only at output encode, rather than before geometric camera calculation.

## Verification

Check native click metadata still reports topic.open/video.open/video.close/topic.close and actual restore target. Inspect camera start, midpoint and endpoint; assert endpoint corners fill output and source aspect ratio remains 16:9. Compare inward endpoint with first body frame and final body frame with outward start. Keep canonical root first/last frames identical. Visual preview audit awaits generated output.

## Final Source Review

The new measured image-ink camera uses correct uniform affine interpolation. Its browser reset runs before pointer dispatch, preserving canonical native hit coordinates. Restore adds the complete camera interval before native cursor travel/press. Focus starts camera at 3.0 seconds after video-open release at 2.8 seconds; the preview should confirm the shared strip has settled within that 0.2-second hold, since silhouette settling alone does not certify strip position. Endpoint direct copying of normalized snapshots ensures exact raster handoff.

A complete Python search under teaching found no production callers of render_shell after body composition switched to fullframe. Remaining shell path is obsolete: presentation.render_shell wrapper, hierarchy-render.shell, shell request cursor suppression and transparent body erasure branch. The silent-source test still defined and patched a shell mock at lines 365–370 during this audit. Asked the UI implementation agent to remove the dead API and coordinate stale mock cleanup, retaining scientific source cursor/ancestor-control helpers.

## Final Preview Visual Audit

Inspected content-camera-zoom.png, content-camera-body.png and content-camera-return.png at their exported 1280 × 720 size. The inward sample shows the source expanding while the outer window cap clips beyond the frame; its blank body is the expected authored opening fade. The body sample fills the entire output with authored content: outer navbar, footer, window cap and restore control are absent. Its two title boxes belong to source content and remain correctly visible. No double viewport bars or visually distorted text/math were present. The return sample restores the actual navbar, footer, title cap and restore control around the source; the source remains proportionate within its native contained window. These three stills support the intended visible sequence. Temporal continuity and exact source order are covered separately by automated frame/audio tests.
