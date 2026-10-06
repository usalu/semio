# General Workspace Direction Audit

The current root Cargo workspace declares 119 members and none are S or Hub members. Its dependency table still contains two S-named bindings for the first-party `semio-framework-os-flow` crate. Both paths resolve into the General OS flow module, so this observation establishes obsolete names, not a physical dependency on S. The intended canonical boundary is the existing General crate identity.

The initial focused search found the two root dependency declarations and two stale removal comments; a complete filtered consumer search is in progress before proposing retirement. The S workspace remains a separate scope with its own domain declarations. No original physical deletion or whole-root compiler claim is made from this static observation.

Evidence: `🗑️generated/general-specific-current-workspace-observation-1.json`, `general-flow-extension-specific-names-current-1.txt`, and the pending full consumer observation `general-flow-extension-specific-names-current-2.txt`.
