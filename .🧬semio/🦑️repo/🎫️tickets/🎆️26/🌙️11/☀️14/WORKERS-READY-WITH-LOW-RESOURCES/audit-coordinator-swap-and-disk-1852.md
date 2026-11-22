# Coordinator Admission After the Owner Cache Change

At 18:52 UTC the native fleet JSON reported no published generation. Builder PID 34356 was progressing and graph-plan PID 97089 reported inference alongside PID 34231. The whole-workspace reader PID 24613 remained first in the 14,336 MiB capacity queue after 27,374 seconds. The evaluator reported `swap`, with warning pressure and 1,647 MiB spare above the 4,096 MiB floor. A separate native capacity sample confirmed `wait/swap` and read 540 MiB observed against the reader's existing 2,747 MiB plan reservation.

The same native capacity sample showed live graph plans PID 34231 at 3,604 MiB observed and PID 97089 at 866 MiB. The active process3d editor Cargo PID 34318 measured 16,533 MiB against 7,168 MiB reserved, and the React dev PID 34196 measured 4,330 MiB against 512 MiB reserved. The active dependency-store producer test PID 95998 measured 1,368 MiB against 512 MiB reserved. These are separate active workloads and snapshots of their process footprints.

The protected host storage sample fell to 15,789,268,992 available bytes against a floor of 16,106,127,360. The ticket filesystem independently reported 18,101,370,880 available bytes in the earlier straddling sample, still below the fixed 60,129,542,144-byte lab gate.

The authenticated complete-only lookup over 13 configured origins returned `none`, with no local held owners. The local pointer was still generation 260 with reason `closure-missing`. Generation 259 retained the matching graph identity and the older implementation. The check found no new head to qualify for adoption. No graph build, process signal, cleanup or admission change was issued.

[Prior coordinator check](verification-low-resource-pulse-1835.md) · [Read-only exact graph probe](probe-archived-graph-compatibility.ts)
