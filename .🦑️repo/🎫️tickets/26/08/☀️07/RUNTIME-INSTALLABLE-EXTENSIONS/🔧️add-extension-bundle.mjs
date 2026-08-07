import fs from "fs";

const pluginPath = process.argv[2];
let src = fs.readFileSync(pluginPath, "utf8");

if (src.includes("pub struct ExtensionBundle")) {
  console.log("ExtensionBundle already present");
  process.exit(0);
}

// Expand plugin_runtime imports
const oldImport = `    use semio_framework_core::{
        kernel::{HostEffect, InvocationResult},
        Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest, ViewModel,
    };`;
const newImport = `    use semio_framework_core::{
        kernel::{CapabilityRequirement, HostEffect, InvocationResult},
        Contribution, Fault, FaultCode, FaultFrom, FaultOrigin, PluginManifest, ViewModel,
    };
    use std::collections::HashMap;`;
if (!src.includes(oldImport)) {
  console.error("plugin_runtime import block not found");
  process.exit(1);
}
src = src.replace(oldImport, newImport);

const insertAfter = `    macro_rules! plugin_exports {
        ($bundle_fn:expr) => {
            fn __semio_install_plugin_bundle() {
                $crate::plugin_runtime::install_plugin_bundle(($bundle_fn)());
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_plugin_bundle_installer_link_shim() {
                $crate::plugin_runtime::register_plugin_bundle_installer(__semio_install_plugin_bundle);
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_plugin_install_bundle() {
                __semio_install_plugin_bundle();
            }

            #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
            #[used]
            static _SEMIO_PLUGIN_COMPONENT_LINK: fn() = $crate::component_export_anchor;
        };
    }
`;

if (!src.includes(insertAfter)) {
  console.error("plugin_exports block not found for insert");
  process.exit(1);
}

