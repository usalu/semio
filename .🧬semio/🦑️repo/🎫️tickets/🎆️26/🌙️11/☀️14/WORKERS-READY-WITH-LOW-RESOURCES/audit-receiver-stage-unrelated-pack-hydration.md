# Receiver Stage and Unrelated Pack Hydration

The receiver groups every queued Git put with the inputs it must await before applying the index. This includes pack files unrelated to the staged blob IDs. A slow unrelated transfer can keep the receiver on its previous stage after the exact needed object and author register are available.

The native fixture in `🔁️replica/🧪️tests/🌿️git.ts` uses the actual author and receiver replicas. It publishes the required staged loose blob and independently verifies its presence while the receiver still has the old tree. It then publishes the author stage and a separate real Git pack carrying another blob. The verified-copy port is suspended only for that unrelated pack, while normal queue ingestion and index code run. SQLite determines the requested new index. The red run showed the receiver still on the old tree `624d5b1e…` while the authoritative tree was `8e5814fe…`. Six assertions entered the intended condition before the exact tree assertion failed.

The red transcript is `🗑️generated/receiver-stage-pack-red.log`. The fixture keeps the transfer suspended through the observation, always releases it before cleanup, and checks that ordinary processing drains afterward once stage scheduling is repaired. All Git and replica state in the reproduction are private scratch artifacts.

The receiver now derives priority loose paths from the pending authenticated incremental registers using the existing Git stage parser. It applies those objects and the affected working paths before other Git object copies. If native Git cannot resolve the required old or new object identities, the same follow call can hydrate the deferred objects once and recheck them. A queued base keeps the existing dependency order. The unrelated-pack fixture then passed all eight assertions, including the exact stage while the pack copy was still suspended and complete queue drain after release.

## Repaired scheduling and required-pack control

After the scheduling repair, the existing held-copy test observes the author target before the unrelated pack transfer completes. The adjacent control stages a blob stored only in an explicitly authorized, actually journaled pack and index. Its journal emits both pack files before the stage entry, with no loose PUT for that blob. While the required copy is held, the worker retains its original index, keeps the exact stage register queued, and emits no Git apply. Releasing the copy lets that same follow call hydrate the pack and index, reach the authoritative tree and drain the queue. The required blob still has no loose file.

Both focused fixtures passed with 20 assertions in 2.62 seconds. The final transcript is `🗑️generated/receiver-stage-pack-positive.log`.
