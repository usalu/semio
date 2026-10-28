# IFC4 Complex Entity Editing

The IFC4 snapshot and sparse entity diff preserve complex instance constituents, but the mutation vocabulary and editor rules cannot change them. This slice adds SetEntityComplex across the schema, Rust and TypeScript contracts, text and binary codecs, editor routing, mutation catalogs and language-neutral fixtures. The existing IFC4 artifact JSON schema incorrectly exposes a text field; align it with the persisted header/entity model and constrain recursive IFC values.

Shared ownership: main owns IFC4 diff algebra; this slice adds only the complex-field diff builder. IFC4x3 files are owned by another agent and remain outside this slice.

Validation will use committed fixtures, mutation inverse/diff/codec tests and existing independent Part-21 readers. Test execution must be routed through the repo interface; its portable CLI is still being located by main.

## Implementation Progress

Authored the language-neutral JSON fixture and Rust regression tests before the production leaf. Added the complete SetEntityComplex mutation path, editor pointer rule, typed TypeScript union, grammar/wire tag, and independent ruststep reader projection including complex constituents. The artifact/snapshot schemas now reflect header/entities and recursive tagged values.

Repository test execution currently stops before IFC: the routed native-build cache allocation cannot enable Windows per-directory case sensitivity (access denied). Build infrastructure is owned by other agents and remains untouched. Static contract validation and direct JavaScript schema evaluation will supplement the authored Rust tests.

## Validation

AJV 8.20.0 independently evaluated the IFC4 snapshot, artifact and aggregate mutation schemas against the committed complex-edit fixture: all four positive checks passed; six malformed recursive value variants were rejected by both snapshot and mutation schemas; clearing constituents with an empty list passed; missing entity ID and the stale text artifact shape were rejected. Catalog/wire tag/TypeScript registration checks passed for ten mutation kinds. The reusable input script is `ifc4-complex-check.mjs` in this ticket.

The direct JavaScript evaluation initially exposed a Node VM cross-realm structuredClone/prototype assertion mismatch; the check now clones through JSON, matching its language-neutral fixture domain, and passes. This did not require changing production code or fixtures.

## Changed Files

- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️set-entity-complex/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️set-entity-complex/🧬️schema/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️set-entity-complex/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧩️set-entity-complex/🧪️tests/🔁️replaces/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🔺️diff/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🔺️diff/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/📖️.grammar.semio`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/📡️.protocol.semio`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧭️edit-rules/🧪️tests/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🔮️oracles/🧪️tests/🔬️unit/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🏗️mutate-ifc-4/🥒️.feature`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/🎯️outcome/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/📸️snapshot/➡️after/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/📸️snapshot/⬅️before/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/🔺️diff/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🧩️set-entity-complex/🔁️replaces/🦠️mutation/🔣️.json`
- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-complex.md`
- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-complex-check.mjs`

The longest new fixture path is 189 UTF-16 code units, below the 256-character path limit.

## Empty IFC4 Export Conformance

A new source regression asserts that a default empty snapshot declares FILE_SCHEMA((IFC4)), exports it, and reconstructs the same snapshot after parsing. IfcHeader::default now retains the conformant minimum generic header records while setting its own IFC4 schema record. This prevents creating an IFC artifact whose default export declares no IFC standard. Native test execution remains blocked by the shared build allocation failure.

- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs`

## Optional Complex Field Gestures

The IFC4 editor now uses its existing per-editor `snapshot_edit_special` hook for complex-field gestures. It plans against the model's default empty constituent list, supports insertion into an omitted list, replacement and nested edits, and maps removal of the entire optional field to SetEntityComplex with an empty list. The shared edit engine remains untouched. New source regressions cover initialization, nested editing and whole-field removal while retaining primary entity arguments.

- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/✏️editor/🦀️.rs`

## Executable TypeScript Mutation Implementation

Added `diffIfcMutation(base, mutation)`, `applyIfcMutation(base, mutation)` and `inverseIfcMutation(base, mutation)` beside the typed union. All ten kinds build sparse diffs directly. Application delegates to the checked central IFC diff applier; explicit undo mutations are generated from the centrally retained sparse inverse, so validation and original-value retention stay consistent across implementations. Entity removal records preserve bigint identity plus original position. No runtime dependency was added.

The reusable `ifc4-mutation-check.mjs` was authored first and failed because the APIs were absent. After implementation it passes all ten operation kinds against independently applied RFC 6902 patches from fast-json-patch 3.1.1, all ten explicit inverse roundtrips, base immutability, the committed complex fixture/diff, invalid target/index rejection, omitted-complex undo and full-width u64 identity retention. Strict TypeScript verification of mutation/diff/snapshot source using repository compiler options reports zero diagnostics. Generated evaluation modules were removed.

- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-mutation-check.mjs`

The local complex handler retains the shared editor admission checks: initialize an omitted optional complex field with InsertValue at the field path, then edit its members. The whole-field insertion, nested edit and whole-field removal regression follows this actual UI gesture sequence.

Final direct validation rerun passed both independent schema and ten-kind mutation checks; every changed Rust include target exists and no generated evaluation module remains.

## Native Mutation Framing and Schema Facets

The mutation binary decoder now checks the declared format byte, rejects unknown tags, requires full payload consumption with EOF, and rejects indices outside the native usize domain instead of truncating them. Two valid and nine malformed frames are committed as language-neutral JSON, with authored native decoder/encoder regression tests. Native tests remain unexecuted because of shared build allocation failure.

GraphQL and protobuf mutation contracts now include SetEntityComplex. The primary text grammar and its ANTLR/EBNF facets include all ten operations and the peer-owned Binary value variant; the binary ABNF exposes the actual format/kind envelope. Existing peer Binary schema, scalar codec branches, and native tags were preserved.

The registered permanent four-group TypeScript suite was executed from its actual checked-in source through a temporary module loader: all groups passed, with zero strict TypeScript diagnostics. This includes independent patch agreement for all ten kinds, exact neutral fixtures, explicit undo, binary values, omitted constituent lists, full-width IDs and rejected invalid targets. Loader input is `ifc4-permanent-mutation-check.mjs`; generated modules were deleted. Harness file URLs use pathToFileURL for native Windows/macOS/Linux paths.

## Final Inventory Additions

- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔗️.graphql`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🛰️.proto`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🧫️fixtures/📨️framing/🔣️.json`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🧪️tests/🦀️.rs`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🟦️.test.ts`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🅰️.g4`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/📝️text/🧬️mutations/🔤️.ebnf`
- `C:/git/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🚪️io/💾️binary/🧬️mutations/🔠️.abnf`
- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-permanent-mutation-check.mjs`
- `C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️21/COMPLETE-IFC-4X3-ARTIFACT-EDITING-AND-INTERCHANGE/ifc4-ordered-diff-review.md`
