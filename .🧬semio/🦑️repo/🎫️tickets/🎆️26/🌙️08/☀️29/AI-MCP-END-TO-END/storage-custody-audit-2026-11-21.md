# Storage Custody Audit

At approximately 19:54 local time on 2026-11-21, `df` showed 4.9 GiB free on `.cache/b/r/m` (`/dev/disk9s1`, a separate 55 GiB mounted image) and 12 GiB free on `/System/Volumes/Data`. Ticket `🗑️generated/native-host` is a symlink to `.cache/b/r/m/.ticket-AI-MCP-END-TO-END/native-host`. Its 12 GiB belongs to the image; the other generated directories below reside on Data. Removing Data files does not release blocks inside the mounted image. Estimates are `du` allocated sizes and APFS sharing can make actual recovery differ.

## Custody

The ticket's image native-host is pinned and currently being rebuilt by cargo PID 19983 (started 19:44:40). `lsof` showed it holding `.lock` files under multiple Data `host-spill` package variants, including `semio-os-mcp-plugin-runtime/13f0f3a13342dc4e`, and an active rustc mapped spill outputs. Do not delete native-host, spill, its rlibs, or its compiler dependencies during this build. The current cargo PID 22094 (started 19:46:39) builds Note using ticket `nd/guest` and pinned `cg`; preserve `cg` (7.4 GiB Data). Preserve `native-parser-test` (255 MiB Data), `m-canon` (356 MiB Data), current witnesses and scripts. Preserve `m-parse` (692 MiB Data), whose symbol-bearing Note image is cited as evidence in `canonical-note-symbol-audit-2026-11-21.md`. Other active builds belong to other tickets and their image paths are not candidates. A process/open-file snapshot cannot guarantee future use; repeat the lease and inode checks immediately before any removal.

No specific image artifact is certified safe for removal while the native-host lease is active. The prior `reclaim-superseded-mcp-native.py` selects old native rlibs by fingerprint closure, protects the newest two and named package families, and explicitly requires checking live readers; it is not a substitute for pinned current dependency custody. The earlier `reclaim-orphaned-wasi.ts` specifies five exact old `native-guests` archive candidates, checks component fingerprint closure, size, nlink and lsof before unlinking; its prior candidates must not be presumed present or safe now. The older report in `mcp-end-to-end-2026-11-20.md` already removed the original `m-fixed` pair and retained its descriptors/receipts. No deletion was performed.

## Conditional Data-only candidates

Old experimental model copies, distinct from `m-canon` and the active `cg`, have no matching open file in this snapshot of `lsof`; their names and component inodes are below. They are *candidates*, not verified disposable evidence: confirm no colleague or script still needs the experiments and protect any required receipt or comparison before deleting. Keep the named `m-parse` comparison model. These candidates cannot alleviate image exhaustion.

| Directory | `du` size | Large component inodes (links each 1, modification local) |
| --- | ---: | --- |
| `🗑️generated/m-paid` | 692 MiB | Note 37648027, 357,506,585 bytes, 16:03:01; WFC 37654199, 313,881,914 bytes, 16:06:59 |
| `🗑️generated/m-bin` | 361 MiB | Note 37682640, 112,729,144 bytes, 16:28:22; WFC 37705842, 211,679,412 bytes, 16:50:39 |
| `🗑️generated/m-both` | 361 MiB | Note 37735817, 112,733,233 bytes, 17:05:45; WFC 37775956, 211,681,216 bytes, 17:21:07 |

The oldest component among those candidates is the 16:03:01 `m-paid` Note inode. The oldest relevant observed live custody is cargo PID 19983 at 19:44:40; lack of an open descriptor does not prove absence of a queued consumer or a retained evidence dependency. `m-fixed`, `m-fast`, and `m-tuned` are each 51 MiB; the prior report explicitly preserved `m-fixed` descriptors and receipts, so do not bulk-delete them without a separate evidence audit. `host-spill` totals 7.3 GiB but is active and not a candidate. The exact ticket-owned image dependency closure and sealed repo leases were not established, so no large image-space removal is recommended on this evidence.

## Reclamation

The coordinator checked the candidate roots again for active commands, open files and current harness references. The three superseded generated copies were removed after recording their raw components.

- `m-paid` recorded component hashes `79e96bedd011ed3034184bb8e465414473e1884ec9c0a5a2adb9e1b79cf34cbd` and `cd50f9af3c3c2dfbceeee21ab5db8d31953eec99ac4586dd3f0f8715b79c0737`.
- `m-bin` recorded component hashes `5ee6da5566d67dd9bc6ba9563750d660625ac56bf307dfb9764c4453735a22ed` and `af63b880b7cd3b3fdfb59c521b31e5117b9f2958090ff5a9ba48d155e4a608ec`.
- `m-both` recorded component hashes `3956d3420018d7307e06eaaa8def860409b6da52dc895892fb2e58f0260a1f9e` and `40bc0332eca2e6804cf60825b0d860affe44ac7734d3fd490446134f76eca83d`.

Host Data reported 14 GiB available after the removal. Current canonical pairs, live compiler trees and legacy named-core attribution evidence were left in their existing locations.

The inactive native control-memory reference was archived on Data and removed from the compiler image after exact decompression matched SHA-256 `7df2af74170a160e086e8477701803477c5fe8b492e0381522c2aa86a661d544`. The managed gateway retry subsequently released its owned leases. The compiler image recovered to roughly 11 GiB available once its managed scratch custody ended, and the parallel managed model lane obtained its own fresh compiler grant.
