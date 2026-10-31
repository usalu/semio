"""🏗️ Publish one independently accepted hall, timber and shared roof-binding source transaction."""
import hashlib,importlib,json,os,runpy,sys,time
from pathlib import Path
from jsonschema import Draft202012Validator
R=Path.cwd();P=Path(__file__).parent;O=P/'🗑️generated/native-r95-hall';sys.path.insert(0,str(R));A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');started=time.monotonic()
def digest(value):
    """🔐 Bind source values and independent receipts by their exact bytes."""
    return hashlib.sha256(value).hexdigest()
def canonical(value):
    """🧾 Compare complete candidate source values independently of dictionary order."""
    return digest(json.dumps(value,sort_keys=True,separators=(',',':'),allow_nan=False).encode())
def read(name):
    """📦 Retain the exact checked receipt bytes and its decoded value."""
    value=(O/name).read_bytes();return value,json.loads(value)
def check_inputs(values):
    """🛡️ Reject every source, producer, helper, fixture or validator drift before publication."""
    for name,value in values.items():assert digest((R/name).read_bytes())==value,('changed-input',name)
def cancelled():
    """🛑 Stop before the atomic source replacement if publication is cancelled."""
    if (O/'skirting.cancel').exists() or (P/'native-r92-cancel').exists():raise KeyboardInterrupt('Hall source publication cancelled')
