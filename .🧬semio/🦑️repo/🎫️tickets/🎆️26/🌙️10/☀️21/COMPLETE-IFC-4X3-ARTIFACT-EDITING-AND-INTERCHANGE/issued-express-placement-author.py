"""📐 Author source-addressed placement/point owner boundaries without changing semantics."""
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️placement'
root.mkdir(exist_ok=True)
body='''#1=IFCCARTESIANPOINT((0.,0.));
#2=IFCCARTESIANPOINT((0.,0.,0.));
#3=IFCCARTESIANPOINT((0.125,0.));
#4=IFCCARTESIANPOINT((0.25,0.));
#5=IFCCARTESIANPOINT((0.,0.,1.));
#6=IFCCARTESIANPOINT((1.,2.,3.));
#10=IFCDIRECTION((1.,0.));
#11=IFCDIRECTION((2.,0.));
#12=IFCDIRECTION((1.,0.,0.));
#13=IFCDIRECTION((0.,1.));
#14=IFCDIRECTION((0.,0.,1.));
#15=IFCDIRECTION((1.125,0.));
#16=IFCDIRECTION((1.25,0.));
#20=IFCAXIS2PLACEMENT2D(#1,$);
#21=IFCAXIS2PLACEMENT2D(#1,#13);
#22=IFCAXIS2PLACEMENT3D(#2,$,$);
#23=IFCAXIS2PLACEMENT3D(#2,#14,#12);
#24=IFCAXIS2PLACEMENT3D(#5,$,$);
#30=IFCLOCALPLACEMENT($,#20);
#31=IFCLOCALPLACEMENT($,#22);
#32=IFCLOCALPLACEMENT(#31,#23);
#33=IFCLOCALPLACEMENT(#31,#20);
#40=IFCCARTESIANPOINTLIST2D(((0.,0.),(1.,1.)),$);
#41=IFCCARTESIANPOINTLIST3D(((0.,0.,0.),(1.,1.,1.)),$);
#50=IFCVECTOR(#12,1.);
#51=IFCLINE(#2,#50);
#52=IFCPOINTONCURVE(#51,0.);
#53=IFCPLANE(#22);
#54=IFCPOINTONSURFACE(#53,0.,0.);
#55=IFCPOINTBYDISTANCEEXPRESSION(IFCLENGTHMEASURE(1.),$,$,$,#51);
#56=IFCVECTOR(#10,1.);
#57=IFCLINE(#1,#56);
#58=IFCPOINTONCURVE(#57,1.);
'''
source="ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('Authored placement owner cases'),'2;1');\nFILE_NAME('placement','2026-10-22T00:00:00',('semio'),('semio'),'semio','semio','');\nFILE_SCHEMA(('IFC4X3_ADD2'));\nENDSEC;\nDATA;\n"+body+'ENDSEC;\nEND-ISO-10303-21;\n'
(root/'source.ifc').write_text(source,encoding='utf-8')
rows=[]
def arg(id):return dict(kind='unknown') if id is None else dict(kind='entity',id=str(id))
def real(value):return dict(kind='unknown') if value is None else dict(kind='real',value=str(value))
def case(owner,id,ids,expected,epsilon='absent'):
 args=[arg(value) for value in ids]
 if epsilon!='absent':args.append(real(epsilon))
 rows.append(dict(owner=owner,id=id,args=args,expected=expected))
def logical(value):return dict(kind='logical',value=value)
unknown=dict(kind='unknown')
for id,args,value,eps in [('equal',[1,1],'true',.25),('2d-3d-zero',[1,2],'true',.25),('different-z',[1,5],'false',.25),('below-boundary',[1,3],'true',.25),('exact-boundary',[1,4],'false',.25),('reversed-boundary',[4,1],'false',.25),('zero-epsilon',[1,1],'false',0),('negative-epsilon',[1,1],'false',-.25),('default-epsilon',[1,2],'true',None),('unknown-point',[None,2],'unknown',.25)]:case('IFCSAMECARTESIANPOINT',id,args,logical(value),eps)
for id,args,value,eps in [('equal',[10,10],'true',.25),('2d-3d-zero',[10,12],'true',.25),('proportional-raw',[10,11],'false',.25),('orthogonal',[10,13],'false',.25),('below-boundary',[10,15],'true',.25),('exact-boundary',[10,16],'false',.25),('zero-epsilon',[10,10],'false',0),('default-epsilon',[10,12],'true',None),('unknown-direction',[None,12],'unknown',.25)]:case('IFCSAMEDIRECTION',id,args,logical(value),eps)
for id,args,value in [('same2d',[20,20],'true'),('same3d',[22,23],'true'),('2d-3d',[20,22],'true'),('rotated2d',[20,21],'false'),('different-location',[22,24],'false'),('unknown-placement',[None,22],'unknown')]:case('IFCSAMEAXIS2PLACEMENT',id,args,logical(value),.25)
for id,args,expected in [('no-parent2d',[20,None],logical('true')),('no-parent3d',[22,None],logical('true')),('parent2d-child2d',[20,30],logical('true')),('parent3d-child2d',[20,31],logical('true')),('parent3d-child3d',[22,31],logical('true')),('parent2d-child3d',[22,30],logical('false')),('unknown-child',[None,31],unknown)]:case('IFCCORRECTLOCALPLACEMENT',id,args,expected)
for id,entity,dim in [('cartesian2d',1,2),('cartesian3d',2,3),('curve2d',58,2),('curve3d',52,3),('surface3d',54,3),('distance3d',55,3)]:case('IFCPOINTDIM',id,[entity],dict(kind='integer',value=str(dim)))
case('IFCPOINTDIM','absent',[None],unknown)
case('IFCPOINTLISTDIM','2d',[40],dict(kind='integer',value='2'));case('IFCPOINTLISTDIM','3d',[41],dict(kind='integer',value='3'));case('IFCPOINTLISTDIM','absent',[None],unknown)
data=dict(version=1,source=dict(file='source.ifc',schema='IFC4X3_ADD2',expSha256='f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860',physical='authored standalone geometric population; no complete project/view claim'),functions=rows)
(root/'cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
argument={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind','id'],'properties':{'kind':{'const':'entity'},'id':{'type':'string','pattern':'^[1-9][0-9]*$'}}},{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'real'},'value':{'type':'string','pattern':'^-?[0-9]+(?:\\.[0-9]+)?$'}}},{'type':'object','additionalProperties':False,'required':['kind'],'properties':{'kind':{'const':'unknown'}}}]}
expected={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'logical'},'value':{'enum':['true','false','unknown']}}},{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'integer'},'value':{'enum':['2','3']}}},{'type':'object','additionalProperties':False,'required':['kind'],'properties':{'kind':{'const':'unknown'}}}]}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['version','source','functions'],'properties':{'version':{'const':1},'source':{'type':'object','additionalProperties':False,'required':['file','schema','expSha256','physical'],'properties':{'file':{'const':'source.ifc'},'schema':{'const':'IFC4X3_ADD2'},'expSha256':{'const':data['source']['expSha256']},'physical':{'type':'string'}}},'functions':{'type':'array','minItems':42,'items':{'type':'object','additionalProperties':False,'required':['owner','id','args','expected'],'properties':{'id':{'type':'string','minLength':1},'owner':{'enum':sorted(set(row['owner'] for row in rows))},'args':{'type':'array','minItems':1,'maxItems':3,'items':argument},'expected':expected}}}}}
(root/'cases.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(cases=len(rows),owners=len(set(row['owner'] for row in rows)))))
