const fs = require('node:fs');
const path = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📣️macros/⚛️wp4-reactor/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
const fields = 'operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled'.split(',');
const args = fields.map(field => `${field}:u64`).join(',');
const budget = `$crate::owned_abi::original_job_budget([${fields.join(',')}])`;
let after = before;
for (const [oldName, newName, call] of [
  ['admit_actor_context', 'admit_actor_context', 'admit_owned(budget,|_|{$ensure();Ok(())})'],
  ['close_actor_context', 'close_actor_context', 'close_owned(budget)'],
  ['close_turn_output', 'close_actor_output', 'close_turn_output_owned(budget)'],
]) {
  const pattern = new RegExp(`        pub unsafe extern "C" fn semio_owned_${oldName}_v1\\([^]*?(?=        #\\[cfg\\(all\\(target_arch = "wasm32", target_env = "p2"\\)\\)\\])`);
  const matches = [...after.matchAll(new RegExp(pattern.source, 'g'))];
  if (matches.length !== 1) throw new Error(`expected one ${oldName}, found ${matches.length}`);
  const replacement = `        pub extern "C" fn semio_owned_${newName}_v1(${args}) -> u64 {\n            match ${budget}{Ok(budget)=>$crate::reactor::original_actor_context::exports::${call},Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}\n        }\n`;
  after = after.replace(pattern, replacement);
}
const temporary = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/guest-scalar-macro.tmp';
fs.mkdirSync(require('node:path').dirname(temporary), {recursive:true});
fs.writeFileSync(temporary, after);
if (fs.readFileSync(path, 'utf8') !== before) throw new Error('shared macro changed; refusing publication');
fs.renameSync(temporary, path);
