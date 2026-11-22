# Replica target sharing operation counts

The native m05 diagnostic digest reported 40,496 generic target sharing failures at 20:35:01.564 UTC, with 40,496 exact `EPERM` codes and 5,072 incoming entries. The native unlink and staged rename branches both fed the same code counter. The available attestation could establish the native code but could not identify an operation or a holder.

The two custody branches now tag the failed operation at the exact existing native-error route. The recurrence tracker adds bounded `blocked-target-sharing-unlink` and `blocked-target-sharing-rename` counts in spare rows after existing totals, reason rows, incoming subset and native code rows. Operation counts are independent totals. Each row keeps the same kind/count/lastMs shape and omits path and identity. The same queued results, retry policy and file operations apply.

The neutral fixture covers mixed operations with a common EPERM code, bounded eviction and saturated row output, unrelated categories, and missing code. The independent Python oracle models the output; the fixture and diagnostic rows pass the closed resilience, replica status and attestation consumers. Routed race tests on three unlink native errors and a timed rename red-failed on the missing rows and then passed with exactly the right operation. The rename timing and count agree with its independent SQLite oracle.

The full native resilience suite passed 26 tests and 360 assertions. The scoped diff check passed. The first unrestricted race invocation omitted the required process-test artifact root, so eight pre-existing cases failed setup while 30 executed successfully. A bounded ticket-local artifact directory was then supplied to the rerun. All 38 race tests passed with 313 assertions in 10.94 seconds.

The m05 Ready PID 48204 had a separate fresh disk-wait heartbeat at 20:34:40.262 UTC, with progress `Starting collector compile-caches.` and unchanged old dependency timing.

At 20:44:04.974 UTC the native m05 replica attested the new digest `e3e9e32d044406facd0c95ae39f1dedc00100821a7fa4643e6e1d7a3e903b67e` and 5,152 incoming entries. The 4,327 generic sharing failures, 4,327 native EPERM failures and 4,327 rename failures all shared the latest timestamp 20:44:04.960 UTC. The bounded receipt had spare slots and showed no unlink count. This identifies `rename(temporary,target)` for the observed blocked staged puts. The receipt does not identify the filesystem or process condition returning EPERM.

The separate Ready supervisor PID 27824 had renewed its 20:43:42.972 UTC waiting receipt. It reported 4,922 MB free against the 6,144 MB need and still said `Starting collector compile-caches.` The prior dependency attempt remained the last recorded run.
