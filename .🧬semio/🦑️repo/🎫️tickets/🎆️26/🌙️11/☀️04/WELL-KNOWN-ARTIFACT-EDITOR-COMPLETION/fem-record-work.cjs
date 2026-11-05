const fs=require("fs"),cp=require("child_process");function close(s,start,left,right){let depth=0,str=false,esc=false;for(let i=start;i<s.length;i++){let c=s[i];if(str){if(esc)esc=false;else if(c==="\\")esc=true;else if(c==='"')str=false;continue}if(c==='"'){str=true;continue}if(c===left)depth++;if(c===right&&!--depth)return i;}throw Error("unbalanced "+s.slice(start,start+60));}
function edits(s,list){for(const e of list.sort((a,b)=>b.at-a.at))s=s.slice(0,e.at)+e.text+s.slice(e.at);return s;}
function appendArgs(s,names){const out=[];const re=new RegExp("\\b("+names.join("|")+")\\(","g");for(const m of s.matchAll(re)){const open=m.index+m[0].length-1,end=close(s,open,"(",")");out.push({at:end,text:s.slice(open+1,end).trimEnd().endsWith(",")?"meter":",meter"});}return edits(s,out);}
function wrappedCalls(s,re){const out=[];for(const m of s.matchAll(re)){const open=m.index+m[0].lastIndexOf("("),end=close(s,open,"(",")");out.push({at:m.index,text:"meter.take("},{at:end+1,text:")"});}return edits(s,out);}
function richScopes(s){const out=[];for(const m of s.matchAll(/\b(?:owner|original|o)\.(receive_nested|receive)::< /g))throw Error("unexpected generic spacing");const re=/\b(?:owner|original|o)\.(receive_nested|receive)::</g;for(const m of s.matchAll(re)){const gs=m.index+m[0].length-1,ge=close(s,gs,"<",">"),open=s.indexOf("(",ge),end=close(s,open,"(",")"),pipe=s.indexOf("|",open),pipeEnd=s.indexOf("|",pipe+1),body=s.indexOf("{",pipeEnd),bodyEnd=close(s,body,"{","}");if(bodyEnd>end)throw Error("scope bounds");if(m[1]==="receive")out.push({at:ge,text:",NativeSnapshotFailure"});out.push({at:m.index,text:"meter.take("},{at:end+1,text:")"},{at:body+1,text:"receiving_turn(|meter|{"},{at:bodyEnd,text:"})"});}return edits(s,out);}
function errors(s){const out=[];for(const m of s.matchAll(/\bErr\(/g)){const open=m.index+3,end=close(s,open,"(",")"),value=s.slice(open+1,end);if(value.startsWith("invalid(")||value.startsWith("ValueError::literal("))out.push({at:end,text:".into()"});}return edits(s,out);}
const names=["declared","text","number","row","rows","lists","variants","node","material","section","support","member","load","load_case","point","region","term","combination","fill","receive"];
for(const [dim,ty]of [["◻️2d","Fem2d"],["🧊️3d","Fem3d"]]){
 const p="✏️s/🔌️plugins/🏗️fem/🗿️artifacts/"+dim+"/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/receive-record.rs",original=fs.readFileSync(p,"utf8");let s=original;
 const functions=[];
 for(const m of s.matchAll(/\bfn\s+([a-zA-Z_][a-zA-Z0-9_]*)/g)){const open=s.indexOf("(",m.index+m[0].length),ae=close(s,open,"(",")"),body=s.indexOf("{",ae),end=close(s,body,"{","}");const args=s.slice(open+1,ae);if(!args.includes("NativeSnapshotDecodeOwner"))continue;functions.push({name:m[1],start:m.index,open,ae,body,end});}
 for(const f of functions.sort((a,b)=>b.start-a.start)){
  let args=s.slice(f.open+1,f.ae),ret=s.slice(f.ae+1,f.body),body=s.slice(f.body+1,f.end);
  args=args.replaceAll("&mut NativeSnapshotDecodeOwner<'_, '_>) -> Result","&mut NativeSnapshotDecodeOwner<'_, '_>,&mut ReceivingWork) -> Result").replaceAll(", ValueError>",", NativeSnapshotFailure>");
  if(f.name!=="receive_exchange_record")args=args.trimEnd()+(args.trimEnd().endsWith(",")?"":",")+"meter:&mut ReceivingWork";
  ret=ret.replaceAll(", ValueError>",", NativeSnapshotFailure>");
  if(f.name==="receive_exchange_record")ret=ret.replace("Result<Self, NativeSnapshotFailure>","RetainedWorkOutcome<Self, NativeSnapshotFailure>");
  body=body.replaceAll("|v, r, o|","|v, r, o,meter|").replaceAll("|kind, r, original|","|kind, r, original,meter|").replaceAll("|value, original|","|value, original,meter|").replaceAll("|hole, o|","|hole, o,meter|");
  body=appendArgs(body,names);
  body=wrappedCalls(body,/\b(?:String|f64|bool|usize)::receive_value\(/g);
  body=wrappedCalls(body,/\bSelf::receive_authoring\(/g);
  body=richScopes(body);
  const selected=[];for(const m of body.matchAll(/\bwork\(/g)){const open=m.index+m[0].length-1,end=close(body,open,"(",")");if(body.slice(end+1,end+3)!=="?;")throw Error("work action shape");selected.push({at:end+3,text:"meter.performed();"});}body=edits(body,selected);
  body=body.replaceAll("body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;","body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:values.len().checked_mul(size_of::<T>()).ok_or_else(invalid)?,maximum_release_bytes:0,maximum_depth:1})?;native.checkpoint()?;meter.performed();body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 1)?;");
  body=body.replaceAll("body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 2)?;","body.admit_frontier(semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:values.len().checked_mul(size_of::<T>()).ok_or_else(invalid)?,maximum_release_bytes:0,maximum_depth:2})?;native.checkpoint()?;meter.performed();body.allocate_vec_into(native, values.len(), slot.as_mut().unwrap(), 2)?;");
  body=errors(body);
  if(f.name==="receive_exchange_record")body="receiving_turn(|meter|{"+body+"})";
  s=s.slice(0,f.open+1)+args+")"+ret+"{"+body+"}"+s.slice(f.end+1);
 }
 s=s.replace("receiving::{DslReceivingField, NativeSnapshotDecodeOwner, work}","receiving::{DslReceivingField,NativeSnapshotDecodeOwner,NativeSnapshotFailure,ReceivingWork,receiving_turn,work}").replace("use semio_framework_value::{retirement::RetireOwned, ValueError, ValueRefusalKind};","use semio_framework_value::{retirement::RetireOwned,RetainedWorkOutcome,ValueError,ValueRefusalKind};");
 const r=cp.spawnSync("rustc",["--edition=2024","-Zunpretty=ast-tree","-"],{input:s,env:{...process.env,RUSTC_BOOTSTRAP:"1"},stdio:["pipe","ignore","pipe"]});if(r.status!==0)throw Error(dim+" syntax: "+r.stderr.toString().slice(0,3000));if(fs.readFileSync(p,"utf8")!==original)throw Error("concurrent source");fs.writeFileSync(p,s);console.log(dim+" "+functions.length+" receiving functions ported, AST green");
}
