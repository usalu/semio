# Phase 3d UI I/O Isolation

## Scope

This packet removed UI-reachable synchronous clipboard, filesystem, process, and identity work from the UI WGPU host, native UI host, TUI, and Shell-owned paths. OS renderer glue, ProgramBridge, stdio transport, and plugin-host findings remained owned by the renderer/stdio packet.

## Implementation

- ClipboardIoJob and NativeClipboardJob are bounded InteractiveJob implementations. arboard is created and used only from the job step; UI callbacks submit/poll mailboxes without waiting.
- TUI clipboard reads and writes use the shared WorkerPool I/O lane, retain FIFO receivers, and expose results only through non-blocking try_recv.
- Shell file open/save/folder/export/media actions use a bounded 64-entry I/O mailbox and drain at most eight completions per frame. Native preference reads and coalesced writes also run on the I/O lane.
- Shell session and OS host document identities use semio_framework_os_kernel::os_identity::time_ordered_id; direct Uuid::now_v7 dependencies were removed from the OS host.
- The renderer owner received and accepted the frozen ClipboardIoJob::{read, write, read_candidate} API.

## Verification

- bun nx run workspace:verify-interactivity: the owned Shell/UI-host/UI-WGPU/TUI paths report zero findings. The audit still exits non-zero with 25 findings: 13 blocking-bridge, 11 synchronous-filesystem, and one synchronous-process finding, all in renderer glue, ProgramBridge, or stdio paths assigned to the renderer/stdio packet.
- UI/TUI Nx test gate: 142/142 passed, including the stalled-clipboard non-blocking callback p99 assertion below 2 ms.
- Renderer quick compilation reached unrelated concurrent stdio decorative-async errors before Shell tests could link. The Shell mailbox p99 test is present but is not recorded as passed here.
- Native OS host verification was queued behind the shared Cargo owner; no unobserved check is claimed as passing.

## Result

The packet-owned UI callback paths are enqueue/poll-only and the interactivity audit contains no owned direct clipboard/filesystem/process calls. Completion of the repository-wide Phase 3 gate depends only on the separately owned renderer/stdio findings.
