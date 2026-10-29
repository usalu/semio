# DSL Intrinsic Closure Funding Audit — 2026-10-28

Read-only triage of actual88804 retained wire-dsl-tests.log. Both crates compiled; DSL model ran15tests with14passes and1failure. Direct tagged ordinal payload and paged octet laws passed. Failed intrinsic funding law at owned/intrinsic/tests12 expected default progress for a denied close grant, received copied_items1/copied_bytes4 with zero allocation/release. No test/build/probe/source edit by this audit; frontend owns repair.

Paths below are beneath 🧰️framework/🔨️modules/🗣️dsl/🧬️schema/🫴️receiving/📥️owned/ unless stated otherwise.

## Exact Denial Versus A Different Affordable Action

Intrinsic tests8 constructs an items0 grant, then reduces each other axis only when its quoted demand is positive. It skips zero-demand copy/capacity/release/depth axes. Therefore this failure is not caused by zeroing an irrelevant, nonrequested axis. Test12 first quotes close_demands(4096), then tests insufficient grants against that quote and expects zero progress/heap plus an unchanged subsequent demand.

Production intrinsic close_step45 instead calls close_demands(g.maximum_copy_bytes). A reduced copy grant changes the body hint used by child retirement and can select a smaller4byte action before the funding check. That new action may fit the grant even though the original4096-body selected action did not. Actual phase progress with zero heap can consequently violate the original quote refusal law without overallocating. The log does not record the exact chosen cut/current demand vector, so which child selector produced4bytes remains unmeasured; the source-level body substitution is established.

The local funds helper at owned31 checks all five currencies of one next demand. owned30 explicitly says admission must precede callback/mutation. Intrinsic fixture defines the document and cancellation prefixes0–99; it does not authorize requoting under a denied grant or exempt metadata progress from the item/copy/depth rules.

## Existing Normative Quote Selection Seam

ControlledRetirement selected_demand at value/♻️retirement/🎮️controlled/🦀️.rs:111 only calls next_demand(body); it is not an implicit cached quote. Its select method113–114 instead returns the existing RetirementSelection.

The canonical implementation value/♻️retirement/🎟️tickets/🎯️selection/🦀️.rs:5–18 stores the original body and demand. demands12 returns the same immutable action. admitted16 first checks every original currency, verifies the selected action still quotes identically under its exact copy frontier, then restricts the executable grant. step18 returns zero progress when that original demand is refused. This supplies direct contract evidence for preserving selected frontier semantics rather than blessing a different smaller action after reducing the grant.

An independently selected smaller frontier can be valid when the caller deliberately asks for that body and funds its quote. It is a different contract step from refusing the already quoted action. Source repair should retain that distinction and keep original source/cursor and allocator receipts intact. No broad interface redesign, test relaxation or invented capacity override is prescribed here; frontend received exact selection references before implementation.

## Evidence Boundary

The current failed test establishes one actual denial-law violation, not loss of the whole original owner or a backend release success. The earlier passing direct-variant and paged child tests remain separate observed successes. A corrected actual suite result is still required.

## Authored Selected-Close Repair Review

Read current intrinsic source52–64 and updated cancellation/selected-currency tests. No concrete blocking source flaw found; this review is not a gate on the parent-authorized coordinated actual suite. No tests/builds/source edits performed here.

The real NativeIntrinsicReceiving cursor now implements existing ErasedSnapshotRetirement and exposes select_close(body) as the canonical RetirementSelection loan. next_demand delegates the actual OwnedFieldReceiving.close_demands(body), close_step delegates its existing raw action and terminal state delegates its real owner. Qualified trait calls avoid accidental recursion. There is no ad-hoc Cell quote cache, fabricated proxy owner or changed raw grant-budget API. The selection borrows the cursor across admission, preserves original body/demand and retains canonical exact-copy quote-stability validation.

Updated test12 requests select_close(4096) for each denied/full close execution. It preserves default progress, zero allocator effect, unchanged next demand after refusal, full progress fits-grant, allocator equality, terminal closure and no-allocation destructor checks across existing cancellation prefixes. The receive/output assertions remain. Additional6currency fixture cases compare actual funds predicate and a separately executed Ajv/Lodash oracle; this extra test is predicate-level, while actual selected close refusal is exercised by the existing full cancellation loop.

Runtime still must establish child quote stability under the canonical exact copy restriction. A successful static review cannot guarantee no test or downstream compiler issue. The coordinated suite remains authoritative; no passing result is claimed here.

## Coordinated86170 Command Placement Audit

Backend supplied its exact active invocation from teaching cwd:

~~~sh
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=release --parallelism=1 -- test --manifest-path ../🧰️framework/Cargo.toml -p semio-framework-replication-wire-core -p semio-framework-dsl-record-model --lib --no-fail-fast -- --test-threads=1
~~~

The first separator ends repository Cargo ingress options. The second separator belongs to Cargo and starts libtest arguments. --no-fail-fast is correctly before that second separator, so Cargo handles it across package test binaries; only --test-threads=1 reaches the harness. Two explicit -p selectors choose both owning framework packages, --lib selects both actual unit-test libraries, and there is no test-name filter or skip. Native ingress readRepositoryCargoLaunchV1 splits only its first separator (launcher65), freezes/carries the actual Cargo tail (152–155), and feeds args into the native compilation input (200). No priority/guard workaround was added.

Required actual output remains two executed test binaries, nonzero test totals and named results including wire retained_decoder_roundtrips_nonzero_original_range_and_refuses_frame_substitution, DSL genuine_tagged_records_receive_original_ordinal_payloads_without_wire_conversion, original_paged_octets_lend_their_actual_native_children, the repaired intrinsic cancellation law and selected_intrinsic_closure_requires_every_quoted_currency. The wire identity Eq-only test is supplementary, not a substitute for the decoder-level payload law. Both binary summaries and final command status must be read. Cargo compilation success alone or output for only one package does not establish both runtime suites; --no-fail-fast cannot make a target run if compilation prevents producing its binary.

At backend supplied observation86170 was still door.capacity; this audit launches no process/test and is not a static gate on the active coordinated suite.
