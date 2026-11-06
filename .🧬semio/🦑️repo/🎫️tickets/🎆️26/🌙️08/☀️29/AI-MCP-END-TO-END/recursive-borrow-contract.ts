import{readFileSync,writeFileSync,renameSync}from"node:fs";
const ticket=import.meta.dir,tests="🧰️framework/🔨️modules/🎒️pack/🔤️json/👁️borrow/🧪️tests";
function edit(path:string,change:(s:string)=>string){const old=readFileSync(path,"utf8"),next=change(old);if(next===old)throw Error("unchanged "+path);if(readFileSync(path,"utf8")!==old)throw Error("concurrent write "+path);const temp=ticket+"/🗑️generated/recursive-borrow-"+Date.now()+".tmp";writeFileSync(temp,next);renameSync(temp,path);}
edit(tests+"/🔣️.json",s=>{const f=JSON.parse(s);f.cases.push({kind:"recursive",input:{id:1,children:[{id:2,children:[]}]},expected:'{"id":1,"children":[{"id":2,"children":[]}]}'},{kind:"internalNewtype",input:{kind:"tree",id:1,children:[{id:2,children:[]}]},expected:'{"kind":"tree","id":1,"children":[{"id":2,"children":[]}]}'},{kind:"internalLinked",input:{kind:"linked",first:9,second:8,third:7},expected:'{"kind":"linked","first":9,"second":8,"third":7}'});return JSON.stringify(f)});
edit(tests+"/🧬️.json",s=>{const f=JSON.parse(s);f.properties.cases.items.properties.kind.enum.push("recursive","internalNewtype","internalLinked");return JSON.stringify(f)});
edit(tests+"/🦀️.rs",s=>s+`
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
struct RecursiveTree{id:u64,children:Vec<RecursiveTree>}
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
#[serde(tag="kind",rename_all="camelCase")]
#[value(tag="kind",rename_all="camelCase")]
enum InternalNewtype{Tree(RecursiveTree),Linked(Handle),Scalar(u64)}
impl serde::Serialize for Handle{
 fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeMap;let mut map=serializer.serialize_map(Some(3))?;map.serialize_entry("first",&9u64)?;map.serialize_entry("second",&8u64)?;map.serialize_entry("third",&7u64)?;map.end()}
}
#[test]
fn recursive_records_and_internally_tagged_newtypes_borrow_the_actual_graph(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let expected=|kind:&str|fixture["cases"].as_array().unwrap().iter().find(|v|v["kind"]==kind).unwrap()["expected"].as_str().unwrap();
 let tree=||RecursiveTree{id:1,children:vec![RecursiveTree{id:2,children:vec![]}]};case(&tree(),expected("recursive"));case(&InternalNewtype::Tree(tree()),expected("internalNewtype"));
 let linked=Linked{edges:std::cell::Cell::new(0)};let root=InternalNewtype::Linked(Handle(JsonBorrowKey([8,1,2,0])));assert_eq!(write(&JsonNativeSource{root:&root,context:&linked}),expected("internalLinked"));assert_eq!(linked.edges.get(),3);assert_eq!(serde_json::to_string(&root).unwrap(),expected("internalLinked"));
 assert!(JsonNativeBorrow::json_node(&InternalNewtype::Scalar(7),0,&JsonBorrowNoContext).is_err());
}
`);
console.log("Authored recursive and internally tagged native graph laws");
