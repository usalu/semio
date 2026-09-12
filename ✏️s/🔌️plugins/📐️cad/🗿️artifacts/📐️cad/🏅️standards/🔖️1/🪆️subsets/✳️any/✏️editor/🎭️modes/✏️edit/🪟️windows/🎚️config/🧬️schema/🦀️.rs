use crate::editor::cad::config::{CadDislocateOptions, CadSunConfig};
use crate::CadCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CadWorldWindowConfig {
    pub camera: CadCamera,
    pub sun: CadSunConfig,
    pub dislocate_options: CadDislocateOptions,
}
