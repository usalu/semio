# Paged Payload Builder Ownership

## Current Boundary

The concrete private native Field/Fragment/Release classes are joined. The builder copies an actual first 257-byte native fragment into two admitted fixed pages and issues a strongly registered sequential reader. Producer child faults are persistent; bounded close remains available. The sections below preserve the earlier staged milestones and their scoped results. There is still no live renderer cutover, whole multi-page operation completion, raw page ACK, semantic UI ACK, heap-size or wall-clock certification.

## Exact Admission

Before field binding, the builder must check both `OwnedKernelReturnInputField.matchesOwner(field, exactInstance, exactActivation, exactLifetime)` and `OwnedUiResidentPayload.matchesOwner(payload, exactInstance, exactActivation, exactLifetime)`. The first is peer-owned and pending; the second is present in resident source. Only the privately minted builder may satisfy `OwnedUiOperationPayloadBuilder.matchesField(builder, field)`. Metadata equality, a raw object or an active tuple alone cannot create this authority.

Builder creation must register a strong concrete child in the original instance before its facade is returned. Field binding and resident-payload adoption must not leave a transferred field or reserved payload unreachable when a caller throws. A failed admission retains the original field owner; an admitted but failed builder remains a serviceable instance child. Instance close services these children and source release receipts before closing their resident scope.

## Storage and Reader Ownership

Each destination page is one 256-byte reservation from the shared host pool. A linked cell is installed before allocation, so reservation/allocation failure remains recoverable. A step either reserves metadata, allocates a fixed page, writes one byte, seals a page, detaches the source reader, issues evidence, or advances one child. No terminal child result is combined with wrapper bookkeeping in the same grant.

A partial destination page can span raw fragments; its already copied bytes are independent of the raw source even before the page is sealed. A copied token therefore requires all fragment bytes written and the source reader detached, not a full final page. The exact peer release receipt must arrive before the next raw fragment is accepted. Capacity exhaustion preserves both the current source fragment and partial destination without requesting another raw page or issuing false copy evidence.

The completed payload's concrete sequential reader will be strongly linked to its consumer before exposure, rather than capturing page aliases and handing an unregistered facade to a caller. The intended single decoder reader is retained by the exact builder/payload owner; any additional concurrent read needs its own admitted owner slot and strong consumer registration. Parent close must service issued readers before waiting for payload counters, avoiding a self-owned close deadlock. There is no whole-buffer concatenation or public arbitrary byte-reader callback.

The existing wire grammar uses only sequential input with bounded text (512 bytes) and numeric (8 bytes) scratch, but currently retains subarrays for prior symbols and map keys. Paged adoption must replace those views with owned bounded scratch and explicit close. The wire grammar and native UI bounds are not being changed. JSON/scene payload dialects remain separate.

## Staged Neutral Cases

The new fixture declares five byte patterns, including an 8193-byte field over three raw fragments and 33 charged pages; five invalid range cases; exact-owner/token forgery; every named cancellation phase; child refusal/throw/over-grant; shared-capacity refusal; and caller/reader-loss laws. Node Buffer will independently reproduce the byte patterns, Ajv will validate the fixture/domain, and an Immer ledger will check shared reservations. These are staged cases, not executed implementation tests or native return proof.

The producer fragment source, instance child registration, decoder consumption, peer release receipts and live host capacity injection still must be implemented and verified together. Raw input release remains distinct from native semantic UI publication acknowledgement and final instance retirement.

## Released Cyclic Brand Interface

`retained/🩹️operations/📥️wire/📄️pages/🟦️component.ts` exports the concrete classes `OwnedUiOperationPayloadBuilder`, `OwnedUiOperationInputCopied` and `OwnedUiOperationInputCancelled`. Constructors require a module-private mint. There is deliberately no factory or mint export and no successful instance construction yet.

`OwnedUiOperationPayloadBuilder.matchesField(builder, field)` checks the direct private builder brand and exact field identity. Each evidence class exposes `matches(token, fragment, field, builder, offset, length)`, checking its distinct private brand, exact identities and checked u64 range. This enables the peer's concrete field module to import the actual types without a structural authority placeholder. Positive construction will be added only with the exact Field/Resident admission and strong instance registration described above.

## Executed Schema and Brand Gates

