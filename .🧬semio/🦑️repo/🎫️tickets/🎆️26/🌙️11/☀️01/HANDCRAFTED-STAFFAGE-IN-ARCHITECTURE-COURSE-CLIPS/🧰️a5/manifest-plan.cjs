const fs = require('fs');
const path = 'C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json';
let t = fs.readFileSync(path, 'utf8');
for (const id of process.argv.slice(2)) {
  const at = t.indexOf('"id": "' + id + '"');
  if (at < 0) throw new Error('missing ' + id);
  const vp = t.indexOf('"viewpoints":', at);
  const end = t.indexOf(']', vp) + 1;
  const block = t.slice(vp, end);
  if (block.includes('"plan"')) { console.log(id, 'already'); continue; }
  const next = block.includes('\n')
    ? block.replace(/"elevation"(\s*)\]/, '"elevation",$1  "plan"$1]')
    : block.replace('"elevation"]', '"elevation", "plan"]');
  t = t.slice(0, vp) + next + t.slice(end);
  console.log(id, JSON.stringify(next));
}
fs.writeFileSync(path, t);
JSON.parse(t);
