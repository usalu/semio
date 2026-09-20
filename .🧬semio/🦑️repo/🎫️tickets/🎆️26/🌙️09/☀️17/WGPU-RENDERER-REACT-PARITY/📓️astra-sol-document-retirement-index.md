# Sol Shell Document Retirement Index

## Measured problem

Checkpoint 14 attributes 1,043.5 ms of self time to `ShellDocumentRetirementRegistry::close_one`.
The registry has 512 fixed owner slots. Every page retirement currently scans from its cursor across
as many as all 512 `Option` slots to find one occupied owner. `close_document_one` separately calls
`terminal_is_empty`, which scans the same 512 slots before deciding whether the process-wide
retirement lane should run. A sparse registry therefore pays slot capacity for every one-page
owner step and every empty check.

## Preserved contract

The repair will retain the existing boxed 512 owner slots, per-slot monotonic epoch, document
generation/surface witness, exact-owner refusal, cursor progression, per-page close grant, and
terminal-empty witness. It will not reduce capacity, drop a nonterminal owner, reconstruct a refused
lease, change the retirement grant, or change the global document/value close order.

## Proposed index

Add one fixed eight-word `u64` occupancy bitmap and an exact occupied count.

- Admission finds the lowest valid zero bit, preserving the current lowest-vacancy rule. Epoch
  overflow still refuses the exact incoming owner before any slot/bitmap/count mutation.
- A successful admission fills the existing slot, sets its bit, and increments the count.
- `close_one` finds the first set bit at or after the existing cursor, then wraps once. This preserves
  the current physical-slot round robin and updates the cursor to `index + 1` exactly as today.
- A nonterminal page leaves its bit and count unchanged. A terminal-empty document is removed only
  after the existing witness, then clears its bit and decrements the count.
- `terminal_is_empty` becomes `occupied_count == 0`. Debug/test invariants compare the bitmap/count
  with the owner slots; the hot path does not scan or allocate.

The bitmap adds 64 bytes plus one count to the fixed registry. Sparse lookup touches at most eight
machine words instead of 512 `Option` discriminants. No heap allocation or string creation enters
the page loop.

## Neutral law and independent oracle

The language-neutral `♻️shell-document-retirement-index` schema/fixture fixes capacity 512, word
width 64, word count 8, a sparse `{1,130,511}` population, a wrapping cursor, terminal and
nonterminal pages, lowest-vacancy reuse, per-slot epoch increments, and exact occupied counts.

A TypeScript `Set` model independently executes every fixture operation. It validates the fixture
with Ajv, scans from the cursor with wrap, chooses the lowest vacancy, increments only the admitted
slot epoch, and requires the final count to reach zero. Focused Bun+Nx Vitest passed **1 file / 2
tests**.

Production remains intentionally untouched while the parent native compile samples the preceding
Settings/Select checkpoint. The Rust product law and bitmap implementation will land together after
that release; no native or runtime performance result is claimed yet.

## Production repair and source validation

After the compiler snapshot release, the registry gained the planned eight-word occupied bitmap and
one `u64` occupied count. A compile-time array equality pins their exact fixed storage to **72
bytes**. Admission searches at most eight words for the lowest valid zero bit, checks epoch overflow
and the exact vacancy before transfer, then updates owner/bit/count together. Page close searches at
most eight words from the existing cursor with one wrap. It clears membership only after the lease
reports a complete close and `terminal_is_empty`; a nonterminal owner remains indexed. Empty checks
now compare the scalar count and preserve the existing fallback to the process-wide document close
lane.

Two native laws extend the existing refusal/ownership corpus. The fixture-driven law constructs the
actual sparse registry and executes every cursor, terminal, admission, vacancy-reuse, epoch, and
count transition while checking every one of the 512 owner/bit memberships. The second law drives
real `close_one` page grants over owners in slots 1, 130, and 511, verifies the wrapping cursor
sequence, and requires bounded convergence with slots, words, and count all empty. Existing laws
continue to cover ninth-owner aliases and exact max-plus-one refusal.

Validation performed locally without Cargo:

- focused Bun+Nx neutral/Ajv/independent oracle: **2/2 passed**;
- schema and fixture parse through `python3 -m json.tool`;
- production and native law parse through `rustfmt --edition 2021 --emit stdout`.

Root owns native compilation, runtime profiling, and fresh performance evidence. No latency
improvement or native pass is claimed before those gates run.
