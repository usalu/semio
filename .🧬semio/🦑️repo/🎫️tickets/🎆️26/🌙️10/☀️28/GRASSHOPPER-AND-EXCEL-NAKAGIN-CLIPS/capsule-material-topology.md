# Capsule Material Topology

The final shared-model gate found four incident faces on a welded shell-back edge. The former rectangular door decomposition extruded independent clipped strips; welding their endpoints retained internal partition faces. This is a material-topology defect even though Rhino Mesh.IsClosed was true.

The corrected rounded panel constructs the actual planar material union: clipped convex cap regions share subdivided boundary edges, while only the exterior union boundary receives depth faces. The sharp/sub-skin panel uses one annular loft around its interior rectangular aperture. Neither correction changes the physical casing extrema, doorway interval, poses or authored floor datum.

The portable casing fixture now requires exactly two incident material faces per edge at radii 0, 0.0005, 0.0012, 0.125 and 0.25 in full and exterior assemblies for all six physical families. NumPy independently counts the actual indexed edges; existing Rhino closure, positive-volume, ray-aperture and rounded-body containment checks remain enabled.

## Verification Chronology

- Initial shared gate: 59 tests, two manifold failures on shell-back; all other 57 passed.
- Original two failing tests after the rounded union correction: PASS, 13.869 seconds.
- New manifold regression exposed the same defect at radius zero before the sharp annular-loft correction.
- The stronger portable manifold and containment tests passed in 25.564 seconds. Directed-edge balance then independently exposed a reversed union boundary winding at radius 0.125; that one-line correction passed the full directed-edge regression in 14.497 seconds.
- Shared 59 tests passed in 84.886 seconds before that winding-only correction. The combined runner then stopped on Windows cp1252 progress output; explicit UTF-8 is now configured. The final 132-test integrated physical gate subsequently passed, including shared 59, casing 22 and circulation 18. All physical semantic start/end/current inputs match; renderer-only drift remains explicitly excluded from physical acceptance. Native replay remains on hold.

A first LF-only replacement anchor against the CRLF source briefly omitted the unchanged file tail; its preserved source tail was restored before the corrected-source runs. Those transient NameError runs are excluded, not counted as geometry evidence.
