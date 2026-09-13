# Retained Pack Value and Record-Body Physical Ownership

## Accepted Boundary

This slice replaces the synchronous retained-value stack allocation and the record-body `Vec<String>` registry with bounded owners. It connects their allocation, logical retirement and physical release protocols to the real Generation2d and Generation3d snapshot and mutation consumers. Constructors are allocation-free, physical ceilings are finite and checked before allocation, and every admitted backing remains owned until an exact close grant releases its actual capacity.

This is a bounded Pack-level slice. It does not establish full retained document persistence. Retained DEFLATE history, typed Generation collections and builders, archive/member ingress, recursive child persistence, persisted-document factories, and several diagnostic allocations remain open.

## Ownership Design

### Value stack

`RetainedValueCursor` owns `ManuallyDrop<Vec<Expect>>`. Its logical frame ceiling is checked `max_depth * 8`; its separate physical ceiling must cover the corresponding `size_of::<Expect>()` request and fit `isize::MAX`. Construction allocates nothing.

The cursor exposes the exact next allocation demand, refuses zero and subexact grants without changing pointer, capacity, initialized roots, pending input or the allocation ledger, and records the allocator's actual capacity. Allocator overgrant beyond the constructor ceiling becomes the first sticky fault while the backing remains available for close. Two root frames initialize in separate admitted logical turns after allocation. No input byte is accepted before both roots exist.

Close retires pending input and logical frames before advertising the exact stack-capacity release. A physical release never consumes logical work fuel. The terminal witness requires closed state, zero frames, zero capacity, zero physical bytes and no pending input. Logical pushes check the declared frame ceiling as well as actual vector capacity, so allocator overcapacity cannot enlarge the grammar limit.

### Shared symbol table

`RetainedPackSymbolTable` owns paged inline symbol spans and Unicode scalar values. The table is shared by the full Pack catalog and the OS record-body cursor. It enforces maximum symbols, cumulative UTF-8 bytes, cumulative scalar values, consecutive spans and checked coordinate conversion at the storage owner.

`PagedList::next_capacity_allocation_bytes(target)` makes capacity demands target-aware. A count beyond one leaf therefore requests the next real page even while the current leaf still has unused logical slots. Symbol ids and span coordinates use checked `u64` to `usize` conversion and checked addition, including the language-neutral 32-bit `2^32` boundary.

Allocation uses actual PagedList page and metadata capacity. Zero and subexact grants preserve state. Actual allocation and admission failures preserve the first compact table fault. Close removes all logical scalars and spans before exposing exact scalar backing and span backing releases. Terminal state requires empty logical lists, empty page/metadata allocations and an all-zero allocation ledger.

### Record body

`RetainedRecordBodyCursor` owns the shared symbol table and an optional retained value cursor. It accepts separate maximum symbol count, cumulative UTF-8, cumulative scalar and aggregate actual-allocation ceilings. Its allocation order follows the pending parser event: target symbol spans, scalar storage, then the value stack. Query and reserve use the same owner and target.

The final byte of a nonempty final symbol can complete both a Unicode scalar and the catalog. The cursor retains `CatalogComplete` as a separate pending event after publishing `SymbolChar`, blocks later ingress until that event is observed, and preserves it across the value-stack allocation turn and cancellation.

Close proceeds through value logical state, symbol logical state, value physical backing and symbol physical backing. The nested value cursor receives an item grant only after its physical demand is empty, which allows its terminal metadata transition without releasing backing early. A global zero-item/zero-byte call is a strict no-op, including an unopened cursor already in cancellation. The explicit unopened law also proves the allocation-free `ValuePhysical(None)` transition under a positive byte-only opportunity.

### Mounted consumers

The Generation2d and Generation3d snapshot binary sessions include the retained value stack in their allocation demand, reserve, allocation ledger and close routing. The two mutation binary sessions do the same for the complete record-body aggregate.

Both mutation authority laws first refuse a subexact external allocation grant, preserve the admitted owner and ledger, then allocate exactly. During cancellation they first refuse a subexact physical release, preserve the owner, and sum every structured physical release. Accumulated physical release must equal the actual admitted source and body capacity; logical work remains a separate item count.

No Store lifecycle, factory, exact-window or generic persisted-artifact capability changed in this slice.

## Schema and Independent Oracle

The retained Pack JSON schema and fixture now specify the value frame relation, partial root initialization, record-body cumulative UTF-8/scalar limits, a 512-symbol multi-leaf case, and the 32-bit first-unrepresentable symbol coordinate. The TypeScript law validates the fixture with Ajv and independently checks UTF-8/scalar behavior with the platform `TextEncoder` and fatal `TextDecoder`.

The registered neutral target `abstraction-ownership-validation:framework-retained-pack-value` passed with Ajv, fast-json-patch, the platform UTF-8 oracle and strict TypeScript. Its final fixture also covers the final-scalar/catalog two-event handoff and unopened zero-grant cancellation. Durable evidence is in `🗑️generated/retained-pack-value-neutral-4.log`.

## Native Law Evidence