cancelled();candidate_bytes,candidate=read('skirting-candidate.json');cert_bytes,cert=read('skirting-validation.json');acceptance_bytes,acceptance=read('acceptance.json');samples_bytes,samples=read('skirting-samples-r94.json');native_bytes,native=read('skirting-native-r94.json');actor_samples_bytes,actor_samples=read('actor-samples.json');actor_native_bytes,actor_native=read('actor-native.json')
assert candidate['source_stable'] and candidate['before']==candidate['after'] and not candidate['stable'] and candidate['unpublished_model_helper'] and not candidate['diagnostic_cache'] and not candidate['partial_inclined_only'] and not candidate['errors']
assert cert['success'] and cert['unpublished_hall_candidate'] and not cert['failures'] and cert['candidate_sha256']==digest(candidate_bytes)
assert acceptance['success'] and acceptance['source_stable'] and acceptance['model_and_geometry_unchanged'] and acceptance['candidate_sha256']==digest(candidate_bytes) and len(acceptance['rows'])==14
assert all(row['fresh_code_equal'] and row['error'] is None for row in acceptance['rows'])
assert samples['stable'] and samples['unpublished_candidate'] and not samples['partial_diagnosed'] and native['success'] and native['unpublished_candidate'] and not native['partial_diagnosed'] and not native['failures'] and native['input_sha256']==digest(samples_bytes)
assert actor_samples['stable'] and actor_samples['unpublished_candidate'] and actor_native['success'] and actor_native['unpublished_candidate'] and actor_native['input_sha256']==digest(actor_samples_bytes)
expected={(obj['tag'],language) for obj in candidate['elements'] for language in ('en','de')};assert {(row['tag'],row['language']) for row in native['rows']}==expected and len(native['rows'])==len(expected)
assert len(actor_native['rows'])==10 and {(row['tag'],row['language']) for row in actor_native['rows']}=={(tag,language) for tag in ('W-G-SERVICE','W-G-ROOM-10','W-G-ROOM-13','D-SERVANT-1','D-SERVANT-2') for language in ('en','de')}
checks=[candidate['after'],cert['source_inputs'],cert['validation_inputs'],acceptance['inputs_before'],samples['before'],native['source'],native['validator_inputs'],actor_samples['before'],actor_native['source_inputs'],actor_native['validator_inputs']]
for values in checks:check_inputs(values)
source=Path(A.__file__).with_name('🔣️.json').resolve();assert source.is_relative_to(R.resolve());original_bytes=source.read_bytes();original=json.loads(original_bytes);assert digest(original_bytes)==candidate['after'][str(source.relative_to(R))]
model=runpy.run_path(str(P/'native-hall-bind-r95.py'))['prepare'](A,original,candidate);assert canonical(model)==acceptance['model_canonical_sha256']
Draft202012Validator(json.loads((Path(A.__file__).parent/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'))).validate(model)
old={item['tag']:item for item in original['elements']};new={item['tag']:item for item in model['elements']};assert set(old)==set(new) and len(old)==685
allowed={item['tag'] for item in candidate['elements']}|{'W-G-SERVICE','W-G-ROOM-10','W-G-ROOM-13','FIN-ROOF-PAVING','FIN-ROOF-GRAVEL','FIN-ROOF-STAIR'}
changed=[tag for tag in old if old[tag]!=new[tag]];assert set(changed)<=allowed
assert all(old[tag]['guid']==new[tag]['guid'] and old[tag]['class']==new[tag]['class'] for tag in old)
for tag in ('FIN-ROOF-PAVING','FIN-ROOF-GRAVEL','FIN-ROOF-STAIR'):
    assert old[tag]['geometry']['parts']==new[tag]['geometry']['parts'] and old[tag]['material']==new[tag]['material'] and 'layer_binding' in new[tag]['geometry']
assert original['materials']==model['materials']
for field in set(original)-{'elements','spaces','relations','models'}:assert original[field]==model[field],('unexpected-model-field',field)
spaces_old={item['tag']:item for item in original['spaces']};spaces_new={item['tag']:item for item in model['spaces']};assert set(spaces_old)==set(spaces_new)
changed_spaces=[tag for tag in spaces_old if spaces_old[tag]!=spaces_new[tag]];assert set(changed_spaces)=={'SP-HALL','SP-SERVANTS-1','SP-SERVANTS-2'}
for old_model,new_model in zip(original['models'],model['models']):
    assert old_model['id']==new_model['id'] and set(old_model['element_tags'])==set(new_model['element_tags']) and len(new_model['element_tags'])==len(set(new_model['element_tags']))
relations_old={item['tag']:item for item in original['relations']};relations_new={item['tag']:item for item in model['relations']};assert set(relations_old)==set(relations_new) and all(relations_old[tag]['guid']==relations_new[tag]['guid'] for tag in relations_old)
for tag,item in relations_old.items():
    if item!=relations_new[tag]:assert item['class'] in ('IfcRelAssociatesMaterial','IfcRelDefinesByType','IfcRelContainedInSpatialStructure'),('unexpected-relation-change',tag)
encoded=(json.dumps(model,ensure_ascii=False,indent=2)+'\n').encode('utf8');report={'ready':True,'published':False,'source_before':digest(original_bytes),'source_after':digest(encoded),'model_canonical_sha256':canonical(model),'products':len(model['elements']),'stock_products':len(candidate['elements']),'stock_cells':sum(len(item['geometry']['parts']) for item in candidate['elements']),'changed_elements':changed,'changed_spaces':changed_spaces,'native_stock_fragments':len(native['rows']),'native_stock_cells':sum(row['physical_body_items'] for row in native['rows']),'native_changed_actor_fragments':len(actor_native['rows']),'native_changed_actor_cells':sum(row['strict_closed_positive_cells'] for row in actor_native['rows']),'candidate_sha256':digest(candidate_bytes),'physical_certificate_sha256':digest(cert_bytes),'acceptance_sha256':digest(acceptance_bytes),'stock_native_sha256':digest(native_bytes),'actor_native_sha256':digest(actor_native_bytes),'publisher_sha256':digest(Path(__file__).read_bytes()),'seconds':time.monotonic()-started,'scope':'One qualified source reconstruction transaction; whole-model material ownership and final clip/native delivery gates remain separate required work.'}
(O/'publication-ready.json').write_text(json.dumps(report,indent=2),encoding='utf8')
if '--publish' in sys.argv:
    cancelled()
    for values in checks:check_inputs(values)
    assert source.read_bytes()==original_bytes,'Concurrent source writer detected'
    stage=O/'source-publication-r95.tmp';assert stage.resolve().is_relative_to((P/'🗑️generated').resolve()) and len(str(stage.resolve()).encode('utf-16-le'))//2<=256
    try:
        with stage.open('xb') as stream:stream.write(encoded);stream.flush();os.fsync(stream.fileno())
        cancelled();assert source.read_bytes()==original_bytes,'Concurrent source writer before atomic replace';os.replace(stage,source)
    finally:stage.unlink(missing_ok=True)
    actual=source.read_bytes();assert actual==encoded and canonical(json.loads(actual))==report['model_canonical_sha256'];report['published']=True;report['source_after']=digest(actual);(O/'publication.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print('[DEBUG] Combined hall/timber/roofbinding '+('PUBLISHED ' if report['published'] else 'READY ')+json.dumps({key:value for key,value in report.items() if key not in ('changed_elements','publisher_sha256')}),flush=True)
