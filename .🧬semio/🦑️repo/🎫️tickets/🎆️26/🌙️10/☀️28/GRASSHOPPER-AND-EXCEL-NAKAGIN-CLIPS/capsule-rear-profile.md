# Rounded Capsule Rear Defect

## Verified Before Change

The integrated front view’s square corners were a body-geometry defect. _shell_parts created the front from a rounded outline but called rectangular _panel for the rear steel and rear lining. The rear body corners lay0.103553390593m outside the analytic nominal r=.25m rounded envelope. An independent Pillow orthographic projection of actual body faces filled all four corner pixels that must be empty. Attachments, bolts, door hardware and service rails were excluded from this envelope check.

Native evidence used actual physical GLB triangle positions, not wireframe or inflated Brep control bounds. The Slash native asset contains1474 physical vertex entries, bounds X±1.30m/Y±2.10m/Z−.20..2.30m. Its constant rounded-body profile is2.60m wide,2.50m high,centreZ1.05m,r=.25m. Maximum signed distance over every physical vertex is1.006e−7m;87 rear-surface vertices atY<−2.09m fit strictly inside that profile (max−.18121m). The rear is rounded inward in the native physical surface as well. This confirms the rectangular rear was not a legitimate source component. These native dimensions are evidence for this local comparison, not a global change to the nominal teaching dimensions.

## Actual Geometry Fix

_profile_panel extrudes the same convex rounded outline used by the front. Back-entry families subtract a real rectangular opening through disjoint closed left/right/top/bottom material strips, clipped against that outline; degenerate polygon points/zero-area strips are removed. Side-entry families receive a full rounded rear. Both steel and lining use this construction. No renderer mask, silhouette overlay or fake triangle fan spanning the door was added. The global _panel helper and core/podium/pose geometry are untouched.

## Test-Driven Verification

The portable detail body_profile cases and two regressions were written first. Both failed against the original source: positive rounded-envelope SDF on shell-back and filled corner pixels. After the physical geometry change, nine dedicated tests pass in6.358s. New checks include:

- NumPy analytic rounded-rectangle signed distance for every actual shell/shell_back vertex in all six families.
- Pillow512px orthographic projection of actual body faces: four outside corner pixels empty and four cardinal edge pixels occupied.
- Rhino valid/closed rear meshes and NumPy positive signed material volume in every family.
- Independent solid-angle containment proves actual back/side doorway centres are empty and surrounding shoulders contain material, for all six families.

Existing sanitary cavities, control/hardware meshes, positive volumes and study separation remain green. Existing six-family/window/portable-part regressions are rerun separately on this snapshot. The fix only removes rear material outside the existing conservative capsule envelope; global dimensions, capsule poses and source datum fields are unchanged.



The three affected existing assembly regressions also pass (9.552s): all-six-family positive closed parts/window wall, portable capsule part contract, and real window aperture.
