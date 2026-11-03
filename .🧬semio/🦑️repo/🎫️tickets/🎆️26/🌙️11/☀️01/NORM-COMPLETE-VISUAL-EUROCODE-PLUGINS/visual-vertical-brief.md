# 🧊 Visual Vertical Brief

Read `fleet-brief.md`, `/home/deck/semio/AGENTS.md`, `📌️important/build-loop.md` (mandatory build method; memory-bound queue: batch edits, one build at a time), `📐️specs/visual-editor-architecture.md` (§3 scene, §3.6 per-artifact map, §4 windows, Wave 3 in §9, §10 definition of done), your code's target spec (visualisation and realtime-editor sections), your code's inventory in `🔍️audits/`, and the shared-package reports `🔍️audits/wp*.md` as they appear. Follow `📌️important/artifact-rollout-checklist.md` once WP-12 (EN 1993 reference vertical) writes it, and look at `🗿️artifacts/🔩️en1993/.../🎬️scene` and the `vis-*` reports of sibling verticals for patterns.

## Phase 1 (build-light, start now)
Create `<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/**` implementing `StructuralProjection` (crate `semio_s_plugin_norm_structural_scene`; see `wp01-scene-schema.md`, `wp0304-tessellation-projection.md`, `wp08-diagrams.md`): 3D entities for everything spatial in the model, utilisation colouring, failure-mode glyphs, deformed/mode shapes where meaningful, 2D diagrams for every governing check, and handles for every dimension/position the engineer edits. Add golden scene tests and a `🥒️.feature`.

## Phase 2 (after the rollout checklist exists)
Replace the editor/viewer windows per the checklist: viewport, diagrams, properties, checks, outliner, live re-evaluation on every edit, drag handles. The viewer shows the same scene read-only. Delete the old windows; no compatibility layers. No JSON is visible anywhere.

## Ownership
You own `🎬️scene/**`, `✏️editor/**` windows/panels/commands/modes, and `👁️viewer/**` of your artifact. Your code's norm-completion agent owns `🧬️schema/**`, tests, oracles and field-meta. Request Placement fields/mutations and CheckUnits via `📌️important/requests.md`. Re-read before every edit, and never revert anyone else's changes.

## Rules
No runtime external libraries. Concise code with no comments inside definitions; docstrings start with a unique emoji. EN + DE throughout. No git writes. Logs go to `🗑️generated/vis-<code>/` (delete when done). Write your report to `🔍️audits/vis-<code>.md`; your final reply is a 5-line summary plus that path.
