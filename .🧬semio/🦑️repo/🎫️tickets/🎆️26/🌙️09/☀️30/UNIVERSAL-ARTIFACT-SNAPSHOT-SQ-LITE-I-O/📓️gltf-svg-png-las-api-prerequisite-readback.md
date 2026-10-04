# GLTF SVG PNG LAS API Prerequisite Readback

Read-only inspection on2026-10-03. No builds/Cargo/production changes. Authored owning candidates GLTF23, SVG10, PNG14, LAS14 are not runtime passes. Actual registered aliases are `@semio-tech/stdio-{gltf,svg,png,las}-rs:test-snapshot-sqlite-native`, not invented dialect aliases. PNG local9 includes three async laws; its mounted audit adds5. LAS local8 plus mounted cohort6 gives14. SVG7 ordinary plus3async gives10; GLTF23 ordinary gives23.

All paths below are rooted at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/`.

## Narrow Concrete Prerequisites

- GLTF `🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/📦️pack/🛫️encoding/🦀️.rs:15`: `to_value_controlled` now returns ValueError, but map_err(OutputError::Field) requires String. Use the existing OutputError::Refusal/From<ValueError> path; retain categories until the final String boundary.
- Same GLTF file line25: EncodedRecord::new now returns ValueError; its OutputError::Field mapping is likewise stale. Route through Refusal rather than converting a typed refusal into a field-string variant.
- LAS `☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/📦️pack/🚦️native/🦀️.rs:10`: fault uses two-argument TextError::new. Invalid literal/type/record errors need InvalidValue; row ceilings need OwnershipLimit, row/work arithmetic overflow WorkLimit. Do not blanket all errors as InvalidValue.
- Same LAS line11: refusal(ValueError) currently extracts only message and routes fault, discarding kind. Use TextError::from_value_error(error,span) to preserve ownership/work/canceled categories. Constructor repair alone is insufficient for honest terminal-kind laws.
- Deflate dependency draft `🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🚦️native/🦀️.rs:41–42`: Encoder implements DeflateEncodeControl methods returning Result<(),String>. Canonical first-party trait at `🧰️framework/🔨️modules/🗜️deflate/🦀️.rs:908–910` requires Result<(),ValueError>. Keep control.charge/begin_stage/advance typed; explicit file-size refusal is OwnershipLimit and backwards/overflow work is WorkLimit. Convert only at outer String owner boundary.
- Same Deflate draft lines47/62: deflate_controlled now returns ValueError, so `?` cannot propagate into String without an explicit ValueError::into_message boundary mapping. These callsites must be changed alongside the adapter trait.

GLTF inspected ValueError constructors in snapshot root, controlled pack and extras already supply kind+message. PNG root hexadecimal/decoder TextError constructors already supply kind+message+span. SVG inspected snapshot and caller constructors also already supply three arguments. No stale one-argument ValueError constructor was identified in this scoped inspection. This is source inspection, not compiler confirmation of every associated macro expansion.

## Actual Dependency Edges

Direct first-party edges already present: GLTF Cargo→stdio-json; SVG Cargo→stdio-xml; PNG Cargo→stdio-deflate. Each owner also directly depends on framework-value and framework-dsl. PNG draft imports existing stdio-deflate owned helpers; it should not add an unnecessary direct framework-deflate edge unless it imports that crate itself.

Deflate draft itself directly imports semio_framework_deflate but `🗜️deflate/📦️packages/🦀️rust/Cargo.toml` still lacks that edge. Author it when mounting; transitive PNG→stdio-deflate does not provide Deflate with a direct canonical engine dependency. Its two trait method mismatches remain dormant while draft is unmounted, but become exact compiler prerequisites on actual mount.

## Terminal and Semantic Discipline

Preserve UnsupportedOwner for genuinely missing controlled hooks, InvalidValue for malformed literals/variants, OwnershipLimit for admission/file/row ceilings, WorkLimit for traversal/counter arithmetic, AllocationFailed for failed admitted reserves and Canceled for callback cancellation. No blanket From<String>, legacy constructor adapter or weakened controlled default is justified. Controlled record/field mapping must preserve ValueError until positioned TextError or deliberate existing String owner boundary. Existing SVG/XML and GLTF/JSON direct edges do not establish owning runtime correctness or permit ordinary fallback.

This audit identifies narrow prerequisites; Native target compiler/runtime receipts remain required, especially owner mounts, exact literal fidelity, cancellation, retained-field retirement and cumulative allocation bridging.
