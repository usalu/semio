"""🚰 Author original guest wall basin and bidet reconstruction without mutating model source."""
import copy,importlib,json,math,sys
from pathlib import Path
root=next(p for p in Path(__file__).resolve().parents if (p/'AGENTS.md').is_file() and (p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();cx,cy=3.25,10.568382352941176;parts=[]
def emit(out,z0,z1,role,mat='MAT-SANITARY-CERAMIC',top=None):
    """🧱 Store one convex material piece with explicit source-local height."""
    parts.append({'outline':out,'z0':z0,'z1':z1,'role':role,'material':mat,**({'top_outline':top} if top else {})})
def circle(x,y,r,n=32):
    """⭕ Resolve a finite hollow neck contour at inferred radius."""
    return [[round(x+r*math.cos(2*math.pi*i/n),12),round(y+r*math.sin(2*math.pi*i/n),12)] for i in range(n)]
def rounded(scale):
    """🫧 Describe corresponding rounded wall-basin sections."""
    return [[round(cx+scale*(x+.055*math.cos(a)),12),round(cy+scale*(y+.055*math.sin(a)),12)] for x,y,start in [(.125,.175,0),(-.125,.175,90),(-.125,-.175,180),(.125,-.175,270)] for a in [math.radians(start+90*i/8) for i in range(9)]]
def ring(outer,inner,z0,z1,role,top_outer=None,top_inner=None):
    """🕳️ Preserve every material annulus through disjoint compatible cells."""
    for i in range(len(outer)):
        j=(i+1)%len(outer);emit([outer[i],outer[j],inner[j],inner[i]],z0,z1,role,top=[top_outer[i],top_outer[j],top_inner[j],top_inner[i]] if top_outer else None)
outer=rounded(.68);hole=circle(cx,cy,.018);floor=api.profile_mesh([outer,hole])
for face in floor['triangles']:emit([floor['vertices'][i] for i in face],.67,.71,'basin_floor')
ring(rounded(.68),rounded(.48),.71,.9,'basin_shell',rounded(1.),rounded(.88));ring(circle(cx,cy,.026),hole,.63,.67,'drain_neck')
for y in [cy-.12,cy+.10]:
    emit([[cx+.09,y],[3.56,y],[3.56,y+.02],[cx+.09,y+.02]],.65,.67,'wall_bracket','MAT-METAL')
    emit([[3.54,y],[3.56,y],[3.56,y+.02],[3.54,y+.02]],.47,.65,'wall_bracket','MAT-METAL')
deck=api._analysis._clip(rounded(1.),[[cx+.10,cy-.3],[cx+.3,cy-.3],[cx+.3,cy+.3],[cx+.10,cy+.3]],digits=None);emit(deck,.9,.915,'tap_deck')
taps=[]
for offset in [-.10,.10]:
    joint=[cx+.14,cy+offset,1.07];plane={'normal':[-1.,0.,1.],'point':joint}
    for role,points,clip in [('tap_stem',[[cx+.14,cy+offset,.915],joint],{'clip_end':plane}),('tap_spout',[joint,[cx-.04,cy+offset,1.07]],{'clip_start':plane})]:
        obj={'guid':api.global_id('GUEST-AUTHORING-'+role),'tag':'GUEST-AUTHORING-'+role,'class':'IfcPipeSegment','storey':'first','system':'water','material':'MAT-METAL','geometry':{'points':points,'tube':{'wall_thickness':{'value':.002},'segments':32,**clip}},'attributes':{'diameter':{'value':.025}}}
        for p in api._service(m,obj)['parts']:
            body={k:p[k] for k in ('mesh','section','outline','top_outline','z0','z1') if k in p}
            if 'mesh' in body:body={'mesh':body['mesh']}
            elif 'section' in body:body={'section':body['section']}
            taps.append({**body,'material':'MAT-METAL','role':role})
bidet=copy.deepcopy(next(e for e in m['elements'] if e['tag']=='BIDET-SHARED')['geometry']['parts']);origin=[2.975735,14.430294];target=[3.129411764705882,11.225]
def rotate(p):
    """🧭 Orient the attested bidet grammar toward lowX with its back tap at highX."""
    return [round(target[0]-(p[1]-origin[1]),12),round(target[1]+p[0]-origin[0],12),*p[2:]]
for part in bidet:
    for key in ('outline','top_outline'):
        if key in part:part[key]=[rotate(p) for p in part[key]]
    if 'mesh' in part:part['mesh']['vertices']=[rotate(p) for p in part['mesh']['vertices']]
    if 'section' in part:raise ValueError('Guest authoring must explicitly rotate section planes')
out={'FIX-GUEST-BASIN':parts,'TAP-GUEST':taps,'BIDET-GUEST':bidet};target_path=Path(__file__).parent/'🗑️generated'/'guest-sanitary-assets-r92.json';target_path.write_text(json.dumps(out,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({k:len(v) for k,v in out.items()}),flush=True)
