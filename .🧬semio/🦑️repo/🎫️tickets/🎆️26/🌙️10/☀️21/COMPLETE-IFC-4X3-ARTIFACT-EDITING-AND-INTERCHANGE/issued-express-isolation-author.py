"""🪟 Author language-neutral root-query isolation and procedural alias laws."""
import json
from pathlib import Path
ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector-arithmetic'
rows=[
('local-alias','FUNCTION Probe(D:IfcDirection):REAL; LOCAL A,B:IfcDirection; END_LOCAL; A:=D; B:=A; B.DirectionRatios[1]:=9.0; RETURN(A.DirectionRatios[1]); END_FUNCTION;', ['4'],'real','9'),
('duplicate-argument-alias','FUNCTION Probe(A,B:IfcDirection):REAL; A.DirectionRatios[1]:=9.0; RETURN(B.DirectionRatios[1]); END_FUNCTION;', ['4','4'],'real','9'),
('aggregate-assignment-copy','FUNCTION Probe(D:IfcDirection):REAL; LOCAL A:LIST[2:3] OF REAL; END_LOCAL; A:=D.DirectionRatios; A[1]:=9.0; RETURN(D.DirectionRatios[1]); END_FUNCTION;', ['4'],'real','3'),
('derived-local-self','FUNCTION Probe(D:IfcDirection):INTEGER; D.DirectionRatios:=[1.0,2.0,3.0]; RETURN(D.Dim); END_FUNCTION;', ['4'],'integer','3'),
('qualified-derived-self','FUNCTION Probe(D:IfcDirection):INTEGER; D.DirectionRatios:=[1.0,2.0,3.0]; RETURN(D\\IfcDirection.Dim); END_FUNCTION;', ['4'],'integer','3'),
('retained-identity','FUNCTION Probe(A,B:IfcDirection):BOOLEAN; A.DirectionRatios[1]:=9.0; RETURN(A :=: B); END_FUNCTION;', ['4','4'],'logical','true'),
('returned-local-reference','FUNCTION Probe(D:IfcDirection):IfcDirection; D.DirectionRatios[1]:=9.0; RETURN(D); END_FUNCTION;', ['4'],'direction','9,4'),
('nested-alias','FUNCTION Probe(D:IfcDirection):REAL; LOCAL A:IfcDirection; END_LOCAL; A:=Change(D); RETURN(D.DirectionRatios[1]+A.DirectionRatios[1]); END_FUNCTION;', ['4'],'real','18'),
('failed-root-isolated','FUNCTION Probe(D:IfcDirection):REAL; D.DirectionRatios[1]:=9.0; RETURN(UnsupportedCall(D)); END_FUNCTION;', ['4'],'error','unsupported-call')]
data=dict(version=1,scope='one-public-query-with-nested-procedures',source='../🧪️vector/source.ifc',helper='FUNCTION Change(D:IfcDirection):IfcDirection; D.DirectionRatios[1]:=9.0; RETURN(D); END_FUNCTION;',cases=[dict(id=id,source=source,args=args,expected=dict(kind=kind,value=value)) for id,source,args,kind,value in rows])
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':list(data),'properties':{'version':{'const':1},'scope':{'const':data['scope']},'source':{'const':data['source']},'helper':{'type':'string','minLength':1},'cases':{'type':'array','minItems':9,'items':{'type':'object','additionalProperties':False,'required':['id','source','args','expected'],'properties':{'id':{'type':'string','minLength':1},'source':{'type':'string','minLength':1},'args':{'type':'array','minItems':1,'items':{'type':'string','pattern':'^[1-9][0-9]*$'}},'expected':{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'enum':['real','integer','logical','direction','error']},'value':{'type':'string'}}}}}}}}
for name,value in [('isolation.json',data),('isolation.schema.json',schema)]: (root/name).write_text(json.dumps(value,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] authored '+str(len(rows))+' isolation laws')
