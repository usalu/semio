# Windows Metadata Root Review

Root reviewed the [concrete proposal](windows-state-metadata-port-proposal.md) and independently opened the primary Microsoft documentation. MSFT_Volume.Flush flushes cached filesystem data for the selected volume, takes no arguments and reports success as return0; its listed failures must remain failures. [Microsoft method contract](https://learn.microsoft.com/en-us/windows-hardware/drivers/storage/msft-volume-flush).

The image stack already depends on actual volume-management permission. AttachVirtualDisk documents refusal when the caller lacks SE_MANAGE_VOLUME_PRIVILEGE. This does not prove permission for the separate CIM flush operation or zero-touch support for every Windows account/edition. [Microsoft attachment contract](https://learn.microsoft.com/en-us/windows/win32/api/virtdisk/nf-virtdisk-attachvirtualdisk).

At11:42UTC Root authorized contained schema-first Source implementation and focused laws only. The actual privately issued Native-image binding, exact local NTFS volume mapping, same issued phase, bounded owned system child and full drain are mandatory. State callers and tests must migrate together to the required first-party metadata capability. Ordinary temporary directories cannot authorize a host-volume flush. Parent/child image lifetime must close metadata scopes before image.close to avoid a self-drain cycle. Unsupported or failed actual system operations remain refusals.

No Native Windows runtime, privilege grant, UAC/service installation or cross-platform physical qualification is authorized or claimed by this review. The existing Hyper-V and hardcoded Windows-root concerns remain explicit follow-up work. No directory FileHandle.sync assumption or allowlist-only change is approved.

## Draft Source Findings

At11:54UTC Root read the in-progress metadata issuer and sent two concrete findings. The rejection handler overwrote scope.failure after every failed barrier while the authentication check still allowed births on a failed scope. It must retain the first failure, including undefined/frozen values, refuse further births after failure and preserve body-error precedence. The one-pending implementation refuses overlap before admission; its docs and laws must not describe that behavior as queued serialization.

The issuer also supplied dirname(image.root) as the policy lease directory to port.observe. That value is not the actual factory lease directory for repository images. Even if a current observer ignores the field, the policy must come from the authenticated factory binding rather than an inferred path. The execution owner is migrating this to actual retained facts and adding focused laws. These draft findings do not claim the completed Source has either defect or that a physical test has run.

## Provisioning Plan Review — 2026-10-16 12:24 UTC

Root read windows-native-provisioning-plan.md in full. The concrete fresh-image receipt publisher still performs a POSIX directory fsync before a fixed image can become ready; the ready-image metadata capability cannot authenticate its own prebirth publication. A preparation-owned containing-volume barrier is the appropriate separate owner, with exact initial-host versus retained-parent identity and existing accepted cleanup. Permanent implementation is held while public SDK86883 closes and its current metadata is observed. No Windows/native physical operation is released by this review.

Root independently opened the official [GetSystemDirectoryW contract](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemdirectoryw) and [GetFinalPathNameByHandleW contract](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfinalpathnamebyhandlew). These support bounded OS system-path resolution and an actual retained directory handle's GUID-volume identity; they do not prove pinned Bun FFI ABI or Windows filesystem behavior. The plan's fixed-VHD encoder can remove the optional Hyper-V creation module, while attachment privileges and storage provider availability remain actual OS requirements.

The current Cargo target matrix admits Windows x64. The Bun bootstrap distribution table separately includes ARM64 hosts, where the pinned FFI implementation is disabled; this is an existing host bootstrap inconsistency, not authorization for a runtime upgrade, new target, artifact download or platform spoof. Before Source release, the agent will prepare exact schema/corpus/oracle inputs and retain this distinction. No all-platform completion follows from modeled Source checks.
