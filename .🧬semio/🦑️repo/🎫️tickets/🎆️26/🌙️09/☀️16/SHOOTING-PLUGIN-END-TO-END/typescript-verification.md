# TypeScript Verification

Reviewed the eight actual test entry files in the shooting TypeScript package script. Parser cases compare the production parser and closed JSON Schema through AJV, including exact valid output. Transform cases compare the production browser transform with neutral point and bounds vectors through Three. Export event cases compare the production ledger with a separate XState lifecycle and exercise asynchronous delivery, failure and cancellation. Camera and sunlight tests independently evaluate the shared native fixture outputs through Three; native production comparisons are owned by the native verification agent.

The registered Bun test ingress is already running in root process session 16173. Child tool sessions cannot access that process; root retains polling ownership and will relay actual test output. No duplicate suite or bypass ingress has been started. No test pass is claimed before the actual result.

## Light Tool Contract Inspection

The repo README line 757 says tools exec waits in the capacity queue unless --light, and its line 750 demonstrates git --version. The command implementation at tool-registry/command line 42 maps --light directly to heavy:false. The process tool run implementation line 155 skips admission for a registry inherit tool unless heavy:true; Bun registry lines 453–456 declare capacity mode inherit, operation command, kind script. Its tool-run option docstring does not define an allowed workload for heavy:false, and no inspected registry documentation defines --light as read-only-only or explicitly permits low-memory unit tests. The README host-capacity line 598 explicitly categorizes a test runner as a heavy command, with a capacity run example for Bun test at line 593. Thus authored evidence supports keeping the unit suite on its normal admitted route. No tests have been started with --light.

## Corrected Test Launch

Root session 16173 exited 1 before executing tests: the eight relative operands lacked an explicit ./ prefix, so Bun treated them as repository-wide filters and the native guard correctly refused broad scanning. The declared owner script uses absolute file paths and does not have this defect. The sole corrected registered Bun suite was started with the exact eight entries, each prefixed ./, without --light or any disabled guard. It is now session 9837, waiting normally for memory admission with zero requests ahead at launch. This is a launch diagnostic, not a test failure.

## Actual Unit Result

Corrected session 9837 completed through normal registered Bun tool ingress with exit code 0. Bun 1.4.2 executed 30 tests across all eight declared entry files: 30 passed, 0 failed, 154 expect calls, 239 ms execution after capacity admission. Export lifecycle: 11 passing cases, including XState events, asynchronous progress/delivery, both failures, cancellation while pending, late delivery suppression, microtask admission cancellation, observer failure cleanup and English/German labels. Icon transforms: 4 passing cases through Three. Parser/AJV: 8 passing cases. Sun fixture/Three: 3 passing cases. Camera fixture/Three: 1 passing case. Existing example assets: 3 passing cases. No TypeScript source changes were needed following the actual run. This proves unit behavior; actual browser and native acceptance remain separately owned.

## Authored Shooting Route Registration

Root added a permanent Bun wrapper at ✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🎥️shooting-route/🔬️unit/🟦️.ts invoking React-owned testShootingRouteV1 against the actual workspace. The existing shooting TypeScript package Script and project now declare9matching test entries. Project test cache inputs explicitly include the hub manifest, Specific shooting variant, neutral test fixture and consumed parser/schema roots. Existing process public facades were checked and correctly reexport their current implementations; no alias/import migration was needed.

Owned Node syntax checks for wrapper+Script exited0; JSON parsed; all9entries and bothwrapper imports exist; max3changedpath length96UTF16. No Bun execution was started by root. Previous30pass/154expectations/8files still describes only the earlier run, not this new9file suite. React sole focused red82568remains capacity queued and no production route repair is applied untilactualfailing-current-source assertion. TypeScript transpileModule checks are syntax/source contract evidence, not runtime boot or fulltypechecking.
