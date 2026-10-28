"""📐 Independent authored direct-counterpart fixtures, never copied original source models."""
import ctypes,json,sys,hashlib,re
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
repo=t
while not(repo/'✏️s').exists():repo=repo.parent
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official';contract=json.loads((t/'alignment-direct-counterpart-vectors.json').read_text(encoding='utf-8-sig'));out=t/'🗑️generated/alignment-direct';out.mkdir(exist_ok=True)
def observe(f,segment):
 s=f.by_id(segment);p=s.DesignParameters;defs=s.Representation;paths=[]
 if defs:
  for r in defs.Representations:
   for item in r.Items:
    paths.append({'definition':defs.id(),'representation':r.id(),'item':item.id(),'type':item.is_a(),'branch':item.SegmentLength.is_a()if item.is_a('IfcCurveSegment')and hasattr(item.SegmentLength,'is_a')else None,'value':item.SegmentLength.wrappedValue if item.is_a('IfcCurveSegment')and hasattr(item.SegmentLength,'wrappedValue')else repr(item.SegmentLength)if item.is_a('IfcCurveSegment')else None})
 return {'segment':s.id(),'parameters':p.id(),'parameterType':p.is_a(),'businessLength':getattr(p,'SegmentLength',getattr(p,'HorizontalLength',None)),'paths':paths}
def make():
 f=ifcopenshell.file(schema='IFC4X3_ADD2');point=lambda xyz:f.create_entity('IfcCartesianPoint',tuple(float(x)for x in xyz));direction=lambda xyz:f.create_entity('IfcDirection',tuple(float(x)for x in xyz));origin=point([0,0,0]);axis=f.create_entity('IfcAxis2Placement3D',Location=origin);placement=f.create_entity('IfcLocalPlacement',RelativePlacement=axis);context=f.create_entity('IfcGeometricRepresentationContext',ContextType='Model',CoordinateSpaceDimension=3,Precision=1e-8,WorldCoordinateSystem=axis);unit=f.create_entity('IfcSIUnit',UnitType='LENGTHUNIT',Name='METRE');assignment=f.create_entity('IfcUnitAssignment',(unit,));f.create_entity('IfcProject',GlobalId='0000000000000000000001',Name='Authored correspondence fixture',RepresentationContexts=(context,),UnitsInContext=assignment);parameters=f.create_entity('IfcAlignmentHorizontalSegment',StartPoint=point([0,0]),StartDirection=0.,StartRadiusOfCurvature=0.,EndRadiusOfCurvature=0.,SegmentLength=0.,PredefinedType='LINE');line=f.create_entity('IfcLine',Pnt=origin,Dir=f.create_entity('IfcVector',Orientation=direction([1,0,0]),Magnitude=1.));curve=f.create_entity('IfcCurveSegment',Transition='DISCONTINUOUS',Placement=axis,SegmentStart=f.create_entity('IfcLengthMeasure',0.),SegmentLength=f.create_entity('IfcLengthMeasure',0.),ParentCurve=line);f.create_entity('IfcCompositeCurve',Segments=(curve,),SelfIntersect=False);representation=f.create_entity('IfcShapeRepresentation',ContextOfItems=context,RepresentationIdentifier='Axis',RepresentationType='Segment',Items=(curve,));definition=f.create_entity('IfcProductDefinitionShape',Representations=(representation,));segment=f.create_entity('IfcAlignmentSegment',GlobalId='0000000000000000000002',ObjectPlacement=placement,Representation=definition,DesignParameters=parameters);return f,segment,parameters,curve,representation,definition,line,axis
physical=[]
for row in contract['physicalCases']:
 path=base/row['file'];digest=hashlib.sha256(path.read_bytes()).hexdigest();assert digest==row['sha256'];f=ifcopenshell.open(str(path));physical.append({'id':row['id'],'sha256':digest,'header':list(f.header.file_schema.schema_identifiers),'expected':row['expected'],'observation':observe(f,int(row['segment'])),'fullExpress':'not rerun; identity/branch observations only'})
