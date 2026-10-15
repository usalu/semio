# Current Original Lease Wait

At 2026-10-15 01:57 Europe/Berlin the two canonical commands PID32348/32359 are behind the actual queued exclusive `repository-graph-facts:/Users/ueli/Documents/semio` resource (86194ef0...). The front original PID30771 retains this EX transaction and `build-graph-state:/Users/ueli/Documents/semio/.cache/b/g/m` (175e70c7...) EX while retaining graph, dependency and diagnostics image readers. The queue ticket identities and immutable resource rows were read without changing any lock. Exact read-only inode/size/mtime comparisons stayed equal.

The catalog fcf482... has 8192 bytes and the actual `queue-claim-catalog` row. Its previously observed holder PID36868 settled. The absent-catalog recovery branch is therefore not proven to cause this wait. The progress label `cache:leases-store` alone does not identify the current resource.

The active graph producer sample recorded 3.7 GiB resident footprint and CPU around 100%; its main thread includes actual lstat calls. This is evidence of active work, not proof of what individual JavaScript loop is expensive. Generated read-only sampling is retained in `🗑️generated/lease-wait-30771.sample.txt`. No process, queue claim, image or transaction was canceled, retired or detached. Caller deadlines and original controls continue unchanged.
