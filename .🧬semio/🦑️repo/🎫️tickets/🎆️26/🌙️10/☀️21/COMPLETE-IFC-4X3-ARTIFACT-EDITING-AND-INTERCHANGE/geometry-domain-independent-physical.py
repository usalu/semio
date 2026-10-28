import pathlib,json,hashlib
import ifcopenshell,ifcopenshell.validate
p=pathlib.Path(__file__).resolve().parent;result={'library':ifcopenshell.version,'raw':[],'admitted':[]}
for group,sub in [('raw','geometry-domain-actual'),('admitted','csg-round-admitted-actual')]:
 folder=p/'🗑️generated'/sub;data=json.loads((folder/'producer.json').read_text())
 for row in data['rows']:
  path=folder/row['step'];raw=path.read_bytes();model=ifcopenshell.open(str(path));logger=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(model,logger,express_rules=True);expected=row.get('sha256',row.get('stepSha256'));item={'id':row['id'],'sha256':hashlib.sha256(raw).hexdigest(),'hashMatch':hashlib.sha256(raw).hexdigest()==expected,'schema':list(model.header.file_schema.schema_identifiers),'expected':row.get('expected'),'diagnostics':[{'attribute':x.get('attribute'),'instance':str(x.get('instance')),'message':x.get('message')}for x in logger.statements]};result[group].append(item);print('[DEBUG] '+group+'/'+row['id']+' '+str(len(logger.statements)),flush=True)
(p/'🗑️generated/geometry-domain-independent-physical.json').write_text(json.dumps(result,indent=2),encoding='utf8');assert all(x['hashMatch']for key in ['raw','admitted']for x in result[key]);assert all(not x['diagnostics']for x in result['admitted']);assert sum(bool(x['diagnostics'])for x in result['raw'])==10
