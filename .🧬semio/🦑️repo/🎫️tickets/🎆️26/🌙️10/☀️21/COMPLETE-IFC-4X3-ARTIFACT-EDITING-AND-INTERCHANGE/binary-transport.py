from pathlib import Path
import json,copy
A=Path('✏️s/🔌️plugins/🗄️stdio/🗿️artifacts')
I=A/'🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any'
B=A/'🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base'
T=A/'📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base'
def edit(p,pairs):
 s=p.read_text(encoding='utf-8')
 for old,new in pairs:
  assert old in s,(str(p),old)
  s=s.replace(old,new)
 p.write_text(s,encoding='utf-8',newline='\n')
pattern='^(?:0[0-9A-F]*|1[0-7][0-9A-F]*|2[0-3][0-9A-F]*|3[01][0-9A-F]*)$'
count=0
for p in I.rglob('*.json'):
 try:d=json.loads(p.read_text(encoding='utf-8'))
 except:continue
 dirty=False
 def visit(node):
  global dirty,count
  if isinstance(node,dict):
   for k,v in list(node.items()):
    if k=='IfcValue' and isinstance(v,dict) and 'oneOf'in v and not any(x.get('properties',{}).get('kind',{}).get('const')=='binary'for x in v['oneOf']):
     v['oneOf'].insert(5,{'type':'object','additionalProperties':False,'required':['kind','value'],'properties':{'kind':{'const':'binary'},'value':{'type':'string','pattern':pattern}}});dirty=True;count+=1
    visit(v)
  elif isinstance(node,list):
   for x in node:visit(x)
 visit(d)
 if dirty:p.write_text(json.dumps(d,ensure_ascii=False,indent=2)+'\n',encoding='utf-8',newline='\n')
edit(I/'🧬️schema/📸️snapshot/🛰️.proto',[('    TYPED_VALUE = 8;','    TYPED_VALUE = 8;\n    BINARY = 9;'),('  repeated IfcValue typed_args = 8;','  repeated IfcValue typed_args = 8;\n  string binary_value = 9;')])
edit(I/'🧬️schema/📸️snapshot/🔗️.graphql',[('  TYPED_VALUE\n','  TYPED_VALUE\n  BINARY\n'),('  stringValue: String\n','  stringValue: String\n  binaryValue: String\n')])
for root,old,new in [(T,'3437','3458'),(B,'4207','4228')]:
 p=root/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎛️semantic.json'
 d=json.loads(p.read_text(encoding='utf-8'));original=d['schemaBytes'];d['schemaBytes']+=21;p.write_text(json.dumps(d,indent=2)+'\n',encoding='utf-8',newline='\n')
 for p in [root/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts',root/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs']:
  t=p.read_text(encoding='utf-8');p.write_text(t.replace(str(original),str(original+21)),encoding='utf-8',newline='\n')
 print(root.name,original,original+21)
edit(I/'🧬️schema/🧬️mutations/🦀️.rs',[('                    IfcValue::String("hi".into()),','                    IfcValue::String("hi".into()),\n                    IfcValue::Binary("17".into()),')])
edit(I/'🧬️schema/🔺️diff/🦀️.rs',[('IfcDiff { file_name: Some(vec![IfcValue::String("changed.ifc".into())])','IfcDiff { file_name: Some(vec![IfcValue::String("changed.ifc".into()), IfcValue::Binary("23".into())])')])
p=I/'🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs';s=p.read_text(encoding='utf-8');s+='''

#[test]
fn ifc_binary_native_and_sqlite_scalars_preserve_canonical_literals() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("binary-cases.json")).unwrap();
    let values:Vec<IfcValue>=cases["canonical"].as_array().unwrap().iter().map(|value|IfcValue::Binary(value.as_str().unwrap().into())).collect();
    let mut snapshot=IfcSnapshot::default();snapshot.header.file_name=values.clone();snapshot.entities.push(IfcEntity{id:1,name:"BINARY_EXTENSION".into(),args:vec![IfcValue::Aggregate(values.clone()),IfcValue::TypedValue(IfcTypedValue{name:"IFCBINARY".into(),items:values.clone()})],complex:vec![IfcComplexType{name:"BINARY_MEMBER".into(),args:values}]});
    let database=project(&snapshot);let reconstructed=IfcSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).unwrap();assert_eq!(reconstructed,snapshot);
    for content in cases["invalid"].as_array().unwrap(){snapshot.entities[0].args=vec![IfcValue::Binary(content.as_str().unwrap().into())];assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits::default())).is_err());}
}
''';p.write_text(s,encoding='utf-8',newline='\n')
print('Transport definitions updated:',count)
