"""🔢 Export binary64 property tokens and a closed oblique stock for independent native SDK review."""
from pathlib import Path
from copy import deepcopy
import importlib,json,hashlib,sys
BASE=Path(__file__).parent;OUT=BASE/'🗑️generated/native-r92';sys.path.insert(0,str(Path.cwd()))
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');ex=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️');case=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🧪️tests.🔢️reals').CASE
m=api.load_model();obj=deepcopy(next(x for x in m['elements'] if api.is_architectural_covering(x)));obj.update(tag='NEUTRAL-BINARY64-STOCK',guid=api.global_id('NEUTRAL-BINARY64-STOCK'),storey='ground',predefined_type='SKIRTINGBOARD',material='MAT-PAINTED-WOOD',geometry={'kind':'synthetic_covering','base_level':'ground','parts':[deepcopy(case['mesh'])]},attributes={},psets={});obj['geometry']['parts'][0]['material']='MAT-PAINTED-WOOD';obj['name']={'en':'Neutral oblique binary64 timber stock','de':'Neutraler schräger binary64-Holzkörper'};m['elements'].append(obj)
records=[]
for language in ('en','de'):
    fragment=api.step_excerpt(obj['tag'],m,language=language);lines=fragment['lines'];maximum=max(int(line.split('=')[0][1:]) for line in lines if line.startswith('#'));properties=[];extra=[]
    for index,row in enumerate(case['cases'],maximum+1):
        token=ex._real(row['value']);extra.append('#'+str(index)+"=IFCPROPERTYSINGLEVALUE('Binary64-"+str(index)+"',$,IFCREAL("+token+"),$);");properties.append({'id':index,'value':row['value'],'expected_bits':row['expected_bits'],'token':token})
    at=lines.index('ENDSEC;',lines.index('DATA;'));lines=lines[:at]+extra+lines[at:];path=OUT/('codec-'+language+'.ifc');path.write_text('\n'.join(lines)+'\n',encoding='utf8')
    records.append({'language':language,'path':str(path),'lines':lines,'ids':fragment['ids'],'numeric_properties':properties,'mesh':case['mesh']['mesh'],'volume':api._polyhedron_volume(case['mesh'])})
report={'codec_hash':hashlib.sha256(Path(ex.__file__).read_bytes()).hexdigest(),'samples':records};(OUT/'codec-samples.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] native codec samples',len(records))
