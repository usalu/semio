# Late Original Class Callback Research

Confirmed separate P2: actual caller-thrown IfcEditError identity is lost in the STEP parser reference-validation catch and writer/streamed-writer source-capture catch. This does not reopen the independently closed four-file immutable receiver/one-time getter fix; the new defect is domain catch classification after correct original callback capture.

`late-class-callback-independent.ts` uses actual public parser/writer/chunk/GLB APIs, original official extruded-solid.ifc and a previously produced physical coarse-circle GLB. It throws actual IfcEditError('cancelled',...) and actual IfcStepError('io.cancelled',...) observer instances at reached noninitial phases. Immutable results: `🗑️generated/late-class-callback-independent-before.json`.

| Actual route/phase | IfcEditError identity | IfcStepError identity |
| --- | --- | --- |
| STEP parse/references completed>0 | replaced | exact |
| STEP write/capture completed>0 | replaced | exact |
| STEP streamed chunks/capture completed>0 | replaced | exact |
| STEP write/write-text | exact | exact |
| GLB read/glb-json-project completed>0 | exact | exact |
| GLB write/glb-json-bytes completed>0 | exact | exact |

All12 targeted observations reach their phase and reject; this is failure-identity/classification loss, not false successful output. Remapped parse error becomes IfcStepError with io.cancelled at physical offset5428; write/chunk at0. It has a new identity and prefixed message. The original error is not genuine signal cancellation merely because its code resembles cancellation.

Current parse catch checks instanceof IfcEditError and maps its code into io.cancelled/step.reference/step.graph. Writer/chunk capture catch similarly maps into io.cancelled/step.schema/step.graph. Those transformations are appropriate only for genuine internal graph failures. They need an original observer-failure latch gate before classification. No new parser, compatibility API, scheduler or separate original-signal cache is required. Existing captureIfcIoControlScope already preserves exact observer failure separately and should be reused coherently at the actual endpoint boundary. Stream ownership must remain an async-generator lifetime with exact original failure on iteration and ordinary cleanup.

## Neutral Test-First Followup

Freeze language-neutral rows across parser references, writer capture, chunks capture, write text and raw GLB read/write late phases. Observer values include genuine IfcEditError with cancelled/invalidSchema/danglingReference/other codes, genuine IfcStepError and undefined/number/opaque/Error values. Require same object/value, not just same code/message. Select completed>0 and completion boundaries so initial-only failure laws cannot cover the gap accidentally.

Keep distinct genuine malformed source/schema/dangling-reference domain diagnostics and genuine original AbortSignal cancellation. These should retain their current normalized STEP errors. Ensure original receiver/getter mutation denial stays closed, frozen captured controls are never reassigned and nested original callbacks are not recaptured from mutable caller fields. Failed streamed iteration must not emit a completed artifact; entered iterator cleanup and private source custody remain explicit. Reuse actual unchanged official source and controlled physical outputs; no mock programs/authority or blanket all-native claim.

Root was notified immediately. No production files were edited, no before output overwritten and all audit processes exited. Native execution/type/physical receipt/publication flags remain false. AFTER source/probe closure awaits root's frozen implementation.
