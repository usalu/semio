# Hard Memory Admission Stops

The capacity evaluator treated critical system pressure and active swap as hard stops. Its deadline branch could still admit a shrinkable request once its priority deadline passed and the available memory covered the minimum. Its first-worker exception could override both active-swap stops. The forked worker can compete for memory precisely when the host needs to reclaim it.

Updated the capacity schema and evaluator so hard stops keep admission closed to both classes. Requests still receive their applicable starvation outcome or wait for recovery. The first-worker exception continues to serve its existing cases under ordinary low available memory and soft stops. A host with occupied but inactive swap can still make progress.

The essential-worker JSON fixtures cover the incident's macOS counters at 2,325 MiB available, 8,637 MiB compressed, 6,980 MiB in swap, and 103 MiB/s churn. They also cover saturated Linux swap traffic, Windows critical pressure with unavailable swap counters, and inactive macOS swap. Hard-stop verdicts agree with the existing SQLite oracle. The separately regenerated Python oracle vectors test the starvation bound across 8, 16, and 32 GiB hosts.

Four targeted Bun tests passed through `repo tools exec bun` while the kernel reported normal pressure. They covered essential-worker decisions, shrink deadlines, the Python oracle vectors, and gate recovery with English and German progress. The scoped whitespace check and Python fixture drift check passed. No compiler was launched after the incident.
