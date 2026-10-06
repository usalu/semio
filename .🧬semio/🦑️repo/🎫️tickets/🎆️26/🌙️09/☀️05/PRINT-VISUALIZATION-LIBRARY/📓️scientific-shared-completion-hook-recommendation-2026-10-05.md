# Scientific Shared Completion Hook Recommendation — 2026-10-05

Read-only source adjudication. Categories reflect reachable numeric projection calls, not domain names; mixed variant families must retain branch timing. Literal dynamic-alias limitations remain.

| Family | Existing coordinate owner | Source | Numeric window evidence |
|---|---|---|---|
| sci-3d | physical structural | semio-viz-scientific-3d.sty:269 | semio_viz_sci_window:nnnn |
| sci-pathway | physical structural | semio-viz-scientific-biology.sty:628 | semio_viz_sci_window:nnnn |
| sci-pedigree | physical structural | semio-viz-scientific-biology.sty:723 | semio_viz_sci_window:nnnn |
| sci-punnett | physical structural | semio-viz-scientific-biology.sty:837 | semio_viz_sci_window:nnnn |
| sci-lifecycle | physical structural | semio-viz-scientific-biology.sty:907 | semio_viz_sci_window:nnnn |
| sci-circos | physical structural | semio-viz-scientific-biology.sty:971 | semio_viz_sci_window:nnnn |
| sci-ideogram | physical structural | semio-viz-scientific-biology.sty:1130 | semio_viz_sci_window:nnnn |
| sci-seqlogo | physical structural | semio-viz-scientific-biology.sty:1501 | semio_viz_sci_window:nnnn |
| sci-contact-map | physical structural | semio-viz-scientific-biology.sty:1596 | semio_viz_sci_window:nnnn |
| sci-nomogram | physical structural | semio-viz-scientific-biology.sty:1807 | semio_viz_sci_window:nnnn |
| sci-schematic | physical structural | semio-viz-scientific-biology.sty:1908 | semio_viz_sci_window:nnnn |
| sci-molecule | physical structural | semio-viz-scientific-chemistry.sty:57 | semio_viz_sci_window:nnnn |
| sci-orbital | numeric projection / mixed | semio-viz-scientific-chemistry.sty:338 | semio_viz_sci_my:n, semio_viz_sci_window:nnnn, semio_viz_sci_window_keys: |
| sci-crystal | physical structural | semio-viz-scientific-chemistry.sty:776 | semio_viz_sci_window:nnnn |
| sci-circuit | physical structural | semio-viz-scientific-engineering.sty:51 | semio_viz_sci_window:nnnn |
| sci-smith | physical structural | semio-viz-scientific-engineering.sty:519 | semio_viz_sci_window:nnnn |
| sci-truss | physical structural | semio-viz-scientific-engineering.sty:933 | semio_viz_sci_window:nnnn |
| sci-dimensioned | physical structural | semio-viz-scientific-engineering.sty:1040 | semio_viz_sci_window:nnnn |
| sci-transform | physical structural | semio-viz-scientific-geometry.sty:423 | semio_viz_sci_window:nnnn |
| sci-tiling | physical structural | semio-viz-scientific-geometry.sty:481 | semio_viz_sci_window:nnnn |
| sci-fractal | physical structural | semio-viz-scientific-geometry.sty:708 | semio_viz_sci_window:nnnn |
| sci-mesh | physical structural | semio-viz-scientific-geometry.sty:852 | semio_viz_sci_window:nnnn |
| sci-pitch | numeric projection / mixed | semio-viz-scientific-geometry.sty:930 | semio_viz_sci_mx:n, semio_viz_sci_my:n, semio_viz_sci_window:nnnn |
| sci-bracket | physical structural | semio-viz-scientific-geometry.sty:1135 | semio_viz_sci_window:nnnn |
| sci-abstract-diagram | physical structural | semio-viz-scientific-mathematics.sty:1099 | semio_viz_sci_window:nnnn |
| sci-grid-diagram | physical structural | semio-viz-scientific-mathematics.sty:1139 | semio_viz_sci_window:nnnn |
| sci-braid | physical structural | semio-viz-scientific-mathematics.sty:1226 | semio_viz_sci_window:nnnn |
| sci-set | physical structural | semio-viz-scientific-mathematics.sty:1374 | semio_viz_sci_window:nnnn |
| sci-upset | physical structural | semio-viz-scientific-mathematics.sty:1503 | semio_viz_sci_window:nnnn |
| sci-probability-tree | physical structural | semio-viz-scientific-mathematics.sty:1767 | semio_viz_sci_window:nnnn |
| sci-simplex | physical structural | semio-viz-scientific-mathematics.sty:1939 | semio_viz_sci_window:nnnn |
| sci-transition | physical structural | semio-viz-scientific-mathematics.sty:2007 | semio_viz_sci_window:nnnn |
| sci-force | physical structural | semio-viz-scientific-physics.sty:87 | semio_viz_sci_window:nnnn |
| sci-feynman | physical structural | semio-viz-scientific-physics.sty:472 | semio_viz_sci_window:nnnn |
| sci-optics | physical structural | semio-viz-scientific-physics.sty:1179 | semio_viz_sci_window:nnnn |
| sci-poincare | physical structural | semio-viz-scientific-physics.sty:1274 | semio_viz_sci_window:nnnn |
| sci-staff | physical structural | semio-viz-scientific-signal.sty:461 | semio_viz_sci_window:nnnn |
| sci-tonnetz | physical structural | semio-viz-scientific-signal.sty:684 | semio_viz_sci_window:nnnn |

