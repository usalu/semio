# Render Audit 2026-11-07

The preview requires 26 fresh masters and segment provenance; see preview-gate.md. The old preview is 289682914 bytes and the manifest was modified November 2. The media-lock directory dates November 3. Render and join logs last changed November 4, so their waiting messages are historical. Confirm lock ownership before removal.

Fleet: PID 60632 running 13.3 hours, command unknown; no identified renderer. Free memory 36.5 GB, spare 26.5 GB, pressure normal, swap 140 MB; storage reader timed out. Two shared leases belong to 60632. Previous Excel renders consumed 9-10 GB each and paged.

Current freshness audit reports physical_fundamentals 0/8 and ventilation 0/22, both masters stale. The status audit is still running. No rendering was launched.

Use repo tools exec bun with the architecture energy authoring script render command, specifying video, language en, audience speaker, quality l, fps 15, no-play, log-output and jobs 1. After every master is fresh run canonical manifest.py en speaker and course_preview.py en under managed Python. The joiner strips audio and emits preview MP4, metadata and stills. Existing PowerShell drivers contain absolute Windows paths and use the old generated folder.

Completed status audit: all 26 English speaker lessons are not fresh. Every planned segment reported stale: physical 0/8, ventilation 0/22, heating 0/35, cooling 0/22, energy balance 0/26, and every BIM, Grasshopper, Excel and optimization lesson 0/1. All six BIM masters are missing; all other 20 are stale. Audit completed in 139 seconds. Stale means an existing segment fails its current state fingerprint, not necessarily invalid audiovisual content.

A Windows process query for PID 60632 returned no instance; its fleet entry and shared leases were stale at the time of the query. This does not establish ownership of the media-lock.

## Prior attempt failures and current source

Attempt logs under videos/en/speaker/s show BIM 114 failed KeyError z0 in model part_band_footprints; 115-118 failed KeyError outline in axon primitives finish_parts. The same direct indexed field reads persist in current source; plan and axon also assume outline/z fields. BIM 119 log ended without an exception, not proof of completion. Staffage agent was notified.

Grasshopper tower, casting and trees attempts 122-124 ended in Manim MemoryError during family/point traversal; previous simultaneous renders were under severe paging pressure. Other attempts were interrupted with no explicit final exception. Excel energy attempt 128 reported FadeIn of NakaginDrawing3D with 617928 submobjects at about 135-142 seconds per frame. Current Nakagin visuals predates those attempts: tower template caches or freezes geometry over 200000 facets, but tower_axon thaws a full live drawing, so cached storage does not by itself bound live animation traversal. Exact current count is unmeasured.

The existing media-manifest contains only physical_fundamentals, marked stale, so it cannot establish completeness across the course. Existing inspection shows 20 old masters, durations 265-2137 seconds; all six BIM masters are absent. Durations alone cannot verify narration or scene completeness; compare clause timings and visually inspect new renders. The current plan status reports existing but stale segment clips, rather than missing planned clips.

## Scene and preview audit

Static AST audit of 134 declared segment occurrences and 126 distinct classes found every script and scene class, nonempty construct, and NARRATION. Every literal narration row has nonempty English and German; eight dynamic f-string or expression rows were manually inspected and include both languages. This confirms declarations only: classes often delegate through present, and static checks do not prove all cues are displayed or timed. Status computed cast keys but stale clips prevent the manifest from validating cast provenance; no complete cast-record audit was established.

The full preview requires all 26 fresh masters and must omit allow-missing. The join reads freshness from the manifest without rechecking live source; regenerate manifest after the final source edits and before joining. Normalization explicitly uses ffmpeg -an, stripping every audio stream, and preserves video at 854 by 480 and 15 fps with one chapter per lesson. Check whether a silent course preview matches intended delivery. No source files were modified.

## Cast cache audit

Read-only bridge audit over source dependencies for all 126 unique scenes found 99 known cast keys. Ninety-two current keyed indices pass viewpoint and _provenance checks; seven lack their current index: bim-consistency-decider, heating-calculation-thermostat, optimization-carbon-curtain-wall, shared-desk-summer, shared-desk-winter, shared-diesel-generator-rated and shared-person-relaxed-line. No older-key directory for those names was present in the staffage cache at inspection. All 99 keys could be derived from the manifest and current asset/preset sources; this is not a check for unknown dynamically generated cast names. staffage_provenance calls ensure_casts and may trigger an exporter; the audit used _directory, _index and _provenance directly and generated no cast. Results are a point-in-time snapshot while rendering continues. Audit script is cast-audit.py at this ticket root.

## Audio Track Check

PyAV inspection confirmed all twenty existing 480p15 masters have one audio stream each. Final full-preview assembly must preserve and normalize these streams rather than use the old silent join. The energy execution agent owns that correction and a synthetic tone/video regression.

## November 7 BIM watchdog

Baseline: current s/114 and s/115 attempt logs reached Manim configuration at 02:40 UTC, with no later exception at 02:55 UTC; s/116–119 logs still reflect November 4. No canonical BIM 480p15 masters observed. Long geometry setup is expected; no process was stopped.

Workspace statfs during render reported 106.87 GiB available of 784.4 GiB; sufficient for a multi-GB normalization estimate, with no hard upper bound established for concurrent intermediates. No cleanup was performed.

Watchdog handoff: current optimization extrema master and sidecar published November 7 03:56 UTC and three sampled frames are reviewed in visual-review.md. Other five optimization masters last observed dated November 4, so no current review was claimed. BIM s/114 and s/115 last observed in the ongoing quiet geometry phase without fresh logged exception; remaining BIM attempts were historical. No process was stopped. Continue monitoring and verify all 26 masters fresh before final join.
