# Retained Pack Pipeline Diagnostic Audit

## Result

The accepted source, catalog, value, record-body, and four mounted caller routes preserve their accepted lower-owner allocation and close protocols. The current end-to-end retained Pack pipeline is **not** allocation-free on malformed turns: anchor, segment, retained-varint, and retained-DEFLATE hot paths still construct `PackError::Malformed { detail: String }`. The existing public `PackError::RetainedMalformed { what, offset, detail }` is the correct compact payload, but it is not yet used for those routes.

This audit was read-only. It did not run Cargo. `🗑️generated/retained-pack-value-native-6.log` records the Generation2d mutation filter passing, then later compilation/lock progress; it is not evidence that the still-pending Generation2d mounted or Generation3d filters have passed.

## Verified Ownership by Pipeline Stage

| Stage | Current result | Evidence |
| --- | --- | --- |
| Source | No owned diagnostic is stored. Static allocation causes remain attached to the source owner; logical pages retire before exact backing release. | `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1155`, `:1316`, `:1344` |
| Catalog and symbol table | Compact `RetainedPackCatalogFault` is sticky. Pending input and active/parser/logical lists block physical release; close drains scalars/spans before their PagedList backing. | `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:2087`, `:2173`, `:2349`, `:2410`, `:2764`, `:2893`, `:2916` |
| Value | Every inspected governed malformed path uses `RetainedMalformed` or another inline Pack error. `grant` preserves the first error, and the stack is only replaced with an empty vector after exact physical bytes are granted. | `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:801`, `:1103`, `:1293`, `:1320`, `:1328` |
| Record body | The body maps table faults to static retained details, preserves the first error, keeps the catalog-complete event separate, and closes value logical → symbol logical → value physical → symbol physical. | `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:1423`, `:1534`, `:1675`, `:1800`, `:1818`, `:1831` |
| Anchor | `grant` directly creates owned `Malformed` details and has no sticky first-fault slot. Its state is inline, but its diagnostic is not. | `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:3014`, `:3047`, `:3052`, `:3078`, `:3093`, `:3133`, `:3157` |
| Segment | The retained varint and segment hot routes directly create owned `Malformed` details. The cursor has neither a first-fault slot nor an allocation/release protocol for its live inflater. | `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs:1415`, `:1453`, `:1512`, `:1529`, `:1557`, `:1648`, `:1678`, `:1698` |
| Retained DEFLATE | `grant` directly creates owned `Malformed` details. `close` clears pending input and drops the `Inflater`; the wrapped inflater owns growable output and dynamic-code-length vectors. | `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:430`, `:455`, `:488`; `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs:262`, `:278`, `:298`, `:349`, `:366`, `:430`, `:483` |

## Confirmed Value/Body and Caller Boundary

`PackError::RetainedMalformed` is public, carries only static references and an integer, and `Display` writes fields directly; allocation happens only if a cold caller explicitly materializes a string. See `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs:9-45`. It does not borrow a source page, a symbol span, or value-stack storage.

The value/body hot region has no `to_string` or `format!` construction. The remaining `String`/format calls in that file are in cold encode/full-decode paths, outside `RetainedValueCursor` and `RetainedRecordBodyCursor` (`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🦀️.rs:182`, `:2028-2844`). The value unit laws specifically assert repeated retained UTF-8 and record-body fault identity and exact cleanup (`🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🌱️value/🧪️tests/🔬️unit/🦀️.rs:899-903`, `:925-936`).

The four real consumers discard the detailed Pack error only when crossing into their pre-existing static authority code. They do not call `Display`, `to_string`, or `format!` on the value/body error route:

| Consumer | Static mapping | Exact retained close evidence |
| --- | --- | --- |
| Generation2d snapshot | `generation2d-mounted.value-malformed` | value → catalog → segment → anchor → source, with `next_retained_release_allocation_bytes` stopping behind earlier owners: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:1190`, `:1343`, `:1411` |
| Generation3d snapshot | `generation3d-mounted.value-malformed` | same mounted protocol: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:1240`, `:1393`, `:1461` |
| Generation2d mutation | `generation2d-mutation.body-malformed` | pending → typed owner → record body, and the body reports its next exact release: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1853`, `:1899`, `:1906` |
| Generation3d mutation | `generation3d-mutation.body-malformed` | same protocol: `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:1852`, `:1898`, `:1905` |

At the authority boundary these static codes are placed into `OwnedSchemaDecodeDiagnostic`, and its `close_step` keeps the session until its reported terminal close completes. The Generation2d snapshot and mutation examples are at `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:2294-2375` and `:2395-2440`; the Generation3d source has matching authority routes. This is a static structured diagnostic, not a formatted Pack error.

`mounted_pack_rt` exports retained cursors and codebase-owned Pack types, not external library error types (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:5466-5477`). Its documented boundary excludes cold whole-document helpers. The general `pack_rt` module still intentionally exports cold `PackError`/decode helpers (`:5281-5320`); that is a separate API and must not be used by a retained WindowConfig replay turn.

## Findings That Block a Whole-Pipeline Claim

1. **Owned diagnostics remain on every upper retained hot route.** The complete inventory is the retained-varint path at format line 1418; segment paths at 1516, 1534, 1538, 1542, 1648, 1678, and 1694; anchor paths at 3052, 3078, 3093, 3100, 3133, 3136, and 3157; and DEFLATE paths at codec 459, 464, 470, and 476. Each uses `Malformed { detail: "...".into() }`, allocating a `String` before the mounted caller maps the result to a static code.

