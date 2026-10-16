#!/usr/bin/env python3
"""📐️ Plugin-host carve plan: structural edits on an in-memory workspace, the crate table, then the rewrite passes of the carve engine."""
import os, sys, re, json, collections
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import ws as wsm, hcmod, carve, cfgeval
from carve import Edits, Carver, Model, REPO, parse_items, attached_start
from helpers import remove_items, drop_mounts, create, append, replace_once, privatize_glob_reexports

H = REPO + '/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/'
PREFIX = 'semio-framework-plugin-host'


def P(rel):
    return H + rel


CRATES = collections.OrderedDict()


def crate(sfx, rel_root, desc):
    name = PREFIX + ('-' + sfx if sfx else '')
    ident = name.replace('-', '_')
    CRATES[ident] = dict(root=P(rel_root), name=name, sfx=sfx, desc=desc)
    return ident


WIT = crate('wit', '🔗️wit-bindings/🦀️.rs', 'WIT bindings of the actor world and the kernel to WIT conversions every plugin-host layer shares')
ENGINE = crate('engine', '🔧️engine/🦀️.rs', 'Shared Wasmtime engine, epoch deadlines, store limiter, compiled-artifact cache, isolated compile and the host worker pool')
OPERATION = crate('operation', '🚪️io/🎛️operation/🦀️.rs', 'Original operation receiving: the inline admission slot, the Wasmtime request bridge and the borrowed return walk')
EFFECTS = crate('effects', '⚡️effects/🦀️.rs', 'Async effect execution: the classifier, the completion sink and the actor scope registry')
IMPORTS = crate('imports', '📥️imports/🦀️.rs', 'Host side of the host-async interface: the awaitable imports of the actor world')
GUEST = crate('guest-runtime', '🎭️guest-runtime/🦀️.rs', 'Guest runtime: the GuestRuntime trait, the scripted mock, the owned interpreter runtime, the Wasmtime runtime and snapshot calls')
WAKE = crate('wake', '🔔️wake/🦀️.rs', 'Original wake currency: the exclusive wake checkout, the embedding root and the driver policy')
RELAY = crate('post-turn-relay', '🔀️post-turn-relay/🦀️.rs', 'Post-turn relay: the guest relay pool, instance slots and leases, cold relay jobs, the mounted registry and the plugin instance handle')
ROUTERS = crate('routers', '🧭️routers/🦀️.rs', 'Routers: IO, inference, plugin graph, mutation, instance directory, transaction coordinator, app and opening resolver')
TRANSPORT = crate('process-transport', '🧵️shard/🚚️process-transport/🦀️.rs', 'Process and stdio shard transports with their length-prefixed framing')
SHARD = crate('shard', '🧵️shard/🦀️.rs', 'ShardLoop: the logic one shard runs, its identity, grant, lifecycle and event custody')
SHARD_LAWS = crate('shard-laws', '🧵️shard/🧪️tests/🏛️laws/🦀️.rs', 'Shard laws: the loop, lifecycle, identity, transport and executor laws over the real shard crates')
GUEST_LAWS = crate('guest-laws', '🧪️tests/🏛️guest-laws/🦀️.rs', 'Guest runtime laws: the owned and Wasmtime runtime, patch ownership, snapshot refusal and shared engine laws over the real guest runtime')
EXECUTOR = crate('shard-executor', '🧵️shard/🧵️executor/🦀️.rs', 'ShardExecutor: shard loops scheduled as jobs onto the shared worker pool')
FACADE = crate('', '📦️packages/🦀️rust/🦀️.rs', 'Sandboxed Wasmtime component plugin host')

BINDINGS_NAMES = ['actor_bindings', 'wit_capabilities', 'wit_effects', 'wit_events', 'wit_host_async', 'wit_lifetime', 'wit_types', 'wit_ui', 'wit_jobs', 'wit_reactor']

ENGINE_ITEMS = ['SharedEngineConfig', 'CORE_INSTANCES_PER_COMPONENT', 'MEMORIES_PER_COMPONENT', 'TABLES_PER_COMPONENT', 'build_shared_engine', 'plugin_host_worker_pool', 'PeriodicPoolTimer', 'EpochDeadlines', 'EpochDeadlinesInner', 'EpochDemands', 'EpochDeadlineCell', 'EpochDemand', 'EpochDemandKind', 'BudgetLimiter', 'default_compiled_cache_root', 'shared_engine_config_hash', 'hex_encode', 'compiled_cache_path', 'load_compiled_component', 'store_compiled_component', 'write_compiled_cache_entry', 'compiled_cache_scratch_path', 'IsolatedCompileTarget', 'compile_component_isolated', 'minimal_component_without_actor_world']

