# Paired Runtime 21

## Artifact and Environment

Observed September 27, 2026, beginning 20:01 UTC through the in-app browser at 1280×720. React runs at port6013 and WGPU at port6014 with HMR disabled. WGPU21 completed its canonical WASM build, prepare and activation. React12 passes2468 tests across142 files with zero skips. React package typecheck also passes.

The pre-journey artifact seal contains12 paths. The8 paths shared with the earlier React seal are byte-identical. The Puzzle core WASM is122549589 bytes, SHA256 `9c8e4d391b146130d863e82573451cde34f84085a56d5890c84b845d7e4a9821`; source-content receipt digests match. Aggregate activation artifact digests differ, so this is not a claim of identical activation receipts. Both browsers load the shared component root. Recheck file bytes after the physical journey before rebuilding.

## First Paint and Projection Comparison

Both browsers paint the Concrete Forest model and reference image. No GPU validation or console error was observed at first paint. React warns about other unactivated plugin modules; Puzzle is available.

At the default Top/Perspective split, the Top reference image spans approximately x70–379 in WGPU and x27–335 in React, with near-equal309px widths. The main object is centered near x214 in both. Orthographic grid spacing is approximately124px in WGPU versus62px in React. The perspective model bounds are similar, but the reference plane transform, outlines and shading differ.

The WGPU Projection menu paints narrow individual glass button rows around100px wide. React uses a300px panel with full-width Tree rows and trailing icons and chevrons. Native accessibility exposes raw owner-prefixed row names rather than the meaningful Tree semantics. Sol Canvas owns the retained-pane repair.

Selecting Curvilinear in the Top window changes the title and visibly activates the WGPU distortion pass without a console error. After settling, WGPU centers the model while React shows a larger model towards the upper-right. This is a current visual failure, not an animation-only observation. Sol Flow and Terra are tracing camera fit, projection switching, reference transforms and grid policy.

The first paint also shows remaining typography, icon order, border and spacing differences in global/window chrome. These are acceptance gaps and will require the same controlled comparison after targeted repairs.

## Window Journey

Focus enlarges the WGPU Curvilinear window to the full dock and retains its title. At the wide aspect ratio the distortion shows a central circle, a thick black annulus and repeated reference image at the outer edges. React's same selected mode and focus action instead show a large object at the upper-right with a much less distorted reference image. Sol Flow is investigating the projection/camera semantics and invalid remap domain.

Unfocus restores the WGPU split and both titles, but its untouched Perspective sibling loses its visible model; only a closer reference plane remains. The sibling briefly carries the generic Puzzle icon, then returns to the Perspective icon after the next layout change. This remains true after settling, with no warning/error console entries. Sol Canvas owns the native hidden-sibling lifecycle repair. React's matching focus/unfocus retains both titles and a visible model in the Perspective sibling.

Closing Curvilinear leaves Perspective occupying the full dock. Closing the remaining Perspective removes both the tab and world body, leaving an empty dock. React shows an empty-dock instruction; WGPU paints an empty background. Both close actions pass their visible postconditions.

Display→Windows→Puzzle3D disclosure exposes template rows in both renderers. Dragging the actual Perspective row transfer handle into the empty dock creates a world window. Dragging Parallel to the right edge creates a second world window. WGPU's new bodies eventually paint their model/reference after asynchronous publication. Moving the split from x640 to x760 moves the visible boundary by120px; the matching React action produces59.449% split. Close-last, reopen, split and resize pass these limited visible postconditions.

Template titles fail parity: WGPU labels both new windows `Puzzle 3D`; React names the same Perspective and Parallel templates `3-Point` and `Orthographic`. Root owns this native title propagation repair. The Display panel also reverses the visible child order: WGPU Perspective/Parallel/Puzzle3D versus React Puzzle3D/Parallel/Perspective. It uses a folder icon for the group where React uses the app icon and uppercase group caption.

Window Options opens a300px pane in both renderers with the same visible controls and values. Placement and overall extent are close, but the native controls differ: checked boxes receive wide red selected-row outlines, Select placeholder/chevron alignment differs, numeric readout/track positions and text/icon details differ. Native accessibility announces raw Select values (`threePoint`, `selected`, `outwards`) while the visible rendered labels are localized. No WGPU warning/error or React error was captured during the complete journey.

All12 sealed artifact files remain byte-identical after this journey. The mismatches therefore belong to this fixed paired runtime observation rather than a mid-journey artifact replacement. No full visual-parity claim is made.

## Current Automated Boundaries

UI10 executed715 tests:712 pass,3 fail,0 skip. Two tooltip lifecycle laws and one encoded world pipeline count failed; all three now have source repairs awaiting verification. Native9 exits1 before assertions on a NonEmptyVec `to_vec` fixture call. These failures are separate from the current successful browser artifact publication.
