"""🧾️ Collect completed per-case evidence without calling an interrupted full run a pass."""
import hashlib
import json
import pathlib

ticket = pathlib.Path(__file__).resolve().parent
output = ticket / '🗑️generated/stepctl'
rows = {}
for line in (ticket / '🗑️generated/step-control-oracle.txt').read_text(encoding='utf-8-sig').splitlines():
    if line.startswith('[DEBUG] '):
        row = json.loads(line.removeprefix('[DEBUG] '))
        rows[row['name']] = row
for row in json.loads((output / 'oracle-subset.json').read_text(encoding='utf-8'))['cases']:
    rows[row['name']] = row
cases = json.loads((output / 'cases.json').read_text(encoding='utf-8'))
assert set(rows) == {case['name'] for case in cases}
for row in rows.values():
    row['retainedHashes'] = {suffix: hashlib.sha256((output / (row['name'] + '.' + suffix + '.ifc')).read_bytes()).hexdigest() for suffix in ('source', 'owned', 'independent')}
result = {'oracle': 'IfcOpenShell 0.9.0.post1', 'collection': 'completed case rows from the partial ten-case log and subsequent explicit two-case run', 'models': len(rows), 'sourcePopulationMatches': sum(row['equalPopulation'] for row in rows.values()), 'sourceInterpretationDifferences': sum(not row['equalPopulation'] for row in rows.values()), 'nativeAuthoredEquivalent': sum(row['nativeAuthoredEquivalent'] for row in rows.values()), 'ownedExpressComplete': sum(next(item for item in row['validation'] if item['input'] == 'owned')['complete'] for row in rows.values()), 'cases': list(rows.values())}
assert result['models'] == 12
assert result['sourceInterpretationDifferences'] == 4
assert result['nativeAuthoredEquivalent'] == 7
assert result['ownedExpressComplete'] == 11
(output / 'evidence.json').write_text(json.dumps(result, indent=2) + '\n', encoding='utf-8')
print(json.dumps({key: value for key, value in result.items() if key != 'cases'}))
