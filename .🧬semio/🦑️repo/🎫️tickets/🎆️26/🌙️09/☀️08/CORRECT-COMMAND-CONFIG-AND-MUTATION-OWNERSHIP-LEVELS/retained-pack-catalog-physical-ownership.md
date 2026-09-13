# Retained Pack Catalog Physical Ownership

## Accepted Prerequisite

The lower retained Pack source slice is accepted. The registered lower native run passes the paged owner, retained source, compressed PackFile and both mounted Generation routes. The separate outer native run passes exact close-demand propagation, non-mutating subexact refusal, aggregate source allocation/release conservation, cancellation and owner-preserving nested-ceiling rejection. Exact evidence remains in `retained-pack-physical-source-ownership.md` and the ticket generated logs.

## Catalog Contract

The catalog constructor owns no allocation. Four independently admitted `PagedList` owners retain decoded symbol scalars, symbol spans, chunk entries and observed chunk headers. A symbol span records scalar start, scalar length and exact UTF-8 byte length; sequential mounted reads use indexed scalar access without rescanning preceding Unicode text.

The constructor receives separate maximum symbol count, cumulative symbol UTF-8 bytes, cumulative symbol scalars, chunk count and total actual allocation bytes. The existing Pack grammar limits remain independent wire limits. Each allocation query previews the pending parser event without consuming it and identifies one exact next backing request. A zero or subexact grant changes no parser, pointer, capacity, initialized length or allocation ledger. An allocator overgrant remains owned, records its actual capacity, establishes the first sticky fault and stays available for exact cleanup.

The completed catalog keeps all table ownership in the cursor. Its validation handoff contains only the inline retained manifest, including the numeric schema symbol reference, hashes, counts and byte spans. It does not resolve that reference into a `String` and does not transfer a `Vec`. The cold `PackFile` and `Manifest` route stays separate.

Cancellation retires pending and inline partial-parser state, then one logical scalar/span/entry/header per item grant. Empty paged backings retire under the exact byte amount returned by their owner. Terminal empty requires no pending event or partial symbol, every logical ledger at zero and all four physical ledgers at zero. The first parser or allocation fault remains observable through cleanup.

## Mounted Scheduling

Generation2d and Generation3d expose one combined retained allocation query and reserve step. During ingress it selects the source page owner; during drive it selects the catalog owner. The outer snapshot authority consumes one job-fuel opportunity for each actual catalog allocation before granting another parser event. The session checks the sum of admitted source and catalog backing against its construction-time retained ceiling.

Close demand selects the exact catalog backing while the catalog is the active close owner, followed by the source backing. Each mounted close result forwards the actual physical bytes released by the nested catalog or source owner. No Store trait or factory capability changes belong to this slice.

## Neutral Evidence

The Pack schema and fixture now specify empty, ASCII and multibyte symbols with exact UTF-8 bytes and scalar values; cumulative byte refusal after individually valid symbols; maximum and maximum-plus-one symbol and chunk counts; a multibyte scalar crossing a 4 KiB input-page boundary; malformed and truncated UTF-8 after valid prefixes; allocation and close refusal; partial-symbol cancellation; sticky first fault; and the terminal ledger.

The registered neutral target passed Ajv 2020 validation, strict TypeScript, independent `TextEncoder`/fatal `TextDecoder` agreement and fast-json-patch cleanup. The final fixture names the cursor, rather than the inline receipt, as the terminal owner. Its clean rerun passed in 16.2s and is preserved in `🗑️generated/retained-pack-catalog-neutral-2.log`. The earlier `retained-pack-catalog-neutral-1.log` preserves the initial wrong Nx module-path attempt followed by its successful correction.

## Native Acceptance

The final registered native run passed:

- five shared PagedList laws;
- eleven Pack source/catalog laws, including empty/ASCII/multibyte scalar access, zero and subexact allocation/close refusal, sticky physical/logical/UTF-8/count faults with pending-input preservation, maximum-plus-one observed and table chunk counts, partial-symbol cleanup, actual backing pointers/capacities, a multibyte scalar crossing source byte 4095, and all-zero terminal ledgers;
- the existing compressed codec 1 PackFile/PackWriter law;
- two Generation2d mounted laws, two Generation2d outer-cancellation laws and two Generation3d mounted laws.

The three-symbol native catalog admitted and exactly released 18,928 physical bytes. The long-symbol/page-boundary case admitted and exactly released 32,368 physical bytes and matched the cold full-verification `PackFile` symbol. The multi-byte chunk retained one observed header for its five raw bytes and matched the full Pack reader.

`retained-pack-catalog-coherence-1.log` first exposed a mounted cleanup stall after Pack 11/11 and codec 1 were green. `retained-pack-catalog-coherence-2.log` records the diagnostic witness: the catalog had zero logical items but retained 18,928 physical bytes while later segment/anchor owners incorrectly hid its release demand. The mounted close-demand order was corrected in Generation2d and Generation3d. `retained-pack-catalog-coherence-3.log` is the exit-zero full source/consumer coherence run.

The sharper `retained-pack-catalog-native-3.log` then reproduced a second ordering defect: after logical scalar retirement, the cursor advertised the scalar backing while the later symbol-span list was still logically nonempty, so the same close step spent its item grant and could not release the advertised allocation. The query now waits for every logical catalog list to empty before exposing physical backing in the exact order used by close. `retained-pack-catalog-native-4.log` is the final focused exit-zero run: all three catalog laws pass, including item-only/byte-only refusal, subexact close refusal, exact 18,928-byte and 32,368-byte release, sticky faults and all-zero terminal ledgers.

## Remaining Ownership

This slice does not retain the value stack, deflate inflater, typed Generation collections, archive ingress or common Store hydration/factory boundary. The cold PackFile still owns String/Vec backings by design. This result does not claim that the whole mounted or recursive document route is physically retained.
