//! 🏛️ Actual domain classification and owned native host configuration projection.
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin_host_configuration::{with_original_host_configuration,testkit::HeapWitness};
use semio_framework_value::DslValue;

#[global_allocator]
static HOST_CONFIGURATION_HEAP: HeapWitness = HeapWitness;

use semio_s_artifact_cad_cad_editor::CadPlayApp;
use semio_s_artifact_cad_cad_config::{CadConfigMutation,SetContributionsEdit};

#[test]
fn original_host_configuration_preserves_cad_raw_config_lane() {
    let input=r#"[{"id":"cad"}]"#;
    let declaration=CadPlayApp::original_host_configuration("setContributions").expect("declared original host source");
    with_original_host_configuration(declaration,Some(DslValue::object([("json".to_owned(),DslValue::String(input.to_owned()))])),|mutation|{
        assert_eq!(mutation,&CadConfigMutation::SetContributions(SetContributionsEdit{json:input.to_owned()}));
    });
    assert!(CadPlayApp::original_host_configuration("setActiveExample").is_none());
}
