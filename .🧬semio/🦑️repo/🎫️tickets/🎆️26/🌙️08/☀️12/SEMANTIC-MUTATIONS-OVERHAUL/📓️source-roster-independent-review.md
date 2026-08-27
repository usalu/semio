# Source Roster Independent Review

## Accepted Boundary

The lower const validator compares a complete descriptor and source provenance against independently supplied aggregate scope. The scope contains the physical-workspace token, mutation root, taxonomy locator, and canonical filenames obtained from taxonomy. Validation requires an immediate owned child, exact provenance equality, all 32 token bytes, safe normalized paths, and exact owner/filename joins. It allocates nothing and adds no runtime dependency or default.

The initial implementation hardcoded current Rust/JSON filenames. Review rejected that second naming authority: the corrected validator validates safe single components and compares against the supplied authoritative filenames. Historical paths fail under the actual current scope; a test authority with different safe filenames and matching provenance passes. Case-insensitive excluded components are rejected consistently with compiler source authority.

## Executed Evidence

The corrected registered lower library build passed with exit 0 in 2.28 seconds (`🧪️source-roster-lower-build-final.log`). The coordinator's actual registered test retry passed 3 metadata tests, 215 filtered, exit 0 (`🧪️source-roster-lower-registered-retry.log`). The earlier test attempt failed before selection on an unrelated transport-test temporary-borrow error; that file was not modified by this task, and its changed source allowed the retry to compile.

The root independently reran the full compiler/Ajv-backed oracle through workspace-scoped Nx. `🧪️source-roster-contract/🧪️root-final.log` and run `🧫️run-GDf9K8` record 58 matching validation/runtime cases plus an invalid const-acceptance guard rejected with `E0080`, with 0 failures and exit 0. Every workspace-token byte is independently changed. The real protocol pair is `9326ffd3ad988ba0` and serde pair `9726de5488b8f586`, with both rlib/rmeta forms supplied. Sources, argv, compiler/runtime outputs, and results are retained per case.

The executor's earlier red run `🧫️run-tJAe23` is retained: it exposed Rust literal encoding of NUL in the harness and case-folded root rejection ordering. Neither was hidden by altering expected outcomes. The corrected source and harness were rebuilt before the root's passing replay.

This accepts the const source validator, not an active aggregate enforcement path, registry identity, source-policy authority, or production mutation conversion. The mandatory aggregate transaction must consume it and prove actual wrong-root/foreign/manual-provider rejection.

## Corrected Oracle Assertion Gate

Final harness inspection found that negative provenance branches returned before asserting the computed Ajv rejection. The 58 Rust/reference outcomes and const guard above remain executed evidence, but complete independent Ajv outcome parity is not yet established. The executor is adding schema constraints for scope/path safety plus exact provenance and asserting the Ajv boolean for every vector, including invalid scopes, before the packet's oracle gate can close. No production validator change is requested by this correction.

The corrected harness now asserts the full Ajv boolean before any negative branch. The coordinator independently replayed it through workspace-scoped Nx: `🧪️source-roster-contract/🧪️root-ajv-final.log`, run `🧫️run-u9pQaM`, exit 0. All 58 schema/reference/runtime results agree, and the invalid-provenance const guard fails with `E0080`. Scope path safety, the immediate descriptor owner, all six provenance values, and every workspace-token byte are included. This closes the bounded oracle gate without any production validator change.
