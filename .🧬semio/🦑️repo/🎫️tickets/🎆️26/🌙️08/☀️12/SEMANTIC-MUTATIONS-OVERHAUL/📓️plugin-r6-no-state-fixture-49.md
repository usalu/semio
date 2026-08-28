# Plugin R6 No-State Fixture Repair

## Boundary

This packet changes only the declaration shared-test trait bound and KeyedTestApp's five no-state disposer/retirement hooks. It leaves TestApp's publication presence/transient owners untouched: KeyedTestApp remains a separate genuine no-state app with `NoPresence`, `NoPresenceMutation`, `NoTransient`, and `NoTransientMutation`. No-state payloads do not make VcsArtifactApp's live presence/transient store containers absent.

## Retained Red

The R6 native inventory recorded six relevant compile errors in [member-plugin-native-inventory-r6-2026-08-27.txt](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-native-inventory-r6-2026-08-27.txt): one E0603 at declaration shared test line 82 (`crate::app::ArtifactPack` is a private import) and five E0308 errors at Plugin lines 33857–33861, where KeyedTestApp forwarded publication-typed TestApp disposers/factories despite declaring no-state associated types.

The first retained source/neutral RED is [run-U3IV57](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-U3IV57/📓️result.md): `9/17`, exposing the private import and type-mismatched forwards.

The intermediate `None` premise was rejected: its apparent source GREEN [run-HqWTw2](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-HqWTw2/📓️result.md) did not model the always-owned `presence_store` and `transient_store`; it is not runtime evidence or acceptance.

The corrected typed-owner expectation produced [run-10JSQR](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-10JSQR/📓️result.md): `11/17`, with exactly two typed disposers, three retirement factories, and the lifecycle law absent.

## Repair

- `🧪️tests/📄️declaration-channels/🧪️tests/🦀️.rs` now bounds `S` with public `store::ArtifactPack`.
- KeyedTestApp's presence hook returns `KeyedNoPresenceStoreDisposer`, which starts a real `PresenceStoreRetirement` against the store-installed bounded local/peer factories and reports terminal only after roots, peers, readers, and retirement owner are terminal-empty.
- The superseded transient-root-only design returned `KeyedNoTransientStoreDisposer`; the owned-store correction below replaces it before native review.
- The two presence factories and one transient factory use the existing `BoundedConfigRetirementFactory` for their exact no-state types.
- `keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners` is an authored native lifecycle law: zero grants make no progress; every root closes within item/byte bounds; the presence store starts retirement and empties its peer root; and typed disposers reach terminal without a missing-disposer fault. This assertion is authored but not yet executed natively.

## Source Gate

The corrected scoped Bun/Nx source gate [run-cXwxGH](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-cXwxGH/📓️result.md) passed `18/18`. It validates the schema-first typed-owner expectation with Ajv2020, retained R6 evidence, exact public import, typed disposers/factories, the no-publication-forwarding boundary, actual lifecycle test body coverage for live containers/roots/grants, and nofollow first/final input stability.

Endpoint SHA-256 values: Plugin main `75fa8da1b355e6d1985ba312cf74e68b19442c27bb89ea07185b5d329ebc0ac8`; declaration shared test `401a2ecb4cdfccfc7aa95932d35366e388b0654d326f8a9bb9eb0ded2bc0e91d`.

Native Rust compilation and execution remain pending root scheduling.

## Owned Transient Store Correction

The prior transient disposer retired `owner.current_root()`, which only owned a cloned `Arc<NoTransient>` and left the original `TransientStore` live while its `terminal_is_empty` method returned true after the clone retired. The corrected fixture-only implementation has explicit `Unstarted`, `Retiring`, and `Complete` states. An unstarted close with zero items or fewer than one decode page leaves the owner untouched. The first admissible close replaces the entire `TransientStore<NoTransient, NoTransientMutation>` with a newly constructed empty unit-store, moves the former store into `BoundedConfigRetirementFactory<TransientStore<...>>::retire_owned`, and retains a weak pointer plus generation witness for the installed replacement.

The authored native lifecycle law now proves zero and short grants do not begin retirement; the original root remains installed before the full grant; the full grant releases exactly one page and changes the installed root; dropping the test's final original-root strong reference makes its weak pointer empty; the following turn completes; and a repeated completion remains idempotent. This is deliberately a fixed-size `NoTransient` fixture contract, not a generic transient payload disposal API. Presence behavior remains unchanged. No Rust compiler or native test was run.

The retained test-first source RED is [run-bChCQe](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-bChCQe/📓️result.md), at `19/23`: it required the explicit lifecycle states, owned-store replacement/retirement, terminal witness, and native-law source assertions before the Rust edit. The retained `23/23` [run-3JKsQN](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-3JKsQN/📓️result.md) is superseded for compiler readiness: it still placed factory construction in the sibling builder test module, where app-private `BoundedConfigRetirementFactory` is inaccessible.

The corrected boundary is a `#[cfg(test)]` child at `crate::app::mutation_fixture::no_state`. It owns the private factory constructions and returns only typed existing disposer/root-factory interfaces; KeyedTestApp's actual hooks call those constructors. The native lifecycle law remains through those hooks and additionally swaps in a fresh transient store after idempotent completion, proving the weak-root/generation witness makes terminal false and the repeated close fail without mutating that fresh owner. The current retained source-only green is [run-hI5q8t](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/🧪️plugin-r6-no-state-fixture-49/🧫️run-hI5q8t/📓️result.md), `25/25`: Ajv2020 expectation validation, nofollow first/final hashes over the full child, mount and hook checks, plus a structural parse-only pass over every child constructor/helper block and its native imports. It is not Rust compilation, native execution, or readiness.
