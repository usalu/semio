"""📐 Independent physical ordered alignment business segment oracle, not full-view certification."""
import ctypes,json,sys
from pathlib import Path
root=Path(__file__).resolve().parent;path=str(root/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(path,short,len(short));path=short.value or path
sys.path.insert(0,path)
import ifcopenshell,ifcopenshell.guid
contract=json.loads((root/'alignment-terminal-cases.json').read_text(encoding='utf-8-sig'));f=ifcopenshell.file(schema='IFC4X3_ADD2');rows=[]
def entity(name,**kw):
 if name in ['IfcAlignment','IfcAlignmentHorizontal','IfcAlignmentVertical','IfcAlignmentCant','IfcAlignmentSegment','IfcRelNests']:kw['GlobalId']=ifcopenshell.guid.compress('%032x'%(len(list(f))+1))
 return f.create_entity(name,**kw)
def parameters(kind,length):
 if kind=='horizontal':return entity('IfcAlignmentHorizontalSegment',StartPoint=entity('IfcCartesianPoint',Coordinates=(0.,0.)),StartDirection=0.,StartRadiusOfCurvature=0.,EndRadiusOfCurvature=0.,SegmentLength=length,PredefinedType='LINE')
 if kind=='vertical':return entity('IfcAlignmentVerticalSegment',StartDistAlong=0.,HorizontalLength=length,StartHeight=0.,StartGradient=0.,EndGradient=0.,PredefinedType='CONSTANTGRADIENT')
 return entity('IfcAlignmentCantSegment',StartDistAlong=0.,HorizontalLength=length,StartCantLeft=0.,StartCantRight=0.,PredefinedType='CONSTANTCANT')
for case in contract['cases']:
 kind=case['layout'];alignment=entity('IfcAlignment',Name=case['id']);layout=entity({'horizontal':'IfcAlignmentHorizontal','vertical':'IfcAlignmentVertical','cant':'IfcAlignmentCant'}[kind],**({'RailHeadDistance':1.5} if kind=='cant' else {}));parent=entity('IfcRelNests',RelatingObject=alignment,RelatedObjects=(layout,));segments=[]
 for length in case['lengths']:segments.append(entity('IfcAlignmentSegment',DesignParameters=parameters(kind,length)))
 if case.get('wrongDesign'):segments[-1].DesignParameters=parameters('vertical',0.)
 nesting=entity('IfcRelNests',RelatingObject=layout,RelatedObjects=tuple(segments)) if segments else None
 ordered=tuple(nesting.RelatedObjects) if nesting else ();attribute='SegmentLength' if kind=='horizontal' else 'HorizontalLength';expectedType={'horizontal':'IfcAlignmentHorizontalSegment','vertical':'IfcAlignmentVerticalSegment','cant':'IfcAlignmentCantSegment'}[kind]
 status='not-applicable' if case['applicable'] is False else 'incomplete' if case['applicable'] is None or not ordered or not ordered[-1].DesignParameters.is_a(expectedType) else 'pass' if getattr(ordered[-1].DesignParameters,attribute)==0 else 'fail'
 rows.append({'id':case['id'],'alignmentId':alignment.id(),'layoutId':layout.id(),'parentNestingId':parent.id(),'segmentNestingId':nesting.id() if nesting else None,'segmentIds':[s.id() for s in ordered],'designParameterIds':[s.DesignParameters.id() for s in ordered],'terminalAttribute':attribute,'terminalValue':getattr(ordered[-1].DesignParameters,attribute,None) if ordered else None,'actual':status,'expected':case['expected'],'match':status==case['expected'],'geometryCounterpart':'not-applicable-no-geometry','schemaConformance':'not-evaluated','viewConformance':'not-evaluated'})
out=root/'🗑️generated'/'alignment-terminal';out.mkdir(exist_ok=True);f.write(str(out/'graphs.ifc'));(out/'oracle.json').write_text(json.dumps({'oracle':'IfcOpenShell '+ifcopenshell.version,'rows':rows,'nativeExecution':False},indent=2),encoding='utf8');print('[DEBUG] '+json.dumps({'cases':len(rows),'matches':sum(r['match'] for r in rows)}));assert all(r['match'] for r in rows)

