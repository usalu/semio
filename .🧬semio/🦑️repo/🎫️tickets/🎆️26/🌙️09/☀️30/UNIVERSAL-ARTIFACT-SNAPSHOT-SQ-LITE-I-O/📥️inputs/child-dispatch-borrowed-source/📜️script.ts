
import{readFileSync,writeFileSync,mkdirSync,existsSync}from"node:fs";import{dirname,join}from"node:path";
const root="/Users/ueli/Documents/semio",input=dirname(import.meta.path),owner="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store",plugin="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs";
type Pair={path:string,before:string,after:string};const read=(p:string)=>existsSync(join(root,p))?readFileSync(join(root,p),"utf8"):"";
function save(name:string,pairs:Pair[],mount:boolean){writeFileSync(join(input,name),JSON.stringify(pairs,null,2)+"\n");if(mount){for(const p of pairs)if(read(p.path)!==p.before)throw Error("fresh wholefile guard "+p.path);for(const p of pairs){mkdirSync(dirname(join(root,p.path)),{recursive:true});writeFileSync(join(root,p.path),p.after);if(read(p.path)!==p.after)throw Error("after guard "+p.path);}}console.log("[DEBUG] "+JSON.stringify({paths:pairs.length,mounted:mount}));}
const command=process.argv[2];
if(command==="demand"){
const path=owner+"/🧪️tests/🔬️unit/🦀️.rs",before=read(path);if(before.includes("dispatch_group_borrowed_child_keeps_exact_sources_on_policy_refusal"))throw Error("already demanded");
const law=readFileSync(join(input,"law.rs"),"utf8");
save("demand-guarded-pairs.json",[{path,before,after:before+"\n"+law},...["fixture.json","schema.json"].map((file)=>{const path=owner+"/🧫️fixtures/🫳️child-dispatch/"+(file==="fixture.json"?"🔣️.json":"🧬️schema.json");return{path,before:read(path),after:readFileSync(join(input,file),"utf8")};})],true);
}else if(command==="stage"){
const path=owner+"/🦀️.rs",before=read(path);
const old="pub struct ChildDispatch {\n    pub child: crate::os_io::ArtifactRef,\n    pub ops: Vec<Vec<u8>>,\n    pub op_schema: SchemaId,\n    pub labels: Vec<crate::LocalizedLabel>,\n}";
const next="pub struct ChildDispatch<'wire> {\n    pub child: crate::os_io::ArtifactRef,\n    pub ops: &'wire [Vec<u8>],\n    pub op_schema: &'wire SchemaId,\n    pub labels: &'wire [crate::LocalizedLabel],\n}\n\nimpl<'wire> ChildDispatch<'wire> {\n    /// 🤲️ Borrows the exact retained child sources for one awaited composite publication.\n    pub fn borrowed(child:crate::os_io::ArtifactRef,ops:&'wire [Vec<u8>],op_schema:&'wire SchemaId,labels:&'wire [crate::LocalizedLabel])->Self {\n        Self{child,ops,op_schema,labels}\n    }\n}";
if(before.split(old).length!==2)throw Error("exact dispatch declaration");
let after=before.replace(old,next).replaceAll("(&mut Mc, ChildDispatch)","(&mut Mc, ChildDispatch<'_>)").replaceAll("member.preview_wire(&dispatch.ops)","member.preview_wire(dispatch.ops)").replaceAll("concat_ops_fingerprint(&dispatch.ops)","concat_ops_fingerprint(dispatch.ops)").replaceAll("build_apply_command_bytes(&children[index].1.ops,","build_apply_command_bytes(children[index].1.ops,");
const pp=plugin,pbefore=read(pp),anchor="ChildDispatch { child: target, ops: child_emit.ops.clone(), op_schema: child_emit.op_schema.clone(), labels: child_emit.labels.clone() }";
if(pbefore.split(anchor).length!==2)throw Error("exact retained caller");
const pafter=pbefore.replace(anchor,"ChildDispatch::borrowed(target, &child_emit.ops, &child_emit.op_schema, &child_emit.labels)").replaceAll("Vec<(*mut M, ChildDispatch)>","Vec<(*mut M, ChildDispatch<'_>)>").replaceAll("Vec<(&mut M, ChildDispatch)>","Vec<(&mut M, ChildDispatch<'_>)>").replace("semio_framework_value::ToValue::to_value(&children.to_vec())","semio_framework_value::ToValue::to_value(children)");
const tp=owner+"/🧪️tests/🔬️unit/🦀️.rs",tbefore=read(tp);
let count=0;
const tafter=tbefore.replace(/ChildDispatch \{[^\n]+\}/g,(row)=>{count++;return row.replace(/ops: vec!\[([^]*?)\], op_schema:/,"ops: &[$1], op_schema:").replace("ops: Vec::new()","ops: &[]").replace("ops: child_ops","ops: &child_ops").replace("op_schema: SchemaId(","op_schema: &SchemaId(").replace("labels: Vec::new()","labels: &[]").replace("labels: vec![","labels: &[");});
if(count!==18)throw Error("current exact authored constructor census "+count);
save("provider-held-pairs.json",[{path,before,after},{path:pp,before:pbefore,after:pafter},{path:tp,before:tbefore,after:tafter}],false);
}else if(command==="mount")save("provider-guarded-pairs.json",JSON.parse(readFileSync(join(input,"provider-held-pairs.json"),"utf8")),true);
else if(command==="current")save("current-production-readback-pairs.json",JSON.parse(readFileSync(join(input,"provider-guarded-pairs.json"),"utf8")).map((p:Pair)=>({...p,after:read(p.path)})),false);
else throw Error("exact demand/stage/mount/current required");
