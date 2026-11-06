const fs = require('node:fs');
const assert = require('node:assert/strict');
const source = fs.readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📣️macros/⚛️wp4-reactor/🦀️.rs', 'utf8');
for (const name of ['admit_actor_context', 'close_actor_context', 'close_actor_output']) {
  const match = source.match(new RegExp('fn semio_owned_' + name + '_v1\\(([^)]*)\\)\\s*->\\s*u64'));
  assert.ok(match, name);
  assert.equal((match[1].match(/:u64/g) || []).length, 19, name);
}
assert.match(source, /fn semio_owned_channel_version_v1\(\)\s*->\s*u32/);
console.log('guest scalar ABI contract passed');
