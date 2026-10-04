# Bounded PNG Reconstruction — 2026-10-03

Schema-first Rust/TypeScript PNG candidate jobs now reconstruct every baseline color type and legal bit depth, packed samples/palettes, full-sample tRNS, all five row filters and Adam7. Stored, fixed and dynamic DEFLATE use first-party cursors. The existing native public decode_png now drives the same validated decoder synchronously, removing its duplicate whole-image path. Caller-scheduled progress/cancellation use PngDecodeJob; the convenience function remains synchronous.

This is progress toward the end-user editor goal. Production Draw PNG remains the reduced scene bridge; responsive document/image/font preparation, editor export scheduling and real downloads still need integration. No browser import/download was performed here. The goal and ticket remain active.

## Ownership and Bounds

Input admits at most 64 MiB, 65,536 chunks, 16,777,216 pixels and sides up to 16,384; caller limits can be lower. Constructors inspect only the small contract/signature. Chunk CRCs advance one byte per work unit. Metadata ordering, consecutive IDAT, critical chunks, CRCs and exact first/last markers are enforced. Unknown ancillary chunks, including future reserved-bit names, are checked and ignored. IDAT ranges reference source bytes without a compressed-data concatenation.

Inflation admits at most one compressed byte and emits at most one raw byte per advance. The ring is no larger than the declared zlib window, capped at 32 KiB. Reconstruction keeps two packed rows and private RGBA, with no full inflated intermediate. Filters advance per byte; expansion advances per pixel. Native buffer initialization uses 4 KiB grants following admitted capacity reservations. TypeScript allocates its admitted contiguous typed buffers; grants do not interrupt platform allocation.

Native retained-history allocation is one bounded allocation up to 32 KiB. A private owner performs required logical/physical release on completion, cancellation, failure and early disposal. The earlier assumption that this allocator supports 4 KiB pages was disproved by inspection and execution; no paging behavior is claimed.

Exact expanded scanline length, final-block termination without whole unused compressed bytes and Adler-32 are required before publication. Malformed Huffman tables, reserved counts and invalid history distances reject; valid literal-only compression may have an empty distance alphabet. Incomplete/failed/cancelled candidates cannot publish. TypeScript yields between grants, checks AbortSignal around observers, and cancels on observer failure. Source bytes stay immutable until completion.

## Fixtures and Oracles

Thirty-two shared PNG fixtures declare literal input bytes and complete expected RGBA. Both implementations reproduce every byte under grants 1, 7 and 4096. Native png independently agrees on full RGBA. pngjs validates visible RGB and every alpha value: its tRNS path clears hidden color, so only zero-alpha RGB is normalized in that independent comparison. Own expected-byte tests preserve and check those hidden channels. Low-bit samples use pngjs scaling; 16-bit samples use unscaled output and explicit high-byte conversion.

Third-party zlib compression and native png encoding produce independent 257 × 41 images exceeding the 32 KiB raw history window. Lifecycle tests check cancellation, private output, owned publication, observer errors, yielding and unfinished native disposal. All **3,535 truncated fixture prefixes** refuse publication in both languages.

Thirty-eight negative fixtures cover chunk ordering/CRC, palette/transparency, resource limits, headers/filters, insufficient/excess expansion, zlib checks/dictionaries/Adler, trailing DEFLATE, stored complements and Huffman alphabets. Executed red tests proved the old native public decoder accepted duplicate IHDR and both cursors accepted incomplete alphabets; these now reject. Native png and public Node:zlib independently reject the crafted malformed alphabets.

pngjs's private bounded-inflate path under Bun returned invalid pixel buffers instead of consistently throwing on malformed streams. Run 14020 exposed that oracle assumption. Public Node:zlib now checks the malformed single-IDAT compression fixtures; pngjs remains the successful-PNG oracle. No third-party runtime codec was added.

## Executed Checks

| Check | Terminal result |
| --- | --- |
| Stub reds 59092 / 16260 | TS: 35 expected failures; native compiled and failed new group, fail-fast stopped siblings |
| Native allocator/release attempts 91622 / 36027 | Incorrect ceiling rejected; required physical release exposed and corrected |
| Corrected focused native 85388 | 3 passed; neutral/malformed diagnostics emitted |
| Public decoder red 36428 | Failed on duplicate IHDR acceptance |
| Huffman reds 25644 / 23948 | New incomplete-alphabet regressions failed before correction |
| Final full pixels TS **4276** | **296 passed, zero failed, 912,082 assertions, 8 files; strict production typechecking passed** |
| Final pixels/DEFLATE native **77383** | **78 passed, zero skipped, 2 binaries; neutral, malformed, prefix and disposal diagnostics emitted** |
| Full Draw TS **31698** | **361 passed, zero failed, 228,871 assertions, 35 files; strict/field/publication checks passed; independent PDF raster check executed** |
| Launch generation **81054** | Exit 0; seed and generated PNG entries confirmed |
| Scoped tracked whitespace | Exit 0 |

Preceding native 37720 passed 77 tests but nextest marked one process leaky. Final expanded 77383 passed all 78 without leaky/skipped status; this is runner process metadata, not allocator-leak measurement. Unfinished-job physical release is independently exercised.

Native Draw 8607 stopped at four UI callers of the newly typed ValueError constructor. Those callers now assign InvalidValue explicitly, following the inspected refusal API and UI AGENTS.md. Corrected **39610 is terminal and unsuccessful**, stopped by shared store/DSL/codec typed-error incompatibilities from concurrent infrastructure changes. No current full native Draw pass is claimed. The earlier 491-test result is historical, not verification of today's tree. No concurrent store/value implementation was rewritten here.

## Source and Remaining Work

Owned changes:

- Pixels PNG decode schema, valid/invalid fixtures, Rust/TS jobs and tests under `📷️png/📥️decode`.
- First-party TS DEFLATE cursor under `🗜️deflate/📥️decode`; native alphabet validation and terminal unused-byte getter.
- Pixels root PNG entry point, existing owner script and Nx inputs.
- UI icon-name/value and component callers of the typed refusal constructor.
- Authoritative launch seed and regenerated launch.json; this report and acceptance/preparation updates.

Full/focused verification stays in the existing owner script and launch order. Native full verification now includes the sibling DEFLATE corpus; Nx inputs include its source. Logs remain under this active ticket's 🗑️generated. No modifying Git, worktree, runtime dependency or extra script was introduced. Repo ticket MCP tools are unavailable in this inventory; no ticket lifecycle operation was claimed.

Remaining: bounded base64/data-URI and asset preparation, actual MIME admission, document/trace/boolean preparation, font resolution/outlines preserving family/weight/spacing, responsive editor/IO scheduling, scene-to-PNG encoding, rebuilt end-user import/download/round-trip verification, color-profile policy and broader multiuser journeys. This decoder returns source RGBA8; it does not implement color management or APNG animation. The existing scanline decoder has a different ownership contract and retains eager interlaced preparation.

Primary format references were inspected in [the preparation report](📥️image-decode-preparation.md): [PNG](https://www.w3.org/TR/png/), [DEFLATE](https://www.rfc-editor.org/rfc/rfc1951.html) and [zlib](https://www.rfc-editor.org/rfc/rfc1950.html). Limits and oracle observations above are local implementation contracts and executed evidence.
