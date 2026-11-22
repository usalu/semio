# Window refresh and patch backing

The November 21 v1 suite reached WindowConfig publication after correcting the canonical source quote. It then refused to issue a world window read because the issuer had no vacant slot. The debugger stopped at the original WorldWindowConfig read issuer and traced the caller to the mounted refresh after publication.

The refresh transfers the new snapshot and posts the ownership receipt on one turn. The mounted caller returns that receipt before reserving the result page. Reentry previously attempted to acquire another read even though the captured authority already held the committed generation. The refresh now recognizes a captured generation that covers the same publication receipt. The native voxel law passed with this change.

The same v1 build reached the agent fixture shutdown with a 1,572,864-byte release demand against the original 1,048,576-byte release grant. GDB captured that demand as `(copy 24, capacity 0, release 1572864, depth 3)` from `MountedTypedCommandFullOperation::worker_retirement_demands`. The inspector patch producers reserved 4,096 rows even though each can emit only one patch per decoded target and the decoded target limit is 512. Part, fastener and grip patches now quote the requested targets at construction, reserve their corresponding backing under the original scope, and refuse any append beyond it. Native agent-loop verification is in progress.

The proximity producer used the same 4,096-row reservation. Each successful emission takes at least four of the existing work units, so its original output backing now reserves at most 1,024 rows. No host limit changed.
