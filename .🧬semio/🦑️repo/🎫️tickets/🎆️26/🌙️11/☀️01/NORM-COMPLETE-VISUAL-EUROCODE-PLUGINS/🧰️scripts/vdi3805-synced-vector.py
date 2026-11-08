"""Regenerates the vector `🎛️change-product-configuration/🔗️synced` with the independent Python oracle (`🔮️oracles/⚖️compliance/🐍️.py --synced-records`) as the only source of the native 210 record strings."""
import copy, json, os, subprocess, tempfile

S = '/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any'
ORACLE = f'{S}/🔮️oracles/⚖️compliance/🐍️.py'
LEAF = f'{S}/🧪️tests/🧫️fixtures/🧬️mutations/🎛️change-product-configuration'
PY = '/home/deck/semio/.venv/bin/python'


def records(product, configuration=None):
    with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False, encoding='utf8') as f:
        json.dump({'product': product, 'configuration': configuration}, f)
    try:
        out = subprocess.run([PY, '-I', ORACLE, '--synced-records', f.name], check=True, capture_output=True, text=True, encoding='utf8').stdout
    finally:
        os.unlink(f.name)
    return json.loads(out)


def load(name):
    return json.load(open(f'{LEAF}/✅apply/🔣️.{name}.json', encoding='utf8'))


before, after, mutation, diff, outcome = (load(n) for n in ('before', 'after', 'mutation', 'diff', 'outcome'))
before['catalog']['products'][0]['records'] = records(before['catalog']['products'][0])
new_configuration = mutation['ChangeProductConfiguration']['newConfiguration']
after_product = after['catalog']['products'][0]
after_product['records'] = records(before['catalog']['products'][0], new_configuration)
for document in (before, after):
    document['catalog']['file']['recordCount'] = sum(len(p['records']) for p in document['catalog']['products'])
diff = copy.deepcopy(diff)
diff['products']['modified'][0]['patch']['records'] = after_product['records']
os.makedirs(f'{LEAF}/🔗️synced', exist_ok=True)
for name, value in (('before', before), ('after', after), ('mutation', mutation), ('diff', diff), ('outcome', outcome)):
    open(f'{LEAF}/🔗️synced/🔣️.{name}.json', 'w', encoding='utf8').write(json.dumps(value, indent=2, ensure_ascii=False) + '\n')
print(after_product['records'][-1]['fields'])
