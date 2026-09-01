# Native Resident Private Consumer Candidate

## Status

The resident-only private-consumer and exact-layout candidate is mounted, not compiled. There are 17 authored Rust tests. No Rust command was run by this lane. The preceding actual native result remains R6: 13 run, 12 passed, one failed, no secondary abort. Its genuine failure was a foreign thread refilling the externally mutable consumer before the last erased Arc release, causing one consumer destructor during release. R4 access12 and R5 Wasm results do not apply to this changed candidate.

Plugin, RuntimeAppCell, Opening, host, Actor and common Kernel source were not edited for this packet. The WGPU dependency hold was respected; resident was excluded from that actual graph. WGPU R17 subsequently ended before tests on separate OS-kernel compiler errors. No resident compilation can be inferred from that run.

## Exact Source Boundary

The sole Rust authority remains `🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs`, SHA-256 `508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f` at this declaration. Its canonical tests are `🧪️tests/🦀️.rs`, SHA-256 `ebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e` at final source release. The earlier test hash in coordination preceded the final fixture-derived cancellation counts and nontrivial original-C allocation-failure preservation join.

Only the existing admission JSON's `nativeOwnership` subsection changed: private phase-qualified access, exact requested Layout, four cancellation frontiers, an injected-null allocation-failure declaration and eight layout fields including consumer-page. Shared capacity semantics and TS logical prices are unchanged. No package, project, launch or workspace metadata changed.

## Private Consumer Authority

`prepare_consumer<C>(partition, grant)` reserves actual native node bytes/one slot/one owner into the original inline root before allocation. Its four separately retained phases are reserved, allocated, initialized and published. Allocation uses the exact `Layout::new::<ConsumerNode<C>>()`. The pending descriptor owns the exact pointer/Layout before control returns; initialization writes an empty typed slot. A null allocation retains the original pending reservation for cancellation/retry.

`prepared_consumer<C>()` privately issues a borrowed `ResidentConsumer<C>` for the original prepared page. It is not an owning Arc, exposes no mutable Option, and cannot be constructed from a pointer or equal numeric identity. `install` moves only from the supplied retained source and checks the root and consumer phases under the original allocation-free gate. A refused source remains unchanged. Source payload funding is not supplied by the borrow or this move.

Forward `prepared_consumer`, `read`, `prepared_admission`, `claim_admission`, record preparation, record alias issuance and record installation are closed when the original root or consumer is closing. Explicit `recover_consumer_for_close`, `read_for_close`, `recover_admission_for_close` and `recover_record_for_close` are close-only access. They do not restore forward installation authority. Recovery aliases still block page release until their actual Drop retires the alias count.

`handoff_for_close_into` makes an original consumer recoverable even if allocation failed before an admission cell existed. Like the existing cell/record handoffs, it validates exact source/phase/vacancy and actual Option move grant, but accepts an ordinary retained typed Option target. This is explicitly structural transfer only, not resident preadmission or original-parent field authority.

The erased consumer descriptor only holds the original private typed page and its admission reference. It cannot own or invoke C's destructor. Root close first revokes forward mutation, then requires the actual C slot empty and the exact consumer/admission aliases gone. Typed C retirement remains the concrete parent's responsibility. Release of an initialized empty node uses its original pointer/Layout; its next link is already detached.

## Exact Backing And Grants

Admission and record backing no longer use one-element Vec allocations or a postallocation capacity adjustment. Admission owns a raw exact-layout descriptor from allocation through initialization/linking and deallocation. Record allocation reserves its envelope plus exact native node before calling the allocator; if allocation returns null before a record exists, the uninstalled reservation is restored. Both preserve the original C owner. Record descriptor transfer and final page release require the larger of actual node bytes and erased descriptor bytes; consumer and shell moves have separate actual Option-size grants. These are scoped operation charges, not a measured strict timing proof of allocator internals.

Allocation observations refer to the allocator's requested Layout, not unobservable heap bins. The null failure is deliberately injected by the test allocator; this is not a claim that the pinned System allocator overallocated or naturally failed. The tests observe deallocation with the same requested Layout exactly once.

## Authored Tests Awaiting Execution

