# Full Render Execution Events

The five-video canonical render started at 640 × 360, 6 fps, low quality, jobs 2, with logs under `🗑️generated/full`.

The first actual-frame failure occurred in physics `Beat4_Waermepumpe`: clause `compare2` exceeded the caption bar's two-line limit. The source exception identifies the German clause beginning “Am Haus liefert ein Elektroheizstab genau 1.” The parent was notified to repair the canonical narration. Other source jobs continue, and normal runtime resume will retry failed or stale source fingerprints.

Failure evidence: `🗑️generated/full/s/5/attempt-1793014828903840200.log`.

Successful decoding and visual review are not yet established.

The actual render later caught ventilation `Beat3_LatenteLast`, the `r` clause describing approximately 2450 kJ/kg of phase-change energy, exceeding the two-line caption limit. Evidence: `🗑️generated/full/s/19/attempt-1793016042341864500.log`. The caption audit agent was notified; this is handled by the same narration repair batch.

Parent repairs changed Physics shared glyph aliases and ventilation occupant placement, and the caption audit repaired measured overflows in Physics and ventilation modules. Whole-module source fingerprints may therefore invalidate otherwise successful earlier renders. The initial two-worker pass continues. A final normal resume will wait for the caption audit's stable-source notice and use up to four workers only when repository capacity permits.

The overview disappearance investigation resolved to stale same-path image-tool display caching. Default/uncached/frozen diagnostic controls and the exact 1600 × 900 overview retained the house. A unique-path copy of the same PNG was independently confirmed correct; the default-render raster visibility regression passes. See `transform.md`.

The parent-authorized initial-pass restart occurred after the all-110-source caption audit declared stable (43 repairs, zero remaining overflows). Exact command line, process ancestry and process creation times identified only the owned full-render Python process and its children. Targeted graceful Ctrl-Break could not attach to its non-console process; the exact verified parent/remaining descendants were stopped. The initial command exited with code 1 as expected for this intentional stop. Published cache was retained; current incomplete sources were handoff and heating fundamentals. No other processes were stopped.

The stable-source four-worker pass cleared the earlier Physics heat-pump caption failure and fully produced the Physics lesson. All 6,149 Physics frames decoded at 640 × 360, nominal 6 fps, duration 1024.821 s; its eight hash-named sampled frames were inspected and sent to the independent auditor.

This pass then exposed a real caller regression in ventilation `Beat1_Lueftungsbedarf`: moisture-droplet placement still referenced the removed `xs` variable after occupant-placement refactoring. Evidence: `🗑️generated/full/s/11/attempt-1793017832642671200.log`. The parent was notified to use the new occupant positions. That source's historical footage remains unapproved until a repaired source render is published.

The auditor identified a real collision in cooling `Beat5_SolarCoolingLoad`: the newly added area label crossed the ceiling, and the final wattage crossed the right wall. The fixed-coordinate assumption was incorrect. The parent is replacing it with a measured right-aligned interior column and an integration case using the actual production label assignments. That solar module's source clips and their visual review will be refreshed through normal cache resume.

The final normal canonical resume completed with exit code 0 and all five complete movies published against current fingerprints. The final pass reused all source renders and only rebuilt the two chapter snapshots affected by earlier source edits. All eight semantic excerpt selections had already succeeded using actual canonical trace data. Full movie and canonical-source decoding checks follow.

## Final Activity Pose Refresh

The late Cooling review found three hollow activity figures. After schema/portable-case-first local SVG repairs and real Manim/SciPy checks, canonical Cooling_02_InternalGains refreshed source ordinals 73–77 with four workers. The chapter render and subsequent full Cooling assembly both exited 0. All five affected sources were verified current and completely decoded; final all-110 source verification passed (44,931 frames). Updated Cooling fully decoded 6,284 frames, unchanged duration 1047.323 s. All four other full-movie SHA256 values remain unchanged. Independent audit accepted the standalone comparison and 12 actual source-74 frames, including 30–40 s. Final updated Cooling contact review is recorded in full-render-audit.md.
