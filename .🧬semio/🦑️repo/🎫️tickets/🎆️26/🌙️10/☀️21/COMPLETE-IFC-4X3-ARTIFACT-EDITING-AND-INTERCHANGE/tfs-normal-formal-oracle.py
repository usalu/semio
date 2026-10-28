import pathlib,json,ifcopenshell,ifcopenshell.validate
p=pathlib.Path(__file__).resolve().parent;root=p.parents[6]
asset=root/'✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧪️fixtures/official/triangulated-item.ifc'
rows=[]
for name,normal in [('zero',(0.,0.,0.)),('nonunit',(2.,0.,0.))]:
 f=ifcopenshell.open(str(asset));e=f.by_id(1021);e.Normals=[normal]*8;log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True)
 rows.append({'id':name,'normal':normal,'diagnostics':[{'attribute':x.get('attribute'),'message':x.get('message'),'instance':str(x.get('instance'))}for x in log.statements]})
(p/'🗑️generated/tfs-admission/normal-formal.json').write_text(json.dumps({'version':ifcopenshell.version,'rows':rows},indent=2),encoding='utf8');print(json.dumps(rows))
