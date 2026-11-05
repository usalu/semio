//! 🔤️ The `flow-extension-primitive` extension component: the one cdylib of `semio-s-plugin-flow-primitive`, holding only its entry point.

semio_plugin::extension_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_s_plugin_flow_primitive::extension_guest::bundle);
