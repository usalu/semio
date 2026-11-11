const fs = require('fs');
const crypto = require('crypto');
const path = require('path');
const inputs = JSON.parse(fs.readFileSync(path.join(__dirname, '🗑️generated', 'cache-freeze-v4.json'), 'utf8'));
Object.assign(inputs, {
  '🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py': '138b3f20dabc7d26dcc2fcbaee83d26845931bc88babf9d130077868d09e5189',
  '🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py': '3f31c63d94a54fc8c654faa99af5313348af528de7228115618e141c407d20be',
  '🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py': '7bdfaf9cf2aad7c847f76af9526912fd50f53c7c5eb22286d7b2b2ef2f923e90',
  '🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts': '3e63ce6a069ee123a90f4ae8fbc752bc21e061ff445876c7aab6529e59efef2f',
});
let failures = 0;
for (const [file, expected] of Object.entries(inputs)) {
  try {
    if (crypto.createHash('sha256').update(fs.readFileSync(file)).digest('hex') !== expected) throw new Error('SHA-256 mismatch');
  } catch (error) {
    console.log(`[DEBUG] ${file}: ${error.message}`);
    failures++;
  }
}
console.log(`[DEBUG] checked ${Object.keys(inputs).length}, failures ${failures}, available GiB ${(require('os').freemem() / 2 ** 30).toFixed(2)}`);
process.exitCode = failures ? 1 : 0;
