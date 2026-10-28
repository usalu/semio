"""📦 Acquire pinned original test-only cancellation oracle packages with registry integrity checks."""
import base64
import hashlib
import io
import json
import pathlib
import tarfile
import urllib.request

ticket = pathlib.Path(__file__).resolve().parent
output = ticket / '🗑️generated' / 'ctrlmod'
rows = []
for name, version in [('p-retry', '8.0.1'), ('is-network-error', '1.3.0')]:
    metadata_url = f'https://registry.npmjs.org/{name}/{version}'
    metadata = json.loads(urllib.request.urlopen(metadata_url, timeout=30).read())
    assert metadata['name'] == name and metadata['version'] == version
    source = urllib.request.urlopen(metadata['dist']['tarball'], timeout=30).read()
    integrity = metadata['dist']['integrity']
    algorithm, expected = integrity.split('-', 1)
    assert algorithm == 'sha512'
    assert base64.b64encode(hashlib.sha512(source).digest()).decode() == expected
    target = output / ('node_modules' if name != 'p-retry' else '') / name
    target.mkdir(parents=True, exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(source), mode='r:gz') as archive:
        for member in archive.getmembers():
            relative = pathlib.PurePosixPath(member.name)
            assert relative.parts[0] == 'package' and '..' not in relative.parts
            if not member.isfile():
                continue
            destination = target.joinpath(*relative.parts[1:])
            assert len(str(destination.resolve())) <= 256
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(archive.extractfile(member).read())
    rows.append({'name': name, 'version': version, 'url': metadata['dist']['tarball'], 'integrity': integrity, 'sha256': hashlib.sha256(source).hexdigest()})
    (output / f'{name}.tgz').write_bytes(source)
(output / 'provenance.json').write_text(json.dumps(rows, indent=2) + '\n', encoding='utf8')
print(json.dumps(rows))
