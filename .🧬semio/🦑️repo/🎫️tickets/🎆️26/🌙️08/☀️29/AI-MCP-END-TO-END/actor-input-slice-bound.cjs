const fs=require('node:fs');const file='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs';
const before=fs.readFileSync(file,'utf8'),old='if pointer==0||length==0||end.zip(memory).is_none_or(|(end,memory)|end>memory)';
if(before.split(old).length!==2)throw Error('guest wire extent guard changed');
const after=before.replace(old,'if pointer==0||length==0||length as u64>isize::MAX as u64||end.zip(memory).is_none_or(|(end,memory)|end>memory)');
const temp='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/actor-input-slice-bound.tmp';fs.writeFileSync(temp,after);if(fs.readFileSync(file,'utf8')!==before)throw Error('shared exports changed');fs.renameSync(temp,file);
