"""🪟 Exercise the status of a Windows process object after its process ID has been recycled."""
from pathlib import Path
from runpy import run_path
import json
t=Path(__file__).resolve().parent
fixture=json.loads((t/'digital-owner-wait.json').read_bytes())
alive=run_path(str(t/'equipment-guard.py'))['parent_wait_alive']
assert alive(fixture['timeout']) and not alive(fixture['signalled'])
try:alive(fixture['failure'])
except OSError:pass
else:raise AssertionError('Failed Win32 wait reported a live process')
print('Native timeout, process exit and failed waits are distinct.')
