# Layered Visual Audit

Read-only inspection of shared browser smoke-0 through smoke-5 and presentation browser TSX on 2026-10-26.

## Actionable Verification Gaps

- Smoke-1 and smoke-3 capture overlapping outgoing overview cards at the upper left. Smoke-2 and smoke-4 resolve cleanly. Treat the former strictly as transition frames; final clip pauses should use settled states so Energie does not cover Physikalische Grundlagen and the overview card does not remain over the fullscreen content.
- These six frames do not independently prove chip-only hover activation or exact chip-to-fullscreen motion. Final previews should include a pointer inside the card but outside Ansehen, a pointer on Ansehen, a midway reveal, and the final opened video. Compare actual fullscreen silhouette bounds (approximately x=3.2, y=32, width=1593.6, height=836) against the clip overlay.
- These six frames show only one camera offset per overview level. Include two pointer positions before opening Energie to verify actual LayeredOverview parallax, then repeat at the video level.

## Preserved Visual Reference

Smoke-0 confirms topic order energy, parametric, bim, grasshopper, excel, optimization; three columns and two rows; bottom-left Ansehen chips; tinted card panels; blurred underlying pane content. Smoke-2 confirms the independent second video overview with five cards. At 1600 by 900, root card bounds are x=6.39/539.72/1073.05, y=135.41/556.61, width=520.55, height=207.95. Navbar and footer heights are 28.8. These shared component measurements should remain the reference rather than hand-recreated approximations.

Presentation source imports actual LayeredOverview, OverviewCard, OverviewCardOpenChip, WindowChrome, Navbar, and Footer. It nests video Overview inside topic panes and passes controlled camera/open/reveal state to the shared component. No source changes made.

## Fast Current Authored Content Preview

Production runtime module: 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️. API: render_segment(segment, media_dir, settings, output_root=...). Use script 🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py and scene Beat1_Leistung. Its opening clauses rate, unit, area, vacuum explain power, watt, energy as diagram area, and a vacuum cleaner example. This standalone canonical scene has no institute intro.

Settings: quality=-ql, resolution=640,360, fps=6, audience=speaker (spectator hides subtitles). Keep media_dir and output_root inside ticket generated. API returns media_dir/v/Beat1_Leistung.mp4 with source provenance, narration trace if available, and cursor intervals. It renders only this beat. Then use production cut_clip(source, output, start, end), e.g. 0–16 seconds. For semantic selection use source_marks and clause_interval(first=rate, last=unit), then cut_clip. render_segment has no time range option, so a complete single beat is the minimum source render.

To wrap fresh content with current real presentation UI: 🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🐍️ API render_presentation(physical_fundamentals, content, output, settings). Raw authored source is fastest proof of current authored visuals. Old full physical.mp4 was not used.

## Final Retained Preview Audit

Reviewed layered-topics.png, layered-videos.png, layered-preview.png, layered-physical-body.png and navigation frames 0036, 0057, 0095, 0104, 0110, 0137, 0150, 0166 against layered-navigation-events.md.

- Replace retained layered-preview.png with a settled opened frame (navigation 0110 or later). Its upper-left Physikalische Grundlagen card and dark quarter-screen tile remain visible over the fullscreen silhouette. Frame 0104 has the same overlay during pre-open reveal; frame 0110 is clean after the 9.000s release, so the film itself resolves the transition correctly.
- The stated 7.917s reveal timestamp (frame 0095) is still mid-motion: fullscreen border ends near x=1433/y=526 with its top clipped, while the source card remains visible. The final fullscreen destination is reached by frame 0104 (8.667s). Name the event reveal starts or record a separate settled timestamp if the event document is used as validation evidence.
- Physical body still shows scientific P [W] in cyan, while the navigation thumbnails now use warm neutral plus red. If the requested style requires one shared palette throughout authored scientific content, this is a remaining source-style difference. Otherwise the body is legible, positioned safely inside the real shell, and has no clipping or overlapping UI.

Confirmed: visible shared tinted panels and bottom-left Ansehen; six correctly ordered distinct topic thumbnails; independent five-card video level; significant blurred-background movement between root body-hover frame 0036 and root handoff 0166 and between video-level frames 0057 and 0137; opened frame 0110 has no lingering card; restore 0137 returns to video level; 0150 and 0166 restore topics. Physical still begins in actual power lesson content with title and P [W], not an institute card. A single body still cannot establish every opening frame of the physical film; the identified standalone Beat1_Leistung source contains no institute intro.

## Final Appendix After Restore Timing Correction

Re-inspected retained layered-preview.png and layered-physical-body.png and read layered-preview-verification.md. The replaced fullscreen still is clean: no overview card, dark quarter-screen overlay, or clipping remains. Physical still preserves safe title/body positioning inside the actual window silhouette.

Verification correctly labels the deliverable as a fresh canonical Beat1_Leistung excerpt and lists 24.000s at 1280 by 720 / 8fps; it makes no claim to a newly rendered full 17-minute lesson. For complete scope clarity, add one sentence stating that the 24-second physical preview consists of 18 seconds of authored source plus 6 seconds of navigation. That split is absent from the verification prose/table as reviewed. The all-five encoded handoff assertion references registered regression coverage; current rerun completion should remain backed by the final regression evidence from the responsible agent. No additional source changes or tests were performed in this review.
