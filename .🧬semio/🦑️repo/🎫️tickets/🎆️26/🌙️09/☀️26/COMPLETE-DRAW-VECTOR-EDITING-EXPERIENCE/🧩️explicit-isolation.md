# Explicit Group Isolation

## Model and Editing

Added the authored `isolation` boolean to groups. Its semantic default is pass-through (`false`), represented compactly by omission; `true` is preserved in snapshots. Scene projection now emits a compositing scope for an explicitly isolated group even when opacity is one and blend mode is normal. Existing canvas, SVG and PDF scope consumers therefore receive the distinction.

Added the schema-first `set-group-isolation` semantic mutation, its Rust/TypeScript mutation/diff/inverse surfaces, canonical before/after/diff/inverse fixtures, aggregate schema/type registrations and native module registration. Sparse diffs apply isolation only to groups. Inspector field parsing validates the boolean and group target before producing that mutation. The existing multi-layer command keeps the edit atomic and undoable.

Owned-state integration carries isolation through retained group cloning, observes it in snapshot and mutation digests, retires the mutation's target string, resolves its group target and applies it in the retained candidate. Native tests cover mutation/whole-snapshot codecs, invalid targets, candidate ownership, digest distinction, projection and one-entry undo/redo.

The inspector exposes a localized group-only isolation toggle. Mixed boolean fields use an empty select with the localized Mixed placeholder and On/Off choices; a shared selection therefore no longer displays the first layer's boolean as though every selected layer agreed. Lock handling remains under the existing selection-wide disabled rule. Native EN/DE inspector tests cover common, mixed and locked states.

SVG import now authors explicit isolation rather than refusing an opaque normally blended isolated group. Invalid isolation values remain errors. Ungroup refuses explicit isolation because removing that boundary can change child blend results; the separate effect-resolution workflow remains unfinished.

## Executed Evidence

- Red run 32139 (`tests-isolation-red.txt`) failed before the new TypeScript mutation existed, as intended; its catalog assertion also reflected the newly added contract.
- Run 57065 (`tests-isolation-current.txt`) passed 272 tests / 148,548 assertions, with one native-PDF oracle skipped; 50 field-patch schema cases and the 36-command publication audit passed.
- Run 94255 (`tests-isolation-final.txt`) also passed 272 tests / 148,548 assertions with the same explicit skip and audits.
- The shared compositing corpus now has 38 cases. The new fully opaque isolated group contains a multiply child over a yellow external backdrop: correct isolation preserves blue; pass-through would produce black. Canvas/SVG pixels and editable SVG round-trip pixels pass against the independent Sharp fixture reference.
- Additional snapshot/diff guard checks are running in 16071 (`tests-isolation-guards.txt`).

Native run 5483 and component build 93519 remain live and were polled this turn. They predate these source changes; inspect their terminal results and coverage before claiming native verification. No browser actions occurred. A malformed argument in a newly copied native fixture test was caught by source review and corrected before any native result was claimed.

## Remaining Verification and Scope

Finish native compilation/tests, then generate the real native PDF corpus and run the PDF.js/Sharp oracle as described in the PDF note. Build and activate the updated component, verify the isolation toggle, mixed boolean choices, undo and SVG import in the browser. PNG/native renderer parity, all sixteen blend modes in inspector choices (currently six), text/image workflows, progress/cancellation and the rest of the acceptance plan remain required. The preceding goal turn was concrete blend/round-trip progress; this turn adds the actual authored isolation setting without reducing the full objective.


## Final Guard Run and Native Diagnostic

Run 16071 passed 273 tests / 148,561 assertions / 28 files, with the native PDF pixel oracle explicitly skipped and both audits passing. Native 5483 then terminated with exit 1 after 39m57s, before any tests ran. Its complete Cargo fingerprint diagnostic identifies the SVG serializer return at line 155: the shared `write_svg_xml` API now returns `Result<String,String>`, so wrapping it in `Ok` produced the wrong nested type. The leaf now returns that result directly, propagating serialization failures. The diagnostic is retained under `🗑️generated/native-compositing-diagnostic.jsonl`.

A fresh native run will include all current isolation/PDF sources and set the PDF fixture directory to this ticket's `🗑️generated/pdf-compositing`. Build 93519 remains active. The earlier statements that 5483 was live describe prior observations and are superseded by this terminal result.

Fresh native handle: **17529**, log `tests-native-isolation-pdf.txt`, full Draw library suite, with `SEMIO_DRAW_PDF_ORACLE_DIRECTORY` pointing at `🗑️generated/pdf-compositing`. Poll this handle; do not restart 5483, which is terminal.
