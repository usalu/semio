# Framework Import Enforcement Independent Audit

Complete current sources for 15 exact route/rule/corpus/General package inputs are retained with SHA-256 in framework-import-enforcement-audit-inputs/current-full-source.json. No source changes, Cargo invocation or full gate execution occurred.

## Existing Permanent Route

Root workspace verify-layering calls root script.ts verify layering. runLayering at line 8280 invokes repo-lib lint-dependency-direction, lint-rust-source-direction and lint-cargo-dependency-direction through Nx. Root verify-canonical-architecture at line 8285 invokes owner canonical-architecture targets. Repo-lib canonical-architecture calls its script.ts test canonical-architecture; lines 227–237 run source/corpus tests and then actual JS, Rust and Cargo verifiers. Launcher contains both root entries around lines 5747–5761. Dedicated framework module product direction at line 7657 calls repo-lib lint-framework-module-product-direction, which checks the framework-modules JS source role.

JS policy is taxonomy-owned. The lint dependency-boundaries config loads canonical bootstrap; pure construction builds framework-no-implementation plus declared role rules. Existing corpus/schema/tests are repo-lib fixtures/schema/tests under dependency-direction. Cargo direction independently admits authored manifests versus metadata and reports normal/dev/build/target aliases separately; its execution requires Cargo and was not invoked here.

## Concrete Executed Future Enforcement Cut

Current taxonomy dependencyDirections.roles.framework-modules.ownerPaths contains only `🧰️framework/🔨️modules`, excluding neutral General package glue under `🧰️framework/📦️packages`. Thus framework-modules-no-products misses a direct General TypeScript package reexport into an OS product package. framework-no-implementation also misses it because both endpoints share the physical framework area.

Independent Bun evaluation executed the actual pure policy builder with current complete taxonomy and Node builtin list. Proposed hostile source `🧰️framework/📦️packages/🟦️typescript/🟦️.ts` to target `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts` matched zero forbidden rules. Changing only source to neutral Replication module matched framework-modules-no-products. workspacePackages was empty in this bounded construction probe, so evidence covers exact resolved physical paths, not installed alias resolution or full dependency-cruiser parsing.

Executable proposed hostile source in General TS glue:

```ts
export * from "../../🛍️products/💻️os/📦️packages/🟦️typescript/🟦️.ts";
```

Add it to the existing closed dependency-direction corpus as a static export case with expected framework-modules-no-products rule. Exercise existing independent dependency-cruiser oracle and rule matching before fixing. Minimal owned fix: extend neutral role ownerPaths with `🧰️framework/📦️packages`, or introduce a schema-authorized General neutral role and reuse the existing no-products rule. Update scoped source inventory semantics so the dedicated route includes neutral package glue. Preserve installed package/subpath hostile variants. No new runtime external package is needed.

This edge is prospective: the inspected current General TS entry contains neutral exports and does not currently contain that reexport. No actual present General→s/hub/plugin/artifact hard edge demonstrably missed by the current Cargo/JS gate was found in this bounded slice. Do not misreport the prospective reproducer as existing production behavior. It establishes a concrete missing acceptance boundary with executed selector evidence.

## Existing Rust Edge Is Already Rejected

General Rust Cargo.toml line 45 still declares normal semio-framework-os-kernel directly. Current cargo-framework-no-products explicitly selects neutral modules and package glue and targets any products owner segment. This is already a strict Cargo violation, not missed enforcement. Physical framework-no-implementation likewise rejects framework→s/hub top-level edges regardless of normal/dev/build declaration kind. Exported macro resolution belongs to separate Rust source graph admission; this audit executed none.

## Limits

No registered gate, dependency-cruiser graph, native compiler, deletion or macro expansion was executed. Source-selector hole is established; full alias/import resolution and fixed-gate RED/GREEN remain future work. Refresh complete source authority before publication because concurrent work can change inputs.