CONVERSION_ITEMS = ['decode_dsl', 'decode_json', 'encode_json', 'wit_message_endpoint_to_kernel', 'kernel_message_endpoint_to_wit', 'kernel_request_outcome_to_wit', 'wit_turn_status_to_kernel', 'kernel_command_page_to_wit', 'wit_command_cursor_to_kernel', 'kernel_command_cursor_to_wit', 'wit_command_ingress_to_kernel', 'wit_effect_to_kernel', 'kernel_message_endpoint_to_wit_reverse', 'wit_surface_ref', 'kernel_lifetime_to_wit', 'wit_lifetime_to_kernel', 'kernel_patch_receipt_to_wit', 'wit_patch_receipt_to_kernel', 'kernel_lifecycle_receipt_to_wit', 'wit_lifecycle_receipt_to_kernel', 'kernel_turn_inputs_to_wit', 'kernel_event_to_wit', 'kernel_activation_event_to_wit', 'kernel_capability_change_to_wit', 'kernel_broker_grant_to_wit', 'wit_snapshot_retirement', 'wit_snapshot_rejection', 'wit_snapshot_limits', 'wit_snapshot_grant']


def read(ws, f):
    return ws.read(f)


def insert_header(ws, file, text):
    src = ws.read(file)
    toks = hcmod.lex_cached(src)
    pos = Carver.header_end(None, src, toks)
    ws.write(file, src[:pos] + '\n' + text.strip('\n') + '\n' + src[pos:])


def cut_text(ws, file, start_marker, end_marker, include_end=False):
    s = ws.read(file)
    assert s.count(start_marker) == 1, (file, start_marker[:60], s.count(start_marker))
    a = s.index(start_marker)
    b = s.index(end_marker, a) + (len(end_marker) if include_end else 0)
    block = s[a:b]
    ws.write(file, s[:a] + s[b:])
    return block


