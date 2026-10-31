"""🏗️ Build source-qualified construction detail payloads without replacing shared source."""
import importlib,json,math,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();ticket=Path(__file__).parent
payload={'elements':[],'materials':[],'stair_nosings':{}}
def evidence(sources,en,de):
    return {'state':'original-completed-1930-1931','status':'inferred_reconstruction','source_kind':'mixed','sources':sources,'uncertainty':{'en':en,'de':de}}
def product(tag,cls,names,storey,material,parts,ev,kind,predefined=None):
    out={'tag':tag,'guid':api.global_id(tag),'class':cls,'name':dict(zip(('en','de'),names)),'storey':storey,'material':material,'geometry':{'kind':kind,'base_level':storey,'parts':parts},'attributes':{},'psets':{},'metadata':{'author':'Atelier (architecture)','revision':1,'domains':['architecture']},'historical_evidence':ev}
    if predefined:out['predefined_type']=predefined
    return out
def material(tag,names,density):
    out={'tag':tag,'guid':api.global_id(tag),'class':'IfcMaterial','name':dict(zip(('en','de'),names)),'density':{'value':density,'unit':'kN/m³','basis':'inferred','source':'Original substrate/bulk unitweight unmeasured'}};payload['materials'].append(out)
material('MAT-SOFFIT-BOARD',('Exterior ramp ceiling board, substrate unmeasured','Deckenplatte Außenrampe, Träger ungemessen'),7)
def cube(vertices,mat,role):
    return {'mesh':{'vertices':vertices,'faces':[[0,3,2,1],[4,5,6,7],[0,1,5,4],[1,2,6,5],[2,3,7,6],[3,0,4,7]]},'material':mat,'role':role}
for tag in ('RF-2A','RF-2B'):
    host=api._find(m,tag);g=host['geometry'];a,b=g['path'];run=math.dist(a,b);u=[(b[i]-a[i])/run for i in (0,1)];v=[-u[1],u[0]];base=api._z(m,g['from_level']);rise=api._z(m,g['to_level'])-base;zs=[base+rise*p for p in g['portion']];slope=(zs[1]-zs[0])/run;factor=math.sqrt(1+slope*slope);finish=g['finish_thickness']['value']*factor;lo,hi=[p['value'] for p in g['structural_offsets']]
    def point(t,side,depth):
        return [a[0]+u[0]*t+v[0]*side+u[0]*slope*depth/factor,a[1]+u[1]*t+v[1]*side+u[1]*slope*depth/factor,zs[0]+slope*t-finish-g['thickness']['value']-depth/factor-base]
    def part(t0,t1,d0,d1,mat,role):
        return cube([point(t,side,depth) for depth in (d1,d0) for t,side in ((t0,lo),(t1,lo),(t1,hi),(t0,hi))],mat,role)
    parts=[part(0,run,.035,.045,'MAT-SOFFIT-BOARD','soffit_board_inferred')]
    count=math.ceil(run/.5)
    for i in range(count):
        t=.0175+(run-.035)*i/(count-1);half=.0175/factor;parts.append(part(t-half,t+half,0,.035,'MAT-WOOD','timber_batten_35mm'))
    ev=evidence(['SAB2024-p138-p139','FLC-H1(12)128-5-item22','FLC-H1(13)309'],'Original exterior-ramp false ceiling and35×35mm battens are attested. Current flight underside and width set the inferred extent. Batten spacing≤0.50m,10mm board thickness, substrate and density are unmeasured teaching assumptions; no insulation is claimed. Embedded timber blocks are attested but their individual stations are unlocated.','Ursprüngliche Unterdecke und35×35mm Latten sind belegt. Ausdehnung folgt abgeleitet der aktuellen Laufunterseite. Lattenabstand≤0,50m,10mm Plattenstärke, Träger und Wichte sind ungemessene Lehrannahmen; keine Dämmung behauptet. Eingelassene Holzklötze belegt, Einzelstationen ungeklärt.')
    payload['elements'].append(product('FIN-SOFFIT-'+tag,'IfcCovering',('Original exterior ramp soffit '+tag,'Ursprüngliche Rampenunterdecke '+tag),'first','MAT-SOFFIT-BOARD',parts,ev,'ceiling','CEILING'))
