"""💡 Author a source-qualified oblique aim about unchanged exterior riser clamps."""
import copy,hashlib,importlib,json,math,sys
from pathlib import Path
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();rows=[]
for tag,degrees in (('LIGHT-EXTERIOR-11',45),('LIGHT-EXTERIOR-6',-45)):
    obj=api._find(m,tag);geometry=copy.deepcopy(obj['geometry']);centre=geometry['axis'];angle=math.radians(degrees);c,s=math.cos(angle),math.sin(angle)
    def rotate(point,vector=False):
        """🧭 Preserve Z and actual host axis while rotating the inferred horizontal azimuth."""
        x,y=point[0]-(0 if vector else centre[0]),point[1]-(0 if vector else centre[1])
        return [c*x-s*y+(0 if vector else centre[0]),s*x+c*y+(0 if vector else centre[1]),*point[2:]]
    parts=[]
    for raw in geometry['parts']:
        part=copy.deepcopy(raw)
        if 'section' in part:
            section=part['section'];part['z0']=min(p[1] for p in section['profile']);part['z1']=max(p[1] for p in section['profile'])
            part={k:v for k,v in part.items() if k not in ('section','outline','z0','z1') }|{'mesh':{'vertices':[rotate(v) for v in api.part_vertices(part)],'faces':api.part_faces(part)}}
        elif 'mesh' in part:part['mesh']['vertices']=[rotate(v) for v in part['mesh']['vertices']]
        else:
            part['outline']=[rotate(v) for v in part['outline']]
            if 'top_outline' in part:part['top_outline']=[rotate(v) for v in part['top_outline']]
        parts.append(part)
    geometry['parts']=parts;geometry['mounting']['point']=rotate(geometry['mounting']['point']);geometry['mounting']['normal']=rotate(geometry['mounting']['normal'],True)
    obj_after=copy.deepcopy(obj);obj_after['geometry']=geometry
    obj_after['psets']['Pset_LightingEvidence']['GeometryScope']['value']+=' Nominal azimuth is rotated45degrees toward the forecourt about the unchanged actual riser clamp, retaining outward/upward aim and clearing the native concrete shaft. This azimuth is inferred physical reconciliation; it is not a measured original angle.'
    before=api._solid_geometry(m,obj);after=api._solid_geometry(m,obj_after)
    rows.append({'tag':tag,'rotation_degrees':degrees,'original_element_sha256':hashlib.sha256(json.dumps(obj,sort_keys=True,ensure_ascii=False).encode()).hexdigest(),'geometry':geometry,'geometry_scope':obj_after['psets']['Pset_LightingEvidence']['GeometryScope'],'volume_before':sum(api._polyhedron_volume(p) for p in before['parts']),'volume_after':sum(api._polyhedron_volume(p) for p in after['parts'])})
result={'rows':rows};out=Path(__file__).parent/'🗑️generated'/'light-aim-candidates-r92.json';out.write_text(json.dumps(result,ensure_ascii=False,indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'rows':[{k:v for k,v in row.items() if k not in ('geometry','geometry_scope')} for row in rows]}),flush=True)