- Existing capacity4, layout/bootstrap, zero/short/foreign refusal, caller loss plus moved original parent, actual record-placement/parent-handoff unwind, final-root grant, allocation-free first access, busy access and sticky scalar poison are retained or migrated to the private capability.
- The original R6 law now performs a real other-thread install through the original private consumer. The attempted payload stays in the outer retained Option. The thread first attempts while the actual empty-observation release gate is held, then again after root closing. It must observe blocked/rejected, unchanged payload pointer and no consumer destructor during final release. Only after the root is closed is the rejected attempted payload explicitly dropped by its owning test parent.
- Three phase vectors exercise forward mint/read/record alias denial and explicit close recovery. Record installation through an already-issued handle must also refuse after consumer-only closure, not merely root closure.
- Four consumer construction cancellation frontiers observe allocation/deallocation counts, exact 64-byte alignment and reservation conservation.
- Injected null failure at consumer, admission and record allocation preserves the declared original reservation/source behavior, then closes the actual retained root before assertions.
- All three page kinds observe exact requested/deallocation Layouts, aligned typed backing, no extra allocations on zero/short grants, and physical allocated-byte/declared-envelope conservation.

The existing Rust target is `@semio-tech/value-resident-rs:test`, with no arguments and explicit exhaustive test level through the sole executor. Expected authored count is 17, no exclusions. Only after an actual passing native result may the existing `check-wasm` target be requested. No live Plugin or UI producer readiness follows from these core laws.

## Remaining Required Integration

An arbitrary Option is not a funded receiver. The actual registered Opening/RuntimeAppCell parent must privately issue one exact typed field descriptor from its already-admitted backing. No public projection callback is authorized; address/extent/alignment containment only checks safety and cannot confer field-selection or funding authority.

The enclosing bootstrap root must remain structurally owned through actual final retirement. This candidate's borrowed facades and moved-parent test do not solve loss of the enclosing original root, poisoned live-root recovery, arbitrary many claimed-owner recovery, or unknown-fault final disposal. The final-root step is an explicit empty-root transition with a required grant, not evidence that its enclosing application object was physically destroyed within a measured callback. Raw page descriptors do not make abandoned roots terminal; no intentional leak is used as a successful test cleanup.

The actual constructor-unwind R4 defect, registry capacity admission, original Opening partial-store ownership, WorkerPool completed-closure-shell witness and native UI/return descendants remain separate mandatory joins. No current native consumer or callback credit is promoted to those scopes.

## Neutral Oracle

After the peer's R60 TS snapshot ended, this lane added only the native-neutral oracle region. The actual canonical `bun x nx run @semio-tech/value-resident:test` then exited 0, including strictTS=0, native trace7, phaseAccess3, cancellationFrontiers4 and the peer's unchanged liveRecord7. Exact output and post-run hashes are preserved in `📓️native-resident-private-consumer-neutral-r2-2026-08-28.md`. Strict Ajv plus Immer validate phase and ownership-model outputs; they cannot establish Rust allocation, thread or destructor behavior.

## Native Source Release

The hashes above were the coherent candidate released for the sole executor's canonical no-argument exhaustive resident17 snapshot. The requested follow-up was the existing two-target Wasm check only if native17 actually passed.

## R7 Executor Terminal: Pre-Compilation Infrastructure Failure

The sole executor attempted the canonical resident target. This lane read the complete actual preserved output at `🧪️member-resident-private-consumer-r7-2026-08-28.md`. It failed in `loadTaxonomy` during package-name resolution, before Cargo: five `wgpu-frame-worker` tracked outputs were missing under the projected renderer `engine/🎯️targets/🧊️wgpu` paths. The reported paths are the Rust builder, Rust binary, Rust-package TypeScript library, renderer registry and generated frame-worker JavaScript. No resident compiler diagnostics and no native test results were produced; 17 authored cases remain unexecuted.

The executor released the resident source hold after this terminal. No native retry or Wasm run is requested until canonical infrastructure is coherent. The candidate hashes above are unchanged by this result. The last actual resident native behavior remains R6's 12 pass/one fail on the prior public-consumer implementation; neither an implementation compile failure nor success may be inferred for the new candidate.
