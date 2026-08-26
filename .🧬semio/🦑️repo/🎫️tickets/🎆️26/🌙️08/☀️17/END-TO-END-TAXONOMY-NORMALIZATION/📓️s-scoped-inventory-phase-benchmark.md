# Scoped Inventory Phase Benchmark

## Scope and result

A read-only current inventory of `✏️s/🔌️plugins/✒️writer` completed with 445 entries and 33 violations in 3,469 ms. It published no artifact and did not start a full census.

| Interval | Time | Share |
| --- | ---: | ---: |
| Before first reported `inventory/directories` event | 3,003 ms | 86.6% |
| Directory canonicalization, 193 events | 81 ms | 2.3% |
| File canonicalization, 252 events | 270 ms | 7.8% |
| Reference scan, 197 events | 81 ms | 2.3% |
| Finalization | 8 ms | 0.2% |

## Finding

The dominant scoped cost is currently invisible to the progress protocol. `inventoryTaxonomy` enumerates the full tracked and untracked Git surface before applying the requested scope in memory. The existing closed phases therefore begin only after roughly 87% of this scoped runtime has elapsed.

## Required optimization order

1. Add closed progress phases for tracked Git enumeration, untracked enumeration, ignored-generator admission, and explicit-ticket admission.
2. Push an exact non-opaque scope pathspec into tracked and untracked Git commands, alongside the existing literal opaque exclusions, so a scoped inventory does not materialize unrelated index rows.
3. Re-run this identical scope and compare phase timings and source/result digests.
4. Only then use a larger representative scope to identify the dominant per-entry phase before the final full sharded census.

No performance claim is made for the full repository from this small sample. The retained 998-second monolith remains the only full-run baseline.
