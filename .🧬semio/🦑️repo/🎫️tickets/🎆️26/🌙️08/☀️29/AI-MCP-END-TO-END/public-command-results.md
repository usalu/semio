# Public Command Results

The previous private channel retained only prepared operations and warning strings. Genuine guest command results were discarded before reaching the MCP client. The public action report now preserves those results while keeping them clearly marked as untrusted data.

## Semantics

- A real nonapplying preview result appears under `PreparedActionReport.preview.untrusted`.
- The prepared handle retains that exact envelope. A genuine read-only computation with no operations returns its result without opening a transaction or inventing an undo step.
- An actual committed result takes precedence over a preview result and appears in `InvocationReport.untrusted`.
- Idempotent invocation replay returns the same retained result.
- The attached frontend continues to defer applying commands until commit; it never fabricates preview output by executing the command early.
- Absence of a result is represented by null. False, scalar and object results remain meaningful values.

The envelope uses the existing untrusted-content format with source `capability-output`. Its hash pins the serialized result, and attribution is explicitly unknown rather than guessed from the invoking principal. The notice is available in English and German. The report and preparation schemas declare the nullable envelope; the preparation preview is now a closed record of operation counts, warnings and result.

## Ownership

The output contract lives under the MCP schema owner. Foundation report types include that schema directly, avoiding a foundation-to-schema dependency cycle. The dispatcher wraps only real channel results. Native and frontend channel implementations supply those results through the existing emission and committed-result frames.

Root changes cover the report schema, untrusted-content source/author/notice, dispatcher propagation, prepared-handle payload, report fixtures and command-result regressions. The bridge executor owns the raw frame carriers, frontend settlement propagation and mirror projection.

## Verification

The neutral corpus covers a canary string, an object result and false. Independent Node crypto pins the expected hashes; the first-party schema validator and AJV agree on valid and falsely attributed envelopes. The public mirror law initially failed because the result field was missing, then passed after projection: 2 tests, 19 assertions.

The native dispatcher regression exercises actual preparation, commit, no-operation results and idempotent replay over the typed channel fixture. It remains pending the native compiler gate. The bridge production-route suite passes 24 tests and 196 assertions, including output preservation and first-party generated-schema versus AJV agreement. The native schema emitter's byte-exact mirror check remains pending.
