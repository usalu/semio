# Source Teaching Video Audit

Source: `C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial`.
Worktree branch: `🦢niloufar/⛳wip` (read from worktree HEAD).
Recommended destination: `C:/git/semio/🎓️teaching`, preserving conventional relative names, with helpers at its root and `intro/`, `energy/demand/` beneath it.

## Canonical Transfer Scope

131 files: 86 Python, 27 Manim configuration, 11 timing/trace JSON, 5 Markdown, 1 PDF, 1 pyproject.toml.
26 section scenes: physical fundamentals 1; ventilation 6; heating 6; cooling 6; energy balance 7. Five complete chapter aggregators and five matching intro variants cover these scenes.

The source has no MP4, MP3, WAV, PNG, SVG, TTF, or OTF files, including ignored/hidden files. Existing rendered video/audio assets cannot be transferred. The sole external visual input is `intro/welfenschloss (1).pdf`. Its custom parser uses Python standard-library re/zlib and NumPy.

Exclude `.agents/` (12 skill/support files), `media/` and `rendered/` (12 generated concat text files and 2 generated Tex files). Source total is 157 files. No AGENTS.md exists under this source teaching subtree; do not copy or alter any repository AGENTS.md.

## Rendering And Dependency Closure

Canonical entry points: `full_physical_fundamentals_video.py`, `full_ventilation_video.py`, `full_heating_video.py`, `full_cooling_video.py`, `full_energy_balance_video.py`. Each supports `-q l --no-play --force`; low quality renders complete timelines at 854×480, 15 fps. Standard full chapter playlists render silent video; narration is separate and no audio inputs exist in the worktree.

Runtime closure for silent rendering is the 26 section scripts, five full chapter scripts, intro scene and PDF, shared `manim_fonts.py` and `manim_visuals.py`, Manim Community, NumPy, Pango/Cairo dependencies, and FFmpeg. No executable MathTex/Tex calls exist; the single grep match is a comment. A TeX installation is unnecessary for the canonical silent path. Fonts resolve installed serif candidates (Georgia, PT Serif, Times New Roman, Liberation Serif, DejaVu Serif, STIX Two Text, Nimbus Roman, FreeSerif, Palatino) and write `.manim_font_cache` at teaching root.

The original pyproject declares Python>=3.11 and manim>=0.18, manimgl>=1.6.1, watchdog>=4, numpy>=1.26, gtts>=2.5, openai>=1.0. Rendering only requires a subset. Audio scripts also optionally import dotenv.

Hardcoded paths require correction on transfer: aggregators construct repository-root / `tutorial`; numbered build scripts and intro/generate_audio import `backend.pipeline.compose` or `backend.pipeline.tts` from `/Users/niloufarghandehariyoon/Nowgetit/NowIGetIt`. The shared tts_pipeline has the same optional external fallback. These external sources are absent from the worktree. Preserve audio scripts as source but replace the machine-specific dependency through the repository-owned interface before claiming portable audio generation. Canonical silent aggregators are self-contained after teaching-root path correction.

Destination currently has other architecture/proctor teaching products and no existing Python/Manim/PDF counterparts, based on targeted file inventory. Preserve those products.

## Tooling Limitation

repo was absent from PATH. Located cached `repo.exe` MCP and `semio-repo.exe` CLI. CLI exposes file list/create/move/delete but no content read or arbitrary execution; GraphQL introspection is unsupported and files query yielded only a repo root link. After protocol checks, direct read-only filesystem commands were used for this audit under parent authorization. No source files or Git state were modified.

