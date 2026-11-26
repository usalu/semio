"""🧪️ Exercise a captured synthetic site through the final archive gate."""
import json
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
matrix = runpy.run_path(str(ticket / 'verify-boot-matrix.py'))
cases = json.loads(matrix['CASES'].read_bytes())
projects = cases['projects']
def test_specs(titles):
    return [{'title': title, 'tests': [{'projectName': project['name'], 'expectedStatus':'passed', 'results':[{'status':'passed'}]} for project in projects]} for title in titles]

specs = test_specs(cases['titles'])
report = {'errors': [], 'suites': [{'specs': specs}]}
behaviour_cases = json.loads(matrix['BEHAVIOUR_CASES'].read_bytes())
behaviours = {'errors': [], 'suites': [{'specs': test_specs(behaviour_cases['titles'])}]}
discovery = {'config': {'projects': projects}}
rosters = matrix['release_rosters']({**report, **discovery}, {**behaviours, **discovery})
access = '127.0.0.1 - - [now] "GET / HTTP/1.1" 200 -'
release_gate = runpy.run_path(str(ticket / 'accept-site.py'))
accept = release_gate['accept']
project = root / 'project'
source = project / '🧪️tests/🎭️acceptance/test.ts'
source.parent.mkdir(parents=True)
(project / '🪧️brand.ts').touch()
source.write_text('v1')
snapshot = release_gate['acceptance_snapshot'](project)
source.write_text('v2')
assert release_gate['acceptance_snapshot'](project) != snapshot
source.write_text('v1')
assert release_gate['acceptance_snapshot'](project) == snapshot
generated = root / 'generated'
generated.mkdir()
accepted = generated / 'static-accepted.sha256'
accepted.write_text('stale acceptance')
lock = generated / 'static-verification.lock'
lock.mkdir()
try:
    accept(site, Path.cwd(), generated, lambda _: None)
except FileExistsError:
    assert accepted.read_text() == 'stale acceptance'
else:
    raise AssertionError('accepted while another verifier held the lock')
lock.rmdir()

def reject(verify):
    try:
        accept(site, Path.cwd(), generated, verify)
    except ValueError:
        assert not accepted.exists()
        assert not lock.exists()
    else:
        raise AssertionError('accepted an invalid matrix or changed site')

reject(lambda _: matrix['verify_release'](rosters, {'errors': [], 'suites': [{'specs': specs[:-1]}]}, behaviours, access))
reject(lambda _: matrix['verify_release'](rosters, report, behaviours, ''))

def verify_matrix(_, behaviour_report=behaviours):
    matrix['verify_release'](rosters, report, behaviour_report, access)

reject(lambda run: verify_matrix(run, {'errors': [], 'suites': []}))

def change_site(run):
    verify_matrix(run)
    (site / 'index.html').write_text('changed during verification')

reject(change_site)
(site / 'index.html').write_text('<title>synthetic fixture</title>')
accept(site, Path.cwd(), generated, verify_matrix)
after = root / 'after'
manifest(site, Path.cwd(), after)
assert accepted.read_bytes() == after.read_bytes()
output = root / 'durable'
command = [sys.executable, str(ticket / 'package-site.py'), str(site), str(output), str(accepted)]
subprocess.run(command, check=True, stdout=subprocess.DEVNULL)
archive, = output.glob('site-*.tar.gz')
verify = runpy.run_path(str(ticket / 'verify-release-archive.py'))['verify']
assert verify(archive, accepted)[0] == len(accepted.read_bytes().splitlines())
(site / 'index.html').write_text('changed after acceptance')
assert subprocess.run(command, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode != 0
assert len(list(output.glob('site-*.tar.gz'))) == 1
print('Synthetic release delivery: manifest, 96-case verdict, archive and changed-site refusal passed')
