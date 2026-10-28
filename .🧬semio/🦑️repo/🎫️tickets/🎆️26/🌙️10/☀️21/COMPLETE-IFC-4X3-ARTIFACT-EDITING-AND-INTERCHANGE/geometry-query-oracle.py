"""🧬 Observe third-party handling of authored wrapper and external complex encodings."""
import ctypes,json,sys
from pathlib import Path
t=Path(__file__).resolve().parent;dep=str(t/'🗑️generated/oracle')
if sys.platform=='win32':
 s=ctypes.create_unicode_buffer(32768);ctypes.windll.kernel32.GetShortPathNameW(dep,s,len(s));dep=s.value or dep
sys.path.insert(0,dep)
import ifcopenshell,ifcopenshell.validate
text=(t/'🗑️generated/rectangle-actual/base.ifc').read_text();profile='#7=IFCRECTANGLEPROFILEDEF(.AREA.,$,$,2.,4.);';cases={'raw':text,'positive-wrapper':text.replace(profile,'#7=IFCRECTANGLEPROFILEDEF(.AREA.,$,$,IFCPOSITIVELENGTHMEASURE(2.),4.);'),'length-wrapper':text.replace(profile,'#7=IFCRECTANGLEPROFILEDEF(.AREA.,$,$,IFCLENGTHMEASURE(2.),4.);'),'nested-wrapper':text.replace(profile,'#7=IFCRECTANGLEPROFILEDEF(.AREA.,$,$,IFCPOSITIVELENGTHMEASURE(IFCLENGTHMEASURE(2.)),4.);'),'wrong-wrapper':text.replace(profile,"#7=IFCRECTANGLEPROFILEDEF(.AREA.,$,$,IFCLABEL('2'),4.);"),'complex-profile':text.replace(profile,'#7=(IFCPARAMETERIZEDPROFILEDEF($) IFCPROFILEDEF(.AREA.,$) IFCRECTANGLEPROFILEDEF(2.,4.));'),'complex-solid':text.replace('#9=IFCEXTRUDEDAREASOLID(#7,$,#8,3.);','#9=(IFCEXTRUDEDAREASOLID(#8,3.) IFCGEOMETRICREPRESENTATIONITEM() IFCREPRESENTATIONITEM() IFCSOLIDMODEL() IFCSWEPTAREASOLID(#7,$));'),'complex-point':text.replace('#1=IFCCARTESIANPOINT((0.,0.,0.));','#1=(IFCCARTESIANPOINT((0.,0.,0.)) IFCGEOMETRICREPRESENTATIONITEM() IFCPOINT() IFCREPRESENTATIONITEM());')};out=t/'🗑️generated/geometry-query';out.mkdir(exist_ok=True);rows=[]
for name,source in cases.items():
 (out/(name+'.ifc')).write_text(source);row={'id':name}
 try:
  f=ifcopenshell.file.from_string(source);row['parsed']=True
  try:row['profileX']=repr(f.by_id(7).XDim);row['point']=repr(f.by_id(1));row['solid']=repr(f.by_id(9))
  except Exception as error:row['projectionError']=str(error)
  logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,logger,express_rules=True);row['diagnostics']=[{'level':r.get('level'),'attribute':r.get('attribute'),'message':str(r.get('message'))}for r in logger.statements]
 except Exception as error:row['parseError']=str(error)
 rows.append(row)
(out/'oracle.json').write_text(json.dumps({'library':ifcopenshell.version,'rows':rows,'scope':'Parser behavior observations, not normative proof of external complex validity','nativeExecution':False},indent=2),encoding='utf-8');print('[DEBUG] '+json.dumps([{'id':r['id'],'parsed':r.get('parsed',False),'diagnostics':len(r.get('diagnostics',[])),'x':r.get('profileX'),'error':r.get('parseError')}for r in rows]))
