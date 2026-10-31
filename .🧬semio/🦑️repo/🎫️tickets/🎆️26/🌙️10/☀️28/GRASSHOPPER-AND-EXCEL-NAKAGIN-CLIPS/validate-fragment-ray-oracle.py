"""🔬 Retain exact independent physical/fragment forward-oracle regression endpoints."""
import hashlib
import io
import json
from pathlib import Path
import runpy
import unittest
import sys
sys.stdout.reconfigure(encoding="utf-8",errors="backslashreplace")
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
OUTPUT=TICKET/'🗑️generated/fragment-ray-oracle'
OUTPUT.mkdir(parents=True,exist_ok=True)
FILES=[TICKET/name for name in ('pixel-plane-lineage-cases.json','pixel-plane-lineage.schema.json','pixel-nearest-surface.py','pixel-ray-cases.json','pixel-paint-fragments.schema.json','pixel-source-export.schema.json','pixel-warped-mesh-cases.json','pixel-warped-mesh.schema.json','pixel-convex-fan-cases.json','pixel-convex-fan.schema.json','test-pixel-nearest-surface.py','test-paint-export.cjs','run-fragment-surface-audit.py','validate-fragment-ray-oracle.py')]+[ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🚶️circulation/🐍️.py',ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py',ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🚶️circulation/🔣️.json']
def hashes():
    """🪪 Record every independent original-triangle and forward-paint input."""
    return {str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
before=hashes()
namespace=runpy.run_path(str(TICKET/'test-pixel-nearest-surface.py'))
stream=io.StringIO()
result=unittest.TextTestRunner(stream=stream,verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(namespace['PixelSurfaces']))
after=hashes()
record={'tests':result.testsRun,'success':result.wasSuccessful(),'failures':len(result.failures),'errors':len(result.errors),'before':before,'after':after,'unchanged':before==after}
(OUTPUT/'tests.log').write_text(stream.getvalue(),encoding='utf-8')
(OUTPUT/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(stream.getvalue())
print('[DEBUG] Fragment forward-oracle endpoints unchanged:',record['unchanged'])
raise SystemExit(0 if record['success'] and record['unchanged'] else 1)
