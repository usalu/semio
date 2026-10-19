# Value Assessment: Mandatory Physical Cache Images

## Recommendation

Keep cache budget policy and recoverable ownership, but **stop making mounted disk images a prerequisite for ordinary dependency setup and development**. Maintain an explicitly selected strict-capacity backend only where a hard physical quota is a real requirement. Further polishing Windows VHD recovery alone will not resolve the architectural mismatch: the mandatory mechanism adds host administration and filesystem lifecycle work to every tool route.

This recommendation is an inference from the repository architecture and failures observed in this ticket, rather than a claim that images are intrinsically slow or unsuitable everywhere.

## Why This Fits This Repository

The stated target is zero-touch Windows/macOS/Linux, devcontainers and CI. A native Windows process already cannot finish setup here because its token lacks SeManageVolumePrivilege. Earlier startup also encountered an unpublished image receipt; both problems occur below package installation or useful builds. A better error is valuable, but explicit refusal still does not satisfy zero-touch setup. Additional backend work can reduce bugs but cannot make a host privilege universally available.

The image mechanism also adds allocate/format/mount/observe/unmount/retire phases, platform-specific native APIs and interrupted-state recovery. Those phases must work across developer machines and ephemeral agents before any useful task runs. That expands maintenance and qualification considerably for an infrastructure feature whose primary purpose is limiting storage. Repeated integrity observations, native command startup, image allocation and retained failed generations are real additional work; this audit does not quantify their cost or claim that mounted filesystem I/O is always slower.

Local-first behavior and event-sourced coordination do not require images. Local directories can hold cache products, leases, journals and durable publication records. Images can still be useful to impose a strict per-workload capacity boundary, contain an uncooperative compiler/package installer, or operate a dedicated controlled build worker. A finite filesystem remains stronger than cooperative bookkeeping.

## Preserve These Distinctions

- **Budget policy:** planned/reserved bytes, ownership, cache eviction, low-disk checks, progress, cancellation and recoverable publication. Worth preserving for all platforms.
- **Hard enforcement:** a filesystem or OS quota prevents the actual workload from exceeding its assigned storage boundary. Ordinary directory size scans and journal counters do not provide this guarantee.
- **Mechanism:** fixed VHD is one Windows implementation of hard enforcement. Requiring its lifecycle for every task is a product policy, not a necessary property of cache reuse, Nx/Bazel selection or event journals.

An ordinary-directory mode must truthfully expose its cooperative bound and must not issue the current hard-enforcement proof. If the repository truly requires a strict aggregate physical limit for every build, then ordinary directories are insufficient; privileged, supported worker environments or another enforcing backend become part of the explicit system requirements. That is a valid choice, but incompatible with claiming unrestricted zero-touch native setup.

## Proposed Architectural Direction

Have normal setup use host-native directory storage with explicit cooperative accounting, global free-space reserve and cancellation/eviction policy. Keep ownership and publication correctness independent from backend selection. Require strict enforcement only for tasks that declare it necessary, and fail those tasks early when that capability is absent. Keep an image backend behind that capability if the strict use case justifies its maintenance. Do not automatically elevate users or silently describe cooperative accounting as a quota.

For the present greenfield design, changing this requirement now is preferable to further entrenching mount privilege in package provisioning, task discovery and IDE launch routes. Removing VHD files alone is not the change: existing operations intentionally refuse without an enforcing backing and require a coherent redesign of capability declarations and backend admission.

## Evidence

- Repository cache budget owner `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts:9` names rust/tools/graph/dependencies/products/diagnostics/nx/leases partitions; :23 derives their finite filesystem envelope.
- Process Native storage owner `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts:47-48` explicitly distinguishes ordinary directories from a portable aggregate physical quota and refuses ordinary Native writes; :85-88 requires enforcing backing.
- Fixed owner `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:65` validates exact fixed capacity; :226 implements Windows allocate/activate/format.
- `storage-2026-10-19.md` records actual Windows privilege refusal before effects and full mounted backing remaining unverified.
- `vhd-purpose-audit-2026-10-19.md` contains the fuller mechanism and independent journal/lease source evidence.

No code changes or tests were performed for this recommendation audit.
