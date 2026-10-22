import sys, os
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "mut"))
from recipes import *

calls = [
    call("interactivityPreparedRasterProducerFailures", "preparedRaster", "preparedRasterDraw", "preparedRasterGpu", "canvasRaster", "interpreterRaster", "rendererGlue", "rendererHost"),
    call("interactivityShardExecutorFailures", "executor", "shard"),
    call("interactivityStoreSyncFailures", "storeSync"),
    call("interactivityDbIoFailures", "dbStorage", "dbSqlite", "dbEngine", "dbTestkit", "hubBin"),
    call("interactivityDbIoDirectWriterFailures", "dbEngine", "dbSnapshot", "dbIndex", "dbWal", "sprFormat", "packFormat"),
    call("interactivityDbIoCursorCallerFailures", "dbSync"),
    call("interactivityArtifactSubmitFailures", "dbEngine", "dbArtifact"),
]
generate("⚖laws/🔬interactivity-audit/🗄database", "🗄database", "🗄 The interactivity database laws accept the live production sources and reject them when a required anchor disappears or a forbidden one appears.", db_sources(), calls)
