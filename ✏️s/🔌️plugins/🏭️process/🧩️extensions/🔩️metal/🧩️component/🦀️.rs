//! 🔩️ The `process-extension-metal` extension component: the one cdylib of `semio-s-plugin-process-metal`, holding only its entry point.

semio_framework_plugin::extension_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_s_plugin_process_metal::bundle);
