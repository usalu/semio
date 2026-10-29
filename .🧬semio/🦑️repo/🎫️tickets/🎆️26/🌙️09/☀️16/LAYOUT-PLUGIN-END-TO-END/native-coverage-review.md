# Native Coverage and Source Admission Review

2026-10-28. Coordinated directly with Rust owner. No compiler/build/test/graph run. Canonical readRustSourceClosureV1 audits only, through repo managed Bun and explicit workspace cwd.

## Actual Combined Selection

Rust owner confirmed one native invocation from Layout artifact Cargo.toml, repeated -p for the current router testPackages, --lib -- --nocapture, with no explicit --features flag. Cargo dev-dependency feature unification activates testkit on editor/engine/export/model and related schema owners. The router now selects14unique packages, including the private editor owner in addition to its behavioral companion. Earlier native-test-inventory.md reports13and lacks that final addition, so it is historical evidence rather than the current final roster.

Fresh canonical selected-owner audit uses cfg(test)+feature=testkit with macOS ARM64/unix cfg, conservatively covering the combined local testkit projection for every selected owner. All14resolve and all fit unchanged32file/262144byte/4000active-line bounds:

| Owner Suffix | Files | Bytes | Active Lines |
| --- | ---: | ---: | ---: |
| -sqlite-member-tests | 3 | 110415 | 39 |
| -mutations-content-tests-2 | 17 | 102244 | 1401 |
| -mutations-content-tests-1 | 18 | 109945 | 1522 |
| -mutations-frames-tests-2 | 15 | 125983 | 1673 |
| -mutations-frames-tests-1 | 16 | 131027 | 1745 |
| -diff-plan | 15 | 77398 | 288 |
| -editor-tests | 4 | 79420 | 726 |
| -editor-commands-tests | 17 | 128532 | 1613 |
| -editor | 22 | 255749 | 3145 |
| -engine-scene-tests | 23 | 218357 | 858 |
| -engine-flow-tests | 7 | 179614 | 97 |
| -engine | 22 | 256515 | 2276 |
| -engine-export-unit-tests | 9 | 107443 | 1148 |
| (root) | 3 | 19898 | 276 |

The engine owner has5629bytes headroom; editor has6395bytes. Existing input bytes, including embedded fixtures, count through the canonical interface. Actual manifests may change concurrently, so these are fresh source measurements rather than immutable compiler receipts.

A second conservative audit of all34Layout normal library owners with feature=testkit and without cfg(test) found0failures. This covers the local dependency projections activated by companion features, including model, engine-export, diff and mutations. Tight normal owners remain IO32files/253706bytes, mutations30/253539, engine20/249092 and engine-export22/245992. This does not inventory all transitive shared-framework crates or establish compiler correctness; the Rust owner retains that one native compiler job.

## Private Behavioral Coverage and Ownership

The selected editor library itself includes args_bridge and its cfg(test) module preflight_host_payloads_keep_the_issue_target. Its neutral JSON fixture is in the canonical source closure. That actual test compares native JSON decoding with serde_json, checks nested/flat issue targets and malformed refusals. The private root also owns transform-tool-transactions. The editor-tests companion mounts cases.rs, while the editor root mounts only shared context support; it does not remount those companion behavioral cases. Thus adding editor is necessary to execute private args-bridge tests and does not duplicate the main behavioral suite.

The selected engine library includes the actual text-fonts native tests: neutral_catalog_is_available_without_registration, package_sources_preserve_role_order_without_registration, missing_native_assets_can_arrive_later and export_catalog_freezes_first_success_across_late_installation. The same test source is absent from the scene/flow companions. This selection retains the engine font laws rather than assuming scene companion ownership covers them.

Compared all reachable test Rust paths across14selected owners. The only repeated path is export/unit/pdf-structure, shared by editor-commands-tests and export-unit-tests. It defines PDF oracle helper functions only and contains no test attributes; it is an intentional shared test oracle, not duplicated test execution ownership. No shared executable test file was found. Shared fixtures/support sources can legitimately appear in more than one test closure.

## Retained Audit Inputs and Result

native-coverage-inventory.ts derives the exact current router roster and canonical closures; native-coverage-data.md records all14rows and input paths. native-coverage-dependencies.ts and native-coverage-dependencies.md records the all34normal testkit audit. They are read-only source tools derived from the existing bounded inventory tooling and remain in this ticket. No checked-in implementation source was copied into the ticket.

No correction requested. Private args bridge and engine font laws are included, package test ownership is coherent, and current local source admission fits the unchanged bounds. Actual native execution/pass status remains Rust-owned and is not inferred from this inventory.

## EnvelopeJobs Bound Follow-up

Read-only inspection confirms EnvelopeJobs contains M only in ActiveArtifactStoreReplacement<A::Snapshot,A::Mutation,M>. Its actual declared M bound is SpaceMember+MemberFactory; other fields are independent of M. EnvelopeJobs has no Drop implementation. Its birth_demands/new/from_original methods allocate or move owned registries, while acknowledge/cancel methods operate on ActiveDocumentArchiveLoad independent of M. No method invokes a ReferenceSpaceMember operation. Therefore relaxing the container and this constructor/common-method impl from ReferenceSpaceMember to SpaceMember+MemberFactory matches its actual field/API requirements; no method-local reference constraint is missing in this inspected impl. No code/test changes were made by the reviewer.
