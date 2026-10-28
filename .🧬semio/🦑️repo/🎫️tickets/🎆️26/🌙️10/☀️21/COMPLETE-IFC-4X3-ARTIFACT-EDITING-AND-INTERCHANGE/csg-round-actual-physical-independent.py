import pathlib,json,hashlib
import ifcopenshell,ifcopenshell.validate
p=pathlib.Path(__file__).resolve().parent;folder=p/'🗑️generated/csg-round-actual';data=json.loads((folder/'producer.json').read_text());rows=[]
for row in data['rows']:
 path=folder/row['step'];raw=path.read_bytes();f=ifcopenshell.open(str(path));log=ifcopenshell.validate.json_logger();ifcopenshell.validate.validate(f,log,express_rules=True)
 item={'id':row['id'],'sha256':hashlib.sha256(raw).hexdigest(),'shaMatches':hashlib.sha256(raw).hexdigest()==row['stepSha256'],'schema':list(f.header.file_schema.schema_identifiers),'entities':len(list(f)),'diagnostics':[{'attribute':x.get('attribute'),'instance':str(x.get('instance')),'message':x.get('message')}for x in log.statements]};rows.append(item);print('[DEBUG] '+row['id']+' '+str(len(log.statements)),flush=True)
(p/'🗑️generated/csg-round-independent/actual-physical.json').write_text(json.dumps({'library':ifcopenshell.version,'rows':rows},indent=2),encoding='utf8')
assert all(x['shaMatches']and x['schema']==['IFC4X3_ADD2']and not x['diagnostics']for x in rows)
