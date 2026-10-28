# Native BESTEST Verification

The persistent `🧪️native-bestest` harness uses production `bestest::simulation_job` and the real `EnergyBatchRun` numerical pipeline, packet custody and close driver. The public engine constructs jobs; no `Engine::run` exists. The harness enables the first-party testing authority and global `HeapWitness` so original caller memory receipts are enforced.

The production EPW adapter pulls the full Stdio artifact and currently encounters unrelated store/replay compilation drift. For numerical verification, `🧪️prepare-bestest-weather.py` maps every simulated weather field from the exact committed official NREL Denver EPW using Python's standard CSV parser. The native simulation receives this typed `WeatherData`; no alternate heat-balance implementation or synthetic weather is used. This does not prove production EPW adapter/browser integration.

## Annual Completion

All four annual cases, 600, 900, 600FF and 900FF, completed with 8,760 hourly samples, six numerical timesteps per hour, and thirty warmup days over the full 2026 representative year. Every real batch receiver transferred its final results and completed terminal retirement against the same original caller.

`📓️native-annual-comparison.md` records the independent EnergyPlus 25.2.0 SQLite comparison. All sixteen acceptance metrics pass the existing tolerances unchanged; two annual mean temperatures are descriptive. Controlled annual heating differs by +0.79% for both 600 and 900; cooling differs by +0.38% and -0.26%. Hourly temperature RMSE is 0.1041 K, 0.0205 K, 0.5598 K and 0.0578 K respectively. The largest free-float extremum gap is 0.8365 K. No numerical physics correction or substitution was necessary for these runs.

## Driver Diagnoses

The initial helper's ten-million-turn ceiling was insufficient for annual input when advancing only one numerical step per turn. Actual cursor evidence showed 3,089,962 canonical identity bytes, all 720 warmup hours, and then 185 of 8,760 simulated run hours. The harness now spends up to 32 real batch steps per caller's 32-rung fuel loan.

The optimized annual run exposed a genuine daily-boundary Fault: packet creation treated an already-spent per-turn page birth as permanent failure. The production packet driver now yields until the same caller's next opportunity. A language-neutral regression failed with `Err(Backing)` before this correction and passed afterward.

The next run stalled at hour 5,453 for case 600 and hour 5,233 for case 900FF. The generic numerical unit fixture's original one-GiB copy stock was inadequate for an annual run: completed case 600 charged 1,561,512,608 copied bytes. The harness now authors its own finite original source at admission: 16 GiB copy stock, one GiB capacity and release stock, 2^30 items/fuel, depth 4,096. No stock is replenished; production per-turn grants and numerical settings are unchanged. It owns the same first-party `OriginalJobFixtureCaller` and persistent decoder throughout. A 100,000-turn unchanged numerical cursor watchdog now detects stalls independently of copied-item receipts.

## Hourly Result Projection

The production numerical pipeline emits genuine hourly zone heating and cooling power, using the kernel's accumulated hourly delivery. The series independently integrate to the annual native meters and exclude warmup samples. BESTEST selects the named zone temperature series and projects actual `Zone Heating Power [name]` and `Zone Cooling Power [name]` traces. It does not assume the first result series is temperature. A language-neutral projection fixture covers an unrelated first series and deliberately unordered stable series storage.

All focused suites pass: sim-unit 29, restore 5, wire 5, BESTEST 16. The separate core simulation suite includes the daily packet-birth regression owned by the coordinator. The reproducible annual harness, typed weather preparation and SQL comparison remain as ticket input sources; generated logs and compiled outputs can be removed after the final integrated checks.

## Publication source oracle

The TypeScript original-source oracle now traverses the actual owned production Rust module tree through `#[path]`, excluding `cfg(test)` modules. All existing no-forged-context, unchanged child-loan, checkpoint custody, native caller and finite fuel assertions remain. The independent Ajv, SQLite and Buffer publication oracle passes with the ticket-owned Bun toolchain; it covers three original fuel vectors and all cross-word ESF surface range combinations. This source/format check is separate from native run execution.
