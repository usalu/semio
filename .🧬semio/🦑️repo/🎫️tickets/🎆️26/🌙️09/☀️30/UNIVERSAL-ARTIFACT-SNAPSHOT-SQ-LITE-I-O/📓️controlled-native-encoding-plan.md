# Controlled Native Snapshot Encoding Plan

## Current verified boundary and remaining gap

The live ArtifactSqliteSnapshot factory import owns a reconstructed typed snapshot, applies its exact semantic subset guard, and invokes an owner-authored expansion preflight. It then calls ordinary encode_pack_with or print_dsl between two EncodeNative checkpoints. This admits conservative maximum output and allows cancellation during the borrowed preflight. It does not establish cancellation or aggregate allocation admission inside typed to-record construction, symbol collection, quoting, document-frame encoding, compression, hashing or final envelope ownership. Existing full erased import successes do not prove that separate boundary.

The current input boundary is genuinely controlled: exact borrowed envelope, full physical Text/Pack parsing, explicit/derived typed construction and actual owner retirement. The new output work must preserve that standard, with no ordinary encoder fallback.

## Observed physical ownership

Pack value encode_document currently builds an owned symbol vector and a second HashMap of cloned symbol strings, encodes a complete owned document payload, then emits compressed frames and a manifest through PackWriter. The first-party Deflate encoder allocates its hash-head table and a predecessor entry per source byte before scanning match chains. Its current deflate function has no callback or caller-owned allocation ceiling. Header/segment/manifest hashes and envelope construction must also be audited for full-buffer operations.

The canonical Text Writer currently owns a Vec of Atom(String)/Verbatim chunks before joining them into another complete String. Quoted strings are formatted before Writer.atom receives them. Adding a checkpoint only to atom would therefore miss escaping allocation and could keep traversing the rest of the snapshot after cancellation. The controlled path must perform quoting and emission in bounded spans, use borrowed key sorting, and stop the actual traversal on refusal.

## Explicit owner API and sequencing

Add an owner-authoritative encode_sqlite_snapshot_native(&self, encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl) -> Result<IoPayload, String> with strict missing-owner refusal. Keep the old factory dispatch unchanged until real output APIs and first owner laws pass; then activate only this hook, with no ordinary fallback. Owner expansion preflight remains necessary for special domains such as Part21 decimal scale and PDF arbitrary decimal coefficients.

A public record-output helper should accept an explicitly declared envelope ID, RecordSpec producer and genuinely controlled typed record constructor. One cumulative encoding ownership control must span typed construction, physical output and final envelope. DslField/DslRecord and canonical ToValue need explicit borrowed controlled output constructors, including first-party child/reference fields and intrinsic bytes. Custom implementations default-refuse until owner-authored; no full snapshot JSON, hex or native file carrier is introduced.

Known collection stages must publish exact workloads and preserve parent stages. Every owned slot/string/octet copy is admitted before allocation. Large primitive copies/escapes and compression/hash loops publish interior checkpoints. Complete/partial generated record owners retire correctly after cancellation and errors. A separately explicit file-byte ceiling bounds final encoded output; conservative preflight and cumulative working ownership are distinct checks.

## Required genuine tests

Use a language-neutral fixture for arbitrary Unicode/NUL text, many empty collection items, intrinsic octets, full integer domains, raw IEEE words and a later invalid field. Independent Bun SQLite/Ajv queries verify exact semantic output through the resulting SQLite state. Independent base64 and compression engines verify emitted primitive payloads; independent hash validation remains enabled.

Prove typed construction late cancellation before all fields exist, quoting cancellation within a long escaped string, Pack traversal cancellation within a known collection, Deflate cancellation within a large source frame, before-allocation refusal for symbol/index/frame materialization and final envelope, and exact retirement of every completed owner. Counter-based fixture codecs must prove the ordinary printer/packer is never called by the controlled factory. A successful Binary/Text roundtrip alone is insufficient.

