use crate::mutations::change_viewport_view::ChangeViewportView;
use crate::schema::NormViewportWindowConfig;
use crate::mutations::NormViewportWindowConfigMutation;

#[semio_framework_async_macros::async_test]
async fn changing_the_view_obeys_the_inverse_sum_law() {
    let views = [
        NormViewportWindowConfig::default(),
        NormViewportWindowConfig { camera_preset: Some("iso".into()), hidden_layers: vec!["loads".into()], colour_map: Some("viridis".into()), unit_system: Some("si".into()) },
        NormViewportWindowConfig { unit_system: Some("engineering".into()), ..NormViewportWindowConfig::default() },
    ];
    for base in &views {
        for next in &views {
            let mutation: NormViewportWindowConfigMutation = ChangeViewportView { camera_preset: next.camera_preset.clone(), hidden_layers: next.hidden_layers.clone(), colour_map: next.colour_map.clone(), unit_system: next.unit_system.clone() }.into();
            protocol::os_spr::protocol_laws::assert_mutation_inverse_sum_law(&mutation, base).await;
        }
    }
}
