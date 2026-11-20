# Selected owner admission frontier

The old closure loop sorted the admitted roster and serialized every owner's target and input revision on every pass. A chain that discovered one owner per pass needed N(N+1)/2 + N revision serializations even though it inspected only N owners. The final full pass confirmed that nothing changed.

The closure now queues only admitted or changed owners, keeping one pending entry per owner ordered by eligible pass and lexical name. New owners become eligible in the next pass. An existing owner dirtied before its lexical position stays eligible in this pass. An owner dirtied at or after its position waits for the next pass. Target presence and optional-to-required changes trigger a revision check. Input growth triggers a revision check. Provider activation still runs at the original pass boundary and can schedule the next pass. The observation callback, fact validation, canonical output order and controls for error and cancellation retain their existing interfaces and order. The existing graph orchestration and file inventory frontiers are tailored to other work, so the pending queue is private to admission.

The neutral fixture exercises cycles, a dependency DAG, existing and new members, optional-to-required promotion, provider activation from consumer targets and named inputs, a missing required target, cancellation and direct task selection. The independent Python sweep matched output and exact inspection order before and after. The deep chain regression first failed at 525,824 revision serializations and now requires exactly 1,024.

| Owners | Revision serializations before | Revision serializations after | Local median before | Local median after |
| ---: | ---: | ---: | ---: | ---: |
| 256 | 33,152 | 256 | 89.17 ms | 24.29 ms |
| 1,024 | 525,824 | 1,024 | 584.36 ms | 115.78 ms |
| 2,048 | 2,100,224 | 2,048 | 1,263.01 ms | 128.76 ms |

Each median uses three local calls to the real closure function with an in-memory descriptor registry, one standalone owner selected, and each inspection returning the next dependency. The benchmark includes registry validation and excludes real filesystem reads, custody waits and graph loading. It measures this component only. No m06 wall-time change is attributed to it.

The two focused tests passed. The seven-file admission and source-demand battery had 63 passes and 12 failures. Loading the exact pre-change admission source from the Git index in memory yielded the same 12 failures plus the new chain regression. The common failures include a missing Original invocation in the bare runner and source-demand fixtures with incomplete extracted-function bindings. No live graph was rerun or retired.

## Resolution of the adjacent baseline failures

The seven-file rerun passed all 74 tests and 4,264 assertions. The earlier failure count included a teardown report that repeated the exec fixture's missing Original invocation. Resolving that error removed the extra report, so the final run lists 74 tests.

| Fixture family | First observed failure | Repair |
| --- | --- | --- |
| Exec admission and teardown | Original parent Script process invocation required | Issue an Original through the existing script envelope for the in-memory planner fixture. Its independent SQLite closure and exact task assertions still run. |
| Native per-inventory memo | A dependency had no whole-file Cargo observation | Check the role-projected Cargo field rows that the production reader records. SQLite still verifies dependency closure. The fixture now also proves the old observations no longer hold after a dependency-edge edit, and the refreshed rows do hold. |
| Extracted owner Source demand | Missing catalog producer bindings in the partial function body | Supply the empty catalog and observed-file sets for the fixture's catalog-free inputs. Runtime, compile-time, generated-output and compiled-package assertions remain active. |
| Extracted script input and assembly | Missing or incomplete manager imports, cache state and assembly stamps | Reuse the real import ports and cache constructor. Supply fixture stamps required by the assembly protocol. An unexpected failure before Source discovery now surfaces directly. |
| Relative Source ownership | The peer path admitted no owner with the stale fixture binding | Supply the same registry-backed relative resolver that the extracted function uses. The independent SQLite ownership check still runs. |
| Named Source input refusal | A typed catalog refusal differed from the obsolete English string | Check the refusal code, catalog key, owner and input. |

The first repair pass left two test failures. Focused reruns exposed the old hand-built cache without the walk-tools map and the missing assembly stamp binding. Both resolved through the current fixture entry points. All changes are confined to the three test files. No production changes were needed for these failures, and the local scratch directory was removed after verification. No live graph ran.
