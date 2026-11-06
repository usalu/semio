const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/📦️packages/🦀️rust/Cargo.toml';
const before=fs.readFileSync(file,'utf8'),old='semio-framework-job-testing-caller = { workspace = true }';
if(before.split(old).length!==2)throw Error('fixture dependency changed');
const after=before.replace(old,'semio-framework-job-testing-caller = { path = "../../../../../../../../🔨️modules/🧵️job/⏱️context/🧪️testing/🎟️caller/📦️packages/🦀️rust" }');
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/actor-input-test-dependency.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared manifest changed');fs.renameSync(temp,file);
