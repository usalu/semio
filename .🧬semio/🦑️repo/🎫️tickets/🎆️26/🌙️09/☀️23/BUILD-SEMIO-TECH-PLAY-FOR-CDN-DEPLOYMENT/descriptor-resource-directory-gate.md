# Descriptor Resource Directory Gate

After the Actor Job wire compiler errors, native descriptor PID 68664 continued under Play driver 57497 in generation `wuONgiG`. Its Cargo child had exited. The descriptor log reported `resource-directory-admission:v1` in shared mode for more than 370 seconds. Its process sample showed a Bun event loop with occasional SQLite work. The default lease listing reported zero waiters and cycles.

A read-only Darwin `F_GETLK` query identified PID 59506 as the owner of an exclusive 512-byte lock covering pending, reserved and shared SQLite lock bytes starting at offset 1,073,741,824. The file name matches the repository's SHA-256 resource key for `resource-directory-admission:v1`. The descriptor and other waiting Bun processes have the same SQLite file open. PID 59506 is in the default graph session, with the graph image and selected shard 56 leases. Process snapshots observed it in uninterruptible sleep and showed a defunct child. This establishes the blocking gate owner behind the descriptor heartbeat.

The source interface already exposes the wire constants named in the compiler error. Post-Cargo handling waits on the resource gate before it can finish its currentness check. The supervisor and its driver remain live.

A later query found the same exclusive lock owner after 468 seconds of descriptor wait. The graph worker was runnable and accumulated CPU time. Its sample contains filesystem metadata calls. The underlying path and JavaScript operation remain unverified. [Graph worker evidence](graph-admission-gate-observation.md)

## Gate Release and Source Plan

The last shared heartbeat before admission was at 472 seconds. The descriptor resumed, read 453 target recipe inputs and admitted all 502 workspace source units in its post-Cargo check. The exact plan diagnostic identified the umbrella framework source `🧰️framework/📦️packages/🦀️rust/🦀️.rs`. It changed from SHA-256 `a53c3003381fb6a2def6f2ffa33a23748182aa115786ea00a66564598a849325` and 27,342 bytes to `c46e5b85f3a9715a420ac3dba83259e34bd2ae497fe7de6d4003a0a6520b44fd` and 27,404 bytes. A current read matched the replacement digest.

The same descriptor process resumed snapshot two after a two-second backoff and requested a new compiler lease. The graph worker continued running after the original gate released. No process or lock intervention was needed.

## Store Replay and the Next Job Process

The second snapshot acquired two cores after 616 seconds. It passed its target recipe input read, then detected changed bytes for Store's `replay-owned-state-operations/🦀️.rs` before spawning Cargo. The next two inner attempts reported the same source path. The package build ledger totals two source admissions, two capacity grants, two pre-spawn checks and one actual Cargo invocation. A current read of the Store source at 22:08 CET found 52,305 bytes at SHA-256 `5e63d0dd393ebdd0b30a362023273dc4238764c8c2bad28e7d36de80a702b843`, with modification time at 21:57:26 CET.

The driver classified the refusal for a new job process and waited 250 milliseconds. The same site generation retains the other finished prerequisite outputs.
