"""🧱 Build authored closed faceted test inputs through a test-only oracle interface."""
import ctypes,hashlib,json,math,re,sys
from pathlib import Path
root=Path(__file__).resolve().parent;dep=str(root/'🗑️generated/oracle')
if sys.platform=='win32':
    buffer=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,buffer,len(buffer));dep=buffer.value or dep
sys.path.insert(0,dep)
class FacetedFixtureOracle:
    def run(self,row,ordinal):raise NotImplementedError
class IfcOpenShellFacetedFixtureOracle(FacetedFixtureOracle):
    def run(self,row,ordinal):
        import ifcopenshell,ifcopenshell.geom,ifcopenshell.guid,ifcopenshell.validate
        f=ifcopenshell.file(schema='IFC4X3_ADD2');mode=row['modifier']['kind'];scale=row['unit']['coordinateScale'];cache={}
        def point(p):
            p=tuple(float(x)*scale for x in p)
            if p not in cache:cache[p]=f.create_entity('IfcCartesianPoint',Coordinates=p)
            return cache[p]
        zero=f.create_entity('IfcCartesianPoint',Coordinates=(0.,0.,0.));z=f.create_entity('IfcDirection',DirectionRatios=(0.,0.,1.));x=f.create_entity('IfcDirection',DirectionRatios=(1.,0.,0.));axis=f.create_entity('IfcAxis2Placement3D',Location=zero,Axis=z,RefDirection=x);ctx=f.create_entity('IfcGeometricRepresentationContext',ContextType='Model',CoordinateSpaceDimension=3,Precision=1.e-8,WorldCoordinateSystem=axis);unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE',Prefix=row['unit']['prefix']);units=f.create_entity('IfcUnitAssignment',Units=(unit,));f.create_entity('IfcProject',Name='Faceted authored test',GlobalId=ifcopenshell.guid.compress('%032x'%ordinal),RepresentationContexts=(ctx,),UnitsInContext=units)
        geometry=row['geometry'];outer=geometry['outer'];holes=geometry['holes'];height=geometry['height']
        loops=[];faces=[]
        def face(rings,reverse=False):
            bounds=[]
            for index,ring in enumerate(rings):
                actual=list(reversed(ring)) if row['orientation']['reversePolygon']^reverse else list(ring);loop=f.create_entity('IfcPolyLoop',Polygon=tuple(point(p)for p in actual));loops.append(loop);bound=f.create_entity('IfcFaceOuterBound'if index==0 and mode!='no-outer'else'IfcFaceBound',Bound=loop,Orientation=row['orientation']['bound']);bounds.append(bound)
            if mode=='outer-last' and len(bounds)>1:bounds=bounds[1:]+bounds[:1]
            entity=f.create_entity('IfcFace',Bounds=tuple(bounds));faces.append(entity);return entity
        def prism(ring,inner,base,depth,reverse=False):
            local=[];bottom=lambda r:[(a,b,base)for a,b in r];top=lambda r:[(a,b,base+depth)for a,b in r]
            local.append(face([list(reversed(bottom(r)))for r in[ring]+inner],reverse));local.append(face([top(r)for r in[ring]+inner],reverse))
            for r in[ring]+inner:
                for i,a in enumerate(r):
                    b=r[(i+1)%len(r)];local.append(face([[(a[0],a[1],base),(b[0],b[1],base),(b[0],b[1],base+depth),(a[0],a[1],base+depth)]],reverse))
            return f.create_entity('IfcClosedShell',CfsFaces=tuple(local))
        shell=prism(outer,holes,geometry['base'],height)
        if geometry['voids']:
            cavities=tuple(prism(v['outer'],v['holes'],v['base'],v['height'],v['reverse'])for v in geometry['voids']);solid=f.create_entity('IfcFacetedBrepWithVoids',Outer=shell,Voids=cavities)
        else:solid=f.create_entity('IfcFacetedBrep',Outer=shell)
        modifier=row['modifier']
        if mode=='point-coordinates':
            selected=list(cache.values())[modifier['ordinal']];selected.Coordinates=tuple(modifier['coordinates'])if'coordinates'in modifier else selected.Coordinates[:modifier['dimensions']]
        if mode=='polygon-order':
            polygon=loops[modifier['ordinal']].Polygon;loops[modifier['ordinal']].Polygon=tuple(polygon[i]for i in modifier['order'])
        if mode=='two-outer':faces[0].Bounds=faces[0].Bounds+(f.create_entity('IfcFaceOuterBound',Bound=loops[0],Orientation=True),)
        if mode=='empty-bounds':faces[0].Bounds=()
        rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='Brep',Items=(solid,));shape=f.create_entity('IfcProductDefinitionShape',Representations=(rep,));local=axis
        pose=row['placement'];local=f.create_entity('IfcAxis2Placement3D',Location=f.create_entity('IfcCartesianPoint',Coordinates=tuple(float(x)for x in pose['origin'])),Axis=z,RefDirection=f.create_entity('IfcDirection',DirectionRatios=(math.cos(pose['rotation']),math.sin(pose['rotation']),0.)))
        placement=f.create_entity('IfcLocalPlacement',RelativePlacement=local);product=f.create_entity('IfcBuildingElementProxy',Name='Faceted authored product',GlobalId=ifcopenshell.guid.compress('%032x'%(ordinal+100)),ObjectPlacement=placement,Representation=shape)
        raw=f.to_string()
        if mode=='wrong-bound':
            plane=f.create_entity('IfcPlane',Position=axis);raw=f.to_string();raw=re.sub(r'#'+str(faces[0].id())+r'=IFCFACE\([^;]+;',f'#{faces[0].id()}=IFCFACE((#{plane.id()}));',raw)
        if mode=='wrong-loop':raw=re.sub(r'#'+str(faces[0].Bounds[0].id())+r'=IFCFACEOUTERBOUND\([^;]+;',f'#{faces[0].Bounds[0].id()}=IFCFACEOUTERBOUND(#{zero.id()},.T.);',raw)
        if mode=='wrong-point':raw=re.sub(r'#'+str(loops[0].id())+r'=IFCPOLYLOOP\([^;]+;',f'#{loops[0].id()}=IFCPOLYLOOP((#{z.id()},'+','.join('#'+str(p.id())for p in loops[0].Polygon[1:])+'));',raw)
        path=out/(row['id']+'.ifc');path.write_text(raw,encoding='utf-8');model=ifcopenshell.open(str(path));result=dict(id=row['id'],expected=row['expected'],file=path.name,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(model.header.file_schema.schema_identifiers),product=str(product.id()),solid=str(solid.id()))
        try:
            logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True);result['diagnostics']=logger.statements
        except Exception as error:result['validationError']=str(error)
        try:
            settings=ifcopenshell.geom.settings();settings.set(settings.USE_WORLD_COORDS,True);owner=ifcopenshell.geom.create_shape(settings,model.by_id(product.id()));mesh=owner.geometry;v=list(mesh.verts);indices=list(mesh.faces);points=[v[i:i+3]for i in range(0,len(v),3)];terms=[]
            for at in range(0,len(indices),3):
                a,b,c=(points[indices[at+i]]for i in range(3));terms.append((a[0]*(b[1]*c[2]-b[2]*c[1])+a[1]*(b[2]*c[0]-b[0]*c[2])+a[2]*(b[0]*c[1]-b[1]*c[0]))/6)
            result['geometry']=dict(vertices=len(points),triangles=len(indices)//3,bounds=[[min(p[k]for p in points)for k in range(3)],[max(p[k]for p in points)for k in range(3)]],signedVolume=math.fsum(terms))
        except Exception as error:result['geometryError']=str(error)
        if row['expected']=='supported':
            assert not result.get('diagnostics')and'validationError'not in result,result
            if geometry['voids']:
                result['compoundGeometryOracle']='unavailable; separate actual shell meshes and Three oriented-volume certificate recorded independently'
            else:assert math.isclose(abs(result['geometry']['signedVolume']),row['volume'],rel_tol=1.e-9,abs_tol=1.e-9),result
        return result
out=root/'🗑️generated/faceted-brep-vectors';out.mkdir(exist_ok=True);vectors=json.loads((root/'faceted-brep-geometry-vectors.json').read_text(encoding='utf-8-sig'));oracle=IfcOpenShellFacetedFixtureOracle();rows=[]
for ordinal,row in enumerate(vectors['cases'],1):
    result=oracle.run(row,ordinal);rows.append(result);(out/'oracle.json').write_text(json.dumps(dict(version='IfcOpenShell0.9.0.post1',nativeExecution=False,rows=rows),ensure_ascii=False,indent=2,default=str)+'\n',encoding='utf-8');print('[DEBUG] '+json.dumps(dict(id=row['id'],diagnostics=len(result.get('diagnostics',[])),geometry=result.get('geometry'),geometryError=result.get('geometryError'))),flush=True)



