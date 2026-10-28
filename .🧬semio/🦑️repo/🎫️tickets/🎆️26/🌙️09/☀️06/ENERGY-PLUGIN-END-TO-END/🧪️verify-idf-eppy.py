"""🧪 Compare the production Rust lexical codec against independent Eppy on official EnergyPlus input."""
import io
import json
from pathlib import Path
import subprocess
from eppy.EPlusInterfaceFunctions import readidf

ticket = Path(__file__).parent
generated = ticket / '🗑️generated'
root = Path.cwd()
owner = root / '✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📤️export/⚡️idf/🧪️tests'
block, _, _, _ = readidf.parse_idd.extractidddata(str(generated / 'Energy+.idd.in'))
idd = readidf.eplusdata.Idd(block, 2)


def independent(text):
    data = readidf.eplusdata.Eplusdata(idd, io.StringIO(text))
    return {key: value for key, value in data.dt.items() if value}


fixture = owner / '1ZoneUncontrolled.idf'
canonical = subprocess.check_output([str(generated / 'idf-lexical-harness'), str(fixture)], text=True)
expected = independent(fixture.read_text())
actual = independent(canonical)
assert expected == actual
count = sum(map(len, expected.values()))
for index, vector in enumerate(json.loads((owner / 'lexical-vectors.json').read_text())):
    path = generated / f'idf-vector-{index}.idf'
    path.write_text(vector['input'])
    result = subprocess.run([str(generated / 'idf-lexical-harness'), str(path)], text=True, capture_output=True)
    if 'error' in vector:
        assert result.returncode != 0 and vector['error'] in result.stderr
        continue
    assert result.returncode == 0, result.stderr
    actual = independent(result.stdout)
    expected = {}
    for record in vector['records']:
        expected.setdefault(record['kind'].upper(), []).append([record['kind'], *record['fields']])
    assert actual == expected, (actual, expected)
print(f'PASS: {count} official EnergyPlus IDF objects match Eppy; 5 language-neutral lexical vectors pass')
