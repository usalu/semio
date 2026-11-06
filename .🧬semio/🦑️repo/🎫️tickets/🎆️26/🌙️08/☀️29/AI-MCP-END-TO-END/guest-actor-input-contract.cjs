const fs = require('node:fs');
const assert = require('node:assert/strict');
const source = fs.readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/🦀️.rs', 'utf8');
const declaration = source.match(/pub enum ActorInput\s*\{([^}]*)\}/);
assert.ok(declaration);
assert.deepEqual([...declaration[1].matchAll(/\b(Job|Poll|Codec|Restore)\(/g)].map(match => match[1]), ['Job', 'Poll', 'Codec', 'Restore']);
assert.doesNotMatch(source, /#\[cfg\(all\(target_arch = "wasm32", target_env = "p2"\)\)\]\s*#\[doc\(hidden\)\]\s*pub mod owned_abi/);
console.log('guest actor input declaration passed');
