"""🧮 Author populated derived-aggregate owner branches, including explicit schema negatives."""
import json
from pathlib import Path

ticket=Path(__file__).parent
repo=next(p for p in ticket.parents if (p/'AGENTS.md').exists())
root=repo/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/⚖️validation/🧪️aggregate'
root.mkdir(exist_ok=True)
body=['#1=IFCCARTESIANPOINT((0.,0.,0.));','#2=IFCCARTESIANPOINT((1.,0.,0.));','#3=IFCCARTESIANPOINT((0.,1.,0.));','#4=IFCCARTESIANPOINT((1.,1.,0.));','#5=IFCCARTESIANPOINT((2.,0.,0.));','#6=IFCCARTESIANPOINT((2.,1.,0.));']
rows=[]
def real(n):return str(float(n))
def numbers(values):return '('+','.join(map(real,values))+')'
def logical(n):return dict(kind='logical',value=str(n).lower())
def case(owner,id,entity,expected,attribute=False):rows.append(dict(owner=owner,id=id,args=[dict(kind='attribute',id=str(entity),name='SEGMENTS',declaredBy='IFCINDEXEDPOLYCURVE') if attribute else dict(kind='entity',id=str(entity))],expected=expected,sourceEntity=str(entity),schemaRole='negative' if expected==logical(False) else 'positive'))
for offset,(id,weights) in enumerate([('ones',[1,1]),('fractions',[.125,.25]),('zero-first',[0,1]),('zero-last',[1,0]),('negative-first',[-.125,1]),('negative-last',[1,-.125])]):
 entity=20+offset;body.append(f'#{entity}=IFCRATIONALBSPLINECURVEWITHKNOTS(1,(#1,#2),.POLYLINE_FORM.,.F.,.F.,(2,2),(0.,1.),.UNSPECIFIED.,{numbers(weights)});');case('IFCCURVEWEIGHTSPOSITIVE',id,entity,logical(all(n>0 for n in weights)))
matrices=[('ones',[[1,1],[1,1]]),('fractions',[[.125,.25],[.5,1]])]
for i in range(2):
 for j in range(2):
  values=[[1,1],[1,1]];values[i][j]=0;matrices.append((f'zero-{i}-{j}',values))
matrices += [('negative-first',[[-.125,1],[1,1]]),('negative-last',[[1,1],[1,-.125]]),('non-square',[[1,.125,2],[.5,4,8]]),('non-square-zero-last',[[1,1,1],[1,1,0]]),('non-square-negative-column',[[1,1,-.125],[1,1,1]])]
for offset,(id,weights) in enumerate(matrices):
 entity=40+offset;wide=len(weights[0])==3;points='((#1,#2,#5),(#3,#4,#6))' if wide else '((#1,#2),(#3,#4))';vm='(2,1,2)' if wide else '(2,2)';vk='(0.,0.5,1.)' if wide else '(0.,1.)';values='('+','.join(numbers(row) for row in weights)+')'
 body.append(f'#{entity}=IFCRATIONALBSPLINESURFACEWITHKNOTS(1,1,{points},.PLANE_SURF.,.F.,.F.,.F.,(2,2),{vm},(0.,1.),{vk},.UNSPECIFIED.,{values});');case('IFCSURFACEWEIGHTSPOSITIVE',id,entity,logical(all(n>0 for row in weights for n in row)))
body.append('#70=IFCCARTESIANPOINTLIST2D(((0.,0.),(1.,0.),(0.,1.),(1.,1.)),$);')
for entity,id,segments,expected in [(71,'single','IFCLINEINDEX((1,2))',True),(72,'lines','IFCLINEINDEX((1,2)),IFCLINEINDEX((2,3)),IFCLINEINDEX((3,4))',True),(73,'arc-line','IFCARCINDEX((1,2,3)),IFCLINEINDEX((3,4))',True),(74,'early-break','IFCLINEINDEX((1,2)),IFCLINEINDEX((3,4))',False),(75,'late-break','IFCLINEINDEX((1,2)),IFCARCINDEX((2,3,4)),IFCLINEINDEX((1,2))',False)]:
 body.append(f'#{entity}=IFCINDEXEDPOLYCURVE(#70,({segments}),.F.);');case('IFCCONSECUTIVESEGMENTS',id,entity,logical(expected),True)
