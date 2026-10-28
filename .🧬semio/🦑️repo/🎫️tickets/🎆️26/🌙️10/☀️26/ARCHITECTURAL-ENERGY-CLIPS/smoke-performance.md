# Scene Smoke Performance Audit

## Confirmed Problems

The original all-source smoke process (managed Python PID 40536, parent 48892) was identified by its full command line containing this ticket's `smoke.py`. Its working set reached 17,605 MB, with 16,849 MB private memory and 125.45 CPU seconds. The simultaneously running authoring suite process (PID 43904) held 11,251 MB. Other unrelated Python processes were inspected read-only and preserved. The root agent authorized termination of the owned smoke process; its exact command line was rechecked before terminating it with exit code 124 to release capacity for bounded diagnostics.

Periodic stack snapshots from an isolated Physics `Beat1_EnergieImAlltag` diagnostic showed actual forward progress, first in the meter readout at Physics line 786, then in the hour caption around line 810. Both stacks spent time inside native `manimpango.text2svg`, called through the existing font interface. The issue was therefore expensive native text conversion, rather than an infinite authored animation loop.

Instrumented actual Text conversion recorded ordinary text, Anta font, 60 px effective shaping size, NORMAL slant, 105 line spacing, empty font overrides, and an 854×480 canvas. Individual unique conversions took roughly 0.5–0.9 seconds and added about 40–47 MB. Cached repeated strings added no memory. The first 52 Text operations grew the working set from about 120 MB to 1,651 MB.

The installed ManimPango `cmanimpango.pyx` creates each layout from the shared Pango font map, then loops through every registered Windows font and calls `add_to_fontmap` for each conversion. Repeating that attachment caused the native allocation growth. A bounded diagnostic which attached each font once held about 238 MB after 646 Text operations.

Separately, setting `config.skip_animations=True` did not enable skipping for manually instantiated scenes. The installed Manim `Scene.__init__` defaults its explicit `skip_animations` argument to False and passes that value to `CairoRenderer`, independently of this config entry. The smoke harness now passes `skip_animations=True` to the real scene constructor. It continues to run the authored construction and final animation updates, preserving error detection.

## Implementation

The existing typography interface now serializes native Windows conversion and font registration with an RLock. The converter temporarily omits fonts already attached to Pango's shared map, records newly attached paths only after successful native conversion, and restores the public registration set in a finally block. Linux and macOS retain their existing native registration paths. No third-party source files were modified and no new dependencies were introduced.

The camera/canvas/profile regression that had failed in the root suite also passes after this font-map fix. No canvas semantic changes were needed: duplicate font-map attachment had affected rendered geometry as well as resource usage.

## Validation

- A language-neutral native font-map case specifies 12 unique labels and less than 128 MB additional private allocation after warmup.
- Before implementation, that regression failed with 428,249,088 bytes additional allocation.
- After implementation, all eight scene integration tests plus the previously failing real-text canvas/camera/cache profile test passed: nine tests in 13.424 seconds.
- A separate regression confirms the native registration set is restored when the external renderer raises.
- The native allocation test renders real third-party Manim/Pango Text objects in a fresh SVG directory, avoiding a cache-only false pass.

## Files

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🐍️integration.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🔣️integration.json`
- Ticket `smoke.py`, `smoke-diagnostic.py`, `pango-diagnostic.py`, and `pango-once-diagnostic.py` are retained diagnostic inputs. All generated tracebacks and JSONL logs are under the ticket generated folder for deletion after verification.

The root agent restarted comprehensive smoke verification with the explicit constructor flag and the native font fix, and restarted the full authoring suite. An isolated faulthandler run exited without producing a final scene result after a partial stack dump within svgelements; it is not counted as a passing scene. Comprehensive results belong to the root smoke report.

The font interface exports no third-party types: native renderer handles, the attachment-path set, and the installation functions are private module names. Native `registered_fonts` is restored after every conversion, including renderer failure, and is available intact to independent font enumeration. Paths are recorded only after successful native conversion; new faces are attached on their first conversion. An RLock serializes this registration/conversion boundary. No unrelated processes were terminated.

## Dry Sweep Superseded by Actual Rendering

The restarted dry sweep passed four canonical sources (intro 6.78 s; Physics beats 1–3: 115.69 s, 105.80 s, and 60.50 s). The fifth attempted source, Physics beat 4, exposed a three-line comparison caption; its English/German clause was shortened and the real two-language caption regression now passes. The test also reproduces rejection of the previous German clause. The updated integration suite passed nine tests in 20.408 seconds.

Only the owned dry-smoke managed Python process PID 53640 was stopped after exact executable/argument verification, with parent authorization, to prioritize the comprehensive five-video real-render path. That path covers all 110 canonical sources and is in progress. A complete pass is not claimed here.

Remaining per-scene text work comes from real SVG shape parsing: Manim Text defaults to use_svg_cache=False, and caption wrapping measures growing word prefixes through actual shaped Text objects. Repeated live readouts also reconstruct SVG shapes. The native font allocation problem is fixed, and parsed-SVG caching is a potential future optimization; no further typography runtime change was made during the real-render run.

## Final Caption Audit

The follow-up caption audit finished all 110 canonical sources and 1,652 EN/DE clauses. All 43 discovered overflows were repaired; the postcheck has zero remaining overflows. All 39 changed-clause cases pass original caption_bar checks in both languages and reproduce the old failures. See `caption-audit.md` and `caption-regression.md` for the complete source/key list and validation. All source edits are stable; comprehensive actual-video verification remains with the root task.

## Caption Count Reconciliation

The portable contract contains **39 canonical scene/key cases and 44 previous-language strings**; five keys have both EN and DE previous strings. This is **43 overflows from the all-source caption sweep plus one earlier Physics comparison failure**. It is not a claim that the sweep found 44 failures.

The retained records establish the partition: `caption-repairs.json` contains 42 unique language-specific sweep repairs; the separately registered ventilation `Beat3_LatenteLast` / `dx` / EN case is the 43rd. The remaining previous string is Physics `Beat4_Waermepumpe` / `compare2` / DE, whose earlier three-line failure is still recorded in `🗑️generated/smoke/results.json`. Every retained repair's previous text matches its portable case. The original caption oracle has validated all 44 previous strings across the 39 keys.

The later **Opaque envelope** qualifier restoration changes one English narration datum and its matching portable case/repair input. It adds no new overflow occurrence or canonical key. The targeted original caption oracle check establishes the restored wording still fits.
