# Sourcing Preview Catalogue Staging

## Problem

Selecting a row in the pool catalogue updated preview geometry via `render_with_request_context` and the `rows` selection domain, but the preview window always published `world3d_default_camera()` with no `fit_json`. Large beams and small fittings appeared at the same scale and often off-frame.

## Fix

- Added `preview_kind_bounds` in the curation schema: AABB for box-built kinds (all `box_parts`), symmetric envelope for GLB `extent`, and mesh vertex bounds for free-form meshes.
- Preview window now builds a full `World3dScene` with `fit_json` (revision keyed on `kind.id`, padding 1.25) so the host performs a one-shot frame when the catalogue selection changes — same contract as generation3d/cad/energy previews.

## Verification

Run unit tests on `semio-s-artifact-sourcing-curation` (preview + schema modules).
