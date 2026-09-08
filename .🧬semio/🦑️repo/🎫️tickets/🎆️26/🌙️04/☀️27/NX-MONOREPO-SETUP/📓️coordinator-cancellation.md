# Coordinator Startup and Cancellation

The coordinator's dev/start targets now depend on its Nx build and execute the produced binary through their own script. A runtime probe used an ephemeral localhost port and ticket-local database, reached `/healthz`, and cancelled the public Bun wrapper.

The first probe exposed a signal-forwarding defect: the synchronous wrapper left Node Nx and the coordinator alive. Explicitly terminating that known test Nx process cleaned up both. Root NxScript now awaits an asynchronous Node child, forwards SIGINT/SIGTERM, and retains the opt-in orchestration budget. Cancellation has a bounded forced cleanup for its own process tree (taskkill on Windows, the owned process group on Unix).

The repeated runtime probe passed: the public wrapper's SIGTERM closed descendant output streams within the fixture's 5-second deadline, returned a nonzero cancellation status, and the health endpoint stopped accepting connections. A subsequent process inventory found neither the test Nx process nor its server. This runtime evidence is macOS-specific; Windows and Linux execution remain part of the broader open goal.

## Watch Callback Descendants

A new isolated test executes the current root Nx wrapper class against the installed Nx watcher. Its callback starts a child that ignores SIGTERM and inherits stdout/stderr. The test failed because cancelling the wrapper left that child and its output pipe alive: the earlier wrapper signalled only the Nx process and cleared its force timer when Nx exited.

The wrapper now signals its owned Unix process group and retains shutdown supervision until the group exits or the five-second force deadline. Windows continues to use taskkill's process-tree termination. The repeated runtime check is pending.

The repeated real Nx watcher exercise passed: the root wrapper class terminated the ignoring callback child and closed every inherited pipe within the fixture's eight-second ceiling. The full public Bun/Nx coordinator launcher was then rerun: it reached `/healthz`, cancelled within its five-second deadline, and closed the endpoint. No tested process remained after shutdown.
