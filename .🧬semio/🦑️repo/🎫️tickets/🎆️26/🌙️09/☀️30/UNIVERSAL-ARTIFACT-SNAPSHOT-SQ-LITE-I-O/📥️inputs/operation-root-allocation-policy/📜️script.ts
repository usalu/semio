import {resolve} from "node:path";
const ticket=resolve(import.meta.dir,"../..");const root=resolve(ticket,"../../../../../../..");const input=import.meta.dir;const command=process.argv[2];
if(command!=="stage-provider"&&command!=="provider")throw Error("exact stage-provider or provider required");
const pairs:{path:string,before:string,after:string}[]=[];
function replace(source:string,before:string,after:string){if(source.split(before).length!==2)throw Error(`exact unique source boundary ${before.slice(0,100)}`);return source.replace(before,after);}
async function pair(path:string,change:(source:string)=>string){const before=await Bun.file(`${root}/${path}`).text();pairs.push({path,before,after:change(before)});}
const helper=`/// 🧾 Applies the caller's cumulative allocation ceiling before any original operation work, restoring its control on every result.
pub fn with_operation_encode_policy<T,E:From<crate::PackRefusal>>(options:&crate::codec::PackEncodeOptions,control:&mut crate::value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut crate::value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
    let maximum=options.limits.max_total_alloc.min(usize::MAX as u64)as usize;
    control.scoped_maximum(maximum,|control|Ok::<_,crate::PackRefusal>(operation(control))).map_err(E::from)?
}

`;
await pair("🧰️framework/🔨️modules/📡️replication/🎮️mutation/📦️bytes/🦀️.rs",source=>replace(source,"#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct OperationByteFault",helper+"#[derive(Clone, Copy, Debug, PartialEq, Eq)]\npub struct OperationByteFault"));
await pair("✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🩹️patch/📦️codec/🫳️borrowed/🦀️.rs",source=>{
 const start="->Result<(),super::kernel::ProtocolError>{\n";const end="        Ok(())\n    }\n}\nimpl JsonWriteSource";
 return replace(replace(source,start,start+"        super::kernel::operation_bytes::with_operation_encode_policy(options,control,|control|{\n"),end,"        Ok(())\n        })\n    }\n}\nimpl JsonWriteSource");
});
await pair("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/🧬️mutations/📦️codec/🫳️borrowed/🦀️.rs",source=>{
 const start="->Result<(),protocol::ProtocolError>{\n";const end="        Ok(())\n    }\n}\n";
 return replace(replace(source,start,start+"        protocol::operation_bytes::with_operation_encode_policy(options,control,|control|{\n"),end,"        Ok(())\n        })\n    }\n}\n");
});
await pair("🧰️framework/🛍️products/📓️print/🧬️schema/🧬️mutations/📦️codec/🦀️.rs",source=>{
 const start="->Result<(),protocol::ProtocolError>{\n";const end="  Ok(())\n }\n /// 🫳️";
 return replace(replace(source,start,start+"  protocol::operation_bytes::with_operation_encode_policy(options,control,|control|{\n"),end,"  Ok(())\n  })\n }\n /// 🫳️");
});
await pair("🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️.rs",source=>{
 const start="fn encode_with_into<T:DslVariants>(op:&T,tag_of:impl Fn(&str,usize)->Result<u64,ProtocolError>,options:&EncodeOptions,output:&mut dyn crate::os_spr::operation_bytes::OperationByteOutput,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<(),ProtocolError>{\n";
 const end="        Ok(())\n    }\n\n    /// 🏷️ Appends a declared tagged operation";
 return replace(replace(source,start,start+"        crate::os_spr::operation_bytes::with_operation_encode_policy(options,control,|control|{\n"),end,"        Ok(())\n        })\n    }\n\n    /// 🏷️ Appends a declared tagged operation");
});
await Bun.write(`${input}/${command}-guarded-pairs.json`,JSON.stringify({state:command==="stage-provider"?"Held":"ExactGuardedOriginalPolicyFamily",pairs},null,2)+"\n");
if(command==="provider"){for(const pair of pairs){if(await Bun.file(`${root}/${pair.path}`).text()!==pair.before)throw Error(`fresh guard ${pair.path}`);}for(const pair of pairs)await Bun.write(`${root}/${pair.path}`,pair.after);}
console.log("[DEBUG] "+JSON.stringify({command,paths:pairs.length,productionWrites:command==="provider"?pairs.length:0}));