## Coordination

Current Note/PDF controlled input and composed schema fixture repairs take precedence. Office expansion waits for this shared output boundary, then uses a literal first-party multi-document XML graph component beside exact OPC metadata/intrinsic non-XML parts. No Office provider was claimed implemented from the initial inventory.

## Authored Output Foundation and Current Admission Stage

`value::native_encoding::NativeEncodeControl` is now an additive first-party resource owner. Exact known work, cumulative admitted bytes, scoped parent work/depth, before-allocation vector admission and 64KiB primitive-copy callbacks are explicit. `DslField::to_value_controlled` and `to_record_controlled`, `DslVariants::to_named_record_controlled`, and generated `__dsl_to_record_controlled` build explicit field plans with guarded intermediate record/list/map/tagged retirement. Custom field/variant defaults refuse. Generated output recursion currently has an explicit depth64 ceiling.

The genuine factory-interior cancellation baseline was authored first in the existing neutral native-encoding fixture and public kernel admission route. Its first attempts stopped before assertions: a stale u64 progress assumption, then generated missing-self receiver and TextError propagation in a String closure. These source errors are corrected. A fresh registered current admission run is pending; no output runtime RED or GREEN is claimed yet. The ordinary import encoder is deliberately still active until the actual physical producer and owner tests prove the replacement. The new strict `ArtifactSqliteSnapshot::encode_sqlite_snapshot_native` declaration is additive and not yet used by the factory.

The expanded neutral fixture also specifies a1024-item typed string projection with cancellation at256 and a large escaped Unicode/NUL/control-character physical Text emission. Bun SQLite/Ajv independently validates the fixture and semantic rows. The public `schema::print_controlled` entrypoint is currently an explicit TDD refusal pending the physical emission runtime RED; it does not call the ordinary printer. No production owner is opted into this unverified output path.

Root has delegated canonical `ToValue::to_value_controlled` and its derive to the Rust value owner. This lane owns the encoding resource control, DSL fields/derive, physical Text/Pack/compression/hash/envelope and final strict dispatch. Office/archive expansion follows this shared boundary.

## Physical Text Ownership Decision

The controlled emitter will traverse borrowed fields, measure its exact final UTF-8 size without owning escaped atoms, then admit one complete output allocation and emit directly. Both traversals remain cancellable. Cancellation evidence must select the second traversal by observing admitted output storage; a callback that only stops measurement would not prove interior emission. Numerical formatting uses a fixed stack buffer, quoted escaping copies each scalar directly, and intrinsic bytes stream their base64 alphabet without an intermediate encoded string.

Sorted maps and record field order use admitted vectors of borrowed references. Native recursive fields retain the explicit64-level controlled-depth ceiling. Known list/map stages count actual items, nested text stages count input UTF-8 bytes, and cumulative storage does not reset when a child stage restores its parent. The canonical Document/Inline spacing, braced record-list items, scalar IEEE words, quoted arbitrary keys and bare-preferred Text must agree with the ordinary printer; this agreement does not permit calling its unbounded lexer or intermediate chunk materializer.

The current PDF retry ended before Cargo on Nx graph admission (createNodes timeout, unresolved renderer producer and test-host dependency). The stale Writer macro failure does not establish a PDF owner regression: the current Value expansion accepts `serialize_controlled_with` and has its explicit controlled branch. No current output runtime result is inferred from those prerequisite failures.

The next public admission compile reached only two Value output hygiene errors: a real directory enum field named `control` collided with the generated resource controller binding. The Value owner now binds named fields through explicit source aliases and has authored neutral regression cases for controller/output/payload/tag names. This lane made no directory workaround. The borrowed Text emitter implementation below the still-refusing public entrypoint compiled in that attempt; output runtime remains unobserved. One current-source registered retry is active with the same native selector and budgets.

