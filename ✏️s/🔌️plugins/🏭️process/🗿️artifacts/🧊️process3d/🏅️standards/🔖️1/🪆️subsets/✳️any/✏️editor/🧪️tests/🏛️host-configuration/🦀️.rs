//! 🏛️ Actual domain classification and owned native host configuration projection.
use semio_framework_plugin::ArtifactEditor;
use semio_framework_plugin_host_configuration::{with_original_host_configuration,testkit::HeapWitness};
use semio_framework_value::DslValue;

#[global_allocator]
static HOST_CONFIGURATION_HEAP: HeapWitness = HeapWitness;

use semio_s_artifact_process_process3d_editor::Process3dPlayApp;
use semio_s_artifact_process_process3d_config::{Process3dConfigMutation,Process3dConfigSetContributions};

#[test]
fn original_host_configuration_preserves_process_installable_config_lane() {
    let input=r#"[{"pluginId":"process-wood","topicContribution":{"topic":"process.machines","payload":{"appId":"process3d-play","moduleId":"wood","label":"Wood","iconId":"beam","machinesJson":"[]"}}},{"pluginId":"foreign","topicContribution":{"topic":"other.topic","payload":{}}}]"#;
    let expected=r#"[{"pluginId":"process-wood","topicContribution":{"topic":"process.machines","payload":{"appId":"process3d-play","moduleId":"wood","label":"Wood","iconId":"beam","machinesJson":"[]"}}}]"#;
    assert_ne!(expected,"[]");
    assert!(expected.len()<input.len());
    let declaration=Process3dPlayApp::original_host_configuration("setContributions").expect("declared original host source");
    with_original_host_configuration(declaration,Some(DslValue::object([("json".to_owned(),DslValue::String(input.to_owned()))])),|mutation|{
        assert_eq!(mutation,&Process3dConfigMutation::SetContributions(Process3dConfigSetContributions{json:expected.to_owned()}));
    });
    assert!(Process3dPlayApp::original_host_configuration("setCursor").is_none());
}
