# Fleet worker readiness during a slow tree comparison

The fleet status reader obtained the worker readiness records and then waited on replica tree and journal comparisons before returning any rows. A five-second timeout discarded the complete readiness readings. The tree comparison could keep reading the store after its caller timed out.

The collector now keeps the latest complete facts published by a reader, records any subsequent timeout against the source, and passes its cancellation signal through the view, attestation and journal reads. The storage ledger also receives its reader's signal. The worker reader publishes the readiness, host name and mapping fields before it starts the optional tree comparison.

The regression failed before the change. The fleet-status suite then passed 31 tests with 2,786 assertions, including independent schema validation of the partial report with Ajv. A real `repo fleet status --json` returned 16 worker rows while correctly marking the worker source as timed out at 5,001 ms. Other source failures remain visible. A worker readiness row alone does not establish a compatible signed compiler slot.
