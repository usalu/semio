# Inspection Render Coordination

The selected review profile is 480 × 270 at 4 fps, German spectator, complete authored timing. All 17 declared videos remain in scope. The manifest has 38 chapters, 125 ordered segment occurrences, 117 unique scenes and eight clause selections. The original production source pool owns one 16 GiB / eight CPU grant and eight workers.

The initial production planner computes the full provenance graph before launching source jobs. This took several minutes and completed normally. Rendering then began with the eight physical scenes. Low pixel dimensions reduce raster work but do not remove vector construction cost.

Current spectator source caches were unavailable; older shared speaker caches fail current dependency/settings checks. No metadata was altered to reuse them. Existing retained videos from earlier requests remain untouched.

For the subsequent UI phase, the reviewed input `inspection-publish.py` uses only existing production composition/publication APIs. After all source, chapter and excerpt certificates pass current guards and the original serial publisher has produced its first certified movie, the original process may be stopped. One new process captures the catalog, republishes the first movie through `publish_presentation(force=True)` to seed its canonical root raster, and publishes the remaining distinct IDs through four copied contexts and bounded threads. Per-attempt work, browser batches and sessions are isolated; shared source and root caches have locks. No concurrent writer may publish the same ID.

The independent audit is in `inspection-publication-audit.md`. The media watcher must invalidate any earlier verification if its file identity changes during republication. All 17 final common root frames must also be compared across videos, in addition to body coverage, full decode and camera seams.

The existing runtime `_stop_process(Popen)` and `ToolSession.cancel()` own process-tree termination on Windows and POSIX. `ToolSession.run()` stops its owned subprocess tree when interrupted. Stage handoff should first use normal Ctrl-C and verify the original publisher and browser/encoder descendants have exited before any replacement publisher writes those IDs. No process-inspection dependency or new OS-specific cancellation implementation is needed.

The retained `inspection-retain.mjs` will accept only all 17 passing measurement records, verify every MP4 against its measured SHA-256 before and after copying, retain coverage/navigation images, and create the playable `inspection.md` index. Generated caches, diagnostics and environments will be removed only after verification and delivery retention. This orchestration requires no production source change.

## Ventilation Render Failure

Source 13 failed in `🌬️ventilation/🎬️clip/📦️volume/🐍️.py`: `Beat2_Luftwechselrate.construct()` calls `show_use()` at line 905, whose simultaneous replacement transforms at line 902 produce incompatible point arrays (320 × 3 and 176 × 3). The root inspected the stack, `show_use`, the static volume label and `_scale_marks` targets. The dot and label targets are independent; the value readout already calls `clear_updaters()` immediately before the failing transform. Isolating the two replacements is necessary to distinguish live-readout mutation from target geometry. The execution agent owns the narrow production fix, and the verification agent owns an independent reproduction/regression. Other source jobs continue. Final publication is gated on successful corrected sources and chapters.

Runtime `segment_state` excludes assets beneath a `🧪️tests` path component from recursive source-asset discovery. Portable regression fixtures belong in the existing test taxonomy. A source-file change to the volume module requires current rerenders of its three scenes; it does not require changing shared runtime or theme code.

The independent real-Manim oracle identified one remaining descendant updater on the house glyph. Recursively clearing glyph updaters before its final replacement transform fixes the original geometry mismatch. The scoped production change is `glyph.clear_updaters()`; the regression fixture and two permanent tests are under the volume module's excluded `🧪️tests` directory.

Because the original planner captured the pre-fix volume source, the initial all-catalog pass cannot certify that changed chapter. Revised handoff: finish the original source pool and unrelated chapter assembly, wait for the first certified physical movie, stop its owned publisher tree, run ordinary production ventilation-only composition to refresh the three changed scenes and chapter, then pass the full 117-source / 38-chapter gate before bounded publication. This releases the original grant before repair and prevents concurrent publication of one ID. All already certified movies are republished into the final shared-cache process as needed.

## Measured Slow Source Frames

Source 12's `_Flow` animation 44 takes roughly 8–20 seconds per frame over 30 frames; source 16 has comparable multi-second frame cost. A direct system memory read reports 21.8 GiB free of 63.9 GiB with 16 logical processors, which does not indicate immediate memory exhaustion. The scout traced repeated unchanged `math_readout` updates through `always_redraw`, `math_label`, `math_text` and `_math_render`. The source batch continues while the read-only audit and a bounded benchmark distinguish text reconstruction cost from vector drawing. No shared rendering change is authorized without concrete measurements and a tested behavior-preserving design.

## Tested Cache Optimization And Full Restart

The completed benchmark in `inspection-math-bench.md` measures warmed live readout wall-time gains of 23.7–30.6× and CPU gains of roughly 3–5×. Real geometry, style, baseline, bounds, Cairo raster, copy-isolation and callable-placement comparisons pass. These gains justify optimizing the shared live readout path instead of continuing the expensive unchanged reconstruction.

