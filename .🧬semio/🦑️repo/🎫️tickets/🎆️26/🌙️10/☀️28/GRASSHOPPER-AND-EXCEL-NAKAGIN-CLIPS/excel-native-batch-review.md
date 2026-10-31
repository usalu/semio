# Excel Native Batch Isolation Review

A sequential runpy batch can retain immutable geometry/font/module caches without changing the existing per-scene assertions or result directories. The inspected Excel lifecycle uses a new probe Scene for every pair; native_runtime patches only Manim Scene.play/wait inside context managers, environment language/audience use patch.dict, and rendering configuration uses tempconfig. All restore on exit. ClipScene.present resets voice language and caption state for every scene. No proven cross-language mutation defect was found in those paths.

Two concrete harness hazards need explicit handling:

1. **Cached code versus current disk:** importlib.import_module retains scene and shared implementation modules across runpy calls. If source changes between pairs, a later start/end map can fingerprint the new file while executing the old cached class. Freeze and verify a full-batch baseline union of all scene/shared/config/font/harness dependencies and runtime identity before every pair, stopping the batch on any change. Do not selectively reload modules and keep a mixed cache. Per-pair endpoint maps and the normal ledger remain required.
2. **Cached temporary directory:** validate-scenes.py updates TMP/TEMP/TMPDIR per pair, but tempfile caches its selected directory after first use. Reset tempfile.tempdir to None before each invocation so it resolves after that invocation sets its new output environment, or set the precise pair output explicitly. Keep previous pair directories until all cached references are no longer needed.

Use fresh per-invocation argv and keep the selected individual scene/language/output records. Catch only successful SystemExit(0); abort on a failed invocation. Release caught traceback objects and run gc.collect between invocations to reclaim cyclic Scenes/updaters/Cairo objects while retaining intentional bounded geometry caches. Existing runtime patches/configuration contexts must remain intact. Full contracts may run first and warm caches, provided the same immutable batch baseline is retained throughout.

This is a ticket-driver review; no production or native assertion source was changed. Native remains on HOLD until the renderer order/sentinel gates are green.
