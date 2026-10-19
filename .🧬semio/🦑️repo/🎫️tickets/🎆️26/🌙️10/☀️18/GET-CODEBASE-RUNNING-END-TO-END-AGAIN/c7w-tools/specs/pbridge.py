from specs.lib import *
F = R + '/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs'
c = Cut(F, 'pbridge.rs')
c.child('🧳', 'wasm-program-exchange', 'wasm_program_exchange_child', [(36, 548)],
        'Wasm program exchange: the document load cancellation constant and the native `wasm_program_exchange` module that hosts wasm C-ABI programs.')
c.child('🌉', 'program-bridge-entry', 'program_bridge_entry', [(554, 1078)],
        'Program bridge entry: the program identity and ephemeral snapshot, the bridge backend and entry, the fixture document, the `ProgramBridgeEntry` methods and the window, tool and engagement section helpers.')
c.child('📡', 'js-calls-and-operation-publications', 'js_calls_and_operation_publications', [(1092, 1472)],
        'Js calls and operation publications: the JS rejection and call helpers, the document backbone JS doors, the browser ephemeral and component codec modules, the operation publications and the action, command and context menu JS doors.')
c.child('🧰', 'document-render-plugin-install-and-native-loading', 'document_render_plugin_install_and_native_loading', [(1474, 1849)],
        'Document render, plugin install and native loading: the document render JS door, the browser value retirement, the invocation packs, the plugin entry parsing, the JS plugin and extension install doors, the plugin host config and the native plugin loading.')
SPEC = c.spec(nest=True, patch={
    '🧳️wasm-program-exchange': [('include!("../../🧪️tests/', 'include!("../../../🧪️tests/')],
    '📡️js-calls-and-operation-publications': [
        ('pub(super) async fn observe(', 'pub(in super::super) async fn observe('),
        ('pub(super) fn snapshot(', 'pub(in super::super) fn snapshot('),
        ('pub(super) fn forget(', 'pub(in super::super) fn forget('),
        ('pub(super) fn register(', 'pub(in super::super) fn register('),
    ]})
