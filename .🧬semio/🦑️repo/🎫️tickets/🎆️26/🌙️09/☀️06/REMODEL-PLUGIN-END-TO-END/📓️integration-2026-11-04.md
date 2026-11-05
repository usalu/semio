# Remodel Integration — 2026-11-04

## Dependency Repair

Reconstruction and mesh used the entire semio-framework SDK only for MeshData. The SDK explicitly reexports that exact type from semio-framework-mesh-engine. Their two manifests now depend directly on that owner, the artifact workspace declares its path, and all seven occurrences in five sources (including the synthetic-scene test return type) name the owner type. Schema and mesh representation are unchanged. This removes an accidental app/plugin SDK dependency without altering source admission budgets.

Removed unrelated energy engine model/results extern aliases from the editor integration entry and both artifact dev dependencies. The surface agent added explicit remodeling command and mode imports; its existing results-panel alias remains explicit. The engine agent owns post-edit native checks; no compiler was duplicated here.

Changed files: artifact Cargo.toml; artifact package Cargo.toml; editor integration entry; reconstruction engine-core, preview-and-products, synthetic-scene test and package manifest; mesh pipeline, interchange-preparations and package manifest.

## Executable Browser Gate

The September serve shell helper has an obsolete absolute macOS path, direct bun invocation and profile dev. Current root dev rejects playground routes. Use:

```
repo run @semio-tech/framework-os-dev:dev-remodel-react-prototype
```

The generator at framework repo library / build / graph / inference / playground-targets / 🟨️.mjs declares dev → activate → prepare. Preparation owns session-remodel, web support, fonts, renderer engines, declared browser session engines and selected component prerequisites. serve-remodel-react-prototype alone only depends on session and may reuse activation, so use the dev target for fresh E2E.

ServeScript accepts serve <variant> react <prototype|release>, sets SEMIO_PLUGIN=remodel, SEMIO_RENDERER=react, SEMIO_BUILD_MODE=prototype, validates activation identity and defaults to declared React port 6063 (S_OS_PORT may override). WGPU metadata port is 6163.

