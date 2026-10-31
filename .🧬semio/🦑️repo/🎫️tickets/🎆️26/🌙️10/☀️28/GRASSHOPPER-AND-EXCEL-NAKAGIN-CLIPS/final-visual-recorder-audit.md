# Final Visual Recorder Source Audit

Read-only audit of validate-final-visuals.py, scene-provenance.py and final-capture.py while root owns contracts39565/capture36769. No recorder/source changes or duplicate capture were made.

The manifest explicitly includes the shared model JSON/schema and recursive visual/occlusion portable JSON cases. It adds every actually loaded repository module, callable zero-required-argument source_files manifests, actual Manim config files, selected tests/capture scripts and harness/verify/provenance files. Recursive static literal import closure supplements loaded modules before execution. Source identity is semantic AST/canonical JSON for those formats and byte SHA256 for other assets; runtime records interpreter/executable/platform, package versions and renderer/config values. Nonempty exact before/after map equality plus runtime equality is required for exit0. Newly discovered inputs after execution fail closed rather than silently disappear.

Contracts mode records per-test success/failure/error and subtest errors, with failfast. Capture wraps final-capture.py as one unittest; that helper sequentially invokes five retained capture scripts. Exceptions prevent success. The recorder is scoped to source/runtime stability and does not itself assert all eight expected output-image names/counts or retain an image hash manifest. Root should separately inventory/hash the eight final images after terminal success before final visual acceptance; this can be done without changing the live recorder.

No concrete input-closure contradiction was found in the current source review. This does not assert that the in-flight jobs succeeded, that every gallery is current, or that native scenes passed. Terminal result maps and actual images remain required.

## Image Inventory Companion

Root prepared inventory-final-images.py for invocation only after the owned capture reaches terminal acceptance. Independently read its exact eight-name list, PNG signature/IHDR checks,1920×1080 dimensions, mtime at/after capture start, image byte sizes/SHA256 and capture success/unchanged/tests1/nonempty-equal-input guards. This closes the identified output-binding limitation once it executes successfully. It binds the recorded capture snapshot; later production writes would still require current-source verification. No inventory result or final image acceptance is asserted before that terminal invocation. Older same-name images are explicitly excluded from this final review.
