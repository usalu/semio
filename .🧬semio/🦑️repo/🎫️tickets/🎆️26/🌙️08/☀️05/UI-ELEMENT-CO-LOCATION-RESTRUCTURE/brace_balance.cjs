const fs = require('fs');
const path = process.argv[2];
const lines = fs.readFileSync(path, 'utf8').split('\n');

let depth = 0;
let inBlockComment = false;
let inString = false;
let inChar = false;
let inRawString = false;
let rawHashCount = 0;
let minDepth = 0;

for (let li = 0; li < lines.length; li++) {
  const line = lines[li];
  let inLineComment = false;
  for (let i = 0; i < line.length; i++) {
    const c = line[i];
    const c2 = line[i + 1];
    if (inLineComment) continue;
    if (inBlockComment) {
      if (c === '*' && c2 === '/') { inBlockComment = false; i++; }
      continue;
    }
    if (inRawString) {
      if (c === '"') {
        let hashes = 0, j = i + 1;
        while (line[j] === '#') { hashes++; j++; }
        if (hashes >= rawHashCount) { inRawString = false; i = j - 1; }
      }
      continue;
    }
    if (inString) {
      if (c === '\\') { i++; continue; }
      if (c === '"') { inString = false; }
      continue;
    }
    if (inChar) {
      if (c === '\\') { i++; continue; }
      if (c === "'") { inChar = false; }
      continue;
    }
    if (c === '/' && c2 === '/') { inLineComment = true; continue; }
    if (c === '/' && c2 === '*') { inBlockComment = true; i++; continue; }
    if (c === 'r' && (c2 === '"' || c2 === '#')) {
      let j = i + 1, hashes = 0;
      while (line[j] === '#') { hashes++; j++; }
      if (line[j] === '"') { inRawString = true; rawHashCount = hashes; i = j; continue; }
    }
    if (c === '"') { inString = true; continue; }
    if (c === "'") {
      const rest = line.slice(i);
      const m = rest.match(/^'(\\.|[^'\\])'/);
      if (m) { inChar = true; }
      continue;
    }
    if (c === '{') depth++;
    else if (c === '}') { depth--; if (depth < minDepth) minDepth = depth; }
  }
}
console.log('final depth=' + depth + ' minDepth=' + minDepth);
