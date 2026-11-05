# Remodel Build Readiness Audit — 2026-11-04

Read-only production audit through repo commands. Read the existing audit first. No production source, contracts, AGENTS files, or git state changed.

## Source Admission Diagnosis

The engine verifier reported composer-api/🦀️.rs in semio-framework-plugin-instance-app: observedBytes 18870, maximumBytes 17528, remainingFiles 17, expectedBytes null.

The matching caller is readRustUnitSourceClosureV1 in 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️inputs/🟦️.ts. It uses min(boundary.maxSourceBytes, unit.limits.maximumSourceBytes), subtracts bytes already read, and credits only a repeated probe its previous extent. Its readPhysicalCargoSourceV1 call has expectedBytes:null and remainingFiles matching the report.

Therefore 17528 is remaining aggregate crate source credit, not evidence of a stale individual source extent. The observed file alone exceeds the remaining budget by 1342 bytes before any later files. The owning instance Cargo.toml declares maxSourceBytes = 262144. A smaller descriptor limit is also possible; either way blindly refreshing observations or raising a per-file cap does not establish a valid repair.

The legitimate remedy is to inspect the selected unit source roster and split the oversized instance-app source closure into domain-owned crates, or deliberately review its authored finite source contract with the infrastructure owner. Do not bypass ingress, disable observation, manipulate cached receipts, or shrink safety validation. A normal rerun can distinguish concurrent edits, but cannot repair a stable aggregate overrun.

## Declared Preparation and Integration

✏️s/🧑‍💻dev/📋️project.json declares generate, verify-catalog, generate-playground-session, and check-playground-session exactly as listed in the prior audit. generate depends on generate-stdio-format-claims, publishes the generated catalog directory, and explicitly marks sourceInputObservation freshness. The catalog publication router receives its original process invocation and delegates to framework GenerateScript/CheckScript using specificCatalogPublicationV1. The playground session router parses its owning sessionSchema through parsePlaygroundSessionPublicationRequestV1 before using framework publication producers. These are legitimate schema-owned routes; direct execution outside the repo ingress is not an appropriate workaround.

generate-playground-session and check-playground-session have no declared dependsOn generate; callers must perform catalog generation first as documented by the existing audit. Their required generated session output is ✏️s/🧑‍💻dev/🤖️generated/🎮️playground-session/🟦️.ts.

## Attempted Verification and Limits

Attempted repo run @semio-tech/s-dev-composition:verify-catalog --dry-run. It remained in the normal capacity queue: request 5223 MB, only 319–1585 MB spare above the 4096 MB floor, with existing native and JavaScript verification ahead. Cancelled that redundant queued request without starting task execution. No command success is claimed.

The root verifier separately reported full graph source scans and shared dependency preparation. No minimal graph refresh was demonstrated safe or effective. Existing owned preparation commands remain the correct route after capacity and the finite native source contract are resolved.

## Scoped Native Compilation Suggestion

The engine manifest at ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📦️packages/🦀️rust/Cargo.toml places semio-framework and editor-kit under dev-dependencies. The ordinary engine dependency list does not directly name plugin-instance-app. Therefore the narrow owned command is repo cargo --store=shared --configuration=prototype -- check --manifest-path ✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/Cargo.toml -p semio-s-artifact-remodel-remodeling-engine --lib. This excludes test compilation; its transitive ordinary dependency closure still needs execution to prove independence. A wasm32 target alone does not exclude dev-dependencies if tests are selected. No native or WASM success claimed here. Framework budget changes or source splits are outside this read-only audit and were not performed.