## Canonical Relative Manifest
- `energy/demand/1_physical_fundamentals/build_full_video.py`
- `energy/demand/1_physical_fundamentals/full_physical_fundamentals_video.py`
- `energy/demand/1_physical_fundamentals/generate_audio.py`
- `energy/demand/1_physical_fundamentals/manim.cfg`
- `energy/demand/1_physical_fundamentals/scene_1.py`
- `energy/demand/1_physical_fundamentals/vo_timing.json`
- `energy/demand/1_physical_fundamentals/vo_trace.json`
- `energy/demand/2_ventilation/1_warum_lueften/build_full_video.py`
- `energy/demand/2_ventilation/1_warum_lueften/generate_audio.py`
- `energy/demand/2_ventilation/1_warum_lueften/manim.cfg`
- `energy/demand/2_ventilation/1_warum_lueften/scene_1.py`
- `energy/demand/2_ventilation/2_luftmenge/build_full_video.py`
- `energy/demand/2_ventilation/2_luftmenge/generate_audio.py`
- `energy/demand/2_ventilation/2_luftmenge/manim.cfg`
- `energy/demand/2_ventilation/2_luftmenge/scene_2.py`
- `energy/demand/2_ventilation/3_waermestrom_luft/build_full_video.py`
- `energy/demand/2_ventilation/3_waermestrom_luft/generate_audio.py`
- `energy/demand/2_ventilation/3_waermestrom_luft/manim.cfg`
- `energy/demand/2_ventilation/3_waermestrom_luft/scene_3.py`
- `energy/demand/2_ventilation/4_natuerliche_lueftung/build_full_video.py`
- `energy/demand/2_ventilation/4_natuerliche_lueftung/generate_audio.py`
- `energy/demand/2_ventilation/4_natuerliche_lueftung/manim.cfg`
- `energy/demand/2_ventilation/4_natuerliche_lueftung/scene_4.py`
- `energy/demand/2_ventilation/5_mechanische_lueftung/build_full_video.py`
- `energy/demand/2_ventilation/5_mechanische_lueftung/generate_audio.py`
- `energy/demand/2_ventilation/5_mechanische_lueftung/manim.cfg`
- `energy/demand/2_ventilation/5_mechanische_lueftung/scene_5.py`
- `energy/demand/2_ventilation/6_uebergabe/build_full_video.py`
- `energy/demand/2_ventilation/6_uebergabe/generate_audio.py`
- `energy/demand/2_ventilation/6_uebergabe/manim.cfg`
- `energy/demand/2_ventilation/6_uebergabe/scene_6.py`
- `energy/demand/2_ventilation/full_ventilation_video.py`
- `energy/demand/2_ventilation/README.md`
- `energy/demand/3_heating/1_introduction/build_full_video.py`
- `energy/demand/3_heating/1_introduction/generate_audio.py`
- `energy/demand/3_heating/1_introduction/manim.cfg`
- `energy/demand/3_heating/1_introduction/scene_1.py`
- `energy/demand/3_heating/2_conduction/build_full_video.py`
- `energy/demand/3_heating/2_conduction/generate_audio.py`
- `energy/demand/3_heating/2_conduction/manim.cfg`
- `energy/demand/3_heating/2_conduction/scene_2.py`
- `energy/demand/3_heating/3_convection/build_full_video.py`
- `energy/demand/3_heating/3_convection/generate_audio.py`
- `energy/demand/3_heating/3_convection/manim.cfg`
- `energy/demand/3_heating/3_convection/scene_3.py`
- `energy/demand/3_heating/4_internal_heat_gain/build_full_video.py`
- `energy/demand/3_heating/4_internal_heat_gain/generate_audio.py`
- `energy/demand/3_heating/4_internal_heat_gain/manim.cfg`
- `energy/demand/3_heating/4_internal_heat_gain/scene_4.py`
- `energy/demand/3_heating/5_solar_heat_gain/build_full_video.py`
- `energy/demand/3_heating/5_solar_heat_gain/generate_audio.py`
- `energy/demand/3_heating/5_solar_heat_gain/manim.cfg`
- `energy/demand/3_heating/5_solar_heat_gain/scene_5.py`
- `energy/demand/3_heating/6_final_calculation/manim.cfg`
- `energy/demand/3_heating/6_final_calculation/scene_6.py`
- `energy/demand/3_heating/full_heating_video.py`
- `energy/demand/3_heating/README.md`
- `energy/demand/4_cooling/1_heating_vs_cooling/build_full_video.py`
- `energy/demand/4_cooling/1_heating_vs_cooling/generate_audio.py`
- `energy/demand/4_cooling/1_heating_vs_cooling/manim.cfg`
- `energy/demand/4_cooling/1_heating_vs_cooling/scene_1.py`
- `energy/demand/4_cooling/1_heating_vs_cooling/vo_timing.json`
- `energy/demand/4_cooling/1_heating_vs_cooling/vo_trace.json`
- `energy/demand/4_cooling/2_internal_gains/build_full_video.py`
- `energy/demand/4_cooling/2_internal_gains/generate_audio.py`
- `energy/demand/4_cooling/2_internal_gains/manim.cfg`
- `energy/demand/4_cooling/2_internal_gains/scene_2.py`
- `energy/demand/4_cooling/2_internal_gains/vo_timing.json`
- `energy/demand/4_cooling/3_transmission_humidity/build_full_video.py`
- `energy/demand/4_cooling/3_transmission_humidity/generate_audio.py`
- `energy/demand/4_cooling/3_transmission_humidity/manim.cfg`
- `energy/demand/4_cooling/3_transmission_humidity/scene_3.py`
- `energy/demand/4_cooling/3_transmission_humidity/vo_timing.json`
- `energy/demand/4_cooling/4_solar_radiation/build_full_video.py`
- `energy/demand/4_cooling/4_solar_radiation/generate_audio.py`
- `energy/demand/4_cooling/4_solar_radiation/manim.cfg`
- `energy/demand/4_cooling/4_solar_radiation/scene_4.py`
- `energy/demand/4_cooling/4_solar_radiation/vo_timing.json`
- `energy/demand/4_cooling/5_systemauslegung/build_full_video.py`
- `energy/demand/4_cooling/5_systemauslegung/generate_audio.py`
- `energy/demand/4_cooling/5_systemauslegung/manim.cfg`
- `energy/demand/4_cooling/5_systemauslegung/scene_5.py`
- `energy/demand/4_cooling/5_systemauslegung/vo_timing.json`
- `energy/demand/4_cooling/5_systemauslegung/vo_trace.json`
- `energy/demand/4_cooling/6_sommerstrategie/build_full_video.py`
- `energy/demand/4_cooling/6_sommerstrategie/generate_audio.py`
- `energy/demand/4_cooling/6_sommerstrategie/manim.cfg`
- `energy/demand/4_cooling/6_sommerstrategie/scene_6.py`
- `energy/demand/4_cooling/6_sommerstrategie/vo_timing.json`
- `energy/demand/4_cooling/full_cooling_video.py`
- `energy/demand/4_cooling/README.md`
- `energy/demand/4_cooling/vo_trace.json`
- `energy/demand/5_energy_balance/1_recap_heizen_kuehlen/build_full_video.py`
- `energy/demand/5_energy_balance/1_recap_heizen_kuehlen/generate_audio.py`
- `energy/demand/5_energy_balance/1_recap_heizen_kuehlen/manim.cfg`
- `energy/demand/5_energy_balance/1_recap_heizen_kuehlen/scene_1.py`
- `energy/demand/5_energy_balance/2_regulatory_framework/build_full_video.py`
- `energy/demand/5_energy_balance/2_regulatory_framework/generate_audio.py`
- `energy/demand/5_energy_balance/2_regulatory_framework/manim.cfg`
- `energy/demand/5_energy_balance/2_regulatory_framework/scene_2.py`
- `energy/demand/5_energy_balance/3_bedarf_vs_leistung/build_full_video.py`
- `energy/demand/5_energy_balance/3_bedarf_vs_leistung/generate_audio.py`
- `energy/demand/5_energy_balance/3_bedarf_vs_leistung/manim.cfg`
- `energy/demand/5_energy_balance/3_bedarf_vs_leistung/scene_3.py`
- `energy/demand/5_energy_balance/4_system_losses/build_full_video.py`
- `energy/demand/5_energy_balance/4_system_losses/generate_audio.py`
- `energy/demand/5_energy_balance/4_system_losses/manim.cfg`
- `energy/demand/5_energy_balance/4_system_losses/scene_4.py`
- `energy/demand/5_energy_balance/5_primary_energy/build_full_video.py`
- `energy/demand/5_energy_balance/5_primary_energy/generate_audio.py`
- `energy/demand/5_energy_balance/5_primary_energy/manim.cfg`
- `energy/demand/5_energy_balance/5_primary_energy/scene_5.py`
- `energy/demand/5_energy_balance/6_energieausweis/build_full_video.py`
- `energy/demand/5_energy_balance/6_energieausweis/generate_audio.py`
- `energy/demand/5_energy_balance/6_energieausweis/manim.cfg`
- `energy/demand/5_energy_balance/6_energieausweis/scene_6.py`
- `energy/demand/5_energy_balance/7_praxis_vs_theorie/build_full_video.py`
- `energy/demand/5_energy_balance/7_praxis_vs_theorie/generate_audio.py`
- `energy/demand/5_energy_balance/7_praxis_vs_theorie/manim.cfg`
- `energy/demand/5_energy_balance/7_praxis_vs_theorie/scene_7.py`
- `energy/demand/5_energy_balance/full_energy_balance_video.py`
- `energy/demand/5_energy_balance/README.md`
- `energy/demand/KONZEPTE_UND_FORMELN.md`
- `intro/generate_audio.py`
- `intro/intro_scene.py`
- `intro/manim.cfg`
- `intro/welfenschloss (1).pdf`
- `manim_fonts.py`
- `manim_visuals.py`
- `pyproject.toml`
- `tts_pipeline.py`
