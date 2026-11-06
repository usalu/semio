import { readFileSync, writeFileSync } from 'node:fs';
const base = '🧰️framework/🔨️modules/🌱️value/📋️list/';
const files = ['🎟️reservation/🧪️tests/🦀️.rs','🧪️tests/⚠️refusal/🦀️.rs','🧪️tests/📋️list/🦀️.rs','🧪️tests/🔬️counter/🦀️.rs'];
for (const name of files) {
  const path = base + name;
  let text = readFileSync(path, 'utf8'), offset = 0;
  const call = /\.reserve_one(?:_using::<[^>]+>)?\(/g;
  while (true) {
    call.lastIndex = offset;
    const match = call.exec(text); if (!match) break;
    let depth = 1, end = call.lastIndex;
    for (; end < text.length && depth; end++) { if (text[end] === '(') depth++; else if (text[end] === ')') depth--; }
    if (depth) throw new Error(`unclosed call in ${path}`);
    const args = text.slice(call.lastIndex, end - 1);
    if (!args.includes('&mut ')) { text = text.slice(0, end - 1) + ',&mut None' + text.slice(end - 1); end += 10; }
    offset = end;
  }
  writeFileSync(path, text);
}
