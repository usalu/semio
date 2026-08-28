# Retained Scene Packet Parser

## Implemented Boundary

The new `ui/contract/retained/scene` TypeScript module parses the native scene tags, not the different UI record format. It captures a privately profile-checked immutable component owner before any original decoder/producer can retire. Strings and byte fields stay as validated offsets into captured 256-byte SurfaceDoc pages. No whole packet array, final string concatenation, recursive value object, or subtree deletion is used in production.

The arena contains frozen constant-size records in the retained numeric index. Sequence/map/option/variant children are addressed by their actual encoded offsets and end positions. Explicit linked frames avoid recursive parser stack growth. Map keys use a numeric parent-offset plus FNV bucket identity and exact incremental byte comparison for collisions; equal hashes are not identity. The known `costarring`/`liquid` collision is admitted as two different keys. Long duplicate Unicode keys are rejected only after comparing all actual bytes.

All production operations use one item and at most 4096 admitted bytes. Varints are at most ten byte steps; Unicode is validated bytewise; text reads emit at most 128 input bytes per step and byte reads copy at most 256 bytes. Scalar integer records preserve the complete u64/i64 domain as bigint. Byte-slice creation only accounts for constant metadata because it does not copy or hash its payload. `completedBytes` is consumed input position, not accumulated work credits. Fixed-size native float bits are preserved; typed host validation is still required.

Private document/reader/retirement mint authority rejects forged JavaScript construction. Each independent reader captures the exact root. Reader lookup frames retire before its root claim, and final arena retirement precedes source component/page retirement. Cancellation explicitly closes the generator, linked frames, active index readers/edits, index retirement queues, then the source. No user-supplied digest, pointer, root or projection is accepted.

## Tests and Actual Results

Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedScene'`.

- R1: missing-module collection RED, four failed suites; no behavior executed.
- R2: one executed failure exposed an incorrectly handcrafted char varint. The fixture was corrected from a different Unicode scalar to the intended broom scalar.
- R3: three executed failures: JSON module transformation lost the literal `__proto__` fixture key; runtime private constructors accepted forged roots; the generic 500000-iteration test guard was below the actual work needed for 4096 retained nested options.
- R4/R5: semantic and hostile tests passed; the explicit two-million-iteration deep-fixture guard was still below actual work. R5 showed 3699 of 4097 records had completed, with all 4097 input bytes consumed. The scene-specific guard is now 1024 iterations per fixture record; the production one-item/4096-byte grant is unchanged. No latency certification is inferred from this work-count guard.
- R6: **4 passed, 542 skipped, 546 total**, five files, exit 0; start 16:37:21, 7.39 seconds total, 4.43 seconds tests. Full output: `🧪️renderer-owned-scene-r6-2026-08-27.txt`.

The four tests cover 19 semantic tag vectors, 16 hostile packets, strict Ajv, Immer expected values, the existing scene codec where its semantics are correct, Node Buffer float/u64 values, fatal UTF-8 decoding with BOM preservation, exact hash collision comparison, 11000-byte Unicode strings/keys, full native 32768-byte packet with a 32764-byte text slice, 4096 nested options, every prefix of a small option packet, representative cancellation frontiers for all observed nested-map phases, mid-key cancellation, independent same-value roots, and readers that outlive their producer and document owner. The fixture is also parsed from its raw JSON source so JavaScript object-literal semantics cannot erase `__proto__`.

Strict typecheck R1 had nine diagnostics: seven existing Dag-owned tutorial joins and two test-oracle discriminant narrowing errors. The oracle now explicitly checks `map` before using its fields. R2 finished with exactly the seven existing tutorial diagnostics; zero scene/test-oracle diagnostics. Full output: `🧪️renderer-owned-scene-typecheck-r2-2026-08-27.txt`.

## Derived Deep-Fixture Work Bound

