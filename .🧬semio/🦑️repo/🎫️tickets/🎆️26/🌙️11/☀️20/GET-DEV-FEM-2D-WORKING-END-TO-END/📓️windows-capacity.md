# Windows host capacity during FEM 2D launch

The repository build and status commands encountered a Windows sharing violation when the capacity gate attempted to unlink a stale grant. A gate test reproduced the failure with a separate .NET process holding the record open without delete sharing. A second test showed that a status-only request deleted the stale grant on an unlocked read.

Status now excludes stale grants and waiters without reclaiming their files. Admission compares the observed modification time before reclamation and leaves busy Windows files for a later admission pass. Stale grants and waiters stay outside the capacity calculation while their files are held open. When the queue directory reaches its record limit, admission counts live waiters before refusing a new request.

The three regressions failed before the change and passed after it. The focused suite passed nine tests, including dead-process, PID-reuse, and pre-boot reclamation. The Windows test independently exercises real file-sharing semantics through .NET.
