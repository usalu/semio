import {readFileSync,writeFileSync,renameSync} from "node:fs";
const file="🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🎭️guest-runtime/🗂️wasmtime-guest-runtime-and-codec-dispatch/🦀️.rs";
let source=readFileSync(file,"utf8");
for(const name of ["admit","close"]){
 const before=`call_${name}_actor_context(accessor,semio_framework_plugin_host_wit::semio::framework::reactor::ActorContextBudget`;
 const after=`call_${name}_actor_context(accessor,semio_framework_plugin_host_wit::wit_reactor::ActorContextBudget`;
 if(source.split(before).length!==2)throw Error(`expected one ${name} crossing`);
 source=source.replace(before,after);
}
const temp=import.meta.dirname+"/🗑️generated/guest-wire-budget-edit.rs";
writeFileSync(temp,source);
renameSync(temp,file);
