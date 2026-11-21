# CAS Pin Publication Ordering

A graph closure holder uses one file at `<cas>/v1/p/graph-closures.<pid>.json`. Previously every hold and timer renewal captured its own digest array and independently wrote a temporary file before renaming it over the holder file. An older call could reach the final rename after a newer one and remove the newer digests from the published set.

The delayed-write fixture blocks the first rename with digest `a`, then requests `b` and `c`. Before the change, the final file contained only `a`. The holder now uses one write sequence per root and holder. Requests arriving during a write share one follow-up publication, which snapshots the current set and computes expiry when the write starts. Timer renewal uses the same sequence and keeps its original cadence. A failed rename leaves the last published pin usable. A following publication can still commit the current set.

The timer and file-operation fixtures reproduced the failure and checked ordered updates, timer renewal, and recovery from a failed write. Python Draft 7 validation independently verified the pin records generated from the language-neutral publication sequence. Four existing CAS tests passed, including closure protection during collection, release, and a pin created during collection. These tests did not attribute the observed generation 259 closure loss to either race.

## Separate Expiry Cleanup Race

A separate controlled interleaving reproduced stale expiry cleanup. The scanner read an expired receipt. A renewal then atomically replaced its path with an unexpired receipt. The scanner's later unlink removed the new file. The hold writer fix leaves expiry cleanup behavior as it was.

Changing that path safely requires a shared cross-process reclamation protocol or generation-specific immutable files. A second read or stat before unlink does not make the read-and-unlink operation atomic. Skipping expired-file cleanup would retain dead holders' files indefinitely.
