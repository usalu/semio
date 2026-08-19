//! 🧩️ Assembly artifact — a WaveFunctionCollapse-style rule/slot composition engine. Authored fresh
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, packet W2-P5): unlike `procedural2d`/
//! `procedural3d`, this artifact never had a `🎛️apps` tree to migrate — the schema/mutations/
//! inferences tree under `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/` predates this ticket and is
//! reused as-is (never guessed); only this file plus the `✏️editor`/`👁️viewer` surfaces are new.

pub use crate::artifacts::assembly::schema::snapshot::ASSEMBLY_DOCUMENT_SCHEMA;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this artifact's ONE subset (`✳️any`) — `s.assembly@1/*`. Lives at the ARTIFACT root
/// (not under `✏️editor`/`👁️viewer`) so a viewer file can read it without ever importing through the
/// sibling editor module. `artifact_kind` matches `AssemblySnapshot`/`AssemblyDiff`'s own real
/// `#[artifact_schema(id = "s.assembly")]` attribute and `ASSEMBLY_DOCUMENT_SCHEMA`'s literal value —
/// grepped against the schema tree before writing this, NOT the `"s.procedural.assembly"` naming this
/// ticket's brief guessed by analogy with `procedural2d`/`procedural3d` (those two nest under the
/// `procedural` plugin id; assembly's own schema tree was authored with a bare `"s.assembly"` id by
/// the wave that built it, predating this ticket — followed as found, not overridden). `standard`/
/// `subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location on disk.
pub const ASSEMBLY_DIALECT: Dialect = Dialect { artifact_kind: ASSEMBLY_DOCUMENT_SCHEMA, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — a headless rule/slot specification (the WFC SOLVE is an
/// inference, never persisted media), so `dimension`/`media_class`/`media_form` follow `energy.model`'s
/// "data" precedent (also a schema-first, app-free artifact authored under this same ticket) rather
/// than `procedural2d`/`procedural3d`'s `Flow` shape — assembly has no flow-graph fixture to render.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.assembly".into(),
        name: "Assembly".into(),
        source_format: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        component_kind: "assembly".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ASSEMBLY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🧾️ Defines `s.assembly`'s immutable runtime capability leaves — the single `schema.artifact`
/// capability this packet's brief asks for. `descriptor`/`claim` use the SAME `"s.assembly"` string
/// `ASSEMBLY_DIALECT` above derives from (verified against the schema tree, not guessed).
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};
    ArtifactDefinition::new(ArtifactIdentity::parse("s.assembly")?).capability(
        ArtifactCapability::new(ArtifactIdentity::parse("s.assembly.schema.artifact")?, ArtifactCapabilityKind::schema())
            .descriptor(b"s.assembly")?
            .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.assembly")?)?,
    )
}

// 🚧️ NO `declaration()` here yet — deliberately, not an oversight. `ArtifactDeclaration::builder(...)
// .schema(descriptor)` (`🧰️framework/…/🔌️plugin/🦀️component.rs:2883`) is typestate-MANDATORY: the
// builder cannot reach `.try_build()` without it, and `ArtifactSchemaDescriptor` needs FOUR facets
// (artifact/snapshot/diff/mutations) each carrying FIVE handcrafted `&'static str` leaves (rust/
// typescript/graphql/json_schema/proto) via `include_str!`. Verified on disk: `🧬️schema/📸️snapshot/`,
// `🔺️diff/`, `🧬️mutations/` each carry ONLY `🦀️component.rs`+`🟦️component.ts` — no `🔣️component.json`/
// `🔗️component.graphql`/`🛰️component.proto` anywhere in this artifact's schema tree, and there is no
// `🧬️schema/🦀️component.rs` artifact-facet file at all (contrast `energy.model`, whose equivalent
// facet + a `energy_model_artifact_schema_descriptor()` fn were built by an EARLIER, separate wave
// before this ticket ever touched it — this ticket's own `📓️w2-cad-report.md` recipe never asks a W2
// surface packet to author schema-descriptor leaves, and this packet's brief scoped "fully real" to
// snapshot/diff/mutations/inferences specifically, not the descriptor facet). Authoring 14 new
// handcrafted GraphQL/JSON-Schema/Protobuf files for a domain this packet did not design is out of
// this packet's named scope; see `📓️w2-p5-assembly-notes.md` for the exact gap and who should close
// it. `artifact_kind()`/`definition()` above are complete and independent of this gap.
//#endregion 🔖️Declaration
