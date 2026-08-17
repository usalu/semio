const fs = require('fs');
const path = process.argv[2];
const startLine = parseInt(process.argv[3], 10); // 1-indexed line of "pub mod X {"
const lines = fs.readFileSync(path, 'utf8').split('\n');

let depth = 0;
let inLineComment = false;
let inBlockComment = false;
let inString = false;
let inChar = false;
let inRawString = false;
let rawHashCount = 0;
let started = false;
let endLine = -1;

for (let li = startLine - 1; li < lines.length; li++) {
  const line = lines[li];
  inLineComment = false;
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
        // check for matching hashes
        let hashes = 0;
        let j = i + 1;
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

    // not in any special state
    if (c === '/' && c2 === '/') { inLineComment = true; continue; }
    if (c === '/' && c2 === '*') { inBlockComment = true; i++; continue; }
    if (c === 'r' && (c2 === '"' || c2 === '#')) {
      // lookahead for r#*"
      let j = i + 1;
      let hashes = 0;
      while (line[j] === '#') { hashes++; j++; }
      if (line[j] === '"') {
        inRawString = true;
        rawHashCount = hashes;
        i = j;
        continue;
      }
    }
    if (c === '"') { inString = true; continue; }
    if (c === "'") {
      // could be lifetime 'a or char literal 'x'
      // heuristic: char literal if pattern 'X' or '\X' closes within a few chars
      const rest = line.slice(i);
      const charLitMatch = rest.match(/^'(\\.|[^'\\])'/);
      if (charLitMatch) { inChar = true; /* will close itself below via scan */ }
      else { continue; } // lifetime, skip
      continue;
    }
    if (c === '{') {
      depth++;
      started = true;
    } else if (c === '}') {
      depth--;
      if (started && depth === 0) {
        endLine = li + 1; // 1-indexed
        console.log(`END_LINE=${endLine}`);
        process.exit(0);
      }
    }
  }
}
console.log('NOT_FOUND depth=' + depth);
