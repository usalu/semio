import json,copy
from pathlib import Path
p=Path(__file__).parent
identity=dict(origin=[0,0,0],originRotation=0,target=[0,0,0],axes=[None,None,None],scale=None,secondary=[None,None])
def layer(**kwargs):return dict(identity,**kwargs)
rows=[dict(id='identity',layers=[layer()],repeatedSource=False),dict(id='origin-translation',layers=[layer(origin=[2,3,4])],repeatedSource=False),dict(id='target-translation',layers=[layer(target=[10,20,30])],repeatedSource=False),dict(id='nonzero-origin-rotation',layers=[layer(origin=[2,3,4],originRotation=.4,target=[10,20,30])],repeatedSource=False),dict(id='reflected-nonuniform',layers=[layer(origin=[2,3,4],originRotation=.4,target=[10,20,30],axes=[[1,0,0],[0,-1,0],[0,0,1]],scale=2,secondary=[3,4])],repeatedSource=False),dict(id='default-axes-axis1',layers=[layer(origin=[2,3,4],originRotation=.4,target=[10,20,30],axes=[[0,2,0],None,None])],repeatedSource=False),dict(id='nested',layers=[layer(origin=[2,3,4],originRotation=.4,target=[10,20,30]),layer(origin=[1,2,0],originRotation=-.2,target=[-4,5,6],scale=2)],repeatedSource=False),dict(id='repeated-source',layers=[layer(target=[10,20,30])],repeatedSource=True)]
refusals=[dict(id='zero-scale',layers=[layer(scale=0)],damage=None),dict(id='negative-scale',layers=[layer(scale=-1)],damage=None),dict(id='parallel-axis1-axis3',layers=[layer(axes=[[0,0,1],None,[0,0,1]])],damage=None),dict(id='cycle',layers=[layer()],damage='cycle'),dict(id='two-dimensional',layers=[layer()],damage='two-dimensional-operator')]
(p/'mapping-neutral-cases.json').write_text(json.dumps(dict(version=1,baseBounds=[[-1,-.5,0],[1,.5,3]],layerOrder='inner-to-outer',cases=rows,refusals=refusals),indent=2)+'\n')
s=(p/'mapping-independent-oracle.py').read_text(encoding="utf-8-sig");s=s.replace("kwargs=dict(LocalOrigin=point(row['target']),Scale=row['scale'])","kwargs=dict(LocalOrigin=point(row['target']),Scale=row['scale'])")
a=s.index('  profile=f.create_entity');b=s.index('  logger=',a)
s=s[:a]+'''  profile=f.create_entity('IfcRectangleProfileDef',ProfileType='AREA',XDim=2.,YDim=1.);solid=f.create_entity('IfcExtrudedAreaSolid',SweptArea=profile,Position=world,ExtrudedDirection=direction([0,0,1]),Depth=3.);rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='SweptSolid',Items=(solid,))
  total=np.identity(4);inverseTotal=np.identity(4);ids=[]
  for data in row['layers']:
   origin=axis(data['origin'],data['originRotation']);mapping=f.create_entity('IfcRepresentationMap',MappingOrigin=origin,MappedRepresentation=rep);kwargs=dict(LocalOrigin=point(data['target']),Scale=data['scale'])
   for i,v in enumerate(data['axes'],1):kwargs['Axis'+str(i)]=direction(v)if v is not None else None
   secondary=data['secondary'];nonuniform=any(v is not None for v in secondary)
   if nonuniform:kwargs.update(Scale2=secondary[0],Scale3=secondary[1])
   op=f.create_entity('IfcCartesianTransformationOperator3DnonUniform'if nonuniform else'IfcCartesianTransformationOperator3D',**kwargs);mapped=f.create_entity('IfcMappedItem',MappingSource=mapping,MappingTarget=op);ids.append(str(mapped.id()));rep=f.create_entity('IfcShapeRepresentation',ContextOfItems=ctx,RepresentationIdentifier='Body',RepresentationType='MappedRepresentation',Items=(mapped,));target=ifcopenshell.util.placement.get_cartesiantransformationoperator3d(op);source=ifcopenshell.util.placement.get_axis2placement(origin);total=target@source@total;inverseTotal=target@np.linalg.inv(source)@inverseTotal
  shape=f.create_entity('IfcProductDefinitionShape',Representations=(rep,));products=[]
  for _ in range(2 if row['repeatedSource']else 1):products.append(f.create_entity('IfcBuildingElementProxy',GlobalId=ifcopenshell.guid.new(),Name=row['id'],ObjectPlacement=f.create_entity('IfcLocalPlacement',RelativePlacement=world),Representation=shape))
  product=products[0]
'''+s[b:]
s=s.replace("mesh=ifcopenshell.geom.create_shape(settings,product).geometry","owner=ifcopenshell.geom.create_shape(settings,product);mesh=owner.geometry")
a=s.index('  target=ifcopenshell.util.placement');b=s.index('  bounds=',a);s=s[:a]+"  corners=[np.array([x,y,z,1.])for x in [-1.,1.]for y in [-.5,.5]for z in [0.,3.]]\n"+s[b:]
s=s.replace('target@source@p','total@p').replace('target@np.linalg.inv(source)@p','inverseTotal@p').replace("mappedId=str(mapped.id())","mappedIds=ids,productIds=[str(p.id())for p in products]").replace("ifcopenshell.util.placement.get_mappeditem_transformation(mapped).tolist()","total.tolist()")
a=s.index('rows=[dict(id=');s=s[:a]+'''data=json.loads((root/'mapping-neutral-cases.json').read_text(encoding="utf-8-sig"));rows=data['cases'];out=root/'🗑️generated/mapping-neutral';out.mkdir(exist_ok=True);oracle=IfcOpenShellMappingOracle();results=[]
for row in rows:
 result=oracle.observe(row);assert result['directMatches'] and not result['diagnostics'];results.append(result);(out/'oracle.json').write_text(json.dumps(dict(version='IfcOpenShell0.9.0.post1',rows=results),indent=2,default=str)+'\\n');print('[DEBUG] '+json.dumps(dict(id=row['id'],direct=result['directMatches'],inverse=result['inverseMatches'],diagnostics=len(result['diagnostics']),bounds=result['actualBounds'])),flush=True)
for row,result in zip(rows,results):
 row['expectedBounds']=result['actualBounds'];row['expectedVolume']=6*abs(float(__import__('numpy').linalg.det(__import__('numpy').array(result['actualMatrix'])[:3,:3])))
(root/'mapping-neutral-cases.json').write_text(json.dumps(data,indent=2)+'\\n')
'''
(p/'mapping-neutral-oracle.py').write_text(s,encoding="utf-8")
