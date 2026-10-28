"""▢ Author explicit rounded-profile math/source contract without manufacturing producer results."""
from pathlib import Path
import json,copy,math,hashlib
p=Path(__file__).parent;repo=next(a for a in p.parents if (a/'AGENTS.md').exists());base={'id':'bath-cutter-profile','dimensions':[1.8,.6],'radius':.2,'depth':.7,'direction':[0,0,1],'profile':{'translation':[0,0],'rotation':0},'solid':{'translation':[0,0,0],'tilt':0},'mappingScale':[1,1,1],'unitPrefix':None,'carriers':{'x':'REAL','y':'REAL','radius':'REAL','depth':'REAL'},'sampling':{'absoluteChordErrorMetres':0,'relativeChordError':.001,'maxSegments':4096},'damage':'none','formal':'valid','geometry':'supported'}
rows=[base]
def add(id,**values):
 r=copy.deepcopy(base);r.update(id=id,**values);rows.append(r)
add('capsule-limit',dimensions=[2,1],radius=.5)
add('circle-limit',dimensions=[2,2],radius=1)
add('profile-position',profile={'translation':[.4,-.2],'rotation':math.pi/3})
add('solid-tilt',solid={'translation':[3,2,1],'tilt':math.pi/2})
add('oblique-negative-reflection',direction=[1,2,-3],profile={'translation':[.4,-.2],'rotation':math.pi/3},mappingScale=[-2,3,4])
add('millimetres',dimensions=[1800,600],radius=200,depth=700,unitPrefix='MILLI')
add('absolute-policy',sampling={'absoluteChordErrorMetres':.00001,'relativeChordError':0,'maxSegments':4096})
add('ordinary-boundary-cap4',sampling={'absoluteChordErrorMetres':1,'relativeChordError':1,'maxSegments':4},geometry='resource-refusal')
add('radius-zero',radius=0,formal='invalid',geometry='refuse')
add('radius-over-half',radius=.31,formal='invalid',geometry='refuse')
add('missing-radius',damage='radius-unset',formal='invalid',geometry='refuse')
add('integer-radius',carriers={'x':'REAL','y':'REAL','radius':'INTEGER','depth':'REAL'},formal='invalid-carrier',geometry='refuse')
add('foreign-profile-position',damage='profile-position-3d',formal='invalid',geometry='refuse')
add('tangent-extrusion',direction=[1,0,0],formal='invalid',geometry='refuse')
add('coordinate-collapse',solid={'translation':[1e20,1e20,1e20],'tilt':0},geometry='precision-refusal')
for r in rows:
 scale=.001 if r['unitPrefix']=='MILLI' else 1;x,y=r['dimensions'];radius=r['radius'];r['idealArea']=x*y-(4-math.pi)*radius*radius if r['formal']=='valid' else None;r['idealVolume']=r['idealArea']*r['depth']*abs(r['direction'][2])/math.hypot(*r['direction'])*abs(math.prod(r['mappingScale']))*scale**3 if r['idealArea']is not None else None
contract={'schema':'IFC4X3_ADD2','scope':'rounded-rectangle-extrusion-proposal','producerExecuted':False,'segmentCap':'total-boundary-edges','cases':rows}
props={'id':{'type':'string'},'dimensions':{'type':'array','minItems':2,'maxItems':2,'items':{'type':'number'}},'radius':{'type':'number'},'depth':{'type':'number'},'direction':{'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}},'profile':{'type':'object','additionalProperties':False,'required':['translation','rotation'],'properties':{'translation':{'type':'array','minItems':2,'maxItems':2,'items':{'type':'number'}},'rotation':{'type':'number'}}},'solid':{'type':'object','additionalProperties':False,'required':['translation','tilt'],'properties':{'translation':{'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}},'tilt':{'type':'number'}}},'mappingScale':{'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}},'unitPrefix':{'enum':[None,'MILLI']},'carriers':{'type':'object','additionalProperties':False,'required':['x','y','radius','depth'],'properties':{key:{'enum':['REAL','INTEGER']}for key in ['x','y','radius','depth']}},'sampling':{'type':'object','additionalProperties':False,'required':['absoluteChordErrorMetres','relativeChordError','maxSegments'],'properties':{'absoluteChordErrorMetres':{'type':'number','minimum':0},'relativeChordError':{'type':'number','minimum':0,'maximum':1},'maxSegments':{'type':'integer','minimum':4}}},'damage':{'enum':['none','radius-unset','profile-position-3d']},'formal':{'enum':['valid','invalid','invalid-carrier']},'geometry':{'enum':['supported','refuse','resource-refusal','precision-refusal']},'idealArea':{'type':['number','null']},'idealVolume':{'type':['number','null']}}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(contract),'properties':{'schema':{'const':'IFC4X3_ADD2'},'scope':{'const':contract['scope']},'producerExecuted':{'const':False},'segmentCap':{'const':'total-boundary-edges'},'cases':{'type':'array','minItems':16,'items':{'type':'object','additionalProperties':False,'required':list(props),'properties':props}}}}
(p/'rounded-rectangle-profile-cases.json').write_text(json.dumps(contract,indent=2),encoding='utf8');(p/'rounded-rectangle-profile.schema.json').write_text(json.dumps(schema,indent=2),encoding='utf8')
files=['🧬️schema/📚️catalog/IFC4X3_ADD2.exp','🚪️io/geometry/circle.ts','🚪️io/geometry/rectangle.ts','🚪️io/geometry/polyline.ts','🚪️io/geometry/circular-solid.ts','🚪️io/geometry/control.ts','🚪️io/geometry/source.ts'];any=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any';artifact=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc';paths=[any/f for f in files]+[artifact/'📐️geometry/🕳️rings/🟦️.ts',artifact/'📐️geometry/🕳️rings/🦀️.rs',artifact/'📐️geometry/⭕️circle/🟦️.ts',artifact/'📐️geometry/⭕️circle/🦀️.rs'];(p/'🗑️generated/rounded-rectangle-source-observations.json').write_text(json.dumps([{'path':str(f),'sha256':hashlib.sha256(f.read_bytes()).hexdigest(),'bytes':f.stat().st_size}for f in paths],indent=2),encoding='utf8');print('[DEBUG]',len(rows),len(paths))
