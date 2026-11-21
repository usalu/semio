"""Save recovery variants and prepare text merges without changing the shared tree."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

root, store, saved, report = map(Path, sys.argv[1:])
saved.mkdir(mode=0o700)
rows = []


def digest(data):
    return hashlib.sha256(data).hexdigest()


def blob(entry):
    fields = entry.split()
    if fields[0] != 'put':
        return None
    sha = fields[1]
    try:
        data = (store / sha[:2] / sha).read_bytes()
    except FileNotFoundError:
        return None
    if digest(data) != sha:
        raise ValueError('Stored blob hash did not match')
    return data


for item in json.loads(report.read_text())['items']:
    if item['classification'] != 'needs-reconciliation':
        continue
    relative = Path(item['path'])
    source = root / relative
    row = dict(item)
    rows.append(row)
    if not source.is_file() or source.is_symlink():
        row['resolution'] = 'receive'
        continue
    prefix = saved / digest(str(relative).encode())[:20]
    local = source.read_bytes()
    local_file = prefix.with_suffix('.local')
    local_file.write_bytes(local)
    row['localFile'] = local_file.name
    base = blob(item['base'])
    remote = blob(item['remote'])
    if local == remote:
        row['resolution'] = 'already-equal'
        continue
    if local == base:
        row['resolution'] = 'receive'
        continue
    if base is None or remote is None:
        row['resolution'] = 'preserved-without-merge-base'
        continue
    try:
        for version in (local, base, remote):
            if b'\0' in version:
                raise UnicodeError('binary file')
            version.decode('utf-8')
    except UnicodeError:
        row['resolution'] = 'preserved-binary'
        continue
    base_file = prefix.with_suffix('.base')
    remote_file = prefix.with_suffix('.fleet')
    base_file.write_bytes(base)
    remote_file.write_bytes(remote)
    merged = subprocess.run(['repo', 'tools', 'exec', '--light', 'git', '--', 'merge-file', '-p',
                             str(remote_file), str(base_file), str(local_file)], capture_output=True)
    if merged.returncode != 0:
        row['resolution'] = 'preserved-overlap'
        continue
    if merged.stdout == remote:
        row['resolution'] = 'already-in-fleet'
        continue
    if relative.suffix == '.json':
        try:
            json.loads(merged.stdout)
        except ValueError:
            row['resolution'] = 'preserved-invalid-json'
            continue
    candidate = prefix.with_suffix('.merged')
    candidate.write_bytes(merged.stdout)
    row.update(resolution='merge-candidate', candidate=candidate.name, fleetSha=digest(remote), localSha=digest(local))

(saved / 'paths.json').write_text(json.dumps(rows, ensure_ascii=False, indent=2) + '\n')
for resolution in sorted({row['resolution'] for row in rows}):
    print(resolution, sum(row['resolution'] == resolution for row in rows))