The latest test replaces the interim 1024 multiplier with an explicit bound from the numeric index source. For `n` inserted records, each AVL height is at most `2*ceil(log2(n+1))`. A rotation allocates at most three nodes and has at most nine reference slots. The reservation pass scans at most nine existing slots for each target; the bound includes slot scanning, reservation, three allocations, three node closures, nine reservation closures and state handoffs. Two trees are rebuilt per insertion; per-level work also includes sixteen conservative search/temporary-retirement handoffs. The current test allows `2*(height+1)*(allocationTurns+16)+9*height+64` turns per record, plus four turns per input byte for frame handling. It asserts nondecreasing consumed input and completed records on every turn, and rejects a no-progress interval longer than that derived per-record bound. This bound applies to the nested-options fixture without map-key collision scans; those are measured separately by exact compared-byte count. No larger runtime step budget or fake byte credits were introduced.

Source accounting, using `value/ordered/numeric/🟦️component.ts`:

- `balancedAllocation` (around 175) has at most three allocation specs. `TreeAllocation.advance` (around 120) selects at most nine reference slots, and its target scan takes at most nine comparisons plus insertion per slot. Hence `referenceSlots*(referenceSlots+1)+referenceSlots`. It then has one reservation commit and up to three node allocations.
- `TreeAllocation.closeStep` (around 156) pops at most three specs and nine reservation cells, then closes once. The expression `allocationNodes*2+referenceSlots+4` covers the three builds, three spec closures, nine reference-cell closures, reservation commit, root handoff, terminal allocation close and its queue removal. These combine with the preceding scan expression into `allocationTurns`.
- The `+16` per tree level covers `TreeEdit.advance` (around 211) search, rebuild, work handoff and phase handoff, plus up to twelve temporary-owner visits for three replaced rotation nodes (node/entry/two child edges). Each retained edge is a separate `NumericIndexRetirement.advance` visit (around 75); no subtree is released by one visit.
- The separate `+9*height` covers the initial `NumericIndexEdit.advance` lookup (around 501), plus up to four owner visits for each obsolete node on each of the two old-source spines. This was corrected from the initial `+3*height` expression so source-spine retirement is attributed directly rather than relying on excess allowance elsewhere.
- The fixed `+64` covers edit entry/publish/ready transitions, two tree construction/handoff transitions, retained source-alias decrements, final entry reference release, two old-index/edited-index retirement queue wrappers, and the scene `#save` begin/result/source-swap/delivery transitions. These are fixed metadata operations outside height-dependent traversal; the allowance exceeds their fewer than 32 distinct transitions.
- The final `+4*inputBytes` reserves tag read, frame push, frame detach and container delivery around the explicit scene stack. The nested-options fixture has one frame per option byte and one scalar byte. It contains no long map comparisons, which have their own byte-work law.

R7 executed the first derived formula with monotonic and no-progress assertions: four passed, 542 skipped, 546 total; start 16:43:52, 7.73 seconds total. The subsequent `+9*height` attribution correction only expands the conservative test termination allowance; the runtime code and grants are unchanged. Its next canonical run remains pending.

The native `owned_scene_neutral_vectors_match_native_serde_packet` test is authored in the scene crate's existing pack tests against the same 19-case fixture. It exercises actual serde serialization and the native scene codec, including enum, bytes, BOM and prototype-named map keys. It has not been compiled or run; the sole native compiler owner was notified.

## Remaining Obligations

This parser is not mounted in live Interpreter, PluginRuntime or wgpu. Typed scene schema validation/projection, nested JSON/pack field preparation, host session ingress/read ownership, native fixture parity, full maximum-record envelope and empirical outer-poll timing remain unverified. The old synchronous Interpreter scene conversion remains explicit and must be replaced before UiNodeView adopts retained bytes. The exact per-instance aggregate and transport-owned ACK/close handoff are also still required. No full renderer, live WASM, browser or end-to-end reactivity credit is claimed from these four tests.
