# Activation Preflight

At 2026-11-04 16:43 UTC, the actual `demonstratorActivationLanes()` and `demonstratorActivationLaneReceiptDirectories(workspace)` APIs were evaluated through registered Node behind `repo tools exec --light`. No receipt was published and no directory was created.

The authored lane set is `generator`, `energy`, `fem3d`. All three activation directories were absent below `.cache/b/p/m/browser/prototype/runtime/react/<lane>/activation`. The preflight is evidence of missing producer outputs, not browser or activation success. The declared `activate-prototype` graph must produce fresh, mutually consistent lane receipts and the exact component union before serving or E2E.

The declared demonstrator `test` target passed 96 tests plus six prerequisites; `test-runtime-catalog` passed its one test plus four prerequisites. The next architecture target is **`canonical-architecture`**. Earlier `architecture-native-r1/r2` attempts used an incorrect `test-canonical-architecture` target name but stopped during build Source admission before target resolution: r1 missing an externally refactored Storybook path, r2 the unchanged Source byte cap. They do not establish an architecture test result.

The current Source capture refusal (`files=1788/2048`, `bytes=16780740/16777216`) also stopped Runtime's native r7 before compilation. Runtime owns exact closure diagnosis and repair; no cap or guard has been raised.
