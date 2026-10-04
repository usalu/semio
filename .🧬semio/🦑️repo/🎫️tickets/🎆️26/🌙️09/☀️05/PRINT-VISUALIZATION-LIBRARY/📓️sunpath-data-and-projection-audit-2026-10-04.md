# Solar Data And Projection Admission

The current full catalogue compiler discovery found `viz-37.tex:60` rejecting `semio/viz/family/arch-sunpath/data`. The authored `sun-path-diagram` and `shadow-diagram` catalogue presets bind `data=demo-arch-sunpath`, but the architecture family exposes only computational latitude, longitude, date and hours controls and never declares that demo table. The owned schema nevertheless describes its month/hour/azimuth/altitude columns. Accepting and ignoring the data key would conceal that missing contract.

The same family declares `mode=stereographic` but its radial formula is `pole*(90-altitude)/90`, an azimuthal equidistant radius. A stereographic map instead uses `pole*tan((90-altitude)/2)`. The installed independent D3 stereographic projection and SunCalc solar-position implementation provide the geometry and astronomical oracles.

The authorized fix will keep one native implementation: optional authored solar-angle table input, computational site/date/hours input when the table name is empty, real demo rows derived through the owned solar solver, and month/daylight arc separation. This requires schema-first data admission and a corrected native projection. No external runtime dependency or silent fallback is intended.

The neutral `tests/🖼️family-containment/☀️sunpath.json` was authored before production edits. It covers both exact gallery presets, a southern-hemisphere computed day, supplied stereographic angles, supplied shadow angles and a below-horizon row. The existing registered family-containment helper now owns `compileNativeSunpaths`, with a focused `PRINT_NATIVE_FAMILY_PHASE=sunpath` route. The compiler consumes actual TeX geometry and actual published PDF pages through independent PDF.js.

The current production source freeze remains in force while the full catalogue wave gathers unrelated errors. The compiler red baseline is running through the registered Bun/Nx route; no passing claim is made before its terminal result. Parent and catalogue owners have been informed before any production change.

## Resumed Observed Red And Native Repair

The focused registered projection baseline (session 70231) compiled and published the computed-south PDF and real geometry stream, then terminated with exit 1 because the helper accidentally selected the document scenario solar although the record scenario is computed-south. The helper now explicitly consumes all document scenarios and limits its PDF title checks to the selected cases. This harness defect did not demonstrate the numeric failure; a separate independent D3 comparison consumed that actual published stream and failed with native point [44.8807224217435,21.34696022897581] versus true stereographic [43.904799613898,21.07762897770352], well beyond the fixed 0.0002 tolerance. Its console explicitly printed the actual values before throwing.

The root schema owner admitted the optional data key before production implementation. The native architecture owner now reads the supplied month/hour/azimuth/altitude columns, preserves row order, separates consecutive month and below-horizon arcs, projects with pole*tan((90-altitude)/2), and casts shadows opposite the solar bearing. Its demo table is declared by the existing solar solver for Hannover, UTC 4 through 20 at two-hour spacing on 2026-03-21, 2026-06-21 and 2026-12-21. Empty data continues to invoke that same owned solver for the authored site/date/hours. These changes add no runtime dependency or separate rendering module.

The retained neutral fixture covers exact gallery presets (27 samples each), computed southern-hemisphere angles, authored angles, below-horizon filtering, and independent month/daylight arc separation. SunCalc independently supplies astronomical values; D3 supplies stereographic coordinates and PDF.js consumes the published pages. The strengthened authored arc fixture expects [2,1,1]; the gallery seasonal fixture expects independent daylight counts [6,8,4]. Focused registered green is running and is not yet certified.

## Rendered Light Inspection

All five actual light pages 2–6 were rendered with the system Poppler at 100 DPI and viewed. The seasonal chart shows three separate daylight arcs within the horizon; the shadow chart shows rays opposite the solar directions; the computed southern arc differs from the gallery; supplied data gives one connected two-point segment and two visible isolated samples; the below-horizon sample produces no mark. The supplied shadow directions follow the authored angles. All case titles, figure borders and marks are visible and contained. Dark-theme compilation and terminal verification remain pending at this inspection.

Poppler could read a Unicode absolute PDF path but failed to write PNGs to that absolute path. Invoking it with ASCII input/output filenames while its working directory remains inside the ticket generated folder succeeded. No artifact was written outside the ticket.

Both actual compiler staging copies of the native architecture owner match current SHA-256 `7d4f4191a32852bceb324795206289e0932b3e345d5e25eb999a50d51dcb2bd9`. Current solar fixture SHA-256 is `2c422649c9bebbb53cb08047341020e0d83208c7ede0079f6860ee466c29af9b`; current shared family helper SHA-256 is `bb86458a0269f91473e7a9098659a1afd706e6cb759dd5d9c5a16a59f46d74b9`. The temporary projection-baseline wrapper and routing branch were removed from final test source before the successful current dispatch.

## Focused Final Runtime Green

Registered Bun/Nx target `@semio-tech/print:test-native-grammar -- --families-only` with `PRINT_NATIVE_FAMILY_PHASE=sunpath` (session 10795) terminated with exit 0. Its target duration was 4m 3s, caching skipped. Actual console output in each theme was `5 cases, 49 actual D3/SunCalc points, 8 PDF.js pages PASS`. Both exact gallery presets, computed southern site/day, both authored-angle modes, 69 total angle samples per theme, all 49 daylight points per theme, and month/daylight separation passed fixed-tolerance assertions. PDF.js consumed all eight pages per theme.

All five actual dark case pages 2–6 were rasterized at 100 DPI and viewed after the final compiler publication. They reproduce the correct seasonal separation, computed southern geometry, supplied isolated samples and opposite shadow bearings with clear contrasting marks and no clipping or collision. Thus ten relevant light/dark page views were inspected in total.

| Actual Published PDF | Bytes | SHA-256 |
| --- | ---: | --- |
| Light solar.pdf | 23701 | `54773900a13ea235b49d34bc4a12bacd43cbcfa2194198a37fc55ac9201e7bc9` |
| Dark solar.pdf | 23676 | `cd79f3802e16ade7e392f6f2a13bb6ed1ebd5991a0cd80b30c21cba8795a438e` |

Production owner, neutral fixture and helper remain frozen for the subsequent full catalogue replay. The focused solar pass alone does not certify the full catalogue.
