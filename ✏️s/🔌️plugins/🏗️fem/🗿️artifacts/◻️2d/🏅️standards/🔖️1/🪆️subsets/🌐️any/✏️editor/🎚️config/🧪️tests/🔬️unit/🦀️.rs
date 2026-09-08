
use super::*;

#[test]
fn fem2d_config_default_is_static_display_with_default_camera_and_locale() {
    let config = Fem2dConfig::default();
    assert_eq!(config.result_mode, "static");
    assert!(config.result_source_id.is_none());
    assert_eq!(config.result_mode_index, 0);
    assert_eq!(config.camera, FemCamera::default());
    assert_eq!(config.locale, "en-US");
}

/// 🧮️ `Fem2dConfig`'s `MutationDiff` is a whole-record replace, mirroring `ShootingConfig`'s
/// identical B1 pilot pattern: `apply` ignores `base` entirely.
#[test]
fn fem2d_config_operation_diff_is_a_whole_record_replace() {
    let base = Fem2dConfig::default();
    let mut replacement = Fem2dConfig::default();
    replacement.locale = "de-DE".into();
    replacement.camera = FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
    let applied = protocol::MutationDiff::apply(&replacement, &base).expect("valid config mutation diff");
    assert_eq!(applied, replacement);
    let mut absorbed = base.clone();
    protocol::MutationDiff::absorb(&mut absorbed, replacement.clone());
    assert_eq!(absorbed, replacement);
}

#[test]
fn config_operation_backwards_always_restores_the_pre_operation_snapshot() {
    let base = Fem2dConfig::default();
    let camera = FemCamera { x: 1.0, y: 2.0, zoom: 3.0 };
    let op = Fem2dConfigMutation::SetCamera { camera: camera.clone() };
    let next = op.diff(&base).diff().clone();
    assert_eq!(next.camera, camera);
    let backwards = op.inverse(&base);
    assert_eq!(backwards, vec![Fem2dConfigMutation::Snapshot { config: base.clone() }]);
    assert_eq!(backwards[0].diff(&next).diff(), &base);
}

#[test]
fn set_result_display_config_operation_round_trips() {
    let base = Fem2dConfig::default();
    let op = Fem2dConfigMutation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 2 };
    let next = op.diff(&base).diff().clone();
    assert_eq!(next.result_source_id.as_deref(), Some("dead"));
    assert_eq!(next.result_mode, "modal");
    assert_eq!(next.result_mode_index, 2);
}

#[test]
fn set_locale_config_operation_round_trips() {
    let base = Fem2dConfig::default();
    let op = Fem2dConfigMutation::SetLocale { value: "de-DE".into() };
    let next = op.diff(&base).diff().clone();
    assert_eq!(next.locale, "de-DE");
}

/// 🧷️ LAW: every `Fem2dConfigMutation` variant owns exactly one `MutationLeafDescriptor`, and
/// `descriptor()` returns the one whose `aggregate_variant` names it.
#[test]
fn every_config_mutation_variant_has_its_own_descriptor() {
    let variants = [
        Fem2dConfigMutation::Snapshot { config: Fem2dConfig::default() },
        Fem2dConfigMutation::SetResultDisplay { source_id: None, mode: "static".into(), mode_index: 0 },
        Fem2dConfigMutation::SetCamera { camera: FemCamera::default() },
        Fem2dConfigMutation::SetLocale { value: "de-DE".into() },
    ];
    assert_eq!(<Fem2dConfigMutation as Mutation<Fem2dConfig>>::DESCRIPTORS.len(), variants.len());
    for (index, variant) in variants.iter().enumerate() {
        let descriptor = variant.descriptor();
        assert_eq!(descriptor, &<Fem2dConfigMutation as Mutation<Fem2dConfig>>::DESCRIPTORS[index]);
        assert_eq!(descriptor.schema_version, 1);
        assert!(!descriptor.display_name.is_empty() && !descriptor.emoji.is_empty());
    }
}

#[test]
fn fem2d_config_operation_text_round_trips_every_variant() {
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dConfigMutation::Snapshot { config: Fem2dConfig::default() });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dConfigMutation::SetResultDisplay { source_id: Some("dead".into()), mode: "modal".into(), mode_index: 1 });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dConfigMutation::SetCamera { camera: FemCamera { x: 1.0, y: 2.0, zoom: 1.5 } });
    semio_framework_os_kernel::os_store::test_support::assert_op_line_round_trip(&Fem2dConfigMutation::SetLocale { value: "de-DE".into() });
}
