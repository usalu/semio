"""📤 Publish only independently accepted fixture source fields after exact freshness guards."""
import hashlib,importlib.util,json,os,sys,time
from pathlib import Path
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True
R=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(R));P=Path(__file__).parent;G=P/'🗑️generated';M=R/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model';start=time.monotonic()
def sha(data):
    """🔐 Bind actual bytes at every source and independent receipt boundary."""
    return hashlib.sha256(data).hexdigest()
def read(name):
    """📋 Retain checked receipt bytes and decoded content without source duplication."""
    data=(G/name).read_bytes();return data,json.loads(data)
def unchanged(values):
    """🛡️ Reject concurrent changes to every captured producer and acceptance input."""
    for path,expected in values.items():assert sha((R/path).read_bytes())==expected,('changed-input',path)
def cancel():
    """🛑 Honor the task-owned cancellation signal before any source write."""
    if(P/'fixture-publication-cancel-r95').exists():raise KeyboardInterrupt('Fixture publication cancelled')
cancel();certbytes,cert=read('fixture-fits-candidate-r95.json');compatbytes,compat=read('fixture-fits-compatibility-candidate-r95.json');schemabytes,schema=read('fixture-candidate-schema-r95.json');nativebytes,native=read('native-r95-fixtures/fixture-fits-native-r95.json');sdkbytes,sdk=read('native-r95-fixtures/fixture-fits-native-results-r95.json');stockbytes,stock=read('fixture-fits-stock-r95.json')
for result,count in((cert,6),(compat,10)):
    assert result['pass']and result['stable']and result['tests']==count and not result['failures']and not result['errors']and not result['skipped'];assert result['before']==result['after']
assert schema['pass']and schema['stable']and schema['products']==685 and not schema['errors']and schema['before']==schema['after']
assert native['stable']and native['producers']==native['after']and sdk['status']=='PASS'and sdk['input_sha256']==sha(nativebytes)
assert sdk['decoder_sha256']==sha((P/'fixture-fits-native-r95.mjs').read_bytes())
checks=[cert['after'],compat['after'],schema['after'],native['after']]
for inputs in checks:unchanged(inputs)
source=M/'🔣️.json';originalbytes=source.read_bytes();original=json.loads(originalbytes);assert sha(originalbytes)==stock['source']==native['source'];spec=importlib.util.spec_from_file_location('fixture_publication_author',P/'fixture-fits-author-r95.py');author=importlib.util.module_from_spec(spec);spec.loader.exec_module(author);candidate=author.apply(original);c=author.fits.case();tags={c['rain']['tag'],c['lamp']['tag'],c['storage']['tag']}|{r['tag']for r in c['taps']};assert len(tags)==6
old={e['tag']:e for e in original['elements']};new={e['tag']:e for e in candidate['elements']};assert set(old)==set(new)and len(old)==685;changed={tag for tag in old if old[tag]!=new[tag]};assert changed==tags
assert {e['tag']:e for e in stock['objects']}=={tag:new[tag]for tag in tags}
for key in original:
    if key!='elements':assert original[key]==candidate[key],('unexpected-model-field',key)
for tag in tags:
    assert old[tag]['guid']==new[tag]['guid']and old[tag]['class']==new[tag]['class']
    for key in old[tag]:
        if key not in('geometry','historical_evidence','psets'):assert old[tag][key]==new[tag][key],('unexpected-element-field',tag,key)
    for key in old[tag]['geometry']:
        if key not in('parts','axis','mounting'):assert old[tag]['geometry'][key]==new[tag]['geometry'][key],('unexpected-geometry-field',tag,key)
    for key in old[tag]['historical_evidence']:
        if key!='uncertainty':assert old[tag]['historical_evidence'][key]==new[tag]['historical_evidence'][key],('unexpected-evidence-field',tag,key)
    assert all(new[tag]['historical_evidence']['uncertainty'][language].startswith(old[tag]['historical_evidence']['uncertainty'][language])for language in('en','de'))
    for key,pset in old[tag].get('psets',{}).items():
        for name,value in pset.items():
            if(key,name)!=('Pset_HistoricalEvidence','ReconstructionUncertainty'):assert new[tag]['psets'][key][name]==value,('unexpected-property',tag,key,name)
    assert len(old[tag]['geometry']['parts'])==len(new[tag]['geometry']['parts'])
    assert [p.get('material')for p in old[tag]['geometry']['parts']]==[p.get('material')for p in new[tag]['geometry']['parts']]
    assert [p.get('role')for p in old[tag]['geometry']['parts']]==[p.get('role')for p in new[tag]['geometry']['parts']]
    if tag==c['lamp']['tag']:
        for key in old[tag]['geometry']['mounting']:
            if key!='point':assert old[tag]['geometry']['mounting'][key]==new[tag]['geometry']['mounting'][key]
        assert old[tag]['geometry']['mounting']['point'][:2]==new[tag]['geometry']['mounting']['point'][:2]
    elif 'mounting'in old[tag]['geometry']:assert old[tag]['geometry']['mounting']==new[tag]['geometry']['mounting']

expected={(tag,language)for tag in tags for language in('en','de')};assert {(r['tag'],r['language'])for r in native['rows']}==expected and len(native['rows'])==12;assert {(r['tag'],r['language'])for r in sdk['rows']}==expected and len(sdk['rows'])==12
Draft202012Validator(json.loads((M/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'))).validate(candidate)
encoded=(json.dumps(candidate,ensure_ascii=False,indent=2)+'\n').encode('utf8');encoded+=b' '*max(0,len(originalbytes)-len(encoded));report={'published':False,'source_before':sha(originalbytes),'source_after':sha(encoded),'changed_tags':sorted(changed),'products':len(old),'cells':sum(r['parts']for r in sdk['rows']),'readbacks':sum(r['exact_binary64_coordinate_readbacks']for r in sdk['rows']),'receipts':{name:sha(data)for name,data in [('contracts',certbytes),('compatibility',compatbytes),('schema',schemabytes),('native_capture',nativebytes),('native_decoder',sdkbytes),('stock',stockbytes)]},'publisher_sha256':sha(Path(__file__).read_bytes()),'seconds':time.monotonic()-start,'scope':'Six source-qualified fixture fits only. Final whole-model material ownership, quantities, native cohort and complete clip/render verification remain required.'}
(G/'fixture-publication-ready-r95.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
if '--publish'in sys.argv:
    cancel()
    for inputs in checks:unchanged(inputs)
    with source.open('r+b')as handle:
        assert handle.read()==originalbytes,'Concurrent source writer detected';cancel();handle.seek(0)
        for offset in range(0,len(encoded),65536):assert handle.write(encoded[offset:offset+65536])==len(encoded[offset:offset+65536])
        handle.flush();os.fsync(handle.fileno())
    assert source.read_bytes()==encoded;report['published']=True;report['seconds']=time.monotonic()-start;(G/'fixture-publication-r95.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
print('[DEBUG] '+json.dumps(report,ensure_ascii=True),flush=True)
