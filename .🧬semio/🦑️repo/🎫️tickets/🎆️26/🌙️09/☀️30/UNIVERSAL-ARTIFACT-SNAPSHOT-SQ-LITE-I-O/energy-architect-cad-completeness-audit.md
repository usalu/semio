# Energy Architect CAD Completeness Audit

Read-only current source proof; no builds/tests. Owning domain bases below are under `✏️s/🔌️plugins/`, each standard1/subsetany. No comparable omitted top-level field gap beyond BIM was demonstrated in these three families.

## Energy

`🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/`: EnergyModelSnapshot has6 original fields, and reconstruct literal explicitly supplies all6: schema,model,structure,zones,referenced_model,weather_link. Model has44 fields at `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔋️model/🦀️.rs:999`. Reconstruct initializes name/version/site/ground_temperature/run_period then uses envelope::reconstruct and systems::reconstruct plus schedules::reconstruct to populate collections initialized with Model::default. All44 actual field names appear in these mounted helper traversals; default initialization is scratch state, not evidence those collections are omitted.

System helpers explicitly restore ModelAirLoop all6 fields, PlantLoopConfig all7 fields, airflow network zone/link sequences, electrical generator/PV/battery IDs, faults, output variables, sizing, daylight and room-air assignments. Child/link reader retains original artifact ID and dialect strings, optional link tags head/checkpoint/snapshot, BlobRef hash/size/media and rejects unused tagged payload. Mounted root69-table DDL is concrete semantic schema; no results_json field appears in actual6-field snapshot. ArtifactPack provider explicitly returns Some(sqlite_codec) at binary snapshot:64. Further proof needs nested subtype/enum branch census and independently nonempty every44 Model field fixture, not a missing-provider assertion.

## Architect

`🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/`: ProgramSnapshot declares72 fields. Actual snapshot_registers invocation lists65 typed registers; core project/restore handles schema,meta,project,governance,knowledge,benchmarks,traces. Exact field-set comparison leaves zero unhandled top-level fields. The macro emits explicit register_project and T::restore calls for each65 fields, so a literal grep for field names in generated code would be misleading.

Mounted register_sql bodies explicitly declare scalar, ordered children and optional singleton children; custom AuditEvent provides original before_state/after_state text fields and trace record. knowledge_payload and benchmarks_payload are actual typed registers, while knowledge/benchmarks are independent child references; neither may replace the other. Root685-table asset supports the authored typed register layout. ArtifactPack binary snapshot:31 explicitly exposes SQLite codec. A next rigorous frontier is nested RegisterSql type fields versus macro field lists and row-width/tag alternatives, especially custom AuditEvent/header/core ownership and the independent child payload pair. No proven omission is claimed without that exact nested comparison.

## CAD

`📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/`: CadSnapshot declares10 fields. Reconstruction installs DecodedFieldOwner with all10 empty fields, then fills actual original schema/id, four distinct optional child slots, drawing/BREP sequences, reference index and nodes. It rejects duplicated child slots and validates exact child target dialect after copying its literal original reference. The7-table schema models parent and child/reference relationships, not the child full payload; completeness requires resolving those persisted child owners independently, not flattening them into a CAD blob. ArtifactPack binary snapshot:39 explicitly registers SQLite provider.

CAD cold restore forwards SQL original allocation via NativeDecodeControl::new_forwarded and uses DecodedFieldOwner; current shared workspace defaults loan allocation ports. That establishes authored source routing, not automatic original body receipt/prefix runtime qualification. Verify nested reference optional scalar presence, node parent/order and exact children in genuine capability law before claiming complete runtime behavior.

## Priority

BIM remains the concrete16-field omission and registration gap. These three selected providers have top-level source coverage and actual explicit provider registration; they are complex unverified validation frontiers rather than demonstrated missing schemas. Next audit should compare every nested entity/register constructor against its actual domain fields and each tagged scalar/ordered-child branch, then feed independently authored complete fixtures through real owning Native factories. Test file presence, broad all-variants labels and table counts are insufficient proof.
