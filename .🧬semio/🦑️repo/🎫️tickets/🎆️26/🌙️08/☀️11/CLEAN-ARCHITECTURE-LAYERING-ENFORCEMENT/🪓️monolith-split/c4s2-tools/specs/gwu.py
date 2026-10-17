from specs.lib import *

CH = [
    ch('👥', 'presence-fanout-and-kick', 'presence_fanout_and_kick', [(50, 127)],
       'Presence colour leases, fanout lanes and the kick signal of the gateway unit laws.', reexport=False),
    ch('🗂', 'static-cors-credential-and-blob', 'static_cors_credential_and_blob', [(129, 241)],
       'Static app hosting, CORS reflection, transport credentials and blob writes of the gateway unit laws.', reexport=False),
    ch('🔁', 'event-stream-and-server', 'event_stream_and_server', [(243, 472)],
       'The replay and live seam, the session relay, the server builder and the document socket identity of the gateway unit laws.'),
    ch('🛰', 'presence-rooms-and-budget', 'presence_rooms_and_budget', [(474, 937)],
       'Presence room readers, pages, the per-address budget and the state frame of the gateway unit laws.'),
    ch('🧵', 'presence-sessions', 'presence_sessions', [(939, 1204)],
       'Presence socket sessions, rosters and watched rooms of the gateway unit laws.', reexport=False),
    ch('🛂', 'edge-and-throttle', 'edge_and_throttle', [(1206, 1511)],
       'The bounded edge of the gateway: client addressing, the throttle, body limits, allowances and route groups.', reexport=False),
]

SPEC = dict(
    orig='gwu.rs',
    file=GWU + '/🦀️.rs',
    children=CH,
    external=[],
)