The lower owner laws cover target-aware multi-leaf allocation, zero/subexact refusal, actual pointer/capacity tracking, checked symbol coordinates, sticky first faults, pending-event preservation, logical-before-physical close, exact aggregate release and all-zero terminal witnesses.

The focused OS laws cover partial value roots 0/1/2, input refusal before root initialization, logical maximum enforcement, record-body empty/ASCII/multibyte symbols, direct indexed Unicode lookup, a 512-symbol table, maximum-plus-one pending faults, unopened cancellation, zero grants, byte-only metadata transition and exact close.

The first focused native run exposed two stale test expectations: wire tag `0x17` is canonically emitted as `Begin(Wire)`, and sealed truncation can retire one already-admitted control frame before observing the missing byte. The fixture now follows the actual grammar, and truncation uses a finite frame-derived allowance with an explicit observed-fault assertion.

The second run passed PagedList 1/1, symbol table 2/2, catalog 3/3, codec 1 1/1 and retained value 3/3. Both record-body tests then exhausted their bounded close helper because the parent's physical phase always passed zero logical items to an already-empty nested value owner. Drop correctly aborted the nonterminal owner. The parent now forwards one item only when the nested physical demand is empty.

The third run proved that close fix: the 512-symbol maximum-plus-one cancellation law passed. Its multibyte law then exposed a lost `CatalogComplete` event after the final scalar. The pending-event field and cancellation path correct that defect. Logs `retained-pack-value-native-1.log`, `retained-pack-value-native-2.log` and `retained-pack-value-native-3.log` preserve the red/fix evidence.

The final registered native target `abstraction-ownership-validation:framework-retained-pack-value-native` exited zero in 8 minutes 19 seconds. It passed:

- PagedList target-aware capacity: 1/1;
- shared symbol table: 2/2;
- retained Pack catalog: 3/3;
- cold compressed PackFile codec 1: 1/1;
- retained value stack: 3/3;
- retained record body: 3/3;
- Generation2d mutation body and mounted snapshot: 1/1 and 2/2;
- Generation3d mutation body and mounted snapshot: 1/1 and 2/2.

Durable evidence is in `🗑️generated/retained-pack-value-native-5.log`. The run compiled all four real consumer callsites under the final API and source. It observed 13,920-byte and 18,928-byte exact shared-table releases and a 22,000-byte record-body aggregate release equal to allocation.

The compact-diagnostic follow-up adds `PackError::RetainedMalformed { what: &'static str, offset: u64, detail: &'static str }`. Every governed value and record-body malformed path now stores and returns that allocation-free payload; the successful path no longer constructs eager owned errors. Repeated malformed UTF-8 and symbol faults preserve the same first fault and allocation ledger. The new record-body law observes and exactly releases 9,456 admitted bytes after rejection.

The registered `retained-pack-value-native-6.log` run passed the neutral oracle, PagedList 1/1, symbol table 2/2, catalog 3/3, cold codec 1 1/1, retained value 3/3, retained record body 4/4 and Generation2d mutation 1/1. It then ended without a resumable exec session or Nx footer. Generation2d mounted 2/2 plus Generation3d mutation 1/1 and mounted 2/2 produced no result, so native run 6 is recorded as interrupted rather than complete.

## Source Ledger

- `🧰️framework/🔨️modules/🌱️value/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/📋️list/🧪️tests/📋️list/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🦀️.rs`
- `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🟦️.ts`
- `🧰️framework/🔨️modules/🎒️pack/🧬️schema/🔣️.json`
- `🧰️framework/🔨️modules/🎒️pack/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs`
- Generation2d and Generation3d snapshot `📸️snapshot/💾️binary/🦀️.rs`
- Generation2d and Generation3d mutation `🧬️mutations/💾️binary/🦀️.rs`
- Both mutation `🧪️tests/🔬️retained-authority-laws/🦀️.rs` modules
- root `📜️script.ts`, root `📋️project.json`, ticket validation `📜️script.ts` and `project.json`
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`, orders 311.232 and 311.233

## Remaining Ownership Work

The retained inflater and segment protocol is now implemented and accepted as the following bounded slice. Cold `PackFile`, the one-shot codecs and codec 1 remain enabled. See `retained-pack-inflater-physical-implementation.md`.

Value and record-body cursor diagnostics are now accounted: their sticky faults contain only static retained fields or another allocation-free `PackError` variant, and their four mounted callers translate them to static codes without formatting. This closes the diagnostic gap for these two cursor owners together with the accepted stack and symbol/span backing.

The pipeline diagnostic follow-up also converted anchor, segment, retained-varint and retained-DEFLATE malformed grants to the sticky static variant. No inspected retained hot method now constructs ordinary `Malformed(String)`, `Schema(String)` or `Io(String)`; those remain cold variants. This still does not close physical inflater ownership: segment close drops inflater backing synchronously and reports no actual bytes. Exact boundary details are in `retained-pack-pipeline-diagnostic-implementation.md`.

Typed mutation builders and snapshot collections still contain ordinary `String` and `Vec` owners. Retained SPR/history, archive ingress, recursive members and the common persisted-document factory remain separate required work. Their next bounded design begins in `retained-typed-snapshot-mutation-and-spr-history-design.md`. This report makes no full-retained or recursive-persistence claim.
