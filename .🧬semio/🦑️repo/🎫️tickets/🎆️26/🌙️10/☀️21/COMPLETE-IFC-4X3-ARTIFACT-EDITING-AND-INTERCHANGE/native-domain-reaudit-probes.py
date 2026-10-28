"""🔬 Reproduce source-derived writer transitions and exact validation work counts, not native execution."""
import json
import pathlib

root = pathlib.Path(__file__).parent
phase = 'Complete'
trace = [phase]
phase = 'Closing'
trace.append(phase)
next_call = 'close' if phase == 'Complete' else 'advance'
assert next_call == 'advance'
assert phase in ('Closing', 'Closed')
rows = [1000, 10000, 64000]
result = {'nativeExecuted': False, 'modelOnly': True, 'writerTrace': trace, 'nextWriterCall': next_call, 'cursorRejectsAdvance': True, 'identityPreviousRowVisits': {str(n): n * (n - 1) // 2 for n in rows}}
(root / '🗑️generated' / 'native-domain-reaudit-probes.json').write_text(json.dumps(result, indent=2), encoding='utf-8')
print('[DEBUG] Source-derived closure transition and identity scan counts reproduced')
