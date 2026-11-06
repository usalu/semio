const fs=require('node:fs'),assert=require('node:assert/strict');
const base='🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/';
const entry=fs.readFileSync(base+'⏱️turn-execution-and-poll-entry/🦀️.rs','utf8');
const turn=fs.readFileSync(base+'🌀️poll-kernel-turn/🦀️.rs','utf8');
assert.equal((entry.match(/cold_pair_page:\s*Option<&semio_framework::kernel::ColdDocumentPairPage>/g)||[]).length,2);
assert.match(turn,/cold_pair_page:\s*Option<&semio_framework::kernel::ColdDocumentPairPage>/);
assert.match(turn,/drive_cold_pair_ingress\(runtime, cold_pair_page, identity, cx\)/);
console.log('cold-pair poll borrowing contract passed');
