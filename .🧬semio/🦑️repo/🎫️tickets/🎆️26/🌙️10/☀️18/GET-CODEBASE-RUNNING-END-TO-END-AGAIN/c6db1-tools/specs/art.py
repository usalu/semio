from specs.lib import *
F = DB + '/🗿️artifact/🦀️.rs'
CH = [
    ch('🧾', 'authority-and-journal-sink', 'authority_and_journal_sink', [(67, 238)],
       'Authority and journal sink: `ArtifactAuthority`, the durable group journal sink and commit values and the build and turn types.'),
    ch('🔁', 'close-retry-and-runner-handoff', 'close_retry_and_runner_handoff', [(240, 447)],
       'Close retry and runner handoff: the close retry state and the runner driver and handoff.'),
    ch('🧹', 'runner-retirement-and-poll', 'runner_retirement_and_poll', [(449, 824)],
       'Runner retirement and poll: the runner retirement hooks and slots, the close poll, the runner state, poll and wake.'),
    ch('🏃', 'runner-turns', 'runner_turns', [(826, 1347)],
       'Runner turns: the `ArtifactRunner` impl.'),
    ch('🎭', 'authority-impl-and-terminal-job', 'authority_impl_and_terminal_job', [(1349, 1714)],
       'Authority impl and terminal job: the `ArtifactAuthority` impl and the runner terminal job.'),
]
SPEC = dict(orig='art.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='art.rs'))
