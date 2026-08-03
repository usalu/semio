//! 📦️ Shooting app — binary document surface + laws (constitutional: pack).

use shooting::ShootingFixture;
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
    use shooting::{ShootingAmbient, ShootingAsset, ShootingCamera, ShootingMaterial, ShootingSavedCamera, ShootingSceneLighting, ShootingShadow, ShootingShot, ShootingSun, SHOOTING_FIXTURE_SCHEMA};

    /// 🎞️ A fixture exercising every field/variant — duplicated verbatim across the `dsl`/`op`/`pack`
    /// crates' test modules (each is its own compilation unit).
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
        store::test_support::assert_dsl_pack_equivalence(&shooting::empty_shooting_fixture());
    }

    #[test]
    fn pack_round_trips_base_icon_example_and_agrees_with_dsl() {
        let fixture = shooting_dsl::parse_dsl(shooting_dsl::SHOOTING_EXAMPLE_TEXT).expect("base-icon example parses");
        store::test_support::assert_dsl_pack_equivalence(&fixture);
        let bytes = encode(&fixture);
        assert_eq!(decode(&bytes).expect("decode"), fixture);
    }
}
//#endregion 🧪️Tests
