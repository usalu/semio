from specs.lib import *
F = DB + '/⌨️cli/🦀️.rs'
CH = [
    ch('✂', 'args-format-and-inspect', 'args_format_and_inspect', [(42, 366)],
       'Args, format and inspect: argv parsing, value and error formatting, the storage and worker bootstrap, health labels, `inspect` and `doc`.'),
    ch('🔚', 'command-close', 'command_close', [(369, 634)],
       'Command close: the mounted WAL, batch, replay and snapshot command close witnesses with their future and drop impls.', glob=False),
    ch('📜', 'wal-and-snapshot-inspect', 'wal_and_snapshot_inspect', [(638, 865)],
       'WAL and snapshot inspect: the WAL cursor and record description plus `wal-inspect` and `snapshot-inspect`.'),
    ch('🩺', 'verify-query-replay-repair-compact-health', 'verify_query_replay_repair_compact_health', [(869, 1203)],
       'Verify, query, replay, repair, compact and health: the document verification and the commands that read, replay, repair, compact and report health.'),
    ch('🚀', 'simulate-migrate-profile-and-main', 'simulate_migrate_profile_and_main', [(1207, 1543)],
       'Simulate, migrate, profile and main: conflict and replica simulation, `migrate`, `profile`, the help text and the `main_impl` entry.'),
]
SPEC = dict(orig='cli.rs', file=F, children=CH, wiring='block', external=mounted(F, orig='cli.rs'))
