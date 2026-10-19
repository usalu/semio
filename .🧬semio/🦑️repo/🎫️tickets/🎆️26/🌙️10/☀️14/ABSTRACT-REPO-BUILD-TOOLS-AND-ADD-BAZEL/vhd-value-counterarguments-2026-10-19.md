# VHD Value Counterarguments — 2026-10-19

## Judgment

The physically bounded storage capability has legitimate value. Making it mandatory for every normal local developer command is not justified merely by wanting bounded caches. For this repository, the strongest future-proof design is a storage contract with an optional hard-bounded backend for workloads that need it, while ordinary development remains usable without disk-image lifecycle dependencies. This is an architectural assessment, not a proposed implementation or a claim that a replacement already exists.

## Strongest Reasons to Keep It

- A finite mounted filesystem enforces its own capacity while a compiler or tool writes. A directory census or preflight estimate can lag concurrent writers and cannot itself stop an uncontrolled child process at the promised byte count. That distinction matters when exhausting the host filesystem would interrupt other jobs or lose unrelated work.
- An isolated build worker can fail locally with out-of-space at a known bound, rather than grow without bound. Independent domains prevent one runaway compiler/artifact workload from consuming every other domain's allocation. These are meaningful operational guarantees for unattended CI, generated fuzz inputs, native compilation, and externally supplied workloads.
- Real fixed allocation plus explicit headroom can make the host storage promise stronger than a logical file-size budget. The current implementation deliberately checks finite physical envelopes (`process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts:55–56`) and fixed VHD allocation geometry (`.../🧱️fixed/📦️vhd/🟦️.ts:13`), rather than treating a directory-size estimate as physical enforcement.
- Separate mounted storage can provide a clean unit for capacity policy and orderly retirement. The current receipts, identities, retained readers, and exclusive-writer proofs protect shared work from accidental concurrent removal (`.../🧱️fixed/🟦️.ts:148–159`). This is useful ownership machinery, though it is also substantial complexity.

A VHD is not a security sandbox. A native process that can write to arbitrary host paths can bypass a bound on its assigned volume. Untrusted execution requires separate access restrictions/process or worker isolation that actually confine writes; the storage image supplies a capacity boundary only within the enforced writable namespace. The present audit did not establish such a security boundary.

## When a Hard Ceiling Is Worth the Cost

Use mandatory hard-bounded storage when all these conditions hold: a documented workload can write unpredictably; exceeding its budget causes material harm; every relevant writable path is confined to the chosen storage; exact hard failure is acceptable; the capacity is sized and observable; and interrupted creation, mount, crash recovery, cancellation, and retirement are demonstrated across supported hosts. A worker that already has an isolated lifecycle is a particularly good fit.

Ordinary developer caches usually tolerate a soft budget with periodic pruning, concurrency-aware accounting, host free-space headroom, cancellation, and a clear failure before dangerous host exhaustion. This is less precise than a filesystem ceiling and must be described honestly. If that precision is not a requirement, mandatory images add an extra failure domain before useful development starts.

## Costs Evidenced Here

The earlier lease image reached physical allocation but not its publication receipt; normal reopen requires the missing receipt (`.../🧱️fixed/🟦️.ts:187`). That is a concrete initialization/recovery liability, not evidence that disk images can never be reliable. The lease image is also a bootstrap dependency of other partitions (`repo/.../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🟦️.ts:42–55`), so its failure blocks tools, dependencies, graphs, and products. A 64 MiB coordination image therefore has a much larger availability impact than its size suggests.

Ownership checks are necessary for shared images, but they create obligations: exact identities, metadata publication, descriptor closure, kernel mapping observations, parent/child custody, cancellation, and exclusive cleanup all have to remain correct. More guarantees do not automatically mean a better default if most commands do not need the hard-capacity property.

## Persistent State Coupling

The products partition mixes rebuildable artifacts with an explicitly non-recomputable development namespace. The code reserves the persistent charge separately (`repo/.../📦️budget/📥️opening/🏪️store/🟦️.ts:40`), and excludes its binding/intent/data paths from artifact restore targets (`process/💾️state/📋️layout/🟦️.ts:7–15`). Those protections are good and show the distinction is recognized.

The operational name and lifecycle must preserve that distinction too: cache reset, image retirement, capacity pressure, or a future backend replacement must not discard development state. Physical/lifecycle separation of durable state from disposable caches makes that rule easier to uphold and easier for developers to understand. Keeping both in one outer image makes whole-image recovery and deletion more consequential, even when logical accounting is disjoint.

## Decision Rule

Keep the finite-filesystem implementation as a deliberate capability if hard storage enforcement remains a stated requirement. Do not keep universal mandatory mounting solely because considerable code already exists. Judge the default on measured risk prevented versus startup availability, cross-platform lifecycle cost, recoverability, and preservation of durable data. On the evidence observed here, mandatory images for the lease/bootstrap layer are currently a setup liability; an isolated opt-in worker/storage profile is the stronger use case for the capability.

No implementation, cache mutation, or external security certification was performed.
