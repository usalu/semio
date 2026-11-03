use crate::mutations::change_viewport_view::ChangeViewportView;
use crate::schema::NormViewportWindowConfig;
use crate::mutations::NormViewportWindowConfigMutation;

pub fn inverse(_payload: &ChangeViewportView, base: &NormViewportWindowConfig) -> Result<Vec<NormViewportWindowConfigMutation>, semio_framework_value::ValueError> {
    Ok(vec![ChangeViewportView { camera_preset: base.camera_preset.clone(), hidden_layers: base.hidden_layers.clone(), colour_map: base.colour_map.clone(), unit_system: base.unit_system.clone() }.into()])
}
