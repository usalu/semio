//! 🎥️ Shooting artifact — the document entity this plugin's app edits: the real icon-studio snapshot
//! (assets, shots, saved cameras, scene lighting).
//!
//! `ShootingSnapshot` lives in `📸️snapshot/🧬️schema` and is re-exported here. Domain records and
//! patch types stay in this root component.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as schema;
extern crate semio_framework_value_derive as value_derive;

pub use crate::standards::v1::subsets::any::schema::mutations::ShootingMutation;

pub use crate::standards::v1::subsets::any::schema::diff::ShootingDiff;

// 🪪️ Declaration of the shooting artifact: the document schema and dialect, the artifact kind, the pilot languages and the definition. See [`super`].

pub use semio_s_artifact_shooting_shooting_inferences::{SHOOTING_DOCUMENT_SCHEMA};

pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::{SHOOTING_DIALECT};

pub use semio_s_artifact_shooting_shooting_shooting::{artifact_kind};

//#region 🔖️Declaration
/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `OnceLock`-backed `io_registry::entries()` convention already used below. Sole caller is
/// `declaration()` below (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "shooting.document",
                    extension: Some("shooting"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("shooting.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "shooting.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::mutations::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("shooting.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "shooting.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(standards::v1::subsets::any::io::text::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("shooting.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "shooting.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("shooting.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "shooting.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(standards::v1::subsets::any::io::binary::mutations::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("shooting.spr"),
                },
            ]
        })
        .as_slice()
}

/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called the io registry/schema/inference/language
/// registries and the document codec registrar directly from a plugin `.setup()` callback.
/// `crate::editor::shooting::config::schema::register_app_schema()` is the one exception, still called
/// from `🎥️shooting/🦀️.rs`'s own `.setup()`: it registers `ShootingPlayApp`'s own
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set.
///
/// DEVIATION (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES reloc-g1): the `.composers(...)`
/// argument is qualified to `standards::v1::subsets::any::io::io_registry::entries()` (the `⚙️engine`
/// directory that used to own this module is gone — deleted, not relocated to a sibling engine) rather
/// than left as the bare `io_registry::entries()` this body used while `io_registry` still lived in that
/// file. Left bare it would now resolve to THIS file's own `io_registry` module below, which has a
/// different, incompatible return type (`&'static [&'static ComposerEntry]`, wrapping the real registry's
/// owned entries) — not the `&'static [ComposerEntry]` `.composers()` expects.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.shooting.shooting.standard.v1", "standard", "1", &[], None),
        ("s.shooting.shooting.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.shooting.shooting.schema.artifact", "schema", "s.shooting.shooting", &[("schema", "s.shooting.shooting")], None),
        ("s.shooting.shooting.inference.artifact", "inference", "s.shooting.shooting.inference", &[("schema", "s.shooting.shooting.inference")], None),
        ("s.shooting.shooting.composer.native", "composer", "s.shooting.shooting@1/*", &[("dialect", "s.shooting.shooting@1/*")], None),
        ("s.shooting.shooting.composer.format-6", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.shooting.shooting.grammar.1", "grammar", "shooting.document", &[("grammar", "shooting.document")], None),
        ("s.shooting.shooting.grammar.2", "grammar", "shooting.op", &[("grammar", "shooting.op")], None),
        ("s.shooting.shooting.grammar.3", "grammar", "shooting.diff", &[("grammar", "shooting.diff")], None),
        ("s.shooting.shooting.grammar.4", "grammar", "shooting.pack", &[("grammar", "shooting.pack")], None),
        ("s.shooting.shooting.grammar.5", "grammar", "shooting.spr", &[("grammar", "shooting.spr")], None),
        ("s.shooting.shooting.codec.document-1", "codec", "shooting.shooting:shooting", &[("codec", "shooting.shooting"), ("codec-extension", "17:shooting.shooting:shooting")], None),
        ("s.shooting.shooting.localization.en", "localization", "Shooting", &[], Some(("en", "Shooting"))),
        ("s.shooting.shooting.localization.de", "localization", "Aufnahme", &[], Some(("de", "Aufnahme"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.shooting.shooting")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::shooting_artifact_schema_descriptor())
        .inferences([standards::v1::subsets::any::schema::inferences::shooting_artifact_inference_descriptor()])
        .composers(standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<editor::shooting::ShootingPlayApp>>()
        .try_build()
}
//#endregion 🔖️Declaration

pub use crate::standards::v1::subsets::any::schema::snapshot::ShootingSnapshot;

// 🌍️ Scene records of the shooting artifact: the cameras, assets, shots, lighting records with their defaults and the camera math. See [`super`].

pub use semio_s_artifact_shooting_shooting_inferences::{ShootingAmbient, ShootingAsset, ShootingCamera, ShootingMaterial, ShootingSavedCamera, ShootingSceneLighting, ShootingShadow, ShootingShot, ShootingSun, default_camera_position, default_camera_target, default_fov, default_glb_format, one_f64, quat_from_axis_angle, quat_mul, shooting_asset_scale};

pub use semio_s_artifact_shooting_shooting_inferences::{empty_shooting_snapshot, shooting_resolve_shot_camera};

pub use semio_s_artifact_shooting_shooting_inferences::{ShootingEmblemChild, shooting_base64_decode, shooting_base64_encode, shooting_emblem_bytes, shooting_emblem_bytes_from_image, shooting_emblem_child_handle, shooting_emblem_image, shooting_emblem_image_from_bytes, shooting_set_emblem_from_base64};

//#endregion 🔖️Composition

// 🩹️ Sparse patches of the shooting artifact: the identified records, the assigned wrapper and the asset, shot, camera and scene patches. See [`super`].

pub use semio_s_artifact_shooting_shooting_schema::{ShootingAssetPatch, ShootingAssigned, ShootingSavedCameraPatch, ShootingScenePatch, ShootingShotPatch};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

pub use semio_s_artifact_shooting_shooting_inferences::{active_asset, active_shot, is_transparent_shooting_background, next_shooting_id};

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "."]
                pub mod schema {
                    pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::schema::diff;
                    #[path = "."]
                    pub mod mutations {
                        pub use semio_s_artifact_shooting_shooting_mutations::standards::v1::subsets::any::schema::mutations::*;

                        #[path = "."]
                        pub mod create_asset {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::create_asset::inverse;

                        }
                        #[path = "."]
                        pub mod delete_asset {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::delete_asset::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::delete_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::delete_asset::inverse;

                        }
                        #[path = "."]
                        pub mod rename_asset {
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::*;
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_tests::standards::v1::subsets::any::schema::mutations::rename_asset::inverse;

                        }
                        #[path = "."]
                        pub mod change_asset_url {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::change_asset_url::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::reorder_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::reorder_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::schema::mutations::reorder_assets::inverse;

                        }
                        #[path = "."]
                        pub mod drag_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::drag_assets::inverse;

                        }
                        #[path = "."]
                        pub mod rotate_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rotate_assets::inverse;

                        }
                        #[path = "."]
                        pub mod scale_assets {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::scale_assets::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::scale_assets::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::scale_assets::inverse;

                        }
                        #[path = "."]
                        pub mod create_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::create_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::create_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::create_shot::inverse;

                        }
                        #[path = "."]
                        pub mod delete_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::delete_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::delete_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::delete_shot::inverse;

                        }
                        #[path = "."]
                        pub mod rename_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rename_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rename_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::rename_shot::inverse;

                        }
                        #[path = "."]
                        pub mod change_shot_width {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_width::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_width::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_width::inverse;

                        }
                        #[path = "."]
                        pub mod change_shot_height {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_height::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_height::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_height::inverse;

                        }
                        #[path = "."]
                        pub mod change_shot_format {
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_format::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_format::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any::standards::v1::subsets::any::schema::mutations::change_shot_format::inverse;

                        }
                        #[path = "."]
                        pub mod change_shot_shape {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::change_shot_shape::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::change_shot_shape::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::change_shot_shape::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_shots {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_shots::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_shots::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_shots::inverse;

                        }
                        #[path = "."]
                        pub mod replace_shot_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_shot_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_shot_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_shot_camera::inverse;

                        }
                        #[path = "."]
                        pub mod create_saved_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::create_saved_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::create_saved_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::create_saved_camera::inverse;

                        }
                        #[path = "."]
                        pub mod delete_saved_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::delete_saved_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::delete_saved_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::delete_saved_camera::inverse;

                        }
                        #[path = "."]
                        pub mod rename_saved_camera {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_saved_camera::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_saved_camera::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::rename_saved_camera::inverse;

                        }
                        #[path = "."]
                        pub mod replace_saved_camera_view {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::replace_saved_camera_view::inverse;

                        }
                        #[path = "."]
                        pub mod reorder_saved_cameras {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::reorder_saved_cameras::inverse;

                        }
                        #[path = "."]
                        pub mod set_active_shot {
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::set_active_shot::*;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::set_active_shot::diff;
                            pub use semio_s_artifact_shooting_shooting_mut_any_2::standards::v1::subsets::any::schema::mutations::set_active_shot::inverse;

                        }
                        #[path = "."]
                        pub mod set_active_asset {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::set_active_asset::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::set_active_asset::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::set_active_asset::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_sun_enabled {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_enabled::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_sun_azimuth {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_azimuth::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_sun_elevation {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_elevation::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_sun_intensity {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_sun_intensity::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_ambient_intensity {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_ambient_intensity::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_shadow_enabled {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_shadow_enabled::inverse;

                        }
                        #[path = "."]
                        pub mod change_scene_material_roughness {
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::*;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::diff;
                            pub use semio_s_artifact_shooting_shooting_set_active_asset::standards::v1::subsets::any::schema::mutations::change_scene_material_roughness::inverse;

                        }
                    }
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::standards::v1::subsets::any::io::*;
pub mod binary {
    pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::standards::v1::subsets::any::io::binary::*;
    pub use semio_s_artifact_shooting_shooting_schema::standards::v1::subsets::any::io::binary::diff;
    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::binary::inferences;
}
pub mod sqlite {
    pub use semio_s_artifact_shooting_shooting_mut_family::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::sqlite::snapshot;
}
pub mod text {
    pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::standards::v1::subsets::any::io::text::*;
    pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::text::inferences;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::txt;
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::txt;
                                pub use semio_s_artifact_shooting_shooting_inferences::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---- Shims: keep pre-migration module paths resolving for external callers ----

pub use semio_s_artifact_shooting_shooting_schema::diff;

pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_shooting_shooting_inferences::snapshot;

#[path = "."]
pub mod examples {
    #[path = "."]
    pub mod demo {
        pub use semio_s_artifact_shooting_shooting_mut_family::examples::demo::*;

    }
    #[path = "."]
    pub mod hexagonal_cut_concrete_forest_left {
        pub use semio_s_artifact_shooting_shooting_mut_family::examples::hexagonal_cut_concrete_forest_left::*;

    }
}

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod shooting {
        pub use semio_s_artifact_shooting_shooting_shooting::editor::shooting::*;
pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::demo_session;

        pub mod config {
    pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config::*;
    pub mod io {
        pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::config::io::*;
        pub mod binary {
            pub use semio_s_artifact_shooting_shooting_inferences::editor::shooting::config::io::binary::*;
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config::io::binary::mutations;
        }
        pub mod text {
            pub use semio_s_artifact_shooting_shooting_inferences::editor::shooting::config::io::text::*;
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::config::io::text::mutations;
        }
    }
}

        pub mod presence {
    pub use semio_s_artifact_shooting_shooting_config::editor::shooting::presence::*;
    pub mod io {
        pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::presence::io::*;
        pub mod binary {
            pub use semio_s_artifact_shooting_shooting_inferences::editor::shooting::presence::io::binary::*;
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::presence::io::binary::mutations;
        }
        pub mod text {
            pub use semio_s_artifact_shooting_shooting_inferences::editor::shooting::presence::io::text::*;
            pub use semio_s_artifact_shooting_shooting_config::editor::shooting::presence::io::text::mutations;
        }
    }
}
        pub use semio_s_artifact_shooting_shooting_mut_family::editor::shooting::terminology;
        pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::ui_helpers_and_io;
        pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::wasm;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::asset;
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::camera;
            pub use semio_s_artifact_shooting_shooting_shooting::editor::shooting::commands::export;
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::document;
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::gumball;
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::scene;
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::selection;
            pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::editor::shooting::commands::shot;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod scene {
                        pub use semio_s_artifact_shooting_shooting_set_active_asset::editor::shooting::modes::edit::windows::scene::*;

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::ambient;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::center_model;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::roughness;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::shadow;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_azimuth;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_elevation;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_enabled;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::scene::options::sun_intensity;
                        }
                    }

                    #[path = "."]
                    pub mod icon {
                        pub use semio_s_artifact_shooting_shooting_mutations::editor::shooting::modes::edit::windows::icon::*;

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::icon::options::format;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::icon::options::shape;
                            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::modes::edit::windows::icon::options::shot;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::catalogue;
            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::document;
            pub use semio_s_artifact_shooting_shooting_tests::editor::shooting::panels::inspection;
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod shooting {
        pub use semio_s_artifact_shooting_shooting_tests_dims_scene_sun_to_half::viewer::shooting::*;

        pub use semio_s_artifact_shooting_shooting_mut_family::viewer::shooting::presence;
        pub use semio_s_artifact_shooting_shooting_mut_family::viewer::shooting::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_shooting_shooting_tests::viewer::shooting::modes::view::*;

                pub use semio_s_artifact_shooting_shooting_tests::viewer::shooting::modes::view::windows;
            }
        }
    }
}

pub use crate::standards::v1::subsets::any::io::{ShootingBuilderConstruction, ShootingParts, ShootingAnalyzerAnalysis, ShootingBuilderFacets, ShootingBuilder, ShootingAnalyzer, ShootingComposer};

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
