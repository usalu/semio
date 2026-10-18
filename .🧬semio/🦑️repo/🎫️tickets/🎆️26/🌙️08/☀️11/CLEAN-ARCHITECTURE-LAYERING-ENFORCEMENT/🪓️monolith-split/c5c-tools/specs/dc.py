from specs.lib import *
DC = OD + '/📇️directory/🔌️client'
UW = [
    unwrap('🖥', 'native-transport', 'native', 'Native transport of the directory client: the ureq backed HTTP transport routed through the services HTTP pool, the compute pool admission, the WS connection and the credential frame reads.'),
    unwrap('🌉', 'browser-transport', 'browser', 'Browser transport of the directory client: the web_sys fetch and WebSocket bridge behind the directory transport seam.'),
]
CH = [
    ch('📜', 'http-and-transport-contracts', 'http_and_transport_contracts', [(36, 237)],
       'HTTP and transport contracts of the directory client: the document HTTP port topic and caps, the installed service records, the document HTTP port records, the HTTP method and response, the transport errors, the platform marker traits and the directory transport and WS connection traits.'),
    ch('⚠', 'errors-and-event-pages', 'errors_and_event_pages', [(240, 487)],
       'Errors and event pages of the directory client: the client error with its conversions, the canonical command receipt, administration page and event page records, the event page acknowledgement and the bootstrap transition.'),
    ch('🔑', 'socket-grants-and-authority', 'socket_grants_and_authority', [(489, 640)],
       'Socket grants and authority of the directory client: the socket grant receipts with their wiping drops, the document socket authority, admission and expectation and the hub socket grant source.'),
    ch('🧰', 'wire-helpers-and-credentials', 'wire_helpers_and_credentials', [(642, 986)],
       'Wire helpers and credentials of the directory client: the JSON and hex decoders, the URL component encoder, the socket grant probes, the wall clock, the socket hello, the local hub credential with its frame reads and the inherited Windows file.'),
    ch('📡', 'directory-client', 'directory_client', [(988, 1542)],
       'Directory client: the client record over a directory transport with its commands, space and administration reads, event paging, session and document socket calls and the hub socket grant source implementations.'),
    ch('🌊', 'directory-stream', 'directory_stream', [(1546, 1709)],
       'Directory stream of the directory client: the reconnect bounds, the WS urls, the backoff, the stream turns and the directory stream driver.'),
]
SPEC = dict(orig='dc.rs', file=DC + '/🦀️.rs', children=CH, unwrap=UW, expect_missing=['pub mod native {', 'pub mod browser {', '}', '}'], external=mounted(DC + '/🦀️.rs', orig='dc.rs'))
