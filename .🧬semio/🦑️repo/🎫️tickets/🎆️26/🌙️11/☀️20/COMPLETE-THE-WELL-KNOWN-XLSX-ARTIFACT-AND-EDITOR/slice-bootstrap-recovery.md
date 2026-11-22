# Resident Slice Bootstrap Recovery

A fresh slice worker resolves the graph entry before entering its runtime error handler. That entry re-exports the graph loader, which imports the owner checkpoint. When the checkpoint temporarily disappeared during replication, Bun exited the fresh process with code 1. The pool retries exit 75, so the plan stopped on its first fresh worker.

The worker now runs its dynamic startup under the same typed half-edited-module classifier used by the runtime handler. The classifier emits the existing restart exit and diagnostic. Ordinary startup exceptions still propagate, and the runtime handler still sends structured errors and stops its reporter.

The regression starts the real slice worker and pool with a Bun loader fixture in place of the graph entry. The fixture imports a relative provider, and each attempt runs in a separate process. The missing provider cases initially exited with code 1. The third case confirmed that an ordinary exception was already terminal. The neutral fixture describes a restored provider, an ordinary exception, and the same-cause retry limit. The independent Node ESM oracle uses the same fixture.

The restart-cause, restart and worker-error suites passed all 22 tests on Bun 1.4.2. The Node oracle reproduced all three fixture outcomes, including the launch and retry counts. Verification used isolated processes and did not restart the running graph hosts.
