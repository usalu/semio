//! ⚠️ Non-compliant DE office RC frame example.

use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_plugin::ExampleSource;
use semio_ui_locale::LocalizedLabel;

pub const ID: &str = "seismic-rc-frame-fail";
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("../../🖼️assets/⚠️seismic-rc-frame-fail/⚠️seismic-rc-frame-fail/🗣️.dsl.semio");

pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Seismic RC frame (DE S_aP,R 1.5 m/s², failing)", "Erdbeben-Stahlbetonrahmen (DE S_aP,R 1,5 m/s², nicht konform)")
}

pub fn snapshot() -> En1998Snapshot {
    En1998Snapshot::noncompliant_de_office().with_example_joints(true)
}

pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}
