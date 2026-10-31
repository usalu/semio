"""🌱 Execute production pure terrain policies through the registered stdlib Python interface."""
import ast,json,sys,hashlib
from pathlib import Path
from types import SimpleNamespace

root=Path.cwd();ticket=Path(__file__).parent
base=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip'
folder=base/'🎨️visuals/🏛️savoye/🧊️axon'
case=json.loads((folder/'🧪️ground/🔣️.json').read_text(encoding='utf-8'))
files=[folder/'🧩️primitives.py',folder/'🐍️.py',folder/'🧪️ground/🐍️.py',base/'🧱️model/🐍️.py',base/'🧪️tests/🐍️.py']
for file in files:ast.parse(file.read_text(encoding='utf-8'),filename=str(file))
names={'_ground_context','_stage','_surface_stage','_column_band','_column_prims','_ground_index'}
tree=ast.parse(files[0].read_text(encoding='utf-8'))
ns={name:index for index,name in enumerate(('STAGE_PLANE','STAGE_UNDER','STAGE_SLAB','STAGE_FIRST','STAGE_WALLS','STAGE_ROOF','STAGE_TOP','STAGE_OVERLAY'))}
ns.update(STAGE_CELLAR=-1,STAGE_CELLAR_SLAB=-2,_item=lambda *args:{})
class Primitive:
    def __init__(self,tag,info,stage,foot,faces,**kwargs):self.tag,self.stage,self.faces=tag,stage,faces
ns['Prim']=Primitive
exec(compile(ast.fix_missing_locations(ast.Module(body=[node for node in tree.body if isinstance(node,ast.FunctionDef) and node.name in names],type_ignores=[])),str(files[0]),'exec'),ns)
terrain={'tag':case['context']['source_tag'],'class':'IfcCovering','predefined_type':'FLOORING','attributes':{'reference_surface_elevation':{'value':case['context']['elevation']}},'psets':{'Pset_TerrainReconstruction':{'UniformExtension':{'value':True}}}}
model={'elements':[terrain]};g={'box':case['context']['box']};context=ns['_ground_context'](g,model)
assert context['elevation']==-.15 and context['surveyed'] is False
assert context['outline']==((0.,0.),(6,0.),(6,6),(0.,6))
assert ns['_ground_context'](g,{'elements':[]}) is None
rows=[]
for item in case['surfaces']:
    stage=ns['_surface_stage'](item,item,3.17,6.6,context['elevation'])
    assert stage==item['expected_stage'],(item,stage)
    rows.append({'item':item,'actual_stage':stage})
source={'outline':[[0,0],[1,0],[1,1],[0,1]],'z0':-2.5,'z1':7.,'profile':{'shape':'circle'},'radius':.1}
item={'tag':'C','guid':'C','x':.5,'y':.5,'parts':[source]}
columns=ns['_column_prims'](item,model,3.17,6.6,-.15)
bands=[(part.faces[0][1][2],part.faces[0][1][3]) for part in columns]
assert bands==[(-2.5,-.15),(-.15,3.17),(3.17,6.6),(6.6,7.)]
assert [part.stage for part in columns]==[-1,1,4,6]
assert sum(b-a for a,b in bands)==source['z1']-source['z0']
model=json.loads((base/'🧱️model/🔣️.json').read_text(encoding='utf-8'))
actual_context=ns['_ground_context']({'box':{'length':19.,'width':21.5}},model)
records=[]
for item in model['elements']:
    if item['class']=='IfcCovering' and item.get('predefined_type')=='FLOORING' and item['storey']=='ground' and item['geometry']['kind']=='flooring':
        for part in item['geometry']['parts']:
            stage=ns['_surface_stage']({**item,'kind':item['geometry']['kind']},part,3.17,6.6,actual_context['elevation'])
            assert stage>0
            records.append({'tag':item['tag'],'stage':stage})
assert len({row['tag'] for row in records})>=12
report={'success':True,'python':sys.version,'scope':'actual pure production terrain functions; no native/NumPy/Manim/SciPy claims','context':context,'portable_stages':rows,'column_bands':bands,'column_stages':[part.stage for part in columns],'actual_floor_tags':sorted({row['tag'] for row in records}),'actual_floor_part_count':len(records),'source_hashes':{str(file.relative_to(root)):hashlib.sha256(file.read_bytes()).hexdigest() for file in files}}
out=ticket/'🗑️generated/native-r92/terrain-stdlib-green.json';out.write_text(json.dumps(report,ensure_ascii=True,indent=2),encoding='utf-8')
print('[DEBUG] terrain pure production policies GREEN '+str(out),flush=True)
