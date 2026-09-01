# Neutral Primary Zero-Byte Probe Boundary Before Test Mount

## Authorized Ticket-Only Refinement

The seven names and all previous ownership/layout/capacity constraints are preserved. The ticket leaf now has one allocation-free `refused_grants(bytes)` helper, used at all five changed short-probe loops: shared phase frontier, primary allocation inline, pointerless poisoned Refund/Clear, capture inline, and paused pin/cursor-clear inline. The shared phase frontier also covers both ordinary/primary construction, recovery and changed close-latch bools.

For every positive required byte count the exact probes are:

1. zero items with the required bytes: ResidentGrant::new(0, bytes);
2. one item with zero bytes: ResidentGrant::new(1, 0);
3. one item with one-short bytes: ResidentGrant::new(1, bytes - 1).

When bytes==1, the third equals the second and is deduplicated. A zero required count is an explicit Count error. The array/flatten iterator has fixed stack backing; no Vec, allocation or grant increase is introduced. These are three boundary probes, NOT all byte lengths.

The historical R3 reference receipt still describes the earlier57728a88 Rust leaf. Its source model result remains valid for that captured source, but it is not relabelled as a run of this revised leaf. No new source reference or native command has run for this revision. The new helper and five call sites have been read and enumerated as source only; compilation/behavior are unexecuted.

## Fresh Pre-Mount Source Boundary

```text
4f6dc45ffc159ee419529114f0eeb2a95f8ab6e6982436a6aa0a5f9bc098cc7f  .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary/🦀️.rs
e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3  🧰️framework/🔨️modules/🌱️value/💾️resident/🦀️.rs
e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175  🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs
```

The canonical module full text and ticket leaf preimage are retained in the current tool orchestration memory before modification. Root authorized ONLY the earlier exact test include and sole-allocator hook delta after this refinement. The canonical production authority must remain e23ec406; canonical tests must still equal the captured e81bcca1 bytes immediately before patch. Any mismatch aborts the mount rather than overwriting peer work.

The next report will record actual mounted module/leaf hashes, an exact inverse for the canonical test-module change, and in-memory verification that the inverse reconstructs the original full e81bcca1 preimage. It will not execute that inverse or request a compiler. No production method/field, router, Opening/CUT1, Runtime or Store edit is authorized.

