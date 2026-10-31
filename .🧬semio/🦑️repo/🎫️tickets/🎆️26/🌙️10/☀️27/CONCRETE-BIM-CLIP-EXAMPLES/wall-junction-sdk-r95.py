"""📄 Capture bilingual native neutral wall relationships and exact owned/reference material."""
import importlib,json,sys,hashlib
from pathlib import Path
R=Path.cwd();sys.path.insert(0,str(R));P=Path(__file__).parent;O=P/'🗑️generated/wall-junctions-r95';O.mkdir(parents=True,exist_ok=True)
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔗️wall-junctions');E=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️');A=T.A
paths=[Path(T.__file__),Path(T.__file__).with_suffix('.json'),Path(T.__file__).with_suffix('.schema.json'),Path(A.__file__),Path(A.__file__).with_name('🔣️.json'),Path(E.__file__),Path(A._wall_junction_backend().__file__),Path(__file__)]
def hashes():
    """🔐 Bind captures to exact fixture, native producers and exporter bytes."""
    return {str(path.relative_to(R)):hashlib.sha256(path.read_bytes()).hexdigest() for path in paths}
before=hashes();samples=[];coated=T.coated_fixture();relation=coated['relations'][0];data=relation.pop('path_connection');relation['class']='IfcRelConnectsElements'
for key in ('relating_connection_type','related_connection_type'):data.pop(key)
relation['connection']={'kind':'material_junction',**data}
for row,m in [(row,T.fixture(row)) for row in T.CASE['cases']]+[({'id':'Positive-2nm'},T.small_interface_fixture()),({'id':'Coated-General'},coated)]:
    for language in T.CASE['languages']:
        fragment=E.wall_junction_excerpt(m['relations'][0]['tag'],m,language=language)
        products=[]
        for index,wall in enumerate(m['elements']):
            g=A._wall_geometry(m,wall);products.append({'id':fragment['ids'][('related.' if index else '')+'element'],'guid':wall['guid'],'parts':len(g['parts']),'volume':g['solid_volume'],'reference_geometry':g['interface_reference_parts'],'reference_prefix':('related.' if index else '')+'reference.','reference_parts':len(g['interface_reference_parts']),'reference_volume':sum(A._polyhedron_volume(part) for part in g['interface_reference_parts'])})
        relation=m['relations'][0];samples.append({'case':row['id'],'language':language,'relationship_class':relation['class'].upper(),'relationship_attributes':11 if relation['class']=='IfcRelConnectsPathElements' else 7,'text':'\n'.join(fragment['lines']),'ids':fragment['ids'],'products':products})
after=hashes();assert before==after;(O/'sdk-samples.json').write_text(json.dumps({'before':before,'samples':samples,'stable':True}),encoding='utf8');print('[DEBUG] Captured',len(samples),'native wall junction fragments')
