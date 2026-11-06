const fs = require('node:fs');
const path = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/🦀️.rs';
const before = fs.readFileSync(path, 'utf8');
let after = before;
for (const name of ['PollInput', 'CodecInput', 'RestoreInput']) {
  const marker = `    pub struct ${name} {`;
  if (after.split(marker).length !== 2) throw Error(`expected one ${name}`);
  after = after.replace(marker, `    #[serde(deny_unknown_fields)]\n    #[value(deny_unknown_fields)]\n${marker}`);
}
const temporary = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/guest-input-strict.tmp';
fs.writeFileSync(temporary, after);
if (fs.readFileSync(path, 'utf8') !== before) throw Error('shared schema changed');
fs.renameSync(temporary, path);
