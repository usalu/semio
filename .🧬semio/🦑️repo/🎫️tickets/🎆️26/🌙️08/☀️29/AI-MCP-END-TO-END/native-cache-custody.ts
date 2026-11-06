import{readFileSync,writeFileSync,renameSync}from"node:fs";
const p="🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🧊️compiled-components/🦀️.rs",before=readFileSync(p,"utf8");let s=before;
s=s.replace("package_hash,resolve:resolve_native_instance_origin", "package_hash,acquire:acquire_native_instance_origin,resolve:resolve_native_instance_origin,release:release_native_instance_origin");
s=s.replace("if compiled.package_hash!=hash", "if compiled.compiled.package_hash!=hash||compiled.homes==0").replace("run(&runtime.runtime,compiled)?", "run(&runtime.runtime,&compiled.compiled)?");
s=s.replaceAll("Mutex<HashMap<[u8; 32], semio_plugin_host::CompiledHandle>>", "Mutex<HashMap<[u8; 32], CompiledParent>>");
s=s.replace("return Ok(compiled.clone());", "return Ok(compiled.compiled.clone());");
s=s.replace("compiled_components().lock().unwrap_or_else(std::sync::PoisonError::into_inner).insert(package_hash, compiled.clone());\n    Ok(compiled)","let mut cache=compiled_components().lock().unwrap_or_else(std::sync::PoisonError::into_inner);\n    Ok(cache.entry(package_hash).or_insert(CompiledParent{compiled,homes:0}).compiled.clone())");
const at=s.indexOf("/// 🧩️ ONE compiled");s=s.slice(0,at)+`#[cfg(not(target_arch="wasm32"))]
struct CompiledParent{compiled:semio_plugin_host::CompiledHandle,homes:u64}
#[cfg(not(target_arch="wasm32"))]
fn acquire_native_instance_origin(identity:u64,hash:[u8;32],held:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{change_native_instance_custody(identity,hash,held,true,cx)}
#[cfg(not(target_arch="wasm32"))]
fn release_native_instance_origin(identity:u64,hash:[u8;32],held:&mut bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{change_native_instance_custody(identity,hash,held,false,cx)}
#[cfg(not(target_arch="wasm32"))]
fn change_native_instance_custody(identity:u64,hash:[u8;32],held:&mut bool,acquire:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
 let refuse=||ValueError::literal(ValueRefusalKind::InvariantViolated,"native parent custody identity or balance differs");
 let runtime=plugin_runtime_origin().get().ok_or_else(refuse)?;if identity==0||identity!=runtime.identity||*held==acquire{return Err(refuse().into())}
 let Ok(mut cache)=compiled_components().try_lock()else{return Ok(())};let parent=cache.get_mut(&hash).ok_or_else(refuse)?;
 if parent.compiled.package_hash!=hash{return Err(refuse().into())}let next=if acquire{parent.homes.checked_add(1)}else{parent.homes.checked_sub(1)}.ok_or_else(refuse)?;
 let grant=cx.retained_grant();let bytes=std::mem::size_of::<u64>()+std::mem::size_of::<bool>();
 if(if acquire{cx.should_yield()}else{cx.retirement_should_yield()})||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}
 cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);parent.homes=next;*held=acquire;Ok(())
}

`+s.slice(at);
if(s===before)throw Error("No custody edit");const temp=import.meta.dir+"/🗑️generated/cache-custody-"+process.pid;writeFileSync(temp,s);if(readFileSync(p,"utf8")!==before)throw Error("Concurrent update");renameSync(temp,p);console.log("Installed exact-parent funded custody counts");
