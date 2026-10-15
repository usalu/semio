# SDK Shared Source Independent Audit

Source-only inspection on 2026-10-15 of derivative projection, index, schema, and capacity interaction. No tests, Originals, probes, builds, or image opens.

## Prior Finding Closed By Source Correction

The original exported-sealer custody hole is closed by the inspected correction. `sdk/📥️source/🔗️derivative/🔏️index/🟦️.ts:45-46` now requires the exact capture from its private WeakMap, exact directory and store, an actually issued live exclusive snapshot lease, the private projection/capture pairing, and Root's accepted derivative-copy custody before filesystem mutation. `requireResourceLeaseCustodyV1` in process leases line 75 validates private lease identity, liveness, resource path, signal, and exclusive mode. Forged `{bytes,stamps:[]}` and copied captures fail before mutation.

Capture publication now exposes defensive byte copies, frozen index entries, frozen stamp snapshots and copied row wrappers while seal retains private bytes and rows (`index/🟦️.ts:38`). Owned chmod updates those private expected stamps. The temporary index name is unique and cleanup only removes a successfully created own temporary file. No new actionable defect established in this correction by Source inspection.

## Inspected Guarantees and Limits

The projection bounds input count and total bytes before cloning input buffers, authenticates the content-addressed snapshot identity and pristine receipt, and retains its own base SH. The bounded source walker checks path/entry/byte limits, materialized ancestry, device identity, known hardlink identity, no-follow file descriptors, and actual file hashes. Existing shared-index validation verifies readonly mode, executable status, size, and hash before returning inode identities.

The intended copy flow releases its own SH before nonblocking EX acquisition, rechecks before sealing, updates expected file stamps after owned chmod, releases EX, restores SH, and rechecks before copying. Failure to obtain EX fails the copy rather than waiting while retaining its own reader. Accepted copy promises are retained by the projection and quote; single-use flags are consumed synchronously. Private projection identity and capacity's private issued quote/accepted-copy scope refuse copied or foreign objects. Source inspection shows callback failure paths wait for accepted copy settlement before releasing their scopes.

The copy creates local directories and symlinks, copies the owned `node_modules/nx/` files, and hardlinks other verified readonly files. This preserves local filesystem namespace for Node resolution; actual Node runtime resolution remains unexecuted in this audit.

Capacity tests now use `.stage-*` paths under `store.root/nx`, and the detached test obtains a fresh second projection. The existing physical test checks successful detached drain. Publication-callback and issuer-callback failure-drain laws remain an explicitly unqualified validation gap; this Source review does not imply runtime success. Root's capacity runtime is currently active and was not touched. Full-clone capacity's verified shared-map union is integrated, but current runtime reader/material integration remains unqualified here. No unknown hardlink allowance is recommended.