next_layer=100
for entity,id,thicknesses in [(90,'single',[.125]),(91,'three',[.125,.25,.5]),(92,'zero',[0]),(93,'including-zero',[.25,0,.5])]:
 layers=[]
 for thickness in thicknesses:
  layers.append('#'+str(next_layer));body.append(f'#{next_layer}=IFCMATERIALLAYER($,{real(thickness)},$,\'Layer\',$,$,$);');next_layer+=1
 body.append(f'#{entity}=IFCMATERIALLAYERSET(({",".join(layers)}),\'{id}\',$);');case('IFCMLSTOTALTHICKNESS',id,entity,dict(kind='real',value=str(sum(thicknesses)).removesuffix('.0')))
step="ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION(('Authored positive and negative aggregate owner cases'),'2;1');\nFILE_NAME('aggregate','2026-10-22T00:00:00',('semio'),('semio'),'semio','semio','');\nFILE_SCHEMA(('IFC4X3_ADD2'));\nENDSEC;\nDATA;\n"+'\n'.join(body)+'\nENDSEC;\nEND-ISO-10303-21;\n'
(root/'source.ifc').write_text(step,encoding='utf-8')
data=dict(version=1,source=dict(file='source.ifc',schema='IFC4X3_ADD2',expSha256='f67c8762b13a099c28082061e6f16b9ef1284ceec34069792afc702725675860',physical='authored mixed positive and explicit negative weights/segment sources; retain all EXPRESS diagnostics'),functions=rows)
(root/'cases.json').write_text(json.dumps(data,indent=2)+'\n',encoding='utf-8')
argument={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind','id'],'properties':{'kind':{'const':'entity'},'id':{'type':'string','pattern':'^[1-9][0-9]*$'}}},{'type':'object','additionalProperties':False,'required':['kind','id','name','declaredBy'],'properties':{'kind':{'const':'attribute'},'id':{'type':'string','pattern':'^[1-9][0-9]*$'},'name':{'const':'SEGMENTS'},'declaredBy':{'const':'IFCINDEXEDPOLYCURVE'}}}]}
expected={'oneOf':[{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'logical'},'value':{'enum':['true','false']}}},{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'real'},'value':{'type':'string','pattern':'^[0-9]+(?:\\.[0-9]+)?$'}}}]}
schema={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['version','source','functions'],'properties':{'version':{'const':1},'source':{'type':'object','additionalProperties':False,'required':['file','schema','expSha256','physical'],'properties':{'file':{'const':'source.ifc'},'schema':{'const':'IFC4X3_ADD2'},'expSha256':{'const':data['source']['expSha256']},'physical':{'type':'string'}}},'functions':{'type':'array','minItems':26,'items':{'type':'object','additionalProperties':False,'required':['id','owner','args','expected','sourceEntity','schemaRole'],'properties':{'id':{'type':'string','minLength':1},'owner':{'enum':sorted(set(row['owner'] for row in rows))},'args':{'type':'array','minItems':1,'maxItems':1,'items':argument},'expected':expected,'sourceEntity':{'type':'string','pattern':'^[1-9][0-9]*$'},'schemaRole':{'enum':['positive','negative']}}}}}}
(root/'cases.schema.json').write_text(json.dumps(schema,indent=2)+'\n',encoding='utf-8')
print('[DEBUG] '+json.dumps(dict(cases=len(rows),owners=len(set(row['owner'] for row in rows)),negative=sum(row['schemaRole']=='negative' for row in rows))))
