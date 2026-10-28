# Canonical Scene Smoke Verification

## Dry Sweep Superseded by Actual Rendering

The restarted dry sweep passed four canonical sources (intro 6.78 s; Physics beats 1–3: 115.69 s, 105.80 s, and 60.50 s). The fifth attempted source, Physics beat 4, exposed a three-line comparison caption; its English/German clause was shortened and the real two-language caption regression now passes. The test also reproduces rejection of the previous German clause. The updated integration suite passed nine tests in 20.408 seconds.

Only the owned dry-smoke managed Python process PID 53640 was stopped after exact executable/argument verification, with parent authorization, to prioritize the comprehensive five-video real-render path. That path covers all 110 canonical sources and is in progress. A complete pass is not claimed here.

Remaining per-scene text work comes from real SVG shape parsing: Manim Text defaults to use_svg_cache=False, and caption wrapping measures growing word prefixes through actual shaped Text objects. Repeated live readouts also reconstruct SVG shapes. The native font allocation problem is fixed, and parsed-SVG caching is a potential future optimization; no further typography runtime change was made during the real-render run.

## Caption Count Reconciliation

The portable contract contains **39 canonical scene/key cases and 44 previous-language strings**; five keys have both EN and DE previous strings. This is **43 overflows from the all-source caption sweep plus one earlier Physics comparison failure**. It is not a claim that the sweep found 44 failures.

The retained records establish the partition: `caption-repairs.json` contains 42 unique language-specific sweep repairs; the separately registered ventilation `Beat3_LatenteLast` / `dx` / EN case is the 43rd. The remaining previous string is Physics `Beat4_Waermepumpe` / `compare2` / DE, whose earlier three-line failure is still recorded in `🗑️generated/smoke/results.json`. Every retained repair's previous text matches its portable case. The original caption oracle has validated all 44 previous strings across the 39 keys.

The later **Opaque envelope** qualifier restoration changes one English narration datum and its matching portable case/repair input. It adds no new overflow occurrence or canonical key. The targeted original caption oracle check establishes the restored wording still fits.