def structural(ws, E):
    R = P('🦀️.rs')
    G = P('🎭️guest-runtime/🦀️.rs')
    # ------------------------------------------------------------------ the standalone test include becomes inline items
    inc = P('🧪️tests/🔬️standalone/🦀️.rs')
    replace_once(ws, R, '#[cfg(test)]\ninclude!("🧪️tests/🔬️standalone/🦀️.rs");', ws.read(inc).rstrip('\n'))
    ws.delete(inc)
    # ------------------------------------------------------------------ engine
    create(ws, CRATES[ENGINE]['root'], '''//! 🔧️ The shared Wasmtime engine of the plugin host: one pooled engine per process, demand-driven epoch deadlines, the per-store resource limiter, the compiled-artifact cache, isolated compilation and the host worker pool.

#[path = "../⏱️deadline/🦀️.rs"]
pub mod original_deadline;
''')
    E.move(R, ENGINE_ITEMS, CRATES[ENGINE]['root'])
    E.proxy[CRATES[ENGINE]['root']] = R
    # ------------------------------------------------------------------ wit bindings: bindgen module, aliases, conversions, wire modules
    wit_root = CRATES[WIT]['root']
    conv = P('🔗️wit-bindings/🔀️conversion/🦀️.rs')
    create(ws, conv, '//! 🔀️ Conversions between the kernel vocabulary and the generated WIT types of the actor world.\n')
    E.move(G, CONVERSION_ITEMS, conv)
    E.proxy[conv] = G
    replace_once(ws, G, 'the owned interpreter runtime and the wasmtime runtime with its generated actor bindings.', 'the recording double, the owned interpreter runtime, the wasmtime runtime, snapshot calls and the component document codec.')
    bind_block = cut_text(ws, G, 'pub(crate) mod actor_bindings {', '\n}\n', include_end=True)
    bind_block = bind_block.replace('pub(crate) mod actor_bindings', 'pub mod actor_bindings').replace('path: "../../../🧬️schema"', 'path: "../../../../🧬️schema"')
    aliases1 = cut_text(ws, G, 'pub(crate) use actor_bindings::semio::framework::{capabilities as wit_capabilities', ';\n', include_end=True)
    aliases2 = cut_text(ws, G, 'pub(super) use actor_bindings::exports::semio::framework::{jobs as wit_jobs', ';\n', include_end=True)
    create(ws, wit_root, '''//! 🔗️ WIT bindings of the actor world and the kernel to WIT conversions every plugin-host layer shares: the crate's one `bindgen!` invocation, its aliases, the cold pair, UI patch and retained turn wire modules.

''' + bind_block.rstrip('\n') + '\n\n' + aliases1.replace('pub(crate) use', 'pub use').rstrip('\n') + '\n' + aliases2.replace('pub(super) use', 'pub use').rstrip('\n') + '''

#[path = "🔀️conversion/🦀️.rs"]
pub mod conversion;
#[path = "../📥️cold-pair/🦀️.rs"]
pub mod cold_pair;
#[path = "../📥️ui-patch/🦀️.rs"]
pub mod ui_patch;
#[path = "../🧬️component-codec/🎟️retained-turn/🦀️.rs"]
pub mod retained_turn_wire;
''')
    E.inline_map[(G, ('actor_bindings',))] = (wit_root, ('actor_bindings',))
    E.proxy[wit_root] = G
    # G: cold pair mount goes to the wit crate; snapshot, codec and the generated trait impls come in
    drop_mounts(ws, G, ['cold_pair', 'service_operation_tests', 'owned_instance_open_tests', 'wasmtime_runtime_tests'], glob_reexports=False)
    create(ws, CRATES[GUEST_LAWS]['root'], '''//! 🏛️ Guest runtime laws: the owned and Wasmtime runtime, patch ownership, snapshot refusal and shared engine laws over the real guest runtime.

#[cfg(test)]
#[path = "../🔬️owned-instance-open/🦀️.rs"]
mod owned_instance_open_tests;

#[cfg(test)]
#[path = "../🔬️wasmtime-runtime/🦀️.rs"]
mod wasmtime_runtime_tests;

#[cfg(test)]
#[path = "../../📥️ui-patch/🧪️tests/🧪️component/🦀️.rs"]
mod ui_patch_component_tests;

#[cfg(test)]
#[path = "../🪶️sqlite/⚠️refusal/🦀️.rs"]
mod sqlite_refusal_tests;

#[cfg(test)]
#[path = "../🔬️shared-wasmtime-engine/🦀️.rs"]
mod shared_wasmtime_engine_tests;
''')
    E.proxy[CRATES[GUEST_LAWS]['root']] = G
    append(ws, wit_root, '#[cfg(test)]\n#[path = "../🧪️tests/💡️service-operation/🦀️.rs"]\nmod service_operation_tests;')
    remove_items(ws, G, lambda it, toks: it.kind == 'use' and 'cold_pair' in ''.join(t.x for t in it.toks_all))
    insert_header(ws, G, '''#[path = "../🚪️io/🪶️snapshot/🦀️.rs"]
mod snapshot_call;
pub use snapshot_call::{SnapshotCall, SnapshotCallOutput, SnapshotCallStatus, SnapshotComponentReceipt};
#[path = "../🧬️component-codec/🦀️.rs"]
mod component_codec;
pub use component_codec::{OwnedComponentDocumentCodec, GUEST_CODEC_BUDGET};
''')
    E.move(R, [], G, with_impls=False, impl_idents=['wasmtime'])
    # ------------------------------------------------------------------ operation
    OP = P('🚪️io/🎛️operation/🦀️.rs')
    replace_once(ws, OP, 'mod return_types{pub use crate::actor_bindings::semio::framework::{effects,ui,types,pure};pub use crate::actor_bindings::exports::semio::framework::{reactor,codec};}', 'mod return_types{pub use semio_framework_plugin_host_wit::actor_bindings::semio::framework::{effects,ui,types,pure};pub use semio_framework_plugin_host_wit::actor_bindings::exports::semio::framework::{reactor,codec};}')
    replace_once(ws, OP, 'pub(crate) use semio_framework_os_vcs::io::binary::entity_identity::control::{OriginalOperationReceiver, OriginalOperationReceiving};', 'pub use semio_framework_os_vcs::io::binary::entity_identity::control::{OriginalOperationReceiver, OriginalOperationReceiving};')
    insert_header(ws, OP, '#[cfg(test)]\n#[path = "../../🧪️tests/🛂️receiving/🦀️.rs"]\npub(crate) mod test_native_authority;')
    # ------------------------------------------------------------------ imports
    IM = P('📥️imports/🦀️.rs')
    replace_once(ws, IM, 'pub(crate) use super::actor_bindings as host_async_bindings;', 'pub(crate) use semio_framework_plugin_host_wit::actor_bindings as host_async_bindings;')
    # ------------------------------------------------------------------ wake and relay
    RL = P('🔀️post-turn-relay/🦀️.rs')
    WK = P('🔔️wake/🦀️.rs')
    E.move(RL, ['GuestRelayWakeReceipt', 'GuestRelayWakeReceiptReceiver'], WK)
    drop_mounts(ws, RL, ['original_wake_receipts'], glob_reexports=False)
    E.move(R, ['wait_for_scripted_guest_relay_release'], RL)
    orphan_doc = cut_text(ws, R, '/// 🧪️ Only `host_transaction_coordinator_tests`', '//#region 📈️RuntimeMetricsPublisher')
    E.move(R, ['host_fault_bytes'], P('🧭️routers/🦀️.rs'))
    replace_once(ws, P('🧭️routers/🦀️.rs'), '#[cfg_attr(not(test), allow(dead_code))]\n// 🚫️async: E1 — pure in-memory encoder', orphan_doc + '#[cfg_attr(not(test), allow(dead_code))]\n// 🚫️async: E1 — pure in-memory encoder')
    # ------------------------------------------------------------------ shard family
    SH = P('🧵️shard/🦀️.rs')
    UNIT = P('🧵️shard/🧪️tests/🔬️unit/🦀️.rs')
    drop_mounts(ws, SH, ['executor', 'tests'], glob_reexports=False)
    E.move(UNIT, ['LoopbackProbe'], SH)
    E.move(SH, ['RecordingRuntime'], G)
    for f in (P('🧵️shard/🔁️lifecycle/🦀️.rs'), P('🧵️shard/🪪️identity/🦀️.rs'), P('🧵️shard/🚚️process-transport/🦀️.rs')):
        drop_mounts(ws, f, ['tests'], glob_reexports=False)
    create(ws, CRATES[SHARD_LAWS]['root'], '''//! 🏛️ Shard laws: the loop, lifecycle, identity, transport and executor laws over the real shard crates.

#[cfg(test)]
#[path = "../🔬️unit/🦀️.rs"]
mod unit;

#[cfg(test)]
#[path = "../../🔁️lifecycle/🧪️tests/🔁️lifecycle/🦀️.rs"]
mod lifecycle_tests;

#[cfg(test)]
#[path = "../../🪪️identity/🧪️tests/🦀️.rs"]
mod identity_tests;

#[cfg(test)]
#[path = "../../🚚️process-transport/🧪️tests/🔬️unit/🦀️.rs"]
mod process_transport_tests;
''')
    E.proxy[CRATES[SHARD_LAWS]['root']] = SH
    # ------------------------------------------------------------------ engine tests and the facade component
    drop_mounts(ws, R, ['operation', 'test_native_authority', 'snapshot_call', 'original_deadline', 'shard', 'process_transport', 'effects', 'imports', 'ui_patch', 'ui_patch_component_tests', 'component_codec', 'retained_turn_wire', 'shared_wasmtime_engine_tests', 'guest_runtime', 'post_turn_relay', 'routers', 'sqlite_refusal_tests'])
    remove_items(ws, R, lambda it, toks: it.kind == 'use' and any(k in ''.join(t.x for t in it.toks_all) for k in ('snapshot_call', 'component_codec', 'original_deadline')))
    ACT = P('🎠️activation/🧪️tests/🎠️activation/🦀️.rs')
    s_act = ws.read(ACT)
    assert s_act.count('crate::component::wit_events::') == 2
    ws.write(ACT, s_act.replace('crate::component::wit_events::', 'semio_framework_plugin_host_wit::wit_events::'))
    BIN = P('🧵️shard/👶️child/🦀️.rs')
    replace_once(ws, BIN, 'use semio_framework_plugin_host::process_transport::StdioTransport;\nuse semio_framework_plugin_host::shard::{ShardLoop, ShardTransports};\nuse semio_framework_plugin_host::{GuestRuntime, GuestRuntimes, PackageHash, PackageId, PackageRef};', 'use semio_framework_plugin_host_guest_runtime::{GuestRuntime, GuestRuntimes, PackageRef};\nuse semio_framework_plugin_host_process_transport::StdioTransport;\nuse semio_framework_plugin_host_shard::{ShardLoop, ShardTransports, native_identity_issuer};\nuse semio_framework_actor::{PackageHash, PackageId};')
    replace_once(ws, BIN, 'semio_framework_plugin_host::shard::native_identity_issuer(', 'native_identity_issuer(')
    MT = P('🧪️tests/🔬️runtime-metrics-publisher/🦀️.rs')
    replace_once(ws, MT, '''/// for why `include_str!` (not `std::fs`) is how a `#[cfg(test)]` block reads it without tripping
/// the crate-purity grep this repo's acceptance criteria runs unconditionally over `component.rs`
/// files (this one is the HOST, not the pure `🎭️actor` crate, but the same discipline is followed
/// here since the file is right next to it and easy to mistake for one).
const SCALE_FIXTURE_REGISTRY_JSON: &str = include_str!("../../../../../🧪️testing/⚖️scale/🤖️generated/📇️registry/🔣️.json");''', '''/// for why the registry is read when the law runs and not embedded: its 1.7 MB would count against the crate's source boundary.
fn scale_fixture_registry_json() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../🧪️testing/⚖️scale/🤖️generated/📇️registry/🔣️.json");
    std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("scale fixture registry {} is not readable: {error}", path.display()))
}''')
    replace_once(ws, MT, 'serde_json::from_str(SCALE_FIXTURE_REGISTRY_JSON)', 'serde_json::from_str(&scale_fixture_registry_json())')
    replace_once(ws, P('🧪️tests/🔬️unit/🦀️.rs'), 'use super::*;\n', '')


