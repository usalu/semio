# VHD Removal Audit — 2026-10-19

Read-only source assessment. No implementation, runtime verification, tests, mounting, or cache deletion performed.

The VHD belongs to repository cache storage enforcement, not to Nx or Bazel themselves. It supplies a genuinely finite filesystem on Windows: a fixed preallocated disk file, mounted as NTFS at the compact partition root. The repository opens all eight authored cache partitions through this mechanism, including the lease partition. Removing a file or bypassing the opener without redesigning its consumers breaks the current storage contract.

An ordinary-directory backend can preserve content digests, cache keys and hits, tool-neutral artifact storage, Nx result caching, Bazel directories, cooperative reservation accounting, journals, queued reader/writer leases, cancellation, progress, and atomic publication protocols. These features are software protocols; they do not intrinsically require a virtual disk. Existing implementations currently couple them to Native image types, identity checks and privately issued ownership, so retaining them requires a schema-first contract refactor, not substituting mkdir for mount.

What changes materially: an ordinary directory does not provide the OS-enforced per-partition filesystem capacity. Byte reservations and inventory become cooperative policy, not a hard limit on arbitrary subprocess writes. Free-space checks and cleanup cannot offer the same guarantee under concurrent or uncontrolled writers. The exact mounted-device identity, backing identity, mount/unmount lifecycle, nested-image funding, volume metadata flushing and image-specific publication proofs must be removed or redesigned honestly. Directory containment, identity rechecks, link rejection and kernel-backed coordination can remain, but a directory alone is not an equivalent isolation boundary or process sandbox.

The architectural replacement should be one tool-neutral cache storage contract with directory roots, ownership/lease operations, reservation/journal access, publication and retention, cancellation/progress, and explicitly described accounting guarantees. Nx, Bazel, Cargo, tools and dependency storage should depend on that contract. Remove image-only schema fields, file inventories, privilege gates, platform backends and image tests together; update schema fixtures, launch commands and cleanup protection. Do not introduce a legacy compatibility fallback or claim directory budgets provide physical enforcement.

On Windows this removes virtual-disk mounting and its volume-management host privilege requirement. Current code requires an already-assigned token privilege and explicitly does not request elevation; saying it always prompts for administrator access would be incorrect. Linux has its own fallocate/mkfs.ext4/loop mount backend; a directory implementation would remove those image-specific dependencies too.

Do not treat every cached byte as disposable. The current partition model includes lease/queue coordination and persistent state, compiler workspaces, downloaded tools/dependencies, receipts and reservation journals alongside reconstructible outputs. Any future removal must identify active owners, quiesce writers, and deliberately retain or retire each state category. Rebuilding outputs may cost time and downloads; losing active coordination or journals is a correctness concern. This explanation does not authorize removal.

## Source Evidence

All paths below are relative to C:/git/semio.

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts:9–26`: eight partitions, finite envelope, disjoint directories and aggregate budget; schema import and policy govern declarations.
- `…/📦️budget/📥️opening/🟦️.ts:24–30,43–58`: mounted namespace, lease image and all other partition image openers; unsupported enforcement is refusal. Lines 5–7 import persistent-state and artifact-cache layers; 70 onward retains operations and funded child lifecycle.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts:37–68`: image/root/capacity contract, exclusive maintenance, containment and filesystem-device checks; subsequent envelope check validates fixed allocation.
- `…/💾️storage/🧱️fixed/🟦️.ts:62–65,171,231`: fixed VHD footer admission; Windows Mount-DiskImage, NTFS formatting and partition access paths; Linux fallocate/mkfs.ext4/mount; platform selection.
- `…/💾️storage/🧱️fixed/🗂️system/🟦️.ts:9–16,57`: bounded host-token privilege inventory, requirement and enablement of already assigned privilege, no elevation request; current observer requires Windows x64 Bun.
- `…/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:28–35`: persisted compiler reservation recovery and allocation receipt, demonstrating that storage includes persistent accounting/workspace state.
- `…/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/🟦️.ts:37–43`: digest-addressed snapshot path, writer/reader leases, temporary write, sync and rename demonstrate separable directory-level protocols.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/💾️storage/🟦️.ts:12–21`: Bazel layout and tools/dependencies/products stores depend on repository storage.
- `…/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:75`: Nx graph initialization explicitly recognizes fixed backing and image receipt.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧼️workspace-cleanup/🛡️protection/🟦️.ts:190`: fixed image filenames are protected cleanup artifacts.
