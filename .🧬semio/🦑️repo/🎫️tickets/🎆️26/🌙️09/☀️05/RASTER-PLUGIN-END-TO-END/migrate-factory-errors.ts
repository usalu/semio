import { readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
function endBlock(text: string, start: number): number {
  let depth = 1, end = start + 1;
  while (depth && end < text.length) { const c = text[end++]; if (c === '{') depth++; else if (c === '}') depth--; }
  if (depth) throw new Error('unbalanced implementation');
  return end;
}
function walk(path: string) {
  for (const entry of readdirSync(path, { withFileTypes: true })) {
    const file = join(path, entry.name);
    if (entry.isDirectory()) { walk(file); continue; }
    if (!entry.name.endsWith('.rs')) continue;
    const original = readFileSync(file, 'utf8');
    let text = original;
    const matches = [...original.matchAll(/\bimpl(?:<[^\n{]*>)?\s+(?:[\w:]+::)?ArtifactOwnedValueRetirementFactory<[^\n{]+?\s+for\s+[^\n{]+\{/g)];
    for (const match of matches.reverse()) {
      const start = match.index!, end = endBlock(text, start + match[0].length - 1);
      let block = text.slice(start, end);
      const method = /fn\s+retire_owned\s*\([^)]*\)\s*->\s*[^\{]+\{/.exec(block);
      if (!method || !method[0].includes('RetainedWorkOutcome') || !method[0].includes('ValueError')) continue;
      const brace = method.index + method[0].length - 1, finish = endBlock(block, brace);
      const failure = file.startsWith('🧰️framework/🔨️modules/🌱️value/') ? 'crate::OriginalRetirementFailure' : 'semio_framework_value::OriginalRetirementFailure';
      const signature = method[0].replace(/\bValueError\b/g, failure);
      const body = block.slice(brace + 1, finish - 1);
      const replacement = `${signature}let turn=(||{${body}})();${file.startsWith('🧰️framework/🔨️modules/🌱️value/') ? 'crate' : 'semio_framework_value'}::RetainedWorkOutcome{result:turn.result.map_err(|(error,value)|(error.into(),value)),work:turn.work}}`;
      block = block.slice(0, method.index) + replacement + block.slice(finish);
      text = text.slice(0, start) + block + text.slice(end);
      console.log(file + ' ' + match[0]);
    }
    if (text !== original) writeFileSync(file, text);
  }
}
walk('🧰️framework');
