"""🚆 Independent original official corpus cant counterpart census; no geometric inference."""
import ctypes,hashlib,json,sys
from pathlib import Path
root=Path(__file__).resolve().parent;dep=str(root/'🗑️generated'/'oracle')
if sys.platform=='win32':
 short=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,short,len(short));dep=short.value or dep
sys.path.insert(0,dep)
import ifcopenshell
repo=root
while not (repo/'✏️s').exists():repo=repo.parent
base=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official'
manifest=json.loads((base/'cases.json').read_text(encoding='utf-8-sig'));files=[]
for case in manifest['cases']:
 if 'alignment' not in case['path']:continue
 path=base/case['file'];digest=hashlib.sha256(path.read_bytes()).hexdigest();assert digest==case['sha256']
 f=ifcopenshell.open(str(path));layouts=[]
 for layout in f.by_type('IfcAlignmentCant'):
  relations=[r for r in f.by_type('IfcRelNests') if r.RelatingObject==layout];lists=[]
  for relation in relations:
   members=list(relation.RelatedObjects);terminal=members[-1] if members else None;parameter=getattr(terminal,'DesignParameters',None);representation=getattr(terminal,'Representation',None);curves=[]
   for shape in getattr(representation,'Representations',[]) or []:
    for curve in shape.Items:
     if not curve.is_a('IfcCurveSegment'):continue
     length=curve.SegmentLength;parents=[{'id':parent.id(),'type':parent.is_a(),'position':list(parent.Segments).index(curve),'count':len(parent.Segments),'last':parent.Segments[-1]==curve,'baseCurve':getattr(getattr(parent,'BaseCurve',None),'id',lambda:None)()} for parent in f.by_type('IfcCompositeCurve') if curve in parent.Segments]
     curves.append({'representation':representation.id(),'shape':shape.id(),'identifier':shape.RepresentationIdentifier,'shapeType':shape.RepresentationType,'curve':curve.id(),'branch':length.is_a(),'value':length.wrappedValue,'parents':parents})
   lists.append({'relation':relation.id(),'members':[s.id() for s in members],'terminal':terminal.id() if terminal else None,'parameter':parameter.id() if parameter else None,'parameterType':parameter.is_a() if parameter else None,'businessLength':getattr(parameter,'HorizontalLength',None),'directCurves':curves})
  layouts.append({'layout':layout.id(),'lists':lists})
 files.append({'file':case['file'],'url':case['url'],'sha256':digest,'header':list(f.header.file_schema.schema_identifiers),'schemaIdentifier':getattr(f,'schema_identifier',None),'cantLayouts':layouts,'segmentedReferenceCurves':[{'id':c.id(),'segments':[s.id() for s in c.Segments],'baseCurve':c.BaseCurve.id(),'terminalBranch':c.Segments[-1].SegmentLength.is_a(),'terminalValue':c.Segments[-1].SegmentLength.wrappedValue} for c in f.by_type('IfcSegmentedReferenceCurve') if c.Segments]})
result={'oracle':'IfcOpenShell','version':ifcopenshell.version,'commit':manifest['commit'],'files':files,'nativeExecution':False,'schemaConformance':'not-evaluated','viewConformance':'not-evaluated'}
out=root/'🗑️generated/alignment-geometry/cant-oracle.json';out.write_text(json.dumps(result,indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps({'files':len(files),'cantLayouts':sum(len(x['cantLayouts']) for x in files),'directCurves':sum(len(l['directCurves']) for x in files for c in x['cantLayouts'] for l in c['lists'])}))

