# VHD Investment and Clean Removal Estimate

## Answer

The existing feature represents substantial implementation, schema, test and debugging work, but **actual historical engineer-hours or agent compute cost cannot be recovered reliably from the inspected records**. A complete image-free replacement is a **15–30 senior engineer person-day planning estimate**, approximately **3–6 focused working weeks for one engineer**. It is an estimate of work, not a promise about elapsed chat time.

## Measured Existing Investment

- The image-focused lifecycle scope contains **93 files and 9,411 nonblank lines** across the fixed backend, platform dispatch, platform image entries, birth, cleanup and mapping.
- Within that subtotal, **26 implementation files contain 1,411 nonblank lines**; **18 schema/data files contain 3,134 lines**; **49 test-path files contain 4,866 lines**. The scope is cross-platform image infrastructure, not only Windows VHD code.
- The entire native storage subtree contains **141 files and 26,090 nonblank lines**. This broader total includes reusable journal, accounting, publication, ownership and metadata concepts; it is not a VHD deletion count.
- A targeted symbol inventory found at least **41 non-test TypeScript consumer files** in the repository library. Indirect consumers and other framework modules are outside that count.
- The current fixed-backend path appears in **eight committed snapshots between October 14 and October 19, 2026**. Those dates cover roughly four days and seven hours of elapsed authorship time across six calendar dates, not active labor. Earlier implementations may have existed at other paths.

The records demonstrate repeated design and implementation iterations in provisioning, identity checks, allocation, writer/reader ownership, metadata durability, publication, cleanup, crash recovery and platform qualification. There is no reliable time ledger. Converting these counts or timestamps to hours would invent precision. Some source is densely formatted onto long lines, so line counts are especially poor productivity or effort measures.

## Scope of the Removal Estimate

The estimate follows the prior recommendation: replace ordinary toolchain storage with one honest directory implementation and remove mounted-image backends, without a compatibility fallback or speculative second backend. Preserve reusable concurrency, ownership, recovery and cache behavior. Separate non-recomputable development state from cache eviction roots.

| Workstream | Senior engineer person-days |
| --- | ---: |
| Classify durable state, caches and active work; author directory ownership and budget schemas | 2–3 |
| Implement directory ownership/lifetimes; adapt leases, reservations and journal access | 3–5 |
| Adapt publication, pins, recovery, eviction and persistent-state placement | 2–4 |
| Convert dependency/tool/Cargo/Nx/Bazel/Bun consumers | 3–6 |
| Remove image-specific APIs, implementations, schemas and stale executable/configuration paths | 1–3 |
| Replace relevant contracts and verify crash/concurrency/cancellation plus actual Windows/macOS/Linux/devcontainer behavior | 4–9 |
| **Total** | **15–30** |

The work is not dominated by deleting mount calls. The existing types and checks require real image identities, finite backing observations, mounted ownership and image-specific cleanup. Those contracts must become coherent directory contracts before the dependent commands can work reliably.

## Assumptions and Uncertainty

- An experienced engineer is familiar with the repository; no backward compatibility, retained image implementation or migration scripts are required.
- The target guarantee is explicitly cooperative accounting and cleanup. Hard per-store filesystem capacity is deliberately relinquished; implementing a different hard-quota system is outside this estimate.
- Existing reusable lease, journal, filesystem descriptor and cache primitives reduce replacement work. Much of the earlier effort remains useful.
- Existing persistent data is identified and preserved before any old image retirement. This investigation did not establish that every managed store is disposable.
- Appropriate test hosts and required access are available. Without them, cross-platform validation has an external calendar dependency.
- Unrelated repository failures and concurrent logical feature work are excluded. The upper end allows significant coupling and newly exposed filesystem lifecycle bugs.

Confidence is moderate to low until the directory contract is authored and a complete consumer inventory is verified. The critical path is ownership/schema design, then directory lifecycle and publication, then consumer conversion, then host qualification. Agents can parallelize independent consumer and test work after the core contract stabilizes; four agents do not imply one quarter of the elapsed time.

The repository size audit also discusses a narrower strategy retaining an explicit strict backend. That is a different scope; the estimate here consistently uses the previously recommended complete image-free ordinary toolchain replacement.

## Supporting Evidence

- [Measured size and reproducible scope](./vhd-effort-size-audit-2026-10-19.md)
- [Bounded history and iteration audit](./vhd-effort-history-audit-2026-10-19.md)
- [Independent removal work breakdown](./vhd-effort-removal-audit-2026-10-19.md)
- [Prior architectural recommendation](./vhd-recommendation-2026-10-19.md)

This was a read-only investigation apart from ticket Markdown records. No storage implementation, image or cache was changed, and no runtime test was needed for the estimate.
