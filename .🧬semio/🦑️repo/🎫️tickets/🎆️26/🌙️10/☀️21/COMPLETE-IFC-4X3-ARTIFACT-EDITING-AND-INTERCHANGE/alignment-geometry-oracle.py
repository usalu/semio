"""🔗 Independent exact-path observation of one pinned official alignment example."""
import ctypes,hashlib,json,platform,sys
from pathlib import Path
root=Path(__file__).resolve().parent;dependency=str(root/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dependency,short,len(short));dependency=short.value or dependency
sys.path.insert(0,dependency)
import ifcopenshell
source=root/'🗑️generated'/'alignment-geometry'/'official-example.ifc';digest=hashlib.sha256(source.read_bytes()).hexdigest()
assert digest=='89a424cc27f58130d40642b369b93a9b5fb04df2804f1f52ec3fac457f34c025'
f=ifcopenshell.open(str(source));rows=[]
header_schemas=list(f.header.file_schema.schema_identifiers)
assert header_schemas==['IFC4X3_ADD2']
for kind,layout_id,relation_id,segment_id,parameter_id,representation_id,shape_id,curve_id,parent_id in [('horizontal',34,44,1101,1102,1631,1632,1201,54),('vertical',45,52,2001,2002,2631,2632,2101,79)]:
 layout=f.by_id(layout_id);relation=f.by_id(relation_id);segments=relation.RelatedObjects;segment=segments[-1];parameter=segment.DesignParameters;representation=segment.Representation;shapes=representation.Representations;shape=next(s for s in shapes if s.id()==shape_id);curve=next(c for c in shape.Items if c.id()==curve_id);parent=f.by_id(parent_id);measure=curve.SegmentLength
 observed=[segment.id(),parameter.id(),representation.id(),shape.id(),curve.id()];expected=[segment_id,parameter_id,representation_id,shape_id,curve_id]
 assert relation.RelatingObject==layout and observed==expected
 assert curve.is_a('IfcCurveSegment') and measure.is_a()=='IfcLengthMeasure' and measure.wrappedValue==0
 assert parent.Segments[-1]==curve
 rows.append({'kind':kind,'layout':layout.id(),'layoutType':layout.is_a(),'relation':relation.id(),'businessSegments':[s.id() for s in segments],'terminal':segment.id(),'parameter':parameter.id(),'parameterType':parameter.is_a(),'businessLength':getattr(parameter,'SegmentLength' if kind=='horizontal' else 'HorizontalLength'),'representation':representation.id(),'representationType':representation.is_a(),'shape':shape.id(),'identifier':shape.RepresentationIdentifier,'shapeType':shape.RepresentationType,'curve':curve.id(),'curveType':curve.is_a(),'segmentLengthBranch':measure.is_a(),'segmentLengthValue':measure.wrappedValue,'parentCurve':parent.id(),'parentCurveType':parent.is_a(),'parentSegments':[s.id() for s in parent.Segments],'parentTerminalIsExactCounterpart':parent.Segments[-1]==curve,'baseCurve':getattr(getattr(parent,'BaseCurve',None),'id',lambda:None)(),'match':observed==expected})
alignment=f.by_id(29);parent_shapes=alignment.Representation.Representations
result={'sourceSha256':digest,'sourceSchemaFamily':f.schema,'sourceHeaderSchemas':header_schemas,'sourceSchemaIdentifier':getattr(f,'schema_identifier',None),'parser':'IfcOpenShell','version':ifcopenshell.version,'python':platform.python_version(),'rows':rows,'alignment':29,'alignmentRepresentation':alignment.Representation.id(),'alignmentShapes':[{'id':s.id(),'identifier':s.RepresentationIdentifier,'type':s.RepresentationType,'items':[i.id() for i in s.Items]} for s in parent_shapes],'schemaConformance':'not-evaluated','viewConformance':'not-evaluated','nativeExecution':False}
out=root/'🗑️generated'/'alignment-geometry'/'oracle.json';out.write_text(json.dumps(result,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'rows':len(rows),'matches':sum(row['match'] for row in rows),'version':ifcopenshell.version,'schema':f.schema}))

