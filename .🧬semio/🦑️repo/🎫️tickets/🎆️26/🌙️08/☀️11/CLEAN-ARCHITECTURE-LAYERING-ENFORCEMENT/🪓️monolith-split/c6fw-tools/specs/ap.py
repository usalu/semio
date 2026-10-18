from specs.lib import *
AP = M + '/🎭️actor/💼️job/📨️publication'
CH = [
    ch('🧭', 'replay-routes-and-policy', 'replay_routes_and_policy', [(6, 143)],
       'Replay routes and policy: the fixed replay capacities, the mounted route identity, the publication policy and kind, the replay faults, steps and close steps and the replay page.'),
    ch('📼', 'replay-records-and-capture', 'replay_records_and_capture', [(145, 306)],
       'Replay records and capture: the record header, the replay record, the capture with their drops, the rejected capture and the replay schedule.'),
    ch('📜', 'replay-log', 'replay_log', [(308, 758)],
       'Replay log: the replay log with its capture, replay and release operations, its drop and the payload accounting and header digest helpers.'),
    ch('🪪', 'progress-identities-and-faults', 'progress_identities_and_faults', [(762, 897)],
       'Progress identities and faults: the progress capacities, the progress identity, kind and live authority, the progress faults and the rejected publication.'),
    ch('🎰', 'progress-slots-and-receipts', 'progress_slots_and_receipts', [(899, 1091)],
       'Progress slots and receipts: the publication owner, the progress and retirement slots, the admission, the receipt, the close step and the overlay store record.'),
    ch('🗃', 'progress-overlay-store', 'progress_overlay_store', [(1093, 1545)],
       'Progress overlay store: the overlay store operations and the checkout with its drop.'),
    ch('🌉', 'publication-error-and-bridge', 'publication_error_and_bridge', [(1548, 1759)],
       'Publication error and bridge: the publication error and the test-only outcome projection and turn bridge.'),
]
SPEC = dict(orig='ap.rs', file=AP + '/🦀️.rs', children=CH, wiring='block', external=mounted(AP + '/🦀️.rs', orig='ap.rs'),
            crate_files=[])