The physical Text callback regression now requires admitted storage at least the measured output length before it cancels inside the known source UTF-8 workload. This distinguishes emission from the first measurement pass. It also compares ordinary and controlled parser results and established canonical printer output. The shared literal reference corpus additionally drives explicit controlled reference/link projection, tiny pre-allocation refusal and pre-cancellation; its owner constructors still await genuine baseline execution before connection.

## Metadata Allocation Audit Still Open

The current lazy `fn() -> RecordSpec` producer owns metadata strings, vectors and boxes before the physical encoder can inspect their size. These producers describe static owner schemas, but nested records currently call them repeatedly. Typed field admission and physical emission therefore do not alone establish before-allocation admission of every lazy schema object. A complete caller-allocation claim must add an owner-authoritative controlled schema producer or borrowed immutable descriptors, with literal generated metadata estimates admitted before construction and no inferred fallback. The present emitter can account for its borrowed sorting vectors and final output; it must not claim that this separate lazy metadata allocation boundary is already controlled.

## Public Fixture Admission Corrections

The current Value hygiene repair is independently runtime green (13 native laws,9 source laws). This lane's next public run compiled the kernel library but stopped at test compilation: ambiguous `DslField`/`ToValue` methods in new Ref/Link probes, a RecordSpec keyword missing `Some`, and stored resource controllers borrowing temporary closures. The separate child fixture also assumed ArtifactRef serde and had temporary callback borrows. These tests now use explicit trait dispatch, the actual keyword type, named callbacks and literal reference fields. The next registered run is pending; these compiler corrections are not counted as the required output runtime RED.

Four new schema-first literal ArtifactChild controlled-output laws are mounted under the existing public native-admission binary, with their own neutral JSON and JSON schema at Store child/🧪️tests/🛫️encoding. They independently validate all five unrestricted literal strings through Ajv/Bun SQLite; compare genuine controlled record/field projection against the established value graph while retaining the local-only owner without copying it; exercise exact cumulative ownership equality/one-below and initial cancellation; and demand genuine interior UTF-8 cancellation inside each owned string. A fourth law covers the handwritten intrinsic ToValue boundary. Provider methods remain strict defaults until these new laws reach actual runtime RED. The root lane is still occupied by the existing native parent prerequisite build, so these laws have not yet executed.

The first current child-output compilation stopped before assertions: ArtifactRef intentionally has no serde deserializer, and three stored controls borrowed temporary callbacks. The fixture constructor now spells the four literal reference fields directly, and stored controls bind callbacks with sufficient lifetimes. These are test compilation prerequisites, not runtime RED; the four controlled output laws and strict providers remain otherwise unchanged.

## Current Physical Output Admission

The current Value output foundation is independently reported by its owner as 108/108 native regression laws, 13/13 focused native output laws, and 9 source laws/138 assertions. Public kernel invocation `native-output-public-fixture-bindings-red.log` compiled the complete kernel and integration consumer but exhausted the unchanged fundamental 15-second assertion budget before reporting any test result. This is an admission timeout, not a genuine Child or physical producer RED. The sole subsequent invocation explicitly selects the existing quick level and retains both Cargo target/build directories.

The authored physical Text implementation remains behind an explicit refusal entrypoint until its actual positive law reaches runtime. A neutral 16-string lexical corpus now covers empty/control/Unicode text, reserved tokens, full IEEE NaN words and non-hex NaN-like identifiers, with canonical printer equality, ordinary/controlled parsing and an independent Bun SQLite literal oracle. Pack controlled record and document entrypoints are explicit refusal stubs for TDD; their two new laws require byte-identical ordinary encoding, exact intrinsic octets, interior byte-work cancellation and pre-allocation tiny-budget refusal. No provider or factory uses these unverified physical entrypoints yet.

The source metadata producer allocation gap remains open. No universal output-control completion claim is made.

## Genuine First Public Output RED