The 38 candidates divide into 36 physical structural renderers and two mixed numeric owners (`sci-orbital`, `sci-pitch`). The window:nnnn entry in every row includes shared reset's [0,1] fallback and is not evidence of a meaningful family window. Orbital reaches my and explicit window keys; pitch reaches mx/my through trajectory but hardcodes [0,1] without applying explicit keys. Physical examples directly multiply layout lengths, radii, indices or normalized positions into millimetres; their chart is not automatically governed by shared mx/my.

Architecture recommendation: retain completion ownership in semio-viz-scientific-field.sty, with a per-invocation pending/frame-drawn flag reset at sci_enter and set by sci_frame. A shared finish operation can draw missing chrome once using the finalized owner canvas/window, while leaving existing numeric owners' frame calls at their current location after data-dependent extent calculation. Do not move their frame into sci_enter: sampling and extent inference happen later. Do not apply window keys unconditionally after all drawing: that changes tick labels without changing projected geometry.

The existing family runner (semio-viz-family.sty:55) only invokes a renderer; a runner completion hook can finalize chrome only if scientific state survives local family groups and renderer scopes are balanced. It cannot itself make explicit domain/range affect coordinates already emitted. It also needs scoped invocation state so a nested family call does not consume an outer pending frame. This makes a blanket runner suffix insufficient for full customization. Prefer explicit shared prepare/finish calls at the scientific family boundary, near sci_enter and the end of each of the 38 existing owner bodies, with frame-drawn suppression for mixed branches. No new module is required. A registry runner hook may safely dispatch the already prepared completion operation after all direct entry paths are proven covered, but avoid changing non-scientific owners or adding a scientific package dependency to the generic registry.

For numeric projection branches, install explicit window keys after the owner has established its default/data extent and before first mx/my/polyline draw. For pitch this must precede surface and all positions/heat/pass/trajectory projections consistently, not just trajectory tick labels. Orbital's molecular-energy branch preserves its existing data window timing; physical shell/lobe branches need a declared physical viewport contract.

For physical structural owners, define the shared physical viewport in native millimetre coordinates and apply a drawing-scope affine mapping from that window to width/height before body paint. Explicit domain/range then change real geometry placement/cropping; default viewport must preserve current layout. Grid/axes/ticks use the same viewport and projected coordinates; titles and axis labels remain canvas chrome outside the transformed body. A finish-only frame that labels the reset [0,1] window while leaving body coordinates untouched is not completion. Some diagrams use negative centred coordinates or extend beyond width/height (sphere, Smith, force, mesh, bracket); use source-specific existing extents, not a blanket [0,width] x [0,height] guess. Capture the body bounds independently and test clipping/containment. Where explicit viewport is designed as cropping, clip body only with the declared window contract; avoid silently fitting data outside the supplied domain/range back inside.

Required neutral regression matrix: all 38 family owners in both English/light and German/dark, baseline versus title/xlabel/ylabel, axes/grid toggles, ticks 2 versus 5, independent explicit domain and range, width/height, and mixed numeric branches for orbital/pitch. Compare actual PDF body geometry and glyph positions with third-party extraction plus numerical affine oracle; title presence alone is insufficient. Include nested-family rendering and existing numeric data-dependent frames to prove once-only completion and no stale-state carryover. Test explicit-window geometry and tick labels together. Metadata correction must derive all family defaults from actual native overrides; the 91 earlier contradictions remain separate admission/default truth defects, not reasons to discard promised controls.

Additional branch-level defects directly traced while adjudicating (not compiler claims):

* sci-orbital molecular-orbital x placement in semio-viz-scientific-chemistry.sty:368–370 hardcodes field x in [-1,1] into 6+(x+1)*(width-12)/2 even though window_keys consumes domain. Range affects shared my; domain does not affect x. Therefore presence of window_keys alone does not prove both axes honored.
* sci-3d family always invokes semio_viz_space_axes at scientific-3d.sty:291; the helper at315 unconditionally draws three projected world-axis paths and never reads shared axes bool. Default axes=false still has the three world axes; toggling shared axes only would not alter existing world axes. Decide explicit world-axis versus viewport-axis contract and implement/test both promised effects consistently.
* Family-level frame reachability overapproximates all branches. sci-phase-diagram chemistry~461 has ternary -> chem_ternary with no frame, while only Cartesian branch invokes frame and window keys. All38 is a lower bound on affected variant branches; the other42 families need per-branch shared chrome reachability, not dismissal because one branch reaches frame.
