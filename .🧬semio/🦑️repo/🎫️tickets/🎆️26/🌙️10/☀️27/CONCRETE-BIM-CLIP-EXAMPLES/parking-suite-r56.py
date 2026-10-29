"""🅿️ Verify the final portable parking contract against current native occupied bands."""
import hashlib,importlib,json,time,unittest
from pathlib import Path
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
parking=importlib.import_module(prefix+'🧱️model.🧪️tests.🚗️parking');importlib.reload(parking)
source=Path('/repo/🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json');before=hashlib.sha256(source.read_bytes()).hexdigest();started=time.monotonic()
result=unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(parking.ParkingContract))
record={'source_before':before,'source_after':hashlib.sha256(source.read_bytes()).hexdigest(),'run':result.testsRun,'seconds':time.monotonic()-started,'failures':[(str(t),m) for t,m in result.failures],'errors':[(str(t),m) for t,m in result.errors]}
(Path(__file__).parent/'🗑️generated/parking-suite-r56.json').write_text(json.dumps(record),encoding='utf8');print(record)
