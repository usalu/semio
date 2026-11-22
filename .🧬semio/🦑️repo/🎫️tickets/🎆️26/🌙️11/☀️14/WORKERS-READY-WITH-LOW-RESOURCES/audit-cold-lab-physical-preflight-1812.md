# Cold Lab Physical Preflight

On 2026-11-21 around 18:12 UTC, the cold lab's existing 41 GiB preflight exited 2. The frozen roster passed with 169,972 tracked paths and a 249-unit maximum clone path. The backing filesystem offered 36,041,158,656 bytes against the required 60,129,542,144, a 24,088,383,488-byte shortfall. The profile remains 8,192 MiB and four cores. The preflight acquired no host grant and produced no lab image or readiness verdict.

A separate native filesystem sample put the ticket's generated directory, `/private/tmp`, `/Volumes/Macintosh HD` and `/Volumes/semio-peers` on device 16777234, with 34,653,880,320 bytes available at that sample. The native external physical disk listing returned no disk. The four previous ticket-generated launch directories contained receipts and notes without a surviving lab backing. The authored remote route audit identified no enrolled cold-lab submission. No alternate authorized backing volume was found.

The previously stalled StorageKit command responded on this host. `diskutil image` returned help in 0.77 seconds. The standalone physical control grew a newly created private case-sensitive APFS UDRW image from 64 MiB to 160 MiB with 67,133,440 allocated bytes. Canonical `diskutil image attach` completed in 1.56 seconds. Independent native queries associated the exact backing and mount with the same device and confirmed APFS and enabled ownership. A non-forced detach completed in 0.70 seconds. The final native inventory had no surviving association and both private paths were removed.

The nested control also passed. A new private 512 MiB sparse APFS image used the lab wrapper's outer `hdiutil attach` options. It attached in 0.64 seconds. Its nested 64-to-160 MiB UDRW backing used the same canonical `diskutil image attach` path as the first Native cache in the cold lab. The nested attach completed in 1.60 seconds. Separate `diskutil info -plist` queries confirmed the two exact APFS devices with owners enabled. The child detached in 0.64 seconds and its parent in 0.66 seconds. Inventory was checked after each non-forced detach and all private probe paths were removed.

A repeat standalone attach completed in 0.89 seconds, verified APFS ownership and cleared its exact association after detach. The final filesystem check found no retained probe work directories or mountpoints. The backing device then had 31,922,479,104 bytes available. The 41 GiB physical preflight still refused. The full lab verdict remains pending.

- [Physical probe](probe-cold-lab-native-attach.py)
- [Previous attach investigation](audit-constrained-native-attach.md)
- [Remote route audit](audit-remote-constrained-lab-route.md)
