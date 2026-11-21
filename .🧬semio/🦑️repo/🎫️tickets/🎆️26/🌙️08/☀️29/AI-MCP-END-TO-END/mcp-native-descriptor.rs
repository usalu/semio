//! 🛂️ Emits one catalog source pair from the installed native form of the actual plugin bundle.

use semio_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};
use semio_plugin::{MountedOwnerPolicyV1, PackageDescriptor, Plugin, PluginApp, PluginAssemblyError};
use semio_plugin_describe::{declared_artifact_kind_pairs, descriptor_kind_identity_apps, kind_identity_faults};
use semio_framework_value::{FromValue, ToValue};
use std::{env, fs, path::PathBuf};

async fn describe<PA: PluginApp>(owner: &str, policy: MountedOwnerPolicyV1, bundle: Result<Plugin<PA>, PluginAssemblyError>) -> (String, Vec<u8>) {
    let runtime = PluginRuntime::<PA>::new(policy).expect("mounted owner policy");
    install_plugin_bundle_result(&runtime, Ok(bundle.expect("actual plugin bundle")));
    let pack = semio_plugin::describe::describe_plugin(&runtime).await;
    let value = semio_os_kernel::pack_rt::decode_wire_value(&pack).expect("owned descriptor Pack");
    let descriptor: PackageDescriptor = FromValue::from_value(value).expect("current typed package descriptor");
    assert_eq!(descriptor.manifest.plugin_id, owner);
    assert_eq!(descriptor.package_id, format!("semio:{owner}"));
    assert_eq!(pack, semio_plugin::describe::encode_package_descriptor(&descriptor));

    let kind_faults = kind_identity_faults(&descriptor_kind_identity_apps(&descriptor));
    assert!(kind_faults.is_empty(), "{}", kind_faults.join("\n"));
    let kinds = declared_artifact_kind_pairs(&descriptor);
    let mut owns_codec = kinds.is_empty();
    for (_, schema) in kinds {
        if let Ok(hash) = semio_plugin::plugin_artifact_pack_schema_hash(&runtime, &schema).await {
            if hash != [0; 32] {
                owns_codec = true;
                break;
            }
        }
    }
    assert!(owns_codec, "the installed bundle must own an advertised codec");

    let value = descriptor.to_value();
    let json = semio_framework_pack_json::to_string_pretty(&semio_framework_pack_json::from_dsl_value(&value));
    (json + "\n", pack)
}

fn main() -> std::io::Result<()> {
    let mut arguments = env::args_os().skip(1);
    let owner = arguments.next().expect("owner ID");
    let owner = owner.to_str().expect("text owner ID");
    let output = PathBuf::from(arguments.next().expect("scratch output directory"));
    assert!(arguments.next().is_none());
    let (json, pack) = match owner {
        #[cfg(feature = "dag")]
        "dag" => semio_framework_async::block_on(describe(owner, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_dag::plugin::plugin())),
        #[cfg(feature = "reasoning")]
        "reasoning" => semio_framework_async::block_on(describe(owner, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_reasoning::plugin::plugin())),
        #[cfg(feature = "remodel")]
        "remodel" => semio_framework_async::block_on(describe(owner, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_remodel::plugin::plugin())),
        #[cfg(feature = "trinity")]
        "trinity" => semio_framework_async::block_on(describe(owner, MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_hub_trinity::plugin())),
        #[cfg(feature = "wfc")]
        "wfc" => semio_framework_async::block_on(describe(owner, semio_hub_wfc::WFC_MOUNTED_OWNER_POLICY, semio_hub_wfc::plugin())),
        _ => panic!("owner feature and request must match"),
    };
    fs::create_dir_all(&output)?;
    fs::write(output.join("🔣️.json"), json)?;
    fs::write(output.join("🛂️.descriptor.semio"), pack)?;
    Ok(())
}
