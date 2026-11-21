# Owner-Pool Throughput Fixture Deadlines

The two in-process throughput fixtures in the owner-pool suite both reached Bun’s default 5,000 ms deadline in the renewal verification. The first runs 64 owners at 40 ms each, sequentially across 1, 2, 4 and 8 slots. Perfect slot utilization still requires 4,800 ms of owner timers before claim and shutdown overhead. The second runs a 200-owner, 20 ms comparison and then a separate run that raises the puller to six slots after 200 ms. Its ideal owner schedule alone exceeds 5,500 ms.

The earlier 4.81× result appears in the renewal green transcript as an unhandled assertion after the timed-out case had returned to Bun and subsequent tests had already started. It measures a different test lifecycle from an isolated comparison. The exact underlying machine scheduling overhead is unavailable. Both throughput ratio checks and the slot-growth assertions passed on isolated reruns.

The two fixtures now declare individual 30-second deadlines. The workload, slot growth, claim-window and throughput assertions are unchanged. An isolated run with a command-line 30-second allowance took 5,186 and 6,570 ms. The final isolated run used default runner settings and the new per-test deadlines and took 5,202 and 6,428 ms. It passed both tests and 12 assertions. The work touched only the test source.
