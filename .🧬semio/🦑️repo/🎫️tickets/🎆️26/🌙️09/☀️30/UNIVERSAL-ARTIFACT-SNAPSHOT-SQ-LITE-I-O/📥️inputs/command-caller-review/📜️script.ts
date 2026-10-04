/** 🧷️ Guarded authored caller-site edits preserve original law bodies and concurrent provider regions. */
import {readFileSync,writeFileSync} from "node:fs";
if(process.argv[2]==="parse"){
 const ticket="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O";const list=JSON.parse(readFileSync(ticket+"/📥️inputs/command-caller-review/🔣️parse-inputs.json","utf8"))as string[];const receipts=[];
 for(const file of list){const run=Bun.spawn(["rustfmt","--edition","2021","--emit","stdout","--config","skip_children=true"],{stdin:"pipe",stdout:"ignore",stderr:"pipe"});run.stdin.write(readFileSync(file));run.stdin.end();const stderr=await new Response(run.stderr).text();const exit=await run.exited;receipts.push({file,exit,stderr});}
 writeFileSync(ticket+"/🗑️generated/physical-caller-and-assets-parser.json",JSON.stringify(receipts,null,2));console.log("[DEBUG] changed authored Rust leaves parsed="+receipts.length+" failures="+receipts.filter(row=>row.exit!==0).length);process.exit(receipts.some(row=>row.exit!==0)?1:0);
}
const root="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules";
function append(source:string,name:string,addition:string):string{
 let cursor=0;while(true){const start=source.indexOf(name+"(",cursor);if(start<0)return source;let depth=1,index=start+name.length+1,quoted=false,escape=false;for(;index<source.length&&depth;index++){const char=source[index];if(quoted){if(escape)escape=false;else if(char==="\\")escape=true;else if(char==='"')quoted=false;}else if(char==='"')quoted=true;else if(char==='(')depth++;else if(char===')')depth--;}
 if(depth)throw Error("unclosed "+name);const at=index-1;source=source.slice(0,at)+", "+addition+source.slice(at);cursor=index+addition.length+2;
 }
}
const testContext='fn test_command_context()->crate::os_pack::PackTransportContext{crate::os_pack::control::admit_command_transport(1048576,semio_framework_async::CancelToken::root_now(),|_|{}).unwrap()}\n';
for(const kind of ["🎒️pack","📡️spr"]){const file=root+"/"+kind+"/⌨️cli/🧪️tests/🔬️unit/🦀️.rs";let source=readFileSync(file,"utf8");if(source.includes("fn test_command_context"))throw Error("already authored "+file);source=testContext+append(source,"main_impl","&test_command_context()");writeFileSync(file,source);}
const file=root+"/📡️spr/🔌️io/🧪️tests/🔬️native-unit/🦀️.rs";let source=readFileSync(file,"utf8");if(source.includes("fn test_command_context"))throw Error("already authored IO");source=source.replace("    use super::*;","    use super::*;\n    "+testContext);
for(const name of ["HistoryFile::create","HistoryFile::open_append","HistoryFile::open_read_only","write_sidecar","recover_file","compact"])source=append(source,name,"&test_command_context()");
for(const name of ["read_sidecar","TailFollower::open"])source=append(source,name,"&ProtocolLimits::default(), &test_command_context()");
source=source.replace('assert!(matches!(result.await, Err(ProtocolError::Io(_))));','let ProtocolError::Pack(crate::os_pack::PackError::TransportFailure(error))=result.await.unwrap_err()else{panic!("missing sidecar retains genuine native failure")};assert!(std::error::Error::source(&error).unwrap().downcast_ref::<std::io::Error>().is_some());');
writeFileSync(file,source);
