"""🧾 Accept only completed native pairs and contracts matching the current semantic source map."""
import importlib.metadata
import importlib.util
import platform
import json
from pathlib import Path
import sys
sys.dont_write_bytecode=True

TICKET=Path(__file__).resolve().parent
ROOT=next(p for p in TICKET.parents if (p/'🧰️framework').is_dir())
GENERATED=TICKET/'🗑️generated'
GENERATED.mkdir(parents=True,exist_ok=True)
SERIES={'grasshopper':('🦗️grasshopper',['Beat1_Anatomy','Beat2_Capsule','Beat3_Tower','Beat4_Casting','Beat5_DataTrees'],12),'excel':('📊️excel',['Beat1_Anatomy','Beat2_References','Beat3_Program','Beat4_Energy'],19)}

SPEC=importlib.util.spec_from_file_location('ledger_provenance',TICKET/'scene-provenance.py')
PROVENANCE=importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROVENANCE)

def current(record):
    source=record.get('source_fingerprints',{})
    if not source or record.get('source_fingerprints_end')!=source:
        return False
    try:
        return PROVENANCE.fingerprints(ROOT,[Path(key.removeprefix('@external:')) if key.startswith('@external:') else ROOT/key for key in source])==source
    except (FileNotFoundError,SyntaxError,ValueError):
        return False

def runtime_current(record):
    runtime=record.get('runtime_identity',{})
    if runtime.get('python')!=sys.version or runtime.get('executable')!=sys.executable or runtime.get('platform')!=platform.platform():
        return False
    for name,expected in runtime.get('packages',{}).items():
        try:
            actual=importlib.metadata.version(name)
        except importlib.metadata.PackageNotFoundError:
            actual=None
        if actual!=expected:
            return False
    return bool(runtime.get('packages'))

def read(path):
    return json.loads(path.read_text(encoding='utf-8')) if path.exists() else {}

def accepted(result,scope):
    return result.get('success') is True and result.get('source_unchanged') is True and result.get('provenance_scope')==scope and current(result) and runtime_current(result) and result.get('runtime_identity_end')==result.get('runtime_identity') and result.get('failures')==0 and result.get('errors')==0

def main():
    report={}
    for series,(folder,scenes,count) in SERIES.items():
        pairs=[]
        for scene in scenes:
            for language in ('en','de'):
                output=GENERATED/f'scenes-{series}-{scene}-{language}'
                result=read(output/'result.json')
                records=[json.loads(line) for line in (output/'checks.jsonl').read_text(encoding='utf-8').splitlines()] if (output/'checks.jsonl').exists() else []
                matching=[r for r in records if r.get('started_at')==result.get('started_at') and r.get('scene')==scene and r.get('language')==language and r.get('outcome')=='success' and r.get('source_unchanged') is True and r.get('source_fingerprints')==result.get('source_fingerprints') and current(r)]
                pairs.append({'scene':scene,'language':language,'accepted':accepted(result,'selected-dependencies') and len(matching)==1,'started_at':result.get('started_at')})
        contracts=read(GENERATED/f'scenes-{series}-all-en-de'/'result.json')
        report[series]={'pairs':pairs,'contracts':{'accepted':accepted(contracts,'all-scenes') and contracts.get('tests')==count,'tests':contracts.get('tests'),'started_at':contracts.get('started_at')}}
    report['accepted']=all(all(p['accepted'] for p in s['pairs']) and s['contracts']['accepted'] for k,s in report.items() if k!='accepted')
    (GENERATED/'native-ledger.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print('[DEBUG] Native ledger '+json.dumps({k:{'accepted_pairs':sum(p['accepted'] for p in s['pairs']),'expected_pairs':len(s['pairs']),'contracts':s['contracts']['accepted']} for k,s in report.items() if k!='accepted'}))
    return 0 if report['accepted'] else 1

if __name__=='__main__':
    raise SystemExit(main())
