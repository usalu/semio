"""📚 Pin and inspect official IFC specification examples without changing source bytes."""
import argparse
import hashlib
import json
import pathlib
import re
import sys
import urllib.request
import urllib.parse

ticket = pathlib.Path(__file__).resolve().parent
output = ticket / '🗑️generated' / 'corpus'
repository = 'buildingSMART/IFC4.x-specification-models'
agent = {'User-Agent': 'semio-ifc-conformance-research', 'Accept': 'application/vnd.github+json'}

def retrieve(url):
    with urllib.request.urlopen(urllib.request.Request(url, headers=agent), timeout=60) as response:
        return response.read()

def save(name, value):
    output.mkdir(parents=True, exist_ok=True)
    (output / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

def scan():
    revision = json.loads(retrieve(f'https://api.github.com/repos/{repository}/commits/main'))['sha']
    tree = json.loads(retrieve(f'https://api.github.com/repos/{repository}/git/trees/{revision}?recursive=1'))
    if tree.get('truncated'):
        raise RuntimeError('Incomplete repository tree')
    files = [{'path': row['path'], 'blob': row['sha'], 'bytes': row.get('size', 0)} for row in tree['tree'] if row['type'] == 'blob' and row['path'].lower().endswith('.ifc')]
    manifest = {'repository': repository, 'commit': revision, 'files': files}
    save('manifest.json', manifest)
    print(json.dumps({'commit': revision, 'models': len(files), 'bytes': sum(row['bytes'] for row in files)}))

def fetch():
    manifest = json.loads((output / 'manifest.json').read_text(encoding='utf-8'))
    if sum(row['bytes'] for row in manifest['files']) > 128 * 1024 * 1024:
        raise RuntimeError('Corpus exceeds the research transfer bound')
    rows = []
    for index, source in enumerate(manifest['files']):
        row = dict(source)
        local = hashlib.sha256(source['path'].encode()).hexdigest()[:20] + '.ifc'
        target = output / local
        if len(str(target)) > 256:
            raise RuntimeError('Research path exceeds portable limit')
        url = f'https://raw.githubusercontent.com/{repository}/{manifest["commit"]}/{urllib.parse.quote(source["path"])}'
        content = target.read_bytes() if target.exists() else retrieve(url)
        if len(content) != source['bytes']:
            raise RuntimeError(f'Source byte count differs: {source["path"]}')
        blob = hashlib.sha1(b'blob ' + str(len(content)).encode() + b'\0' + content).hexdigest()
        if blob != source['blob']:
            raise RuntimeError(f'Source blob differs: {source["path"]}')
        target.write_bytes(content)
        header = re.search(rb'FILE_SCHEMA\s*\(\s*\(\s*\x27([^\x27]+)\x27', content[:65536], re.IGNORECASE)
        row.update({'url': url, 'file': local, 'sha256': hashlib.sha256(content).hexdigest(), 'schema': header.group(1).decode('ascii') if header else None})
        rows.append(row)
        save('downloaded.json', {'repository': repository, 'commit': manifest['commit'], 'files': rows})
        print('[DEBUG] ' + json.dumps({'completed': index + 1, 'total': len(manifest['files']), 'schema': row['schema'], 'path': row['path']}), flush=True)

parser = argparse.ArgumentParser()
parser.add_argument('action', choices=['scan', 'fetch'])
args = parser.parse_args()
try:
    {'scan': scan, 'fetch': fetch}[args.action]()
except KeyboardInterrupt:
    sys.exit(130)