const extensionRegion = `    //#region 🧩️Extension
    /// 🧩️ Extension guest bundle — no apps; contributes + invoke handlers.
    pub struct ExtensionBundle {
        pub manifest: ExtensionManifest,
        handlers: HashMap<String, Box<dyn Fn(&[u8]) -> Result<Vec<u8>, Fault> + Send + 'static>>,
    }

    /// 📦️ Manifest for a runtime-installable extension (WIT \`extension::manifest\` payload).
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ExtensionManifest {
        pub extension_id: String,
        pub label: String,
        pub version: String,
        pub extends: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub capabilities: Vec<CapabilityRequirement>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub contributions: Vec<Contribution>,
    }

    impl ExtensionBundle {
        /// 🧩️ Starts an extension bundle with identity + version.
        pub fn new(extension_id: impl Into<String>, label: impl Into<String>, version: impl Into<String>) -> Self {
            Self {
                manifest: ExtensionManifest {
                    extension_id: extension_id.into(),
                    label: label.into(),
                    version: version.into(),
                    extends: String::new(),
                    capabilities: Vec::new(),
                    contributions: Vec::new(),
                },
                handlers: HashMap::new(),
            }
        }

        /// 🔗 Declares the host app/plugin this extension extends.
        pub fn extends(mut self, extends: impl Into<String>) -> Self {
            self.manifest.extends = extends.into();
            self
        }

        /// 🔒️ Declares a capability requirement for the extension.
        pub fn capability(mut self, capability: CapabilityRequirement) -> Self {
            if !self.manifest.capabilities.contains(&capability) {
                self.manifest.capabilities.push(capability);
            }
            self
        }

        /// 🧩️ Adds a contribution declaration to the extension manifest.
        pub fn contributes(mut self, contribution: Contribution) -> Self {
            self.manifest.contributions.push(contribution);
            self
        }

        /// 🔀️ Registers a capability handler invoked via WIT \`extension::invoke\`.
        pub fn handler(mut self, capability: impl Into<String>, handler: impl Fn(&[u8]) -> Result<Vec<u8>, Fault> + Send + 'static) -> Self {
            self.handlers.insert(capability.into(), Box::new(handler));
            self
        }
    }

    thread_local! {
        static EXTENSION_BUNDLE: RefCell<Option<ExtensionBundle>> = const { RefCell::new(None) };
        static EXTENSION_ACTIVE: Cell<bool> = const { Cell::new(false) };
    }

    /// 📤️ Installs the process-local extension bundle (from \`extension_exports!\`).
    pub fn install_extension_bundle(bundle: ExtensionBundle) {
        EXTENSION_BUNDLE.with(|slot| {
            *slot.borrow_mut() = Some(bundle);
        });
        EXTENSION_ACTIVE.with(|slot| slot.set(false));
    }

    static EXTENSION_BUNDLE_INSTALLER: std::sync::OnceLock<fn()> = std::sync::OnceLock::new();

    /// 🧩️ Registers the embedding extension crate's bundle installer (expanded from \`extension_exports!\`).
    pub fn register_extension_bundle_installer(install: fn()) {
        let _ = EXTENSION_BUNDLE_INSTALLER.set(install);
    }

    fn ensure_extension_initialized() {
        EXTENSION_BUNDLE.with(|slot| {
            if slot.borrow().is_none() {
                if let Some(install) = EXTENSION_BUNDLE_INSTALLER.get() {
                    install();
                }
            }
        });
    }

    /// 📦️ Returns the installed extension manifest (empty defaults when unset).
    pub fn extension_manifest() -> ExtensionManifest {
        ensure_extension_initialized();
        EXTENSION_BUNDLE.with(|slot| {
            slot.borrow()
                .as_ref()
                .map(|bundle| bundle.manifest.clone())
                .unwrap_or_else(|| ExtensionManifest {
                    extension_id: String::new(),
                    label: String::new(),
                    version: String::new(),
                    extends: String::new(),
                    capabilities: Vec::new(),
                    contributions: Vec::new(),
                })
        })
    }

    /// 🚨️ Marks the extension active for subsequent \`extension_invoke\` calls.
    pub fn extension_activate() -> Result<(), Fault> {
        ensure_extension_initialized();
        let ready = EXTENSION_BUNDLE.with(|slot| slot.borrow().is_some());
        if !ready {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.missing"), "extension bundle not installed"));
        }
        EXTENSION_ACTIVE.with(|slot| slot.set(true));
        Ok(())
    }

    /// 🛑 Clears the active flag without dropping handlers.
    pub fn extension_deactivate() {
        EXTENSION_ACTIVE.with(|slot| slot.set(false));
    }

    /// 🔀️ Dispatches \`capability\` to the registered handler with wire-encoded \`request\` bytes.
    pub fn extension_invoke(capability: &str, request: &[u8]) -> Result<Vec<u8>, Fault> {
        ensure_extension_initialized();
        if !EXTENSION_ACTIVE.with(|slot| slot.get()) {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.inactive"), "extension not activated"));
        }
        EXTENSION_BUNDLE.with(|slot| {
            let bundle = slot.borrow();
            let Some(bundle) = bundle.as_ref() else {
                return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("extension.missing"), "extension bundle not installed"));
            };
            let Some(handler) = bundle.handlers.get(capability) else {
                return Err(Fault::new(
                    FaultOrigin::Plugin,
                    FaultCode::new("extension.unknown-capability"),
                    format!("unknown extension capability '{capability}'"),
                ));
            };
            handler(request)
        })
    }

    /// 🧩️ Installs an extension crate's bundle builder into TLS for WIT guest exports.
    #[macro_export]
    macro_rules! extension_exports {
        ($bundle_fn:expr) => {
            fn __semio_install_extension_bundle() {
                $crate::plugin_runtime::install_extension_bundle(($bundle_fn)());
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_bundle_installer_link_shim() {
                $crate::plugin_runtime::register_extension_bundle_installer(__semio_install_extension_bundle);
            }

            #[doc(hidden)]
            #[unsafe(no_mangle)]
            pub extern "C" fn semio_extension_install_bundle() {
                __semio_install_extension_bundle();
            }

            #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
            #[used]
            static _SEMIO_EXTENSION_COMPONENT_LINK: fn() = $crate::extension_guest_export_anchor;
        };
    }

    //#region 🧩️ExtensionGuest
    /// 🔌️ WIT \`extension-world\` guest wiring lives behind \`feature = "component-guest"\` + wasm32/p2.
    /// Dual-world \`wit-bindgen\` is not generated alongside \`plugin-world\` yet — these anchors + the
    /// public \`extension_*\` APIs are the guest call surface hosts/tests use until a separate bindgen
    /// invocation lands.
    #[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
    pub fn extension_guest_export_anchor() {
        let _ = (
            extension_manifest as fn() -> ExtensionManifest,
            extension_activate as fn() -> Result<(), Fault>,
            extension_deactivate as fn(),
            extension_invoke as fn(&str, &[u8]) -> Result<Vec<u8>, Fault>,
        );
    }

    #[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
    pub fn extension_guest_export_anchor() {}
    //#endregion 🧩️ExtensionGuest
    //#endregion 🧩️Extension

`;

src = src.replace(insertAfter, insertAfter + "\n" + extensionRegion);

// Crate-root re-exports
const oldPubUse = `pub use plugin_runtime::{install_plugin_bundle, plugin_attach_backbone, plugin_detach_backbone, plugin_document_pack, plugin_ingest_operations, plugin_load_document_pack};`;
const newPubUse = `pub use plugin_runtime::{
    extension_activate, extension_deactivate, extension_guest_export_anchor, extension_invoke, extension_manifest, install_extension_bundle,
    install_plugin_bundle, plugin_attach_backbone, plugin_detach_backbone, plugin_document_pack, plugin_ingest_operations, plugin_load_document_pack,
    ExtensionBundle, ExtensionManifest,
};`;
if (!src.includes(oldPubUse)) {
  console.error("pub use plugin_runtime not found");
  process.exit(1);
}
src = src.replace(oldPubUse, newPubUse);

fs.writeFileSync(pluginPath, src);
console.log("ExtensionBundle inserted");