def prepare_roots():
    return collections.OrderedDict((k, v['root']) for k, v in CRATES.items())


def main():
    base_ws = wsm.WS()
    hcmod.FS = base_ws
    base = Model(base_ws, collections.OrderedDict([('semio_framework_plugin_host', P('📦️packages/🦀️rust/🦀️.rs'))]), allow_dup=True)
    ws = wsm.WS()
    E = Edits(ws)
    structural(ws, E)
    return ws, E, base, base_ws


def external_idents():
    return set()


def prepare(stop=None):
    import fixers
    ws, E, base, base_ws = main()
    roots = prepare_roots()
    C = Carver(ws, E, base)
    C.build_new(roots)
    C.pass1()
    C.build_new(roots)
    C.pass2()
    C.build_new(roots)
    C.fix_log = {}
    C.fix_log['opaque'] = fixers.fix_opaque(ws, C, roots)
    C.fix_log['root'] = fixers.fix_root_names(ws, C, roots, None)
    C.fix_log['traits'] = fixers.fix_trait_imports(ws, C, roots)
    C.build_new(roots)
    C.pass3(roots)
    C.build_new(roots)
    C.finalize_exports()
    C.build_new(roots)
    C.open_test_imports()
    C.build_new(roots)
    C.flip_not_test_partners()
    C.build_new(roots)
    import scan
    found = scan.member_scan(C.new)
    C.widen_members(found)
    C.member_findings = found
    C.build_new(roots)
    import autovis
    applied, missed = autovis.apply_decisions(ws, C, autovis.load_decisions())
    C.vis_log = (applied, missed)
    C.build_new(roots)
    C.fix_log['async'] = fixers.fix_async_trait_allow(ws, C, roots)
    C.fix_log['prune'] = fixers.prune_unused_uses(ws, C, roots, P('🦀️.rs'))
    import facade_gen
    froot = CRATES[FACADE]['root']
    ws.write(froot, facade_gen.generate_text(C, ws.read(froot)))
    C.build_new(roots)
    for p in list(ws.cur):
        if p.endswith('.rs') and p not in ws.deleted:
            t = fixers.tidy_regions(ws.cur[p])
            if t != ws.cur[p]:
                ws.write(p, t)
    C.build_new(roots)
    return ws, C, roots, base_ws


if __name__ == '__main__':
    ws, C, roots, base_ws = prepare()
    print('missing mounts:', [m.path for m in C.new.missing])
    C.report.dump()
    print('changed files', len(ws.changed()))
