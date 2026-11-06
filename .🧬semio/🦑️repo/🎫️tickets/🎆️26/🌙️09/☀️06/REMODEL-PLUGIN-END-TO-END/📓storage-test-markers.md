# Native storage test marker migration

Updated direct test-only NativeOwnedStorage and NativeRecordStorage reservation calls in storage unit, header work, public work and record storage work tests. Assertions retain actual marker evidence for denied, failed and successful allocations. The existing storage marker law covers zero-sized backing. Production callers are owned separately. No tests run.