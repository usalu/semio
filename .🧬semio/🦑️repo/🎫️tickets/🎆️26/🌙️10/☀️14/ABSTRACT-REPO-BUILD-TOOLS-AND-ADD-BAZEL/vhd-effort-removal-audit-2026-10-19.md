# Image-Free Removal Effort Audit — 2026-10-19

Planning estimate: **15–30 senior engineer person-days**, approximately **3–6 focused working weeks for one engineer**, for a complete image-free replacement with cross-platform validation. A practical planning allowance is **about 22 person-days**. This is a source-based estimate, not measured implementation time or a commitment. Concurrent changes in these files and unavailable host validation can extend calendar time.

A quick bypass of the mount/privilege check might take hours to one or two days, but is not the requested clean removal: downstream code requires actual image ownership, physical capacity proofs, mounted identities and receipts. Silencing those checks gives false guarantees and can corrupt coordination/accounting. It is not an acceptable estimate for finishing the feature.

## Work Breakdown

| Work | Senior person-days | Completion criterion |
|---|---:|---|
| Audit ownership and classify persistent state, reconstructible cache and active workspaces; author schema/guarantees | 2–3 | Directory ownership and honest cooperative budget semantics; retention/recovery rules agreed in code fixtures |
| Implement directory owner and lifetime/identity operations; adapt leases, reservations and journal access | 3–5 | Cancellable accepted operations, exclusive maintenance and recovery remain coherent without image authority |
| Rework staging/publication/pins/eviction and persistent state placement | 2–4 | Durable coordination outside eviction roots, atomic verified publication, no eviction of active or uncertain ownership |
| Update repository partition/store consumers, Cargo/tools/dependencies, Nx, Bazel and Bun integration | 3–6 | All supported command routes use directory contract; no required mount or volume privilege |
| Delete image backends, image-only interfaces/schemas/fixtures/proofs and cleanup exemptions; update script routes and launch configurations | 1–3 | No alternate image backend, compatibility layer or stale image guarantee |
| Language-agnostic cases and independent oracles; crash/concurrency/cancellation integration; Windows/macOS/Linux/devcontainer runtime validation | 4–9 | Evidence for actual supported behavior and limitations across hosts |
| **Total** | **15–30** | Complete design replacement and validation |

The phases overlap conceptually but the total assumes one engineer and does not count overlapping days twice. Validation includes meaningful new feature coverage required by repository instructions, not merely removing image tests until existing commands pass.

## Critical Path and Uncertainty

The critical path is **schema and state classification → directory owner → journal/publication integration → caller conversion → multi-host runtime validation**. Mechanical deletion can happen only after remaining consumers no longer require image authority. Parallelizing caller conversions helps calendar time only after the core contract stabilizes; it does not eliminate design and platform verification effort.

Lower end assumes existing lease/journal mechanics are reusable with clean root ownership, executable routes are already understood, healthy test hosts exist, and current tasks need no hard physical quota. Upper end allows substantial coupling in funded-child storage, publication/recovery and tool bootstrap, plus concurrency failures and cross-platform fsync/rename/locking differences. If preserving hard OS-enforced capacity is required, this estimate does not apply: ordinary directories do not satisfy that requirement and a separate enforced execution/storage design is needed.

Assumptions: one experienced developer familiar with the repository, no legacy compatibility, no migration scripts, no need to retain image support, no external runtime dependency, and permission to replace authored fixtures/configurations coherently. Existing active disk images are not blindly deleted; identifying and safely retiring or retaining live state is part of ownership handling. Unrelated feature work is excluded. No promise is made that all current cache payloads can be discarded.

## Scope Evidence

On this checkout, `rg --files` counts **141 files** under `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage`. This includes generic contracts and tests as well as image mechanics; it is not a count of files to delete. A targeted `rg -l` for `NativeImageStorageV1|openNativePlatformStorageV1|withRepositoryCacheStoreV1|withRepositoryCachePartition` finds **41 non-test TypeScript consumer files** in the repository library subtree. This is a lower-bound coupling signal, not the complete change set: indirect consumers, schemas, scripts, fixtures and other framework modules are excluded.

Exact architectural evidence is in `vhd-removal-audit-2026-10-19.md`; the proposed single-directory implementation and state separation are in `vhd-future-architecture-2026-10-19.md`.

Historical effort already invested cannot be derived reliably from current file volume or code complexity. This audit measures present scope and estimates future engineering effort only. Commit timestamps also would not prove hours worked. No source edits, tests, runtime checks, mounts or cache deletion were performed for this estimate.
