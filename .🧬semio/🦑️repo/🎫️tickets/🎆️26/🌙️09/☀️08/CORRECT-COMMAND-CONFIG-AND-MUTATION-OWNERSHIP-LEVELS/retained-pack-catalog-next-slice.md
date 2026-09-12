# Retained Pack Catalog Next Slice

## Boundary And Prerequisite

This root review follows the source/backing implementation in `retained-pack-physical-source-ownership.md`. Finish that slice's selected native suite and release Cargo to FEM28 before beginning the catalog source change. This document specifies work; it claims no new native result. Do not combine the generic Store capability rename or shared hydrator activation into this slice.

The catalog belongs to framework Pack. It owns validated symbols, chunk entries, observed chunk framing and manifest coordinates. Exact artifact/window identity and typed domain snapshots belong to later consumers. Existing canonical and codec 1 documents must continue through the same grammar.

## Current Source Evidence

- Pack format source `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1987` has three eager Vec rosters. Constructor lines 2012–2047 reserve all capacities synchronously. The symbol parser near 1850 owns a fourth current String and reserves it after reading a length.
- `grant` removes the pending event before parsing. Allocation refusal must preserve that event or retain an explicit continuation; merely returning a blocked result after removal would lose input.
- `take` near 2168 constructs a cold Manifest through `resolve_manifest`, transfers two Vecs, clears observed headers and sets handed_back. This does not retire the observed roster backing. Manifest may own a schema String, so a retained handoff must not hide that allocation.
- `close_step` near 2212 pops logical entries and reports zero physical bytes. The current symbol parser's partial String is absent from the terminal witness.
- `symbol_chars` performs a full character count and `symbol_char` uses chars().nth(index). Generation2d/3 snapshots and mutation decoders request successive scalar indices, so copying a long symbol repeatedly scans its prefix. A new representation must allow bounded scalar access rather than keeping this quadratic read pattern.
- There are two real mounted snapshot catalog consumers, Generation2d and Generation3d; the corresponding mutation consumers use the separate RetainedRecordBodyCursor. The value/record-body physical stack repair is a subsequent slice, not already covered by this catalog change.

Line positions are observations and may shift under concurrent edits.

## Required Contract

Use paged scalar or UTF-8 storage plus paged symbol spans, and paged chunk/observed-header rosters. A scalar pool can provide constant-cost scalar indexing; a UTF-8 pool needs a byte-position cursor or an accounted index to avoid rescanning. Preserve exact UTF-8 wire bytes, including empty and multibyte symbols. The shared collection owner remains Store-independent.

Construction is allocation-free. Keep separate logical symbol count, total symbol byte/scalar limits, chunk count and total physical allocation credit. Every allocation and release uses actual backing capacity including list metadata. No Vec<String> or unresolved String may remain outside that ledger. An over-grant allocation stays retained under a sticky fault and is closable with its actual size.

Each caller grants one allocation, parser event/token, item retirement or physical backing release explicitly. Allocation is a distinct scheduling opportunity. A pending input event is neither consumed twice nor lost when the next allocation cannot fit. The scheduler must not manufacture its own physical grant from a query inside ordinary byte admission. Expose the next required allocation/release to the real caller.

The completed retained catalog must either keep ownership behind borrowed scalar/metadata access until explicit close, or hand a move-only retained owner to the consumer with the same close contract. Do not materialize a cold Vec/String catalog during a supposedly retained take. Manifest schema resolution can retain the validated symbol reference/span and borrow scalars. Preserve the cold PackFile/Manifest API separately where existing non-retained parsing uses it; no compatibility shim is needed for the retained API being replaced.

Terminal empty covers every backing and pending partial symbol, including the transferred-output owner if one exists. Insufficient or zero grants leave pointers, capacities, initialized lengths, input cursor and ledgers unchanged. Logical pop and physical backing release remain separate progress categories.

## Schema-First Evidence

Extend the Pack neutral corpus and strict schema before production changes. Use independent Ajv/JSON Patch plus TextEncoder/TextDecoder or another existing independent codec to validate exact empty, ASCII and multibyte symbol values and bytes. Required rows include multiple individually valid symbols exceeding the cumulative limit; maximum and maximum-plus-one symbol/chunk counts; multibyte codepoints crossing input pages; malformed/truncated UTF-8 after a valid prefix; insufficient allocation and close grants; cancellation during a partial symbol; and transferred catalog cleanup if that route is retained.

Native laws must drive the actual cursor and actual backing allocator, preserve the first parser/allocation fault through cleanup, compare complete catalog/document output with PackFile full verification, and assert aggregate allocations equal aggregate physical releases. Keep the existing multi-byte chunk and compressed canonical laws. Tests must explicitly retire owners before an assertion can unwind through retained Drop guards. Register additional filters on the existing Bun/Nx route and preserve its launch entry.

Both mounted Generation snapshot consumers must compile and execute after the API change. Their physical-allocation steps must be reflected in their real fuel use. Do not claim complete mounted/Store retained ownership: value stacks, domain field backings, inflater ownership and archive ingress remain separate pending work.
