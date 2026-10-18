from specs.lib import *
F = DB + '/🗿️artifact/📜️history/🦀️.rs'
CH = [
    ch('🧭', 'replay-limits-view-and-reservation', 'replay_limits_view_and_reservation', [(13, 365)],
       'Replay limits, view and reservation: the replay limits, the history view and the replay reservation with its close cursor.'),
    ch('🛠', 'reservation-construction', 'reservation_construction', [(367, 683)],
       'Reservation construction: the construction slots, claims and hand-backs of the replay reservation.'),
    ch('🔖', 'page-set-and-cursors', 'page_set_and_cursors', [(685, 1027)],
       'Page set and cursors: the history page set and the envelope and frontier cursors.'),
    ch('⏳', 'replay-future', 'replay_future', [(1029, 1593)],
       'Replay future: the replay phases and transitions and the `HistoryReplayFuture`.'),
]
SPEC = dict(orig='hist.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='hist.rs'))
