# Managed Capacity Observation Successor

2026-10-16T03:43:55.561618+00:00. Read-only lstat/statvfs only. No Native/allocator/image/lease/maintenance/compiler/SDK operation ran. These observations are not custody or admission evidence. Current paths may race other processes; no ownership/deletion decision follows from them.

| Partition | Exists | Device | Inode | Filesystem Bytes | Available Bytes |
| --- | --- | ---: | ---: | ---: | ---: |
| rust | True | 16777238 | 2 | 16834846720 | 4303372288 |
| tools | True | 16777241 | 2 | 8244912128 | 885862400 |
| graph | True | 16777243 | 2 | 3949944832 | 1186205696 |
| dependencies | True | 16777255 | 2 | 8244912128 | 5711187968 |
| products | True | 16777257 | 2 | 25424781312 | 20254294016 |
| diagnostics | True | 16777264 | 2 | 2012176384 | 1862967296 |
| leases | True | 16777260 | 20 | 66019328 | 15278080 |

The unchanged new prototype compiler-state creation predicate needs8589934592 available bytes (new distinct4GiB funding plus unchanged4GiB floor). The observed Rust path reports4303372288 bytes; this is below that predicate. Actual admission still requires the genuine funded owner and fresh checked observation. No new state volume was created, cap/floor relaxed or old material retired.
