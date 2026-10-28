import pathlib,json,math,hashlib
p=pathlib.Path(__file__).resolve().parent
for name in ['normal-matrix-cases.json','normal-matrix.schema.json']:
 data=(p/name).read_bytes();(p/'🗑️generated/gltf-normals'/('before-direction-'+name)).write_bytes(data)
file=p/'normal-matrix-cases.json';data=json.loads(file.read_text())
expected={'identity':[1/math.sqrt(14),2/math.sqrt(14),3/math.sqrt(14)],'rotation-z90':[-2/math.sqrt(14),1/math.sqrt(14),3/math.sqrt(14)],'reflection':[-1/math.sqrt(2),0,1/math.sqrt(2)],'nonuniform':[6/math.sqrt(61),4/math.sqrt(61),3/math.sqrt(61)],'shear':[8/math.sqrt(649),12/math.sqrt(649),21/math.sqrt(649)],'extreme-axis':[0,1,1e-300],'extreme-balanced':[1/math.sqrt(3)]*3,'extreme-reflected':[-1/math.sqrt(3),1/math.sqrt(3),1/math.sqrt(3)],'singular':None,'zero-normal':None}
for row in data['cases']:row['expectedDirection']=expected[row['id']]
file.write_text(json.dumps(data,indent=2)+'\n',encoding='utf8')
file=p/'normal-matrix.schema.json';schema=json.loads(file.read_text());row=schema['properties']['cases']['items'];row['required'].append('expectedDirection');row['properties']['expectedDirection']={'anyOf':[{'type':'null'},{'type':'array','minItems':3,'maxItems':3,'items':{'type':'number'}}]};file.write_text(json.dumps(schema,indent=2)+'\n',encoding='utf8')
reference=json.loads((p/'🗑️generated/gltf-normals/matrix-mp.json').read_text())
for row in data['cases']:
 actual=next(x for x in reference['rows']if x['id']==row['id'])
 if row['expectedDirection']is None:assert not actual['available'];continue
 for x,y in zip(row['expectedDirection'],actual['direction']):assert abs(x-y)<=4*math.ulp(y),(row['id'],x,y)
print(json.dumps({'expectedDirections':10,'checkedMpmath':True,'tolerance':'4ulp'}))
