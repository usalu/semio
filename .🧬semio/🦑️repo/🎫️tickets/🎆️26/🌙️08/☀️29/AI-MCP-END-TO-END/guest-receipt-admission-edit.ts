import {readFileSync,writeFileSync,renameSync} from "node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs";
let source=readFileSync(file,"utf8");
for(const name of ["admit_owned","close_owned","poll_owned","close_turn_output_owned"]){
 const marker=`pub fn ${name}`;const begin=source.indexOf(marker);if(begin<0)throw Error(name);
 const body=source.indexOf("{",begin)+1;
 source=source.slice(0,body)+"if !actor_receipt_available(){return semio_framework_plugin_abi::operation_authority::owned_result::encode_refusal(ValueRefusalKind::InvariantViolated)}"+source.slice(body);
}
const temp=import.meta.dirname+"/🗑️generated/guest-receipt-admission-edit.rs";
writeFileSync(temp,source);
renameSync(temp,file);
