# Lowpoly Create-Object Schema Repair

The original draft07 CreateObject schema contained JSON null at properties/object/properties/meshState. Strict Ajv rejected the schema itself. The committed canonical mutation fixture contains an explicit nullable meshState; the Rust LowpolyObject stores Option<LowpolyMeshState>, and the public artifact schema already defines this exact complete managed-state owner.

The mutation now references the public LowpolyObject meshState property rather than inventing a replacement or allowing arbitrary data. Its required object fieldset matches the current public canonical object, including meshContent and meshState. Existing localized UI annotations and all other mutation property constraints remain in place. The managed topology remains independent of literal source text and the mesh-child handle.

TDD: the original Source produced0passing/9failing tests at meta compilation. The repaired schema passes10tests/23assertions against the strict central semio Ajv oracle, owned validator and actual managed-state parser: unchanged committed payload, explicit null, complete empty owned topology, scalar/partial/unknown state refusals, missing canonical fields and public fieldset parity. An intermediate test incorrectly passed JSON numeric vectors to the intrinsic Binary32 object parser; that test boundary was corrected to validate JSON through schema validators and managed state through its actual parser. No runtime parser coercion or compatibility path was added.

Portable state/refusal data and Bun tests live in create-object/tests. The same data is registered in the owning objects Rust test binary using the owned validator and six actual referenced schema resources; only an existing codebase package was added as a dev dependency. Actual canonical Rust execution is pending coordination; no Rust test pass is claimed yet.

Changed production paths under the lowpoly v1/any schema:
- mutations/create-object/schema/JSON
- mutations/create-object/tests/{JSON,TS,Rust}
- mutations/objects/tests/Rust
- mutations/objects/packages/Rust/Cargo.toml

No producer regeneration, arbitrary schema placeholder, validation bypass or grant change was used.

Full actual TypeScript semantic53168 passed17roots4303importedfiles with no diagnostics, including this test. Widened only the Bun equality expectation to unknown so malformed neutral values remain intentionally represented. Added this exact suite path to existing lowpoly TypeScript package test suites; current direct10tests23assertions pass0. Actual Rust52783 still compiling dependencies, not yet a test result.

Read-only registration audit found schema/oracle Source outside old explicit inputs. Added actual framework/schema subtree to existing test inputs and all three exact suites as schema-owned runtimeSourceEntries, preserving old SQLite suites. Actual project-schema Ajv validation initially rejected missing workspaceRoot prefixes; corrected all three to the required rooted grammar, then current project validates0. These are input/entry declarations only, no grant widening.

Actual funded Rust52783 terminated1 before test execution after compiling the real dependency closure. Its eight diagnostics all identify missing LowpolyObjectPatch in model/diff, although the canonical public snapshot module already defines it. Added only that existing type to the current explicit import; no replacement type, API adapter, or implementation change. Incremental retry will follow the independent daemon proof to avoid contention. No Rust acceptance claimed yet.

Incremental retry90033 passed the model/diff compilation and reached a second prerequisite: apply-paint-stroke inverse delegates to EditPaintLayer but omitted the required generic conversion bound. Actual E0277 before tests. Declared the existing From<EditPaintLayer> requirement on that inverse function and its cold mutation impl, keeping ordinary mutation metadata capabilities unchanged. No alternate inversion implementation or conversion fallback added.

Retry71608 advanced past paint compilation and failed two derive diagnostics in geometry CreateMesh: its explicit value codec still addressed crate::managed_mesh, while this crate is the geometry mutation owner. Replaced the single verified stale attribute with the canonical model::managed_mesh::json path. A bounded mutation-subtree search found no second occurrence. No alias module or compatibility export introduced.

Actual normal-admitted final44865 TERMINAL0: release dependency closure compiled43.73seconds and canonical_create_object_schema_preserves_complete_managed_mesh_ownership passed1/1,64other tests filtered. This executes the same portable neutral mesh-state/refusal cases and unchanged committed mutation payload through the owned Rust validator and actual six referenced resources. All three prerequisite compile repairs are accepted in this real closure; no full Lowpoly package-suite or monorepo green claim.

Additional accepted prerequisite Source files:
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️apply-paint-stroke/↩️inverse/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🖌️apply-paint-stroke/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕸️create-mesh/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript/📜️script.ts
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📦️packages/🟦️typescript/📋️project.json