The declared-byte schema initially accepted `18446744073709551616`; the strict boundary test failed as intended. The schema now references only the canonical neutral actor byte-page `definitions/word` domain, including zero, without borrowing actor page authority. An intermediate schema-reference resolution failure was corrected by using the repository's relative identifier convention; it was not a behavioral pass.

The private-brand test first failed because the concrete class exports did not exist. The implementation now rejects structural evidence, hostile getters and reflected constructors without touching hostile fields. R3 executed **2 passed, 643 skipped, 645 total**, exit 0, 10.49 seconds. The eight decimal boundaries are checked against Node Buffer's u64 round trip and strict Ajv. These are rejection/domain tests, not positive copying or retirement proof.

The subsequent canonical strict check reported exactly the seven existing tutorial joins and no paged-input diagnostics. Logs: `🧪️renderer-owned-paged-schema-red-r1-2026-08-27.txt`, `🧪️renderer-owned-paged-brand-red-r1-2026-08-27.txt`, `🧪️renderer-owned-paged-schema-brand-r3-2026-08-27.txt`, and `🧪️renderer-owned-paged-strict-r1-2026-08-27.txt`.

Canonical focused command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedPaged'`. No live reader, payload, copied token, native release, resident heap or latency certification follows from this gate.

## Concrete Admission and Cancellation Join

The peer's actual private input module is now joined. `OwnedUiOperationPayloadBuilder.begin(owner, activation, lifetime, field, resident, grant)` first checks its fixed 1024-byte metadata grant, the actual Field owner and the actual Resident payload owner. It reserves one additional shared owner slot before allocating the fixed builder, attaches that builder strongly to the resident payload, then binds the actual field. The original instance already strongly owns this scope and payload. No caller callback or tuple-only stand-in enters that chain.

`OwnedUiResidentPayload.reserveBuilder/attachBuilder` keep even an uninstalled reservation or failed field binding recoverable. A second field-binding refusal leaves the original builder's source readable. Payload close first drives the registered builder; it does not erase it on refusal, exception, or terminal child work. The extra owner slot is returned in a separate 64-byte step after the builder is actually empty. A child reporting 4096 terminal bytes is forwarded unchanged before that wrapper step.

The builder currently implements admission and cancellation before copying. Its close phases capture/disable the selected source, issue a distinct private cancelled proof, obtain the actual peer release receipt, retire that exact receipt/proof pair, then clear its owner references. Each phase charges 128 bytes and never performs child work plus wrapper retirement in one grant. The original raw page/response and native input ACK remain peer-owned and are not certified by this local release. No source byte is read in this cancellation path.

The 1024/128 metadata allowances cover fixed fields and fixed private identity checks only. They do not depend on the declared field length, copy a field name or payload, or reserve destination pages. They are logical work allowances, not measured physical heap sizes or an 8 ms allocator certificate. Positive copying, registered sequential readers, removal of unchecked resident-page capture and multi-page source advancement remain subsequent gates.

### Actual Gates

Admission R1 executed the intended missing-`begin` RED: **1 failed, 645 skipped, 646 total**, 13.10 seconds. R2 reached the actual native fixture but rejected a Node Buffer at the actor byte-page constructor's current realm gate; the fixture now explicitly materializes its bounded cold page as the expected Uint8Array, without changing that production gate. R3 passed the real admission/caller-loss/revocation/cancel case: **1 passed, 645 skipped**, 16.01 seconds.

R4 executed the combined schema/brand/admission/hostile-close cohort: **4 passed, 643 skipped, 647 total**, exit 0, 24.47 seconds. This includes the genuine private native field, shared owner saturation, caller-loss recovery, zero-read cancelled proof, failed second bind, source-release refusal and terminal-child bookkeeping separation. The neutral admission counters are in the existing paged fixture/schema. The full captured logs are `🧪️renderer-owned-paged-admission-{red-r1,green-r2,green-r3,green-r4}-2026-08-27.txt`.

