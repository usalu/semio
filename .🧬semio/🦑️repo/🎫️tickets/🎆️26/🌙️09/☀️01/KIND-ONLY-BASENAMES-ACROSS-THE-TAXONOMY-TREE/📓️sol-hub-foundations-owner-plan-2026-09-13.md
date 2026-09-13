# Hub Foundations Ownership Plan

## Scope

This plan covers the first bounded Hub extraction only: the five current public declarations, Cargo stage environment projection, local-bootstrap framing/authentication/process/credential foundations, direct-child credential delivery, directory ordered-publication verification, the two stale native credential source proofs, and the registered portable source gate. It does not move admin/browser relays, directory journeys, trusted catalog/GIS process composition, or the remaining 52-command implementation bodies.

No product owner has been materialized for this plan yet.

## Proposed owners

1. `🌎️hub/🏗️build/🛂staging-environment/🟦️.ts`
   - `CargoStageEnvironments`
   - `exactCargoStageEnvironments`
   - Projects the inherited build environment and the separately scoped native stack override.
2. `🌎️hub/🧪️tests/🧬️schema/🛂expectation/🟦️.ts`
   - `HubFixtureExpectationV1`
   - `assertHubFixtureExpectation`
   - Owns the shared fixture-stage admission result vocabulary; this receives two of the five current public declarations.
3. `🌎️hub/🚀️local-bootstrap/📡️framing/🟦️.ts`
   - `LOCAL_BOOTSTRAP_FRAME_MAX`
   - `LOCAL_BOOTSTRAP_DEADLINE_MS`
   - `LocalFrameReader`
   - `writeLocalFrame`
   - Owns the four-byte length prefix, retained-input/outstanding-read bounds, deadlines, JSON admission, zeroing, EOF/error handling, and bounded writes.
4. `🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts`
   - `LOCAL_BOOTSTRAP_SCHEMA`
   - `LocalClientClass`
   - `LocalProfile`
   - `hmacProof`
   - `authenticatedFrame`
   - `verifyAuthenticatedFrame`
   - Owns canonical authenticated frame construction and verification and the profile/client-class admission types. It imports the framing byte bound.
5. `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts`
   - `LOCAL_READINESS_DEADLINE_MS`
   - `LocalHubRun`
   - `LocalHubStartOptions`
   - `LocalHubRunAllocationOperations`
   - `allocateLocalHubRunRoot`
   - `freeLoopbackPort`
   - `hubBinaryPath`
   - `hubDevBinaryPath`
   - `startLocalHub`
   - `waitForReadiness`
   - `waitForChildExit`
   - `finishLocalHub`
   - Owns process allocation, start, handshake, readiness, bounded diagnostics, endpoint retirement, secret zeroing, and final cleanup. Allocation accepts an explicit parent/operation boundary so portable proofs never allocate in the system temporary directory. It imports framing/authentication and the real GIS checkpoint bound.
6. `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts`
   - `issueLocalCredential`
   - Owns one authenticated issue exchange and complete credential-envelope binding. It imports framing, authentication, and the execution-owned `LocalHubRun` type without a reverse edge.
7. `🌎️hub/🔐️auth/📤️credential-delivery/🟦️.ts`
   - `DIRECT_CHILD_BENIGN_ENV_KEY`
   - `DIRECT_CHILD_BENIGN_ENV_VALUE`
   - `isProtectedDirectChildEnvironmentKey`
   - `sealedDirectChildEnvironment`
   - `directChildEnvironment`
   - `deliverCredentialEnvelopeToChild`
   - `deliverNativeCredentialEnvelope`
   - `deliverMcpCredentialEnvelope`
   - Owns protected-parent environment removal, the fixed fd-3 marker, native/MCP stdio differences, one-shot frame delivery, failure cleanup, and capability erasure. The last two declarations complete the five current public declarations.
8. `🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts`
   - `OrderedAppendBroadcastFixture`
   - `orderedDirectoryPublicationOracle`
   - Owns the four-case fixture oracle and the source-as-data proof that append and broadcast remain under one writer-guard lifetime.
9. `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts`
   - `sourceDefinitionBodies`
   - `nativeCredentialSourceOrderConforms`
   - `mcpCredentialSourceOrderConforms`
   - `proveNativeCredentialSourceOrder`
   - `proveMcpCredentialSourceOrder`
   - Owns the current WGPU/MCP entrypoint and runner source proof. It selects one exact current authority for each runner, extracts actual function/class bodies with balanced lexical boundaries, and compares execution calls rather than declarations or global token positions.
10. `🌎️hub/🧪️tests/🧱️foundation-source/🏃️execution/🟦️.ts`
    - `HubFoundationSourceScript`
    - Owns the bounded Bun-test command invocation through the repository's lower routing and owned-process authorities. The Hub package command imports this class and only registers it.

The existing `🌎️hub/💡️inference/🧬️schema/🟦️.ts` becomes the single source of `GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES`. The local-bootstrap execution owner and existing GIS proof callers import it. The value is removed from the package command rather than copied into a new local-bootstrap owner.

## Dependency graph

The ten-owner graph has seven intended internal edges:

- authentication → framing;
- execution → framing and authentication;
- credential issuance → framing, authentication, and execution by type;
- credential delivery → framing.

