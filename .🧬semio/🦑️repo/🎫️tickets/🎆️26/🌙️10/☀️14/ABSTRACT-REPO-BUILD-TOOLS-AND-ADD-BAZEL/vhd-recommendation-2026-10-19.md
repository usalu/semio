# Recommendation on Mandatory VHD Storage

## Decision

**Replace mandatory mounted disk images with ordinary-directory storage for this repository's developer toolchain.** Retain explicit storage budgets, ownership, concurrency control and recovery. Do not maintain a second VHD implementation solely for a hypothetical future requirement.

This is an architectural recommendation, not a claim that ordinary directories enforce physical quotas or that the refactor has been implemented. The previous repairs made existing failure handling safer under the current contract. They do not establish that mandatory images are the best contract for the project.

## Why This Fits the Requirements

The user's requirements prioritize zero-touch native Windows, macOS, Linux and devcontainers, efficient incremental builds, concurrency, local-first operation and maximum control. The present design requires privileged native storage facilities before even lease bookkeeping and dependency setup. The current Windows mount failure is direct evidence that routine tool availability depends on a host facility beyond the task's actual build requirements. Additional recovery code cannot remove that privilege dependency.

Each platform image backend adds allocation, formatting, mounting, mapping identity, capacity observation, detach, interrupted-birth recovery and retirement states. Making all ordinary commands depend on those states expands the environment and failure matrix. My assessment is that this is a poor default for a portable developer toolchain unless hard per-store filesystem capacity is an explicit, indispensable product requirement. No such requirement has been established in the current user request.

Storage budgets remain valuable. Cache accounting, age/size-based eviction, free-space headroom, bounded logs, limiting in-flight work and cancellation directly address routine development storage pressure. Their guarantees should be named honestly: cooperative accounting and cleanup cannot replace a filesystem-enforced capacity ceiling against arbitrary native writers.

## What the Build Tools Actually Need

Bazel documents ordinary output roots, per-user/per-workspace separation and configurable output directories as its standard build-state layout. Mounted disk images are an additional repository policy. [Bazel output-directory documentation](https://bazel.build/remote/output-directories).

Nx describes task hashes, local/remote result lookup and restoration of declared output files. Correct task inputs and outputs are central to reuse and selective execution; a VHD is not part of that caching mechanism. [Nx caching documentation](https://nx.dev/docs/concepts/how-caching-works).

Container storage already has a separate owner and lifecycle. Docker documents managed volumes and persistence independently of container lifetime. Requiring another image lifecycle inside every container adds an integration obligation; this is an architectural inference, not a claim that Docker volumes automatically provide hard quotas. [Docker volume documentation](https://docs.docker.com/engine/storage/volumes/).

## Strongest Reason to Keep Hard Enforcement

A finite filesystem is valuable when workload output is unpredictable, writes are actually confined to it, and exceeding a budget must be prevented by the filesystem rather than detected later. An unattended worker handling expensive native jobs can justify the provisioning and lifecycle requirements. A VHD alone does not confine a process's other host writes or provide a security sandbox.

If that requirement becomes concrete, place it at an explicitly provisioned execution/worker boundary with measured capacity semantics. Do not require ordinary repository discovery, help, synchronization or cache reuse to acquire it. A specific future requirement can justify a separately designed enforcing execution environment; keeping today's entire alternate image implementation speculatively would retain most of its maintenance burden now.

## Clean Replacement Direction

1. Keep a first-party storage ownership interface backed by ordinary directories, with short, explicit roots and validated identities.
2. Separate persistent development data from reconstructible caches. Keep in-flight and pinned artifacts out of eviction candidates.
3. Retain leases, publication journals, recovery, hash verification, bounded concurrency, progress and cancellation with directory-specific contracts.
4. Provide explicit size targets and free-space thresholds; prune only eligible recomputable entries and refuse new work when required headroom is unavailable.
5. Remove image-specific APIs, schema claims and lifecycle paths from the ordinary toolchain, rather than retain a compatibility fallback that silently changes guarantees.
6. Preserve any existing persistent state before retiring old storage. Do not blanket-delete files described as cache storage.

This recommendation favors an explicit, narrower guarantee that works in ordinary development environments. It accepts the loss of a hard per-store filesystem cap; it does not hide that tradeoff or imply that accounting offers equivalent enforcement.

## Evidence and Scope

- [Existing purpose and ownership trace](./vhd-explanation-2026-10-19.md)
- [Value and portability audit](./vhd-value-audit-2026-10-19.md)
- [Strongest counterarguments](./vhd-value-counterarguments-2026-10-19.md)
- [Future storage architecture audit](./vhd-future-architecture-2026-10-19.md)

Three read-only exploration agents and the coordinator assessed this question. No implementation, image deletion or new runtime tests were performed. The existing ticket remains open for the original unfinished toolchain recovery.