candidates=json.loads((ticket/'stair-metal-details-source-r92.json').read_text(encoding='utf8'))['candidate_schedule']
def clip(poly,n,offset):
    out=[]
    for a,b in zip(poly,poly[1:]+poly[:1]):
        da=sum(n[i]*a[i] for i in (0,1))-offset;db=sum(n[i]*b[i] for i in (0,1))-offset
        if da<=1e-10:out.append(a)
        if (da<0<db) or (db<0<da):out.append([a[i]+(b[i]-a[i])*da/(da-db) for i in (0,1)])
    return out
for tag in ('ST-CG','ST-GF','ST-FR'):
    stair=api._find(m,tag);parts=[]
    for candidate in [c for c in candidates if c['stair']==tag]:
        step=stair['geometry']['step_parts'][candidate['tread_index']];poly=step['outline'];a,b=candidate['front_edge'];length=math.dist(a,b);n=[-(b[1]-a[1])/length,(b[0]-a[0])/length];centre=[sum(p[i] for p in poly)/len(poly) for i in (0,1)]
        if sum(n[i]*(centre[i]-a[i]) for i in (0,1))<0:n=[-p for p in n]
        offset=sum(n[i]*a[i] for i in (0,1))
        for depth,z0,z1,role in ((.04,step['z1']-.004,step['z1'],'steel_nosing_horizontal_40x4'),(.004,step['z1']-.025,step['z1']-.004,'steel_nosing_vertical_25x4')):
            ring=clip(clip(poly,[-p for p in n],-offset),n,offset+depth)
            if api._area(ring)<=0:raise ValueError((tag,candidate['tread_index'],ring))
            parts.append({'outline':ring,'z0':z0,'z1':z1,'material':'MAT-METAL','role':role})
    payload['stair_nosings'][tag]=parts
voids=[{'outline':api._expand(m,e['geometry']['outline']),'z0':-.04,'z1':0} for e in m['elements'] if e['class']=='IfcSlab' and e['storey']=='ground' and e['tag'].startswith('SL-GROUND')]
for obj in m['elements']:
    if obj['class']=='IfcColumn':
        g=api.column_geometry(m,obj)
        if g['base']<0 and g['top']>-.04:voids.extend({'outline':p['outline'],'z0':-.04,'z1':0}for p in g['parts'] if p['z0']<0 and p['z1']>-.04)
    elif obj['class']=='IfcWall' and obj['storey']=='ground':
        path=api._wall_path(m,obj);half=api._wall_thickness(m,obj)/2
        for a,b in zip(path,path[1:]):
            length=math.dist(a,b);n=[-(b[1]-a[1])*half/length,(b[0]-a[0])*half/length]
            voids.append({'outline':[[p[i]+side*n[i]for i in (0,1)]for p,side in ((a,-1),(b,-1),(b,1),(a,1))],'z0':-.04,'z1':0})
raw=[{'outline':[[0,0],[19,0],[19,21.5],[0,21.5]],'z0':-.04,'z1':0,'material':'MAT-GRAVEL','role':'pilotis_gravel_inferred_depth'}]
parts=api._subtract_prisms(raw,voids)
ev=evidence(['SAB2024-p192'],'Gravel beneath the pilotis circumdrive is attested. Bounded extent stops at the19×21.5m under-building rectangle and excludes actual ground slabs, wall footprints and columns. Exact perimeter, grading,40mm depth, subgrade and drainage are inferred; no estate surface is reconstructed.','Kies auf der Umfahrt unter den Pilotis ist belegt. Begrenzte Fläche endet am19×21,5m Gebäudegrundriss und spart reale Erdgeschossdecken, Wandgrundrisse und Stützen aus. Exakter Rand, Körnung,40mm Tiefe, Untergrund und Entwässerung sind abgeleitet; keine Grundstücksfläche rekonstruiert.')
payload['elements'].append(product('FIN-PILOTIS-GRAVEL','IfcCovering',('Original pilotis circumdrive gravel, bounded extent','Ursprünglicher Kies unter Pilotis, begrenzte Fläche'),'ground','MAT-GRAVEL',parts,ev,'flooring','FLOORING'))
(ticket/'🗑️generated'/'missing-construction-payload-r92.json').write_text(json.dumps(payload,ensure_ascii=False,indent=2),encoding='utf8');print(json.dumps({'products':[(e['tag'],len(e['geometry']['parts']))for e in payload['elements']],'nosings':{tag:len(p)//2 for tag,p in payload['stair_nosings'].items()}}))
