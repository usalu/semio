"""🟢 Execute native furnishing schema/body regressions on the corrected source."""
import hashlib,importlib,json,time,unittest
from pathlib import Path
module=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🪑furnishings');root=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip');ticket=Path(__file__).parent;source=root/'🧱️model/🔣️.json';before=hashlib.sha256(source.read_bytes()).hexdigest();started=time.monotonic()
result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(module.NativeFurnishingContract))
record={'source_before':before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'run':result.testsRun,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors],'seconds':time.monotonic()-started}
(ticket/'🗑️generated/furnishing-native-r77-green.json').write_text(json.dumps(record,ensure_ascii=False));print(record)
