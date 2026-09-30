# Shared Directory Policy and Plugin Publication Contracts

The renderer's document-opening code imported the hub access-policy implementation and policy data. The framework plugin client imported the hub's trusted-catalog schema. Both are inverted ownership edges: shared contracts belong to the framework, and the concrete hub consumes them.

## Directory Policy

The policy schema, declared data, TypeScript parser/evaluator, Rust types/evaluator, and decision fixture now belong to `💻️os/🔨️modules/📇️directory/🛡️access-policy`. Public types are `DirectoryAccess*`; the wire document is `semio.os.directory.access-policy/v1`. Hub-owned exports, implementations and tests at the old paths were removed. Hub authorization and the renderer consume the same owner directly, without compatibility forwarding exports.

Before implementation, the new language-neutral policy cases and Ajv test were added. Nx failed with the expected missing shared parser module. The implemented strict parser rejects unknown fields, old schemas, unknown choices, empty grants and duplicate role/action/kind choices. Rust parsing now enforces the same uniqueness constraint as JSON Schema.

Executed checks:

- `@semio-tech/framework-os:test quick --testNamePattern='shared directory access policy'`: 3 tests passed; 8 policy cases and 160 decision vectors, including reversed grant order. Ajv independently validates the same policy cases.
- `@semio-tech/framework-os-kernel:canonical-architecture`: 2 Rust tests passed; runtime output confirms 7 additional policy cases and 160 decisions. Existing unrelated Rust warnings remain visible.

The TypeScript and native owning projects contribute their checks to `canonical-architecture`.

## Plugin Publication

The shared trusted plugin manifest/index schema and TypeScript/Rust types now belong to the existing registry deployment schema owner. The browser installation schema retains only its local persisted records; shared functions and types are imported from their canonical owner. Hub catalog publication retains its server-specific contracts and imports shared types directly. The neutral manifest/index fixture was moved to deployment, and canonical wire documents use `semio.os.plugin-module*`.

The existing deployment owner already contained catalog and route definitions. An initial extraction overwrote those definitions; they were restored and merged into the schema before proceeding. Its existing `component.json` schema identifier is preserved. This correction is covered by the deployment consumers and plugin module regression suite. The fixture's canonical digests and lengths were recomputed after the wire identifier change; no migration or alternate API remains.

`@semio-tech/framework-os:canonical-architecture` passed: 4 test files, 74 tests. Runtime results cover the policy parser/evaluator, plugin bundle validation and canonical content hashes, program resolution, cache installation, serving and collection. Ajv validates the same neutral publication schema. `os-hub:test -- --lib plugin_module -- --nocapture` passed all 7 native laws: canonical bytes and SHA256/BLAKE3, manifest/index acceptance cases, content tampering refusals, file serving/media types and bounded bundle loading. The initial BLAKE3 mismatch identified an authored fixture expectation missed after changing the wire identity; its corrected value agrees independently with both native and TypeScript hashing.

## Root Verification

`verify canonical-architecture` runs the Nx targets contributed by owners. The normal gate runs the same suite. `verify layering` now invokes the strict resolved dependency graph check rather than the obsolete textual reference-count baseline. That baseline counted comments, fixtures and schema inventories as coupling while missing package aliases; it was removed, including its writer and old public APIs.

The first plugin-registry generator run completed and regenerated `launch.json`. Final regeneration will include every later contributed command. Full-tree semantic checks currently surface remaining real ownership violations; they are not waived or counted as passes.

## Mutation Publication Contribution

The expanded `@semio-tech/framework-os-kernel:canonical-architecture` passed 12 exact native laws. They cover shared policy, the existing language-neutral publication fixture and independent JSON oracle, O(1) immutable artifact/presence/transient roots, publication of one and 200 mutations, cancellation without publication, forged cursor/digest refusal, stale/saturated admission and exact owner checks. The exact-law runner resolves named tests and records their runtime receipts, avoiding a zero-test filter pass. This verifies the inspected live store boundary; it does not assert that every artifact implementation across the repository has been exhaustively checked.

The schema check caught invalid internal `$defs` names and implicit claims that existing deployment JSON-only contracts also had TypeScript/Rust implementations. The affected new owners now use PascalCase export identities and explicit `x-semio-formats`, including JSON-only fixture/helper contracts. A thirteenth native law checks shared plugin schema registration against all nine JSON exports and their exact supported formats. The final expanded target passed all 13 exact native laws, including that registration law; runtime receipt logs record each resolved test and its success.

Global schema generation succeeded with 3,538 scopes. Global schema check remains failing on thousands of broader schema/placement findings; none are suppressed by a baseline. Final touched-owner review and refreshed inventory checks are running. The broader schema findings remain explicit follow-up work.

## Final Inventory and Aggregate Checkpoint

After refreshed schema generation/docs, global schema check reports 9,360 broader findings. Filtering the actual check output by the newly changed IO, reconcile, broker, provider, policy, deployment and dev-contribution schema owners yields zero findings; the larger catalog remains failing.

The root aggregate executed 14 owner contributions: 13 passed and CAD failed on a dispatcher-capture structural law. This failure is being investigated against the actual helper/stage implementation, not exempted. Final read-only audit found Rust optional policy fields accepting explicit null, provider profile grammar drift/missing child-spawn error handling, and graph closure validation gaps; regression repairs and the new aggregate are pending.

## Audit Regression Repair: Explicit Null

Added the language-neutral `null-space-kinds` case before changing Rust. The native canonical target failed on that exact case (`left: true`, `right: false`), reproducing the audit finding. Present `spaceKinds` now deserializes as a non-null array; omission still has its declared optional meaning. No public interface or compatibility branch was added.

After repair, `@semio-tech/framework-os-kernel:canonical-architecture` passed all 13 exact native laws (27.6 seconds); `@semio-tech/framework-os:canonical-architecture` passed all 74 TypeScript tests (3.1 seconds). Both consume the same neutral policy fixture, with Ajv validating the schema independently.
