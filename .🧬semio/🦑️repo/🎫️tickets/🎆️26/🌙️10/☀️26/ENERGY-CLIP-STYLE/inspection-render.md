# Inspection Render

The complete current manifest is being rendered afresh: all 17 videos (five energy, six BIM, six optimization), German spectator, including the production virtual UI and transitions.

The quick inspection profile is 480 × 270 pixels at 4 fps, with a global source concurrency limit of eight jobs. The render holds a repository capacity grant of 16,384 MiB and eight CPUs. It was admitted after a brief capacity wait. The cache scout found no provenance-valid spectator artifacts matching the current inputs; older speaker caches are excluded.

The retained launcher is inspection-render.py. Its production invocation omits --video and --chapter, so it covers the complete current manifest. It uses the registered shared teaching Python environment through repo tools exec python --light, under repo capacity run --memory=16384 --cpu=8.

Temporary progress is durable in 🗑️generated/i/master.log, status.json, and command.json. Production output artifacts are beneath 🗑️generated/i/de/spectator/. The parent task will retain validated deliverables before generated scratch cleanup.

The production compositor validates source states before assembly and presentation input state before publishing each final video. It renders sources globally with the specified concurrency, assembles chapters, and then publishes complete videos sequentially. Each presentation publication has an isolated tool session and unique temporary candidate, but its hierarchy capture belongs to the compositor invocation. No in-flight implementation is being changed to parallelize publication.

## Staged Publication

The reviewable inspection-publish.py helper can check all 117 canonical source certificates and 38 chapter certificates through production reuse guards, including clause traces, excerpt provenance and cursor intervals. Its --check-only mode writes generated/i/publish-gate.json. After that gate passes and at least one final movie exists, the original serial publisher can be stopped and the helper can run under one capacity grant. It republishes any existing first movies from certified content through force=True, then publishes remaining videos through production compose_videos concat_only with four threads, each with a copied captured catalog context. Actual browser dependency and handoff caches have explicit locks; generated candidates and work directories are isolated. One shared process and canonical handoff raster cache preserve cross-video joins.


## Source Render Failure

Canonical source 13, Beat2_Luftwechselrate, failed during the first house-to-scale-marker ReplacementTransform in ventilation/clip/volume, line 902. Manim raised a ValueError because aligned source geometry changed from (320, 3) to (176, 3) while interpolating. The immutable attempt is generated/i/de/spectator/s/13/attempt-1793093899074368100.log. Other jobs continue and successful certificates are retained. An independent reproduction is isolating the live geometry updater before a narrow authored-source fix; shared runtime changes are avoided to preserve unrelated certificates. The volume owner has exactly three canonical sources, ordinals 12–14, which require fresh provenance after any source edit.


## Revised Recovery Sequence

The independent oracle and two permanent regression tests passed after freezing the completed room glyph. Because this known source repair prevents the initial full gate from passing, the approved sequence is: finish every original source job and unrelated chapter assembly; wait for the first certified physical movie; cancel the original serial publisher and confirm its entire owned tool tree has exited; release its capacity grant; run an ordinary production ventilation-only composition with three jobs to refresh the volume scenes and chapter; then require the complete 117-source/38-chapter gate before starting staged publication. This prevents competing publishers and avoids reserving a second heavy capacity grant. The first physical and repaired ventilation movies are republished from certified content in the single staged process, so every delivered movie shares its common handoff raster cache.


## Updated Performance Recovery

The root coordinator authorized a bounded, defensive math-readout template cache after an independent benchmark showed roughly 24–31× warmed wall-clock gain with identical geometry and raster output. This supersedes the previous ventilation-only recovery plan. The narrow volume updater fix remains in place.

Before any shared visuals edit, the original owned all-17 batch was cancelled normally through its execution session. The original CLI PID 49740, capacity wrapper PID 106960, and all eight active source-attempt PIDs were confirmed exited through native process exit-code queries; generated/i/stop-check.json records those checks. Repository process diagnostics then showed only the independent verification watcher, with no remaining render, Manim, browser, or FFmpeg descendants. Capacity grants and lease holders were empty. Shared-edit clearance was sent only after these checks.

All generated artifacts remain untouched until the cache tests pass. After the approved shared source edit and green focused tests, the full canonical 17-video production command will restart with the same profile and output root. Every old source certificate is stale by design because it includes the shared visuals source hash; production guards must rerender every source against current inputs. No old artifact will receive rewritten or copied success metadata. Original supervisor logs and status will be archived beneath the generated root before restart. After fresh sources and chapter assembly, the full 117-source/38-chapter gate is again required before any staged parallel publication.


