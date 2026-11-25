# Exact capacity command reservations

The first protected Norm launch requested 6,144 MB and four cores. The gate's lone-waiter fallback returned 1,958 MB, and the sweep refused it before launch. A simulated host reproduced the gate failure with an explicit 2,048 MB request that started under a 1,452 MB reservation. An inherited undersized grant also started a child.

An explicit `repo capacity run --memory` now sets the declared minimum equal to the requested size. The lone-waiter fallback honors declared minima, while catalogue estimates still reduce when enough headroom covers their lower bound. The launcher reads the live same-ID reservation immediately before spawning. It accepts a sufficient parent grant and refuses undersized, quiet or lent grants and governor bypass. The refusal reports the required and available memory and cores in English and German.

The two capacity suites passed 32 tests and 249 assertions. The suite verifies the exact and estimated requests against three fresh-admission cases and six inheritance cases using fixture hosts and mock child processes. SQLite independently reproduced all nine expected fixture decisions. Two status tests now look for the grant row by its label because the warden status can add a preceding line.

No browser, Vite or compiler ran in this pass.
