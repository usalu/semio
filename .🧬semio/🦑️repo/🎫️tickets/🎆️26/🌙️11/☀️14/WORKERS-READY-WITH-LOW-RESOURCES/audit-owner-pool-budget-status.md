# Owner pool memory status regression

The original test failed alone and in the combined suite. Its capacity fixture offered three children, with six owners configured to complete in 10 ms each. The supervisor reports `starting` while live children are below a positive target and promotes a finished wave out of the open set. The short wave completed before it could exercise the assertion's `nothing-fits` state.

The updated test publishes a six-owner wave to its loopback cache and supplies a 16 GiB host observation with foreground work. With 4,763 MB available, the 2,048 MB floor and 2,457 MB foreground hold leave 258 MB for a child needing 1,237 MB. The fixture's target is zero. The test verifies the matching wave, six claimable owners, zero children, zero target, and all four memory quantities in English and German. It cancels the supervisor when the status arrives. The neutral JSON fixture and the independent Python sizing implementation agree on the same input and expected limits.

The memory-status regression launches zero children.

## Ramp fixture cancellation

The adjacent ramp test originally canceled the supervisor immediately after its first `starting` status. The supervisor sent a drain request and released the detached child. The fixture then removed the temporary fleet root, including the kept wave's staged arguments, while the child was starting. An instrumented run reproduced one spawned child, zero child exits when the supervisor returned, and ENOENT while the child read the staged arguments. The production kept wave normally survives supervisor cancellation. Temporary fallback staging follows a separate cleanup path and was outside this reproduction.

The ramp test now records the real subprocess handles, retains the initial English and German status assertions, waits for the children to report ready, then cancels the supervisor. It waits for each child's drained exit before releasing the fixture. Teardown terminates an unresponsive fixture child after ten seconds and the test requires a drained exit.

The focused repository run passed all eight budget tests and the independent Python comparison, with nine passes and zero failures. The output contained no ENOENT. Production code and coordinator configuration were untouched.
