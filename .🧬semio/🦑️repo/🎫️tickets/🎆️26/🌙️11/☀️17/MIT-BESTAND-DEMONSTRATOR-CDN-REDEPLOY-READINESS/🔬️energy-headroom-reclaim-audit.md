# Energy Compiler Headroom Audit

Read-only snapshot 2026-11-20 around 14:50 PST. No deletion, signal to another process, image reclaim or remount. The Energy headroom log requested 14.4 GiB plus the running reservation and 2.9 GiB floor. Its displayed free space fell from 24.7 GiB to about 21.6 GiB while the remaining running estimate fell from 13.0 to 9.9 GiB; these change during work. The operator should use the live ledger, not add historical figures to current free space.

| Partition | Backing physical allocation | Mounted used | Apparent difference | Ownership observation |
| --- | ---: | ---: | ---: | --- |
| Rust `.cache/b/r/fixed.dmg` | 53.13 GiB | about 20–21 GiB | about 32–33 GiB | Active Procedural Cargo, independent Generator Cargo and the cache watcher have open files. Not idle. |
| Products `.cache/b/p/fixed.dmg` | 12.69 GiB | 8.5 GiB | about 4.2 GiB | Recursive `lsof` returned no files at snapshot; this does not prove absence of repo image lease or future users. |
| Dependencies `.cache/b/d/fixed.dmg` | 11.27 GiB | 11 GiB | about 0.3 GiB | Recursive handle scan did not complete promptly; idle not established. |
| Graph `.cache/b/g/fixed.dmg` | 6.76 GiB | 6.6 GiB | about 0.2 GiB | Recursive handle scan did not complete promptly; idle not established. |
| Tools `.cache/b/t/fixed.dmg` | 3.18 GiB | 2.9 GiB | about 0.3 GiB | Running Bun is on this image. |
| Nx `.cache/b/n/fixed.dmg` | 0.75 GiB | 756 MiB | negligible | No files found in completed recursive handle check; lease state unproven. |
| Diagnostics `.cache/b/o/fixed.dmg` | 65 MiB | 1.1 MiB | at most about 64 MiB | No files found in completed recursive handle check; lease state unproven. |
| Leases `.cache/b/l/fixed.dmg` | 63 MiB | 5.2 MiB | at most about 58 MiB | Active repo process holds SQLite files. |

Backing allocation is `stat` allocated 512-byte blocks; mounted use is rounded `df`. Their difference is an upper-level slack indicator, not a measured or guaranteed reclaim yield. Other non-Rust images combined have less than 5 GiB apparent slack even before lease, eligibility and reclaim overhead. The supported pressure collector takes a non-waiting exclusive image lease, checks pressure and eligibility and refuses a busy image. A coordinated idle Rust reclaim could potentially recover substantial slack, but cannot safely run while current users hold that image, and actual bytes would have to be measured after the supported cycle. Earlier `🔬️image-reclaim-throughput.md` describes the pressure and routine predicates.

This ticket's host-side `🗑️generated` directory occupies about 2.5 GiB total, including active diagnostic inputs and logs, so it cannot by itself supply 5 GiB. Rust-image old run folders total 4.9 GiB and stage cache 2.3 GiB apparent size, but they are not individually proven unused and image-internal deletion is not guaranteed to return backing blocks to the host without a safe image reclaim. No exact idle set with a proven ≥5 GiB host gain was found. The cached browser-module logs inspected for the remaining candidates all contain the same 14.4 GiB headroom prediction; none establishes a smaller eligible reservation or successful receipt under current conditions.

Energy driver PID 79835 accumulated 3:41 CPU over ten minutes, but its instantaneous utilization varied from about 50% to 1%; watcher PID 48509 accumulated 17.5 seconds in 44 minutes and was idle at the snapshot. The admission loop can request single-flight GC at two-second intervals when space is short, and the collector claim prevents concurrent collectors on the same root. Repeated scans are a plausible cost; these process snapshots do not establish duplicate simultaneous collectors or a leak. No process was stopped. The status log and active reservations remain authoritative.
