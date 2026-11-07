const fs = require('fs');
const path = require('path');
const root = '🧰️framework/🔨️modules/🖱️ui/🧪️tests';
function update(file) {
  let source = fs.readFileSync(file, 'utf8');
  const edits = [];
  for (let at = 0; (at = source.indexOf('.frame_into_step', at)) >= 0; at += 16) {
    let open = at + '.frame_into_step'.length;
    if (source.startsWith('::<', open)) { const end = source.indexOf('>', open + 3); if (end < 0) continue; open = end + 1; }
    if (source[open] !== '(') continue;
    let depth = 1, quote = null, escaped = false, index = open + 1;
    for (; index < source.length && depth; index++) {
      const char = source[index];
      if (quote) { if (escaped) escaped = false; else if (char === '\\') escaped = true; else if (char === quote) quote = null; continue; }
      if (char === '"') { quote = char; continue; }
      if (char === '(') depth++;
      else if (char === ')') depth--;
    }
    if (depth) throw new Error(`Unclosed frame call: ${file}`);
    const close = index - 1;
    edits.push([close, ', ui_contract::UI_WORKER_RETIREMENT_POLICY', close + 1, '.0']);
    at = close;
  }
  for (const [before, argument, after, suffix] of edits.reverse()) source = source.slice(0, before) + argument + source.slice(before, after) + suffix + source.slice(after);
  if (edits.length) { fs.writeFileSync(file, source); process.stdout.write(`${file}: ${edits.length}\n`); }
}
function walk(dir) { for (const entry of fs.readdirSync(dir, { withFileTypes: true })) { const file = path.join(dir, entry.name); if (entry.isDirectory()) walk(file); else if (file.endsWith('.rs')) update(file); } }
walk(root);
