"""🧪️ Exercise the captured synthetic 32-receipt site through the final archive gate."""
import json
import os
from pathlib import Path
import runpy
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = Path(sys.argv[1]).resolve()
capture = runpy.run_path(str(ticket / 'captured-site.py'))['captured_site']
capture_root = root / 'capture'
first = capture_root / '.cache/b/p/m/vite/release/first/out'
second = capture_root / '.cache/b/p/m/vite/release/second/out'
for output in (first, second):
    output.mkdir(parents=True)
    (output / 'index.html').write_text('synthetic capture')
def event(path):
    return '[script] ' + json.dumps({'command':'vite:entry', 'activity':{'resource':'artifact-capture:' + str(path)}})
assert capture(capture_root, [event(first), event(first)]) == first
for lines in ([], [event(first), event(second)]):
    try:
        capture(capture_root, lines)
    except ValueError:
        pass
    else:
        raise AssertionError('invalid site capture accepted')
site = root / 'site'
(site / 'index.html').write_text('<title>synthetic fixture</title>')
(site / 'CNAME').write_text('example.test\n')
(site / '.nojekyll').write_bytes(b'')
vendor = site / '🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js'
vendor.parent.mkdir(parents=True)
vendor.write_text('export const poll = 1;')
manifest = runpy.run_path(str(ticket / 'site-manifest.py'))['write_manifest']
accepted, after = root / 'accepted', root / 'after'
manifest(site, Path.cwd(), accepted)
matrix = runpy.run_path(str(ticket / 'verify-boot-matrix.py'))
titles = json.loads(matrix['CASES'].read_bytes())['titles']
specs = [{'title': title, 'tests': [{'projectName': language, 'expectedStatus':'passed', 'results':[{'status':'passed'}]} for language in ('en','de')]} for title in titles if title.endswith(' boot')]
report = {'errors': [], 'suites': [{'specs': specs}]}
access = '127.0.0.1 - - [now] "GET / HTTP/1.1" 200 -'
matrix['verify'](report, access)
manifest(site, Path.cwd(), after)
assert accepted.read_bytes() == after.read_bytes()
env = dict(os.environ, PACKAGE_PYTHON=sys.executable)
output = root / 'durable'
command = ['bash', str(ticket / 'package-site.sh'), str(site), str(output), str(accepted)]
subprocess.run(command, check=True, env=env, stdout=subprocess.DEVNULL)
archive, = output.glob('site-*.tar.gz')
verify = runpy.run_path(str(ticket / 'verify-release-archive.py'))['verify']
assert verify(archive, accepted)[0] >= 32
(site / 'index.html').write_text('changed after acceptance')
assert subprocess.run(command, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode != 0
assert len(list(output.glob('site-*.tar.gz'))) == 1
print('32-receipt synthetic delivery: manifest, 16-case verdict, archive and changed-site refusal passed')
