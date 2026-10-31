"""✂️ Retain independent kernel validation and exact implementation/contract endpoints."""
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
BASE=ROOT/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/📐️drafting/✂️fragments'
OUTPUT=TICKET/'🗑️generated/fragment-kernel'
OUTPUT.mkdir(parents=True,exist_ok=True)
FILES=[BASE/'🐍️.py',BASE/'🔣️.schema.json',BASE/'🧪️cases/🔣️.json',BASE/'🧪️tests/🐍️.py',BASE/'🧪️tests/🟨️.cjs',Path(__file__)]
def hashes():
    """🪪 Read the exact source and independent test inputs at both endpoints."""
    return {str(path.relative_to(ROOT)):hashlib.sha256(path.read_bytes()).hexdigest() for path in FILES}
before=hashes()
namespace=runpy.run_path(str(BASE/'🧪️tests/🐍️.py'))
stream=io.StringIO()
result=unittest.TextTestRunner(stream=stream,verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(namespace['FragmentKernel']))
after=hashes()
record={'tests':result.testsRun,'success':result.wasSuccessful(),'failures':len(result.failures),'errors':len(result.errors),'before':before,'after':after,'unchanged':before==after}
(OUTPUT/'tests.log').write_text(stream.getvalue(),encoding='utf-8')
(OUTPUT/'result.json').write_text(json.dumps(record,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(stream.getvalue())
print('[DEBUG] BSP proof exact endpoints unchanged:',record['unchanged'])
raise SystemExit(0 if record['success'] and record['unchanged'] else 1)
