import json, glob, os
ART = '/home/deck/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
BASE = '/home/deck/semio/🌎️hub/🧩️compositions/🗄️stdio'
rec = json.load(open(BASE + '/🔌️plugin/📇️catalog/📜️native-codec-factories.json'))['receipts']
R = {}
for r in rec:
    R.setdefault(r['artifact'], []).append(r)
defs = {}
for p in glob.glob(ART + '/*/📜️artifact-definition.json'):
    d = json.load(open(p)); defs[d['artifact']] = d
nat = {}
for a, d in defs.items():
    for c in d['codecs']:
        if c.get('native_factory'):
            nat.setdefault(a, []).append((c, d))
print('native codecs in defs but no receipt:', sorted(set(nat) - set(R)))
print('receipts but no native codec in def:', sorted(set(R) - set(nat)))
for a, rs in R.items():
    for r in rs:
        m = [(c, d) for c, d in nat.get(a, []) if c['id'] == r['descriptor_codec_id']]
        if not m:
            print(a, 'descriptor_codec_id not a native codec in def', r['descriptor_codec_id']); continue
        c, d = m[0]; nf = c['native_factory']
        for k, v in [('factory_id', nf['factory_id']), ('artifact_kind', nf['artifact_kind']), ('artifact_schema', nf['artifact_schema']), ('extension', nf['extension']), ('pack_schema_hash', nf['pack_schema_hash']), ('runtime_capability_id', nf['runtime_capability_id'])]:
            if r[k] != v: print(a, k, 'receipt', r[k], 'def', v)
        if r['definition_path'].split('/🗿️artifacts/')[1].split('/')[0] != d['directory']:
            print(a, 'definition_path dir mismatch', r['definition_path'])
        rel = r['protocol_path'].split('/🗿️artifacts/')[1]
        if not os.path.exists(os.path.join(ART, rel)): print(a, 'protocol path missing', rel)
        for k in ['protocol_source_sha256']:
            pass
for a, lst in nat.items():
    for c, d in lst:
        if c['native_factory']['runtime_capability_id'] not in [x['id'] for x in d['runtime_capabilities']]: print('rc missing', a)
import hashlib
print('--- sha256 of protocol files vs receipts')
for r in rec:
    rel = r['protocol_path'].split('/🗿️artifacts/')[1]
    h = hashlib.sha256(open(os.path.join(ART, rel), 'rb').read()).hexdigest()
    if h != r['protocol_source_sha256']: print(r['artifact'], 'sha mismatch file', h, 'receipt', r['protocol_source_sha256'])
print('--- generated trusted catalog vs receipts')
G = json.load(open('/home/deck/semio/✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/trusted-stdio-catalog.json'))['nativeCodecs']
GI = {g['factoryId']: g for g in G}
RI = {r['factory_id']: r for r in rec}
print('generated count', len(G), 'receipts', len(rec))
print('in receipts not generated:', sorted(set(RI) - set(GI)))
print('in generated not receipts:', sorted(set(GI) - set(RI)))
for f in sorted(set(RI) & set(GI)):
    r, g = RI[f], GI[f]
    for k, gk in [('artifact_kind', 'artifactKind'), ('artifact_schema', 'artifactSchema'), ('pack_schema_hash', 'packSchemaHash'), ('extension', 'extension'), ('protocol_source_sha256', 'protocolSourceSha256')]:
        if r[k] != g[gk]: print(f, k, 'receipt', r[k], 'generated', g[gk])
