# Canonical Teaching Pipeline Wiring

Completed all delegated work: five complete-lesson wrappers, 25 composition builders and 26 speech generators now use the canonical layout and direct `import_module` namespace imports. They discover the workspace through `🧰️framework` and add only that root to Python import discovery. Shared imports use `🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring`. No ASCII module aliases or compatibility layer remain.

Builders reference each chapter's `🐍️.py` and `🔊️speech` audio, including the canonical institutional intro. Existing scene order, composition output names and narration remain unchanged. Timing manifests use `⏱️timing/🔣️.json`; default traces use `📍️trace/🔣️.json`. All six synthesis-only generators support `--help` without synthesis.

The 20 trace commands retain `--disable_caching` and use shared `render_config`. With an explicit output root, media uses `trace-media/<chapter-hash>` and records use `trace/<chapter-hash>/<beat>/🔣️.json`. The hash is the first 10 hex characters of SHA256 of the canonical chapter path relative to the workspace. The shared-helper owner implemented the matching writer and directory reader, separating chapters that both contain `Beat4_WinterUndSommer`.

## Test-Driven Validation

The retained language-neutral `taxonomy-pipelines.tests.json` locks module inventory, canonical imports, protected wrapper class/playlist and generator BEATS/NARRATIONS AST digests, and duplicate trace-owner cases. Initially the canonical-source check failed for all 56 modules; the duplicate-owner test also failed before scoping the paths. After fixes:

| Check | Result |
| --- | --- |
| Canonical taxonomy and protected authored ordering/narration | 4 tests pass in 4.964 seconds |
| Existing pipeline suite, including independent NumPy/PyAV duration and tiny real FFmpeg mux/concat comparisons | 12 tests pass in 1.295 seconds |
| Generator imports, help and missing-key synthesis runs | All 26 pass |
| Intercepted configured trace commands | All 20 pass |
| Composition builder dry-runs | All 25 pass |
| Complete-wrapper help entries | All 5 pass |

The combined source smoke finished in 6.336 seconds. Network access was patched to reject TTS requests; no audio was produced. The shared pinned environment was provisioned once under generated/v: Python 3.12.11, Manim 0.22.0 and test-only FontTools 4.63.0. All test media, caches and logs stayed under generated. Existing pydub warnings and the narrated concat fixture's AAC DTS adjustment did not invalidate decoded-frame assertions.

Updated retained validation inputs include `section-build-cases.json`, `test_pipeline.py`, `pipeline-checks.ts`, `smoke-pipelines.py` and the new taxonomy contract/capture/check inputs. This agent did not edit authored scene bodies, the PDF, timing/trace assets, shared helpers/runtime or the git index.

## First Smoke Attempt: Preserved Failure Evidence

The first stable attempt used session 73814 and redirector PID 6892 under generated/taxonomy-render. It preserved 54 successful raw sidecars before parent-authorized cancellation. Three mechanical-ventilation beats failed Text glyph-count validation:

| Raw unit | Scene | Failing text |
| --- | --- | --- |
| 27 | Beat1_Grundtypen | erst beide Ströme geführt → Filter, Feuchte, Wärmerückgewinnung möglich |
| 28 | Beat2_Waermerueckgewinnung | Nur der Anteil 1 − η bleibt als Verlust: bei η |
| 29 | Beat3_Anlagenvergleich | volle Lüftungsverlust von 1500 W bleibt, η |

The initial interleaved-log report assigned the last string to Beat1; source caption/class matching corrected that attribution. A normal-canvas isolated probe rendered the string successfully. The shared-font owner then proved that Pango clips long trial text on a 320-pixel canvas.

Cancellation verified the exact controller identity, recorded its 31-process tree and terminated PID 6892 with its descendants. Taskkill returned 128 because some redirectors had already exited. A separate native CIM check confirmed zero surviving owned processes; session 73814 closed with exit 1. The initial PowerShell 5 parser-only failure was corrected by adding a UTF-8 BOM before cancellation executed. All successful clips, partials, logs and inventories were preserved in that separate output root.

## Fixed Full Layout Smoke: Complete

The shared owner fixed Text construction with a scoped minimum 854×480 canvas and a fixed 16-hex low-width cache namespace, restoring selected render settings afterward. Default/high profiles retain their geometry and cache identity. Its 32 native tests and independent source audit passed. Before launch this agent verified the exact frozen font helper SHA256:

`4c55b9d6a7372ce8de70a4de29d627f2093dc6fa27deedcc953f07350e7b2796`

The parent authorized a fresh run through `taxonomy-render-fixed.ts`: generated/taxonomy-fixed, session 6112, redirector PID 29216, 320×180 at 5fps, 14 workers, force and no-play. That controller completed without a handoff or restart and closed with exit 0. Its log timestamps span 2026-10-26T08:50:17.979Z to 09:23:48.862Z, approximately 33 minutes 31 seconds.

All 117 raw MP4/sidecar pairs, 31 ordered logical MP4/sidecar pairs and five final movie/sidecar pairs were published. Every raw marker pins the frozen font hash. The log records 117 completion events and zero tracebacks. All three formerly failing beats succeeded. Final smoke movies:

- `Full_Physical_Fundamentals_320x180p5.mp4`
- `Full_Ventilation_320x180p5.mp4`
- `Full_Heating_Demand_320x180p5.mp4`
- `Full_Cooling_Demand_320x180p5.mp4`
- `Full_EnergyBalance_320x180p5.mp4`

The native post-completion CIM check ended successfully with zero owned descendants; the empty result is recorded as an explicit empty JSON array. Generated execution-summary.json records all raw names/sizes, logical clips, final names and ownership completion. The verifier owner was notified only after this controller closed and its owned inventory reached zero. Parent subsequently confirmed that independent verification of all 153 artifacts and all five previews passed, with the verifier also ending successfully and leaving no children.

## Ownership Released

Every build, test, diagnostic and render subprocess owned by this agent has ended. No render ownership is pending. No paid TTS request, modifying Git command or staging operation was performed. Both render attempts remain preserved for the parent's final audit and generated cleanup; the parent owns ticket closure and staging.
