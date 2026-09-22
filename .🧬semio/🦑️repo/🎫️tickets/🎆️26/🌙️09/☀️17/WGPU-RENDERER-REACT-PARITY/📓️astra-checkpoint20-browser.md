# Checkpoint 20 Browser Verification

The authored-compositor wasm build completed in14m44s. Canonical prepare and activation succeeded. Both renderer activation receipts identify Puzzle component `1db04e39cf7c07e23ba875a17ea6e44d76b6de72669999fb170aa32ca87d9d1f`. Fresh paired tabs4/5 both measure1600×1000. Browser evidence is under `🗑️generated/astra-runtime/checkpoint20-iab/`; the physical journey file retains snapshots and actual consoles beside screenshots.

## Observed Behavior

Fresh boot rendered both Puzzle windows. Physical Display footer toggle and Settings open/General/close all responded. General now exposes intrinsic accessible labels, including German Darstellung, Sprache, and Terminologie. The three-tab Settings width limit remains visible in this built artifact; its repair is separate ongoing work.

Window Options now occludes the World correctly in the captured popup region: the former scene-over-popup compositing failure is absent in this screenshot. Its structure still differs materially from React: WGPU uses tall, left-aligned stacked groups and full-width toggle controls, while React renders a compact right-anchored tree. Locale and value differences remain, including Sun intensity1 versus0.85 and a Spacing accessibility value10.5 for the displayed10.

The first Top-close click immediately following popup closure did not remove the window. A separate physical click after the popup retirement did remove Top, refocused Perspective, and repainted its expanded viewport. This first rapid sequence is not counted as accepted. Closing the final Perspective window at18:44:53.880Z removed its scene and controls and produced an empty dock, with no new browser console error at the subsequent capture. Display then reopened and published its tabs while the dock remained empty. The console remained free of new errors47.274 seconds after the final close. This confirms continued responsiveness beyond the earlier stall interval.

Semantic locator clicks on WGPU mirror switches returned successfully but did not change state; physical pointer clicks worked. A source audit is distinguishing mirror pointer semantics from real accessibility activation behavior. No accessibility activation acceptance is claimed from these locator calls.

The full57-step, eight docking cases, and representative surface-family acceptance remain incomplete. Goal and ticket remain open.
# Late Failure: Prepared GPU Commands

At 2026-09-21T19:40:07.118Z, browser20 reported a new renderer quarantine: `prepared frame submit step: prepared GPU opportunity exceeded the two millisecond ceiling for 4 consecutive opportunities: Commands took 3100 us`. A fresh DOM inspection confirmed `input accepted: no`. This differs from the older 17:42 InteractionCheckout fault. The final window had closed at 18:44:53.880Z, about 55 minutes earlier; the earlier successful interaction at +47.274 seconds remains only that bounded observation.

Evidence was captured with the browser skill as `🗑️generated/astra-runtime/checkpoint20-iab/13-late-post-close-quarantine*`, including current DOM, screenshots, and console logs. Browser20 predates the source change that limits one GPU clip piece per opportunity. Sol and Terra are determining whether that repair covers this Commands failure or whether a separate timing/progress problem exists. No threshold increase or new browser acceptance is claimed.
