"""🧪️ Record only the actual terminal expanded run and preserve earlier checkpoint evidence."""
from pathlib import Path
import json
import re
import shutil

ticket = Path(__file__).parent
generated = ticket / '🗑️generated'
selection = json.loads((generated / 'ifc4x3-and-details-selection.json').read_text(encoding='utf-8'))
assert selection['suites'] == 135
log = (generated / 'ifc4x3-and-details-tests.txt').read_text(encoding='utf-8')
totals = {key: int(re.findall(r'^\s*(\d+) ' + word + r'$', log, re.MULTILINE)[-1]) for key, word in [('pass', 'pass'), ('skip', 'skip'), ('fail', 'fail')]}
totals['assertions'] = int(re.findall(r'^\s*(\d+) expect\(\) calls$', log, re.MULTILINE)[-1])
footer = re.findall(r'Ran (\d+) tests across (\d+) files\. \[(.*?)\]', log)[-1]
totals.update(tests=int(footer[0]), files=int(footer[1]), elapsed=footer[2], code=selection['code'], changedTests=len(selection['changedTests']))
assert totals['files'] == 135
shutil.copyfile(generated / 'ifc4x3-and-details-tests.txt', generated / 'tests135.txt')
shutil.copyfile(generated / 'ifc4x3-and-details-selection.json', generated / 'selection135.json')
paths = [path.replace('\\', '/') for path in selection['paths']]
scope = {'ifc': sum('/🏅️standards/🔖️4x3/' in path for path in paths), 'details': sum('/🔎️details/' in path for path in paths)}
scope['shared'] = 135 - scope['ifc'] - scope['details']
(generated / 'checkpoint135-totals.json').write_text(json.dumps({'totals': totals, 'scope': scope}, indent=2), encoding='utf-8')
status = f"Expanded135 actual terminal{totals['code']}: {totals['pass']}pass/{totals['skip']}skip/{totals['fail']}fail/{totals['assertions']}assertions/{totals['tests']}tests/{totals['files']}files/{totals['elapsed']}, {totals['changedTests']} selected test-source hash changes; scope{scope['ifc']}IFC4x3/{scope['details']}Details/{scope['shared']}shared. See checkpoint135.md for retention and independent evidence boundaries. Prior127 remains historical. Full IFC4x3/all-MVD goal/ticket remain active and incomplete. Native ingress/type/runtime/receipt/capability proof remains unavailable; native SET is still active authored-law work. Older paragraphs are historical."
for name in ['current-state.md', 'registered-test-scope.md', 'exchange-profile-current-state.md']:
    path = ticket / name
    title, separator, body = path.read_text(encoding='utf-8').partition('\n\n')
    if not body.startswith(status):
        path.write_text(title + separator + status + '\n\n' + body, encoding='utf-8', newline='\n')
print('[DEBUG] ' + json.dumps({'totals': totals, 'scope': scope}))
