# Bounded Typed Canonical Reader Handoff

## Latest Native Verification

Coordinator canonical r6 passed 22 tests, zero failed, 847 filtered (37.12 seconds compile, 0.22 seconds tests). All four reader laws passed, including the unchanged negative closing-transfer law and the new guarded-drop law. Two caught lifecycle panics were expected. The coordinator approved reader adoption for Flow and Procedural2d/3d. Historical pending/failed counts below describe earlier checkpoints and are superseded by this result.

## API And Ownership

ArtifactCanonicalJsonReader<T> is exported by Store. new accepts an exact frozen Arc<T> and its SnapshotRetirementFactory<T>. encode_chunk accepts the existing one-item grant and a caller output slice; output is capped by grant bytes, slice length, and256. completed_bytes reports actual emitted bytes; is_complete reports finished traversal. cancel prevents further encoding. begin_close/close_step retire frames first and then the exact root through its retained factory.

take_root returns the same Arc allocation only when encoding is complete or cancellation has already closed every frame. It resets empty traversal and marks the reader closing; callers still drain close_step to release the retirement factory. It never exposes mutable root access. No edit publication token, caller-supplied JSON/digest, checkpoint, raw pointer, or unsafe public contract is exposed.

The reader shares ArtifactCanonicalEditEncoder with the sealer. Its private generic bind now accepts a typed frozen root held by either Box or Arc and checks allocation identity before cached references are used. Fixed indexed sources use the same combined64-depth budget. Reader field declaration and explicit close both put frame destruction before root ownership. Source Sync and retained iterator Send requirements remain unchanged; there is no unsafe Send implementation.

No checkpoint API is exposed by this reader. Consumers can retain/move the owner to another worker; any fresh replay starts from a fresh typed reader rather than imported iterator/hash state. Consumers own their ordinary content hash sink, but cannot use that hash to mint Store edit authority.

## Schema-First Tests And Exact Counts

The strict language-neutral reader fixture references the existing recursive-map fixture, whose canonical output is4964bytes and contains a4407-byte escaped Unicode key. Its plain JSON SHA-256 is c6f612c4cbe854362771a36663b370c076bfc602846f224edb74c8a97a8eb878.

Three native tests are authored and await the coordinator:

1. One/seven/4096-byte grants produce exact serde_json bytes and Node fixture SHA; zero grants perform no work; completed transfer preserves the Arc allocation.
2. Cancellation before polling, mid-key (with cross-worker transfer), and after completion closes frames before the one tracked root is destroyed.
3. Root rebinding fails before borrowed-reference access; mid-encoding transfer is refused; cancellation permits transfer only after all frames close.

The existing root verifier adds11 checks: one strict valid fixture/Node digest oracle, three strict schema hostile cases, three positive-grant canonical byte cases, and four source-hostile cases (early root destruction, early transfer, monolithic serialization, ignored grant). Supplemental execution of the actual exported helper passed47 combined checks (original21+maps15+reader11). Targeted git diff --check passed. Canonical Nx workspace:verify-interactivity --args='tool-jobs --self-test' passed702 checks (33 exact factory owners,254 custom rows,31 generic rows). Native execution remains coordinator-owned.

## Integration And Limitations

Flow's owner accepted the API and is wiring canonical FlowWorkingScene content hashing with its own retained root retirement. The coordinator approved that separate child-content identity integration. This reader does not implement the Flow projection or Generator slider command itself.

Concrete typed visitor determinism, serde fidelity, and bounded iterator construction/next/drop remain reviewed app obligations. The public type guarantees borrowed ownership, not instruction counts for arbitrary custom iterators. No Miri or negative compilation proof is claimed. The root may be shared externally; final ownership retirement must remain coordinated through the supplied factory.
# Native Closing-Transfer Repair

Coordinator native execution compiled21canonical tests. The reader byte oracle and cancellation laws passed, but the closing-transfer half of the rebound/lifetime test failed: after the last frame closed, depth was zero while started/root_address were still bound. The old take_root condition treated that intermediate closing state as completed encoding. Its failing assertion then exposed a second panic from the unclosed reader destructor. That broad native run aborted and is not recorded as passing.

The repair separates closing from normal completion: closing transfer requires fully terminal encoder state, including cleared binding; normal completion is unavailable after cancellation or an encoding error. Encoder errors latch failure until explicit close. The complete private reader state now sits in ManuallyDrop; a nonterminal ordinary drop fails loudly without automatic recursive root/traversal destruction, and unwinding does not panic again. Terminal close explicitly drops the empty state.

The original negative fixture remains; it additionally asserts that closing is never reported as completed encoding. A fourth native test covers invalid live reader drop and primary-panic unwinding with exact root/iterator counters. Four reader native tests now await rerun. Public typed reader APIs are unchanged.
