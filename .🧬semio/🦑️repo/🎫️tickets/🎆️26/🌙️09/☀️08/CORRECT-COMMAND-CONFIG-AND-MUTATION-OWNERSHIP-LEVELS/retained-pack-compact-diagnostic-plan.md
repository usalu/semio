# Retained Pack Compact Diagnostic Plan

## Problem

The accepted value-stack and symbol/span backing ledger does not account for `PackError::Malformed { detail: String }`. Retained value and record-body cursors can store that owned string as their sticky first fault until cancellation completes. Some formerly eager `ok_or` expressions also built and dropped a string on successful parser turns; those have been made lazy, but malformed hot paths still allocate.

Moving conversion one helper outward would leave an allocation on the same governed step. The public hot variant itself must be allocation-free.

## Boundary

Add `PackError::RetainedMalformed { what: &'static str, offset: u64, detail: &'static str }` at the shared Pack codec level. Retained parsers use this variant directly. It preserves the existing public `Result<_, PackError>` signatures while making the actual malformed variant entirely inline and Copy in content. `Display` writes the borrowed static fields without creating an owned string; a caller that explicitly calls `to_string` owns that cold diagnostic allocation outside the retained cursor.

`LimitExceeded(&'static str)`, `NonCanonical(&'static str)`, `Truncated(u64)` and the numeric codec variants are already allocation-free. The value and record-body cursor first-fault slots may continue to use `Option<PackError>` only if every error they can store is one of these allocation-free variants. Native laws must exercise all local malformed paths used by sticky faults and verify the retained variant rather than infer the invariant from the enum's type.

This slice does not change ordinary cold `PackError::Malformed { detail: String }`, `Schema(String)` or `Io(String)`. It also does not yet claim retained anchor/segment/inflater diagnostics whose delegated cold decoder paths can still return owned errors.

## Mounted Ownership

Generation2d and Generation3d snapshot sessions own any returned retained error as their first fault category, stop admitting later source events, and continue their established close order through value, catalog, anchor and source owners. Generation2d and Generation3d mutation sessions do the same for record-body errors, then close the body and source and propagate exact physical release bytes.

The retained error contains no reference to source page, symbol or stack storage. The mounted caller therefore does not need to keep a backing alive for diagnostic text. It must still drain the rejected cursor: pending byte/event, logical parser frames, then each exact physical allocation demand until the nested and mounted terminal witnesses are true.

The four concrete callers do not format or copy the retained diagnostic. Generation2d and Generation3d snapshot sessions map a value failure to the static codes `generation2d-mounted.value-malformed` and `generation3d-mounted.value-malformed`; their nested value cursor retains the original first fault until exact close. Generation2d and Generation3d mutation sessions do the same with `generation2d-mutation.body-malformed` and `generation3d-mutation.body-malformed`; their record-body cursor retains the first fault and its admitted stack/symbol backing until close. The outer authorities convert those static codes to their existing structured decode diagnostic. No caller creates an owned diagnostic string on the value/body fault route.

## Retained Hot API Inventory

| Public retained boundary | Current hot diagnostic storage | Compact status |
| --- | --- | --- |
| `RetainedPackSourceCursor::grant` and `close_step` | `&'static str`; allocation errors retain the source owner plus static cause | Inline for this boundary |
| `RetainedPackCatalogCursor`, `RetainedPackSymbolTable` query/reserve/grant methods | `RetainedPackCatalogFault { code: &'static str, offset: u64 }` | Inline for this boundary |
| `RetainedValueCursor::{try_new,next_allocation_bytes,seal,grant}` | `PackError::RetainedMalformed` or another allocation-free `PackError` variant | Closed by this slice |
| `RetainedRecordBodyCursor::{try_new,next_allocation_bytes,seal,symbol_chars,symbol_char,grant}` | `PackError::RetainedMalformed` or another allocation-free `PackError` variant | Closed by this slice |
| `RetainedPackAnchorCursor::grant` | ordinary `PackError::Malformed { detail: String }` on malformed/replay/closed paths | Open |
| `RetainedPackSegmentCursor::grant` | ordinary `PackError::Malformed { detail: String }`, including retained-varint faults and delegated inflater faults | Open |
| `DeflateRetainedCursor::grant` | ordinary `PackError::Malformed { detail: String }` | Open with inflater ownership |

No inspected retained hot method constructs `PackError::Schema(String)` or `PackError::Io(String)` directly. Those owned variants remain available to cold Pack APIs. Anchor and segment callers currently map their errors to static mounted codes, but the discarded intermediate `Malformed` still allocates during the governed step. The complete source-to-value pipeline therefore is not allocation-free yet.

## Schema-First Laws

The retained Pack fixture names the hot variant and requires inline storage, zero diagnostic heap bytes, no allocation by a public retained step, sticky first-fault identity, and cold-only owned-string conversion. Ajv and the existing TypeScript oracle validate that language-neutral contract.

Native laws will:

1. produce malformed retained UTF-8 after real stack allocation, observe `RetainedMalformed`, repeat the grant to prove the identical first static fault, and show `allocated_bytes` changes only for the stack;
2. produce a record-body malformed symbol fault, prove the pending event and actual symbol/value ledger remain owned, and close exactly;
3. assert successful retained value and multibyte record-body parsing never requires ordinary `Malformed { detail: String }`;
4. recompile and run both Generation snapshot and mutation callsites without converting the retained fault into owned text on their hot close path.

## Source Ledger

- `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`: shared public inline `RetainedMalformed` variant and allocation-free display.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs`: value and record-body hot errors, sticky first-fault storage and lazy successful checks.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs`: repeated inline-fault and exact rejected-owner close laws.
- `🧰️framework/🔨️modules/🎒️pack/🧬️schema/🔣️.json`, `🧰️framework/🔨️modules/🎒️pack/🧫️fixtures/🔣️.json` and `🧰️framework/🔨️modules/🎒️pack/📐️format/🧪️tests/🔬️retained-pack-source-laws/🟦️.ts`: language-neutral diagnostic contract and independent oracle.
- Generation2d and Generation3d snapshot `📸️snapshot/💾️binary/🦀️.rs` and mutation `🧬️mutations/💾️binary/🦀️.rs`: the four static mounted mappings verified by the registered native route.

## Acceptance and Remaining Work

The schema and fixture now require the `retained-malformed-static` hot variant, inline storage, zero heap bytes, a non-allocating public retained step and sticky first-fault identity. The Ajv/TypeScript/platform oracle passed in `🗑️generated/retained-pack-value-native-6.log`. Native value 3/3 and record-body 4/4 also passed there; the malformed record-body law preserved the pending input and actual ledger across repeated observation, then released exactly 9,456 admitted bytes.

The registered run remains live as exec session `26866` while shared Cargo work serializes. Completed filters are PagedList 1/1, symbol table 2/2, catalog 3/3, cold codec 1 1/1, retained value 3/3, retained record body 4/4 and Generation2d mutation 1/1. Generation2d mounted 2/2 plus Generation3d mutation 1/1 and mounted 2/2 remain pending in that same durable run; the process has emitted no failure. Root owns monitoring after this frozen handoff.

Acceptance closes diagnostic ownership only for the retained value and record-body cursors and their four mounted caller mappings, subject to the pending caller compile filters above. Retained anchor/segment delegated errors, DEFLATE state/history, typed builders and collections, Store factories, archive ingress and recursive members remain explicit work. Inflater production edits start only after the separate audit and authorization.
