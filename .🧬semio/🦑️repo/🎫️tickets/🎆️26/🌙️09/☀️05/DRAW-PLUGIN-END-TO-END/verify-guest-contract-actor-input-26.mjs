import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { parse } from 'smol-toml';

const plugin = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin';
const abi = join(plugin, '🚪️io');
const guest = join(plugin, '🧬️guest-contract');
const reactor = join(plugin, '⚛️reactor/🎟️context');
const call = join(reactor, '🌉️call');
const input = join(call, '📥️input');
const manifest = (owner) => join(owner, '📦️packages/🦀️rust/Cargo.toml');
const read = (file) => readFileSync(file, 'utf8');
const parseFile = (file) => parse(read(file));
const edge = (parent, dependency, target) => {
  const parentManifest = manifest(parent);
  const reference = parseFile(parentManifest).dependencies[dependency];
  assert.ok(reference?.path, `${parentManifest}: missing ${dependency}`);
  const childManifest = resolve(dirname(parentManifest), reference.path, 'Cargo.toml');
  assert.equal(parseFile(childManifest).package.name, dependency);
  if (target) assert.equal(childManifest, resolve(manifest(target)));
  return join(dirname(childManifest), parseFile(childManifest).lib.path);
};

assert.match(read(join(guest, '🦀️.rs')), /use semio_plugin_abi::actor_input_receipt as original_input;/);
assert.match(read(join(guest, '🎟️original-actor-context/🦀️.rs')), /#\[path="\.\.\/\.\.\/⚛️reactor\/🎟️context\/🌉️call\/🦀️.rs"\]/u);
assert.match(read(join(reactor, '🦀️.rs')), /pub use semio_plugin_actor_input as original_input;/);
assert.match(read(join(input, '🦀️.rs')), /pub use semio_plugin_abi::actor_input_receipt::\{([^}]+)\}/);
assert.match(read(join(abi, '🦀️.rs')), /#\[path = "📥️actor-input-receipt\/🦀️\.rs"\]\npub mod actor_input_receipt;/u);
const receipt = read(join(abi, '📥️actor-input-receipt/🦀️.rs'));
const receiving = 'semio-framework-pack-json-output-receiving';
const nativeDecode = 'semio-framework-job-native-decode';
assert.match(receipt, /pub use semio_framework_pack_json_output_receiving::OriginalJsonOutputFailure;/);
assert.match(read(edge(abi, receiving)), /pub enum OriginalJsonOutputFailure\b/);
edge(abi, nativeDecode);
edge(guest, 'semio-plugin-abi', abi);
edge(input, 'semio-plugin-abi', abi);
edge(reactor, 'semio-plugin-actor-input', input);
assert.equal(parseFile(manifest(guest)).dependencies['semio-plugin-actor-input'], undefined);

const symbols = new Set();
for (const relativePath of ['🦀️.rs', '📤️failure/🦀️.rs', '📤️receipt/🦀️.rs', '📤️turn-output/🦀️.rs']) {
  const source = read(join(call, relativePath));
  assert.doesNotMatch(source, /\bsemio_plugin_actor_input::/);
  const references = [...source.matchAll(/\bcrate::original_input::(\w+)/g)];
  assert.ok(references.length, relativePath);
  for (const [, symbol] of references) symbols.add(symbol);
}
for (const symbol of symbols) {
  assert.match(receipt, new RegExp(`\\bpub (?:struct |fn |use .*::)${symbol}\\b`));
  assert.match(read(join(input, '🦀️.rs')), new RegExp(`pub use semio_plugin_abi::actor_input_receipt::\\{[^}]*\\b${symbol}\\b`));
}
assert.deepEqual([...symbols].sort(), ['OriginalActorInputFailureIdentity', 'OriginalActorInputFailureView', 'OriginalJsonOutputFailure', 'retire_step']);

const locks = process.argv.slice(2);
assert.ok(locks.length);
const stale = new Map();
for (const file of locks) {
  const packages = new Map(parseFile(file).package.map((entry) => [entry.name, entry]));
  const has = (owner, child) => packages.get(owner)?.dependencies?.includes(child);
  const missing = [];
  if (!has('semio-plugin-abi', receiving)) missing.push(receiving);
  if (!has('semio-plugin-abi', nativeDecode)) missing.push(nativeDecode);
  if (packages.has('semio-plugin-guest-contract')) {
    if (!has('semio-plugin-guest-contract', 'semio-plugin-abi')) missing.push('semio-plugin-abi');
    if (has('semio-plugin-guest-contract', 'semio-plugin-actor-input')) missing.push('unexpected semio-plugin-actor-input');
  }
  if (missing.length) stale.set(file, missing.join(', '));
}
for (const [file, problem] of stale) console.error(`[DEBUG] ${file} ${problem}`);
assert.equal(stale.size, 0, 'Locks contradicting live ABI receipt-owner edges');
console.log(`[DEBUG] Verified ${symbols.size} receipt symbols and ${locks.length} Cargo lock boundaries.`);
