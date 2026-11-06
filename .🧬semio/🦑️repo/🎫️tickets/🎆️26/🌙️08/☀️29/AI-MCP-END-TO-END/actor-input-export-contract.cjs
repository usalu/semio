const fs=require('node:fs');const assert=require('node:assert/strict');
const macro=fs.readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📣️macros/⚛️wp4-reactor/🦀️.rs','utf8');
const schema=fs.readFileSync('🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🔁️persistent-local/🦀️.rs','utf8');
for(const [name,prefix] of [['admit_actor_input',['pointer:u32','length:u32']],['start_job',['job:u64']],['publish_actor_failure',['job:u64']],['read_actor_failure',['job:u64']],['acknowledge_actor_failure',['job:u64']],['acknowledge_actor_failure_chunk',['job:u64','offset:u64','length:u64']]]){
  const match=macro.match(new RegExp('fn semio_owned_'+name+'_v1\\(([^)]*)\\)\\s*->\\s*u64'));
  assert.ok(match,name);const fields=match[1].split(',');assert.deepEqual(fields.slice(0,prefix.length),prefix);assert.equal(fields.length-prefix.length,19,name);assert.ok(fields.slice(prefix.length).every(field=>field.endsWith(':u64')));
}
const borrow=macro.match(/fn semio_owned_borrow_actor_failure_v1\(([^)]*)\)\s*->\s*u64/);
assert.ok(borrow);assert.equal(borrow[1].split(',').length,5);
assert.doesNotMatch(schema,/take_actor_json|take_actor_start_job_input|take_actor_poll_input/);
console.log('actual actor input, start and failure export signatures passed');
