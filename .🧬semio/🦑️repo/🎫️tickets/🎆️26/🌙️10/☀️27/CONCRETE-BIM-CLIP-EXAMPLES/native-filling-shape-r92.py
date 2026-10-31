"""🪟 Recheck the exact typed station cause of old native setup errors without rerunning the whole moving cohort."""
import hashlib,importlib,json,sys,time
from pathlib import Path
ROOT=Path.cwd();sys.path.insert(0,str(ROOT));OUT=Path(__file__).parent/'🗑️generated/native-r92';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
paths=[Path(api.__file__),Path(api.__file__).with_name('🔣️.json')]
def hashes():
    """🔐 Bind actual source and first-party producer bytes."""
    return {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest() for p in paths}
before=hashes();m=api.load_model();start=time.monotonic();rows=[]
for obj in m['elements']:
 if obj['class'] not in ('IfcDoor','IfcWindow'):continue
 if (OUT/'filling-shape.cancel').exists():raise KeyboardInterrupt('Filling-shape audit cancelled')
 spec=api._find(m,obj['type'])['filling'];stations=[] if 'panel_layout' in spec else obj['geometry'].get('panel_stations',spec['panel_stations'])
 assert all(isinstance(v,dict) and v['unit']=='1' and isinstance(v['value'],(int,float)) for v in stations),obj['tag']
 geometry=api._filling_geometry(m,obj);assert geometry['parts'];rows.append({'tag':obj['tag'],'class':obj['class'],'parts':len(geometry['parts'])})
after=hashes();report={'stable':before==after,'before':before,'after':after,'seconds':time.monotonic()-start,'fillings':rows}
(OUT/'filling-shape.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'stable':report['stable'],'count':len(rows),'seconds':report['seconds']}),flush=True)
raise SystemExit(0 if report['stable'] else 1)
