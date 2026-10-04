# PDF Natural File Readiness — October 4, 2026

This source audit and implementation record is incomplete. No new PDF native or browser acceptance is claimed.

All ten PDF editors use the retained PDF 1.7 snapshot. The 1.7 decoder retains indirect objects and the trailer; its default writer reconciles edits onto that graph. Natural Open/Save must use this codec and preserve unrelated objects, metadata, references, and stream contents. The first implementation cut mounts the base 1.7 editor, with a neutral edit/save/reopen law and the existing independent lopdf oracle.

PDF 1.4 still has a separate page-text-only IO snapshot. Its A/X validators explicitly report warning-only schema gaps. These older IO functions cannot back the rich editor without losing content. They require canonical retained-model integration and real profile admission before their natural-file routes can be advertised.

The six constrained 1.7 subsets have existing conformance checks. Each route must call its exact checker before import publication and before output, refusing hard failures without changing the document. Merely declaring a codec would not establish profile preservation.

The 1.7 decoder supports password-aware decoding but the natural-file interface currently supplies bytes only. Incorrect passwords produce an engine refusal. A credential interaction remains a separate required feature; no password support is claimed for the pending basic Open action.

The browser transfer path now reads bounded raw Blob slices. Native PDF decoding and encoding still operate on complete buffers, so deep progress, cancellation, and cumulative allocation admission remain required before large-file acceptance.

## Test-First Cut

The neutral fixture rotates page two of the existing two-page PDF by 90 degrees. The pending native law decodes through the editor's natural-file interface, applies the existing PDF mutation, exports, compares the complete existing lopdf semantic projection with an independently rotated copy, reopens, and publishes through a whole-document mutation. It also checks exact format metadata and malformed-input refusal. The oracle is a development-only dependency; production retains first-party codecs.

The launch seed registers the focused law. Fixture include paths were verified before compilation. The production mount is deliberately still absent pending the first native gate. Native execution is serialized with the fleet's PNG/DOCX tests. A registered Cargo lock refresh first exposed a BMP oracle dependency path mistake; its owning agent is correcting that prerequisite.

## Integration Prerequisites

The first native attempt did not reach compilation: lopdf 0.44's development dependency requires bitflags 2.13, while the workspace locked 2.11.1. Exact nested oracle workspace exclusions and a selective dependency update were applied through Nx and the existing repository Cargo relay. The subsequent registered workspace lock refresh passed in 18 seconds (`stdio-lock-refresh-oct04-5.log`). The one-time repair launch was removed after use; the permanent PDF test launch remains. No external runtime dependency was added.

A second law now drives the actual registered `PluginApp::consume_media` / `produce_media` ports, editing, saving, opening under instance 2, and independent Undo/Redo histories. Inspecting this route found the editor-specific import branch still expected base64 although natural input now emits intrinsic bytes. The ownership lane repaired both shared importer branches and added its own registered-editor witness. Codec-only tests could not detect this mismatch. Native PDF attempt 2 is compiling; neither law has a result yet.

Attempt 2 subsequently stopped in lopdf's optional `time` integration: the locked `time` API has no `FormatItem::StringLiteral` used by lopdf 0.44. Both owned PDF/document oracle manifests now disable unneeded default date/thread integrations; an exact source scan found no uses of those optional date APIs in the oracle modules. Production code was not changed to accommodate an oracle.

After writing and attempting both laws, the PDF 1.7 editor now mounts paired `.pdf` metadata, the existing retained decoder/default reconciled writer, and a single SetSnapshot import event. The first attempted runs were prerequisite failures, **not** observed feature-test failures or successes. Current native/browser validation is still pending. Other PDF profiles are not yet mounted. The shared neutral matrix's PDF row now corresponds to source-mounted code.

## Current Native Boundary — October 4

Attempt 3 reached the PDF test crate and stopped at six test compilation errors: the new registered law imported media types from the plugin root rather than its app module, and two older SQLite tests used a now-private OS FromValue reexport. These direct imports are corrected; attempt 4 is active. The underlying lopdf oracle compiled with optional default date integrations disabled.

The shared registered natural importer also produced an observed runtime failure in its own focused law: its default reserved-job builder was absent. The ownership lane has now saved a default job with prepublication cancellation and an explicit small-input limit, pending native verification. Byte-accounting before a synchronous codec does not establish bounded codec work, expansion limits, or large-file support. Sole-owner teardown and absent-consumer cleanup have been sent for further audit.

A scoped diff whitespace check passed for the new PDF editor mount, registered/codec laws, package manifest, and launch seed. The wider PDF subtree includes unrelated concurrent whitespace changes; no whole-subtree clean claim is made.

## Executed PDF 1.7 Receipt

`🗑️generated/pdf-natural-file-native-attempt-7.log` passed **2/2** tests (706 outside the selected natural-file scope), with Nx cache disabled, in 2m44s. The first law independently rotates page two through lopdf and compares its full existing semantic/object projection with the first-party natural encoder output. It verifies unchanged page text and metadata, whole-snapshot publication, and malformed-byte refusal. The second law drives actual registered PluginApp Open, malformed import with unchanged state, page rotation, Save, a fresh instance-2 Open, separate source/reopened Undo/Redo histories, and fixture teardown.

Attempt 6 passed the codec/oracle law but failed the registered law because that test wrongly required malformed refusal to happen only after dispatch; the runtime correctly rejected during consume_media. The witness now admits either synchronous admission refusal or later operation refusal, while requiring the same unchanged document and history.

This receipt covers the small two-page base 1.7 fixture. It does not establish other PDF profiles, passwords, large-file yielding/expansion limits, or browser file transfer. Native tests do not replace the pending browser acceptance.

## Profile Admission Regression Staged

Root read-only review found that PDF/A, PDF/X, and PDF/E (and therefore PDF/VT through X) inspect only retained indirect objects for the security handler. The base decoder deliberately removes the security dictionary after decryption and records encryption in typed `snapshot.encryption`. Thus their existing encryption refusal can disappear after a real password-aware decode.

A third neutral native witness is now authored before production repair. It writes a real encrypted PDF, checks the trailer independently with lopdf, verifies base Open refuses without the required password, decodes with the fixture password, and requires the exact existing hard encryption codes from A/X/E/VT. A plain typed state must not produce those codes. Native execution is queued after the Media routes; this is an unverified regression witness, and those profile mounts remain unavailable.
