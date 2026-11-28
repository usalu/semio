# H66: growing images while builds run, lane images sized from measurement

## Measured on macOS 27.2 (scratch images only)
- No image can be stretched while attached: `hdiutil resize` fails with status 35 (UDRW, SPARSE), `diskutil image resize` says `Resource busy` (UDRW, ASIF). So true "grow while mounted" does not exist on macOS (Windows VHDX likewise only dismounted). Linux loop devices can (truncate, `losetup --set-capacity`, `resize2fs`); implemented behind a port with a fake-runner test, NOT run on real Linux.
- ASIF (side finding, not adopted): 32 GiB virtual = 16 MB on disk, resize detached needs no host-free envelope; but 512 MB written then deleted stayed allocated after 5 s (unmap unverified).
- Existing detach-grow retries 24 times (~5 min) on a busy image; found in my first real test run. The new grower counts holders first and answers `wait` at once.

## Landed (11:00)
- `🪜️growth`: `nativeImageGrowthPlanV1` (never shrinks, cap, 85 % threshold, delta limited by 66 % of host free and host free left >= 3 GiB keep (machine floor raised, max 8 GiB), commitment budget, partial growth allowed, detached+held = `wait`), `nativeImageLaneSizingV1` (p95 of recorded lane sizes, floor 4 GiB, headroom 50 % gate; preferred size unchanged so roomy hosts get the same 16 GiB), `growNativeImageShardPlannedV1` (exclusive lease nonwaiting, holders, busy detach = wait, ledger event only after success), `growNativeImageOnlineV1`.
- Pool: new lane shards are gated and sized from the measured lanes (m11 case: 21 GiB host free refused before, now births 13 GiB); existing lane images grow by the plan.
- Doctor/heal: `growShardImagesV1` grows every shard (cargo, components, lanes) of `r` between stages under the partition and shard leases plus holder count; busy = skipped `held`, host short = `unfit` (bilingual).

## Tests
growth 9/0 (plan and lane sizing equal Python oracle on 313 + 106 rows, invariants, real scratch shard: attached resize refused, holder makes growth wait with the mount writable, quiet image grows 1->2 GiB with data kept, sparse); pool 10/0; doctor probes 26/0. `repo typecheck`: 1 error, in untouched `🏊️pool/🧪️tests/🚦️admission` (line 27, pre-existing); dangling-imports clean.

## Not done / unverified
- Nothing runs live on m11/m12 yet; their cargo shard only grows when it is idle between stages (a continuously building worker still waits). Overflow shards for non-lane roots are not possible (mounting over existing files).
- Linux online path and windows untested live. Not wired: ASIF, oversize (>32 GiB) images.
