"""🧪 Verify owned native-worker temporary placement and failure propagation without scene rendering."""
from pathlib import Path
import os
import sys
import tempfile
TICKET=Path(__file__).resolve().parent
path=Path(tempfile.gettempdir()).resolve()
if not path.is_relative_to((TICKET/'🗑️generated').resolve()):
    raise RuntimeError('Worker temporary output escaped ticket')
print('[DEBUG] Worker UTF-8 äöü and inherited temporary placement PASS',flush=True)
raise SystemExit(7 if '--fail' in sys.argv else 0)
