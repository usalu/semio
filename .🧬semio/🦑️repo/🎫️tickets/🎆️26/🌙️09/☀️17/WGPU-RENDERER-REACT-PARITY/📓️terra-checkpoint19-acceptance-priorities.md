# Checkpoint 19 acceptance priorities

## Scope

This is a short triage list derived only from the recorded checkpoint18 main and Dock8 reports/receipts. It makes no current-runtime claim. Each item requires the freshly activated paired checkpoint19 evidence to confirm or retire it.

## 1. Frame progression and responsive boot

Checkpoint18 recorded a Settings command reaching the exact `framework.settings` control, then no frame turn for about 43.64 seconds while generation 10 remained in `Uploads` ([`astra-sol-checkpoint18-frame-liveness.md`](📓️astra-sol-checkpoint18-frame-liveness.md)). This was visible as a shell that accepted input but did not update its pixels. The same recorded run also took roughly 190 seconds before the World bridge reached the first requestable GLB scene state; after ready, mesh publication was immediate ([`terra-checkpoint18-world-mesh-latency.md`](📓️terra-checkpoint18-world-mesh-latency.md)).

Fresh19 must establish the actual boot timestamp, then prove the smoke journey reaches tour dismissal, Settings, App, General, Drivers open/close, and Settings close within the existing 2,400 ms per-step settle. Preserve console timestamps and frame phase receipts. A timeout, a long silent generation gap, or a missing post-command screenshot is a runtime failure; do not extend waits.

## 2. Presented window layout and input must agree

At checkpoint18 step 12, the published controls and World reservation already described the Settings body while the screenshot still showed the preceding full-width World. The candidate NumberSteppers were live pointer targets before their pixels had been presented ([`astra-sol-checkpoint18-input-presentation-generation.md`](📓️astra-sol-checkpoint18-input-presentation-generation.md), [`astra-sol-checkpoint18-settings-presentation.md`](📓️astra-sol-checkpoint18-settings-presentation.md)). The evidence distinguishes this timing defect from scene/UI geometric overlap.

Fresh19 must compare one settled Settings step across screenshot, chrome hit registry, geometry, and dispatched action: the visible Settings body occupies the published right-side bounds; the World ends at that bound; a Settings control routes only after that frame is presented. Any logical/physical generation divergence remains a user-visible authority failure even if the eventual panel appears.

## 3. Dock interactions must promote beyond the boot census

The checkpoint18 WGPU Dock8 baseline failed all eight cases while React passed all eight. The recorded WGPU cases remained at boot generation 1 with 43 controls; tab drags never promoted and the template tree never appeared during the deadline ([`astra-sol-dock-interactions.md`](📓️astra-sol-dock-interactions.md), [`astra-checkpoint18-plan.md`](📓️astra-checkpoint18-plan.md)). This receipt predates later presentation and retirement work, so it is not proof of the current implementation.

After the smoke run is responsive, fresh19 must run the unchanged eight-case Dock probe and require each physical result: four split directions, merge, reorder, Escape cancellation, and template configuration. Each WGPU receipt needs a post-gesture chrome generation/action outcome and screenshot that differs from boot where the operation changes layout. A repeated generation-1 census or no template tree identifies a live window-interaction regression; it must not be classified from old source reports.

## Gate order

Run the smoke pair first as specified in [`astra-checkpoint19-runtime.md`](📓️astra-checkpoint19-runtime.md). Only a responsive, generation-consistent smoke result should proceed to Dock8 and then the complete 57-step journey. These three priorities are probes, not acceptance claims.
