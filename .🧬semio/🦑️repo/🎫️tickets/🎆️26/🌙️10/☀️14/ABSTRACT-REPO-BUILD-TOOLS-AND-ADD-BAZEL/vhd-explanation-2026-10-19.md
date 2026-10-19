# VHD Purpose, Ownership and Removal Consequences

## Purpose

The VHD implements a hard physical storage budget for framework-managed generated files on Windows. A fixed-size virtual disk is formatted and mounted as a filesystem; writes directed into that filesystem reach its finite capacity. This protects the declared managed storage budget from a compiler or other cooperating tool writing more than the application-level reservation predicted. It does not sandbox a process's access to the rest of the host.

The VHD is an existing custom framework architecture choice. Nx, Bun and Bazel do not inherently require a VHD to build or cache results. Their repository integration currently depends on the framework's physically enforced stores.

The responsible feature is **repository cache budget enforcement**, backed by **process/native-build enforcing storage**. The native storage implementation explicitly refuses ordinary directories because they do not provide a portable aggregate physical quota: `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`, lines 47–48 and 83–88. The fixed-image platform implements that contract on Windows with a fixed VHD and native mount/identity observations.

## What Uses It

The repository budget declares separate Rust, tools, graph, dependencies, products, diagnostics, Nx and lease stores in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json`.

The lease store is `.cache/b/l`; its capacity is 64 MiB. It is opened before the other stores so their concurrent work can use the managed lease directory. Consequently even dependency setup first reaches native image admission. The configured total repository budget is 64 GiB, with separate partition and control allowances; that is a configured bound, not a claim that all partitions have been allocated on this machine.

The Windows host privilege requirement originates in this mounted-storage implementation. The new preflight reports the missing privilege before allocation; it did not introduce the underlying VHD architecture.

## Removing the Backend Properly

Replacing mandatory images with an honest ordinary-directory storage implementation can retain the developer commands, dependency graph, content fingerprints, Nx/Bazel cache reuse, leases, journals, cancellation and deliberate cleanup. Those features do not intrinsically require disk images, although their current implementations share image-specific interfaces and checks.

The practical benefits would be removing VHD allocation/format/mount/detach work and its mount privilege requirement from that path. It would also reduce backing-image lifecycle and recovery complexity. It does not by itself prove faster builds or resolve every remaining provisioning issue.

The guarantee that changes is the **filesystem-enforced per-store capacity ceiling**. A reservation ledger and periodic directory-size checks can enforce budgets for writes that obey the framework, but cannot provide the same hard limit against arbitrary native writers. Another enforcing quota/storage mechanism would be needed to retain that guarantee. The current schemas must explicitly describe the weaker accounting model rather than continue claiming physical volume enforcement.

This is a storage-interface and lifecycle refactor, rather than deleting a mount call. `NativeImageStorageV1`, backing identity and capacity observations, maintenance exclusion, child storage, retirement and repository store openers currently assume real mounted filesystems. Locks and journals can survive the refactor, but the image-specific proof and cleanup contracts must be replaced with directory-specific ones.

## Deleting Existing Image Files Is Different

Deleting a VHD does not disable the feature: current openers will attempt provisioning again or reject missing/inconsistent backing. It also deletes whatever that image contains. Reconstructible dependency/build caches would need to be restored or rebuilt, and removing synchronization storage during active work is unsafe.

The products partition also reserves a separate **non-recomputable persistent development state** area. The source explicitly excludes that state and its binding/intent records from cache publication and restoration: `🧰️framework/🔨️modules/🏃️process/💾️state/📋️layout/🟦️.ts`, lines 7–15. Therefore not everything under the managed cache storage can be treated as disposable. Actual state contents were not mounted or inventoried in this investigation; no claim is made that a particular products image currently contains user data. Existing state must be preserved before any storage replacement or retirement.

The earlier retained lease generations are historical evidence described in the storage report. An empty inactive mount directory does not establish that the image file itself contains no data. No VHD, cache or implementation was removed in response to this explanatory question.

## Supporting Audits

- [Purpose and exact source references](./vhd-purpose-audit-2026-10-19.md)
- [Partition contents and data-loss distinctions](./vhd-data-audit-2026-10-19.md)
- [Removal architecture and retained guarantees](./vhd-removal-audit-2026-10-19.md)
- [Actual Windows privilege and recovery evidence](./storage-2026-10-19.md)

This follow-up was read-only source inspection apart from the requested ticket research records. No runtime implementation change or new test execution was needed.
