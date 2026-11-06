const fs = require('node:fs');
const base = '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/';
const changes = [
  ['🌀️poll-kernel-turn/🦀️.rs', 'drive_cold_pair_ingress(runtime, cold_pair_page, identity, cx)', 'drive_cold_pair_ingress(runtime, cold_pair_page.as_ref(), identity, cx)'],
  ['🧊️cold-pair-loads-and-process-pool/🦀️.rs', 'cold_pair_page: Option<semio_framework::kernel::ColdDocumentPairPage>, identity:', 'cold_pair_page: Option<&semio_framework::kernel::ColdDocumentPairPage>, identity:'],
];
for (const [file, old, replacement] of changes) {
  const path = base + file, before = fs.readFileSync(path, 'utf8');
  if (before.split(old).length !== 2) throw Error(`expected one ${file}`);
  const temporary = '.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️29/AI-MCP-END-TO-END/🗑️generated/poll-cold-' + changes.indexOf(changes.find(row => row[0] === file)) + '.tmp';
  fs.writeFileSync(temporary, before.replace(old, replacement));
  if (fs.readFileSync(path, 'utf8') !== before) throw Error('shared source changed');
  fs.renameSync(temporary, path);
}
