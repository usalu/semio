"""📇 Declare lazily discovered registry data before invoking the unchanged shared static verifier."""
import json
from pathlib import Path
import runpy
import sys

TICKET=Path(__file__).parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
sys.modules['excel_registry_static_inputs']=sys.modules[__name__]

def source_files():
    """🔗 Audience, presentation-schema and course inputs are known before registry metadata execution."""
    return (Path(__file__),TICKET/'excel-registry-inputs.json',*(ROOT/path for path in json.loads((TICKET/'excel-registry-inputs.json').read_text(encoding='utf8'))))

sys.argv=[str(TICKET/'validate-excel-static.py'),'--gate','registry']
runpy.run_path(str(TICKET/'validate-excel-static.py'),run_name='__main__')