2. **Upper cursors do not own a sticky first fault.** Catalog/value/body call `remember` or store `fault` before returning subsequent work (`format:2127`, `:2659`; `value:1293`, `:1800`). Anchor, segment, and `DeflateRetainedCursor` only return errors. A caller normally starts closing after the first error, but a repeated direct public `grant` can construct a later owned error. A compact fault slot must preserve the first `PackError` at those public boundaries, reject later ingress, and leave close responsible for every retained item/allocation.

3. **Segment/inflater termination currently drops retained state synchronously.** A completed compressed segment calls `inflater.close()` and removes it in the ordinary parser turn (`format:1646-1652`). Segment close again takes the inflater, calls `close`, clears its pending source event, and reports complete (`format:1698-1706`). `DeflateRetainedCursor::close` clears `pending` and drops the wrapped inflater (`codec:488-492`). This cannot prove pending-input retirement, actual allocation bytes, or exact physical release. It is the explicit boundary where the planned retained ring and segment close protocol are required.

4. **The baseline inflater is materially unbounded.** `Inflater` keeps all produced bytes in `output: Vec<u8>` and appends for literals/stored bytes/matches (`deflate:278-293`, `:349`, `:430`, `:483`). Dynamic code-length phases also own a `Vec<u8>` (`:262-264`, `:366`, `:390-410`). No upper cursor can report those actual capacities. The proposed 32 KiB retained ring plus inline maximum-318 code lengths remains necessary.

5. **The current mounted release query is sound only for today’s owners.** Snapshot sessions account source + catalog + value (`Generation2d snapshot:1263-1324`) and then close segment as a one-item owner with no bytes (`:1387-1390`). They must add segment/inflater allocation and release between catalog and anchor/source in the inflater slice. Until then, a byte demand cannot truthfully represent codec-1 backing.

## Exact Bounded Sol Scope Before Inflater Ownership

Implement this as a compact diagnostic-only slice; do not start the retained history/ring, dynamic-Huffman, segment allocation, Store candidate, or WindowConfig production work in it.

1. In `🧰️framework/🔨️modules/🎒️pack/📐️format/🦀️.rs`, replace every retained-varint, segment, and anchor `PackError::Malformed { detail: String }` listed above with `PackError::RetainedMalformed` using equivalent static `what`, `offset`, and `detail`. Keep cold `PackFile`, `PackIdentityChunkCursor`, one-shot decode, and batch codec paths on `Malformed(String)`.
2. In `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`, replace only `DeflateRetainedCursor::grant`'s four owned malformed paths with `RetainedMalformed`. Do not alter the cold `read_varint_*`, identity codec, or one-shot `deflate_decompress` diagnostics.
3. Add a first-fault slot and one `remember` boundary to anchor, segment, and retained DEFLATE. Once faulted, their public `grant` returns the exact first inline error, refuses new ingress, and remains closable. The cursor must not replace the original error during cancellation or a repeated grant.
4. Add schema/fixture/oracle fields for anchor, segment/varint, and DEFLATE hot-diagnostic storage, then native laws that force each representative malformed route after a real lower allocation where applicable. Each law must prove `RetainedMalformed`, same-first-error repetition, no formatting/materialization in the mounted route, pending logical state is closed through bounded turns, and any presently accounted lower backing releases exactly.
5. Keep the four mounted caller mappings static. Add their error-route checks only if needed to prove they call no formatting; do not switch them to a generic String diagnostic or widen their public API.

This diagnostic slice may make retained error payloads allocation-free, but cannot claim exact pipeline physical ownership while `DeflateRetainedCursor::close` and `RetainedPackSegmentCursor::close_step` still clear/drop live inflater state. That change belongs atomically to the next retained-inflater/segment API slice.

## Inflater and WindowConfig Blocking Boundary

The retained inflater slice must replace synchronous segment completion/cancellation with `next_allocation_bytes`, reserve, actual-capacity ledger, `next_release_allocation_bytes`, and `close_step(items, bytes)`. It must retire pending input before logical inflater state, release the ring exactly once, and make the mounted snapshot aggregates include source + catalog + value + inflater. It must also replace the dynamic code-length vector with bounded inline storage and verify stored/fixed/dynamic/wrap vectors against the existing dev-only oracle, as specified in `retained-pack-value-and-inflater-next-slice.md`.

Only after that lower contract exists can a WindowConfig typed token replay be made retained. The design requires the WindowConfig candidate to own its first returned diagnostic, reject later ingress, then drain pending source/body events, logical tokens/candidate state, and every exact physical release before its terminal witness. The current `mounted_pack_rt` cursors are usable lower seams, but the generic typed `ArtifactPack`/WindowConfig factory replay over `RetainedValueToken`, retained SPR/history decode, and candidate `ConfigStore` construction are still absent. See `window-config-retained-exact-pack-load-capability-design.md` under “Existing Store Runtime Boundary” and “Frozen lower fault boundary.”

## Validation Status

No new validation was run for this audit. Existing evidence supports lower value/body and the Generation2d mutation filter only; it does not close native-6 overall. After the bounded diagnostic slice, rerun the neutral oracle and focused retained source/catalog/value/body laws, then wait for the existing registered caller run’s Generation2d mounted, Generation3d mutation, and Generation3d mounted filters to finish before recording a complete caller acceptance.
