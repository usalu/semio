//! 🧱️ The `process-extension-concrete` extension component: the one cdylib of `semio-s-plugin-process-concrete`, holding only its entry point.

semio_framework_plugin::extension_exports!(semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_s_plugin_process_concrete::bundle);
