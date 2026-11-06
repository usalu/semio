const fs = require('node:fs');
const path = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
let after = before;
for (const name of ['take_actor_admission_input', 'take_actor_output_close_input']) {
  const pattern = new RegExp(`    /// [^\n]*\n    pub unsafe fn ${name}\\([^\n]*\n\n`);
  if ([...after.matchAll(new RegExp(pattern.source, 'g'))].length !== 1) throw new Error(`expected one ${name}`);
  after = after.replace(pattern, '');
}
const temporary = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/guest-scalar-schema.tmp';
fs.mkdirSync(require('node:path').dirname(temporary), {recursive:true});
fs.writeFileSync(temporary, after);
if (fs.readFileSync(path, 'utf8') !== before) throw new Error('shared schema changed; refusing publication');
fs.renameSync(temporary, path);