rows=[];unsupported={'unknown-parameters','ambiguous-length-owner','duplicate-source-id','nonfinite-select-real'}
for row in contract['neutralCases'] + [{'id':'mixed-case-select-tag','expected':'pass'}]:
 name=row['id'];
 if len(sys.argv)>1 and name!=sys.argv[1]:continue
 record={'id':name,'expected':row['expected'],'requestKind':'vertical' if name=='wrong-known-kind' else 'horizontal','applicable':name!='applicability-false-missing'}
 if name in unsupported:record.update(physical=False,reason='Internal malformed/complex/native-word law; IfcOpenShell physical authority unsupported or source ambiguity not faithfully represented');rows.append(record);continue
 try:
  f,s,p,c,r,d,line,axis=make();selected=s.id();record['requestSegments']=[999] if name=='applicability-false-missing' else [selected,selected] if name=='duplicate-request-target' else [selected];tag='IFCLENGTHMEASURE';length=0.
  if name=='nonzero-selected-business':p.SegmentLength=1.;s.Representation=None
  elif name=='parameter-zero':c.SegmentLength=f.create_entity('IfcParameterValue',0.);tag='IFCPARAMETERVALUE'
  elif name=='signed-negative-zero':c.SegmentLength=f.create_entity('IfcLengthMeasure',-0.0)
  elif name=='nonzero-direct-geometry':c.SegmentLength=f.create_entity('IfcLengthMeasure',2.);f.create_entity('IfcCurveSegment',Transition='DISCONTINUOUS',Placement=axis,SegmentStart=f.create_entity('IfcLengthMeasure',0.),SegmentLength=f.create_entity('IfcLengthMeasure',0.),ParentCurve=line)
  elif name=='missing-direct-geometry':s.Representation=None
  elif name=='two-direct-items':other=f.create_entity('IfcCurveSegment',Transition='DISCONTINUOUS',Placement=axis,SegmentStart=f.create_entity('IfcLengthMeasure',0.),SegmentLength=f.create_entity('IfcLengthMeasure',0.),ParentCurve=line);r.Items=(c,other)
  elif name=='repeated-same-curve-path':r.Items=(c,c)
  elif name=='two-representations':other=f.create_entity('IfcShapeRepresentation',ContextOfItems=r.ContextOfItems,RepresentationIdentifier='Axis',RepresentationType='Segment',Items=(c,));d.Representations=(r,other)
  source=f.to_string()
  if name=='missing-item-reference':source,count=re.subn(r'^#'+str(c.id())+r'=.*;\n','',source,flags=re.M);assert count==1
  elif name=='mixed-case-select-tag':source=source.replace('IFCLENGTHMEASURE(0.),IFCLENGTHMEASURE(0.)','IFCLENGTHMEASURE(0.),iFcLeNgThMeAsUrE(0.)')
  elif name=='raw-untyped-select-zero':source=source.replace('IFCLENGTHMEASURE(0.),IFCLENGTHMEASURE(0.)','IFCLENGTHMEASURE(0.),0.')
  elif name=='unknown-select-tag':source=source.replace('IFCLENGTHMEASURE(0.),IFCLENGTHMEASURE(0.)','IFCLENGTHMEASURE(0.),IFCPOSITIVELENGTHMEASURE(0.)')
  elif name=='bad-select-arity':source=source.replace('IFCLENGTHMEASURE(0.),IFCLENGTHMEASURE(0.)','IFCLENGTHMEASURE(0.),IFCLENGTHMEASURE(0.,0.)')
  path=out/(name+'.ifc');path.write_text(source,encoding='utf-8');parsed=ifcopenshell.file.from_string(source);record.update(physical=True,segment=selected,sha256=hashlib.sha256(path.read_bytes()).hexdigest(),header=list(parsed.header.file_schema.schema_identifiers))
  try:record['observation']=observe(parsed,selected)
  except Exception as error:record['observationError']=str(error)
  logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(parsed,logger,express_rules=True);record['diagnostics']=[{'attribute':x.get('attribute'),'message':str(x.get('message'))}for x in logger.statements]
 except Exception as error:record['constructionError']=str(error)
 rows.append(record)
(out/('oracle-'+sys.argv[1]+'.json' if len(sys.argv)>1 else 'oracle.json')).write_text(json.dumps({'library':ifcopenshell.version,'physicalOriginals':physical,'neutral':rows,'nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'originals':len(physical),'neutral':len(rows),'physical':sum(r.get('physical',False)for r in rows),'errors':[r['id']for r in rows if r.get('constructionError')],'diagnostics':[{r['id']:len(r.get('diagnostics',[]))}for r in rows if r.get('physical')]}))
