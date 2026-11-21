# Warm Preparation and Inference Completion

The graph coordinator previously fed the warm producer’s attempted-owner count into the inference clock. The warm producer increments that count after either success or failure. Its successful preparations may still need adoption. On the neutral fixture, a warm run with four attempts and two preparations emitted inference completion counts of 2, 3 and 4 before any owner was adopted or inferred. The monotone completion meter then hid the next real lower counts. A scheduler skip also increments the internal traversal counter when fleet waiting ends.

The `project-nodes` reporter now counts distinct owners at the existing cache-adoption and inference completion boundaries. It flushes that count before closing the inference phase. The internal traversal count still drives the same scheduler and in-process diagnostics. The warm-running row reports the sum of settled and already durable preparations. Its human-readable attempt and failure diagnostics remain available.

The neutral fixture runs the actual warm producer and graph phase clock with the project coordinator’s progress callbacks. SQLite independently counts fixture attempts, successful preparations and distinct accepted owners. The fixture observed no inference completion during warm failures or scheduler skips. It then observed exact completion counts of 1 and 2 despite a repeated owner and checked the terminal clock count.

The test failed before the change with premature counts of 2, 3 and 4. The focused repo execution passed 32 tests and 95 assertions across warm progress, warm/take concurrency, fetch-first ordering and graph phase clock. Evidence was collected locally through the repository runner.

The broader integration exposed a duplicate terminal warm report. A deterministic fixture reproduced two identical complete counts when the last owner settled exactly at a reporting boundary. The warm reporter leaves the outstanding terminal count to its final callback. The independent SQLite oracle compared the single count, and the 16-file suite passed 129 tests and 1,111 assertions after this correction.
