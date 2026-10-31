# Independent Native Drawing Composition Test-First Review

Read-only review of Rust's eight-file authored packet, both actual test bodies, defining OriginalJobFixtureCaller implementation, mounts, dependency declarations and allocator declarations. No Source edit, semantic test, compiler, probe or runtime was performed. This is independent review of Rust's native tests, not approval of this lane's TypeScript code.

## Blocking Coverage Finding: Final Error Effects

The scene law measures each paid call's posted receipt and compares context fuel spent to last_turn_work, which is useful. However, the only positive effect assertion is aggregate capacity>0 && cpu>0 after the loop. The final Err branch stores fault and breaks without requiring any positive effect on THAT failing turn. Earlier font/catalog/preparation turns can satisfy the aggregate.

Consequently an implementation that drops both the final failing turn's receipt and last_turn_work can pass if the final observed heap and context fuel are also zero. This is particularly relevant to the current boundary-neutral NUL error case: its actual final-turn effect is not established by authored Source. The equality zero==zero is not an error-after-progress witness.

Capture final-error posted receipt, physical allocation/release and fuel/last_turn_work separately. Require the intended error case's actual positive final-turn channels, independently of prior totals; preserve them in the debug record. A claim of allocation-on-error specifically needs nonzero actual heap delta matched by that final receipt. If this semantic NUL case honestly errors before effects, retain it as a zero-effect refusal case and add/refine a genuine bounded semantic error that reaches the claimed positive final-error behavior. Do not fabricate a StepContext, alter production to create test-only effects, or weaken error/custody guards. CPU and retained effects are independent axes; the contract must say which positive witnesses the chosen case requires. Root must review this correction before relying on error-path coverage.

## Genuine Authority and Composition

OriginalJobFixtureCaller::admit is the defining first-party test issuer: it actually admits cancellation, payload authority and source parent using its finite original stock, with per-thread heap/receipt checks. turn_with_decoder delegates to the same original parent borrow_context and with_retained_loan, binds the original clock, operation and generation, and records receipt/fuel on success or error. The authored laws use this actual public issuer and actual OriginalJobDecodeOwner, not replacement StepContext constructors or private maps. The pinned logical fixture clock is an existing fixture input, not a forged production invocation.

The scene body invokes actual DocumentVectorJob.advance_original and checks zero-effect denial, same-context repeat and actual context receipt/fuel. It is mounted exactly once by the existing Drawing scene-tests companion. Production dependency compilation does not run this body.

The IO law creates a genuine ArtifactStore and snapshot read, initializes real ExportProducer, and advances its real resident Vector to a positive font-capacity demand. It then calls full ExportProducer.advance with capacity one byte short under the same real Original caller. It checks pending return, zero receipt/heap/fuel, unchanged demand, no output and source authority. This is materially stronger than a flat ledger stub or fabricated phase. The IO owned test module mounts it once.

## Custody and Heap Limits

The tests retain and explicitly close vector/font results, faults, document, export source read, returned read queue, serializer/store, decoder and original roots. close_original_retained_step is used under original retirement custody; close refusals unwrap/fail rather than being swallowed. Scene close_owned compares actual heap to the original caller's posted receipt.

Two narrower coverage improvements remain: new measured scopes do not explicitly assert heap.overflowed is false, and terminal drop(closing)/drop(producer) occurs outside a zero-cost heap assertion. Existing direct text close laws cover zero-cost final drop, but they do not independently establish it for the real ExportProducer/store composition. Add exact final-drop observation if this new composition law is claimed to prove absence of hidden terminal release; do not infer that merely from terminal_is_empty. These are test-strength findings, not a discovered production lifetime defect.

The scene companion has one global HeapWitness allocator; IO has one allocator in the new font module. They belong to separate selected lib-test binaries, so those two declarations are not duplicates in one owner. A bounded source scan of each relevant owner tree found no second declaration. Actual canonical Source admission and compiler linking still determine fit/validity.

## Evidence State

The packet is authored, not executed, and references proposed production advance_original/next_work_demand/last_turn_work APIs. Existing three direct text laws and fourteen glyph corpus rows remain complementary semantic/independent-font tests. Nothing here establishes a Native pass or permits a new runtime.

Reviewed current identities:

- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🧪️tests/🎟️original/🦀️.rs: 77424adab169c2775d54897ba27665d5cfa1ba01981c84c4fa3500e50bf7403b; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🧪️tests/🎟️original/🔣️.json: 4fdb9f6efb5e6459f513594db46648da206029db6da777e312d6277183e5e23b; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🔤️text/🧪️tests/🎟️original/📐️.json: 017f6a1fb6bb77da1c1cf774107d7eb39f60cdc72e11ed4e76ff3bd42a9bd68a; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧪️tests/🥣️companion/🦀️.rs: ca019c04c51b3d6cffa81727af2fd894551a702970bd8a6d458984de6d1d3cf7; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🧪️tests/🥣️companion/📦️packages/🦀️rust/Cargo.toml: fa6b1b1c64e828f840fe79a823e03d9cb5b792bb7c7ebcbe1471fec9a9aab05a; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🧪️tests/🎟️font/🦀️.rs: 25eea3c30003fe901fc6b3e4b82976a1d4d036366251447ed675aefb98e6815b; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/📦️owned/🧪️tests/🦀️.rs: aa3f463b31bb4a570f79fcc26872cccb51db90e39cc8d70e9bb396090134688a; matches authored packet: true
- ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml: fc617df767c474e4cc6b08dff77b3d9f57ef86a93fb88d164f5e4ddf73a7ee74; matches authored packet: true
