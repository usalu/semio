# Coordinator Independent P4d Second-remediation Final Re-audit — 2026-08-23

## Verdict

**RED — rejected for source correctness.** The retained census, terminal-intent, and close cursors are
materially better than the first remediation, but the claimed fixed collection backing contract is
not real. P4d therefore still cannot prove exact pre-admission ownership or exact terminal credit
return. No Cargo, Nx, Wasm, browser, or runtime gate was run while other Rust packets remain active.

## Audited Scope

- `✏️editor/⏳️precompute/🪣️fill/🦀️component.rs`
- `✏️editor/⏳️precompute/📐️geometry/🦀️component.rs`
- the P4d implementation report and its direct fixtures

Read-only evidence used source inspection, `rg`, numbered excerpts, and the previous independent
rejection contract. The author's scoped formatter and interactivity-verifier claims were not rerun.

## Blocking Finding

### P4d-R6 — the thirteen fixed pages do not back or bound any live standard collection

`FillBuilderCollectionBackings::new` allocates ten independent `Box<[u8; 16 KiB]>` values, while the
actual retained authorities remain ordinary `BTreeMap`, `BTreeSet`, and `HashMap` values. Likewise,
`CollisionIndexCollectionBackings::new` allocates three unrelated boxes beside the spatial index's
three ordinary tree collections. No collection stores nodes or buckets in those pages, no allocator
is connected to them, and no operation derives or enforces an allocation bound from them.

The census consequently charges each unrelated box as though it were the collection backing and
charges only semantic entry strings/values afterward. The real node, bucket, hash-table control,
alignment, and allocator-rounding storage of all thirteen standard collections remains unmeasured.
Cardinality `<= 32` does not establish a byte bound independent of standard-library implementation
layout. A collection can therefore be admitted with exact credit for the decorative page while its
actual backing allocation is outside the operation and process byte authorities.

The close path demonstrates the same mismatch. `pop_first`, `remove`, `pop`, and map/set destruction
release real standard-library nodes or buckets while the independent pages stay alive; later
`collection_backings.retire_one` releases only those unrelated boxes. The terminal witness proves
that both sets of owners are gone, but it cannot return an exact credit for the real collection
allocations because those allocations were never admitted.

The new fixture `retained_owner_census_uses_fixed_backing_pages_not_pair_size_heuristics` is
tautological: it asserts that the ten boxes themselves cost ten pages. It never proves that a
populated map/set/hash collection allocates from those pages, cannot allocate outside them, or has an
exact independent backing bound. The spatial fixture has the same gap.

## Required Repair

Replace every retained standard collection in this owner graph with an owned fixed/paged collection
whose node/control storage is the credited storage by construction, or implement a genuinely exact
allocate-inspect-admit authority that includes the actual live allocation and can return the same
credit incrementally. Merely keeping a separately allocated token page beside a standard collection
is not a backing contract and must be denied by the permanent verifier. Add a discriminating fixture
that populates each collection at max and max + 1 and proves that no allocation can occur outside the
credited pages, then proves the identical pages return one at a time during close.

P4d and Phase 4 remain open. P4e must not begin until this source defect is independently cleared.
