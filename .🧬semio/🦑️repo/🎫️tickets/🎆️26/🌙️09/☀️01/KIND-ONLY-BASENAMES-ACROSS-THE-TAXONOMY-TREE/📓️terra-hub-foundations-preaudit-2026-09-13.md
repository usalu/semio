# Hub Foundations Composition Preaudit

## Status

This report began as preparation. The current accepted evidence is in the next section; the historical reconnaissance did not itself run a Hub command, child process, browser, or service.

The production command remains the large router `🌎️hub/📦️packages/🦀️rust/📜️script.ts`. This report proposes the bounded first split only. It does not imply a full Hub-route migration or runtime acceptance.

## Current Evidence And Bounded Acceptance

**Accepted for the Hub foundations source-ownership slice.** Final ordinary package and isolated cache-skipped Nx routes both passed **11 tests / 259 assertions**; the registered `os-hub:foundation-source-check` target took 3.9 s. The earlier 11/256 route is superseded by the final three-assertion closure. Its private graph-cache ENOSPC log did not prevent uncached target execution, and the executor removed its private Nx directories afterward.

The final fixture records 10 owners, 47 moved declarations, 7 internal edges, 33 root imports, 26 contexts, and 27 exact inputs. The source graph has no root compatibility facade.

The permanent cfg-test syn oracle ran through a ticket-private minimal Cargo harness: 1 passed, 0 failed, 0 ignored, 0 filtered, and 0.01 s test body. It used the locked workspace syn 2.0.117 and exercised the raw-string/comment hostile plus current MCP and WGPU source parsing. A broader Hub Cargo target never reached semio-hub because unrelated live plugin WindowConfig compilation errors E0308 and E0616 stopped it first. This is a compilation-environment limit, not a failed Hub law.

The accepted boundary covers source ownership, one-way imports, source/data/input/launch registration, portable fixture controls, and the selected syntax-aware Rust parser law. It does not claim a Hub service, child credential delivery, a WGPU or MCP process, browser relay, database transition, or full Hub Cargo-suite execution.

## Current Public Surface

The router currently exports exactly five declarations:

| Declaration | Current responsibility | Proposed owner concern |
| --- | --- | --- |
| `HubFixtureExpectationV1` | fixture expectation shape | shared verification contract |
| `assertHubFixtureExpectation` | enforces declared contract admission disposition | shared verification contract |
| `orderedDirectoryPublicationOracle` | reads the ordered-directory fixture and source-checks the single-writer Rust seam | directory ordered-publication verification |
| `deliverNativeCredentialEnvelope` | native specialization of one-shot credential delivery | credential delivery |
| `deliverMcpCredentialEnvelope` | MCP specialization of the same delivery | credential delivery |

A bounded whole-workspace symbol search found no external import of these exports. That permits relocation without introducing a package facade. The resulting source test still needs to prove all direct imports/re-exports and computed consumers after the move; this search is not evidence about dynamically constructed imports.

## Proposed Bounded Ownership Map

| Concern | Proposed semantic owner | Current contents |
| --- | --- | --- |
| Cargo-stage environment policy | `🌎️hub/🏗️build/🛂staging-environment/🟦️.ts` | `exactCargoStageEnvironments`; its build versus native stack policy |
| Fixture-admission helper | `🌎️hub/🧪️tests/🧬️schema/🛂expectation/🟦️.ts` | `HubFixtureExpectationV1`, `assertHubFixtureExpectation` |
| Local frame transport | `🌎️hub/🚀️local-bootstrap/📡️framing/🟦️.ts` | frame max/deadline, `LocalFrameReader`, bounded read/write, parse/EOF/error cleanup |
| Local authenticated-frame proof | `🌎️hub/🚀️local-bootstrap/🛂authentication/🟦️.ts` | bootstrap HMAC domain, authenticated frame creation and verification |
| Local Hub process lifecycle | `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` | `LocalHubRun`, loopback-port and binary selection, start/ready/finish and redacted diagnostics |
| Local credential issuance | `🌎️hub/🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts` | authenticated `issue` frame and envelope binding |
| Credential environment and child delivery | `🌎️hub/🔐️auth/📤️credential-delivery/🟦️.ts` | protected-key filtering, fd-3 marker, one-shot envelope delivery, native/MCP public wrappers |
| Ordered append/broadcast verification | `🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts` | fixture oracle and exact Rust source/read boundary |
| GIS checkpoint-control bound | retain its current GIS/inference authority | the 256-byte control bound used only to construct the optional checkpoint frame reader |

The final path names remain executor choices subject to current taxonomy. The concern separation is necessary: framing, authenticated proof, process lifecycle/readiness, credential delivery, directory publication verification, and Cargo stage policy have different consumers and failure modes. Neither a single replacement monolith nor duplicate helpers across route owners preserves those boundaries.

`LocalHubRun` belongs with process lifecycle. The credential-issuance owner may depend on that type and authenticated framing; the lifecycle owner must not depend back on credential delivery. Credential delivery depends on framing for its one-shot pipe but does not own Hub startup. This creates one-way dependencies.

