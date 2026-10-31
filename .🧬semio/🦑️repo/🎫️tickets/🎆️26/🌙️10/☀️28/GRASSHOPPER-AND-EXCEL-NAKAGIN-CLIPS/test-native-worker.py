"""🧰 Exercise the exact repo-worker launch interface and reject a failed child."""
import ast
import os
from pathlib import Path
import subprocess
import sys
sys.stdout.reconfigure(encoding="utf-8",errors="backslashreplace",line_buffering=True)
TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
output=TICKET/'🗑️generated/native-worker-smoke'
output.mkdir(parents=True,exist_ok=True)
source=ast.parse((TICKET/'replay-native-pairs.py').read_text(encoding='utf-8'))
function=next(node for node in source.body if isinstance(node,ast.FunctionDef) and node.name=='repo_child')
namespace={**globals()}
exec(compile(ast.Module(body=[function],type_ignores=[]),'production-repo-child','exec'),namespace)
namespace['repo_child'](TICKET/'native-worker-smoke.py','nw-pass',[])
try:
    namespace['repo_child'](TICKET/'native-worker-smoke.py','nw-fail',['--fail'])
except RuntimeError:
    print('[DEBUG] Native worker failure propagation PASS',flush=True)
else:
    raise AssertionError('Failed child was accepted')
for label in ('nw-pass','nw-fail'):
    text=(output/(label+'.log')).read_text(encoding='utf-8')
    if 'UTF-8 äöü' not in text:
        raise AssertionError('Child UTF-8 bytes were not preserved')
print('[DEBUG] Worker launch/temp/UTF-8/exit contracts PASS',flush=True)
