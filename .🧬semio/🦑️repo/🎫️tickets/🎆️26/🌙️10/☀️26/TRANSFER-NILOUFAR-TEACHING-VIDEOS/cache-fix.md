# Interrupted Render Cache Fix

The compose loop now removes a chapter's render sidecar immediately before launching a replacement render. If the render is interrupted after replacing the MP4 with more than 1,000 bytes, the partial clip has no matching provenance and cannot be reused. A successful render also recomputes the authored-source fingerprints and fails before publishing provenance if those sources changed during rendering.

Language-neutral cases were added to video_runtime.tests.json and consumed by test_video_runtime.py: interrupted forced rerender, and successful render concurrent with an authored intro change. Both cases failed before the fix. After the three-line runtime change, the complete runtime suite passed all 8 tests in 4.048 seconds with actual Manim enabled. All test directories were created under this ticket's generated/cache-test directory.

Files changed: 🎓️teaching/video_runtime.py, 🎓️teaching/test_video_runtime.py, 🎓️teaching/video_runtime.tests.json. Ticket input test-cache-fix.ts retains the repo-compatible test invocation. No authored scene or asset changed and no live render process was interrupted by the fix.

clip_state(settings, sources) retains its existing settings + SHA-256 sources metadata shape, so the parallel staging monitor can continue validating and publishing the same sidecar format.

## Fresh Chapter Simulation Addendum

Every actual compose_video render now unconditionally passes --disable_caching, including a stale/missing chapter sidecar at the same quality with force=False. Whole completed chapters with matching provenance are still reused without invoking Manim.

The new language-neutral non_reusable_chapter_render case exercises both missing and mismatched sidecars with unchanged settings. Its test captures the actual Manim command emitted by the compose loop. Both variants failed before the one-line branch change; the final complete suite passed all 12 runtime tests with actual Manim discovery enabled in 10.476 seconds. The test command completed with exit 0 and no owned test process remains.

This follows the real negative recovery proof in recovery.md. Local Manim 0.22 CairoRenderer.play lines 104–110 uses the same skip_animations shortcut on partial-cache hits: it adds exact scene.duration instead of frame-quantized render time. Scene.get_time_progression lines 1125–1129 uses a single [run_time] sample, and lines 1412–1413 omit the final update_mobjects(0) when skipping. Partial reuse can therefore change downstream narration budgets and dt-dependent state. Complete chapter reuse does not reconstruct downstream animation state and remains safe behind the existing provenance checks.

Only the compose render-branch flag, its consuming regression, and the neutral case changed in this addendum. Authored sources, live jobs, and clip_state metadata format remain unchanged.

## Optional Authoring Tool Cache Policy

The same fresh-simulation policy now covers tts_pipeline.build_section_video and every existing cmd_trace command. The catalog contains 26 audio generators: 20 offer trace mode, and six provide synthesis only. All 20 trace commands now include --disable_caching; the shared section helper applies that flag to all 25 section builders. This prevents cached Manim plays from changing renderer.time, updater history, or hold_for voiceover budgets, as demonstrated with real Manim in recovery.md.

TDD updated the language-neutral pipeline render_policy case and consuming command tests first. The section-builder test failed for the missing flag, and the all-generator trace check identified all 20 missing flags before the production fix. After one bounded update, the full pipeline suite passed all 12 tests in 1.564 seconds, and all 26 real-Manim module imports plus all 20 intercepted trace commands passed in 8.093 seconds. No TTS credential was used, no speech request was sent, and no expensive render was launched. Shared video_runtime code and interfaces were unchanged in this follow-up, so its already passing 12-test suite did not require repetition.

Both test commands completed with exit 0. No owned process remains. No authored scene, PDF, visual asset, live controller, or generated-cache repair was changed.

Changed product files:

- `🎓️teaching/tts_pipeline.py`
- `🎓️teaching/energy/demand/1_physical_fundamentals/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/1_warum_lueften/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/2_luftmenge/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/3_waermestrom_luft/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/4_natuerliche_lueftung/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/5_mechanische_lueftung/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/6_uebergabe/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/1_heating_vs_cooling/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/2_internal_gains/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/3_transmission_humidity/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/4_solar_radiation/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/5_systemauslegung/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/6_sommerstrategie/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/1_recap_heizen_kuehlen/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/2_regulatory_framework/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/3_bedarf_vs_leistung/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/4_system_losses/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/5_primary_energy/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/6_energieausweis/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/7_praxis_vs_theorie/generate_audio.py`

## Interrupted Generated SVG Preparation

render_config now inspects only direct *.svg files in its selected generated media_dir/t cache. It parses them with standard-library ElementTree and removes only empty or malformed XML files. Valid SVG bytes, non-SVG files, nested directories, and assets outside that text-cache directory remain unchanged. File symlinks are skipped, as are text-cache directory symlinks or Windows junctions. A disappearing file during preparation is ignored. No recursive or broad cleanup is performed.

Because compose_video and tts_pipeline.build_section_video share render_config, the same preparation applies to both without changing their interface, authored scenes, fonts, or visual assets. It does not enable partial movie cache reuse; the state-faithful --disable_caching policy remains in place.

TDD added neutral valid, empty, truncated, non-SVG, nested, outside-asset, and link fixtures before the runtime change. Both broken-SVG cases failed before the fix. The complete runtime suite then completed successfully: 15 tests in 5.524 seconds, with 14 passing and one native-symlink test skipped because this Windows account lacks symbolic-link privileges. Deterministic link-metadata tests still verify both file and directory refusal to follow links. The valid SVG is independently parsed by ElementTree and actual Manim SVGMobject, confirming it remains usable.

All cache mutations in tests were confined to temporary directories under the ticket generated/cache-test root. No live QA text cache, repair controller, or render process was inspected, cleaned, or stopped by this change. The test command exited 0 and all owned tiny test processes finished. Changed product files: video_runtime.py, test_video_runtime.py, video_runtime.tests.json.
