"""🎯 Reproduce the current portable clipping topology contract through the owned validator."""
import importlib,json,time,unittest
from pathlib import Path
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.⚡️partitions')
result=unittest.TestResult();started=time.monotonic()
unittest.TestSuite([module.SlabPartitionContract('test_clipping_retains_authored_reference_precision')]).run(result)
record={'run':result.testsRun,'seconds':time.monotonic()-started,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors]}
(Path(__file__).parent/'🗑️generated'/'clip-topology-current-r79.json').write_text(json.dumps(record,ensure_ascii=False),encoding='utf8')
print({'run':result.testsRun,'failures':len(result.failures),'errors':len(result.errors)})
