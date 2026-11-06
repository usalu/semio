const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/🦀️.rs';
const before=fs.readFileSync(file,'utf8');let after=before;
for(const pattern of [
 /    #\[derive\(Deserialize, FromValue, Serialize, ToValue\)\]\n    pub struct StartJobInput \{[^]*?    \}\n\n/,
 /    unsafe fn take_actor_json<T: serde::de::DeserializeOwned>\([^]*?\n    \}\n\n/,
 /    \/\/\/ 📥️ Retains the original job source and budget without an error-text carrier\.\n    pub unsafe fn take_actor_start_job_input\([^]*?\n\n/
]){if([...after.matchAll(new RegExp(pattern.source,'g'))].length!==1)throw Error('actor adoption marker changed');after=after.replace(pattern,'')}
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/remove-actor-json-adoption.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared schema changed');fs.renameSync(temp,file);