The 256-byte GIS checkpoint-control frame policy is currently declared at the later GIS/inference slice of the router. Local bootstrap should import that real policy only for the optional inference-control pipe; it must not copy the number into every local-frame caller. Its normal local bootstrap frame maximum remains the separately declared 16,384-byte contract.

## Current Consumers And Inputs

`exactCargoStageEnvironments` has 23 statement readers in the intake graph (24 textual calls including its definition). It supplies both normal and native Cargo execution environments. Its owner therefore needs every direct Hub command/proof caller and relevant project inputs; it must not become an exported general environment API.

`LocalHubRun` has 36 statement readers in the current graph. Its current lifecycle functions are used across secure local smoke, admin relay, directory, MCP, GIS, and browser-process proofs. They need direct imports from the lifecycle owner. Any test allocation must inject a ticket-private root or allocator: the current `startLocalHub` uses `mkdtempSync(join(tmpdir(), "semio-hub-run-"))` and cleans it in `finishLocalHub`, which is unsuitable as unqualified acceptance-test behavior.

Credential delivery currently:

1. removes token/session/capability/bearer/authorization/cookie and Hub user/url variables from the inherited environment;
2. sets only `S_LOCAL_CREDENTIAL_FD=3` plus a benign sentinel;
3. opens fd 3 as the one-shot credential pipe;
4. writes a local credential envelope through the bounded frame writer;
5. clears the capability field in all return/error paths.

Those are one concern and should stay together with both public `native` and `mcp` wrappers. The code must retain separate direct-child stdio choices for native and MCP, with a focused hostile same-environment control rather than a full child-service claim.

The ordered directory oracle owns source-as-data access to:

- `🌎️hub/📇️directory/🧫️fixtures/📣️ordered-append-broadcast-v1/🔣️.json`;
- `🌎️hub/📇️directory/🦀️.rs`.

Its four fixture cases are a bounded source/law seam. They are not process, database, or browser acceptance.

The existing local-bootstrap source/data seam is already available:

- schema: `🌎️hub/🚀️local-bootstrap/🧬️schema/🔣️.json`;
- pipe fixture: `🌎️hub/🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json`;
- idle-admission fixture: `🌎️hub/🚀️local-bootstrap/🧫️fixtures/⏳️idle-admission-v1/🔣️.json`;
- auth schema/fixture: `🌎️hub/🔐️auth/🧬️schema/🔣️.json` and `🔐️auth/🧫️fixtures/🔑️capability-v1/🔣️.json`.

The Hub Rust project currently has a broad default input over `🌎️hub/**/*.rs` and project-root files, plus two OS activation/hash inputs. The extracted TypeScript owners, their schema/fixtures/tests, the direct directory Rust source reader, the GIS bound owner, and seed/derived launch entries must become explicit target inputs for the new ownership route. Existing `os-hub` commands and their launch entries remain routing authority; extraction must not remove or execute them.

## Existing Native And Portable Seams

The existing local-bootstrap Rust test owner provides three focused native seams:

- `local_bootstrap_hmac_matches_neutral_node_oracle_and_rejects_boundaries`;
- `local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded`;
- `local_session_commit_survives_cancellation_observed_by_final_progress`.

The first compares local HMAC behavior with the neutral Node fixture and boundary rejections. The second proves idle listener/admission/deadline behavior through a private duplex. The third proves cancellation observed after durable local-session commit. They are suitable focused native evidence after the JavaScript owner move, subject to private artifact/Cargo coordinates. None was run by this preaudit; none establishes whole-Hub startup or browser behavior.

Portable controls should independently parse the local-bootstrap and auth schemas with Ajv, validate their frozen fixtures, verify the frame prefix/size/deadline and HMAC proof laws, reject malformed/non-fd3/credential-class/environment vectors, and source-check the exact direct owner edges. The ordered fixture needs a separate pure decision oracle and source-as-data linkage to the Rust publication seam.

## Known Separate Defects And Boundaries

- `proveMcpCredentialSourceOrder` currently constructs two identical existing MCP runner paths and then requires exactly one. The Sol Hub lane owns that product source-selector repair; root's invocation was read-only. Do not use this preaudit as evidence that an MCP native command failed.
- The Sol Hub lane owns the stale native/MCP proof-selector repair; root's earlier source-proof invocation was read-only. This map preserves their source-data closure but does not modify it.
- The router has dynamic OS-dev imports at current intake lines 7054, 9197, and 12063. Their rebind belongs to the active OS extraction and must be preserved in any Hub source closure.
- The current root command has 52 registered commands and many domain-specific classes. This preaudit does not collapse them, classify all of them, or accept their execution.
- No schema/fixture control should use the router body hash as an implementation oracle. It should assert declarations, direct imports, source-data paths, target inputs, registrations, and behavior-specific portable/native laws.

## Required Acceptance Before This Slice Can Be Accepted

