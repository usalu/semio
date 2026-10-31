# Post-Panel Retained-Order Evidence

The independent oracle uses actual exported source triangles, NumPy camera reconstruction/rays and exact source-plane depths. Four portable tests pass before each query. There is no colour-threshold or antialiasing waiver.

## Current Narrow/Sparse 512 Snapshot

The completed coverage-overlap-512 source has 2,368 source Items and 373 retained. Source/raster/script endpoints are unchanged. All three antialiasing-disabled source-ID changes correct the full/stable baseline at their pixel centres:

| Pixel X,Y | Full/stable index | Candidate index | Independent nearest | Candidate closer by mm |
|---|---:|---:|---:|---:|
| 333, 363 | 425 | 319 | 319 | 12.945402 |
| 335, 364 | 425 | 319 | 319 | 30.900596 |
| 396, 278 | 305 | 278 | 278 | 0.240760 |

The first two corrected sources are window-frame index319 over window-reveal-closure425; the third is window-rim278 over window-frame305. Both sources actually intersect each centre ray. They are not coplanar numerical ties or AA-only differences. The full/stable baseline is therefore not a physical ground truth for these three pixels; its unchanged colour cannot be a requirement when it paints a farther surface.

The finite query includes all three centres and 5 × 5 subpixel samples, 75 rays total. Ideal point painter errors against independent nearest material: full 11, stable 11, candidate 0. Sampled geometry contributor removal and nearest-depth changes remain zero. This is bounded evidence for the three requested changed pixels, not an exhaustive whole-scene claim. Other renderer sentinel/performance gates remain GH-owned.

## Earlier After-Panel Ordering Failure

The preceding 2,368-source/470-retained snapshot had a genuine candidate regression at (174,182). Retained glazing340 at depth3.323537632989 m was 0.776801045 mm in front of frame318, yet recomputed painter order selected the frame. Full/stable correctly selected glass. An independent intersection of their actual projected triangles proved 0.547934471637 px² of positive overlap, with frame-minus-glass depth extrema −0.005345216611…approximately0 m across every overlap vertex. No sign reversal occurs, so one frame-before-glass dependency is physically justified. GH repaired that relationship before the newer 512 query.

Across that earlier 319-pixel/7,975-ray changed region, full/stable had15 wrong point samples in10 pixels; candidate had33 in16. The difference consisted of21 new errors,3 corrected errors and12 shared baseline errors. These counts are retained as failing chronology, never accepted current rendering proof.

## Preserved Artifacts

The earlier failing after-panel source/camera/18rasters and ray report are hash-preserved under generated/coverage-after-panel-failed-order. The original pre-panel snapshot is separately preserved under generated/coverage-before-panel. Current raw query is generated/pixel-surface-attribution-overlap-512.json; its exact input endpoint maps and all75 samples are retained until final generated-output cleanup. The current query imports no production renderer and edits no physical/model source.
