# Audit: Viewpoint Consumers (Haiku, read-only, 2026-11-02)

## All three views are already supported
The following consumers already handle `elevation`, `front` and `plan`:
- the schema and its typed twin
- depiction (ground line, dimensions)
- the check CLI, which fails on any empty viewpoint
- copy detection
- the export and preview CLIs (`--viewpoint`)
- the SVG metadata
- the React viewer (segmented Side, Front and Top control, URL parameter `vp`, i18n in en and de) and downloads
- the cast exporter, which passes `cast.viewpoint` on to depiction

## Elevation is the default
The default is elevation in these places; that is acceptable, since every view is now mandatory:
- the cast `viewpoint` (optional, `?? "elevation"`)
- the CLI flag `--viewpoint`
- the `textlessDepictionOptions` parameter
- `DEFAULT_VIEWER`
- the pipeline request

## Gaps
1. The `🎭️cast-export` test (line 53), `🧍️front-projection` (line 99) and `🧍️plan-projection` (line 124) still assert on `presentation.viewpoints`. FW is assigned to them.
2. The fixtures `asset-validation`, `sample-figure`, `reference-walker`, `sparse-smooth-walk` and `continuous-rotation` still carry `presentation.viewpoints`. FW is assigned to them.
3. The written `CastIndex` (`E/🧬️schema/🟦️.ts:43-57`) has no `viewpoint` field, so the Python visuals `_index` cannot tell which viewpoint a cast was drawn in.
4. The `🧍️front-projection` scenario "A cast of a viewpoint without geometry is refused" has no assertion. It is moot now that front and plan are mandatory, so the scenario must be removed or rewritten.
5. The course manifest already uses `front` casts (radiator, split-ac, tilt-window) and `plan` casts (standing-adult, vintage-car-1929). They need re-exporting after the redraw.

## End to end
A course clip can request front or plan through the manifest entry `viewpoint`. The Python functions `staffage`, `staffage_motion` and `fitted_staffage` select by cast id, which is enough because the id already fixes the viewpoint.
