# CAS8: host store at the hard reserve (2026-11-24 18:09-)

B = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/`. Landing: `T/landings.md` (CAS8). Lock `lock.cas-admission`.

## 0. Coordinator items (18:11)
- Collector pid 37192: sampled 5 s: main thread idle in `kevent64`, thread pool in `lstat` (281) / `stat` (96) / `openat` (84) / `unlink` (24): it was walking the store, not stuck. It freed little because of the costs in section 1.
- Pin `remote-cache-live-76726.json`: 0 digests. 76726 is alive: the local remote-cache server of `repo service host run` (listens on localhost:6090). It holds nothing, so dropping it would free nothing; no stale-pin rule needed.
- Freed now: `gc --unreferenced` cannot wait on a busy lock any more (section 2a); a manual run 18:15:16-18:28:01 removed 5,160 objects, 2.75 GiB (94,400 kept, 765 s incl. 120 s lock wait). Free 74.3 GiB (18:15) -> 80.2 GiB (18:28) -> 83.2 GiB (18:41), the rest by the sidecar's collections and the build's own churn.
- "17.34 GiB referenced by nobody" was NOT collectable: the old status computed it as total minus (pins + holders + every lease row); objects hard-linked elsewhere (foreign links), younger than the 180 s grace, or with ended leases were in it. The census (section 4) now splits those classes.

(sections 1-6 below)
