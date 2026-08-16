# Wave 0 R1-A Plugin Remediation

## Landed

- Replaced synthesized capability mutation with immutable, exact category-and-full-claim validation. Runtime facets now require declared schema, inference, codec, grammar, composer, subset-validator, and plural representation rows; malformed standard codec/mutation/inference identities require `vN`.
- Added public declaration capability inventory rows through `runtime_capability_requirements()` on ready builders and built declarations.
- Replaced cold inference callbacks with bounded request/execution types. Wire and WIT results echo policy, budgets, previous state, cache mode, and dependencies; requests enforce finite budget, cache policy, dependency identities, result budget, and cancellation.
- Replaced the host IO router's independent maps with one lock-protected state, typed route/runtime conflicts, and no overwrite or poisoned-lock fallback.
- Replaced sequential plugin registration with a frozen `PluginRuntimeRegistry` for definitions, schemas, app schemas, languages, inference services, and contribution mutation rows. `PluginBuilder` performs exactly one external IO/store aggregate commit through `ArtifactAssemblyRegistryPlan`; no fallible operation follows it.

## Evidence

- `cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml artifact_inference_wire_tests --lib -- --test-threads=1` passed: 3 passed, 0 failed. This compiles the final plugin library boundary and covers full echo plus mid-flight cancellation.
- `cargo test --manifest-path 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml artifact_inference_router_tests --lib -- --test-threads=1` passed: 1 passed, 0 failed. This verifies that the host only publishes exactly echoed guest results.

## Coordination

- R1-B supplies `store::begin_artifact_assembly` and `io::commit_artifact_assembly_registry_plan`; this lane calls the aggregate once as the final builder operation.
- R1-C is consuming the declaration inventory to materialize every stdio runtime capability and is adapting GLTF inference call sites to the request-aware API.

## Outstanding

None in the R1-A lease. R1-C's stdio capability materialization and GLTF call-site migration remain its independent integration lane.
