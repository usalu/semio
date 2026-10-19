from specs.lib import *
F = R + '/🧱️elements/🔗️AgentBridge/🎯️targets/🧊️wgpu/🦀️.rs'
c = Cut(F, 'abridge.rs')
c.child('⚙', 'config-and-shared-types', 'config_and_shared_types', [(28, 275)],
        'Config and shared types: the bridge version and reconnect constants, the bridge config and its discovery, the shell kind, flags, refusal, approval decision and withdrawal and the frame fault.')
c.child('📨', 'wire-and-gateway-messages', 'wire_and_gateway_messages', [(278, 672)],
        'Wire and gateway messages: the `wire` frame codec, the gateway to shell messages and the shell to gateway messages.')
c.child('🧠', 'bridge-state-and-dialer', 'bridge_state_and_dialer', [(675, 1210)],
        'Bridge state and dialer: the bridge status, presence, approvals, conversation entries and state, the inbound shell command decoder, the socket and dial states, the dialer and the agent label.')
SPEC = c.spec()
