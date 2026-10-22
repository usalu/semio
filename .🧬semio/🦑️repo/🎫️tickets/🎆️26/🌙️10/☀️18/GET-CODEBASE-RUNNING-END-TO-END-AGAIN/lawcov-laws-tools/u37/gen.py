import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "mut"))
from recipes import *

S = db_sources()
calls = [
    call("interactivityDatabaseCatalogBootstrapFailures", "dbEngine"),
    call("interactivityDatabaseCreateCatalogFailures", "dbEngine", "asyncRuntime", "p1xContract"),
    call("interactivityDatabaseCompactionFailures", "dbCompact", "dbEngine", "dbSnapshot", "dbIndex", "p1yContract"),
    call("interactivityDatabaseSyncHelloFailures", "dbSync", "dbEngine", "hubBin", "dbWal", "replicationWire", "p1zContract"),
    call("interactivityArtifactHistoryFailures", "dbEngine", "dbArtifact", "dbWal"),
    call("interactivityDbCliFailures", "dbCli"),
    call("interactivityMcpHttpTransportFailures", "mcpTransport", "mcpBridge", "mcpRoot"),
]
generate("⚖laws/🔬interactivity-audit/🗄database/🌱database-lifecycle", "🌱lifecycle", "🌱 The database lifecycle laws accept the live production sources and reject them the moment a required anchor disappears or a forbidden one appears; every anchor named by the corpus is checked against an independent source reader.", S, calls)