## Fresh Canonical Pass

All five focused readout-cache tests passed in 22.742 seconds. The final source hashes were verified immediately before restart: visuals 90899ef0b7e7b9110f9cd29b0b53ff22307f79a8fd070791d0855a924c702387 and fonts 23e2e4c79406b2613dc1333b2c778289fe6b9811655acecfa9059cf7bfc362e0. The fresh all-17 production invocation was admitted under the same 16,384 MiB/eight-CPU grant, 480 × 270 at 4 fps, eight jobs. Its execution session is 67560.

The old supervisor command, master log, status and native stop evidence are archived in generated/i/run-1. Original per-source immutable attempts remain beside each source. Current-pass monitoring must use the new master log or filter attempts by their start time against the new supervisor start; historical cancelled attempts and the original source-13 failure are retained evidence. No source certificate or movie was recertified by editing metadata.


## Fresh Tool Timing Samples

At 15 minutes into the fresh pass, four source completion messages were recorded and there were no new-pass tool failures. Completed-tool durations, including startup, are mixed:

| Source | Original Seconds | Fresh Seconds |
| --- | ---: | ---: |
| Beat8_Zusammenfassung | 168.14 | 188.27 |
| Beat4_Waermepumpe | 424.43 | 527.31 |
| Beat1_Lueftungsbedarf | 413.40 | 354.70 |

These complete-scene samples do not yet demonstrate a large overall rendering gain from the isolated warmed readout benchmark. Several heavier dynamic sources are still running. The stable approved production run continues without further shared source changes.


## Native Volume Tool Timings

The coordinator requested comparison of existing immutable attempt durations; no additional benchmark or source edit was performed. Original Innenvolumen completed its native tool successfully but was subsequently denied certification because the authored volume source changed during that attempt.

| Scene | Attempt | Native Seconds | Exit |
| --- | --- | ---: | ---: |
| Beat1_Innenvolumen | original | 1561.93 | 0 |
| Beat1_Innenvolumen | fresh | 495.83 | 0 |
| Beat3_Volumenstrom | original | 500.46 | 0 |
| Beat3_Volumenstrom | fresh | 445.29 | 0 |


## Windows UTF-8 Assembly Recovery

After every native source job finished, the first chapter concatenation failed because the inspection supervisor had configured UTF-8 standard streams but not Python UTF-8 mode. FFmpeg emitted an emoji path in UTF-8; the duration probe used the Windows CP1252 subprocess default, causing a reader-thread UnicodeDecodeError and then a secondary stderr=None TypeError. The normal documented authoring entry already enables Python UTF-8 mode.

The inspection launcher now supplies -X utf8 to the inner Python CLI and inventory/test entries, and inherits PYTHONUTF8=1. Publication/check-only invocations will use the same explicit mode. Shared speech/runtime sources remain untouched, so valid source fingerprints remain reusable. The failed supervisor history is archived in generated/i/run-2. Repository process diagnostics confirmed the original tree exited and all capacity/lease grants were released; only the independent verifier watcher remained. Native jobs with exit zero all had canonical sidecars once the final hydraulic-balancing job finished.

A full canonical all-17 resume is required because no chapter certificate had been published before the failure. Normal production guards will refresh stale owner sources, assemble all 38 logical chapters, and publish the first physical movie before the planned shared-cache publication handoff. Targeted cooling and sun gates are GREEN.


The exact existing speech duration probe passed under verified Python UTF-8 mode on the emoji-path Beat1_Leistung MP4: {'utf8_mode': 1, 'preferred_encoding': 'utf-8', 'clip': 'C:\\git\\semio\\.🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️26\\ENERGY-CLIP-STYLE\\🗑️generated\\i\\de\\spectator\\s\\1\\v\\Beat1_Leistung.mp4', 'duration': 170.75}


## Selected Recovery Inventory

Production provenance guards identified 14 stale canonical sources among 47 sources selected by cooling, energy_balance.

