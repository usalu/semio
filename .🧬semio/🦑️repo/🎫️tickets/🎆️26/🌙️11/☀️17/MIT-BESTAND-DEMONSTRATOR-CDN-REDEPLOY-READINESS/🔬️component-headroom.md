# Component headroom attribution

The Energy and Procedural reservations both used the whole-workspace label `component/*`. The observed headroom history had five cold and six warm component samples, all attributed to that same key. The Procedural invocation reserved 14.4 GiB. It waited for the filesystem to reclaim an idle lane before starting a cold build.

The component driver resolved every selected package from its manifest but omitted those packages when acquiring the compiler lane. It now passes their declared Cargo package names to the existing lane API. The lane selector and estimator can distinguish a repeated component from a different selection. A first build can still use general component history. The quantities and headroom floor are unchanged.

The driver wiring test failed before the fix. The component lease-order and headroom suites then passed 39/39, including the independent lock-order oracle. Existing running builds retain their original reservations. A wall-clock improvement has not been measured.
