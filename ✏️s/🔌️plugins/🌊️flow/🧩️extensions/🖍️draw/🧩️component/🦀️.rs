//! 🖍️ The `flow-extension-draw` extension component: the one cdylib of `semio-s-plugin-flow-draw`, holding only its entry point.

semio_plugin::extension_exports!(semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, semio_s_plugin_flow_draw::extension_guest::bundle);
