const fs=require('node:fs');
const ticket='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/';
const root='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/';
const changes=[
 ['🦀️.rs','use std::mem::ManuallyDrop;','use std::mem::ManuallyDrop;\n\n#[path="🌉️call/📥️input/🦀️.rs"]\npub mod original_input;'],
 ['📦️packages/🦀️rust/Cargo.toml','semio-framework-job = { workspace = true }\n','semio-framework-job = { workspace = true }\nsemio-framework-job-native-decode = { path = "../../../../../../../../🔨️modules/🧵️job/⏱️context/🛬️decode/📦️packages/🦀️rust" }\nsemio-framework-pack-json = { path = "../../../../../../../../🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust" }\nsemio-framework-pack-json-output-receiving = { path = "../../../../../../../../🔨️modules/🎒️pack/🔤️json/📥️decode/🧳️receiving/🫴️unique/📦️packages/🦀️rust" }\n'],
 ['📦️packages/🦀️rust/Cargo.toml','[dev-dependencies]\n','[dev-dependencies]\nsemio-framework-job-testing-caller = { workspace = true }\nsemio-framework-value-testing-allocator = { path = "../../../../../../../../🔨️modules/🌱️value/🧪️testing/💾️allocator/📦️packages/🦀️rust" }\nserde_json = { workspace = true }\n']
];
for(let i=0;i<changes.length;i++){
 const [file,old,replacement]=changes[i],path=root+file,before=fs.readFileSync(path,'utf8');
 if(before.split(old).length!==2)throw Error(`expected one marker in ${path}`);
 const temporary=ticket+'🗑️generated/mount-actor-input-'+i+'.tmp';fs.writeFileSync(temporary,before.replace(old,replacement));
 if(fs.readFileSync(path,'utf8')!==before)throw Error(`shared source changed: ${path}`);
 fs.renameSync(temporary,path);
}
