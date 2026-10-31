# Artifact Plan and MCP Owning Discovery

Current read-only Source/config audit; no execution or edit.

| Scope | Actual ordinary owner/discovery | Current limit |
| --- | --- | --- |
| New process native-build/plan/tests/TS | @semio-tech/framework-process:test-native-build → script test native-build; suites JSON trees=[native-build]. Recursive discovery selects test-directory TS importing bun:test; deepest-tree routing assigns this file to native-build. | Ordinary discovery exists; initial explicit-files-only audit missed this tree owner and is corrected. No additional mount required. Actual ordinary execution remains unproved. |
| Existing native artifact laws | @semio-tech/framework-process:test-native-artifacts → process/script test native-artifacts. Explicit suite roster contains native-build/tests, receipt, jobserver, admission, telemetry, stage-cache, component-pass (7files). | New plan file absent. Preserve all7 while adding appropriate plan ownership. |
| Existing recipe laws | @semio-tech/framework-process:test-cargo-target-recipe → process/script test cargo-target-recipe. Suite selects recipe/tests/TS only. | Recipe semantics do not own complete admitted-plan lifecycle automatically. |
| OS MCP general TS/integration | @semio-tech/framework-os-mcp:test → package TestScript requires real executable, then existing Vitest config. include is MCP/tests/*/TS. | Covers resolverbinarypath including staging lifetime plus declared top-level legacy/modern/hygiene/end-to-end suites. Does not recursively include stdio/staging/tests. |
| OS MCP native staging Source laws | @semio-tech/framework-os-mcp:test-native-staging → exact stdio/staging/tests/TS via ordinary declared router,15s budget,no Rust prerequisite. | Separate declared owner, not implied by general MCP suite. |
| Actual canonical stdio | @semio-tech/framework-os-mcp:test-canonical-stdio → runCanonicalOsMcpProbeV1 under issued finite phase and actual native executable prerequisites. | Real wire/native lifecycle scope separate from pure staging or plan DATA tests. |

Normal commands use repo run <exact-owner> without forwarded filename/name filters; no tests were launched here. The existing native-build tree provides ordinary ownership for current plan/lifecycle files; future physical child helpers still need correct helper/parent routing and actual positive owner counts, not only standalone execution. No generic wildcard broadening or removal of existing scopes is required by this finding.

Correction verified from actual suites/TS discoverProcessTestFilesV1 (recursive test-directory+extension+test import), routeProcessTestFilesV1 (explicit files else deepest tree) and expandProcessTestSuiteV1. Plan tests import bun:test and are under the native-build tree with no deeper override; the native-build suite has no name filter and supports all three host platforms. No tool/test was executed to establish this Source routing.
