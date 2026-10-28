"""🧭 Author discriminating issued vector-owner inputs from the pinned released formulas."""
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️vector'
root.mkdir(exist_ok=True)
directions={1:[1.,0.],2:[0.,1.],3:[-1.,0.],4:[3.,4.],5:[1.,0.,0.],6:[0.,1.,0.],7:[-1.,0.,0.],8:[3.,4.,0.],9:[0.,0.,1.],10:[1.,2.,2.]}
body='\n'.join('#'+str(i)+'=IFCDIRECTION(('+','.join(str(n) for n in values)+'));' for i,values in directions.items())+'\n#11=IFCVECTOR(#8,5.);\n#12=IFCVECTOR(#8,0.);\n'
step="ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('Authored issued vector cases'),'2;1');\nFILE_NAME('vectors','2026-10-22T00:00:00',('semio'),('semio'),'semio','semio','');\nFILE_SCHEMA(('IFC4X3_ADD2'));\nENDSEC;\nDATA;\n"+body+'ENDSEC;\nEND-ISO-10303-21;\n'
(root/'source.ifc').write_text(step,encoding='utf-8')
rows=[]
def direction(values):return dict(kind='direction',ratios=values)
def vector(values,magnitude):return dict(kind='vector',orientation=values,magnitude=magnitude)
def case(owner,id,args,expected,tolerance=0):rows.append(dict(owner=owner,id=id,args=[str(arg) if arg is not None else None for arg in args],expected=expected,tolerance=tolerance))
unknown=dict(kind='unknown')
for id,arg,values in [('axis2',1,[1,0]),('opposite2',3,[-1,0]),('three-four2',4,[.6,.8]),('axis3',9,[0,0,1]),('three-four3',8,[.6,.8,0]),('diagonal3',10,[1/3,2/3,2/3])]:case('IFCNORMALISE',id,[arg],direction(values),1e-15 if id=='diagonal3' else 0)
case('IFCNORMALISE','vector',[11],vector([.6,.8,0],1))
case('IFCNORMALISE','zero-vector',[12],unknown)
case('IFCNORMALISE','absent',[None],unknown)
for id,args,value in [('parallel2',[1,1],1),('opposite2',[1,3],-1),('orthogonal2',[1,2],0),('scaled2',[1,4],.6),('parallel3',[5,5],1),('orthogonal3',[5,9],0),('scaled3',[5,8],.6)]:case('IFCDOTPRODUCT',id,args,dict(kind='real',value=value))
for id,args in [('mixed-dimension',[1,5]),('absent-left',[None,5]),('absent-right',[5,None])]:case('IFCDOTPRODUCT',id,args,unknown)
for id,args,values,mag in [('right-handed',[5,6],[0,0,1],1),('reverse',[6,5],[0,0,-1],1),('parallel',[5,5],[1,0,0],0),('antiparallel',[5,7],[1,0,0],0),('parallel-scaled',[8,8],[3,4,0],0),('oblique',[5,8],[0,0,.8],.8)]:case('IFCCROSSPRODUCT',id,args,vector(values,mag))
for id,args in [('two-dimensional-left',[1,5]),('two-dimensional-right',[5,1]),('absent-left',[None,5]),('absent-right',[5,None])]:case('IFCCROSSPRODUCT',id,args,unknown)
for id,arg,values in [('x',1,[0,1]),('y',2,[-1,0]),('scaled',4,[-4,3])]:case('IFCORTHOGONALCOMPLEMENT',id,[arg],direction(values))
for id,arg in [('three-dimensional',5),('absent',None)]:case('IFCORTHOGONALCOMPLEMENT',id,[arg],unknown)
for id,arg,values in [('default',None,[[1,0],[0,1]]),('scaled',4,[[.6,.8],[-.8,.6]]),('negative',3,[[-1,0],[0,-1]])]:case('IFCBUILD2AXES',id,[arg],dict(kind='directions',items=values))
for id,args,values in [('default',[None,None],[[1,0,0],[0,1,0],[0,0,1]]),('explicit-default',[9,5],[[1,0,0],[0,1,0],[0,0,1]]),('opposite-reference',[9,7],[[-1,0,0],[0,-1,0],[0,0,1]]),('y-axis',[6,5],[[1,0,0],[0,0,-1],[0,1,0]])]:case('IFCBUILDAXES',id,args,dict(kind='directions',items=values))
data=dict(version=1,numericEquality='finite-real-value-with-zero-sign-equivalence',source=dict(schema='IFC4X3_ADD2',file='source.ifc',expSha256='f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860',physical='authored standalone direction/vector population; no project/view claim'),functions=rows)
(root/'cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
scalar={'type':'number'}
ratios={'type':'array','minItems':2,'maxItems':3,'items':scalar}
result={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind'],'properties':{'kind':{'const':'unknown'}}},{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'real'},'value':scalar}},{'type':'object','additionalProperties':False,'required':['kind','ratios'],'properties':{'kind':{'const':'direction'},'ratios':ratios}},{'type':'object','additionalProperties':False,'required':['kind','orientation','magnitude'],'properties':{'kind':{'const':'vector'},'orientation':ratios,'magnitude':scalar}},{'type':'object','additionalProperties':False,'required':['kind','items'],'properties':{'kind':{'const':'directions'},'items':{'type':'array','minItems':2,'maxItems':3,'items':ratios}}}]}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['version','numericEquality','source','functions'],'properties':{'version':{'const':1},'numericEquality':{'const':'finite-real-value-with-zero-sign-equivalence'},'source':{'type':'object','additionalProperties':False,'required':['schema','file','expSha256','physical'],'properties':{'schema':{'const':'IFC4X3_ADD2'},'file':{'const':'source.ifc'},'expSha256':{'const':data['source']['expSha256']},'physical':{'type':'string'}}},'functions':{'type':'array','minItems':41,'items':{'type':'object','additionalProperties':False,'required':['id','owner','args','expected','tolerance'],'properties':{'id':{'type':'string','minLength':1},'owner':{'enum':sorted(set(row['owner'] for row in rows))},'args':{'type':'array','minItems':1,'maxItems':2,'items':{'oneOf':[{'type':'string','pattern':'^[1-9][0-9]*$'},{'type':'null'}]}},'expected':result,'tolerance':{'type':'number','minimum':0,'maximum':1e-15}}}}}}
(root/'cases.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(owners=len(set(row['owner'] for row in rows)),cases=len(rows))))
