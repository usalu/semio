"""🧱 Observe original faceted shells and normal independent geometry outputs."""
import ctypes,hashlib,json,math,sys,time
from pathlib import Path
root=Path(__file__).resolve().parent;dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,buffer,len(buffer));dep=buffer.value or dep
sys.path.insert(0,dep)
class FacetedOracle:
    def observe(self,path):raise NotImplementedError
class IfcOpenShellFacetedOracle(FacetedOracle):
    def observe(self,path):
        import ifcopenshell,ifcopenshell.geom
        model=ifcopenshell.open(str(path));solids=[]
        for solid in model.by_type('IfcFacetedBrep'):
            faces=[]
            for face in solid.Outer.CfsFaces:
                faces.append(dict(id=str(face.id()),type=face.is_a(),bounds=[dict(id=str(bound.id()),type=bound.is_a(),orientation=bound.Orientation,loopType=bound.Bound.is_a(),points=[dict(id=str(p.id()),coordinates=list(p.Coordinates))for p in getattr(bound.Bound,'Polygon',[])])for bound in face.Bounds]))
            solids.append(dict(id=str(solid.id()),type=solid.is_a(),outer=str(solid.Outer.id()),voids=[str(v.id())for v in getattr(solid,'Voids',[])],faces=faces))
        outputs=[];settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True)
        for product in model.by_type('IfcProduct'):
            if not product.Representation:continue
            start=time.perf_counter()
            try:
                owner=ifcopenshell.geom.create_shape(settings,product);mesh=owner.geometry;coords=list(mesh.verts);indices=list(mesh.faces);points=[coords[i:i+3]for i in range(0,len(coords),3)];terms=[]
                for i in range(0,len(indices),3):
                    a,b,c=(points[indices[i+j]]for j in range(3));terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
                volume=math.fsum(terms);outputs.append(dict(product=str(product.id()),type=product.is_a(),vertices=len(points),triangles=len(indices)//3,bounds=[[min(p[k]for p in points)for k in range(3)],[max(p[k]for p in points)for k in range(3)]],signedVolume=volume,volume=abs(volume),seconds=time.perf_counter()-start))
            except Exception as error:outputs.append(dict(product=str(product.id()),error=str(error),seconds=time.perf_counter()-start))
        return dict(oracle='IfcOpenShell.geom',version=ifcopenshell.version,header=list(model.header.file_schema.schema_identifiers),solids=solids,geometry=outputs)
base=Path(sys.argv[1]);oracle=IfcOpenShellFacetedOracle();rows=[]
for file in ['brep-model.ifc','basin-faceted-brep.ifc']:
    path=base/file;row=dict(file=file,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),**oracle.observe(path));rows.append(row);print('[DEBUG] '+json.dumps(dict(file=file,solids=len(row['solids']),geometry=row['geometry'])),flush=True)
(root/'🗑️generated/faceted-brep-geometry-independent.json').write_text(json.dumps(dict(nativeExecution=False,rows=rows),ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
