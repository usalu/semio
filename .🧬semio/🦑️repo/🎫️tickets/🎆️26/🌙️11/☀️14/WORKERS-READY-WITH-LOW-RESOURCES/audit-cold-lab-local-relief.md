# Local Controller Disk Relief for the Cold Lab

At 11:23:24 UTC the checkout and `/private/tmp` shared device 16777234 with 54,343,749,632 bytes available. The unchanged cold-lab requirement was 60,129,542,144 bytes, leaving a 5,785,792,512-byte gap. The separate native image service gate was outside this storage check.

| Exact local owner scope | Eligible entries | Preview bytes |
| --- | ---: | ---: |
| Read verdicts older than two hours whose reader PID is gone | 40 | 444,387,328 |
| Aged or dead-owner `semio-*` directories in the user temp root | 9 | 124,219,392 |
| Footprint logs past the authored age or count | 49 | 5,373,952 |
| Expired owner waves, replica-lost entries, and confirmed owner-pool staging | 0 | 0 |
| **Total** | **98** | **573,980,672** |

The three positive scopes did not overlap. A separate same-device census completed all 121,235 entries below those names, found no hard links, symlinks or missing entries, and confirmed the same allocations. APFS snapshots and cloned blocks can still prevent the full estimate from returning to the volume. Scoped ledger previews returned zero bytes for sandbox, scratch, bootstrap recovery, command and test evidence. Loose test results, evidence and sandbox storage occupied 114,688 bytes together. The checkout scratch selection found aged empty units and zero planned bytes. No checkout sandbox images appeared in the native attached inventory. There were no task-view entries at the declared checkout path. The diagnostics test-runs owner lives inside a mounted fixed image, outside this check.

The largest complete local loose footprints were Elan at 2,944,413,696 bytes, replica lost at 921,894,912 bytes and Playwright at 600,072,192 bytes. The existing selectors declared none of those bytes eligible. The current ticket generated directory occupied 56,717,312 bytes and contains active receipts and logs. No cleanup authority was inferred from that measurement.

The CLI currently runs the three positive local controller bounds after an unfiltered cache collection, which can enter additional producers outside this scope. The bounded preview called the local selectors with dry-run behavior, no SQLite mutation, and no removal callback for the dedicated staging selector. There was no production apply, mounted-image change, remote store operation or second cold lab. At 11:26:05 UTC the host had 52,688,805,888 bytes free. Even the full estimated return would leave 6,866,755,584 bytes of that sample's gap.

## Completed native CAS preview at 11:48 UTC

The official local CAS previews at 11:26 and 11:29 UTC returned `locked`. They did not establish that the store had zero eligible entries. An ordinary automatic collector later recorded the current scope digest and a 34,319,593,743-byte usage sample at 11:47:06 UTC. That confirms the live collector wrote the new proof. Available physical disk briefly rose to 57,082,687,488 bytes. The exact source of all free-space movement is not isolated by these samples.

Once the existing collector released its lock, a native `repo cache cas gc --dry-run --json --lang=en` completed on the same local partition. It found 17,776 candidate entries and 320,805,692 planned bytes, with 34,367,987,448 protected bytes. The earlier scoped candidate total and this preview sum to 894,786,364 bytes even before accounting for overlap. At 11:48 the native lab was short by 3,478,724,608 bytes. The eligible local work is insufficient to start it. The preview performed no cache deletion and no apply was issued by this trace.
