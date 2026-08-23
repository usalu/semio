# Coordinator Independent Draw Fixed-Owner Overlay Audit — 2026-08-23

## Verdict

**REJECT — source cohort only.** The latest Draw repair closes the previously reported BTreeMap rebuild, ambiguous duplicate framing, dual history-ID clone, and synthetic-boundary seams. It still allocates retained mutation arenas before aggregate admission and still performs a live post-admission `String::try_reserve_exact` during duplicate rewriting. The independently accepted structural placeholder count therefore remains **14**; Draw's proposed decrement to 13 is not accepted.

Terra audit admission was unavailable because the collaboration scheduler retained the completed root child limit. This is a coordinator Sol Extra High independent audit, not a Terra verdict.

## Blocking findings

### 1. Retained arenas are allocated before aggregate admission

`DrawMutationCandidateAuthority::try_new` allocates both 64-slot container vectors, the overlay-page vector, sixteen 4,096-byte String pages, and the duplicate-ID String owner before constructing the candidate. The actual aggregate admission is performed later in `PreflightMutation` through `DrawMutationAggregateReservation::admit`.

This ordering measures allocator-returned capacities only after the memory already exists. A request rejected by the 4,096-item / 262,144-byte aggregate gate has already allocated the exact owners that the gate is supposed to admit. The fixed arenas are per-candidate allocations, not a process authority pool retained before operation admission. This does not satisfy admission-before-allocation or exact process aggregate control.

Evidence:

- Draw owned component lines 3193–3206: `try_new` allocates/reserves all candidate arenas.
- Draw owned component line 3372: aggregate admission occurs later after source/mutation preflight.

Required repair: allocate these arenas in a retained process/app authority pool before operation admission and transfer exact slots into the candidate only after a successful ledger claim, or use inline fixed arrays whose full retained bytes are claimed by the outer operation admission before `try_new` can construct the candidate. Rejection must return the exact pool slots without per-request allocation.

### 2. Duplicate name rewriting still grows a live String after admission

`DrawDuplicateRewriteAuthority::step` phase 10 calls `base.name.try_reserve_exact(suffix.len())`, then phase 11 appends the suffix. The phase checks only the logical maximum length, not that existing capacity was pre-admitted. This is a post-admission allocator call on the live retained mutation route.

Evidence:

- Draw owned component lines 3047–3056.

Required repair: move the name into an already admitted fixed String/page owner, build the suffixed name through retained page steps, atomically replace it, and cursor-retire the displaced owner. No `reserve`/`try_reserve` may occur after candidate admission.

## Verified positives

- Duplicate identity hashing is explicitly framed by domain, ID length/bytes, and name length/bytes.
- Initial Draw snapshot ownership is moved into the unpublished initialization runtime rather than cloned.
- The source uses a fixed asset range cursor; the prior `iter().nth`, synthetic BTree node constant, `structural_copies`, and `exact_for_test` seams are absent.
- Shared history revision records retain fixed digests and move the prepared history String once.
- Draw has zero live `reject_whole_buffer_artifact_envelope_ingress` occurrence.
- Repository census is 14 Rust occurrences: one shared fail-closed definition plus 13 other live callers.

## Independently rerun gates

| Gate | Result |
| --- | --- |
| edition-2021 scoped rustfmt check on Draw owned/editor and shared store | PASS |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test --format json` | PASS: 258 clean |
| `bun ./📜️script.ts verify interactivity` | PASS: DENY clean, one recorded test-only bridge |
| fixed-overlay ledgers | PASS: byte-identical SHA-256 `b24a25c754648a9792fe9348ee141ddf48c1b9d0f5af807d88c4cab44fe481b8` |
| working/staged/HEAD diff checks | PASS |
| Cargo/Nx/native/Wasm/browser/runtime | Not run; RED/unverified |

## Residual gate state

Phase 8 remains RED at 0/884 with 18 global failure classes. Draw remains source-rejected; runtime/timing and the 13 other live placeholder callers remain open.
