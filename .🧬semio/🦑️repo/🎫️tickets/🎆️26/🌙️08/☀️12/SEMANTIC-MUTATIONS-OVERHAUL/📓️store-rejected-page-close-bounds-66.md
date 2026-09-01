# Rejected Page Close Cleanup Bounds — Preparation Correction 66

## Result

Replaced the four fixed `0..8`/`0..32` cleanup loops in the **unmounted test fixture only**. Every replacement uses a checked sum of the actual remaining record page count and named finite ownership phases. No expected vector, schema, controller, production wrapper, include/mount, launch seed, or generated launch output changed. Strict Drop checks and discrepancy-before-teardown ordering remain.

The single requested rerun of the existing source/reference controller completed **Nx exit0,63/63** at [run-sGevqh](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️store-rejected-page-close-66/🧫️run-sGevqh/📓️receipt.md), with a complete [sibling receipt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-rejected-page-close-66-source-sGevqh.md). All five input endpoints were stable. No native compilation or execution occurred; this is still preparation, not native RED/GREEN.

## Exact Phase Derivation

Let `N` be the actual record's `tokens.pages.page_count()` at cleanup entry, or zero if the record has already been removed. It is not the schema maximum, a large global constant, or an arbitrary retry allowance.

Each named phase below is one because the actual fixture owns one optional counted token, one field shell, one lease, one returned owner, and one record shell. The real cursor retires at most one page per call. Different physical phases may complete within one outer loop iteration; adding them separately gives a conservative finite upper bound without inventing retries.

| Cleanup | Bound | Concrete phases |
| --- | --- | --- |
| Returned field owner | 2 | One counted token close, then the field shell close through the returned owner. An already-terminal field needs only the latter. |
| Unadmitted rejection/setup refusal | N+3 | N pages, counted token, field shell, record-completion shell. |
| Registered rejection | N+5 | N pages, counted token, lease return, registry detach, returned field shell close, record-completion shell. |
| Refused unstarted authority setup | N+6 | Registered phases plus the actual authority's separate final terminal-state turn after its record shell is removed. |

The registered wrapper requires its child to be terminal before returning the lease, so its registered bound counts the token once before return and the field shell once after detach. It does not count a second live token after reclamation. The general returned-owner helper allows both token and shell phases because it is also used for bounded cleanup of a returned counted owner.

The authority's extra turn is grounded in its existing `InteractiveJob::close_step`: removing its empty record returns Pending; a later turn commits its terminal state. The two rejection wrappers complete with their record-shell removal and do not receive that extra phase.

## Exact Changed Source

[Native fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🧬️rejected-page-close/🦀️.rs:202).

Added six phase constants and five test-local helpers:

```rust
const COUNTED_TOKEN_CLOSE_PHASES: usize = 1;
const FIELD_SHELL_CLOSE_PHASES: usize = 1;
const LEASE_RETURN_PHASES: usize = 1;
const REGISTRY_DETACH_PHASES: usize = 1;
const RECORD_COMPLETION_PHASES: usize = 1;
const AUTHORITY_COMPLETION_PHASES: usize = 1;

/// 🧮️ Adds actual remaining pages and explicit finite ownership phases without overflow.
fn checked_close_bound(maximum_pages: usize, phases: &[usize]) -> usize {
    phases.iter().try_fold(maximum_pages, |bound, phase| bound.checked_add(*phase)).expect("fixture close phase bound fits usize")
}

/// 📤️ A returned counted owner closes its single token, then its field shell.
fn returned_owner_close_bound() -> usize {
    checked_close_bound(0, &[COUNTED_TOKEN_CLOSE_PHASES, FIELD_SHELL_CLOSE_PHASES])
}

/// 📦️ Unadmitted rejection closes the token, field shell, pages, and final record shell.
fn unadmitted_close_bound(maximum_pages: usize) -> usize {
    checked_close_bound(maximum_pages, &[COUNTED_TOKEN_CLOSE_PHASES, FIELD_SHELL_CLOSE_PHASES, RECORD_COMPLETION_PHASES])
}

/// 🪪️ Registered rejection additionally returns and detaches its lease before the returned field shell closes.
fn registered_close_bound(maximum_pages: usize) -> usize {
    checked_close_bound(maximum_pages, &[COUNTED_TOKEN_CLOSE_PHASES, LEASE_RETURN_PHASES, REGISTRY_DETACH_PHASES, FIELD_SHELL_CLOSE_PHASES, RECORD_COMPLETION_PHASES])
}

/// 🏁️ The untransferred authority needs one terminal-state turn after its record shell closes.
fn authority_close_bound(maximum_pages: usize) -> usize {
    checked_close_bound(registered_close_bound(maximum_pages), &[AUTHORITY_COMPLETION_PHASES])
}
```

Replaced only these loop sites:

- `detach_and_close`: fixed8 → `returned_owner_close_bound()`.
- `Subject::new`, failed registry admission: fixed32 → `unadmitted_close_bound(record.tokens.pages.page_count())`, captured before moving the record into actual unadmitted rejection.
- `Subject::new`, refused public rejection: fixed32 → `authority_close_bound` using the still-owned authority's current record page count.
- `Subject::teardown`: fixed32 → registered/unadmitted bound selected from the actual subject and its current record page count.

All additions use `checked_add` through `try_fold`; overflow is an explicit fixture failure, not saturation, wrapping, a larger fallback cap, or success. No `for _ in 0..<numeric literal>` cleanup loop remains in the fixture.

A read-back comparison reconstructed the prior source by removing the new helper block and reversing exactly those four substitutions plus their local bound declarations. The rest of the code matched the prior source exactly, accounting for one final blank line (one LF) removed by apply_patch. No assertion, Drop implementation, ownership operation, vector expectation, or test name changed.

## Exact Hashes and Preserved Evidence

| Input | SHA256 |
| --- | --- |
| Prior native fixture,20374 bytes | `3183a23b62aa769835dad0d1a01da6513c0f5161c8c005b056b97c8e81eed34a` |
| Current native fixture,22610 bytes | `ce76c55dbfa74756365226a8be5bcc0c7155853c40336bdf9df4cea583f8cd4f` |
| Unchanged canonical schema | `b26e851b5cd1317b4ca799dbbfc117ed33df010ad0178bcf5a2e5db3820bb9a1` |
| Unchanged canonical vectors | `efe7c7d8de5e99f140b606c58134afab3e4d375dbb8a0489b543a92aab0524bb` |
| Unchanged source/reference controller | `5fb860042a37e7a511a127f814aa349d33dbd0d67063ea07d036b5545604306e` |
| Unchanged production Store | `7450f9d6837055d0766a55c5fc98aae22d068ac813acda09c1385a1df48d4c9c` |

Original reference57 and source/reference63 receipts at run-KHtCPn and run-1nwmS9 remain untouched. This correction adds the new run-sGevqh receipt rather than overwriting either result. It supersedes only the old fixture hash and magic-loop readiness in [preparation66](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️store-rejected-page-close-preparation-66.md), not its remaining scope/ownership limitations.

The existing source/reference command was run exactly once for this correction. Its source-marker checks do not execute the Rust bound functions or prove native compilation. The desired two-law native RED and any production fix still await the root/runtime mount and compiler authorization.
