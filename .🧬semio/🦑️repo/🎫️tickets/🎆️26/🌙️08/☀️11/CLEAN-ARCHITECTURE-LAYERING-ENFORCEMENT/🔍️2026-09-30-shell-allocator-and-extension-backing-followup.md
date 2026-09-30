# Shell Allocator And Extension Backing Follow-Up

Read-only source review; no independent builds or runtime tests. Extension metadata owner subsequently confirmed the intrusive frontier and sorted handler source settled.

## Session Receipt Safety

`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🦀️.rs:735–758` records the actual Global allocation Layout in the allocator stored in each Arc handle. Clone copies the byte requirement and clears the receipt pointer. Private receipt installation occurs only immediately before consuming that exact handle with `Arc::into_inner` or final drop. The stack AtomicUsize remains live until consumption returns; surviving cloned handles carry null receipt pointers. The Arc allocator is per handle, so recording one handle does not publish a stack pointer to the allocators of its already existing peers. No escaping receipt pointer was found in these source paths.

Capture handoff lines 783–792 admits the original state's allocation requirement before potentially freeing it. Non-final release reports zero actual bytes, while final extraction reports the original allocator Layout. Final shell release lines 795–803 admits the sum of its possible state and five family shell allocations, then reports actual allocator deallocations. Conservative admission can exceed actual frees when peer family handles remain; the returned receipt reflects actual frees. Private handles have no public weak-reference creation path.

The first-party crate root `🌊️session/📦️packages/🦀️rust/🦀️.rs:1` enables allocator_api, and repository `rust-toolchain.toml:2` pins nightly-2026-07-07. These std APIs are therefore consistent with the repository's selected toolchain. Source inspection does not establish compilation on every native or WASM target.

## Portable Fixture Linkage

The native law `🌊️session/🧪️tests/🏷️session-lifetime/🦀️.rs:116–165` checks zero grant, insufficient original-shell grant, shared-reader release without sealing root, final original-shell receipt, insufficient final-family grant, and exact final-shell release. It reads shell ratios and expected phases/items from the portable retirement fixture. The independent TypeScript/Ajv owner `🧪️tests/🏷️ownership/🟦️.ts:25–34` checks the same portable shell ratios across five layout sizes; actual native allocator sizes remain measured by the native law rather than assumed portable constants. No test was executed in this review.

## Corrected Prior Observation

Removing a terminal inline ExtensionBundle from a registry Option frees no heap for the inline struct itself. It should report zero bytes when its resource Box, handler backing, manifest payloads, and metadata frontier/backing have actual terminal-empty witnesses. The prior report's demand to charge inline bundle size is withdrawn. Resource Box size and owned backing allocations remain the relevant admissions.

## Pending Extension Review

The shared source already uses a sorted handler Vec with explicit capacity-zero terminal witness and transfers its empty backing to metadata retirement. The intrusive typed metadata frontier is still being authored; the older boxed metadata Vec implementation must not be treated as the settled result. A subsequent section will record its actual settled invariants after handoff.

## Concrete BREP Owner Shell Gap

`✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🦀️.rs:2133` still defines `BrepExtensionResources { session: Session }`. Its terminal query at line 2177 establishes native family cursor emptiness, while its close slices drain that cursor without consuming the raw Session handle. Neutral resource removal at `🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs:92–96` then admits only the concrete Box allocation size and drops the owner. That destruction also releases the raw Session state's and family's Arc allocations without the new ShellAllocator receipt scope. This is an actual nested heap release omitted from the byte grant, not an inline bundle-size estimate. Give the BREP owner an explicitly retained SessionCapture (or an equivalent real shell cursor), consume its final handle through shell admission before declaring resource terminal, then admit only the remaining shallow Box allocation.

This gap was sent directly to the extension implementation owner and root. It is the only concrete new actionable defect found in the allocator follow-up so far.

## Introduced Playbook And Flow Port Boundaries

The same raw-handle gap exists in `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️.rs:364–365`: ModuleGeometryOwner owns a raw Session. Its close path at line 436 drains the native family cursor, and its terminal predicate at line 447 reports terminal while the raw Session handle still owns shell allocations. Generic instance-owner removal then drops those allocations. Root was notified to make actual final shell retirement part of this owner before terminal acknowledgement.

Neutral `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2678` (FlowHostRetirement constructor) and line 4073 (FlowEvaluationSession.begin_close) take the geometry-port Box, call its sealing-only close, and ordinarily drop it immediately. Session.port allocates a separate SessionState authority, so this destroys the port Box and authority Arc shell outside any byte grant, even though native payloads remain safely retained by the producing Session. The port must remain in the retained owner until an actual shell-release operation is admitted. Current payload-only terminal criteria do not witness physical allocation emptiness for that port.

## Intrusive Metadata Source Review

The latest extension frontier source at `🔌️plugin/🧩️extension/🚪️retirement/🦀️.rs:271–335` replaces the pending Vec with a ManuallyDrop singly linked chain. One step detaches the next pointer before dropping a node, admits node Box plus concrete metadata-owner Box requirements, transfers nested fields rather than dropping their trees, charges String capacity, and charges empty Vec capacity times element size. This removes the earlier pending-Vec backing omission. DslValue Object is a first-party Vec of String/value tuples, handled by the same typed Vec and tuple frontiers. Sorted handler storage transfers empty backing and requires capacity zero before terminal. No further concrete metadata defect was found in this source snapshot; final owner handoff and runtime validation remain separate.

## BREP Capture Correction Reviewed

The updated BREP guest now creates `Session::new().capture()` at line 2122, owns SessionCapture at line 2133, delegates real capture close slices, and reports capture emptiness at line 2177. It exposes `shell_byte_requirement` through `next_close_byte_demand` at line 2178 and guards begin/cancel/resume against an already empty capture. The temporary root Session drops while the capture still owns the same state; final state handoff and shell release therefore remain in the retained capture. This resolves the BREP raw Session finding in the source snapshot. No additional concrete defect was found in that adaptation. Playbook and neutral Flow port findings remain for their respective owners to resolve.
