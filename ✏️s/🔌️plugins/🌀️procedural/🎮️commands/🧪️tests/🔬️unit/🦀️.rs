//! 🔬️ The plugin-scope command facet's own laws: the declared roster reaches
//! `PluginManifest::commands`, and its handler answers with exactly the roster the plugin installs.

use super::*;

/// 🎛️ The command must arrive on the built manifest — a `plugin_command` that is declared but never
/// chained onto the builder is invisible to the palette and to `handle_plugin_command`.
#[test]
fn list_flow_extensions_is_declared_on_the_built_manifest() {
    let plugin = crate::plugin().expect("procedural plugin manifest should build synchronously");
    let declared: Vec<&str> = plugin.manifest.commands.iter().map(|command| command.id.as_str()).collect();
    assert_eq!(declared, vec![LIST_FLOW_EXTENSIONS]);
}

/// 🌊️ The answer is the same roster `plugin()` installs, in declaration order — the point of the
/// command is that a focused 2D or 3D editor can ask which extensions its flow host will resolve.
#[test]
fn list_flow_extensions_answers_the_installed_roster() {
    let invocation = ManifestCommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::Plugin { plugin_id: "procedural".into() }, command_id: LIST_FLOW_EXTENSIONS.into() }, arguments: Default::default() };
    let meta = ActionMeta { actor: "test".into(), instance_id: 1, view_state: None };
    let result = list_flow_extensions(&invocation, &meta).expect("the roster command answers");
    let DslValue::Array(rows) = &result.output else { panic!("the roster is an array, got {:?}", result.output) };
    assert_eq!(rows.len(), crate::FLOW_EXTENSIONS.len());
    assert!(result.mutations.is_empty(), "a View-kind command must not emit mutations");
    let first = rows.first().expect("nine extensions are installed");
    assert_eq!(first.get("id").and_then(DslValue::as_str), Some("s.procedural.flow-extension.brep"));
    assert_eq!(first.get("extension").and_then(DslValue::as_str), Some("brep"));
    let ids: Vec<&str> = rows.iter().filter_map(|row| row.get("id").and_then(DslValue::as_str)).collect();
    assert_eq!(ids.len(), crate::FLOW_EXTENSIONS.len(), "every row carries an id");
}

/// 🔁️ The plugin's own dispatcher accepts the command through the manifest definition it declared.
#[test]
fn the_plugin_dispatches_its_own_scope_command() {
    let plugin = crate::plugin().expect("procedural plugin manifest should build synchronously");
    let invocation = ManifestCommandInvocation { address: CommandAddress { owner: CommandOwnerAddress::Plugin { plugin_id: "procedural".into() }, command_id: LIST_FLOW_EXTENSIONS.into() }, arguments: Default::default() };
    let result = plugin.handle_plugin_command(&invocation, &ActionMeta { actor: "test".into(), instance_id: 1, view_state: None }).expect("the plugin owns this command");
    let DslValue::Array(rows) = &result.output else { panic!("the roster is an array") };
    assert_eq!(rows.len(), crate::FLOW_EXTENSIONS.len());
}
