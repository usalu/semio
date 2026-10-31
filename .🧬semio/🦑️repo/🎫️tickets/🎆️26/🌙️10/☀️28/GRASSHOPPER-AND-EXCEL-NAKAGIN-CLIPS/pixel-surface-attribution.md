# Independent Changed-Pixel Source Attribution

**Snapshot scope:** the 2,253-Item export and controls precede the later authorized conformal panel correction. This is retained relative-culling evidence for that exact immutable source/raster snapshot, not final-source visual acceptance. The physical missing-corner defect is separately documented and corrected under panel-coverage-plan.md.

The ticket-local oracle uses the actual 2,253 source Items (442 retained), preserving every separately exported world face. It independently ear-clips concave faces with the existing circulation oracle, fits the world-to-pixel transform with NumPy least squares, obtains the parallel ray direction from its nullspace and orients it using the explicit camera metadata. It imports no renderer. The maximum fitted projection residual is 2.913225216616411e-13 pixels.

The final-source comparison queries 3,568 camera-facing triangles at every changed pixel centre and a 5 × 5 interior subpixel grid. There are 464 distinct changed pixels across the nine controls: **11,600 sampled rays**. Transparent layers remain contributors until opaque material; coplanar source ties are preserved rather than assigned arbitrarily.

**No sampled ray loses a visible removed contributor, changes its nearest physical depth, or changes its ideal retained painter-order contributor.** The AA-disabled material, silhouette and source-ID controls independently have zero baseline/stable/reordered differences at 256 pixels, and material/silhouette also have zero differences at 1,024 pixels. This supports physical coverage under the tested camera; finite sampling is not an exhaustive AA proof or a colour-tolerance waiver.

| Mode | Stage | Changed pixels | Channels | Visible removal | Depth change | Point-order change |
|---|---|---:|---:|---:|---:|---:|
| aa-fill | removal | 194 | 578 | 0 | 0 | 0 |
| aa-fill | retained-order | 14 | 32 | 0 | 0 | 0 |
| aa-fill | total | 208 | 610 | 0 | 0 | 0 |
| aa-strokes | removal | 220 | 636 | 0 | 0 | 0 |
| aa-strokes | retained-order | 36 | 86 | 0 | 0 | 0 |
| aa-strokes | total | 254 | 719 | 0 | 0 | 0 |
| material | removal | 348 | 885 | 0 | 0 | 0 |
| material | retained-order | 97 | 231 | 0 | 0 | 0 |
| material | total | 433 | 1092 | 0 | 0 | 0 |

The 194 AA-fill removal pixels split into 167 opaque-facet-join hidden-underpaint candidates and 27 sampled-hidden-underpaint pixels. Both classifications require that removed material lies behind opaque physical contributors; the first also spans multiple nearest source facets within the pixel. Retained-order AA drift occurs without any ideal point-order change in the sampled rays. Exact Cairo AA composition remains GH's responsibility; no higher arbitrary colour threshold is proposed.

## Retained Baseline/Candidate Boundary Discrepancies

Five absolute Cairo-ID versus mathematical centre-ray discrepancies are identical in full, stable and culled controls. Both the Cairo source and nearest mathematical source are retained in every case. They do not identify a culling regression. Exact exported-edge distances and extrapolated source-plane depth differences are preserved below. These are measurements, not a claimed Cairo quantization model or depth-tolerance waiver.

| Pixel X,Y | Cairo / ray source ID | Cairo-source edge distance px | Extrapolated depth minus ray depth m | Status |
|---|---|---:|---:|---|
| 52, 84 | 20 / 11 | 0.001436541 | 1.401576e-5 | retained in both |
| 99, 107 | 21 / 20 | 0.004429403 | 6.713363e-5 | retained in both |
| 102, 191 | 11 / 17 | 0.001535674 | 2.045459e-5 | retained in both |
| 128, 128 | 77 / 208 | 0.009040443 | -1.045792e-3 | retained in both |
| 198, 139 | 0 / 224 | 0.003839811 | -5.817569e-14 | retained in both |

At (198,139), the front skin and window rim extrapolate to the same physical plane. At (99,107), both IDs belong to adjacent facets of shell-edge-y-right-top-0. Other pairs are flat/rolled or rolled/spherical casing joins. The largest extrapolated difference is approximately 1.046 mm at the rolled/spherical corner, within a 0.018-pixel source-edge neighbourhood; its renderer-ID selection is unchanged by culling. The measurements are independently cross-checked by a direct JavaScript segment/plane calculation; the final NumPy replay passed with exact input endpoints unchanged, and its independent boundary witnesses agree with the JavaScript cross-check.

## Input Identity And Verification

The original completed result is retained at generated/pixel-surface-attribution.json with exact source/image/script endpoint hashes and unchanged=true. The renderer worker confirms that the later metadata export added only camera_facing and coverage-camera.json while model/data/renderer and Item generation remained unchanged. The old pre-metadata JSON blob was not retained, so this is explicit edit/snapshot provenance, not a historical blob-equality claim. Current source and image byte hashes are bound in pixel-input-attestation.json.

Portable cases preceded implementation. Three independent ray tests passed in 0.834 seconds: hidden/visible removal, concave opening, disconnected faces, transparent foreground, sloped-depth crossing against an independent NumPy plane solve, and exact centre/subpixel coordinates. Strict AJV accepts the exact source export and seven portable geometries and rejects three malformed exports. The final replay passes all four portable tests, including exact boundary distance/depth witnesses, and retains unchanged source/raster/script/oracle endpoint hashes. Progress/cancellation callbacks are available; no physical source/test or production renderer has been edited by this work.

## Preserved Input Snapshot

The exact pre-panel-correction source export, camera and all 18 hash-matched 256-pixel rasters are archived in generated/coverage-before-panel, together with the ray result, hash attestation and preservation manifest. All 20 primary source/camera/raster bytes match the recorded endpoint hashes. The 1,024-pixel controls and control summary are also retained. GH confirmed that no new capture overwrote this snapshot; subsequent captures use a distinct coverage-after-panel output.
