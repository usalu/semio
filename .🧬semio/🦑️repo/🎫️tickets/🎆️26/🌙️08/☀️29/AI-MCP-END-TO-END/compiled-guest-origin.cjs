const fs=require('node:fs');
const path='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧠️interpreter/🏃️runtime/🧮️owned-constants-and-codec-origin/🦀️.rs';
const before=fs.readFileSync(path,'utf8');const marker='impl OwnedCompiledGuest {\n';
if(before.split(marker).length!==2)throw Error('compiled guest owner changed');
const after=before.replace(marker,marker+'    /// 🪪️ Both handles must borrow the same actual artifact and codec-origin owner.\n    pub fn same_origin(&self,other:&Self)->bool{Arc::ptr_eq(&self.artifact,&other.artifact)&&Arc::ptr_eq(&self.codec_origin,&other.codec_origin)}\n');
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/compiled-guest-origin.tmp';
fs.writeFileSync(temp,after);if(fs.readFileSync(path,'utf8')!==before)throw Error('shared source changed');fs.renameSync(temp,path);
