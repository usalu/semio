const fs = require('fs');
const path = process.argv[2];
const text = fs.readFileSync(path, 'utf8');
let depth = 0;
for (const ch of text) {
  if (ch === '{') depth++;
  else if (ch === '}') depth--;
}
console.log('final brace depth:', depth);
