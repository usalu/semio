# W0 Contract Parity Audit

## Findings (deviations and missing implementations only)

| Point | Verdict | Evidence | Severity |
|-------|---------|----------|----------|
| **1. Frozen codes:** Only 4 of 7 implemented | PARTIAL | Grep over spr module finds only `mutation.target-missing` (1), `mutation.clamped` (1), `mutation.invariant` (2), `mutation.cascade` (2). Missing: `mutation.no-op`, `mutation.partial`, `mutation.duplicate-id`. 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/** ; used in command/component.rs:190–196, wire/component.rs comment, conflict/component.rs tests. | Medium — frozen set incomplete; **per contract "no code outside the frozen 7"** this is not a breach yet (no OTHER codes introduced), but implementation is not finished. |
| **3. Severity order** | CONFORMS | `🗣️dsl/⚠️diagnostic/🦀️component.rs:70–74` declares `Info, Warning, Error, Fatal` in order with `#[derive(PartialOrd, Ord)]`. `as_u8/from_u8` 0..3 at lines 79–97. No `Hint` variant. | — |
| **2. Policy matrix** | CONFORMS | `📡️spr/🧾️wire/🦀️component.rs:167–206` declares `MergePolicy { LaissezFaire, #[default] Normal, Vigilant }`. `rejects` impl at lines 179–186: LaissezFaire rejects `Fatal` only (line 181), Normal rejects `>= Error` (line 182), Vigilant rejects `>= Warning` (line 183). `as_u8/from_u8` 0..2 at lines 189–205. | — |
| **4. Law 1 enforcement** | NOT-ENFORCED | `MutationOutcome::absorb_messages` (command/component.rs:233–235) accepts arbitrary messages including Fatal without validating that diff is `Default`. A caller can do `MutationOutcome::new(non_empty_diff).absorb_messages(vec![MutationMessage::fatal(..)])`, violating Law 1. Docstring (line 159–163) **explicitly acknowledges** "Laws 1/2 are upheld by every `diff` leaf's own construction ... not enforced by this type itself", but **nothing in the type system prevents violation**. | High — Real violation surface exists. |
| **4. Law 2 enforcement** | CONFORMS | Error outcomes are constructed via `MutationOutcome::error` (line 195) which forces `diff = D::default()`, ensuring no change. No public builder attaches Error to non-default diff. | — |
| **4. Law 3 enforcement** | CONFORMS | `diff` returns `MutationOutcome<Self::Diff>` deterministically from same input; messages list is deterministic for same (op, base). No stochastic source in implementation. | — |
| **4. Fatal forces default** | CONFORMS | `MutationOutcome::fatal` (line 190) forces `diff: D::default()` via struct construction. | — |
| **5. C10 deletions:** `reconcile_with_last` | STUBBED-NOT-DELETED | `🏪️store/🦀️component.rs:2068–2070` is a no-op stub `fn reconcile_with_last(..) -> (P, Vec::new())`. Called at lines 2091, 2677, 2681, 2856. Contract says "delete"; 0-A admitted stubbing it. **Per-contract this should be deleted, not stubbed.** | Medium |
| **5. C10 deletions:** `SpaceConflict` | STUBBED-NOT-DELETED | `🏪️store/🦀️component.rs:4880` struct definition still present. Used by `reconcile_with_last` return type and throughout materialize functions. **Contract says "delete `SpaceConflict` & friends"; still alive.** | Medium |
| **5. C10 deletions:** `MergeStrategyKind` | DEVIATES | `🎠️kernel/🦀️component.rs:668–674` declares `pub enum MergeStrategyKind { LwwRegister, OrderedSequence, ... }`. Re-exported from `🧰️framework/📦️packages/🦀️rust/📦️glue.rs:103` as part of `manifest::kernel` public surface. **Contract C10 says "MergeStrategyKind ... deleted"; still exported.** | High — Violates explicit deletion directive. |
| **5. C10 deletions:** Stale docstring | DEVIATES | `🛢️db/⚔️conflict/🦀️component.rs:13` (2-E's lease, outside W0) still has `protocol_crdt::merge_concurrent_diffs` reference. 0-A found 6 of 7 stale refs in-lease (flagged as pending 2-E fix in their report line 119). | Low — Outside W0-A/W0-B scope, acknowledged. |
| **6. Derive parity** | CONFORMS | `diff <derive/🦀️component.rs> <derive/📦️packages/🦀️rust/📦️glue.rs>` (no arguments) returns exit 0, empty output. Files are byte-identical. Both strips `fn validate`, adds `MutationOutcome<..>` return types, removes `MutationDescriptor` 4th arg. | — |
| **Frozen codes NOT found** | NON-FROZEN-CODES | Repo-wide grep (excluding .🦑️repo/.🦩️repo/target/node_modules) finds NO uses of `mutation.rejected` (correctly unused — it's a separate fault code for channel, per contract C8/C9). All other non-frozen codes found (`mutation.invalid-touched-paths`, `mutation.index-out-of-range`, etc.) are from plugin glTF/flow/other subsystems not in W0 scope and pre-dated this contract. | — |

## Summary

- **CONFORMS:** Severity order (C1), MergePolicy matrix (C3), Laws 2–3 (C2), derive parity (C6)
- **NOT-ENFORCED:** Law 1 via `absorb_messages` — type system does not prevent attaching Fatal to non-empty diff
- **STUBBED (should be DELETED):** `reconcile_with_last`, `SpaceConflict` — 0-A admits both (report line 121–122)
- **DEVIATES:** `MergeStrategyKind` still exported from kernel public API (should be deleted per C10)
- **INCOMPLETE:** Frozen 7 codes partially implemented (4 of 7 used so far; no unauthorized codes introduced)

---

**Audit completed:** `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`  
**Read-only auditor for lanes 0-A (W0-A) and 0-B (W0-B)**
