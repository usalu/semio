# Architectural Silhouette Update

The energy clips now use architectural entourage and physical profiles throughout shared artwork and chapter-specific call sites. Standing, walking and running use curated natural contours; seated and sleeping profiles are hand-authored. Equipment uses solid elevations, the sun and moon use single filled contours, and architectural annotations replace thought bubbles. The ventilation roadmap shows a projected open sash. Industrial and tree cues use an industrial elevation and a branching tree silhouette.

The original standing, seated and activity sizes and floor anchors remain intact. The running heat origin was moved into the new torso. Metabolic values remain 80, 100, 200 and 800 W. Sources and CC0 attribution are in `silhouette-sources.md` and the silhouette library README.

## Implementation

Portable JSON contracts and acceptance cases preceded implementation. The first tests failed against geometric figures and outline equipment. Curved geometry and independent raster checks caught a crossed calf, a wrist loop and a misplaced running heat origin; these were corrected. The shared renderer reads local Bezier coordinates, and chapter activity assets remain local SVG. No new runtime dependency was added.

Actual scene execution exposed a room fade failure: the material compositor discarded its geometry while the starting animation copy was transparent, changing the animation family and point counts. It now retains the hidden contour while no opaque material is present. A failing real Manim FadeIn regression preceded this fix.

## Verification

- Final focused suite: **42 tests passed in 13.867 s** after the final artwork changes. This covers silhouettes, actual fade animation, architectural dimensions, activity heat origins, connected masks and section consumers.
- Broader visual suite: **77 tests passed in 429.862 s**. It completed before the final natural-contour curation; the subsequent 42-test gate covers those changed paths.
- AJV independently validated both the architectural drawing profile and silhouette-library JSON schemas.
- All **105 canonical source scenes imported**. Five selected production scene portions executed at 640×360 and yielded **26 settled narration frames with no errors** after the final changes.
- Visual inspection covered the complete 17-item artwork sheet and production contact sheets for physics, ventilation, heating, cooling and primary energy. The open-window roadmap and revised branch/industrial silhouettes were checked again after their final changes.
- Manim supplies independent dimension and raster results; SciPy component labelling and hole filling validate the local component count. The actual activity scene verifies heat attachment points.
- The exploratory full authoring aggregate was cancelled while its native-presentation fixture was still encoding. No aggregate pass or complete final-video encode is claimed. This task verifies source artwork and settled production frames.

The exact file inventory is in `silhouette-files.md`. Reproduction inputs remain in this ticket: `silhouette-run.mjs`, `silhouette-preview.py`, `silhouette-scenes.py`, `silhouette-schema.mjs` and `silhouette-ticket.mjs`. All commands enter through repo; no modifying Git command, worktree, IDE, dashboard or AGENTS.md edit was used. Native Windows execution is verified; the sources and scripts use portable paths and the managed Python environment. Other operating systems were not executed in this session.

## Review Outputs

Temporary review images, downloaded candidate vectors, logs, caches and incomplete aggregate fixtures are removed from the owned generated directory after verification. Source art, portable tests, provenance, reproduction inputs and this report remain.

Cleanup completed: the owned generated directory and its empty parent were removed. Retained inputs, source attribution and verification notes are listed in silhouette-files.md.