Strict R1 reported seven existing tutorial joins plus five new test-only typing errors (Node assertion binding, Immer's mutable oracle value, and a private constructor type query). Exact fixture typing repairs were applied; R2 is pending. Neither full renderer regression nor positive byte-copy/reader behavior is claimed by the four-test gate.

## First-Fragment Copy and Reader

The missing `advance` law produced a real RED in `🧪️renderer-owned-paged-copy-red-r1-2026-08-27.txt`. R2 passed **1 test, 647 skipped, 648 total**, 11.29 seconds, using actual Shard return/field/fragment authorities and Node Buffer's independent byte pattern. The source fixture presents one genuine 257-byte fragment, not the fixture's staged three-fragment schedule. Two pages reserve 512 logical resident bytes. The copied token is issued only after the final source read is detached, and the actual private release receipt is checked before settlement.

Reader admission charges its own shared owner slot and strongly installs the concrete reader in the original builder before exposure. It exposes one byte per advance and never exposes a page or backing array. A lost caller's reader is closed by the parent before pages and owner credit retire; revocation alone does not invalidate already admitted reads. The existing resident-page `capture()` remains a separate unchecked consumer-registration boundary and is not used by this reader or certified for live admission.

The finite copy loop allowance is `n + 4*ceil(n/256) + 16`: one write per byte, one reserve/allocate/seal per page, and fixed field-admit/detach/proof/release/receipt/ready transitions. The reader allowance is `n + pages + 2`, from one scalar read per byte and one page-link transition per page. These are transition ceilings, not an 8 ms or physical-memory proof.

## Child Fault and Cancellation Evidence

`🧪️renderer-owned-paged-fault-red-r1-2026-08-27.txt` executed **2 failed, 648 skipped, 650 total**, 12.30 seconds. A rejected page allocation resumed on the next producer call; a reader exception escaped after its offset increment. The separate before/after-write test initially failed on the before-write case: **1 failed, 650 skipped, 651 total**, 12.76 seconds (`🧪️renderer-owned-paged-partial-red-r2-2026-08-27.txt`).

The producer now checks its retained failure before any subsequent source read, page write or successful admission. Actual child rejected/over-grant results preserve raw item/byte counts and prevent phase movement. Blocked remains retryable. Exceptions before or after an actual page mutation fail closed; close still detaches the input and retires all admitted pages. Reader offsets advance only after a successful scalar read; a read fault remains explicit and cannot skip into another byte on retry.

R3 passed **8 tests, 643 skipped, 651 total**, 9.27 seconds (`🧪️renderer-owned-paged-fault-green-r3-2026-08-27.txt`). It executes all five allocation child vectors, both write-fault timings, the exact registered-reader fault, and the earlier schema/admission/copy cases. A thrown child supplies no structured work receipt; the zero-byte fault return is not a claim that the child performed zero physical work.

R4 passed **1 cancellation test, 651 skipped, 652 total**, 6.83 seconds (`🧪️renderer-owned-paged-cancel-r4-2026-08-27.txt`). The test iterates every prefix until actual first-fragment readiness, asserts the finite transition ceiling, visits all ten observed copy phases, and also closes the issued reader. Before detachment it requires exactly one Cancelled proof; after detachment it requires exactly one Copied proof. Close performs zero additional source reads. Every prefix returns the shared pool to zero and preserves zero-grant refusal. No opaque larger retry cap substitutes for the successful production-step bound.

## Constructor Registration Fault

The constructor-finalization law exposed a distinct actual RED: **1 failed, 652 skipped, 653 total**, 9.58 seconds (`🧪️renderer-owned-paged-registration-red-r5-2026-08-27.txt`). An already allocated private builder captured during a throwing finalization was not reachable through its parent after close. The neutral test also checks a field-binding throw without inventing a source authority.

The old public `attachBuilder` method has been removed. The private builder constructor now installs itself into its already reserved exact parent slot before finalization or field binding. A narrow direct-private-state bridge accepts only an already minted builder with the exact resident identity; it cannot mint an object or select a foreign parent. A refused installation clears the constructor's two fixed references and makes the never-bound object terminal within the existing metadata allowance. Normal caller loss and constructor/bind exceptions leave the parent as the strong retirement owner.

Combined R6 passed **10 tests, 643 skipped, 653 total**, 17.18 seconds (`🧪️renderer-owned-paged-registration-green-r6-2026-08-27.txt`). The unchanged exact constructor-finalization failure law and field-bind failure both execute. Resident R7 separately passed **3 tests, 650 skipped, 653 total**, 12.60 seconds (`🧪️renderer-owned-paged-resident-r7-2026-08-27.txt`), preserving existing pool/alias behavior. Canonical strict R3 completed with exactly **seven existing tutorial diagnostics**, zero paged-input, resident, constructor-fixture or Node assertion diagnostics; full output is `🧪️renderer-owned-paged-strict-r3-2026-08-27.txt`. No full 653-test run is claimed.

## Source Continuation Review

The peer's complete updated `kernel/return/content/input` source was read after the first-copy gate. `field.advance(grant)` now consumes at most one original byte of an already Copied range into the original framing, and `field.complete` denotes only that selected field. A Cancelled range never advances as copied. When a source page ends partway through the field, the fragment becomes null and the source remains blocked: next-page minting and raw page InputAck are still absent. The UI does not synthesize a next fragment or treat its copied token as whole-return retirement.

## After-Bind Partial Fault: Executed Repair

The next exact native-owner law executes the real `field.bind`, confirms that it returned true and permits the original source byte read with the private builder, then throws before the UI caller observes success. R8 actually failed: **1 failed, 653 skipped, 654 total**, 12.74 seconds (`🧪️renderer-owned-paged-bound-red-r8-2026-08-27.txt`). Parent close recovered the builder but issued zero cancelled releases instead of one. The earlier constructor/before-bind green laws therefore do not cover this distinct partial binding mutation.

The neutral fixture separately specifies constructor-finalization, before-bind and after-bind outcomes. The source now exports `OwnedKernelReturnInputField.matchesBuilder(field, builder)`, checking the exact private identity even after close begins. UI close checks that authority before capturing cancellation input. It neither infers ownership from a missing return nor consumes a source byte to probe binding. R9 passed **11 tests, 643 skipped, 654 total**, 13.87 seconds (`🧪️renderer-owned-paged-bound-green-r9-2026-08-27.txt`), including the unchanged after-mutation law.

## Source-Owned Continuation and Persistent Faults

R10 actually failed both newly added continuation laws: **2 failed, 654 skipped, 656 total**, 13.02 seconds. The builder previously reported readiness before original framing consumed the copied range. The source API now requires `field.advance(grant, exactBuilder)`, and only that privately bound builder may drive it. R11/R12 exposed the missing second argument during the peer's API change; these were real rejected joins, not source-framing successes. R13 passed **13 tests, 643 skipped, 656 total**, 15.30 seconds (`🧪️renderer-owned-paged-continuation-green-r13-2026-08-27.txt`).

The builder now drives at most one original source byte per child call after the exact copied-release receipt. It preserves blocked/rejected and raw child counts, retains faults after before/after-work throws, and never synthesizes next-page offsets or input ACK. A source-page boundary with no new genuine fragment remains blocked and owned. The six child vectors include a terminal 4096-byte result and a throw after actual source mutation.

## Reader Construction and Completion Observation

R14 executed **2 failures, 656 skipped, 658 total**, 7.77 seconds (`🧪️renderer-owned-paged-boundary-red-r14-2026-08-27.txt`). First, a reader captured during throwing constructor finalization was not parent-owned. Second, source completion metadata was inspected during a child call already reporting the entire 4096-byte grant.

The private reader constructor now installs itself into the exact builder's reserved slot before finalization. A lost or throwing caller cannot strand that reader or refund its reservation while it remains nonterminal. Source advancement now records only the returned child kind and forwards its work unchanged. A separate 128-byte observation phase reads completion/consumed/new-fragment metadata, validates it and selects the next phase. Terminal observation and subsequent readiness are separate turns.

R15 passed **15 tests, 643 skipped, 658 total**, exit 0, 15.78 seconds, at 22:50:53 (`🧪️renderer-owned-paged-boundary-green-r15-2026-08-27.txt`). The two unchanged hostile laws now pass, including spies proving zero completion/consumed getter calls in the full-grant child turn. Every-prefix cancellation now includes source advance and observation. The current first-fragment success ceiling is `3*n + 4*ceil(n/256) + 16`: one copy-byte turn, one source-byte child turn, one source-observation turn per byte; page reserve/allocate/seal/link allowances and fixed admission/proof/release/readiness transitions occupy the remaining terms. This replaces the earlier copy-only bound and is not a latency certificate.

Strict R4 completed with **11 diagnostics**: seven existing tutorial joins and four peer in-progress `ActorReturnResultFraming` missing-export diagnostics; zero paged/resident/reader errors. Full output is `🧪️renderer-owned-paged-strict-r4-2026-08-27.txt`. No full 658-test suite or live renderer cutover is claimed. The source-owned next-page path, removal of unchecked resident page capture, shared neutral ledger injection and paged typed decoder remain open.
