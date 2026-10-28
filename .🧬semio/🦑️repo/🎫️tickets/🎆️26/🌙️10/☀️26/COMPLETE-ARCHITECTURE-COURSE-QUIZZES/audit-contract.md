# Independent Contract Parity Audit

Read root AGENTS.md and applicable products AGENTS.md. All inspection and verification used repo tools exec; no source, Git state or worktree was modified.

## Finding

P2: Positive shared-course query remains on version 8 outside the course product. File 🧰️framework/🛍️products/🖥️server/🧪️tests/🧫️fixtures/🔌️wire/🔣️.json:688 defines course-root-query-envelope version 8. File 🧰️framework/🛍️products/🖥️server/🧪️tests/🔬️wire/🟦️.ts:67 validates that positive vector with AJV const 8. This is not a deliberate bad-version case. Both should track course WireVersion 9. Sent to parent for integration.

## Confirmed Parity

JSON schema and TypeScript/Rust models agree on optional Mockup for classification, sorting, matching and SheetItem. Widget variants agree on parameter, component, bounded numeric slider, localized panel lines and localized plugin label. Rust serde uses camelCase for Excel fields and optional bounds, tagged kebab-case unions, strict unknown-field rejection and omitted absent options. Rust module and public crate reexports include MockupWidget and other new types.

Both domain validators enforce structural bounds, localized values, grid rectangularity and selection bounds, unique node/port identities, output-to-input wire references, tree paths and highlight references. Widget slider range and value semantics match. The schema intentionally permits structurally valid dangling references and range inconsistencies that domain validation rejects; fixture schemaValid and valid flags distinguish these layers.

Both sheet implementations retain given mockups for all three task kinds while selecting only presentation fields. Known Rust struct literal callsites under course challenge, validation, scoring and companion-support explicitly supply mockup: None. No omitted new field was found in the bounded course source walk. No stale literal version 8 or affected item struct literals were found in the bounded teaching proctor source walk.

WireVersion is 9 in JSON schema, TypeScript alias/constant and Rust constant. Independently recomputed canonical prose-free FNV-1a64 equals a517a07210e9b0ec, exactly matching the neutral wire-version fixture.

Durable TypeScript tests compare the same neutral mockup vectors with independent AJV and owned validation for all three item kinds, then check projections across all four challenges. Durable Rust test software_mockup_vectors_agree_with_every_item_projection exists in validation/🧪️tests/🔬️unit/🦀️.rs; its include_str path resolves to the same neutral fixture. It deserializes/validates vectors and checks three projected items at all challenges, preserved mockup JSON and absence of category/value/values/explanation/familiar. An earlier preliminary message overlooked this Rust unit-test location; corrected here.

## Executed Verification and Limits

repo tools exec bun -- ticket/verify-mockups.ts passed: 21 vectors, 168 projected items, 14 generic localized renders and 390 authored localized renders. Canonical fingerprint was independently recomputed with Node through repo.

No official broad build or Cargo gate retried: parent reports infrastructure failure before test execution at missing .cache/b/l/fixed.dir. Rust compilation and execution are therefore unverified; source inspection does not substitute for compilation. Cross-language test sources are present, but only TypeScript direct verification ran here. Searches were bounded and dependency/cache/generated folders were pruned. This is a source and contract audit, not browser or pixel-fidelity verification.
