import pathlib,json,ifcopenshell
p=pathlib.Path(__file__).resolve().parent;folder=p/'🗑️generated/geometry-domain-actual';data=json.loads((folder/'producer.json').read_text());rows=[]
for row in data['rows']:
 if row['target']not in ['point','axis','reference']:continue
 f=ifcopenshell.open(str(folder/row['step']));primitive=f.by_type('IfcCsgPrimitive3D')[0];obj=primitive.Position.Location if row['target']=='point'else primitive.Position.Axis if row['target']=='axis'else primitive.Position.RefDirection;values=obj[0];rows.append({'id':row['id'],'rawEntity':str(obj),'sdkValues':list(values),'sdkKinds':[type(v).__name__ for v in values]})
(p/'🗑️generated/geometry-domain-independent-sdk-aggregates.json').write_text(json.dumps({'library':ifcopenshell.version,'rows':rows},indent=2),encoding='utf8');print('[DEBUG] '+str(len(rows))+' aggregate observations')

