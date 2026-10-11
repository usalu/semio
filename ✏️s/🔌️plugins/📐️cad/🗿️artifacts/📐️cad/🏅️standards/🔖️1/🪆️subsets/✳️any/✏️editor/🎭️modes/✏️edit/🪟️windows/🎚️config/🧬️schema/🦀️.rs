use crate::editor::cad::config::{CadDislocateOptions, CadSunConfig};
use crate::CadCamera;
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct CadWorldWindowConfig {
    pub camera: CadCamera,
    pub sun: CadSunConfig,
    pub dislocate_options: CadDislocateOptions,
}