Cargo environment, fixture expectation, ordered-publication verification, credential source-order verification, and source-gate execution have no edge to another proposed Hub owner. Source-gate execution imports only the existing repository process routing and owned-execution authorities. No owner imports the Hub package command or Hub package root. The package command imports the owners directly, leaving no compatibility facade or cycle.

## Context and source registration

The extraction adds exact contextual semantic kinds for the owner paths and the schema/fixture authorities. The planned context inventory is 26:

1. `hub-build`
2. `hub-build-staging-environment`
3. `hub-tests`
4. `hub-tests-schema`
5. `hub-fixture-expectation`
6. `hub-local-bootstrap`
7. `hub-local-bootstrap-framing`
8. `hub-local-bootstrap-authentication`
9. `hub-local-bootstrap-execution`
10. `hub-local-bootstrap-credential-issuance`
11. `hub-auth`
12. `hub-auth-credential-delivery`
13. `hub-directory`
14. `hub-directory-publication`
15. `hub-directory-publication-tests`
16. `hub-directory-ordered-append-broadcast`
17. `hub-foundation-source-test`
18. `hub-foundation-source-execution`
19. `hub-auth-tests`
20. `hub-auth-credential-source-order`
21. `hub-inference`
22. `hub-inference-schema`
23. `hub-schema`
24. `hub-foundation-source-schema`
25. `hub-fixtures`
26. `hub-foundation-source-fixture`

The schema-first contract files will be:

- `🌎️hub/🧬️schema/🧱️foundation-source/🔣️.json`
- `🌎️hub/🧫️fixtures/🧱️foundation-source/🔣️.json`
- `🌎️hub/🧪️tests/🧱️foundation-source/🟦️.ts`

The Hub Rust project will gain exact `hubFoundationSources` inputs covering the ten owners, existing GIS bound authority, source fixture/schema/test, local-bootstrap and auth schemas/fixtures, ordered-publication fixture/Rust source, the two current WGPU/MCP source-proof populations, package router/project/Cargo manifest, taxonomy, and seed/derived launch catalogs. The registered route will be:

- command: `foundation-source-check`;
- Nx target: `os-hub:foundation-source-check`;
- launch: `⚖️gate🧱️hub-foundations📐️source`, group `4_gate`, order `411.10755`.

Existing Hub command names and all current source-data readers remain intact. Package-local readers that inspect still-local process/GIS bodies continue to read the package command. The new source contract directly binds the moved declaration population to the actual owners and rejects residual moved declarations in the package command.

## Credential source-proof repair

The WGPU proof will read the one current runner `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts`. It will prove the actual `RunScript.run` → `runNativeSession` → `runNativeBinary` → `runTool` chain and the non-wasm Rust `main` order from credential claim through plugin selection and native activation. It will no longer reference the removed Rust-package runner or the obsolete `runCmdStatus(nativeBinaryPath(ship))` token.

The MCP proof will read its current runner once. It will select the Rust `main` body containing `claim_inherited_local_hub_credential("mcp")` and prove that call precedes the `parse_args()` invocation inside the same body. The global `fn parse_args` declaration cannot satisfy this predicate. Existing workspace credential/grant injection, probe-schema, directory protocol, and direct-binary supervision checks remain. Portable positive and mutated hostile source vectors must fail or pass through the same pure conformance functions. These are source-order proofs only; they do not claim native credential delivery or runtime security.

## Portable and native evidence plan

The portable test will:

- validate its ownership fixture through Ajv and independently parse the JSON with `jsonc-parser`;
- verify all exports, owner paths, exact internal imports, context chains, root imports, absence of moved package declarations, exact source inputs, command/Nx/seed/derived launch registrations, and acyclicity;
- reproduce the pipe fixture HMACs through the extracted owner and Web Crypto, frame and read valid split frames through an in-memory duplex, and reject zero/oversize/malformed/deadline/EOF/outstanding-read hostiles;
- validate the local-bootstrap and auth fixtures against their existing schemas;
- verify process allocation uses an injected private parent, readiness classification is pure/injectable, secrets are redacted, and finish is idempotent without starting a Hub;
- verify protected environment removal, fd-3-only delivery projection, client-class mismatch, capability erasure, and native/MCP stdio selections through an injected child boundary;
- execute the ordered-publication oracle against the real fixture/Rust source and its hostile mutations;
- execute corrected WGPU/MCP source conformance against current sources and in-memory hostile reorder/removal vectors;
- compare Cargo environment projections for default, explicit override, build, and native cases.

After the portable route is green, the native proof will select only the existing local-bootstrap laws:

- `local_bootstrap_hmac_matches_neutral_node_oracle_and_rejects_boundaries`;
- `local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded`;
- `local_session_commit_survives_cancellation_observed_by_final_progress`.

Any native run will use bounded Cargo/Nx execution and ticket-private output coordinates. It will not start the Hub executable, browser, database, relay, or service. The source route does not require a test-artifact environment because all portable process boundaries are injected and no filesystem mutation is performed.

## Explicit limits

This slice will not claim full Hub startup, readiness over a live service, browser/admin relay behavior, database transitions, actual native/MCP child credential delivery, full Cargo feature coverage, or security inference beyond the exact portable/native laws run. It will not execute `dev`, `secure-local-smoke`, a historical command, Git, a live workspace publication, or any full Hub process journey.
