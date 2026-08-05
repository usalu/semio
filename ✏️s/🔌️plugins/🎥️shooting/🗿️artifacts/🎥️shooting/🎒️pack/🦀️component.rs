//! 📦️ Shooting artifact — binary document surface + laws (constitutional: pack).
//!
//! `store::DocumentPack for ShootingFixture` is implemented directly on the artifact type (see
//! `🗿️artifacts/🎥️shooting/🦀️component.rs`'s doc comment for why). This component only adds the thin
//! artifact-facing `encode`/`decode` wrappers plus the pack↔dsl equivalence law.

use crate::artifacts::shooting::ShootingFixture;
use store::PackError;

/// 📦️ Encodes a `ShootingFixture` to its binary pack form.
pub fn encode(fixture: &ShootingFixture) -> Vec<u8> {
    store::DocumentPack::encode_pack(fixture)
}

/// 📖️ Decodes a `ShootingFixture` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<ShootingFixture, PackError> {
    <ShootingFixture as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::{ShootingAmbient, ShootingAsset, ShootingCamera, ShootingMaterial, ShootingSavedCamera, ShootingSceneLighting, ShootingShadow, ShootingShot, ShootingSun, SHOOTING_FIXTURE_SCHEMA};
    use crate::artifacts::shooting::dsl;
    use crate::artifacts::shooting::op::ShootingOperation;

    /// 🎞️ A fixture exercising every field/variant — duplicated verbatim across the `dsl`/`op`/`pack`
    /// component tests (each region is its own concern).
    #[allow(clippy::approx_constant, reason = "0.7071 is deliberately an approximate quaternion component in this fixture, not the FRAC_1_SQRT_2 constant")]
    fn representative_fixture() -> ShootingFixture {
        ShootingFixture {
            schema: SHOOTING_FIXTURE_SCHEMA.into(),
            assets: vec![
                ShootingAsset { id: "a1".into(), name: "Base \"Mesh\"".into(), url: "/mesh/a1.glb".into(), format: "glb".into(), origin: [1.0, 2.0, 3.0], orientation: Some([0.0, 0.0, 0.7071, 0.7071]), scale: Some([2.0, 2.0, 2.0]) },
                ShootingAsset { id: "a2".into(), name: "Plain".into(), url: "/mesh/a2.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None },
            ],
            saved_cameras: vec![ShootingSavedCamera { id: "cam1".into(), label: "Hero".into(), camera: ShootingCamera { position: [9.0, 9.0, 9.0], ..Default::default() } }],
            scene: ShootingSceneLighting {
                background: "#111111".into(),
                sun: ShootingSun { enabled: true, azimuth: 12.5, elevation: 33.0, intensity: 3.0, color: "#ff00ff".into() },
                ambient: ShootingAmbient { intensity: 0.9, color: "#00ffff".into() },
                shadow: ShootingShadow { enabled: false, opacity: 0.5, softness: 0.2 },
                material: ShootingMaterial { color: "#abcdef".into(), metalness: 0.3, roughness: 0.7, emissive: "#123456".into(), emissive_intensity: 0.1 },
                emblem_base64: Some("data:image/png;base64,abc==".into()),
            },
            shots: vec![
                ShootingShot { id: "s1".into(), label: "Overview".into(), width: 256, height: 256, format: "svg".into(), shape: "rectangle".into(), background: Some("#ffffff".into()), camera_id: Some("cam1".into()) },
                ShootingShot { id: "s2".into(), label: "Detail".into(), width: 512, height: 512, format: "png".into(), shape: "ellipse".into(), background: None, camera_id: None },
            ],
            active_shot_id: "s1".into(),
            active_asset_id: "a1".into(),
        }
    }

    #[test]
    fn pack_round_trips_representative_fixture() {
        store::test_support::assert_dsl_pack_equivalence(&representative_fixture());
        let bytes = encode(&representative_fixture());
        assert_eq!(decode(&bytes).expect("decode"), representative_fixture());
    }

    #[test]
    fn pack_round_trips_empty_fixture() {
        store::test_support::assert_dsl_pack_equivalence(&crate::artifacts::shooting::empty_shooting_fixture());
    }

    #[test]
    fn pack_round_trips_base_icon_example_and_agrees_with_dsl() {
        let fixture = dsl::parse_dsl(dsl::SHOOTING_EXAMPLE_TEXT).expect("base-icon example parses");
        store::test_support::assert_dsl_pack_equivalence(&fixture);
        let bytes = encode(&fixture);
        assert_eq!(decode(&bytes).expect("decode"), fixture);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `ShootingOperation`'s `Edit` round-trips through `protocol::OperationEnvelope`s beside this
    /// file's existing dsl/pack round-trip laws (same pattern as `dag`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use protocol::{CollectionOperation, DocumentId, Edit, SchemaId};
        use store::{create_document_envelope, DocumentCommand, DocumentStore};

        let mut store: DocumentStore<ShootingFixture, ShootingOperation> = DocumentStore::new(create_document_envelope(SHOOTING_FIXTURE_SCHEMA, "shooting", crate::artifacts::shooting::empty_shooting_fixture(), None));
        store
            .dispatch(DocumentCommand::Apply {
                operations: vec![ShootingOperation::Assets(CollectionOperation::Add { id: "a1".into(), item: ShootingAsset { id: "a1".into(), name: "Asset".into(), url: "/mesh/a1.glb".into(), format: "glb".into(), origin: [0.0, 0.0, 0.0], orientation: None, scale: None }, at: 0 })],
                description: None,
            })
            .expect("apply");
        let edit: &Edit<ShootingOperation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::test_support::assert_command_envelope_round_trip::<ShootingFixture, ShootingOperation>(edit, &DocumentId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests
