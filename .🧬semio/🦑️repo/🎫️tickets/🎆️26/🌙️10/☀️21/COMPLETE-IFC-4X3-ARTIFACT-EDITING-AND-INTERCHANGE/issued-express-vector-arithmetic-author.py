"""➕ Author exact scalar/vector branches over the unchanged closed source population."""
import json
import math
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector-arithmetic'
root.mkdir(exist_ok=True)
rows=[]
unknown=dict(kind='unknown')
def entity(id):return unknown.copy() if id is None else dict(kind='entity',id=str(id))
def scalar(value):return unknown.copy() if value is None else dict(kind='real',value=str(value))
def vector(orientation,magnitude):return dict(kind='vector',orientation=orientation,magnitude=magnitude)
def case(owner,id,args,expected,tolerance=0):rows.append(dict(owner=owner,id=id,args=args,expected=expected,tolerance=tolerance))
for id,n,argument,orientation,magnitude in [('direction-positive',2,4,[.6,.8],2),('direction-negative',-2,4,[-.6,-.8],2),('direction-zero',0,4,[.6,.8],0),('vector-positive',2,11,[.6,.8,0],10),('vector-negative',-2,11,[-.6,-.8,0],10),('zero-vector',-2,12,[.6,.8,0],0)]:case('IFCSCALARTIMESVECTOR',id,[scalar(n),entity(argument)],vector(orientation,magnitude))
case('IFCSCALARTIMESVECTOR','absent-scalar',[scalar(None),entity(11)],unknown);case('IFCSCALARTIMESVECTOR','absent-vector',[scalar(2),entity(None)],unknown)
for owner,operation in [('IFCVECTORSUM',1),('IFCVECTORDIFFERENCE',-1)]:
 case(owner,'orthogonal2',[entity(1),entity(2)],vector([1,operation],math.sqrt(2)),1e-15)
 case(owner,'scaled2',[entity(4),entity(1)],vector([.6+operation,.8],math.sqrt((.6+operation)**2+.8**2)),1e-15)
 case(owner,'parallel2',[entity(1),entity(1)],vector([2,0] if operation==1 else [1,0],2 if operation==1 else 0))
 case(owner,'opposite2',[entity(1),entity(3)],vector([1,0] if operation==1 else [2,0],0 if operation==1 else 2))
 case(owner,'vector-direction3',[entity(11),entity(5)],vector([3+operation,4,0],math.sqrt((3+operation)**2+16)),1e-15)
 case(owner,'vector-vector3',[entity(11),entity(11)],vector([6,8,0] if operation==1 else [.6,.8,0],10 if operation==1 else 0))
 case(owner,'zero-vector3',[entity(11),entity(12)],vector([3,4,0],5))
 case(owner,'dimension-mismatch',[entity(1),entity(5)],unknown)
 case(owner,'absent-left',[entity(None),entity(5)],unknown);case(owner,'absent-right',[entity(5),entity(None)],unknown)
data=dict(version=1,numericEquality='finite-real-value-with-zero-sign-equivalence',source=dict(file='../🧪️vector/source.ifc',schema='IFC4X3_ADD2',expSha256='f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860',physical='unchanged authored closed vector source; no duplicate source asset'),functions=rows)
(root/'cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
argument={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind','id'],'properties':{'kind':{'const':'entity'},'id':{'type':'string','pattern':'^[1-9][0-9]*$'}}},{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'real'},'value':{'type':'string','pattern':'^-?[0-9]+(?:\\.[0-9]+)?$'}}},{'type':'object','additionalProperties':False,'required':['kind'],'properties':{'kind':{'const':'unknown'}}}]}
expected={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind'],'properties':{'kind':{'const':'unknown'}}},{'type':'object','additionalProperties':False,'required':['kind','orientation','magnitude'],'properties':{'kind':{'const':'vector'},'orientation':{'type':'array','minItems':2,'maxItems':3,'items':{'type':'number'}},'magnitude':{'type':'number','minimum':0}}}]}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['version','numericEquality','source','functions'],'properties':{'version':{'const':1},'numericEquality':{'const':data['numericEquality']},'source':{'type':'object','additionalProperties':False,'required':['file','schema','expSha256','physical'],'properties':{'file':{'const':data['source']['file']},'schema':{'const':'IFC4X3_ADD2'},'expSha256':{'const':data['source']['expSha256']},'physical':{'type':'string'}}},'functions':{'type':'array','minItems':28,'items':{'type':'object','additionalProperties':False,'required':['id','owner','args','expected','tolerance'],'properties':{'id':{'type':'string','minLength':1},'owner':{'enum':['IFCSCALARTIMESVECTOR','IFCVECTORSUM','IFCVECTORDIFFERENCE']},'args':{'type':'array','minItems':2,'maxItems':2,'items':argument},'expected':expected,'tolerance':{'type':'number','minimum':0,'maximum':1e-15}}}}}}
(root/'cases.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(cases=len(rows),owners=3)))
