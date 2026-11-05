//! 🏢️ Compliant DE office RC frame example.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "seismic-rc-frame";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio");

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Seismic RC frame (DE S_aP,R 1.5 m/s², compliant)", "Erdbeben-Stahlbetonrahmen (DE S_aP,R 1,5 m/s², konform)")
}

pub fn snapshot() -> En1998Snapshot {
    En1998Snapshot::compliant_de_office().with_example_joints(false)
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
