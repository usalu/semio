# Native final links and browser core ceilings

The canonical native-only FEM/GIS pair `wKvvlU8` finished with 1,124 executed compiler units, Cargo exit 101, zero linked guests and zero staged components. The guarded local finals ran in sequence after the descriptor tool completed. Cargo emitted no fresh artifact for either component. Each wrapper exited 143 following the PID-specific host-warden TERM. The pair wrote its terminal report before the outer retry dispatcher was retired. The failed pass left the global provenance seal unreached.

| Final | Warden PID | Measured action over 4,096 MiB ceiling | Available at cancellation | Swap-out at cancellation | Remaining disk at cancellation |
| --- | ---: | ---: | ---: | ---: | ---: |
| FEM | 75,917 | 4,168 MiB, earlier observation 4,332 MiB | 3,240 MiB | 24.16 MiB/s | 9,755 MiB |
| GIS | 84,943 | 4,279 MiB, earlier observation 4,277 MiB | 2,738 MiB | 287.27 MiB/s | 7,607 MiB |

The current final rustc command used release `opt-level=s`, thin LTO and 16 codegen units, retaining the WASI stack and 512 MiB linear-memory bounds. Historical FEM final compiler telemetry recorded 4,139 and 4,058 MiB in two completed November 20–21 runs. GIS recorded 6,828 MiB on November 19 and 2,619 MiB on November 21 across different source and compiler observations. Procedural's latest two recorded November 24–25 finals were 6,445 and 6,473 MiB. Historical success is therefore insufficient to demonstrate current local fit.

The existing successful local prototype comparison for Sourcing records 5,519 MiB with thin LTO and about 2.5 GiB with LTO disabled. A previous unoptimized GIS prototype exceeded the Wasm function ceiling with a larger Cargo closure and a different optimization/CGU profile. A pinned compiler query independently confirms availability of `-Z no-parallel-backend`, which preserves ThinLTO and the chosen codegen-unit count. It is a separate candidate for guarded measurement if optimized no-LTO GIS fails the real browser function bound.

The ordinary modular browser publisher uses the repository artifact law of 8,589,934,592 total bytes, 16,384 files and 256 path units. The 67,108,864-byte document-browser-actor limit belongs to the separate document compiler path. It does not govern these modular plug-in cores. Browser engine compile and production Binaryen optimization still exercise the actual per-core function ceiling. WASI's 512 MiB declared linear-memory maximum is independent of each file's byte size. For a fresh release use actual stage receipt, raw/component and optimized core digest custody, browser probe and official localized acceptance. A size/function inventory alone does not show behavior.

The table measures current staged optimized core files and their corresponding compact Vite file sizes. It is a baseline for comparison, and the files precede the new final-link policy.

| Main plug-in | Staged bytes (cores) | Compact bytes (cores) | Max defined functions in one staged core |
| --- | ---: | ---: | ---: |
| cad | 158,326,160 | 37,133,458 | 235,210 |
| demonstrator | 284,324,232 | 65,126,311 | 388,307 |
| energy | 267,650,448 | 49,529,022 | 404,660 |
| fem | 91,802,095 | 22,698,269 | 168,863 |
| flow | 193,858,034 | 48,520,681 | 328,468 |
| gis | 97,822,997 | 25,446,749 | 164,635 |
| playbook | 149,655,544 | 33,432,615 | 215,228 |
| procedural | 129,960,119 | 34,543,481 | 241,284 |
| process | 154,171,028 | 36,468,072 | 232,371 |
| puzzle | 142,540,912 | 34,779,164 | 224,877 |
| sourcing | 160,181,006 | 37,218,496 | 229,875 |

The second, genuinely lean FEM/GIS pair wL2CqpJ selected opt-level s, LTO off and 16 codegen units in its actual Cargo graph. The FEM hub library, before its final link, was terminated by warden record 38421.d0c59fd1.json at 1,871 MiB against the 1,536 MiB library budget, with 2,766 MiB host available and 68.59 MiB/s swap-out. The earlier thin pair had completed this library at 2,260 MiB without the simultaneous host pressure. A domain-owner split is in progress. GIS drains under the same managed release workspace.

The lean GIS final linked successfully within its real owner. Cargo recorded a fresh=false 144,126,651-byte WebAssembly component at SHA-256 e68bca31b4889512fdfdc015e9f8d4c3e18bb332ffc6fbef01f2bfed13360a6c. Its linked WASI component contains a 144,040,504-byte primary core with 397,276 defined functions, a 1,129-byte adapter with 45, and a 510-byte core without defined functions. The raw inventory checks matching function and code counts inside each original core extent. This proves the lean primary is 602,724 defined functions below the prior 1,000,000-function threshold. Browser translation, optimization, release seals and served behavior are separately verified.

The fresh GIS factory and its three canonical transpiled cores were independently retained, alongside a SHA-256-verified copy of the original linked Wasm in ticket diagnostic custody. The shared host descriptor emitter crossed another actual source-input change and entered a fourth attempt after GIS had linked. The combined pair could not seal because of the independent FEM failure. After the diagnostic received verified copies, the operator retired the unsealable aggregate and the exact retry owner, while keeping its compiled evidence and avoiding publication on the false aggregate result.

Stdio's release policy now requests the same size-oriented release configuration as Sourcing, GIS, FEM and Procedural, with LTO disabled and sixteen codegen units. The new neutral fixture first reproduced thin LTO on Stdio. The corrected consumption and policy suites passed 38 tests and 2,113 assertions. The independent pinned-Cargo argument test was combined with those suites in one Bun process and encountered their closed test receiver; it requires its own suite invocation. The Stdio final compiler and browser artifact are pending.

The isolated prototype invocation then passed against the pinned Cargo executable, with all 398 release argument assertions and the private compiler-workspace check passing. The command used the ticket-owned isolated-profile-arguments.test.ts input.
