"""👁️ Inspect current continuous guard geometry from two independent raster projections."""
import hashlib,importlib,json,math,sys
from pathlib import Path
from PIL import Image,ImageDraw
sys.dont_write_bytecode=True
root=next(p for p in Path(__file__).resolve().parents if(p/'AGENTS.md').is_file()and(p/'package.json').is_file());sys.path.insert(0,str(root));ticket=Path(__file__).parent;out=ticket/'🗑️generated';api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️');source=api.load_model();rows=[]
for tag in ('GUARD-RM-1','GUARD-RM-2'):
    for p in api._solid_geometry(source,api._find(source,tag))['parts']:
        vertices=api.part_vertices(p)
        for face in api.part_faces(p):rows.append(([vertices[i] for i in face],p.get('role',''),tag))
for angle,name in ((-35,'southeast'),(145,'northwest')):
    a=math.radians(angle);ca,sa=math.cos(a),math.sin(a)
    def project(p):
        x,y,z=p[0]-9.5,p[1]-13,p[2]-3.5;u=ca*x-sa*y;v=sa*x+ca*y;return [u,v*.38-z,v+z*.38]
    points=[project(p) for face,_,_ in rows for p in face];x0,x1=min(p[0] for p in points),max(p[0] for p in points);y0,y1=min(p[1] for p in points),max(p[1] for p in points);scale=min(1260/(x1-x0),930/(y1-y0));image=Image.new('RGB',(1400,1100),'#f4f6f7');draw=ImageDraw.Draw(image)
    draw.text((45,30),'Villa Savoye - current continuous ramp guards / '+name,fill='#162129');draw.text((45,52),'Diagnostic native occupied cells; 6cm cores, inclined tops, midlanding returns and RF2B 12x28cm upstand',fill='#425563')
    for face,role,tag in sorted(rows,key=lambda row:sum(project(p)[2] for p in row[0])/len(row[0]),reverse=True):
        q=[project(p) for p in face];v=[[70+(p[0]-x0)*scale,100+(p[1]-y0)*scale] for p in q];a,b,c=face[:3];u=[b[k]-a[k] for k in range(3)];w=[c[k]-a[k] for k in range(3)];n=[u[1]*w[2]-u[2]*w[1],u[2]*w[0]-u[0]*w[2],u[0]*w[1]-u[1]*w[0]];length=math.hypot(*n);shade=.76+.18*abs(n[2])/length if length else .76;base=(151,180,192) if tag=='GUARD-RM-1' else (193,184,163)
        if 'upstand' in role:base=(125,143,151)
        if 'cap' in role:base=tuple(min(255,c+30) for c in base)
        draw.polygon(v,fill=tuple(int(c*shade) for c in base))
    image.save(out/('guard-preview-'+name+'-r92.png'))
print('[DEBUG] '+json.dumps({'faces':len(rows),'source':hashlib.sha256(Path(api.__file__).with_name('🔣️.json').read_bytes()).hexdigest()}))
