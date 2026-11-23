# Heating Preview Full Render

## Result

Full low-quality heating master is available:

`videos/preview/de/speaker/rendered/Full_Heating_Demand_speaker_de_480p15.mp4`

- Profile: `preview` (480p15)
- Audience: `speaker`
- Language: `de`
- Duration: ~1840.9 s (~30.7 min)
- Size: ~33 MB
- Source: assembled from all six chapter clips (`Heating_01` … `Heating_06`)

## Chapters

| Chapter | Path |
| --- | --- |
| Heating_01_Introduction | `videos/preview/de/speaker/3/v/Heating_01_Introduction.mp4` |
| Heating_02_Conduction | `videos/preview/de/speaker/3/v/Heating_02_Conduction.mp4` |
| Heating_03_Convection | `videos/preview/de/speaker/3/v/Heating_03_Convection.mp4` |
| Heating_04_InternalGains | `videos/preview/de/speaker/3/v/Heating_04_InternalGains.mp4` |
| Heating_05_Solar | `videos/preview/de/speaker/3/v/Heating_05_Solar.mp4` |
| Heating_06_FinalCalculation | `videos/preview/de/speaker/3/v/Heating_06_FinalCalculation.mp4` |

## Notes

- Pipeline `concat-only` failed at presentation-catalog capture (`render.mjs` esbuild) because generated UI tokens/icons are missing and `repo` graph host is blocked by a broken `.cache/b/t` native receiver.
- Published master is the assembled Manim content (`3/content.mp4`) copied to the canonical `rendered/` name; presentation chrome overlay was not re-applied.
- Logs: `🗑️generated/logs/render-heating-preview-retry{5,6,7}.log`
