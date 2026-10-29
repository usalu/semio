"""🧪 Run the source material oracles with externally visible wall progress."""
import hashlib
import importlib
import io
import json
import time
import unittest
from pathlib import Path

module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🪚️ownership')
importlib.reload(module.api);module.api._source.cache_clear();importlib.reload(module)
progress=Path(__file__).parent/'🗑️generated/wall-ownership-r30-progress.json'
original=module.MaterialOwnershipContract._assert_source_wall_partition
def observed(self,tag,empty_band=None):
    """📍 Retain the current physical wall while an independent geometry operation runs."""
    start=time.perf_counter();progress.write_text(json.dumps({'wall':tag,'status':'running'}),encoding='utf8')
    original(self,tag,empty_band)
    progress.write_text(json.dumps({'wall':tag,'status':'completed','seconds':time.perf_counter()-start}),encoding='utf8')
module.MaterialOwnershipContract._assert_source_wall_partition=observed
stream=io.StringIO();result=unittest.TextTestRunner(stream=stream).run(unittest.defaultTestLoader.loadTestsFromModule(module))
Path(__file__).parent.joinpath('🗑️generated/wall-ownership-r30.txt').write_text(stream.getvalue(),encoding='utf8')
print({'tests':result.testsRun,'failures':len(result.failures),'errors':len(result.errors)})
