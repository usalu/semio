# Publication Wire Source Review

Source-only review; native tests were not run.

## Concrete refusal allocation defect

The adapter itself uses borrowed values, fixed stack cells and literal errors. However its reused JSON sink (`pack/json/encode/borrowed`) constructs ValueError::new on oversized output and collection item-policy refusal. ValueError::new converts prose into an owned String; it is not equivalent to ValueError::literal. Consequently the native adapter test asserting zero requested/released heap bytes for output length 32 or maximum_items 0 should fail on these paths. The allocation originates in the shared writer, so the clean repair belongs to its static refusal sites rather than catching/converting the owned error after allocation. Runtime owner and root were notified.

## Payload and custody checks

UiDirtyScope Partial has kind plus five booleans and includes windowBodies/panelBodies only when nonempty, matching both actual serde and ToValue annotations. Full/None kind strings, lane receipt camelCase keys, accepted tuple and download tuple/null encoding match the neutral shapes. Node paths borrow only producer strings and slices for the duration of the synchronous call. No reference is stored or returned beyond the source lifetime. No runtime dependency or replacement control is constructed in production adapter code.

The sink updates written after each accepted output copy. Raw output updates written before control.advance, so an advance refusal still reports the actual physically copied prefix. Checked bounds prevent overwrite, and the 129-byte source/65-byte output test expects the accepted 64-byte chunk correctly. JSON output similarly retains its exact accepted prefix if traversal, policy or cancellation refuses. These are synchronous prefix writes, not resumable continuation cursors; the caller must retain input/intent and account for returned written bytes even on error. NativeEncodeControl source progress does not itself equal escaped output-copy bytes, especially punctuation and escaped text. The result exposes written separately and does not claim a RetainedCloneProgress receipt.

## Native-test scope

Tests compare actual source views with first-party JSON and independent serde_json, observe heap around writes only, cover refused output/depth/items and cancellation, and cover raw accepted-prefix refusal. They do not execute physical page birth, producer intent custody, original forwarded-control binding or lifecycle admission; those remain separate production/native obligations. No other concrete source defect was found in this bounded review.
