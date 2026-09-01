# Runtime Opening Parent — Staged Test Hardening

## Corrected Boundaries

Only the ticket's staged Rust, fixture/schema and design report changed. All seven ownership goals remain. No native/API implementation, canonical include, dependency, Plugin/Store/UI/WGPU source or resident17 input changed.

1. The receiver law now names the actual proposed private `RuntimeInstanceAllocation<OpeningApp>` handle. Rust's native `Layout::new::<Option<OpeningHandle>>()` independently measures its slot size/alignment. Actual typed source/destination and remaining metadata write-region Layouts are compared with that measurement; their checked sum must match `required_move_bytes` and be positive and<=4096. The short grant is measured-minus-one, not API-number-minus-one. The actual completed write trace must match the typed field plan. Missing, oversized or mismatched layouts are captured with measured/declared/slot diagnostics and go through cleanup without attempting an enlarged grant. These missing native observation methods must inspect the actual production fields; they cannot repeat the API's constants or mint permission.

2. Live-owner observations now use Result/Option, not `input.as_ref().unwrap()` or assertions on observed grant sizes. Forward failures return to an outer scope retaining every original runtime/root/input. All original runtime slots are closed, including unexpected input transfers and partial initialization. Remaining input objects are retired through real close steps and are taken/dropped only after their typed terminal witness. Cleanup records the first fault or actual overspend and continues within its captured work allowance. Intended assertions occur after cleanup; nonterminal cleanup is reported first, never hidden by resetting or dropping a live owner. Callback rendezvous is resumed/joined on observation errors, and injected clock mode is restored before cleanup. Final release counts use a separate test-only observer rather than a freed parent key.

3. Fixture capacity is explicitly isolated64MiB total/8MiB control. It neither changes nor borrows the native UI32MiB envelope, and it proves no live composition fit. Positive tests fail after cleanup if the eventual actual layout cannot fit; they must not increase capacity. The100000 loop was removed. Each preparation/close driver uses checked `sum(ceil(actualRegionLayout.size/4096)) + originalLogicalSlots + declaredTypedOwnershipTransitions + 3 terminal checks`. Inventories come from actual registered Layout descriptors and the declared per-field phase plan, never capacity totals or observed success. Exhaustion means incomplete retained ownership. This bound is not a liveness, throughput or strict timing proof.

The work-inventory and write-observer APIs remain proposed cfg-only read/measurement seams, not implemented success helpers. Original first-panic/fault handoff, root lifetime, private destination registration, scheduler-tail receipt and separately retained Free→Refund remain required production work after review.

## Exact Current Ticket Sources

Paths are under `🧪️runtime-opening-parent`:

| File | SHA256 |
| --- | --- |
| 🦀️.rs | 01d75c62a738771d492b9619f8d02e87057958975a88ca1d62c415aa2d9e27e1 |
| 🔣️.json | 18a7d4d13790f59897fae10816672f543e971d229ee0a85e356188b8f7ebe729 |
| 🧬️schema/🔣️.json | 089088860a36e347466d2be8269be9e340dbeeeecc86d0a8cb0a4afa1992111f |
| 📜️script.ts (unchanged) | 5b8b17927bbd4fc3c551a206d87477b200379aa6e695ca151fe6c55b349a3499 |

`📓️runtime-opening-parent-red-packet-2026-08-28.md` now describes these exact hardened boundaries. No tests were added or removed from the seven-law roster.

## Execution History Preserved

No Nx retry, native command or new source oracle was executed during this hardening, as requested. `📓️runtime-opening-parent-reference-r3-2026-08-28.md` remains unchanged and contains the actual prior scoped Nx exit0: seven cases,39 Immer reference transitions,two hostile checks and seven native names **for its captured prior Rust/fixture/schema hashes**. Its R1 relative-path failure and R2 accidental all-project repeated reference invocation/interruption remain preserved. The hardened current fixture/schema/Rust have not been executed. An unchanged controller is not a new passing-source claim.

Resident17 is immutable and first in the sole executor queue: source508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f; testsebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e. Plugin main remains2ad816977def25ded3175c87c0f7d03344f1bae57549689b17388adf871736ca. This lane has no running process and has made no native/source mount.
