# Actual Renderer Test Discovery Audit

Read-only config/source discovery2026-10-21. Corrects earlier owned-control-fixture-map runner paragraph: OSroot config is NOT the renderer ordinary/owned test project. No test discovery success is claimed.

Actual owner: renderer/engine/targets/react/packages/typescript/script.ts TestScript:35 calls runVitest(this.root,rest,"../../tests/config/TS"). Public testing/config entry reexports renderer/engine/targets/react/tests/config/TS. That source config is jsdom at292. longInSourceSuites:259 contains UI retained typed wire plus Interpreter/TSX;exhaustiveInSourceSuites:263 adds UiDocumentStore/TSX. Default fundamental/quick includes only quick suite and fundamental fixes one testNamePattern. Therefore a mere OSroot invocation or importing helperregistration module cannot execute OwnedRootSource.

Required selectors for configured runVitest (not a verified CLI execution): SEMIO_TEST_LEVEL=exhaustive,filter absolute UiDocumentStore/TSX entry, --testNamePattern=OwnedRootSource or TypedWire. Ordinary Input/Select/Button source suites: SEMIO_TEST_LEVEL=long or exhaustive,filter Interpreter/TSX and exact testname. Passing raw helper path owned-intake-and-surface/TSX is insufficient because it exports register function and is not includeSource root. Avoid including a file in both include/includeSource.

Actual in-source chain: UiDocumentStore/TSX:690 imports typedwire registration;that invokes owned-intake-and-surface register;Interpreter/TSX registers its ordinary child suites under import.meta.vitest. These roots must be transformed by actual target config for registration to run.

Get-Command repo returned absent in current environment;no repo/MCP test command was available for bounded --list/filter verification. No runner bypass,configrepair,new testauthority or fakegreen was created. Parent should use actual renderer project policy route with list/filter when authentic runner becomes available. Source paths/config prove intended discovery only;runtime module/package resolution and test execution remain gates. Earlier direct broad imports already encountered missing package exports;do not stub them to manufacture a pass.
