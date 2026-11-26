# Durable Graph Reservation Integration

The production graph reservation entry point was exercised against a private capacity ledger and controlled host samples. The test admits a 3,870 MiB graph grant, attaches a small real worker process and waits until the producer records 126 MiB of observed memory. The observed memory is injected, so the real process does not allocate the fixture's memory. The test lets a one-second forecast ramp pass before spawning a separate contender process that opens the same ledger and requests an exact 2,663 MiB.

SQLite independently verifies that at 5,305 MiB available and a 2,048 MiB desktop floor, the contender fits alone. With 3,744 MiB of unconsumed graph commitment, both requests cannot fit. The contender reports a memory wait and does not enter its body. The test checks that the producer serialized `retainReservation: true` on the active graph grant. After the worker exits and its owner releases the grant, the contender enters with all 2,663 MiB, exits, and leaves the private grant and queue folders empty.

The full graph-plan suite passed all 13 tests and 70 assertions through `repo tools exec --light bun`. The suite used an isolated temporary ledger and two small subprocesses for this fixture. No compiler ran, no shared lease was acquired and no host pressure or service restart was forced.
