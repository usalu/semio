# Public API Canonicalization Inventory — 2026-10-14

## Current Admitted Scope

The current framework-neutral gate admitted 2399 source modules and 7419 dependencies under 1569 exact strict rules. It refused 1067 rule-qualified findings across 753 distinct source/target dependencies. This is a followed neutral-source closure, not the full workspace census. No present graph unresolved-read exception was required.

## Most Consumed Private Targets

| Target | Distinct Consumers |
| --- | ---: |
| 🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts | 30 |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/🔢️numeric/🟦️.ts | 21 |
| 🧰️framework/🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts | 20 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx | 20 |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🔣️.json | 17 |
| 🧰️framework/🔨️modules/🌱️value/⚠️refusal/🟦️.ts | 16 |
| 🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🟦️.ts | 13 |
| 🧰️framework/🔨️modules/🌱️value/💾️resident/🟦️.ts | 8 |
| 🧰️framework/🔨️modules/🔏️hash/🟦️.ts | 7 |
| 🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts | 7 |
| 🧰️framework/🔨️modules/🎠️kernel/🟦️.ts | 7 |
| 🧰️framework/🛍️products/💻️os/🟦️.ts | 7 |
| 🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts | 7 |
| 🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json | 7 |
| 🧰️framework/🔨️modules/🛂️manifest/🟦️.ts | 6 |
| 🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts | 6 |
| 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🩹️patch/🟦️.ts | 6 |
| 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts | 6 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts | 6 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts | 6 |
| 🧰️framework/🔨️modules/🌱️value/🧬️schema/🌳️intrinsic/🟦️.ts | 6 |
| 🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts | 5 |
| 🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️current/🟦️.ts | 5 |
| 🧰️framework/🔨️modules/🪪️identity/📁️installation/🟦️.ts | 5 |
| 🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts | 5 |
| 🧰️framework/🔨️modules/⚠️diagnostic/🚧️text-error/🟦️.ts | 5 |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts | 5 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts | 5 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts | 5 |
| 🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🛠️build-tooling/🟦️.ts | 5 |

## Value Ownership

The Value owner currently has a Rust package manifest with no TypeScript export authority; it contains many intrinsic TypeScript contracts and behavioral ports. A correct public surface needs explicit owner-authored TypeScript facets, through actual package export targets, and framework SDK facets for plugin consumers. Publishing arbitrary private descendants or fixtures through wildcards is not a clean interface. Migrate concrete consumers after portable export/type/behavior tests, then rerun the actual graph.

The policy constructor already unions export targets from language packages that share one semantic source owner; repeated language package findings do not mean it rejects another language package public target. Do not weaken that policy or infer a bug from duplicate rule names.

## Next Execution Slice: Numeric Value Interface

The current numeric index already owns private constructors for index, edit and reader custody; the public operations use first-party bounded grants, typed steps and explicit retirement. Its twenty-one cross-owner TypeScript consumers currently address implementation paths. Publish a Value-owned TypeScript package with an exact `./ordered/numeric` export target and all four runtime classes plus four existing grant/step/ordinal types explicitly named. Add the corresponding framework SDK facet for plugin consumers. Repoint actual General UI, OS document store, framework SDK and verification consumers to the authored owner facet. Keep same-owner implementation tests beside their source. This is public authority for a deliberate bounded interface, not a wildcard opening of the Value tree.

Before production changes, add a closed portable contract for the public export keys and finite numeric operations. Require actual owner/SDK loading, identical ownership-preserving behavior with the existing third-party numeric oracle, declaration/type consumption and a bundler closure that survives removal of specific roots. Register the owned source receiver through its package script, Nx and both launch assets. Re-run the actual admitted dependency-direction graph after migrating consumers; keep every remaining public/private finding as an error. Full Value refusal/resident/wire/schema facets remain subsequent work.

Numeric and scalar execution evidence is retained in the numeric-api-results and scalar-api-results reports. The latest admitted scalar-stage graph has2421modules/7500dependencies/1570strict rules and1109rule-qualified violations across716distinct forbidden dependencies. Original numeric/refusal/IEEE implementation targets and their authored public facets have zero violations. Actual broad consumer strict closure still fails; scoped owned public declarations and source laws pass. No broader green claim or rule exception is made.

The one current framework-no-products edge is a newly present DSL record derive Rust command importing Repository exact Cargo orchestration. Cut that actual executable dependency through the existing General configuredExactCargoLawPreparationPortV1 and runExactCargoLaws interface. General selects the actual two owned manifests and retains the original build/test budgets, target directory, compiler environment, invocation and cancellation. The Repository caller continues to author preparation through its supported native owner-command; no product discovery or default preparation can move into General. Add an actual script closure removal law before editing production, reuse the existing ownership command registration, and qualify strict/source behavior. Native optional-custody execution needs the original provider receipt and sufficient storage; do not infer it from source bundling.

The derive-command-boundary report now records genuine red/green removal proof, actual6/52 ownership laws, strict original router/law and37/752 exact preparation laws. Current admitted graph:2430modules/7528dependencies/1570strict rules,1108rule-qualified findings/715distinct forbidden dependencies, zero framework-no-products and zero published Value facet findings. The whole gate still fails correctly. Resident/wire/schema public authority and current consumer type/API debt remain subsequent slices; native host application and concrete composition acceptance remain open.

## Follow-On Work

## Next Execution Slice: Refusal and IEEE Value Interfaces

Publish two explicit Value-owned TypeScript facets for the existing original typed refusal and exact IEEE word contracts. The refusal facet must explicitly name ValueError, valueRefusalKindFromWire, ValueRefusalKind and ValueErrorRetainedProgress; decoration preserves typed kind and the original physical receipt, and prose cannot classify a refusal. The IEEE facet names binary32/binary64 capture, parsing and numeric projection plus their two first-party word interfaces. Publish the corresponding General SDK facets and manually repoint each actual cross-owner source specifier; keep Value implementation and intrinsic tests adjacent.

Add a closed language-neutral public-interface corpus before production edits. Validate actual package export resolution, exact runtime identity through both facets, emitted first-party declarations and General-only bundled source closure. Refusal behavior uses an independent SQLite path projection; IEEE word projection and capture use the installed ieee754 test oracle. Keep all present architecture rules closed and fail remaining private dependencies. No compatibility barrel, arbitrary wildcard, copied implementation or new runtime dependency is authorized by this slice.

- Canonicalize Value/Diagnostic/Pack error public interfaces and real consumers, preserving typed original failure authority.
- Move remaining root-owned generic test imports behind present owner command contributions.
- Rehome OS typed snapshot / Draw renderer / Procedural editable-control integration tests to the concrete composition owners, while retaining neutral host contract laws.
- Finish catalog publication and native Jack content binding; actual application compiler/build proof is required beyond neutral fixture packages.
