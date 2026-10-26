# Teaching Pipeline Portability

The transferred pipeline is self-contained. All 25 section builders use the local rendering/composition interface. All 26 audio generators import local speech helpers. Root speech synthesis uses Python's standard HTTP, WAV, filesystem, and concurrency libraries; it imports no external project, HTTP client, dotenv package, or Manim scene types.

Speech is optional. Without an explicit TTS_API_KEY or OPENROUTER_API_KEY, synthesis returns a skipped result, makes no network requests, and creates no clause directories. Existing shell TTS_MODEL and TTS_BASE_URL settings are preserved. Credentials are never requested, inferred, or generated. No paid speech request was made during validation.

Media execution uses the active Python interpreter for Manim and the shared FFmpeg executable interface. Section outputs use short deterministic section IDs beneath SEMIO_TEACHING_OUTPUT_ROOT, preserve every authored scene name/order, and fail if rendering or muxing fails. Audio padding preserves the longer stream and normalizes frame rate before extending the final frame. FFprobe is used when available; bundled FFmpeg metadata supplies duration when FFprobe is absent.

## Test-Driven Evidence

The language-independent pipeline-cases.json and section-build-cases.json contracts are exercised by test_pipeline.py. Before changes, the baseline produced 106 portability failures and four import/API errors. An additional no-credentials test first failed because empty clause directories were created, then passed after the early skip. The bundled-FFmpeg duration test first failed because FFprobe was mandatory, then passed with the metadata fallback. The mux fixture exposed frame-padding quantization at the input frame rate; frame-rate normalization corrected the output duration.

Current checks: 11 unittest tests pass (final run: 0.956 seconds). They cover imports without Manim/httpx, forbidden machine paths/imports, all 25 section configurations, subtitle contracts, optional synthesis, HTTP request/output bytes, explicit endpoint configuration, WAV measurement, FFmpeg-only video duration, silent/narrated muxing, and full concat decoding. WAV and video measurements are independently compared against PyAV; NumPy creates the sample waveform. All 26 audio generators also import successfully with real Manim without synthesis. An extended command test exercises every trace-mode implementation with rendering intercepted, verifies the active Python interpreter and VO_TRACE environment, and exposed/fixed the high-quality trace variants before final validation. Synthetic mux outputs decode completely with PyAV, retain narration longer than the video, and concatenate to the exact expected 105 video frames.

A real minimal section build also passed at 1920×1080 and 60 fps through Manim, silent mux, and concat, with all 30 frames independently decoded.

The complete authored heating video is rendering separately at 854×480 and 15 fps. Parent coordination and video_portability own continued render scheduling, wrapper adoption, and final whole-movie evidence.

## Files

- `🎓️teaching/energy/demand/1_physical_fundamentals/build_full_video.py`
- `🎓️teaching/energy/demand/1_physical_fundamentals/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/1_warum_lueften/build_full_video.py`
- `🎓️teaching/energy/demand/2_ventilation/1_warum_lueften/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/2_luftmenge/build_full_video.py`
- `🎓️teaching/energy/demand/2_ventilation/2_luftmenge/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/3_waermestrom_luft/build_full_video.py`
- `🎓️teaching/energy/demand/2_ventilation/3_waermestrom_luft/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/4_natuerliche_lueftung/build_full_video.py`
- `🎓️teaching/energy/demand/2_ventilation/4_natuerliche_lueftung/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/5_mechanische_lueftung/build_full_video.py`
- `🎓️teaching/energy/demand/2_ventilation/5_mechanische_lueftung/generate_audio.py`
- `🎓️teaching/energy/demand/2_ventilation/6_uebergabe/build_full_video.py`
- `🎓️teaching/energy/demand/2_ventilation/6_uebergabe/generate_audio.py`
- `🎓️teaching/energy/demand/3_heating/1_introduction/build_full_video.py`
- `🎓️teaching/energy/demand/3_heating/1_introduction/generate_audio.py`
- `🎓️teaching/energy/demand/3_heating/2_conduction/build_full_video.py`
- `🎓️teaching/energy/demand/3_heating/2_conduction/generate_audio.py`
- `🎓️teaching/energy/demand/3_heating/3_convection/build_full_video.py`
- `🎓️teaching/energy/demand/3_heating/3_convection/generate_audio.py`
- `🎓️teaching/energy/demand/3_heating/4_internal_heat_gain/build_full_video.py`
- `🎓️teaching/energy/demand/3_heating/4_internal_heat_gain/generate_audio.py`
- `🎓️teaching/energy/demand/3_heating/5_solar_heat_gain/build_full_video.py`
- `🎓️teaching/energy/demand/3_heating/5_solar_heat_gain/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/1_heating_vs_cooling/build_full_video.py`
- `🎓️teaching/energy/demand/4_cooling/1_heating_vs_cooling/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/2_internal_gains/build_full_video.py`
- `🎓️teaching/energy/demand/4_cooling/2_internal_gains/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/3_transmission_humidity/build_full_video.py`
- `🎓️teaching/energy/demand/4_cooling/3_transmission_humidity/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/4_solar_radiation/build_full_video.py`
- `🎓️teaching/energy/demand/4_cooling/4_solar_radiation/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/5_systemauslegung/build_full_video.py`
- `🎓️teaching/energy/demand/4_cooling/5_systemauslegung/generate_audio.py`
- `🎓️teaching/energy/demand/4_cooling/6_sommerstrategie/build_full_video.py`
- `🎓️teaching/energy/demand/4_cooling/6_sommerstrategie/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/1_recap_heizen_kuehlen/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/1_recap_heizen_kuehlen/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/2_regulatory_framework/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/2_regulatory_framework/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/3_bedarf_vs_leistung/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/3_bedarf_vs_leistung/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/4_system_losses/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/4_system_losses/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/5_primary_energy/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/5_primary_energy/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/6_energieausweis/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/6_energieausweis/generate_audio.py`
- `🎓️teaching/energy/demand/5_energy_balance/7_praxis_vs_theorie/build_full_video.py`
- `🎓️teaching/energy/demand/5_energy_balance/7_praxis_vs_theorie/generate_audio.py`
- `🎓️teaching/intro/generate_audio.py`
- `🎓️teaching/tts_pipeline.py`

## Final Authoring Cache Policy

The shared section builder and all 20 existing audio trace modes explicitly disable Manim partial caching. All 26 audio generators import, and every trace command is intercepted and checked without speech synthesis. The complete pipeline suite now passes 12 tests; the 26-import/20-command check also passes. Full test-driven evidence and the 21 touched product files are recorded in cache-fix.md. The six synthesis-only generators require no rendering-command change.


## Canonical Taxonomy Follow-up

All56owned wrappers/builders/generators now use canonical namespace imports and facet paths; authored order/narration contracts are preserved. See taxonomy-pipeline.md for red/green evidence, complete safe import/help/dry-run coverage and coordinated chapter-scoped trace output.
