# BIM Complete Semantic Gap Audit

Read-only current source proof; no build or runtime test. Domain base `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`. Hub `🌎️hub/🧩️compositions/🏙️bim/🦀️.rs` selects model artifact. This family provides a concrete universal completeness frontier rather than missing-hook heuristics.

## Exact omitted fields

ModelSnapshot in `🧬️schema/📸️snapshot/🦀️.rs` declares65 persisted fields. Mounted SQLite asset has93 CREATE TABLE statements. Actual P/R root `🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs::project` calls visit_rows, an explicit scalar/map visitor. restore's literal ModelSnapshot constructor includes49 fields and ends directly `};r.finish()?;Ok(result)` with no `..Default`. Exact declaration/constructor comparison identifies16 omitted fields:

| Field | Original domain type/owner | Required authored relationships |
|---|---|---|
| supports | StructuralSupport; snapshot/🦴️structure | member/location/offset/restraints; exact map key |
| load_cases | LoadCase; snapshot/🦴️structure | name/category/factor |
| loads | StructuralLoad; snapshot/🦴️structure | load_case/member/location/force/moment |
| option_groups | OptionGroup; snapshot/🧱️entities | name |
| design_options | DesignOption; snapshot/🧱️entities | group/name/primary |
| worksets | Workset; snapshot/🧱️entities | name/default_visible |
| element_options | ElementMembership; snapshot/🧱️entities | element map key/target option |
| element_worksets | ElementMembership; snapshot/🧱️entities | element map key/target workset |
| space_conditions | SpaceConditions; original snapshot imports | space key and every authored condition field |
| clash_sets | ClashSet; snapshot/🧱️entities | exact selections/tolerance/policy fields |
| rules | Rule; snapshot/🧱️entities | scope and authored condition/expression policy |
| issues | Issue; snapshot/🧱️entities | original issue identity/state/anchors/viewpoint fields |
| issue_comments | IssueComment; snapshot/🧱️entities | issue relationship plus authored comment fields |
| components | Component; snapshot/🧱️entities | family/storey/position/elevation/rotation/mirrored/optional host/system/name |
| component_overrides | ComponentOverride; snapshot/🧱️entities | component/name/value formula |
| mep_elements | MepElement; snapshot/🧱️entities | storey/system/shape/path and authored fields |

This is not merely grep absence: inspected macros integer/enumeration/scalar_record/entity implement explicit scalar/entity codecs only for authored listed types; they do not iterate ModelSnapshot fields. visit_rows explicitly calls each included map and never visits these16; restore literal constructor cannot receive them through a generic helper. Imported semantic_cells handles limits/record admission, not projection field traversal. None of16 appear in that helper. DDL has no tables for those omitted domains. With current struct declaration and literal constructor, source requires a compiler consistency repair even before a successful completeness roundtrip; no actual compiler error is claimed without execution.

## Actual capability route

IO root declares `bare_artifact_codec<ModelSnapshot,ModelMutation>` and mounts sqlite module. Its `💾️binary/📸️snapshot/🦀️.rs` ArtifactPack impl supplies encode/decode/record_spec but no sqlite_snapshot_codec. Current ArtifactPack trait default returns None; bare factory's snapshot_sqlite is P::sqlite_snapshot_codec. Therefore mounted handwritten SQL implementation does not itself register the public capability. Existing `🚪️io/🧪️tests/🪶️sqlite/🦀️.rs` already expects bare/public snapshot_sqlite Some and has source Native laws; these are authored expectations, not evidence that runtime registration works.

## Runnable next contract

Handcraft semantic DDL and exact language-neutral fixture for all65 top-level fields before adding provider registration. Every map needs document relation, ordinal and literal map_key distinct from display name; option/workset memberships, component override, load case and comments need their explicit original target references. Original scalar bit/class representation already used by BIM should extend to new float fields. Tagged structural location/MEP/condition unions need normalized authored tag/variant fields and child sequences, not JSON blobs. No derived geometry/elevation/clashes/analysis result should be persisted as substitute for authored inputs.

At least one independently nonempty authored row for each16 omitted maps is necessary: all-empty defaults cannot expose loss. Fixture should connect meaningful parent references, optional absent/present variants, exact bool/int/UTF8/NUL/formula strings, nonfinite scalar bit patterns where permitted, ordered point/list children and map order. Independent SQLite validates exact tables/relationships/ordinals and reconstructs a comparable authored semantic object. Strict Source law checks the actual original input schema and explicit provider mount; Native laws call direct P/R then genuine bare factory in both native encodings, compare all65 fields, exercise malformed relationships and original cancellation/grant/custody cleanup.

Execution should use existing owning BIM registered Source/Native route after bootstrap compiler floor changes; do not execute a reduced synthetic crate to claim public capability. Preserve concurrent owner migrations and shared workspace receiving defaults. This audit supplies source evidence only.
