import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "mut"))
from recipes import *

calls = [
    call("interactivityDbIoB1B6Failures", "dbStorage", "dbSqlite", "dbPostgres", "dbNeo4j", "hubBin"),
    call("interactivityP1qR4Failures", "packFormat", "dbSnapshot", "dbIndex", "dbWal", "dbArtifact", "dbQuery", "dbEngine", "dbCompact", "dbCli", "dbState"),
]
generate("⚖laws/🔬interactivity-audit/🗄database/🧮b1b6-p1q-r4", "🧮b1b6-p1q-r4", "🧮 The retained database I/O B1-B6 and P1q-R4 laws accept the live sources and reject them when a required anchor disappears or a forbidden one appears.", db_sources(), calls)