Current component owner: 🌎️hub/🧩️compositions/📸️remodel/🧩️component. Metadata selects s.remodel.remodeling@1/*#editor. Its committed descriptor has capture/model/analyze modes, remodeling-main/frames/report windows, a viewer window, and demo/demo-session/synthetic-orbit examples. Descriptor presence does not establish fresh publication or mounted runtime success.

## Component Closure and Child Persistence

Current metadata has no depends-on, consumes, extends or host field; runtimeComponentClosure selects only remodel for this app-scoped playground. The September report describing remodel plus stdio is stale.

No missing stdio plugin dependency was established. Remodel directly links specific stdio codecs. Model-window world_meshes_json resolves composed child content through resolve_bounded_remodeling_mesh(scene.durable_artifacts, handle), then sends mesh buffers inside World3dScene. Frames rendering resolves remodeling_asset(scene, asset_id) and sends a data URL in Canvas2dScene. Neither route dispatches a stdio child editor. Child dialect names alone do not require mounting the stdio plugin.

The existing six-window ownership integration test exercises document byte stability, separate config lanes, render isolation and reopening. Its neutral JSON fixture is checked by the TS oracle against Ajv and fast-json-patch. Registered editor tests cover replicated edits, retained publication and import/export; reading source does not establish their execution success.

## Independent Source Review

Surface import code returns on base64/decode failure before constructing mutations. Workflow append reserves frame indices and orphan asset keys and checked_add refuses exhaustion. Shared photo-admission JSON regression vectors use the third-party png crate. The engine session harness now uses an original caller and return origin, acknowledges outcomes and retires ownership rather than directly constructing a StepContext. Compiler/test execution remains decisive.

Root and engine own native, WASM, JS and catalog gates. No browser was launched here. After the owned dev gate succeeds, browser verification must inspect current component load and nonempty windows, then drive actual action/import/reconstruction/cancellation and reopening.

## Review Follow-up

Found a remaining concrete API mismatch after the import repairs: command_from_action tests still passed Option values directly, while ArtifactEditor now requires mutable option references for args and original fault. Handed the five callers to the surface owner for repair before native test compilation.

## Executed Read-only Regression Check

Ran repo tools exec --light node -- <ticket>/🐍️integration-source-check.mjs (exit 0). It independently asserts the two direct mesh owner manifest dependencies and workspace entry, five MeshData source files free of umbrella SDK references, removed energy aliases/dependencies, canonical editor id, three modes/windows, three examples, en/de labels for modes/windows/panels/examples, matching source window ids/body keys and all exact config-owner registrations. This checks source/projection consistency, not fresh descriptor generation or execution. The retained ticket script is input, not generated output.

Surface owner subsequently confirmed all five action bridge callers now pass mutable option references matching the trait.

## TypeScript Target Input Contract Repair

The owner target @semio-tech/remodel-js:test previously hashed only its package subtree and plugin Vitest config. Its actual artifact suites, TypeScript twins, neutral vectors and example assets live outside that package subtree. This was a concrete stale cache key gap.

Changed the named input to retain the finite Remodel artifact tree with extensions ts, tsx, json, semio, png, absent and rs. The suite reads JSON/vector/example bytes, synthetic PNG signatures and Rust case-path existence, so all these classes are necessary. The tree is freshness data, not implicit executable authority. The target explicitly names default/^default/testEnv, six actual runtime source entries (config, four suites, declaration regression), source execution version 1, freshness observation, and exact resource entries for package.json plus the project schema read by the regression.

Semantics were read from framework repo build graph admission and script-inputs inference owners: buildInputSourceRoleV1 uses freshness for named inputs; readBuildRuntimeSourceEntriesV1 validates exact finite source paths; targetScriptSources adds these exact modules to the real import closure; readBuildSourceResourceEntriesV1 adds non-executable resources. The project-node/owner-discovery owners exclude source execution targets from native compiler preparation. This is a proper source role for a pure Vitest TypeScript/JSON test; it does not bypass source observation, cache safety, lock handling or immutable dependency verification. Comparable source targets include framework OS MCP test-stdio-source and test-native-staging, and specific dev catalog tests.

No evidence establishes that this role will eliminate the long dependencies:current-input wait. That status originates in dependency storage rehashing current immutable input bytes under its resource control; that work remains authoritative.

Added a language-neutral declaration fixture beside the package test and a Vitest regression in the actual selected config. It compares manifest runtime/data declarations against the fixture, validates runtime entries and resources with existing Ajv, checks actual evidence files and independently checks artifact input matching using existing micromatch. No runtime library dependency was added.

TDD evidence: the standalone equivalent declaration check failed before the manifest change because execution metadata was undefined. After the repair, repo tools exec --light node -- <ticket>/🐍️typescript-declaration-check.cjs passed, including Ajv runtime/data schema validation and micromatch coverage of code, schema, vector, asset, absent-marker and native-evidence suffixes. The actual full Vitest run remains with the root verifier.

Additional changed files: Remodel TypeScript project JSON; plugin Vitest config; package declaration regression TS and its neutral JSON fixture; retained ticket standalone declaration check.

## Current Instance Source Budget Addendum

Read the actual instance Cargo manifest: semio-framework-plugin-instance-app declares maxSourceBytes=262144, maxSourceFiles=32, maxActiveLines=4000. Source admission applies the minimum of the manifest and compilation-unit limits and debits aggregate physical bytes before reads; the earlier 17528 remainder was aggregate credit, not a per-file cap.

Current source has changed concurrently since the initial refusal: composer-api is now 15690 bytes versus the earlier observed 18870. A preliminary explicit-module scan was followed by the real readRustSourceClosureV1 parser from the process native-build source owner, invoked read-only through repo tools exec --light node, with ordinary fs byte reads (no compiler or fabricated resource grant). The parser finds 16 files totaling 254618 physical bytes for no features, artifact-app-testing, and test plus artifact-app-testing. Active-line counts are respectively 3091, 3146 and 3148. This leaves 7526 bytes under the authored 262144-byte crate ceiling.

Largest physical sources: plugin-app-impl root 76750 bytes; close-ladder 28116; render-surface 27173; media-transfer 18464; root 16706; ingress 16595; composer-api 15690; ctx-builders 12195. The other eight files bring the aggregate to the total above.

This establishes current parsed Source facts, not the exact selected compilation unit descriptor or full rustc cfg snapshot. A unit may impose a smaller limit, and shared concurrent edits can change these facts. The compiler gate remains authoritative. If the old rejection recurs, compare its current source identity and actual unit limit before changing architecture. No framework source or budget was modified. A legitimate stable overflow repair would move a domain responsibility into an existing owner crate and delegate through its typed context, or split a bounded owner crate with a schema-owned source boundary; raising caps, trimming observation or admitting an incomplete roster is not a repair. Current totals do not justify such a change.

## JPEG Peer Review

Reviewed surface-owner JPEG workflow changes and neutral regression fixture. The port selects PNG signature or JPEG SOI, decodes that MIME and rejects zero dimensions before reserving an index or building mutations. Stored assets preserve JPEG MIME. The fixture adds truncated JPEG rejection and a 2×3 white RGBA/luma case. The regression uses the stdio JPEG third-party oracle to encode and decode, applies actual durable mutations, resolves stored MIME/dimensions and compares native decoded luma. No concrete source/API mismatch was found; native execution remains required.
