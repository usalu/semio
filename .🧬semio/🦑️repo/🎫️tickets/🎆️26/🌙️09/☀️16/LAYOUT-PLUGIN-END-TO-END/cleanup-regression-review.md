# Native Child Cleanup Regression Review

2026-10-28. Independent source review of activation-cleanup.md, the declared process target/table, platform retirement function, regression source/fixture/schema and actual invocation cleanup implementation. No builds, tests, graph operations or physical cache operations performed.

## Finding

The reported2test/77assertion pass is a meaningful executable ordering regression for the present platform coordinator. No actionable defect found within its stated test-owned boundary. It does not prove real native-cache retirement, as the packet correctly states.

The regression reads the current platform source at runtime, finds the exact top-level retireNativePlatformFundedChildStorageV1 declaration with the TypeScript AST, extracts its actual getText and transpiles that function. It does not reconstruct its decisions in a second hand-authored coordinator. The function runs through new Function with explicit ports. Its present production body captures cleanup, attempts physical retirement, awaits beginAcceptedPhysicalCleanupV1, and only then selects successful/absent/retained-reader settlement, preserving its first failure. The separate AST law requires awaited begin and awaited settlement calls later in source; this structural check alone is weaker than control-flow domination, but the concrete branch executions complement it.

## Authentic Controls and Pending Drain

The test imports the actual routing implementation. withScriptProcessEnvelope/createScriptProcessEnvelope/createIssuedResourceControlV1 issue authentic fixture controls; withIssuedResourceOperationV1, withAcceptedPhysicalCleanupV1, beginAcceptedPhysicalCleanupV1 and checkAcceptedPhysicalCleanupV1 are passed directly from that imported module, not mocked. Native require/refinement checks remain authentic where they check issued control lineage.

The empty cleanup/writer objects are used only by test-owned native ports, which compare identity. They are never passed into a real private native cleanup custody gate. The enrollment tested by the real check is supplied by the actual withAcceptedPhysicalCleanupV1 callback. Capture explicitly asserts that check refuses the body phase. A20ms child is registered using the actual withIssuedResourceOperationV1 on the accepted body control, deliberately not awaited by the port. At settlement the real cleanup check must pass and bodyStopped must already be true. The exact expected event order also places body-child-stopped before either settlement. Actual begin drains accepted.body, stops body acceptance and switches phase only after drainage. This tests the previously failing ordering without forging cleanup authority.

## Concrete Cases and First Failure

The fixture really exercises all five declared modes:

- Absent begins without namespace; settlement rechecks filesystem absence with existsSync and lstatSync after drainage.
- Present creates namespace/child directories, performs actual rmdirSync on the test child before settlement, then rechecks child absence and exact writer identity.
- Capture failure returns no cleanup, preserves the same captureFailure object and leaves held charge1.
- Retirement failure throws the original physicalFailure object, then independently refuses successful settlement because the child still exists; final caught object must remain physicalFailure, not the later refusal.
- Reappeared creates namespace/child during capture after absent was sampled; later absent settlement rejects the actual present namespace and leaves charge held.

Test namespaces are fresh mkdtemp children of the owned SEMIO_TEST_ARTIFACT_DIR and are removed in finally. Test native image/policy/open/maintenance/retirement/settlement ports operate exclusively there. They confer no real image, mount, journal, reservation or cache custody. The suite never calls the actual production acceptNativeStorageCleanupV1 or settlement journal implementation. Its held variable is fixture accounting rather than real journal accounting.

## Independent Oracles and Route

Ajv independently validates the portable JSON fixture shape, allowed modes and required outcomes. It does not independently derive those expected outcomes. SQLite independently evaluates the recorded event rows for settlement-before-drain violations and derives held-charge outcome from settlement events. This is a useful independent ordering/accounting cross-check; it is not an independent physical/journal observation because its input is the same test event ledger. Exact fixture event equality, real test-directory observations and direct gate assertions supply the complementary evidence.

The project declares test-native-child-cleanup as uncached repo:script with the storage/invocation source inputs and command repo ./script.ts test native-child-cleanup. The owned suite table names the exact test file, strict typecheck,30000ms budget and darwin/linux/win32 platforms. The script receives its existing original invocation and routes through runProcessTestSuiteV1. No alternate fabricated manual invocation is introduced by the regression.

## Evidence Limits

The actual retained-reader fallback is intentionally a refusing test port and is not exercised; successful/absent/capture/physical-failure/reappearance paths are the claimed coverage. Real physical cache backing files, writer leases, journal charge release, mount operations and backend integration are outside this regression. The passing packet justifies retrying activation with current captured source; it does not establish successful activation, browser readiness or goal completion. The reported run result was reviewed from the packet, not independently rerun.
