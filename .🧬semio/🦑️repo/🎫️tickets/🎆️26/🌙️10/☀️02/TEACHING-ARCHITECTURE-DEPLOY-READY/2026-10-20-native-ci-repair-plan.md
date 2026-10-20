# Architecture Native CI Repair Preparation

## Current Recovery Correction

The proposed recovery command is now `bun repo run repo-mcp:build --skip-cache` with SEMIO_BUILD_MODE=release at process birth. Do not pass --configuration: current mode resolution retains that argument and the early recovery grammar rejects it, causing ordinary selected-graph preparation. This supersedes earlier candidate command/hash text below. Current main patch SHA256 `c5d7d92fb5db5b4d90fbb35d8989d4a737777b95383da6c679c3a1b1dda663c0`; supplemental schema consumer patch SHA256 `816f0a991bf6011f9d3cc788a48263a8ba9d97501c975790e6250f7ca8306cc4`. Neither is applied.


Preparation only while Root63416 is active: no test, command or production edit ran. All newly authored data is in this ticket. No Source file was copied.

## Supported Provisioning

No .github/actions directory or reusable native-frontend CI action exists in the checkout. Existing owner `repo-mcp:build` builds current Go frontend through runCanonicalGoBuild, publishing `.🧬semio/🦑️repo/⚡️cache/🗃️bin/<mode>/repo/repo` on Linux/macOS (repo.exe on Windows). Mode is selected by the original SEMIO_BUILD_MODE/configuration; no GOOS override is needed on the authored Linuxx64 runner. `repo-mcp:install` additionally builds and installs to the user PATH while editing profiles. CI can consume the published owner output directly, so permanent user-shell installation is unnecessary.

The build skill explicitly permits an absent/unusable native frontend to recover through the existing owned repo-mcp:build Bun build ingress. This exception is confined to installation recovery. It does not permit ordinary product commands through Bun or raw Go/Cargo. Proposed setup probes the exact release output first, recovers at most once only after failure, probes the exact native output again, and publishes its directory through GITHUB_PATH. GITHUB_PATH affects subsequent steps; probes inside the setup step use the absolute published executable.

The candidate recipe is authored as DATA in `🎯️contracts/native-ci.json`, not installed into a workflow. It preserves the existing sudo env PATH/HOME/SEMIO_BUILD_MODE/CI envelope for the initial/final native probes and one owned recovery. Native --help must exit0; a receipt or ELF magic alone is insufficient (Root observed stale release MCP-only help1). Subsequent ordinary steps use repo, with the same envelope, and no Bun prefix. Pinned Bun remains the internal cold recovery tool, not the ordinary command entry.

## Smallest Production Patch After RED

1. Add one identical native-repo setup step after pinned Bun and before preflight in check/site/proctor only: id native-repo, shell bash, fixture recipe. These are three independent job workspaces and each needs availability proof. No new action/reusable workflow or generic installer is introduced.
2. Replace Bun entry with native repo for the existing13ordinary commands only (6check+3site+4proctor). Preserve argv, release config/env, runner/default-branch/dispatch guards, action pins, permissions, artifact path and login-before-image-publish ordering. Update3header examples to native spelling. Do not add external deployment, artifact upload, push trigger, broad bootstrap bypass, new suites or full deploy-check to this narrow repair.
3. Update the owning readiness fixture release command expectations and native ingress data, and extend the existing compose/Caddy/workflow test. Existing product README command modernization is separately reviewable scope; it is not required to make CI execute natively.

## Schema-First Fixture

Ticket `🎯️contracts/native-ci.schema.json` (draft07, closed objects) governs `native-ci.json`.17neutral cases include usable native/no recovery; absent native/single recovery; stale MCP-only/recovery; missing/late provisioning; docs-only rename; missing binary/failed recovery/wrong profile; GITHUB_PATH visibility; unconditional recovery; raw compiler; ordinary Bun ingress; envelope loss; secret/publish order and scope expansion. Status outputs are expectations, not test results.

## Actual-Source TDD And Independent Oracle

- Reuse current `yaml` parser in course `🧪️tests/🧪️deploy/🧰️deploy-fixtures/🟦️.ts`; it already reads the actual workflow path. Existing test registration is `🧩️compose-caddy-and-workflow/🟦️.ts`. The first new assertion must read this actual checkout YAML and require native setup before preflight in every named job. Current Source must fail for missing native provisioning/native command ingress before any workflow mutation. This is the meaningful actual RED; syntax-checking a fixture alone is not RED.
- Schema checks use existing Ajv as the independent JSON-schema oracle. YAML parses the real workflow into job/step DATA; compare its normalized command and ordering projection against language-neutral expected DATA, without shell execution or workflow-string eval. Neutral negative rows transform that in-memory actual parsed model or explicit launch result rows, ensuring the contract catches a stale help1 even with receipt+ELF, a successful build that produces no usable output, wrong profile, and forbidden ingress. Do not synthesize a passing clone of Source.
- Retain the old guard/order/pins/artifact assertions and extend Step type only for id/shell when needed. Replace existing dependency-sync string detection with native ingress detection so the test does not silently ignore the changed commands. Do not hardcode unrelated dependency step positions.
- Parent schedules firstRED through native `repo run @teaching/architecture-course:test --configuration=release -- quick`, after adding the owned test/fixture assertion and before editing workflow. If the test command refuses before test collection, that is infrastructure refusal and does not count RED. Once actualRED is observed, apply the smallest workflow/fixture patch, then run the same gate for actualGREEN. No raw Bun/Vitest command.
- Source test GREEN proves declarations and control-flow contract only. Zero-touch completion still needs the existing owner to actually produce a release native frontend and native help0 under original release controls, plus an actual clean Linuxx64 CI/preflight/product gate using only the one cold-recovery exception. Do not call the repair finished from a YAML/string assertion. Dispatching the existing release workflow would exercise existing Pages/GHCR publication; do not trigger it merely to verify setup. A read-only/local owned setup proof must precede any separately authorized publishing run.

## Source Fences

- `.github/workflows/architecture-course.yml`: 9010B / SHA256 `d6d1e450b1d8141b20c70485909d6fd5ccb2081f01db4216151a4809629ce75c`.
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🚀️readiness/🔣️.json`: 4174B / SHA256 `d21a3406644fd438f741663990e97358186cc4d1082518048479be3ab1e19a96`.
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🧩️compose-caddy-and-workflow/🟦️.ts`: 8086B / SHA256 `064619a596700cb4be39ab907eba84314f79237b32679f1a8375543de79f3179`.

Fixture SHA256 `80555b87c78f61fcd7ee133a307e6885869509df7f4846558b33ee15d586a86d`; schema SHA256 `0cf645720ddf34e7c86734aadb6ee41dc700d943a1ee672d9ed1db99ca3f2693`. No provisioning, frontend rebuild, test, workflow edit or outward action has run from this preparation.
