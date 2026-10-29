const {toolJobPluginInstanceSource}=await import(process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📤️publication/🟦️.ts");
const {toolJobProductionSource,toolJobRustBlock}=await import(process.cwd()+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts");
const s=toolJobProductionSource(toolJobPluginInstanceSource(process.cwd()));
for(const name of ["dispatch_typed_command_inner","advance_latest_wins_command_one"]){const a=s.indexOf("fn "+name+"(");const b=toolJobRustBlock(s,s.indexOf("{",a))?.body??"";console.log("[DEBUG] "+name+" "+b.replace(/\s+/g,"").slice(0,20000));}
console.log("[DEBUG] callers "+s.replace(/\s+/g,"").split("start_typed_command_operation(").length);
