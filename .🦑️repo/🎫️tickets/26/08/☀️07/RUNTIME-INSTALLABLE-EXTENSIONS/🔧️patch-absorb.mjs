import fs from "fs";
const path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️component.rs";
let text = fs.readFileSync(path, "utf8");

const absorbOld = `        if other.install_program.is_some() {
            self.install_program = other.install_program;
        }
        if other.uninstall_program.is_some() {
            self.uninstall_program = other.uninstall_program;
        }
    }
}`;

const absorbNew = `        if other.install_program.is_some() {
            self.install_program = other.install_program;
        }
        if other.uninstall_program.is_some() {
            self.uninstall_program = other.uninstall_program;
        }
        if other.install_extension.is_some() {
            self.install_extension = other.install_extension;
        }
        if other.uninstall_extension_id.is_some() {
            self.uninstall_extension_id = other.uninstall_extension_id;
        }
        if other.set_extension_enabled_id.is_some() {
            self.set_extension_enabled_id = other.set_extension_enabled_id;
            self.set_extension_enabled = other.set_extension_enabled;
        }
    }
}`;

if (!text.includes(absorbOld)) throw new Error("absorbOld not found");
text = text.replace(absorbOld, absorbNew);

const diffOld = `            SpaceOperation::InstallProgram { plugin_id } => diff.install_program = Some(plugin_id.clone()),
            SpaceOperation::UninstallProgram { plugin_id } => diff.uninstall_program = Some(plugin_id.clone()),
        }
        diff
    }`;

const diffNew = `            SpaceOperation::InstallProgram { plugin_id } => diff.install_program = Some(plugin_id.clone()),
            SpaceOperation::UninstallProgram { plugin_id } => diff.uninstall_program = Some(plugin_id.clone()),
            SpaceOperation::InstallExtension { extension_id, version, source_uri, package_hash, enabled } => {
                diff.install_extension = Some(InstalledExtension {
                    extension_id: extension_id.clone(),
                    version: version.clone(),
                    source_uri: source_uri.clone(),
                    package_hash: package_hash.clone(),
                    enabled: *enabled,
                });
            }
            SpaceOperation::UninstallExtension { extension_id } => diff.uninstall_extension_id = Some(extension_id.clone()),
            SpaceOperation::SetExtensionEnabled { extension_id, enabled } => {
                diff.set_extension_enabled_id = Some(extension_id.clone());
                diff.set_extension_enabled = Some(*enabled);
            }
        }
        diff
    }`;

if (!text.includes(diffOld)) throw new Error("diffOld not found");
text = text.replace(diffOld, diffNew);

const backwardsOld = `            SpaceOperation::InstallProgram { plugin_id } => {
                if base.programs.contains(plugin_id) {
                    Vec::new()
                } else {
                    vec![SpaceOperation::UninstallProgram { plugin_id: plugin_id.clone() }]
                }
            }
            SpaceOperation::UninstallProgram { plugin_id } => {
                if base.programs.contains(plugin_id) {
                    vec![SpaceOperation::InstallProgram { plugin_id: plugin_id.clone() }]
                } else {
                    Vec::new()
                }
            }
        }
    }`;

const backwardsNew = `            SpaceOperation::InstallProgram { plugin_id } => {
                if base.programs.contains(plugin_id) {
                    Vec::new()
                } else {
                    vec![SpaceOperation::UninstallProgram { plugin_id: plugin_id.clone() }]
                }
            }
            SpaceOperation::UninstallProgram { plugin_id } => {
                if base.programs.contains(plugin_id) {
                    vec![SpaceOperation::InstallProgram { plugin_id: plugin_id.clone() }]
                } else {
                    Vec::new()
                }
            }
            SpaceOperation::InstallExtension { extension_id, .. } => match base.extensions.iter().find(|existing| &existing.extension_id == extension_id) {
                Some(existing) => vec![SpaceOperation::InstallExtension {
                    extension_id: existing.extension_id.clone(),
                    version: existing.version.clone(),
                    source_uri: existing.source_uri.clone(),
                    package_hash: existing.package_hash.clone(),
                    enabled: existing.enabled,
                }],
                None => vec![SpaceOperation::UninstallExtension { extension_id: extension_id.clone() }],
            },
            SpaceOperation::UninstallExtension { extension_id } => base
                .extensions
                .iter()
                .find(|existing| &existing.extension_id == extension_id)
                .map(|existing| {
                    vec![SpaceOperation::InstallExtension {
                        extension_id: existing.extension_id.clone(),
                        version: existing.version.clone(),
                        source_uri: existing.source_uri.clone(),
                        package_hash: existing.package_hash.clone(),
                        enabled: existing.enabled,
                    }]
                })
                .unwrap_or_default(),
            SpaceOperation::SetExtensionEnabled { extension_id, .. } => base
                .extensions
                .iter()
                .find(|existing| &existing.extension_id == extension_id)
                .map(|existing| {
                    vec![SpaceOperation::SetExtensionEnabled {
                        extension_id: extension_id.clone(),
                        enabled: existing.enabled,
                    }]
                })
                .unwrap_or_default(),
        }
    }`;

if (!text.includes(backwardsOld)) throw new Error("backwardsOld not found");
text = text.replace(backwardsOld, backwardsNew);

fs.writeFileSync(path, text);
console.log("patched absorb/diff/backwards");
