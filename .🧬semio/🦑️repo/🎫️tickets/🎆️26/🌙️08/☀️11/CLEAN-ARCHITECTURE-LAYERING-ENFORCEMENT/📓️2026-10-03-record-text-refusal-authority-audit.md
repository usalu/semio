# Record Text Refusal Authority

The actual controlled Record text reader maps NativeDecodeControl and RecordSpecProducer refusals into TextError.message. The output emitter similarly maps NativeEncodeControl errors into TextError after its String-returning internal chains. Those conversions currently erase the lower ValueRefusalKind. PackRecord then calls text parse/print and further maps their Display output into String, so merely changing the schema-producer trait would leave an intermediate erasure.

## Current Owner Graph

Diagnostic directly depends on Value. Value does not depend on Diagnostic. Therefore Diagnostic may own typed text-error authority using the actual ValueRefusalKind without a new upward cycle. Current TextError owns only message, span, and expected. Its encoded value schema and controlled transport reflect those three fields. Syntax and limits constructors currently select that same undifferentiated identity.

## Proposed Canonical Boundary

The preferred design is a mandatory kind on the actual TextError, mandatory explicit kind in its constructor, and an explicit retained-value-error constructor that carries the actual kind and message into the source span. All syntax call sites would declare InvalidValue; allocation, depth, work and cancellation sites retain their concrete authority. The schema and original serialization fixtures must be authored jointly with exact caller argument bindings. No prose classifier, compatibility constructor or From<String> is appropriate.

A distinct RecordError with Malformed(TextError) and ValueRefusal(ValueError) can preserve the control error without changing Diagnostic, but adds a second error identity throughout callers. Parent architecture review has been requested. No text-error production source is changed in the current native-held epoch.

## Exact Candidate Capture

The complete current TextError textual candidate capture contains 107 Rust files and 9962161 bytes. Each file records its exact length, SHA256, and whole source in text-error-refusal-before.json. These are source candidates, including tests and documentation, rather than admitted API callers. Canonical Rust syntax/module/provider admission must distinguish actual owned calls before authoring a source cut. The current typed Value/JSON/Diagnostic production epoch remains held for complete native proof, followed by the Record test-only missing-boundary RED.

## Adopted Owner Design and Scope Correction

Parent selected the actual TextError kind floor, with mandatory kind in both new and expected and explicit from_value_error(error, span). Diagnostic-to-text conversion also requires an explicit kind argument. The original 107-file census covered Framework: its attempted alternate service path did not exist. A separate higher census now adds 850 current Rust candidate files (14421916 bytes) from the actual S, Hub, client, native-test and root-schema owners. The immutable initial capture remains unchanged. This yields 957 source candidates, still requiring actual syntax/provider admission. Diagnostic agent owns the type/schema/TS/controlled serialization and original Diagnostic fixture binding. This lane owns actual lower lexical/Record/compiler/higher/S callers and struct literals after the current complete native epoch releases.
