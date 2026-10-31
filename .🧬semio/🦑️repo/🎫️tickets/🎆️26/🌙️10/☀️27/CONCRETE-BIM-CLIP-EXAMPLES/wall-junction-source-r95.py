"""🪢 Declare manually reviewed local wall continuity without changing source axes or material envelopes."""
from copy import deepcopy
from fractions import Fraction

RELATION_CLASSES={
'IfcRelationship':{'supertype':'IfcRoot','adds':[]},
'IfcRelConnects':{'supertype':'IfcRelationship','adds':[]},
'IfcRelConnectsElements':{'supertype':'IfcRelConnects','adds':['ConnectionGeometry','RelatingElement','RelatedElement']},
'IfcRelConnectsPathElements':{'supertype':'IfcRelConnectsElements','adds':['RelatingPriorities','RelatedPriorities','RelatedConnectionType','RelatingConnectionType']},
}

NODES=[
('ground-west-room10',[3.15,11.9],['W-G-W','W-G-ROOM-10']),
('ground-west-servicewc',[3.15,12.95],['W-G-W','W-WC-GROUND-SERVICE-N']),
('ground-west-room13',[3.15,16.06],['W-G-W','W-G-ROOM-13']),
('ground-west-room16',[3.15,18.2],['W-G-W','W-G-ROOM-16']),
('ground-north-west',[3.15,21.4],['W-G-N','W-G-W']),
('ground-north-chauffeurwest',[8.23,21.4],['W-G-N','W-G-CHAUFFEUR-W']),
('ground-north-bedroom',[12.35,21.4],['W-G-N','W-CHAUFFEUR-BEDROOM']),
('ground-north-east',[16,21.4],['W-G-N','W-G-E']),
('ground-north-chauffeurbath',[14.1,21.4],['W-G-N','W-CHAUFFEUR-BATH']),
('ground-east-garagecorner',[16,10.75],['W-G-E','W-G-GARAGE-CORNER']),
('ground-east-chauffeur',[16,14.95],['W-G-E','W-G-CHAUFFEUR']),
('ground-east-wccurve',[16,17.85],['W-G-E','W-WC-CHAUFFEUR-CURVE']),
('ground-garage-endcorner',[13.82,6.17],['W-G-GARAGE-END','W-G-GARAGE-CORNER']),
('ground-room10-service',[7.015,11.9],['W-G-ROOM-10','W-G-SERVICE']),
('ground-service-room13',[7.015,16.08],['W-G-SERVICE','W-G-ROOM-13']),
('ground-room16-service',[7.015,18.2],['W-G-ROOM-16','W-G-SERVICE']),
('ground-room10-wc',[4.75,11.9],['W-G-ROOM-10','W-WC-GROUND-SERVICE-E']),
('ground-ramp-return-room16',[8.23,18.1],['W-G-CHAUFFEUR-W','W-G-CHAUFFEUR-RETURN','W-G-ROOM-16']),
('ground-chauffeur-bedroom-return',[12.35,18],['W-G-CHAUFFEUR','W-G-CHAUFFEUR-RETURN','W-CHAUFFEUR-BEDROOM']),
('ground-chauffeur-wccurve',[15.25,15.576712328767123],['W-G-CHAUFFEUR','W-WC-CHAUFFEUR-CURVE']),
('ground-chauffeur-bath',[14.1,16.53767123287671],['W-G-CHAUFFEUR','W-CHAUFFEUR-BATH']),
('ground-garage-return',[10.9,18],['W-G-CHAUFFEUR-RETURN','W-G-GARAGE']),
('first-salon-kitchen',[4.8,6.006],['W-SALON','W-KITCHEN']),
('first-pantry-cross',[3,6.006],['W-PANTRY','W-PANTRY-SOUTH','W-PANTRY-WEST','W-PANTRY-PATIO']),
('first-private-pantry',[3.6,8.9],['W-PRIVATE','W-PRIVATE-8.9']),
('first-private-sharednorth',[3.6,15.555882352941],['W-PRIVATE','W-SHARED-BATH-N']),
('first-private-sharedsouth',[3.6,14.1],['W-PRIVATE','W-SHARED-BATH-S']),
('first-private-entry',[3.6,18.5],['W-PRIVATE','W-SON-ENTRY']),
('first-pantry-privatebranch',[3,8.9],['W-PRIVATE-8.9','W-PANTRY-PATIO']),
('first-parentbath-return',[8.102941176470589,13.013235294117647],['W-PARENT-BATH','W-PARENT-BATH-S']),
('first-boudoir-east',[14.25,15.5],['W-BOUDOIR-E','W-BOUDOIR']),
('roof-parapet-southeast',[18.85,.15],['PAR-S','PAR-E']),
('roof-parapet-southwest',[.15,.15],['PAR-S','PAR-W']),
('roof-parapet-northwest',[.15,21.35],['PAR-N','PAR-W']),
('roof-parapet-northeast',[18.85,21.35],['PAR-N','PAR-E-REAR']),
('cellar-stair-south',[4.892371,8.221293],['W-CELLAR-W','W-CELLAR-STAIR-S']),
('cellar-stair-north',[4.892371,10.331769000000001],['W-CELLAR-W','W-CELLAR-STAIR-N']),
('first-son-parentbath-entry',[4.4,18.5],['W-SON-ENTRY','W-SON-PRIVATE','W-PARENT-BATH-W']),
('first-wc-southwest',[4.5,11.616176],['W-PARENT-BATH-W','W-WC-FIRST-W','W-WC-FIRST-S']),
('first-wc-northeast',[6.286765,10.219118],['W-WC-FIRST-N','W-WC-FIRST-E']),
('first-wc-northcentre',[5.364706,10.219118],['W-WC-FIRST-N','W-WC-FIRST-C']),
('first-wc-northwest',[4.5,10.219118],['W-WC-FIRST-N','W-WC-FIRST-W']),
('first-wc-southeast',[6.286765,11.616176],['W-WC-FIRST-E','W-PARENT-BATH-S','W-WC-FIRST-S']),
('first-wc-southcentre',[5.364706,11.616176],['W-WC-FIRST-S','W-WC-FIRST-C']),
('ground-servicewc-northeast',[4.75,12.95],['W-WC-GROUND-SERVICE-E','W-WC-GROUND-SERVICE-N']),
('roof-screen-arrival',[3.075,6.095],['SC-1','W-STAIR-ROOF']),
]


