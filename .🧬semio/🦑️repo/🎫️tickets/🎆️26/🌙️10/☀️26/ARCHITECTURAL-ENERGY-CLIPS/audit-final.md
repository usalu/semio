# Final Audit Status

All findings listed below are historical and have been corrected or scope-resolved. Final source rechecks and independent architecture, ventilation and overview visual acceptance passed; see [audit-addendum.md](audit-addendum.md) for exact corrections, test attribution, and the resolved stale-image observation. No remaining actionable issue was identified by this audit. Full five-lesson rendering is the coordinator-owned integration check.

# Independent Architectural Energy Clips Audit

Read-only source audit performed after the architectural helper integration, while the parent ran unique-scene smoke renders. No application code was modified. Findings below describe the inspected revision and may be resolved concurrently.

## P1: Actual Canonical Sources Are Rendered Without Narration

🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py:333–351 invokes Manim and publishes the rendered source and trace without invoking speech synthesis, aligned scene audio, or muxing. 🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2516–2846 does not attach any audio in the canonical thermal-mass scene. The new manifest selects that source for both heating and cooling. cut_clip at runtime lines 397–400 maps optional audio, which preserves existing audio but silently accepts its absence. Consequently the real reusable explanation lacks the narration required by acceptance criterion 5. The synthetic audio-cut test proves preservation of an injected tone, not presence of canonical spoken narration.

Wire the existing speech interface into canonical-source production before publishing its success state, using the measured clause marks for alignment. Include speech implementation, audio assets and voice/language configuration in provenance. Independently inspect a real canonical source and selected excerpt for a nonempty narration stream and matching spoken clause boundaries. The existing separate section builder already muxes scene audio at 🏗️authoring/🔊️speech/🐍️.py:398; the curriculum runtime does not use that path.

## P2: Shared Geometry Contract Does Not Invalidate Cached Video

🏗️authoring/🎬️clip/⚙️runtime/🐍️.py:300–310 includes the shared visuals Python module in source fingerprints but omits its new adjacent 🔣️.json contract. 🏗️authoring/🎨️visuals/🐍️.py:25–26 loads this contract, and person/radiator/house geometry reads its values. Scene-local asset enumeration at runtime lines 304–305 cannot reach the shared contract. Changing only a silhouette outline or architectural dimension can therefore leave complete source clips, selected excerpts, chapters and final videos accepted as reusable.

Add the shared contract to the dependency set, preferably through an explicit shared-source inventory. Add a regression proving that a contract-only change changes segment state and invalidates both complete and excerpt cache artifacts.

## P2: Canonical Summer Excerpt Still Opens An In-Plane Rotated Rectangle

🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py:2804–2813 creates open_glass by rotating the room glazing rectangle 28 degrees in the drawing plane. It then restores another room glazing rectangle at lines 2820–2829. This bypasses the new recessed projected-sash API and makes the canonical summer explanation visibly inconsistent with the other window conversions. The new cooling reuse now repeats this unconverted sequence.

Build a shared window_glyph in the room aperture, hide the room glazing rectangle, and animate open_window/window_opening for opening and closing. Verify the window reveal remains fixed while the projected sash opens within the room.

## Audio Scope Resolution

The coordinator clarified that this task preserves and reuses available canonical audio and does not generate new speech or select providers. Read-only inspection found zero checked-in audio assets for these canonical sources. The initial P1 audio finding is therefore scope-resolved: preserving authored silence is expected for the current corpus. Available-asset attachment and preservation still require verification after implementation; neither synthetic audio tests nor optional stream handling establish that actual existing canonical clips contain narration.
