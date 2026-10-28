import pathlib,json
p=pathlib.Path(__file__).resolve().parent
c=json.loads((p/'🗑️generated/compliance-infrastructure-census.json').read_text())
rows=[]
for entry in c['executableDefinitions']:
 d=entry['definition'];concept=d.get('id',d.get('concept'));predicate=d.get('predicate','body-three-clauses')
 rows.append({'concept':concept,'predicate':predicate,'definitionPath':entry['path'],'definitionSha256':entry['sha256'],'source':d['source'],'implementedScope':'three-selected-body-clauses' if predicate=='body-three-clauses' else predicate,'viewRequirementBinding':'unresolved'})
obj={'schema':'ifc4x3-executable-requirement-inventory','release':'IFC4X3_ADD2','completeViewRequirementSet':False,'entries':rows}
(p/'closed-view-executor-requirements.json').write_text(json.dumps(obj,indent=2)+'\n',encoding='utf8')
report={'$schema':'https://json-schema.org/draft/2020-12/schema','type':'object','additionalProperties':False,'required':['schema','scope','view','requirementSetComplete','status','complete','valid','obligations','diagnostics'],'properties':{'schema':{'const':'IFC4X3_ADD2'},'scope':{'enum':['selected-contracts','whole-view']},'view':{'enum':['ReferenceView','Alignment-basedReferenceView',None]},'requirementSetComplete':{'type':'boolean'},'status':{'enum':['pass','fail','incomplete','not-applicable']},'complete':{'type':'boolean'},'valid':{'type':'boolean'},'obligations':{'type':'array','items':{'type':'object','additionalProperties':False,'required':['concept','predicate','target','applicability','implementation','status'],'properties':{'concept':{'type':'string','minLength':1},'predicate':{'type':'string','minLength':1},'target':{'type':'string','pattern':'^[1-9][0-9]*$'},'applicability':{'enum':['applicable','not-applicable','unknown']},'implementation':{'enum':['implemented','unimplemented']},'status':{'enum':['pass','fail','incomplete','not-applicable']}}}},'diagnostics':{'type':'array','items':{'type':'string','minLength':1}}},'allOf':[{'if':{'properties':{'complete':{'const':False}}},'then':{'properties':{'valid':{'const':False}}}},{'if':{'properties':{'scope':{'const':'whole-view'},'requirementSetComplete':{'const':False}}},'then':{'properties':{'complete':{'const':False},'valid':{'const':False}},'not':{'properties':{'status':{'enum':['pass','not-applicable']}}}}},{'if':{'properties':{'valid':{'const':True}}},'then':{'properties':{'status':{'const':'pass'},'complete':{'const':True}}}}]}
(p/'closed-view-executor-report.schema.json').write_text(json.dumps(report,indent=2)+'\n',encoding='utf8')
print(len(rows))