The explicit existing quick-level public native invocation reached runtime: 40 selected, 15 executed, 10 passed and 5 failed before fail-fast, 16.802 seconds of assertions. All four child output laws executed and failed. Literal record and cumulative slot laws hit the missing controlled record projection, intrinsic value output hit missing controlled value encoding, and the interior literal-copy law did not reach its required work checkpoint. The fifth failure is the real Binary SQLite import boundary: ordinary encoding completed instead of canceling during output ownership. Parent was notified to bind its actual child output methods after these genuine failures.

The first run did not execute the physical Text/Pack and owned reference projection laws. A sole subsequent focused invocation disables fail-fast and selects output/reference laws under the unchanged quick budget. The failed assertion no longer dumps 131072 repeated bytes; its truth condition and interior checkpoint/ordinary-call requirements remain identical. No unexecuted law is marked RED or GREEN.
# Child Output Binding After Executed Failure

The quick public native run selected 40 laws and executed 15: 10 passed, five failed, and 25 remained unrun after fail-fast. All four `sqlite_snapshot_native_child_*` laws failed because controlled record/intrinsic projection was missing or interior string-copy cancellation was never reached. The fifth failure was a distinct binary-factory output law. This is executed assertion evidence, not a compiler failure.

Following those four failures, the handwritten `ArtifactChild` controlled `ToValue`, controlled DSL field, and controlled DSL record methods were added in the store owner. They charge the two-field frontier before allocation, copy the child ID through the cumulative control, delegate literal target fields to their owning controlled implementation, and retain iterative partial-value retirement. The controlled DSL child method still depends on the separately owned `ArtifactRef` binding. These new child bindings have not passed a subsequent native run yet. Native schema construction and physical factory output remain separate incomplete work.

## Complete Focused Output RED And First Producer Binding

Focused public invocation `native-output-all-physical-red.log` executed every selected law: 14 selected/run, 4 passed, 10 failed, 26 filtered, 2.210 seconds assertions. Parent child intrinsic ToValue already passed after its owner binding; its three record/cancellation laws reached the missing reference controlled producer. The owned reference law, both physical Text laws, both physical Pack laws and both native factory interior laws genuinely failed.

After these runtime failures, the literal reference/link/pin guarded projections and physical Text emitter were connected. Text performs controlled borrowed measurement, admits and reserves its exact result capacity before allocation, then emits under the same persistent control. A focused green attempt selects Text, references, child output and typed projection, without unimplemented Pack or factory laws. It is unverified while the invocation is running. Pack, schema metadata allocation and central output factory remain unfinished.


### 2026-10-02: Actual literal-reference and physical Text results

Nextest64bb1b3f-10e3-4579-b63d-f00bac60dce9/public route quick ran13selected laws without fail-fast:8passed,5failed,31filtered (1.359s assertions/3m12s Nx). All four parent-authored Child output laws, both literal Ref/Link output laws, generated typed record projection and long physical Text copy/output interior cancellation passed. Literal lexer-boundary Text failed on the nonhex `nan64_` identifier case; four explicit metadata tests failed as expected before producer dispatch. No Pack or full erased-import factory output test was selected, and neither is declared green.

The changed-source follow-up uses the same13-law selection. The controlled lexer now checks exact16/8hexadecimal word widths before token classification; input/output metadata producer methods invoke only their owner controlled factory inside preserved cumulative stage/depth control. The actual recursive physical law deliberately counts ordinary schema calls and is expected to expose the still-bare lazy shape references. This remains an attempt until assertions return.


### 2026-10-02: Complete physical output and owner helper proof

The public kernel quick route executed all 21 selected physical output laws successfully (Nextest e2975333-bcdf-4255-ad4e-2bf650db4931, .462 seconds assertions). It covers controlled terminal Pack, complete document framing and raw chunks, codec 0/1 equality with ordinary output, independent BLAKE3 hashes, controlled Deflate, typed table fields, raw IEEE words, physical Text, recursive metadata and literal reference construction. The separate first helper/literal-expression attempt executed 23 laws with 21 passing and two genuine failures: the deliberately missing owner output helper and arbitrary empty/reserved expression identifiers. Both were repaired.