def _contains(point,path):
    """📐 Accept native path incidence only when actual binary source axes meet exactly."""
    p=[Fraction(value) for value in point]
    for first,second in zip(path,path[1:]):
        a,b=([Fraction(value) for value in row] for row in (first,second))
        if (b[0]-a[0])*(p[1]-a[1])==(b[1]-a[1])*(p[0]-a[0]) and all(min(a[i],b[i])<=p[i]<=max(a[i],b[i]) for i in (0,1)):return True
    return False


def apply(api,source):
    """📜 Add only reviewed local physical relationships to a fresh shared source candidate."""
    m=deepcopy(source);declared=[]
    for name,definition in RELATION_CLASSES.items():
        if name in m['classes'] and m['classes'][name]!=definition:raise ValueError('Conflicting native relationship class definition '+name)
        m['classes'][name]=deepcopy(definition)
    for name,point,tags in NODES:
        extent=.32 if name.startswith('cellar-') else .28 if name in ('ground-ramp-return-room16','ground-chauffeur-wccurve','roof-screen-arrival','first-son-parentbath-entry') else .18
        bounds=[point[0]-extent,point[1]-extent,point[0]+extent,point[1]+extent];walls=[api._find(m,tag) for tag in tags];priorities={}
        for index,wall in enumerate(walls):
            layers=api._wall_junction_backend()._layers(api,m,wall);core=100-index*10;skin=40-index*2
            priorities[wall['tag']]=[0 if layer.get('is_void') else skin if layer['material'] in ('MAT-PLASTER','MAT-RENDER') else core for layer in layers]
        for i,first in enumerate(walls):
            for second in walls[i+1:]:
                if name in ('ground-ramp-return-room16','first-son-parentbath-entry') and i!=0:continue
                paths=[api._wall_path(m,wall) for wall in (first,second)];native=None
                for path in paths:
                    for endpoint in (path[0],path[-1]):
                        if all(_contains(endpoint,other) for other in paths):native=endpoint;break
                    if native is not None:break
                if not all(api._wall_junction_backend().path_material_layers(api,m,wall) for wall in (first,second)):native=None
                tag='JUNCTION-'+name.upper()+'-'+str(i+1)+'-'+str(tags.index(second['tag'])+1)
                data={'junction':'JUNCTION-'+name.upper(),'bounds':bounds,'relating_priorities':priorities[first['tag']],'related_priorities':priorities[second['tag']],'basis':'inferred','source':'AuthoringRegister-2026-10-30-wall-junctions-r95: registered source axes and exact occupied material interfaces; declared continuous through-core and branch termination, buried plaster ends at connected masonry; no attested historic bond or changed axes/thickness.'}
                relation={'tag':tag,'guid':api.global_id(tag),'class':'IfcRelConnectsPathElements' if native is not None else 'IfcRelConnectsElements','relating':first['tag'],'related':[second['tag']],'name':{'en':'Declared material junction '+name,'de':'Erklärter Materialanschluss '+name}}
                if native is not None:
                    data.update({field:'ATSTART' if native==path[0] else 'ATEND' if native==path[-1] else 'ATPATH' for field,path in zip(('relating_connection_type','related_connection_type'),paths)});relation['path_connection']=data
                else:relation['connection']={'kind':'material_junction',**data}
                assert not any(row['tag']==tag for row in m['relations']);m['relations'].append(relation);declared.append(relation)
    api._wall_junction_backend().context(api,m)
    return m,declared
