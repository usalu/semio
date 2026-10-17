# Strong Multiset Witness and Cursor Lifetime Research

Prospective design only. Direct primary URLs were read; no search tool, Source edit, runtime experiment, mounted census or cap change occurred. Existing failures remain failures.

## Primary Algorithm Evidence

Bitcoin Core documents order-independent insertion/removal and associative combination, and explicitly says the accumulator does not detect prior membership or enforce uniqueness. That distinction matters for malformed/repeated directory entries. [Bitcoin Core interface](https://raw.githubusercontent.com/bitcoin/bitcoin/master/src/crypto/muhash.h).

Its implementation hashes an element with SHA256, expands to384 bytes with ChaCha20, multiplies modulo2^3072−1103717, and finalizes by SHA256 of the384-byte little-endian reduced product. Removal uses a denominator and one final inverse; insertion-only census needs no removal API. This is a cryptographic multiplicative-group construction, not XOR or an unreviewed sum. Do not assert a numerical security level or substitute a smaller modulus from this audit. [Bitcoin Core implementation](https://raw.githubusercontent.com/bitcoin/bitcoin/master/src/crypto/muhash.cpp).

The independent Python implementation uses six ChaCha20 blocks with counters0–5, the same modulus/byte order and a shared C++ final test vector. It is a suitable third-party test oracle for a future first-party implementation, behind a test-only interface with pinned reviewed revision/license. No runtime external dependency is necessary. [Bitcoin Core Python oracle](https://raw.githubusercontent.com/bitcoin/bitcoin/master/test/functional/test_framework/crypto/muhash.py).

## Necessary Witness Contract

A future schema-owned algorithm discriminator must identify exact MuHash3072 framing and mapping. Insert complete domain-separated canonical entry frames including exact relative path, kind, metadata and content SHA; preserve separate application/service accumulators and raw journal framing. Final counts/type counts/logical bytes remain explicit and safe-integer/volume bounded. Both full censuses must compare complete results; current external-writer non-atomic limitations remain.

Permutation must agree; addition/removal/duplicate multiplicity, renamed paths, changed metadata/content, empty input, Unicode/BOM names and domain confusion must differ under independent oracles. MuHash is a multiset commitment, not a uniqueness detector: actual directory cursor/roster contract must guarantee each actual name is visited once, or reject duplicates using an explicitly bounded mechanism. Do not silently equate repeated entries with a set. Existing SHA stream results are not algorithm-compatible; greenfield change requires coherent schema/assets/tests and fresh captures, no legacy fallback.

## Hashing Alone Does Not Solve Cursor Lifetime

Current sorted64-name windows rescan because cursor closes before child descent. An order-independent digest removes sorting, but closing/reopening Node Dir loses the cursor position. Reopening and skipping previously read positions still scans O(N²) in the worst case and performs real per-IO checks; no counter cache may cross awaits. Portable seek/reopen cookies are not established by Node Dir and cannot be assumed from platform APIs.

A true recursive single-pass DFS retains one live parent directory cursor per depth plus the current file descriptor. Existing ArtifactDescriptor protocol allows32 owners while witness depth bound is64; retaining all64 parent cursors would violate existing custody or introduce new refusal, and increasing the cap is not authorized. A claimed universal two-descriptor DFS for an arbitrary tree with neither retained parent cursor, unbounded names, resumable cookie nor temporary storage is not justified.

## Bounded Useful Alternative

For wide mostly-file directories, a deliberately reviewed two-descriptor lifetime can keep one directory cursor open while each bounded file is hashed and closed; it closes before descending into subdirectories. This changes the old close-before-file-processing discipline but does not increase numeric caps (one directory plus one file, below32). Preserve complete directory stamp/ancestor checks around the cursor/file sequence, no uncontrolled callbacks, and descriptor-first retention on cancellation.

Keep at most64 subdirectory names per existing window and close the parent cursor before recursive subdirectory descent. Process non-directory entries only on the first full scan; subsequent scans select only the next bounded subdirectory window. This avoids rehashing files and is linear for a flat directory, but worst-case directory-heavy trees still rescan. Report that limit honestly. It uses existing per-depth64-name memory, no global name array or temporary store, and real phase/EX/observer checks on every IO.

True universal linear traversal requires a separate approved cursor-lifetime strategy or temporary-storage contract; MuHash alone cannot provide it. First benchmark exact duplicate-check reduction already held, then choose the smallest lifetime change supported by actual custody counterlaws. No speed or120-second completion guarantee is established here.
