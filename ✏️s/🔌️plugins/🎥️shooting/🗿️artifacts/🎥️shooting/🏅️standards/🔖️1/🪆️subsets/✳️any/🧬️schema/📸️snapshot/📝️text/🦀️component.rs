//! 📜️ Shooting artifact — textual document grammar surface + laws (constitutional: dsl).
//!
//! `store::ArtifactDsl for ShootingSnapshot` is implemented directly on the artifact type (see
//! `🗿️artifacts/🎥️shooting/🦀️component.rs`'s doc comment for why). This component only adds the thin
//! artifact-facing `parse_dsl`/`print_dsl` wrappers plus the canonical example-fixture constant and its
//! round-trip law.


//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


use crate::artifacts::shooting::ShootingSnapshot;

/// 🗄️ The base-icon example snapshot, handcrafted in `shooting`'s DSL (`store::ArtifactDsl`).
pub const SHOOTING_EXAMPLE_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");

/// 📖️ Parses `.shooting` DSL text into a `ShootingSnapshot`.
pub async fn parse_dsl(text: &str) -> Result<ShootingSnapshot, store::TextError> {
    <ShootingSnapshot as store::ArtifactDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `ShootingSnapshot` back to `.shooting` DSL text.
pub async fn print_dsl(snapshot: &ShootingSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::shooting::{ShootingAmbient, ShootingAsset, ShootingCamera, ShootingMaterial, ShootingSavedCamera, ShootingSceneLighting, ShootingShadow, ShootingShot, ShootingSun, SHOOTING_DOCUMENT_SCHEMA};

    /// 🎞️ A fixture exercising every field/variant, shared verbatim by the DSL and OpText law tests.
    #[allow(clippy::approx_constant, reason = "0.7071 is deliberately an approximate quaternion component in this snapshot, not the FRAC_1_SQRT_2 constant")]
    async fn representative_snapshot() -> ShootingSnapshot {
        ShootingSnapshot {
            schema: SHOOTING_DOCUMENT_SCHEMA.into(),
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
            },
            shots: vec![
                ShootingShot { id: "s1".into(), label: "Overview".into(), width: 256, height: 256, format: "svg".into(), shape: "rectangle".into(), background: Some("#ffffff".into()), camera_id: Some("cam1".into()) },
                ShootingShot { id: "s2".into(), label: "Detail".into(), width: 512, height: 512, format: "png".into(), shape: "ellipse".into(), background: None, camera_id: None },
            ],
            active_shot_id: "s1".into(),
            active_asset_id: "a1".into(),
            emblem: Some(crate::artifacts::shooting::shooting_emblem_child_handle(&crate::artifacts::shooting::shooting_emblem_image_from_bytes(vec![137, 80, 78, 71]))),
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_dsl_round_trips_representative_fixture() {
        store::os_store::test_support::assert_dsl_round_trip(&representative_snapshot());
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_dsl_round_trips_empty_fixture() {
        store::os_store::test_support::assert_dsl_round_trip(&crate::artifacts::shooting::empty_shooting_snapshot());
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_dsl_round_trips_base_icon_example() {
        let snapshot = parse_dsl(SHOOTING_EXAMPLE_TEXT).expect("base-icon example parses");
        store::os_store::test_support::assert_dsl_round_trip(&snapshot);
    }

    #[semio_framework_async_macros::async_test]
    async fn shooting_dsl_angle_deg_field_round_trips_bit_exactly() {
        let mut snapshot = representative_snapshot();
        snapshot.saved_cameras[0].camera.fov = 30.0;
        snapshot.scene.sun.azimuth = 30.0;
        let text = print_dsl(&snapshot);
        let reparsed = parse_dsl(&text).expect("parse_dsl");
        assert_eq!(reparsed.saved_cameras[0].camera.fov, 30.0);
        assert_eq!(reparsed.scene.sun.azimuth, 30.0);
        assert_eq!(snapshot, reparsed);
    }
}

//#endregion 🧪️Tests