Fresh current-source verification then executed **23/23 selected laws successfully**, Nextest **aa0153ab-bd9a-46a7-b04c-9e31be7c3d4f**, .337 seconds assertions / 26.7 seconds uncached Nx. Log: `🗑️generated/native-output-owner-helper-current.log`. The new `store::encode_sqlite_snapshot_record_native(encoding, envelope_id, RecordSpecProducer, construct, control)` carries one cumulative NativeEncodeControl through the owner's controlled metadata, controlled typed RecordValue construction, physical Text or Pack, and literal Semio envelope. Its public roundtrip law exercises both encodings, cancellation inside the known 1024-item construction stage and refusal at a one-byte ownership allowance. Arbitrary quoted expression variable/call identities now roundtrip through ordinary and controlled paths.

This proves the helper and physical producers. The erased SQLite import factory still invokes an ordinary encoder in current source; its strengthened counter/refusal/interior-cancellation laws are being run for an authentic RED before strict owner-controlled dispatch replaces that branch. Universal artifact output coverage remains incomplete until each owner binds and verifies its explicit hook.


### Strict erased output dispatch

The strengthened factory laws executed all three selected cases and genuinely failed: ordinary encoder count was one instead of zero, and both Binary/Text returned success without the requested interior output cancellation. Following that RED, the erased import factory replaced ordinary `encode_pack_with`/`print_dsl` with the strict owner hook `encode_sqlite_snapshot_native`. Semantic subset diagnostics and the immediate owner retirement guard remain in effect.

Fresh verification is GREEN **3/3**, Nextest **099aaf0f-1c91-4096-b8d6-c841d48ebc7e**, .330 seconds assertions / 46.8 seconds uncached Nx (`🗑️generated/native-output-strict-dispatch-current.log`). It verifies zero ordinary encoder calls, bounded successful output, pre-encoding refusal/cancellation and real cancellation after the first 64KiB of a 128KiB ownership stage for both encodings. A further explicit neutral case distinguishes admitted preflight from an absent controlled owner encoder: preflight alone cannot satisfy native output.

The retained snapshot fixture now declares a controlled scalar encoder, with pre-admitted constant bounded integer Text formatting and exact eight-byte Binary ownership. The complete public admission/lifecycle regression is running; its success is not inferred from the three-law dispatch result. Artifact owners lacking the controlled hook now refuse erased import. Their previous ordinary-output passes are historical, while direct typed semantic SQLite APIs retain their separate guarantee.


### Full public admission regression after strict activation

The fresh full public quick route executed **52/52 laws successfully, zero skipped**, Nextest **4e542712-2576-48ad-81c3-3dba06bd7eb8**, 3.103 seconds assertions / 18.9 seconds uncached Nx. Log `🗑️generated/native-output-strict-dispatch-full-current.log`. This includes controlled metadata/input/output, compressed input ceilings, complete physical Pack/Text framing and hashes, exact literal references, cumulative ownership, strict input/output dispatch, and retained-owner lifecycle on success/cancellation/refusal. The new neutral case explicitly admits the preflight but lacks the controlled encoder; it refuses without calling ordinary output.

The full DSL regression is being run independently for the quoted expression identity correction. Artifact-specific Note/DXF/PDF/JPG/GIF and remaining archive/Office hooks remain unfinished. Physical depth limits and cumulative owned-byte admission are explicit; this result is not an unrestricted-depth or universal owner coverage claim.


### Broad DSL regression prerequisite propagation

