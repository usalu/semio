# Native Terrain Archive Witness

The browser export was 8,933 bytes with SHA-256 `4180dd9b9303b2b0a8f4228715bb85c7c91aca6e590ed4d475132788ffc04dbf`. Its version 1 carrier held a 2,255-byte parent Pack, a 6,672-byte parent SPR and zero members. The production cold SPR decoder returned document `01a1ff70-1b95-734f-a393-a61a4fae178a`, schema `gis.terrain`, one edit, two transitions and zero conflicts.

The retained decoder used the archive constructor's limits and a 32 KiB copy grant. A native original job caller reconciled each receipt through `OriginalNativeDecodeSource::receive_observed_work`, the same receiver the archive maintenance lane uses. The retained result equaled the cold history and the decoder reached terminal-empty custody on turn 484.

| Source turn | State | Copied items reported to archive progress | Receipt and custody |
| ---: | --- | ---: | --- |
| 481 | Progress | 481 | One copied item. The decoded history transferred. |
| 482 | Progress | 481 | Empty physical receipt and two observed work units. The terminal inner decoder was removed. |
| 483 | Progress | 481 | 336 bytes released and two observed work units. |
| 484 | Ready | 481 | One copied item. All decoder custody was empty. |

The browser stopped at 481/482 with an empty fault. The archive maintenance lane adds only the copied-item count from DecodeParent progress to the two wire counters. It consumes the Ready receipt, releases the terminal decoder and switches to HydrateParent without another wire count increment. The same wire count can therefore persist across the three decode tail turns and into hydration. The archive adds the final count only when hydration produces the envelope and starts store replacement.

The SPR source returns `Closed` when it removes the terminal inner decoder and converts that result to `Progress` even with an empty physical receipt. The maintenance scanner skips the empty receipt at that point, and batching ends when copied items are zero. Current reactor source separately advertises runnable archive work, which should issue follow-up turns when the archive's quoted demand fits the source stock. The observed wire counter alone does not establish whether the browser missed a follow-up turn or reached hydration.

The complete registered Terrain native app witness could not link from the retained build artifacts. The GIS editor library with label `526cc1907456600d` and the plugin facade both require kernel metadata hash `9d315bed5a7ccbb78bbd12c800925c1d`. The second editor library requires `1cb856e00c7598063938eb376b43d505`. The available native kernel with the same Cargo filename label has hash `7f26a36d215f8dea61a35e7f2769b1b1`, and the native unit cache did not provide either older matching kernel. The evidence here covers carrier and SPR receiving through its terminal handoff. Browser hydration and registered native app hydration remain unverified.

The source and narrow link command are in `native-archive-witness.rs` and `run-native-archive-witness.py`. Pass `history` to run the verified retained history path.
