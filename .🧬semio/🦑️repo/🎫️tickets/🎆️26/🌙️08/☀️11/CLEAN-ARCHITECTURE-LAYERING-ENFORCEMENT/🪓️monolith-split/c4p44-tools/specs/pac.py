from specs.lib import *

ORIG = open(os.path.join(T, 'orig', 'pac.rs'), encoding='utf-8').read()
CUTS = [(373, 374, [631, 803, 931])]

CH = [
    ch('🧱', 'channel-record', 'channel_record', [(30, 108)],
       'Channel record of the plugin artifact channel: the channel with its guest runtime, session and budget state, and the lazily built inference route.'),
    ch('📬', 'pending-exchanges-and-agent-ops', 'pending_exchanges_and_agent_ops', [(110, 287)],
       'Pending exchanges of the plugin artifact channel: the pending command exchange, its response page, the headless agent operation filter, the abandoned exchange close and the exchange registry.'),
    ch('💰', 'budgets-and-limits', 'budgets_and_limits', [(289, 371)],
       'Budgets and limits of the plugin artifact channel: the inference ordinal and ceilings, the headless open, command and inference budgets, the wall budgets and the shared compiled component.'),
    ch('📂', 'documents-and-backbone', 'documents_and_backbone', [(373, 630)],
       'Documents and backbone of the plugin artifact channel: construction, backbone egress, the session document load, reseed and the receive path.'),
    ch('💡', 'inference-route-and-actions', 'inference_route_and_actions', [(632, 805)],
       'Inference route and actions of the plugin artifact channel: the inference route, real inference, the addressed action invocation and the typed faults.'),
    ch('🧩', 'instances', 'instances', [(807, 936)],
       'Instances of the plugin artifact channel: instance discard, open and scoped open, the real exchange entry and the sequence counter.'),
    ch('🔁', 'exchange-turns', 'exchange_turns', [(938, 1224)],
       'Exchange turns of the plugin artifact channel: child dependencies, the sequenced exchange, one command turn, reply frame admission and the response wait.'),
]

SPEC = dict(
    orig='pac.rs',
    file=PAC + '/🦀️.rs',
    children=CH,
    pre=split_impls(CUTS),
    pre_extra=impl_extras(ORIG, CUTS),
    external=[],
)
