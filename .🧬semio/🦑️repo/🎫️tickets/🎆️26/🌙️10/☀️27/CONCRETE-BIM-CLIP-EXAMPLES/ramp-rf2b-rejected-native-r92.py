"""🛝 Retain every rejected RF2B native operand and strict face diagnostic without changing geometry."""
import importlib,json,sys,hashlib,time,traceback,math
from pathlib import Path
from collections import Counter
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));sys.dont_write_bytecode=True
sourcepath=root/'🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json';apipath=sourcepath.with_name('🐍️.py');before={'source':hashlib.sha256(sourcepath.read_bytes()).hexdigest(),'api':hashlib.sha256(apipath.read_bytes()).hexdigest()};api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');m=api.load_model();ticket=Path(__file__).parent;start=time.monotonic()
import numpy as np
from scipy.spatial import ConvexHull
flight=api._find(m,'RF-2B');g=api._ramp_geometry(m,flight);out={'before':before,'flight_source':flight,'ramp_source':api._find(m,flight['geometry']['ramp']),'part_count':len(g['parts']),'rejected':[],'structural_cutters':{}}
for tag in flight['geometry'].get('interface_owners',[]):out['structural_cutters'][tag]=api._interface_owner_parts(m,tag)
for index,part in enumerate(g['parts']):
    try:api._native_material_cells(part)
    except Exception:
        row={'index':index,'error':traceback.format_exc(),'part':part,'faces':[]};vertices=np.asarray(api.part_vertices(part),dtype=float);faces=api.part_faces(part);edges=Counter(tuple(sorted((a,b)))for face in faces for a,b in zip(face,face[1:]+face[:1]));row['edge_incidence_valid']=all(n==2 for n in edges.values());row['duplicate_vertex_count']=len(vertices)-len({tuple(v)for v in vertices});row['native_volume']=api._polyhedron_volume(part)
        try:row['scipy_hull_volume']=float(ConvexHull(vertices).volume)
        except Exception as error:row['scipy_hull_error']=str(error).splitlines()[0]
        for face_index,face in enumerate(faces):
            a,b,c=[vertices[i]for i in face[:3]];first=np.cross(b-a,c-a);firstscale=float(np.linalg.norm(first));normal=None
            for j in range(1,len(face)-1):
                candidate=np.cross(vertices[face[j]]-a,vertices[face[j+1]]-a)
                if np.any(candidate):normal=candidate;break
            if normal is None:row['faces'].append({'face':face_index,'indices':face,'first_scale':firstscale,'all_collinear':True});continue
            scale=float(np.linalg.norm(normal));distances=(vertices-a)@normal/scale;planarity=max(abs(float(distances[i]))for i in face);outside=float(max(distances))
            if firstscale==0 or planarity>1e-9 or outside>1e-9:row['faces'].append({'face':face_index,'indices':face,'first_scale':firstscale,'first_valid_triplet_offset':j,'planarity_m':planarity,'outside_m':outside})
        out['rejected'].append(row)
out.update(seconds=time.monotonic()-start,after={'source':hashlib.sha256(sourcepath.read_bytes()).hexdigest(),'api':hashlib.sha256(apipath.read_bytes()).hexdigest()});(ticket/'🗑️generated'/'ramp-rf2b-rejected-native-r92.json').write_text(json.dumps(out,indent=2),encoding='utf8');print(json.dumps({'before':before,'parts':out['part_count'],'rejected':[{k:v for k,v in row.items()if k not in('part','error')}for row in out['rejected']],'seconds':out['seconds']}))