1. Schema-first owner/ancestry fixture records the declared bounded owner set, all direct source/import/data consumers, exact target inputs, package routes, and seed/derived launch registrations.
2. The production router becomes a thin direct dispatcher for this slice; no moved local-bootstrap, credential, Cargo-stage, or directory-oracle body remains there.
3. The owner graph is acyclic; process lifecycle, credential issuance, and delivery follow the one-way edges described above.
4. Portable fixtures prove exact local bootstrap/auth/directory contracts and hostile cases, including the 16,384-byte local-frame bound and sourced GIS 256-byte checkpoint bound.
5. Native execution uses ticket-private artifacts and selected local-bootstrap unit laws. It must distinguish native law evidence from process/browser/service execution.
6. A registered ownership target runs only after explicit inputs and launch registration are current; it must not execute live `dev`, `secure-local-smoke`, database, browser, or service commands.

## Limits

This is a read-only architecture and source/consumer review. It does not prove Hub behavior at runtime, process cleanup, child credential delivery, a Cargo stage, a database transition, or browser relay behavior.


## Current Credential Source-Proof Boundary

Root's bounded source-proof intake, [`📓️coordinator-hub-credential-source-proof-intake-2026-09-13.md`](./📓️coordinator-hub-credential-source-proof-intake-2026-09-13.md), establishes two **test-oracle defects**, not native credential or child-process failures. It transpiled and invoked only the two helper bodies under `node:vm`; it did not start a Hub, native binary, MCP executable, listener, or child process.

`proveNativeCredentialSourceOrder` still reads the removed WGPU Rust-package command coordinate. Replacing that string only in memory reaches a second stale condition requiring `runCmdStatus(nativeBinaryPath(ship))`. The current semantic chain is the WGPU native-entrypoint's `runNativeSession` → `runNativeBinary` → `runTool`, so the future proof must parse/assert that actual call chain and its direct owner/input graph. A fixed old pathname or token check would not validate current supervision.

`proveMcpCredentialSourceOrder` builds the same current MCP command pathname twice and then demands exactly one result. With the duplicate removed only in memory, its `indexOf("parse_args()")` compares the credential claim with the Rust `fn parse_args` declaration at line 93, rather than the execution call in `main` at line 188. The actual `claim_inherited_local_hub_credential("mcp")` occurs before that main-body parse call. The repair therefore needs one authoritative runner selection and a syntax-aware function/body-order oracle with positive and hostile cases; it must not perform a blind token substitution or turn this current source-proof result into a native security claim.

These two helpers belong to verification concerns associated with the current WGPU/MCP process owners, outside the proposed bounded Hub foundation behavior owners. The Hub source gate must retain their source-data/registration relationship where it invokes them, while native child-pipe evidence stays separately labeled if and when it runs.

## Owner-Plan Review

I reviewed [`📓️sol-hub-foundations-owner-plan-2026-09-13.md`](./📓️sol-hub-foundations-owner-plan-2026-09-13.md). **No structural blocker found.** Its ten-owner split preserves the mandatory concern boundaries: Cargo environment policy; fixture vocabulary; framing; authenticated framing; local process lifecycle; issuance; direct-child delivery; ordered directory verification; source-order verification; and route-only ownership-test execution. The seven proposed edges are one-way and preserve the important execution/issuance type boundary without adding a package/root facade. Retaining the GIS checkpoint maximum in the existing inference-schema authority is correct.

The credential source-order verifier needs one explicit semantic exception: the MCP Rust `main` performs a `schemas` subcommand preflight before it claims a credential, then returns without filesystem/network/credential handling. The new syntax-aware source oracle should model that exact pure schema-export path and separately prove that, for serving paths, `claim_inherited_local_hub_credential("mcp")` precedes the `main`-body `parse_args()` invocation and workspace activation. It must not use a global argv-token ordering predicate that would mistake this declared preflight for a credential-flow violation.

Before green acceptance, the schema-first source contract still needs exact accounting for every reader of `exactCargoStageEnvironments`, every moved declaration caller, the current WGPU/MCP source populations, the local/auth/directory schema and fixture inputs, and package/target/seed/derived registration. The new portable verifier must isolate all allocation/child behavior; neither its injected process boundaries nor selected local-bootstrap Rust laws may be reported as a Hub service, MCP child, or native WGPU security execution.

## Current Static Review — Rust Boundary Parity Awaiting Execution

The live foundation schema/fixture and ten anonymous owner paths now materialize the planned one-way import graph, direct package route, named input, and seed/derived launch records. The portable test independently checks its lexical `sourceDefinitionBodies` helper against a TypeScript AST. The earlier missing Rust parity is now materialized as the test-only `🌎️hub/🔐️auth/🧪️tests/🧭️credential-source-order/🔮️oracles/🦀️.rs`, mounted only under `#[cfg(test)]` in the Hub library and using the `syn` dev dependency.

Static review confirms that the selected `hub_credential_source_order_syn_parity` law parses the shared raw-string/comment hostile from the schema fixture, then independently parses current MCP and WGPU entrypoints. It permits the exact MCP schema-export preflight, proves serving-call order, and selects one credential-bearing WGPU `main` from the two current `main` functions. The oracle and its Hub library mount are in the named input closure. The updated execution result is recorded above; it does not require starting a Hub, child, browser, or service.
