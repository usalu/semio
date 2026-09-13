# Retained Pack Inflater Physical Implementation

## Result

The retained codec-1 path now owns one finite DEFLATE history allocation and carries its actual capacity through the Pack segment cursor and both real Generation snapshot sessions. Construction is allocation-free. Compressed input remains pending until the exact history demand is granted. Identity segments allocate zero bytes. Sequential compressed segments reset logical decoder state while retaining and reusing the same backing. Cancellation retires pending input, logical history, inline decoder state, and then the physical ring in that order.

This closes the retained inflater/segment allocation boundary described in `retained-pack-inflater-physical-design.md`. It does not close typed Generation collections, typed mutation builders, retained SPR/history, archive ingress, recursive members, persisted-document factories, or cold whole-file `PackFile` allocation.

## Physical Owner

`RetainedInflateHistory` in `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs` owns `ManuallyDrop<Vec<u8>>`. Its logical target is `min(maximum_raw_segment_bytes, 32_768)`. Its physical ledger is the vector's observed capacity, not a fixed page estimate. The constructor rejects a target above the separate construction-time physical ceiling and rejects ceilings outside the signed allocation range.

`next_retained_allocation_bytes` advertises the complete finite ring target before any compressed byte can be consumed. Zero and subexact grants preserve the pending byte, parser state, pointer and ledger. The reserve path checks the observed capacity against both the caller's current grant and the construction-time ceiling. A breached ceiling becomes the first sticky retained fault while the backing remains owned for exact close.

The retained ring tracks logical length and write position. `read_back` rejects distance zero and distance beyond available history, then resolves valid distances with checked circular coordinates. Once full, writes replace the oldest byte. A reset clears logical history and parser state without changing the allocation or pointer.

The remaining DEFLATE scratch is inline. Huffman symbols use `[u16; 288]`; dynamic literal/length and distance code lengths use `[u8; 318]` plus an initialized length. No dynamic-Huffman vector remains on the retained decode path.

## Segment and Mounted Propagation

`DeflateRetainedCursor` now exposes allocation demand, reserve, actual bytes, pointer, release demand, bounded close and terminal-empty witness. Its first `RetainedMalformed` remains sticky. It rejects later ingress while faulted and retains any pending byte for close.

`RetainedPackSegmentCursor` receives a separate inflater physical ceiling. A compressed Begin creates or resets the allocation-free retained decoder and publishes Begin once. Payload admission then blocks until the history allocation is admitted. Completion keeps the inflater owner for reuse instead of dropping it. Segment close retires a pending source event, delegates inflater logical and physical close, and finally retires the inline segment state.

The Generation2d and Generation3d mounted snapshot sessions include segment demand in the same query/reserve order and include `segment.allocated_bytes()` in their aggregate retained ledger. Their close paths forward exact segment `released_items` and `released_bytes` before anchor and source retirement. The mounted laws capture `retained_allocated_bytes()` before cancellation, sum every returned `released_bytes`, and require exact equality at terminal close. The total deliberately has no portable hardcoded constant because source, catalog and value allocations use allocator-observed capacities. The inflater contribution is exactly 32,768 bytes in the recorded native environment and is released once.

No Store capability, document factory, WindowConfig API, codec id, cold `PackFile`, or one-shot compression API changed.

## Schema and Independent Oracles

The Pack schema and fixture specify the 32,768-byte history limit, 318 dynamic code lengths, 288 Huffman symbols, stored/fixed/dynamic/window-wrap blocks, distances 1 and 32,768, repeat codes 16/17/18, allocation-before-input, zero/subexact refusal, actual backing accounting, cross-segment reuse, identity zero demand, sticky first fault, close order and terminal zero ledger.

The language-neutral law validates the contract with Ajv 2020 and uses fast-json-patch for the terminal transition. The native byte-result oracle uses the existing dev-only `miniz_oxide`; no runtime dependency was added. The maximum-distance fixture is a handcrafted first-party fixed-Huffman stream over a 32,768-byte prefix and its duplicate, independently decompressed by `miniz_oxide`. This matters because the earlier generic compressor fixture produced no back-reference and observed maximum distance zero; that was a fixture-coverage failure, preserved in `🗑️generated/retained-pack-inflater-native-low-green-2.log`, rather than a decoder rejection.

The updated schema also contains the planned next typed-persistence ownership contract. Its neutral Ajv/fast-json-patch law passed separately in `🗑️generated/retained-pack-typed-persistence-neutral-2.log`. The first launch attempt used the wrong local Nx path and is retained as command evidence in `🗑️generated/retained-pack-typed-persistence-neutral-1.log`; it is not a test result.

## Native Evidence

The focused lower result is `🗑️generated/retained-pack-inflater-native-low-green-3.log`:

- retained inflater: 3/3;
- actual allocation and exact release: 32,768/32,768 bytes;
- distance-1 overlap: observed;
- distance 32,768 across ring wrap: observed;
- dynamic repeat codes 16, 17 and 18: observed;
- retained pointer reuse across two direct decoder resets: observed.

The focused Pack result is `🗑️generated/retained-pack-inflater-native-focused-2.log`:

- retained inflater: 3/3;
- retained Pack segment laws: 2/2;
- identity allocation: 0 bytes;
- three compressed segments: one backing pointer;
- cancellation order: pending input, history logical, decoder logical, history physical;
- segment history release: 32,768/32,768 bytes.

The complete selected caller result is `🗑️generated/retained-pack-inflater-native-callers-1.log`. Nx exited zero in 4 minutes 49 seconds and passed:

- deflate physical laws: 3/3;
- Pack inflater physical laws: 2/2;
- compact retained-DEFLATE fault law: 1/1, including its 28/28-byte small-ring release;
- cold compressed PackFile codec-1 round trip: 1/1;
- Generation2d mounted laws: 2/2;
- Generation3d mounted laws: 2/2.

Both mounted suites require accumulated physical release to equal their complete runtime-observed source + catalog + value + inflater allocation ledger before terminal close. No fixed 4 KiB release is substituted for the ring or any other backing.

## Source and Command Ledger

- `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs`
- `🧰️framework/🔨️modules/🗜️deflate/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🟦️.ts`
- `🧰️framework/🔨️modules/🎒️pack/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🎒️pack/🧫️fixtures/🔣️.json`
- Generation2d and Generation3d snapshot `💾️binary/🦀️.rs` mounted sessions and retained-mounted laws
- root `📜️script.ts` and `📋️project.json`
- ticket `validation/project.json`
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`, orders 311.244 and 311.245

## Remaining Boundary

The mounted typed snapshot owners still allocate container stacks, strings, row and presence vectors, nested JSON/DSL collections, ordered maps and final domain collections without caller admission or an actual physical ledger. The typed mutation owners have the same class of hidden owners. Their Boolean logical close methods drop backing without exact byte release.

`RetainedHistoryDecode` still constructs an ordinary `HistoryLog`, dictionary strings, edit-id strings and nested record/payload vectors. Member open also copies the complete verified SPR span into `Vec::with_capacity(total)`, clones decoded history into a second envelope representation, and truncates logical bytes before dropping capacity without reporting it. These are the next retained ownership boundaries. The bounded plan is `retained-typed-snapshot-mutation-and-spr-history-design.md`.