| Ordinal | Scene | Source |
| ---: | --- | --- |
| 69 | Beat1_OfficeRoom | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py |
| 70 | Beat2_HumanFactor | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py |
| 71 | Beat3_DevicesLighting | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py |
| 72 | Beat4_CumulativeLoad | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py |
| 73 | Beat5_Mitigation | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/👥️internal-gains/🐍️.py |
| 78 | Beat1_SolarIrradiance | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py |
| 82 | Beat5_SolarCoolingLoad | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/☀️solar-radiation/🐍️.py |
| 85 | Beat1_LastZuerstSenken | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py |
| 86 | Beat2_SorptionsKuehlung | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py |
| 87 | Beat3_KomfortStrategie | 🏛️architecture/⚡️energy/❄️cooling/🎬️clip/🌤️summer-strategy/🐍️.py |
| 92 | Beat1_GEGundNormen | 🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py |
| 93 | Beat2_Monatsbilanz | 🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py |
| 94 | Beat3_Klimadaten | 🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py |
| 95 | Beat4_Gradtagszahl | 🏛️architecture/⚡️energy/📊️demand/🎬️clip/⚖️regulations/🐍️.py |


## Guarded Shared Publication Recovery

The original serial chapter publisher was stopped after source completion and partial chapter assembly. The audited ticket helper first validates current unaffected canonical sources, serially prepares all eight clause excerpts, seeds physical UI rendering in one process, then uses four distinct-video production composers. Any owner whose source certificate is stale receives ordinary composition; other owners reuse current sources. All source, chapter, cursor and final presentation certificates remain production-owned. Final acceptance still requires all117 sources,38 chapters and17 outputs current.

The previous publisher process did not survive the tool-runtime reset. Repository process and capacity diagnostics confirmed no surviving publisher or verifier watcher before restart. Its logs/status are archived in generated/i/publish-previous-1793109986. Ticket-only periodic Python stack dumps now expose slow boundaries. A first guarded restart rejected BIM source114 after concurrent input changes; the helper now inventories exact stale owners before selecting ordinary composition, preserving valid sources.


The final staged input uses ordinary no-force composition for all seventeen IDs, as directed by the coordinator. The initial source inventory is informational rather than a publication gate; all eight shared clause sources/excerpts are nevertheless validated or recovered serially before parallel composers. The resumed inventory found twelve stale disjoint sources across all six BIM and all six optimization owners. The normal production guards refresh those sources. Changed-input retries and unchanged-native-error waiting now apply to every owner. No source certificates are manually rewritten.


## Diagnostic Watchdog Recovery

The first resumed publication process reached physical focus capture, then exited with a Windows native access violation while the added asynchronous faulthandler watchdog emitted its third dump. Corrupted frame line numbers prevent attributing that failure to Path.resolve or a production input. Independent path inspection found finite iteration and bounded import traversal. The ticket-only asynchronous C watchdog was removed; its evidence remains retained. The next launch instead records phase timing and source counts through the actual production caller and uses a Python daemon with sys._current_frames/traceback.format_stack under the GIL. No browser or shared production source was changed to chase the corrupted stack.


## First Certified Complete Movie

The capacity-constrained stage runs two distinct-video composers under a4GiB/four-CPU grant, maintaining the480x270/four-FPS profile. Physical fundamentals was certified after322.72s of normal composition and UI publication. Its4107-frame full movie assembly took4.9s. Actual dependency discovery returned2517 files; subsequent full provenance checks took approximately15–19s each. The physical seed populated the process-shared real-UI root handoff raster. Ventilation and heating began concurrently afterward. Final acceptance remains the complete current17-video/117-source/38-chapter gate and independent verification.

## Four-Level Publication Resume

The reset left no owned process. The historical run had five completed energy outputs, no full completion gate, and stale in-progress BIM status. Publication was deliberately held while the actual wrapper was expanded to course → topic → clip → chapter without modifying the body runtime or interaction provenance.

The ticket publisher now retains its ordinary no-force guarded composition of all seventeen lessons and additionally validates all thirty-eight individual chapter exports. Canonical chapter paths come from the production collection API. Each chapter must have the exact current raw logical body and raw cursor hash, the expected current parent presentation certificate, valid published cursor intervals and the current chapter navigation clock, owner and authored order. Full source117/chapter38 guards remain mandatory, and completion additionally records chapter_exports38. Body-frame counts are structurally checked here; the independent verifier measures actual native media clocks and pixel identity.

The obsolete extra forced physical publication was removed. Individual chapter publication seeds the generic versioned ancestor raster cache. This revised launcher has not started; the root-owned source refresh and wrapper smoke have exclusive current ownership.

The independent launcher audit accepts the chapter/order gates. Publication remains no-force: a separate current physical smoke is reused normally. Root requires measured independent fresh-process boundary determinism; the shared in-process raster cache alone is insufficient completion evidence. No manual raster seed or certificate rewrite is permitted.

All 38 canonical chapter export filenames satisfy the 256-character path limit; longest path is 177 characters.