The broader registered kernel DSL test attempt stopped before assertions after six minutes, while the dedicated public admission route had already executed 52/52. Its library-test compilation exposed output fixture names imported only through the public integration harness, and ordinary schema-producer invocations retained in DSL/store tests. The output fixture now explicitly imports first-party DSL types. Narrow ordinary test calls use producer.ordinary; IEEE table metadata and the composed renamed-link metadata now declare real controlled producer factories rather than restoring bare-function compatibility. Files: DSL viewport pose, boxed-field, unit and IEEE tests; Store unit and composed-pack-schema tests; the owned native output test module. Log: `🗑️generated/native-dsl-quoted-expression-regression.log`. No broader DSL runtime success is claimed.

The measured Text file-ceiling feature has a schema-first neutral maximumFileBytes=128 case. It requests a file ceiling separate from a large cumulative ownership allowance and requires refusal before output allocation. The new explicit physical printer ceiling parameter is an intentional unimplemented RED-stage seam until its registered native law executes. The public owner helper signature remains unchanged.


### Measured file ceiling RED and bounded measurement frontier

The explicit Text file ceiling law genuinely failed after reaching assertions: Nextest 0845f693-304a-4142-881c-5e59fa6332b2, one selected/run failure, .217 seconds assertions / 1m16s uncached Nx. It requested maximumFileBytes=128 with a much larger independently admitted RAM allowance; the printer returned a body instead of refusing.

The controlled Text emitter now carries an independent maximum_output_bytes through measurement and emission. Its raw-byte accumulator stops as soon as the physical ceiling would be exceeded, before allocating the owned output String. The generic owner helper counts the declared Semio framing without allocating an identity token and subtracts that exact prefix before controlled Pack/Text output. Final file checks remain in place.

A full changed-source run executed all 52: **51 passed, one failed**, Nextest 00801fbd-2395-4fdc-b36a-571d0b094ebc, 2.548 seconds assertions. The byte-ceiling refusal already succeeded; the remaining assertion incorrectly demanded zero total ownership, despite the legitimate eight-byte borrowed field-order measurement frontier. The law now requires the admitted measurement frontier to stay below the tiny 128-byte file ceiling, excluding allocation of the large output body. No production cancellation or byte ceiling was weakened. Exact complete-envelope positive admission and just-below-file-size refusal passed for both Binary and Text in that full run. A fresh full run verifies the corrected distinction.


The complete current-source file-ceiling refinement is **GREEN 52/52, zero skipped**, Nextest **526f4378-109d-4ab9-bd48-2481e1716a4b**, 1.698 seconds assertions / 17.7 seconds uncached Nx. Log `🗑️generated/native-output-measured-file-ceiling-frontier-current.log`. It verifies independent physical Text ceilings before large ownership, the bounded borrowed measurement frontier, exact full-envelope admission and one-byte-too-small refusal for Binary/Text, and all existing controlled admission/lifecycle laws. Public owner helper ABI is unchanged. The broader DSL regression is being retried after the separately recorded library-test producer/import propagation fixes.


### Follow-up: retire the approximate preflight obligation after owner-controlled output coverage

The current strict factory retains the earlier approximate owner preflight before the newly verified exact controlled encoder. Several earlier guards re-project the full snapshot into relational tables and multiply row/string budgets conservatively. They therefore perform an additional controlled traversal and can refuse a file even when the exact typed/physical encoder would fit its requested allowance. The clean final contract should use the genuine owner-controlled encoder as the mandatory authority, with any domain-specific expansion preflight kept inside that owner's controlled path before its dangerous operation. The obsolete universal preflight obligation should be removed only as coordinated owner work, rather than silently bypassed in this lane. Current runtime proofs retain it and do not claim that a conservative refusal is exact admission.

## Current Broad DSL Regression and Exact Output Admission Follow-Up

The registered framework kernel `test-native -- dsl::` completed all **257 selected tests successfully**, with 1143 tests filtered by selector. Nextest run `25d6c1ab-898c-4b17-b76b-011812ad24be`; assertions 216.916 seconds, Nx 8 minutes 38 seconds. Retained output: `🗑️generated/native-dsl-ordinary-fixture-declarations-current.log`. This is fresh runtime evidence for the existing ordinary/controlled grammar boundary changes; it does not establish all artifact owners' output hooks.

