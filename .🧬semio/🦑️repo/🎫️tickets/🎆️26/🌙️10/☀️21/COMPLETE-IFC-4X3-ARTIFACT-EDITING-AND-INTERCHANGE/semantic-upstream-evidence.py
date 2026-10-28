"""🔬 Retain primary upstream issue evidence behind a small public-read interface."""
import json
import pathlib
import urllib.request

class PublicIssueEvidence:
    """🌐 Read public issue metadata without credentials or repository mutation."""
    def read(self, number):
        request = urllib.request.Request(f'https://api.github.com/repos/IfcOpenShell/IfcOpenShell/issues/{number}', headers={'User-Agent': 'semio-ifc-conformance-research', 'Accept': 'application/vnd.github+json'})
        with urllib.request.urlopen(request, timeout=20) as response:
            return json.load(response)

ticket = pathlib.Path(__file__).resolve().parent
output = ticket / '🗑️generated' / 'semantic-upstream'
output.mkdir(parents=True, exist_ok=True)
reader = PublicIssueEvidence()
for number in [3187, 8528]:
    value = reader.read(number)
    (output / f'{number}.json').write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
    print('[DEBUG] ' + json.dumps({'number': number, 'title': value['title'], 'state': value['state'], 'url': value['html_url']}))