Root authorized the High verification agent to implement a bounded defensive-template cache with captured font/theme/kerning/text-canvas namespace and focused language-neutral/Manim/Cairo tests. The execution agent must first stop only our owned original batch and verify its descendants are gone, then allow the shared edit. Once the scoped tests pass, restart all 17 through the production renderer with fresh fingerprints. Old source certificates are stale and must never be recertified by changing metadata. Keep old generated artifacts until the new run is verified and final cleanup is safe.

This replaces the earlier ventilation-only repair sequence: the complete restart incorporates both the one-line volume updater fix and the tested shared readout optimization. The later complete-source/chapter gate, first certified movie handoff and shared-cache bounded UI publication still apply.

Root reviewed the written cache: `_math_readout_template` owns at most 32 unplaced `math_text` results; `_math_readout_text` normalizes full RGBA and returns a defensive copy, while `math_readout` still evaluates text and callable position on every update and uses the original `place_math`. The original `math_text` and `math_label` paths remain the independent oracle. The font-owned primitive namespace includes cached asset identity, face, fallback candidates, size/spacing, kerning and canvas settings, frame/renderer configuration and stable Text constructor/hash descriptors. Context is read per update without recomputing source-file hashes.

All five focused cache tests passed in 22.742 seconds, covering context and canvas/renderer/frame/default changes, original-updater history, unchanged-value reuse, moving placement, distinct returned objects, source/size/RGBA keys, 32-entry eviction, full geometry/style/baseline and copy mutation isolation, and exact Cairo pixels/anchors. Separate actual-source volume regression tests also pass. Full canonical restart was authorized immediately after these greens.

Final shared source SHA-256 identities at restart: visuals `90899ef0b7e7b9110f9cd29b0b53ff22307f79a8fd070791d0855a924c702387`; fonts `23e2e4c79406b2613dc1333b2c778289fe6b9811655acecfa9059cf7bfc362e0`. No further shared editing is planned during the fresh source batch.

## Cooling Room Blocker

At 63 fresh source completions, source 69 (`Beat1_OfficeRoom`) failed at line 207 of the cooling internal-gains module. The failing `ReplacementTransform(facade, room["shell"])` raises Manim's strict family-zip mismatch. The execution agent owns a narrow authored-scene fix; the verification agent independently reproduces the actual geometry transition and adds a portable regression. Other jobs continue, and shared font/visual/runtime helpers stay unchanged. Any local-module hash changes will be recovered through ordinary production composition before the complete-source/chapter publication gate.

Independent reproduction shows the office failure is aliased drawable-family topology, not a live-updater-only issue; clearing source, target or both does not resolve it. Source 78 (`Beat1_SolarIrradiance`) fails on the same strict-family morph. Source 86 (`Beat2_SorptionsKuehlung`) passes a two-component coordinate to its local microbe glyph, whose native movement contract requires three components.

The verification agent owns a concise neutral snapshot helper and portable geometry/paint-equality regression cases; only affected authored owners will import it. The execution agent owns those two import/call changes and the explicit three-coordinate point correction. Shared umbrella helpers will not import the new module, avoiding global fingerprint invalidation. Ordinary cooling-only composition will refresh the three affected owners after the main batch is stopped at its first certified physical output, followed by the complete gate and bounded publication.

The preliminary visual concern in ventilation source 29 is resolved: the 50.75-second midpoint samples an intentional moving-symbol transition; 48.75- and 52.75-second settled layouts are clean. No layout fix or publication delay is warranted.

## Demand Sun Contract And Recovery Scope

Demand regulation sources 93 and 94 fail during initial sun FadeIn: their local updater assumes obsolete ring/ray child indices, while the canonical sun glyph has one filled outline in a one-child group. The authored updater will retain its live fill law and remove invalid ring/ray operations, with a portable fixture and native stage/FadeIn regression. No bounds-check fallback or legacy glyph structure is added.

Recovery now selects both `cooling` and `energy_balance` through ordinary production composition after the main batch/unaffected assembly and first certified physical publication. The original publisher tree must be fully stopped before the recovery grant; all current source/chapter/cursor/excerpt guards remain mandatory before bounded final publication.

Root reviewed the new neutral `🎬️clip/🎞️animation/🐍️.py` snapshot helper: it copies unique drawable members in native paint order, strips copied descendants to avoid double painting, and clears/replaces the consumed root's geometry/updaters while preserving root identity. It is intended only for committed morph boundaries where callers no longer depend on the former child hierarchy; actual Cairo and translucent-paint equality are regression gates.


## UTF-8 Assembly Resume

All native source jobs finished, but first chapter assembly failed because the top-level Python subprocess reader decoded FFmpeg UTF-8 emoji paths as Windows cp1252. The stderr reader returned no string and duration parsing then failed. The existing source tool sessions already configure their children correctly; the top-level launcher must pass Python -X utf8 at startup. Setting only stdout encoding or changing the environment from inside an already running Python process does not activate UTF-8 mode.

Recovery uses a normal full all17 resume, no force, explicit -X utf8 plus inherited PYTHONUTF8=1, same480x270/4fps/eight jobs/16GiB. Valid sources are reused; only stale local-owner sources refresh. This also assembles all38chapters, which selected-only recovery could not do after the first-chapter failure. Publication and check-only Python launchers receive the same UTF-8 startup flag. No shared speech source is changed.
