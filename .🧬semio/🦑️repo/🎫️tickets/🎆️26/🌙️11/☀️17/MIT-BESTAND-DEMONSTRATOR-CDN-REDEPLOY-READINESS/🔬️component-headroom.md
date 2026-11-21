# Component headroom attribution

The Energy and Procedural reservations both used the whole-workspace label `component/*`. The observed headroom history had five cold and six warm component samples, all attributed to that same key. The Procedural invocation reserved 14.4 GiB. It waited for the filesystem to reclaim an idle lane before starting a cold build.

The component driver resolved every selected package from its manifest but omitted those packages when acquiring the compiler lane. It now passes their declared Cargo package names to the existing lane API. The lane selector and estimator can distinguish a repeated component from a different selection. A first build can still use general component history. The quantities and headroom floor are unchanged.

The driver wiring test failed before the fix. The component lease-order and headroom suites then passed 39/39, including the independent lock-order oracle. Existing running builds retain their original reservations. A wall-clock improvement has not been measured.

## Full collection pacing

The Demonstrator waiter had spent 1 minute 47 seconds of CPU in 8 minutes 14 seconds of elapsed time. Sampling observed filesystem metadata calls. The loop timed its next collection from the start of the previous scan. In a controlled fixture with a three-second scan the waiter launched scans at 0, 3, 6 and 9 seconds although available space did not grow.

The corrected admission loop measures the next collection interval from completion. A completed collection that yields no net free space uses the collector’s existing 30-second stalled interval. A productive collection or a busy collector uses the normal two-second interval. Admission continues to poll free space and the reservation ledger throughout those intervals. The running command’s emergency monitor is unchanged.

All 17 headroom tests passed with 4,093 assertions. A neutral timing fixture and SQLite reference cover prompt admission after independent recovery, repeated shortage, and a productive partial scan. The existing waiting process had already loaded the earlier implementation. No production throughput change has been measured.