The earlier broad route's pre-assertion compiler failures came from ordinary-only regression fixtures retaining bare metadata function pointers after the mandatory producer migration. Their local producers explicitly refuse controlled construction; unchanged ordinary metadata callbacks are now declared under the actual producer type. These test producers do not provide any controlled capability and are separate from the runtime-proven controlled metadata/physical public admission suite.

A new schema-first `exactEncodingAdmission` neutral case declares 32768 exact emitted bytes, a 32768 caller byte/file ceiling, and an explicitly overestimated 32x legacy preflight. The independent Bun SQLite/Ajv oracle materializes the declared output, while the owner-controlled encoder admits precisely its owned buffer and retains strict ordinary-encoder refusal. The focused genuine RED is currently running. The authorized initial production removal scope is only the central factory preflight invocation; owner trait overrides and their direct tests remain until their domain checks are explicitly adopted inside genuine controlled encoders and the obsolete hook is retired in coordinated source cleanup.

### Authentic Approximate Preflight Over-Refusal

The focused registered public route executed the new exact-output law and failed as intended: Nextest `bb9e3e88-e6cb-4696-a503-c4fd73fb147e`, one selected/failed, 52 filtered, .181 seconds assertions / 1 minute 55 seconds Nx. The independent Ajv/SQLite oracle passed first; factory import then returned `relational SQLite resource limit: value bytes` because the old 32x estimate rejected a precisely bounded 32768-byte output under a 32768-byte caller ceiling. Retained output: `🗑️generated/native-output-approximate-preflight-red.log`.

The central import factory's approximate preflight invocation is now removed. It still validates and retires the exact reconstructed snapshot and invokes only its mandatory genuine controlled encoder, with no ordinary fallback. The trait signature and explicit owner/direct preflight tests remain untouched for coordinated domain-guard cleanup. The full public admission route, now 53 laws, is running for fresh verification. The explicit fixture encoder checks the file ceiling before buffer reservation; just-below-file refusal remains asserted in both encodings.

### Current Exact Admission Verification

The complete registered public quick route is **GREEN 53/53 executed, zero skipped**, Nextest `6ba338e0-8a10-45b2-b96f-4712f8f7b229`, 1.706 seconds assertions / 1 minute 51 seconds uncached Nx. Retained output: `🗑️generated/native-output-exact-admission-current.log`. Both exact-output branches now pass independently of the deliberately overestimated old preflight; one-byte-below physical file refusal is retained. All prior controlled metadata/physical framing/hash/compression, exact references, strict no-ordinary input/output, cancellation and owned retirement laws also executed successfully.

The Note owning output RED is now running through its existing registered `verify-sqlite-snapshot-native` quick route. That route retains its existing budgets and now uses `--no-fail-fast` so every selected owner law executes. The new owner output hook remains strict/unimplemented until runtime failure; no Note fresh output or guest GREEN is inferred from the shared foundation.

## JPG controlled output law preparation

The JPG owner now has a schema-first 131072-octet encoding fixture with an interior cancellation boundary at 65536, a 128-byte ownership refusal, and a one-row refusal. Its new law requires both erased encodings to retain all seventeen literal fields and reads the actual projected `jpg_segment_octet` rows using Bun SQLite after Ajv validates the neutral control fixture. The owner output hook remains strict and unimplemented until an actual runtime RED. This law is authored, not executed.

Owned additions are `snapshot/🧫️fixtures/🛫️encoding/🔣️.json`, its `🧬️schema/🔣️.json`, and `snapshot/🧪️tests/🪶️sqlite/🦀️.rs` under JPG jfif-1.01/document. Note continues on the single warm native lane after the canonical Drawing dependency repair.
